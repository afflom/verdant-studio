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
    F77(u8, Bytes),
    F190(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F77(k0, k1) => f77(*k0, k1.clone(), p0),
            Fn0::F190(k0) => f190(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F122,
    F245,
    F340,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F122 => f122(p0, p1),
            Fn1::F245 => f245(p0, p1),
            Fn1::F340 => f340(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F213(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F213(k0) => f213(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F310(Bytes),
    F311(Bytes),
    F327(Bytes, u64, Bytes, u64),
    F400(Bytes),
    F401(u64, Bytes),
    F408(Bytes),
    F468(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F310(k0) => f310(k0.clone(), p0),
            Fn3::F311(k0) => f311(k0.clone(), p0),
            Fn3::F327(k0, k1, k2, k3) => f327(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F400(k0) => Ok(f400(k0.clone(), p0)),
            Fn3::F401(k0, k1) => f401(*k0, k1.clone(), p0),
            Fn3::F408(k0) => f408(k0.clone(), p0),
            Fn3::F468(k0) => f468(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F314(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F314(k0) => f314(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F390(Bytes),
    F503(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F390(k0) => f390(k0.clone(), p0),
            Fn5::F503(k0, k1) => Ok(f503(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F392,
    F494,
    F654,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F392 => f392(p0, p1),
            Fn6::F494 => f494(p0, p1),
            Fn6::F654 => f654(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F395,
    F495,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F395 => f395(p0, p1),
            Fn7::F495 => f495(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F424,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn8::F424 => f424(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F427,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn9::F427 => f427(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F429,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn10::F429 => f429(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F432,
}

impl Fn11 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn11::F432 => f432(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F454(Bytes, u64, u64),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn12::F454(k0, k1, k2) => f454(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F473,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn13::F473 => f473(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F476,
}

impl Fn14 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn14::F476 => f476(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F507,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F507 => f507(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F509,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F509 => f509(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F512(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F512(k0) => f512(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F530,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F530 => f530(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F536,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F536 => f536(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F540,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F540 => f540(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F586,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F586 => f586(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F589,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F589 => f589(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F616,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F616 => f616(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F640,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F640 => f640(p0),
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
                                            Ok(Adt12::C4(f8(Str::lit("read-webidl, read-xml, read-xsd and read-yaml run in verdant_readers, the command of the readers project\u{a}"))))
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
                                Ok(Some((Adt2::C7(std::rc::Rc::new({
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
                            Ok(Some((Adt2::C7(std::rc::Rc::new({
                                let (h375, _) = v6;
                                h375
                            }), std::rc::Rc::new(Adt2::C6)), {
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
                                                Ok(Some((Adt2::C8({
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
                                            Ok(Some((Adt2::C8({
                                                let (_, h406) = v9;
                                                h406
                                            }, std::rc::Rc::new({
                                                let (h407, _) = v10;
                                                h407
                                            }), std::rc::Rc::new(Adt2::C6)), {
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
                            Ok(Some((Adt2::C3({
                                let (_, h416) = v13;
                                h416
                            }), {
                                let (h417, _) = v13;
                                h417
                            })))
                        }
                    }
                } else {
                    if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt2::C0(true), {
                            let a418 = v1;
                            let a419 = 4u64;
                            nat_add(a418, a419)?
                        })))
                    } else {
                        if f32(f46(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt2::C0(false), {
                                let a420 = v1;
                                let a421 = 5u64;
                                nat_add(a420, a421)?
                            })))
                        } else {
                            let a422 = v5;
                            let a423 = 91u64;
                            if nat_eq(a422, a423) {
                                let a426 = f116(v0.clone(), {
                                    let a424 = v1;
                                    let a425 = 1u64;
                                    nat_add(a424, a425)?
                                })?;
                                let a427 = 93u64;
                                if nat_eq(a426, a427) {
                                    Ok(Some((Adt2::C4(std::rc::Rc::new(Adt2::C6)), {
                                        let a428 = v1;
                                        let a429 = 2u64;
                                        nat_add(a428, a429)?
                                    })))
                                } else {
                                    let m432 = f75(v0, {
                                        let a430 = v1;
                                        let a431 = 1u64;
                                        nat_add(a430, a431)?
                                    }, v4, 1u64)?;
                                    match m432 {
                                        None => {
                                            Ok(None::<(Adt2, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt2::C4(std::rc::Rc::new({
                                                let (h433, _) = v14;
                                                h433
                                            })), {
                                                let (_, h434) = v14;
                                                h434
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a435 = v5;
                                let a436 = 123u64;
                                if nat_eq(a435, a436) {
                                    let a439 = f116(v0.clone(), {
                                        let a437 = v1;
                                        let a438 = 1u64;
                                        nat_add(a437, a438)?
                                    })?;
                                    let a440 = 125u64;
                                    if nat_eq(a439, a440) {
                                        Ok(Some((Adt2::C5(std::rc::Rc::new(Adt2::C6)), {
                                            let a441 = v1;
                                            let a442 = 2u64;
                                            nat_add(a441, a442)?
                                        })))
                                    } else {
                                        let m445 = f75(v0, {
                                            let a443 = v1;
                                            let a444 = 1u64;
                                            nat_add(a443, a444)?
                                        }, v4, 2u64)?;
                                        match m445 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt2::C5(std::rc::Rc::new({
                                                    let (h446, _) = v15;
                                                    h446
                                                })), {
                                                    let (_, h447) = v15;
                                                    h447
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m448 = f119(v0, v1)?;
                                    match m448 {
                                        None => {
                                            Ok(None::<(Adt2, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h449) = v16;
                                                h449
                                            }, {
                                                let (h450, _) = v16;
                                                h450
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

pub fn f76(v0: Fn0, p455: u64, p456: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m454 = (p455, p456);
    {
        let mut m454 = m454;
        loop {
            let step = {
                let (v1, v2) = m454;
                let m451 = v1;
                if m451 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m451 - 1;
                    let m453 = {
                        let c452: Fn0 = v0.clone();
                        c452.apply(v2.clone())?
                    };
                    match m453 {
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
                Ok(next) => { m454 = next; }
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
    let m457 = f125(v0.clone());
    match m457.uncons() {
        None => {
            let m458 = f126(f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f127(v0.clone())?);
            match m458.uncons() {
                None => {
                    let v5: Adt15 = f128(v0.clone())?;
                    let v6: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(f126(f129(f130(match v5.clone() {
                        Adt15::C0(h459, _, _) => {
                            h459
                        }
                    }, f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f132(v6.clone())), f133(f130(match v5.clone() {
                        Adt15::C0(h460, _, _) => {
                            h460
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
        Adt1::C0(_, _, h461, _) => {
            h461
        }
    })) {
        Adt12::C2(f8(Str::lit("npx")), f138(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f51(f52(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h462, _) => {
                h462
            }
        })) {
            Adt12::C2(f8(Str::lit("npx")), f138(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f82(v0: Adt1) -> R<Adt12> {
    let m463 = f56(v0.clone(), f139());
    match m463.uncons() {
        None => {
            let m464 = f140(v0.clone(), f139())?;
            match m464.uncons() {
                None => {
                    let m467 = f141(match v0.clone() {
                        Adt1::C0(_, h465, _, _) => {
                            h465
                        }
                    }, f126(f142(), f143(match v0.clone() {
                        Adt1::C0(_, h466, _, _) => {
                            h466
                        }
                    }, false)));
                    match m467.uncons() {
                        None => {
                            let m469 = f56(v0.clone(), f143(match v0.clone() {
                                Adt1::C0(_, h468, _, _) => {
                                    h468
                                }
                            }, true));
                            match m469.uncons() {
                                None => {
                                    let m470 = f56(v0.clone(), f144());
                                    match m470.uncons() {
                                        None => {
                                            let m471 = f145(v0.clone())?;
                                            match m471 {
                                                None => {
                                                    let m473 = f146(v0.clone(), f143(match v0.clone() {
                                                        Adt1::C0(_, h472, _, _) => {
                                                            h472
                                                        }
                                                    }, true))?;
                                                    match m473 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f147(Adt17::C0(Adt16::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f148(v0.clone())?, f149(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f149(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f97(v0.clone(), f8(Str::lit("model/strata.json")))?, f150(f97(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f143(match v0 {
                                                                Adt1::C0(_, h474, _, _) => {
                                                                    h474
                                                                }
                                                            }, true)))?;
                                                            let (h475, _) = v13;
                                                            if h475 {
                                                                Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h476) = v13;
                                                                    h476
                                                                }))
                                                            } else {
                                                                Ok(Adt12::C4({
                                                                    let (_, h477) = v13;
                                                                    h477
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
        Adt1::C0(_, h478, _, _) => {
            h478
        }
    }, f152())) {
        Ok(Adt12::C1(List::cons(f152(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f153(f154(f152(), f155(match v0.clone() {
            Adt1::C0(_, h479, _, _) => {
                h479
            }
        }, f152())))?;
        let m480 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m480.uncons() {
            None => {
                let m481 = f156(v0.clone(), f157(f158(f96(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?, f159(v2, v1.clone()))?;
                match m481 {
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
    let m482 = f125(v0.clone());
    match m482.uncons() {
        None => {
            let m483 = f126(f56(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f161(), List::<Bytes>::nil())))), f127(v0.clone())?);
            match m483.uncons() {
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
    let m488 = {
        let a485 = v0;
        let a486 = 0u64;
        let a487 = {
            let a484 = v1.clone();
            length_bytes(a484)
        };
        slice_bytes(a485, a486, a487)
    };
    match m488 {
        None => {
            false
        }
        Some(v2) => {
            let a489 = v2;
            let a490 = v1;
            equal(a489, a490)
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
                            let m491 = v2;
                            match m491.uncons() {
                                None => {
                                    Ok(Adt12::C4(f79()))
                                }
                                Some((v4, v5)) => {
                                    f184(v3, v1, v4, v5)
                                }
                            }
                        } else {
                            if f32(v0, f8(Str::lit("record-decision"))) {
                                f185(v3, v1)
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

pub fn f88(v0: Bytes) -> R<bool> {
    f186(v0.clone(), 0u64, {
        let a492 = v0;
        length_bytes(a492)
    })
}

pub fn f89(p497: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m496 = p497;
    {
        let mut m496 = m496;
        loop {
            let step = {
                let v0 = m496;
                let m493 = v0;
                match m493.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h494, _) = v2;
                            h494
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h495) = v2;
                                h495
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m496 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m501 = {
        let a499 = match v0 {
            Adt1::C0(h498, _, _, _) => {
                h498
            }
        };
        let a500 = f40(v1);
        index_list(a499, a500)
    };
    match m501 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m502 = f187(f188(v0.clone(), v1.clone()), v1.clone());
    match m502 {
        None => {
            f43(f64(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f92(v0: Adt2) -> Option<Adt14> {
    let m503 = v0;
    match m503 {
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
        Adt2::C4(r510) => {
            let v8 = (*r510).clone();
            let m504 = f92(v8);
            match m504 {
                None => {
                    None::<Adt14>
                }
                Some(v9) => {
                    Some(Adt14::C3(std::rc::Rc::new(v9)))
                }
            }
        }
        Adt2::C5(r511) => {
            let v10 = (*r511).clone();
            let m505 = f92(v10);
            match m505 {
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
        Adt2::C7(r512, r513) => {
            let v12 = (*r512).clone();
            let v13 = (*r513).clone();
            let m506 = f92(v12);
            match m506 {
                None => {
                    None::<Adt14>
                }
                Some(v14) => {
                    let m507 = f92(v13);
                    match m507 {
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
        Adt2::C8(v16, r514, r515) => {
            let v17 = (*r514).clone();
            let v18 = (*r515).clone();
            let m508 = f92(v17);
            match m508 {
                None => {
                    None::<Adt14>
                }
                Some(v19) => {
                    let m509 = f92(v18);
                    match m509 {
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

pub fn f93(v0: Option<Adt14>) -> Option<Bytes> {
    let m516 = v0;
    match m516 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m517 = v1;
            match m517 {
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

pub fn f94(p523: Adt14, v1: Bytes) -> Option<Adt14> {
    let m522 = p523;
    {
        let mut m522 = m522;
        loop {
            let step = {
                let v0 = m522;
                let m518 = v0;
                match m518 {
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
                    Adt14::C4(r519) => {
                        let v5 = (*r519).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r520, r521) => {
                        let v3 = (*r520).clone();
                        let v4 = (*r521).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m522 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m524 = v0;
    match m524.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m525 = f189(v2, v1.clone())?;
            match m525 {
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
        let a526 = v0.clone();
        length_bytes(a526)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h527, _) = f76(Fn0::F190(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h527
    };
    let v3: u64 = {
        let (h528, _) = {
            let (_, h529) = v2.clone();
            h529
        };
        h528
    };
    let v4: List<Bytes> = {
        let (_, h530) = {
            let (_, h531) = v2;
            h531
        };
        h530
    };
    let a532 = v3;
    let a533 = v1;
    if nat_lt(a532, a533) {
        Ok(f78(List::cons(f46(v0, v3, {
            let a534 = v1;
            let a535 = v3;
            nat_sub(a534, a535)
        }), v4)))
    } else {
        Ok(f78(v4))
    }
}

pub fn f97(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f191(f57(v0, v1)?))
}

pub fn f98(v0: List<Adt14>, v1: Bytes) -> List<Adt14> {
    let m536 = v0;
    match m536.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f192(v2, v1.clone()), f98(v3, v1))
        }
    }
}

pub fn f99(p541: List<Bytes>, p542: List<Bytes>) -> bool {
    let m540 = (p541, p542);
    {
        let mut m540 = m540;
        loop {
            let step = {
                let (v0, v1) = m540;
                let m537 = v0;
                match m537.uncons() {
                    None => {
                        let m538 = v1;
                        match m538.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m539 = v1;
                        match m539.uncons() {
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
                Ok(next) => { m540 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: List<Adt14>) -> List<Bytes> {
    let m543 = v0;
    match m543.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m544 = f94(v1, f8(Str::lit("std")));
            match m544 {
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
    let a545 = f193(v0)?;
    let a546 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a545, a546))
}

pub fn f102(v0: List<Adt14>) -> Adt14 {
    let m547 = v0;
    match m547.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f102(v2)))
        }
    }
}

pub fn f103(v0: Adt13) -> Bytes {
    let m548 = v0;
    match m548 {
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
    let a550 = {
        let a549 = v2.clone();
        length_bytes(a549)
    };
    let a551 = 0u64;
    if nat_eq(a550, a551) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m552 = f194(v0.clone(), f60())?;
        match m552 {
            None => {
                if f51(f52(f197(), match v0.clone() {
                    Adt1::C0(_, _, h553, _) => {
                        h553
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f197()))
                } else {
                    let m554 = f56(v0.clone(), List::cons(f198(), f199(f60())?));
                    match m554.uncons() {
                        None => {
                            Ok(Adt12::C3(f200(v0.clone(), f60(), v1.clone(), v2.clone(), f201(f64(v0, f198()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f195(v3.clone())?, f196(v3, v2)))
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

pub fn f115(v0: Bytes, p590: u64, p591: u64, p592: bool, p593: bool, p594: u64, p595: Bytes) -> R<Bytes> {
    let m589 = (p590, (p591, (p592, (p593, (p594, p595)))));
    {
        let mut m589 = m589;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m589;
                let m555 = v2;
                if m555 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f202(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m555 - 1;
                    let m558 = {
                        let a556 = v0.clone();
                        let a557 = v1;
                        index_bytes(a556, a557)
                    };
                    match m558 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f202(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f72(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a559 = v1;
                                    let a560 = 1u64;
                                    nat_add(a559, a560)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a561 = v9;
                                    let a562 = 34u64;
                                    !nat_eq(a561, a562)
                                }, (if v4 {
                                    false
                                } else {
                                    let a563 = v9;
                                    let a564 = 92u64;
                                    nat_eq(a563, a564)
                                }, (v5, v6))))))
                            } else {
                                let a565 = v9;
                                let a566 = 32u64;
                                if if if if nat_eq(a565, a566) {
                                    true
                                } else {
                                    let a567 = v9;
                                    let a568 = 9u64;
                                    nat_eq(a567, a568)
                                } {
                                    true
                                } else {
                                    let a569 = v9;
                                    let a570 = 10u64;
                                    nat_eq(a569, a570)
                                } {
                                    true
                                } else {
                                    let a571 = v9;
                                    let a572 = 13u64;
                                    nat_eq(a571, a572)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a573 = v1;
                                        let a574 = 1u64;
                                        nat_add(a573, a574)?
                                    }, (v7, (false, (false, ({
                                        let a575 = v1;
                                        let a576 = 1u64;
                                        nat_add(a575, a576)?
                                    }, f202(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    let a577 = v9;
                                    let a578 = 110u64;
                                    if if nat_eq(a577, a578) {
                                        f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("null")))
                                    } else {
                                        false
                                    } {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a579 = v1;
                                            let a580 = 4u64;
                                            nat_add(a579, a580)?
                                        }, (v7, (false, (false, ({
                                            let a581 = v1;
                                            let a582 = 4u64;
                                            nat_add(a581, a582)?
                                        }, {
                                            let a583 = f202(v0.clone(), v5, v1, v6);
                                            let a584 = f8(Str::lit("[]"));
                                            append_bytes(a583, a584)
                                        }))))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a585 = v1;
                                            let a586 = 1u64;
                                            nat_add(a585, a586)?
                                        }, (v7, ({
                                            let a587 = v9;
                                            let a588 = 34u64;
                                            nat_eq(a587, a588)
                                        }, (false, (v5, v6))))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m589 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: Bytes, v1: u64) -> R<u64> {
    let m598 = {
        let a596 = v0;
        let a597 = v1;
        index_bytes(a596, a597)
    };
    match m598 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a643 = {
                let a619 = {
                    let a607 = {
                        let a601 = {
                            let a599 = v3;
                            let a600 = 128u8;
                            fixed_u8::bit_and(a599, a600)
                        };
                        let a602 = 128u8;
                        if equal(a601, a602) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a608 = {
                        let a605 = {
                            let a603 = v3;
                            let a604 = 64u8;
                            fixed_u8::bit_and(a603, a604)
                        };
                        let a606 = 64u8;
                        if equal(a605, a606) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a607, a608)?
                };
                let a620 = {
                    let a617 = {
                        let a611 = {
                            let a609 = v3;
                            let a610 = 32u8;
                            fixed_u8::bit_and(a609, a610)
                        };
                        let a612 = 32u8;
                        if equal(a611, a612) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a618 = {
                        let a615 = {
                            let a613 = v3;
                            let a614 = 16u8;
                            fixed_u8::bit_and(a613, a614)
                        };
                        let a616 = 16u8;
                        if equal(a615, a616) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a617, a618)?
                };
                nat_add(a619, a620)?
            };
            let a644 = {
                let a641 = {
                    let a629 = {
                        let a623 = {
                            let a621 = v3;
                            let a622 = 8u8;
                            fixed_u8::bit_and(a621, a622)
                        };
                        let a624 = 8u8;
                        if equal(a623, a624) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a630 = {
                        let a627 = {
                            let a625 = v3;
                            let a626 = 4u8;
                            fixed_u8::bit_and(a625, a626)
                        };
                        let a628 = 4u8;
                        if equal(a627, a628) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a629, a630)?
                };
                let a642 = {
                    let a639 = {
                        let a633 = {
                            let a631 = v3;
                            let a632 = 2u8;
                            fixed_u8::bit_and(a631, a632)
                        };
                        let a634 = 2u8;
                        if equal(a633, a634) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a640 = {
                        let a637 = {
                            let a635 = v3;
                            let a636 = 1u8;
                            fixed_u8::bit_and(a635, a636)
                        };
                        let a638 = 1u8;
                        if equal(a637, a638) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a639, a640)?
                };
                nat_add(a641, a642)?
            };
            nat_add(a643, a644)
        }
    }
}

pub fn f117(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f203(v0.clone(), v1, {
        let a646 = {
            let a645 = v0.clone();
            length_bytes(a645)
        };
        let a647 = 1u64;
        nat_add(a646, a647)?
    })?;
    let a648 = f116(v0.clone(), v2)?;
    let a649 = 34u64;
    if nat_eq(a648, a649) {
        Ok(Some(({
            let a650 = v2;
            let a651 = 1u64;
            nat_add(a650, a651)?
        }, f46(v0, v1, {
            let a652 = v2;
            let a653 = v1;
            nat_sub(a652, a653)
        }))))
    } else {
        f204(v0.clone(), v1, {
            let a655 = {
                let a654 = v0;
                length_bytes(a654)
            };
            let a656 = 1u64;
            nat_add(a655, a656)?
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
        let a657 = f116(v0.clone(), v1)?;
        let a658 = 45u64;
        nat_eq(a657, a658)
    };
    let v3: u64 = if v2 {
        let a659 = v1;
        let a660 = 1u64;
        nat_add(a659, a660)?
    } else {
        v1
    };
    let v4: u64 = f205(v0.clone(), v3)?;
    let v5: u64 = {
        let a661 = v4;
        let a662 = v3;
        nat_sub(a661, a662)
    };
    let v6: bool = {
        let a663 = f116(v0.clone(), v4)?;
        let a664 = 46u64;
        nat_eq(a663, a664)
    };
    let v7: u64 = if v6 {
        let a665 = v4;
        let a666 = 1u64;
        nat_add(a665, a666)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f205(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f116(v0.clone(), v8)?;
    let v10: bool = {
        let a667 = v9;
        let a668 = 101u64;
        if nat_eq(a667, a668) {
            true
        } else {
            let a669 = v9;
            let a670 = 69u64;
            nat_eq(a669, a670)
        }
    };
    let v11: u64 = f116(v0.clone(), {
        let a671 = v8;
        let a672 = 1u64;
        nat_add(a671, a672)?
    })?;
    let v12: bool = f118(v10, f206({
        let a673 = v11;
        let a674 = 43u64;
        nat_eq(a673, a674)
    }, {
        let a675 = v11;
        let a676 = 45u64;
        nat_eq(a675, a676)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a677 = v8;
            let a678 = 2u64;
            nat_add(a677, a678)?
        } else {
            let a679 = v8;
            let a680 = 1u64;
            nat_add(a679, a680)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f205(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a681 = v14;
        let a682 = v13;
        nat_sub(a681, a682)
    });
    let v16: u64 = f207(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a684 = {
            let a683 = v15;
            length_bytes(a683)
        };
        let a685 = v16;
        nat_sub(a684, a685)
    });
    let a686 = v5;
    let a687 = 0u64;
    if if nat_eq(a686, a687) {
        true
    } else {
        if f118({
            let a688 = 1u64;
            let a689 = v5;
            nat_lt(a688, a689)
        }, {
            let a690 = f116(v0.clone(), v3)?;
            let a691 = 48u64;
            nat_eq(a690, a691)
        }) {
            true
        } else {
            if f118(v6, {
                let a692 = v8;
                let a693 = v7;
                nat_eq(a692, a693)
            }) {
                true
            } else {
                f206(if v10 {
                    let a694 = v14;
                    let a695 = v13;
                    nat_eq(a694, a695)
                } else {
                    false
                }, {
                    let a697 = 18u64;
                    let a698 = {
                        let a696 = v17.clone();
                        length_bytes(a696)
                    };
                    nat_lt(a697, a698)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m702 = {
            let a700 = {
                let a699 = v17.clone();
                length_bytes(a699)
            };
            let a701 = 0u64;
            if nat_eq(a700, a701) {
                Some(0u64)
            } else {
                f208(v17)?
            }
        };
        match m702 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m707 = f209(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a703 = v8;
                    let a704 = v7;
                    nat_sub(a703, a704)
                }), f118(v12, {
                    let a705 = v11;
                    let a706 = 45u64;
                    nat_eq(a705, a706)
                }), v18)?;
                match m707 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a710 = 19u64;
                        let a711 = {
                            let a709 = {
                                let (h708, _) = v19.clone();
                                h708
                            };
                            length_bytes(a709)
                        };
                        if nat_lt(a710, a711) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m713 = f208({
                                let (h712, _) = v19.clone();
                                h712
                            })?;
                            match m713 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f118(v2, {
                                        let a714 = 0u64;
                                        let a715 = v20;
                                        nat_lt(a714, a715)
                                    });
                                    if f210(v21, v20) {
                                        Ok(Some((v14, {
                                            let a717 = {
                                                let (_, h716) = v19.clone();
                                                h716
                                            };
                                            let a718 = 0u64;
                                            if nat_eq(a717, a718) {
                                                Adt2::C1(v21, v20)
                                            } else {
                                                Adt2::C2(v21, v20, {
                                                    let (_, h719) = v19;
                                                    h719
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
        let (h720, _) = v2.clone();
        h720
    };
    let v4: u64 = {
        let (h721, _) = {
            let (_, h722) = v2.clone();
            h722
        };
        h721
    };
    let v5: List<Bytes> = {
        let (_, h723) = {
            let (_, h724) = v2;
            h724
        };
        h723
    };
    let m725 = f211(v0.clone(), v3);
    match m725 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a726 = v6;
            let a727 = v1;
            if equal(a726, a727) {
                Ok(Some(({
                    let a728 = v3;
                    let a729 = 1u64;
                    nat_add(a728, a729)?
                }, ({
                    let a730 = v3;
                    let a731 = 1u64;
                    nat_add(a730, a731)?
                }, List::cons(f46(v0, v4, {
                    let a732 = v3;
                    let a733 = v4;
                    nat_sub(a732, a733)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a734 = v3;
                    let a735 = 1u64;
                    nat_add(a734, a735)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f121(v0: Fn1, p739: List<Bytes>, p740: List<Bytes>) -> List<Bytes> {
    let m738 = (p739, p740);
    {
        let mut m738 = m738;
        loop {
            let step = {
                let (v1, v2) = m738;
                let m736 = v2;
                match m736.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c737: Fn1 = v0.clone();
                            c737.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m738 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f123(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m741 = f212(Fn2::F213(v1), None::<Bytes>, v0);
    match m741 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f124() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f125(v0: Adt1) -> List<Bytes> {
    if f151(match v0.clone() {
        Adt1::C0(_, h742, _, _) => {
            h742
        }
    }, f214()) {
        if f151(match v0 {
            Adt1::C0(_, h743, _, _) => {
                h743
            }
        }, f215()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f215(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f214(), List::<Bytes>::nil())
    }
}

pub fn f126(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m744 = v0;
    match m744.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt1) -> R<List<Bytes>> {
    Ok(f56(v0.clone(), List::cons(f216(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f137(v0)?)))))
}

pub fn f128(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f148(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f149(f97(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f129(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m745 = v0;
    match m745.uncons() {
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
    let m746 = v0;
    match m746.uncons() {
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
    let m747 = v0;
    match m747.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f126(f150(f191(f94(v1, f8(Str::lit("defers"))))), f131(v2))
        }
    }
}

pub fn f132(v0: List<Adt14>) -> List<Bytes> {
    let m748 = v0;
    match m748.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m749 = f94(v1, f8(Str::lit("std")));
            match m749 {
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
    let m750 = v0;
    match m750.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f162(f217(v2.clone()), f218(f219(v0, v1.clone(), v2)?));
    f220(f153(f221(v1))?, v3.clone(), {
        let a752 = {
            let a751 = v3;
            length_list(a751)
        };
        let a753 = 1u64;
        nat_add(a752, a753)?
    })
}

pub fn f135(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m754 = v1;
    match m754.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f222(v2.clone(), f8(Str::lit("subject"))) {
                f223(v0.clone(), v2.clone())
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
    let m755 = v1;
    match m755.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m756 = f224(f96(f64(v0.clone(), v2.clone()))?, f85(v2, f215()))?;
            match m756 {
                None => {
                    f136(v0, v3)
                }
                Some(v4) => {
                    Ok(f225(v4, f136(v0, v3)?))
                }
            }
        }
    }
}

pub fn f137(v0: Adt1) -> R<List<Bytes>> {
    Ok(f126(f153(f226(f214(), f155(match v0.clone() {
        Adt1::C0(_, h757, _, _) => {
            h757
        }
    }, f214())))?, f153(f226(f215(), f155(match v0 {
        Adt1::C0(_, h758, _, _) => {
            h758
        }
    }, f215())))?))
}

pub fn f138(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f139() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f216(), List::<Bytes>::nil())))))))
}

pub fn f140(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m759 = v1;
    match m759.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m760 = f57(v0.clone(), v2.clone())?;
                match m760 {
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
    let m761 = v1;
    match m761.uncons() {
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
    let m762 = v0;
    match m762.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f126(f227({
                let (h763, _) = v2;
                h763
            }, {
                let (_, h764) = v2;
                h764
            }, v1), f143(v3, v1))
        }
    }
}

pub fn f144() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f145(v0: Adt1) -> R<Option<Bytes>> {
    f228(f229(v0, f144())?)
}

pub fn f146(v0: Adt1, p768: List<Bytes>) -> R<Option<Bytes>> {
    let m767 = p768;
    {
        let mut m767 = m767;
        loop {
            let step = {
                let v1 = m767;
                let m765 = v1;
                match m765.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m766 = f230(f64(v0.clone(), v2.clone()), f231())?;
                        match m766 {
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
                Ok(next) => { m767 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: Adt17) -> R<(bool, Bytes)> {
    let v1: Adt16 = match v0.clone() {
        Adt17::C0(h769, _, _, _, _) => {
            h769
        }
    };
    let v2: Adt18 = f232(v1.clone())?;
    let v3: List<Bytes> = f233(v1.clone());
    let v4: List<Bytes> = f149(match v0.clone() {
        Adt17::C0(_, _, h770, _, _) => {
            h770
        }
    }, f8(Str::lit("path")));
    let m772 = f234(v2, List::<u64>::nil(), f235(v1.clone(), match v0.clone() {
        Adt17::C0(_, h771, _, _, _) => {
            h771
        }
    }, 0u64)?);
    match m772 {
        None => {
            if f237(f238(match v0.clone() {
                Adt17::C0(_, _, _, _, h774) => {
                    h774
                }
            }, f126(match v0.clone() {
                Adt17::C0(_, _, _, _, h775) => {
                    h775
                }
            }, v4.clone()))?, f238(v4.clone(), f126(match v0.clone() {
                Adt17::C0(_, _, _, _, h776) => {
                    h776
                }
            }, v4.clone()))?, f239(match v0.clone() {
                Adt17::C0(_, _, h777, _, _) => {
                    h777
                }
            })) {
                let m781 = f240(f238(match v0.clone() {
                    Adt17::C0(_, _, _, h778, _) => {
                        h778
                    }
                }, f126(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h779, _) => {
                        h779
                    }
                }))?, f238(v3.clone(), f126(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h780, _) => {
                        h780
                    }
                }))?);
                match m781 {
                    None => {
                        let m784 = f241(f149(match v0 {
                            Adt17::C0(_, h783, _, _, _) => {
                                h783
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m784 {
                            None => {
                                let m785 = f241(v4, List::<Bytes>::nil());
                                match m785 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f236(f126(v3, match v0 {
                            Adt17::C0(_, _, _, h782, _) => {
                                h782
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f236(match v1 {
                Adt16::C0(_, _, _, _, h773) => {
                    h773
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f148(v0: Adt1) -> R<List<Bytes>> {
    let m786 = f242(f64(v0, f216()))?;
    match m786.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f243(v2))
        }
    }
}

pub fn f149(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m787 = v0;
    match m787.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f149(v3, v1))
        }
    }
}

pub fn f150(v0: List<Adt14>) -> List<Bytes> {
    let m788 = v0;
    match m788.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m789 = f93(Some(v1));
            match m789 {
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

pub fn f151(p793: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m792 = p793;
    {
        let mut m792 = m792;
        loop {
            let step = {
                let v0 = m792;
                let m790 = v0;
                match m790.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h791, _) = v2;
                            h791
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
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

pub fn f152() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f153(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f244(f121(Fn1::F245, List::<Bytes>::nil(), f246(v0)?)))
}

pub fn f154(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m794 = v1;
    match m794.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f247(v0.clone(), v2), f154(v0, v3))
            } else {
                f154(v0, v3)
            }
        }
    }
}

pub fn f155(p799: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m798 = p799;
    {
        let mut m798 = m798;
        loop {
            let step = {
                let v0 = m798;
                let m795 = v0;
                match m795.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h796, _) = v2;
                            h796
                        }, v1.clone()) {
                            let (_, h797) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h797)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m798 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, p802: List<Bytes>) -> R<Option<Bytes>> {
    let m801 = p802;
    {
        let mut m801 = m801;
        loop {
            let step = {
                let v2 = m801;
                let m800 = v2;
                match m800.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f248(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m801 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: List<List<Bytes>>) -> R<List<(List<Bytes>, List<Bytes>)>> {
    let m803 = v0;
    match m803.uncons() {
        None => {
            Ok(List::<(List<Bytes>, List<Bytes>)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f249(v1.clone())?, v1), f157(v2)?))
        }
    }
}

pub fn f158(p806: List<Bytes>, p807: List<Bytes>, p808: List<List<Bytes>>) -> List<List<Bytes>> {
    let m805 = (p806, (p807, p808));
    {
        let mut m805 = m805;
        loop {
            let step = {
                let (v0, (v1, v2)) = m805;
                let m804 = v0;
                match m804.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f250(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f250(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m805 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m809 = v0;
    match m809.uncons() {
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
        Adt1::C0(_, h810, _, _) => {
            h810
        }
    }, f251())) {
        Ok(Adt12::C1(List::cons(f251(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f159(f153(f154(f251(), f155(match v0.clone() {
            Adt1::C0(_, h811, _, _) => {
                h811
            }
        }, f251())))?, v1.clone());
        let m812 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m812.uncons() {
            None => {
                let v5: List<Bytes> = f149(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m813 = f56(v0.clone(), f252(v0.clone(), v2.clone(), v5.clone())?);
                match m813.uncons() {
                    None => {
                        let m814 = f253(v0.clone(), v2, v5)?;
                        match m814 {
                            None => {
                                f254(v0, v1)
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
    let m815 = v0;
    match m815.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f162(v3, v1))
        }
    }
}

pub fn f163(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m816 = f97(v0.clone(), f161())?;
    match m816.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f255(v0)?), List::<(Bytes, Bytes)>::nil()))
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
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f256(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f167() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f256(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
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
    let v4: List<List<Bytes>> = f257(f242(f64(v0, f216()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f258(v4.clone(), f243(v4), f219(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f134(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f171(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f259(f128(v0.clone())?, f136(v0.clone(), f137(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f172(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f260(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f261(f97(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f173() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f174(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m817 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m817.uncons() {
        None => {
            if f262(f96(f64(v0, v1))?)? {
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
    let m818 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m818.uncons() {
        None => {
            let v5: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f263(v5)?
                } else {
                    f264(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f247(f265(), f266(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f247(f265(), f266(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f267()
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
    let m819 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m819.uncons() {
        None => {
            let m820 = f268(f96(f64(v0.clone(), v1.clone()))?)?;
            match m820 {
                None => {
                    Ok(Adt12::C4(f269()))
                }
                Some(v4) => {
                    let m821 = f270(f96(f64(v0, v1))?)?;
                    match m821 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f271(v5)))
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
    let m822 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m822.uncons() {
        None => {
            let m823 = f272(f96(f64(v0, v1))?)?;
            match m823 {
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
    let m824 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m824.uncons() {
        None => {
            let v5: Bytes = f247(if v2 {
                f215()
            } else {
                f214()
            }, f266(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m825 = f224(f96(f64(v0.clone(), v1.clone()))?, v2)?;
                match m825 {
                    None => {
                        Ok(Adt12::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a827 = {
                            let a826 = v6;
                            length_list(a826)
                        };
                        let a828 = 0u64;
                        if nat_eq(a827, a828) {
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
    let m829 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m829.uncons() {
        None => {
            let v4: List<Bytes> = f96(f64(v0.clone(), v1))?;
            let m830 = f273(v4.clone(), f8(Str::lit("requirement")))?;
            match m830 {
                None => {
                    Ok(Adt12::C4(f267()))
                }
                Some(v5) => {
                    let m831 = f273(v4, f8(Str::lit("admission")))?;
                    match m831 {
                        None => {
                            Ok(Adt12::C4(f267()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f126(f149(v5.clone(), f8(Str::lit("id"))), f149(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f153(f126(f149(v7.clone(), f8(Str::lit("id"))), f149(v8.clone(), f8(Str::lit("construct")))))?;
                            let m832 = f274(f132(v6.clone()), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m832 {
                                None => {
                                    let a834 = {
                                        let a833 = v9.clone();
                                        length_list(a833)
                                    };
                                    let a835 = 0u64;
                                    if nat_eq(a834, a835) {
                                        Ok(Adt12::C4(f267()))
                                    } else {
                                        if f51({
                                            let a838 = {
                                                let a836 = f153(f126(v9.clone(), v10.clone()))?;
                                                length_list(a836)
                                            };
                                            let a839 = {
                                                let a837 = f126(v9.clone(), v10);
                                                length_list(a837)
                                            };
                                            nat_eq(a838, a839)
                                        }) {
                                            Ok(Adt12::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt12::C3(List::cons(f276(f8(Str::lit("model/requirements.json")), f225(v7, v5))?, List::cons(f276(f8(Str::lit("model/admissions.json")), f225(v8, v6))?, List::cons(f277(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f123(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f275(v11))
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
    let m840 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m840.uncons() {
        None => {
            let m841 = f91(v0.clone(), v1.clone())?;
            match m841 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m842 = f278(v4.clone(), f8(Str::lit("schema")));
                    match m842 {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f279(v0, f280(v5.clone())?, v5, f281(v4.clone(), f8(Str::lit("name"))), f282(f283(f278(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
    let m843 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m843.uncons() {
        None => {
            let m844 = f57(v0.clone(), v1.clone())?;
            match m844 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f284())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m845 = f94(v4.clone(), f8(Str::lit("source")));
                            match m845 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f93(f94(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f247(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f247(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m846 = v8;
                        match m846 {
                            None => {
                                f285(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f285(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f247(v9, f266(f58(v7.clone(), f8(Str::lit("url"))))?);
                                    if f51(f52(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h847, _) => {
                                            h847
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f58(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m848 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m848.uncons() {
                                            None => {
                                                if f32(f286(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f285(v0, v4, v5, v10, Some((v12, v11)))
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
    let m849 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m849.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f242(v4.clone())?;
            if f51(f99(f287(v5.clone()), f288())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f289(Str::lit("id"), v6.clone())?, List::cons(f289(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f289(Str::lit("artifact"), f216())?, List::cons(f289(Str::lit("checksum"), f286(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f216(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f290(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f277(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f123(f243(f257(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f183(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m850 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m850.uncons() {
        None => {
            let m851 = f57(v0, v1)?;
            match m851 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f291(f191(Some(v4))) {
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
    let m852 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m852.uncons() {
        None => {
            let m853 = f91(v0.clone(), v1.clone())?;
            match m853 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f279(v0.clone(), f64(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f185(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m854 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f161(), List::<Bytes>::nil())));
    match m854.uncons() {
        None => {
            let m855 = f292(f96(f64(v0.clone(), v1))?)?;
            match m855 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f97(v0, f161())?;
                    let v6: List<Bytes> = f149(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f153(f149(v5.clone(), f8(Str::lit("at"))))?;
                    let a857 = {
                        let a856 = v6.clone();
                        length_list(a856)
                    };
                    let a858 = 0u64;
                    if if nat_eq(a857, a858) {
                        true
                    } else {
                        f51({
                            let a861 = {
                                let a859 = f153(f126(v6.clone(), v7.clone()))?;
                                length_list(a859)
                            };
                            let a862 = {
                                let a860 = f126(v6.clone(), v7);
                                length_list(a860)
                            };
                            nat_eq(a861, a862)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f276(f161(), f225(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f123(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f186(v0: Bytes, p916: u64, p917: u64) -> R<bool> {
    let m915 = (p916, p917);
    {
        let mut m915 = m915;
        loop {
            let step = {
                let (v1, v2) = m915;
                let m863 = v2;
                if m863 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m863 - 1;
                    let m866 = {
                        let a864 = v0.clone();
                        let a865 = v1;
                        index_bytes(a864, a865)
                    };
                    match m866 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f293({
                                let v5: u8 = v4;
                                let a911 = {
                                    let a887 = {
                                        let a875 = {
                                            let a869 = {
                                                let a867 = v5;
                                                let a868 = 128u8;
                                                fixed_u8::bit_and(a867, a868)
                                            };
                                            let a870 = 128u8;
                                            if equal(a869, a870) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a876 = {
                                            let a873 = {
                                                let a871 = v5;
                                                let a872 = 64u8;
                                                fixed_u8::bit_and(a871, a872)
                                            };
                                            let a874 = 64u8;
                                            if equal(a873, a874) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a875, a876)?
                                    };
                                    let a888 = {
                                        let a885 = {
                                            let a879 = {
                                                let a877 = v5;
                                                let a878 = 32u8;
                                                fixed_u8::bit_and(a877, a878)
                                            };
                                            let a880 = 32u8;
                                            if equal(a879, a880) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a886 = {
                                            let a883 = {
                                                let a881 = v5;
                                                let a882 = 16u8;
                                                fixed_u8::bit_and(a881, a882)
                                            };
                                            let a884 = 16u8;
                                            if equal(a883, a884) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a885, a886)?
                                    };
                                    nat_add(a887, a888)?
                                };
                                let a912 = {
                                    let a909 = {
                                        let a897 = {
                                            let a891 = {
                                                let a889 = v5;
                                                let a890 = 8u8;
                                                fixed_u8::bit_and(a889, a890)
                                            };
                                            let a892 = 8u8;
                                            if equal(a891, a892) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a898 = {
                                            let a895 = {
                                                let a893 = v5;
                                                let a894 = 4u8;
                                                fixed_u8::bit_and(a893, a894)
                                            };
                                            let a896 = 4u8;
                                            if equal(a895, a896) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a897, a898)?
                                    };
                                    let a910 = {
                                        let a907 = {
                                            let a901 = {
                                                let a899 = v5;
                                                let a900 = 2u8;
                                                fixed_u8::bit_and(a899, a900)
                                            };
                                            let a902 = 2u8;
                                            if equal(a901, a902) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a908 = {
                                            let a905 = {
                                                let a903 = v5;
                                                let a904 = 1u8;
                                                fixed_u8::bit_and(a903, a904)
                                            };
                                            let a906 = 1u8;
                                            if equal(a905, a906) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a907, a908)?
                                    };
                                    nat_add(a909, a910)?
                                };
                                nat_add(a911, a912)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a913 = v1;
                                    let a914 = 1u64;
                                    nat_add(a913, a914)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(p922: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m921 = p922;
    {
        let mut m921 = m921;
        loop {
            let step = {
                let v0 = m921;
                let m918 = v0;
                match m918.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h919, _) = v2;
                            h919
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h920) = v2;
                                h920
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m921 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m926 = {
        let a924 = match v0 {
            Adt1::C0(_, _, _, h923) => {
                h923
            }
        };
        let a925 = f40(v1);
        index_list(a924, a925)
    };
    match m926 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f189(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), v1) {
        let m927 = f44(v0, 34u8)?;
        match m927.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m928 = v3;
                match m928.uncons() {
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

pub fn f190(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f294(v0, v1)
}

pub fn f191(v0: Option<Adt14>) -> List<Adt14> {
    let m929 = v0;
    match m929 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f295(v1)
        }
    }
}

pub fn f192(v0: Adt14, v1: Bytes) -> Adt14 {
    let m930 = v0;
    match m930 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r931) => {
            let v10 = (*r931).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r932) => {
            let v5 = (*r932).clone();
            Adt14::C4(std::rc::Rc::new(f192(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r933, r934) => {
            let v11 = (*r933).clone();
            let v12 = (*r934).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r935, r936) => {
            let v3 = (*r935).clone();
            let v4 = (*r936).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt14::C7(v2, std::rc::Rc::new(f296(v3, v1.clone())), std::rc::Rc::new(f192(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f192(v4, v1)))
            }
        }
    }
}

pub fn f193(v0: Adt14) -> R<Bytes> {
    let m937 = v0;
    match m937 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f297(v2, v3)
        }
        Adt14::C2(v4) => {
            f298(v4)
        }
        Adt14::C3(r938) => {
            let v5 = (*r938).clone();
            Ok(f299(f300(v5)?))
        }
        Adt14::C4(r939) => {
            let v6 = (*r939).clone();
            f301(f302(v6)?)
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

pub fn f194(v0: Adt1, p943: List<Adt13>) -> R<Option<Adt13>> {
    let m942 = p943;
    {
        let mut m942 = m942;
        loop {
            let step = {
                let v1 = m942;
                let m940 = v1;
                match m940.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f195(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h941, _) => {
                                h941
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
                Ok(next) => { m942 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Adt13) -> R<Bytes> {
    Ok(f247(f303(), f266(f304(v0))?))
}

pub fn f196(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f304(v0), List::<Bytes>::nil())))))
}

pub fn f197() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f198() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f199(v0: List<Adt13>) -> R<List<Bytes>> {
    let m944 = v0;
    match m944.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f195(v1)?, f199(v2)?))
        }
    }
}

pub fn f200(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m945 = v1;
    match m945.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f305(v5.clone(), v2.clone(), v3.clone(), f286(f64(v0.clone(), f195(v5)?))?, v4.clone())?), f200(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f201(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f306(v0.clone())?;
    let v2: u64 = f307(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a946 = v1;
        let a947 = v2;
        nat_sub(a946, a947)
    }))
}

pub fn f202(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a950 = v3;
    let a951 = f46(v0, v1, {
        let a948 = v2;
        let a949 = v1;
        nat_sub(a948, a949)
    });
    append_bytes(a950, a951)
}

pub fn f203(v0: Bytes, p964: u64, p965: u64) -> R<u64> {
    let m963 = (p964, p965);
    {
        let mut m963 = m963;
        loop {
            let step = {
                let (v1, v2) = m963;
                let m952 = v2;
                if m952 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m952 - 1;
                    let v4: u64 = f116(v0.clone(), v1)?;
                    let a953 = v4;
                    let a954 = 34u64;
                    if nat_eq(a953, a954) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a955 = v4;
                        let a956 = 92u64;
                        if nat_eq(a955, a956) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a957 = v4;
                            let a958 = 32u64;
                            if nat_lt(a957, a958) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a959 = v4;
                                let a960 = 256u64;
                                if nat_eq(a959, a960) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a961 = v1;
                                        let a962 = 1u64;
                                        nat_add(a961, a962)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m963 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Bytes, p991: u64, p992: u64, p993: Bytes) -> R<Option<(u64, Bytes)>> {
    let m990 = (p991, (p992, p993));
    {
        let mut m990 = m990;
        loop {
            let step = {
                let (v1, (v2, v3)) = m990;
                let m966 = v2;
                if m966 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m966 - 1;
                    let v5: u64 = f203(v0.clone(), v1, {
                        let a970 = {
                            let a968 = {
                                let a967 = v0.clone();
                                length_bytes(a967)
                            };
                            let a969 = v1;
                            nat_sub(a968, a969)
                        };
                        let a971 = 1u64;
                        nat_add(a970, a971)?
                    })?;
                    let v6: Bytes = {
                        let a974 = v3;
                        let a975 = f46(v0.clone(), v1, {
                            let a972 = v5;
                            let a973 = v1;
                            nat_sub(a972, a973)
                        });
                        append_bytes(a974, a975)
                    };
                    let v7: u64 = f116(v0.clone(), v5)?;
                    let a976 = v7;
                    let a977 = 34u64;
                    if nat_eq(a976, a977) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a978 = v5;
                            let a979 = 1u64;
                            nat_add(a978, a979)?
                        }, v6))))
                    } else {
                        let a980 = v7;
                        let a981 = 92u64;
                        if nat_eq(a980, a981) {
                            let m982 = f308(v0.clone(), v5, v6)?;
                            match m982 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a985 = {
                                        let (_, h983) = {
                                            let (_, h984) = v8.clone();
                                            h984
                                        };
                                        h983
                                    };
                                    let a986 = 0u64;
                                    if nat_eq(a985, a986) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h987, _) = v8;
                                            h987
                                        }, (v4, {
                                            let (h988, _) = {
                                                let (_, h989) = v8;
                                                h989
                                            };
                                            h988
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
                Ok(next) => { m990 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes, v1: u64) -> R<u64> {
    let (h994, _) = f309(Fn3::F310(v0.clone()), {
        let a996 = {
            let a995 = v0;
            length_bytes(a995)
        };
        let a997 = 1u64;
        nat_add(a996, a997)?
    }, v1)?;
    Ok(h994)
}

pub fn f206(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f207(v0: Bytes) -> R<u64> {
    let (h998, _) = f309(Fn3::F311(v0.clone()), {
        let a1000 = {
            let a999 = v0;
            length_bytes(a999)
        };
        let a1001 = 1u64;
        nat_add(a1000, a1001)?
    }, 0u64)?;
    Ok(h998)
}

pub fn f208(v0: Bytes) -> R<Option<u64>> {
    if f312(v0.clone())? {
        Ok(Some({
            let (_, h1002) = {
                let (h1003, _) = f313(Fn4::F314(v0.clone()), {
                    let a1004 = v0;
                    length_bytes(a1004)
                }, (0u64, 0u64))?;
                h1003
            };
            h1002
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f209(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a1006 = {
            let a1005 = v1.clone();
            length_bytes(a1005)
        };
        let a1007 = f315(v1)?;
        nat_sub(a1006, a1007)
    });
    let v5: Bytes = {
        let a1008 = v0;
        let a1009 = v4.clone();
        append_bytes(a1008, a1009)
    };
    let v6: u64 = f207(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a1011 = {
            let a1010 = v5;
            length_bytes(a1010)
        };
        let a1012 = v6;
        nat_sub(a1011, a1012)
    });
    let v8: u64 = {
        let a1013 = v4;
        length_bytes(a1013)
    };
    let v9: u64 = {
        let a1014 = v7.clone();
        length_bytes(a1014)
    };
    let a1015 = v9;
    let a1016 = 0u64;
    if nat_eq(a1015, a1016) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f206(v2, {
            let a1017 = v3;
            let a1018 = v8;
            nat_lt(a1017, a1018)
        }) {
            let v10: u64 = if v2 {
                let a1019 = v3;
                let a1020 = v8;
                nat_add(a1019, a1020)?
            } else {
                let a1021 = v8;
                let a1022 = v3;
                nat_sub(a1021, a1022)
            };
            let v11: u64 = f315(v7.clone())?;
            let a1023 = v10;
            let a1024 = v11;
            if nat_le(a1023, a1024) {
                Ok(Some((f46(v7, 0u64, {
                    let a1025 = v9;
                    let a1026 = v10;
                    nat_sub(a1025, a1026)
                }), 0u64)))
            } else {
                let a1029 = {
                    let a1027 = v10;
                    let a1028 = v11;
                    nat_sub(a1027, a1028)
                };
                let a1030 = 19u64;
                if nat_le(a1029, a1030) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1031 = v9;
                        let a1032 = v11;
                        nat_sub(a1031, a1032)
                    }), {
                        let a1033 = v10;
                        let a1034 = v11;
                        nat_sub(a1033, a1034)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1035 = v3;
                let a1036 = v8;
                nat_sub(a1035, a1036)
            };
            let a1039 = {
                let a1037 = v9;
                let a1038 = v12;
                nat_add(a1037, a1038)?
            };
            let a1040 = 19u64;
            if nat_le(a1039, a1040) {
                Ok(Some(({
                    let a1041 = v7;
                    let a1042 = f316(v12);
                    append_bytes(a1041, a1042)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f210(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1043 = 0u64;
        let a1044 = v1;
        if nat_lt(a1043, a1044) {
            let a1045 = v1;
            let a1046 = 9223372036854775808u64;
            nat_le(a1045, a1046)
        } else {
            false
        }
    } else {
        let a1047 = v1;
        let a1048 = 9223372036854775807u64;
        nat_le(a1047, a1048)
    }
}

pub fn f211(v0: Bytes, v1: u64) -> Option<u8> {
    let a1049 = v0;
    let a1050 = v1;
    index_bytes(a1049, a1050)
}

pub fn f212(v0: Fn2, p1054: Option<Bytes>, p1055: List<Bytes>) -> Option<Bytes> {
    let m1053 = (p1054, p1055);
    {
        let mut m1053 = m1053;
        loop {
            let step = {
                let (v1, v2) = m1053;
                let m1051 = v2;
                match m1051.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1052: Fn2 = v0.clone();
                            c1052.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1053 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1056 = v1;
    match m1056 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1059 = {
                    let a1057 = v3;
                    let a1058 = v0;
                    append_bytes(a1057, a1058)
                };
                let a1060 = v2;
                append_bytes(a1059, a1060)
            })
        }
    }
}

pub fn f214() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f215() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f216() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f217(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1061 = v0;
    match m1061.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f217(v2))
        }
    }
}

pub fn f218(v0: List<Adt19>) -> List<(Bytes, Bytes)> {
    let m1062 = v0;
    match m1062.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt19::C0(h1063, _, _) => {
                    h1063
                }
            }, match v1 {
                Adt19::C0(_, _, h1064) => {
                    h1064
                }
            }), f218(v2))
        }
    }
}

pub fn f219(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt19>> {
    let v3: List<(Bytes, Bytes)> = f317(v1);
    let v4: List<(Bytes, Bytes)> = f318(v0, v2);
    f319(v3.clone(), f162(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f220(p1071: List<Bytes>, v1: List<(Bytes, Bytes)>, p1072: u64) -> R<List<Bytes>> {
    let m1070 = (p1071, p1072);
    {
        let mut m1070 = m1070;
        loop {
            let step = {
                let (v0, v2) = m1070;
                let m1065 = v2;
                if m1065 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1065 - 1;
                    let v4: List<Bytes> = f153(f126(v0.clone(), f320(v0.clone(), v1.clone())))?;
                    let a1068 = {
                        let a1066 = v4.clone();
                        length_list(a1066)
                    };
                    let a1069 = {
                        let a1067 = v0.clone();
                        length_list(a1067)
                    };
                    if nat_eq(a1068, a1069) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
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

pub fn f221(v0: List<Adt14>) -> List<Bytes> {
    let m1073 = v0;
    match m1073.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1074 = f94(v1, f8(Str::lit("std")));
            match m1074 {
                None => {
                    f221(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f222(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f221(v2))
                    } else {
                        f221(v2)
                    })
                }
            }
        }
    }
}

pub fn f222(v0: Adt14, v1: Bytes) -> bool {
    f51({
        let a1076 = {
            let a1075 = f58(v0, v1);
            length_bytes(a1075)
        };
        let a1077 = 0u64;
        nat_eq(a1076, a1077)
    })
}

pub fn f223(v0: Adt15, v1: Adt14) -> bool {
    if f321(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f321(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f224(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m1078 = v0;
    match m1078.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f201(v2)?;
            let a1080 = {
                let a1079 = v4.clone();
                length_bytes(a1079)
            };
            let a1081 = 0u64;
            if nat_eq(a1080, a1081) {
                f224(v3, v1)
            } else {
                let m1082 = f322(v4)?;
                match m1082 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f323(v5.clone())
                        } else {
                            f324(v5.clone())
                        } {
                            let m1083 = f224(v3, v1)?;
                            match m1083 {
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

pub fn f225(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m1084 = v0;
    match m1084.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f225(v3, v1))
        }
    }
}

pub fn f226(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1085 = v1;
    match m1085.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f247(v0.clone(), v2), f226(v0, v3))
            } else {
                f226(v0, v3)
            }
        }
    }
}

pub fn f227(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1086 = v1;
    match m1086.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f325(v3.clone())
            } else {
                f51(f325(v3.clone()))
            } {
                List::cons(f247(v0.clone(), v3), f227(v0, v4, v2))
            } else {
                f227(v0, v4, v2)
            }
        }
    }
}

pub fn f228(p1089: List<Bytes>) -> R<Option<Bytes>> {
    let m1088 = p1089;
    {
        let mut m1088 = m1088;
        loop {
            let step = {
                let v0 = m1088;
                let m1087 = v0;
                match m1087.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f326(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f201(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1088 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1090 = v1;
    match m1090.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f126(f96(f64(v0.clone(), v2))?, f229(v0, v3)?))
        }
    }
}

pub fn f230(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1091 = v0.clone();
        length_bytes(a1091)
    };
    let v3: u64 = {
        let a1092 = v1.clone();
        length_bytes(a1092)
    };
    let v4: u64 = {
        let (h1093, _) = f309(Fn3::F327(v1.clone(), v3, v0.clone(), v2), {
            let a1094 = v2;
            let a1095 = 1u64;
            nat_add(a1094, a1095)?
        }, 0u64)?;
        h1093
    };
    if f328(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f231() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f232(v0: Adt16) -> R<Adt18> {
    Ok(Adt18::C0(f329(match v0.clone() {
        Adt16::C0(h1096, _, _, _, _) => {
            h1096
        }
    }, 0u64)?, f329(match v0.clone() {
        Adt16::C0(_, h1097, _, _, _) => {
            h1097
        }
    }, 0u64)?, f329(match v0.clone() {
        Adt16::C0(_, _, h1098, _, _) => {
            h1098
        }
    }, 0u64)?, f329(match v0 {
        Adt16::C0(_, _, _, h1099, _) => {
            h1099
        }
    }, 0u64)?))
}

pub fn f233(v0: Adt16) -> List<Bytes> {
    f126(match v0.clone() {
        Adt16::C0(h1100, _, _, _, _) => {
            h1100
        }
    }, f126(match v0.clone() {
        Adt16::C0(_, _, h1101, _, _) => {
            h1101
        }
    }, f126(match v0.clone() {
        Adt16::C0(_, _, _, h1102, _) => {
            h1102
        }
    }, match v0 {
        Adt16::C0(_, _, _, _, h1103) => {
            h1103
        }
    })))
}

pub fn f234(v0: Adt18, p1109: List<u64>, p1110: List<Adt20>) -> Option<u64> {
    let m1108 = (p1109, p1110);
    {
        let mut m1108 = m1108;
        loop {
            let step = {
                let (v1, v2) = m1108;
                let m1104 = v2;
                match m1104.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f330(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1105) => {
                                h1105
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1106, _) => {
                                    h1106
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1107, _) => {
                                    h1107
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: Adt16, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m1111 = v1;
    match m1111.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f331(v0.clone(), v3)?), f235(v0, v4, {
                let a1112 = v2;
                let a1113 = 1u64;
                nat_add(a1112, a1113)?
            })?))
        }
    }
}

pub fn f236(p1117: List<Bytes>, p1118: u64) -> Bytes {
    let m1116 = (p1117, p1118);
    {
        let mut m1116 = m1116;
        loop {
            let step = {
                let (v0, v1) = m1116;
                let m1114 = v0;
                match m1114.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1115 = v1;
                        if m1115 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1115 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f332(v0.clone(), v1.clone()) {
        if f332(v1, v0) {
            f333(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f238(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1119 = v0;
    match m1119.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f334(v2, v1.clone())?, f238(v3, v1)?))
        }
    }
}

pub fn f239(v0: List<Adt14>) -> List<u64> {
    let m1120 = v0;
    match m1120.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f335(v1), f239(v2))
        }
    }
}

pub fn f240(p1123: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1122 = p1123;
    {
        let mut m1122 = m1122;
        loop {
            let step = {
                let v0 = m1122;
                let m1121 = v0;
                match m1121.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f336(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1122 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(p1126: List<Bytes>, p1127: List<Bytes>) -> Option<Bytes> {
    let m1125 = (p1126, p1127);
    {
        let mut m1125 = m1125;
        loop {
            let step = {
                let (v0, v1) = m1125;
                let m1124 = v0;
                match m1124.uncons() {
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
                Ok(next) => { m1125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt23 = f337(v0.clone(), 0u64, {
        let a1128 = v0;
        length_bytes(a1128)
    }, Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt23 = {
        let a1131 = {
            let a1130 = match v1.clone() {
                Adt23::C0(_, _, h1129, _, _) => {
                    h1129
                }
            };
            length_bytes(a1130)
        };
        let a1132 = 0u64;
        if if nat_eq(a1131, a1132) {
            let m1134 = match v1.clone() {
                Adt23::C0(_, _, _, h1133, _) => {
                    h1133
                }
            };
            match m1134.uncons() {
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
            f338(v1)
        }
    };
    Ok(f339(match v4 {
        Adt23::C0(_, _, _, _, h1135) => {
            h1135
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f243(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1136 = v0;
    match m1136.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1137 = v1;
            match m1137.uncons() {
                None => {
                    f243(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f243(v2))
                }
            }
        }
    }
}

pub fn f244(v0: List<Bytes>) -> List<Bytes> {
    f121(Fn1::F340, List::<Bytes>::nil(), v0)
}

pub fn f245(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f341(v0, v1)
}

pub fn f246(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f342(f343(f344(v0))?))
}

pub fn f247(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f248(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1138 = f57(v0, v2)?;
    match m1138 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f345(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1139 = v4.clone();
            match m1139.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1141 = f93(f94({
                        let m1140 = f94(v3.clone(), f8(Str::lit("source")));
                        match m1140 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1141 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f346(f347(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f249(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1142 = v0;
    match m1142.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f348(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1146 = f44(f46(v1.clone(), 6u64, {
                    let a1144 = {
                        let a1143 = v1;
                        length_bytes(a1143)
                    };
                    let a1145 = 6u64;
                    nat_sub(a1144, a1145)
                }), 34u8)?;
                match m1146.uncons() {
                    None => {
                        f249(v2)
                    }
                    Some((v3, v4)) => {
                        let m1147 = v4;
                        match m1147.uncons() {
                            None => {
                                f249(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f249(v2)?))
                            }
                        }
                    }
                }
            } else {
                f249(v2)
            }
        }
    }
}

pub fn f250(p1150: List<Bytes>, p1151: List<Bytes>) -> List<Bytes> {
    let m1149 = (p1150, p1151);
    {
        let mut m1149 = m1149;
        loop {
            let step = {
                let (v0, v1) = m1149;
                let m1148 = v0;
                match m1148.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f252(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1152 = v1;
    match m1152.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f349(v0.clone(), v3)?;
            if f52(f350(f281(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f351(v5)?, f252(v0, v4, v2)?))
            } else {
                f252(v0, v4, v2)
            }
        }
    }
}

pub fn f253(v0: Adt1, p1156: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1155 = p1156;
    {
        let mut m1155 = m1155;
        loop {
            let step = {
                let v1 = m1155;
                let m1153 = v1;
                match m1153.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f349(v0.clone(), v3.clone())?;
                        if if f52(f350(f281(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f280({
                                let m1154 = f278(v5.clone(), f8(Str::lit("schema")));
                                match m1154 {
                                    None => {
                                        Adt2::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f351(v5)?))
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
                Ok(next) => { m1155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f254(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f151(match v0.clone() {
        Adt1::C0(_, h1157, _, _) => {
            h1157
        }
    }, f265())) {
        Ok(Adt12::C1(List::cons(f265(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f159(f153(f352(f155(match v0.clone() {
            Adt1::C0(_, h1158, _, _) => {
                h1158
            }
        }, f265())))?, v1);
        let m1159 = f56(v0.clone(), List::cons(f161(), List::cons(f216(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1159.uncons() {
            None => {
                let v5: (bool, Bytes) = f353(v0.clone(), v2)?;
                let (h1160, _) = v5;
                if h1160 {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h1161) = v5;
                        h1161
                    }))
                } else {
                    let m1162 = f125(v0.clone());
                    match m1162.uncons() {
                        None => {
                            let m1163 = f127(v0.clone())?;
                            match m1163.uncons() {
                                None => {
                                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h1164) = v5;
                                        h1164
                                    }, List::cons(f7(f153(f354(f128(v0.clone())?, f136(v0.clone(), f137(v0.clone())?)?))?), List::cons(f355(v0)?, List::<Bytes>::nil()))))))
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

pub fn f255(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f356(f97(v0, f161())?)?)))
}

pub fn f256() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f257(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1165 = v0;
    match m1165.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f258(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt19>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m1166 = v1;
    match m1166.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f126(f357(v5.clone(), v2.clone()), f358(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f359(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1168 = {
                    let a1167 = v7.clone();
                    length_list(a1167)
                };
                let a1169 = 0u64;
                if nat_eq(a1168, a1169) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f123(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f258(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f259(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1170 = v1;
    match m1170.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f222(v2.clone(), f8(Str::lit("domain"))) {
                f360(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1171 = f94(v2.clone(), f8(Str::lit("role")));
                    match m1171 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f361(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1172 = f94(v2.clone(), f8(Str::lit("origin")));
                    match m1172 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1173 = f94(v2, f8(Str::lit("evidence")));
                    match m1173 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f259(v0, v3))
            } else {
                f259(v0, v3)
            }
        }
    }
}

pub fn f260(v0: List<Adt14>) -> List<Bytes> {
    let m1174 = v0;
    match m1174.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f260(v2))
        }
    }
}

pub fn f261(v0: List<Adt14>) -> List<Bytes> {
    let m1175 = v0;
    match m1175.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f362(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f261(v2))
        }
    }
}

pub fn f262(p1181: List<Bytes>) -> R<bool> {
    let m1180 = p1181;
    {
        let mut m1180 = m1180;
        loop {
            let step = {
                let v0 = m1180;
                let m1176 = v0;
                match m1176.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1178 = {
                            let a1177 = f201(v1)?;
                            length_bytes(a1177)
                        };
                        let a1179 = 0u64;
                        if nat_eq(a1178, a1179) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: List<Bytes>) -> R<bool> {
    if f363(f292(v0.clone())?) {
        Ok(f51({
            let a1183 = {
                let a1182 = f364(f292(v0)?);
                length_list(a1182)
            };
            let a1184 = 0u64;
            nat_eq(a1183, a1184)
        }))
    } else {
        Ok(false)
    }
}

pub fn f264(v0: List<Bytes>) -> R<bool> {
    if f363(f273(v0.clone(), f8(Str::lit("requirement")))?) {
        if f363(f273(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1186 = {
                    let a1185 = f225(f364(f273(v0.clone(), f8(Str::lit("requirement")))?), f364(f273(v0, f8(Str::lit("admission")))?));
                    length_list(a1185)
                };
                let a1187 = 0u64;
                nat_eq(a1186, a1187)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f265() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f266(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f267() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f268(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1188 = v0;
    match m1188.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f201(v1)?;
            let a1190 = {
                let a1189 = v3.clone();
                length_bytes(a1189)
            };
            let a1191 = 0u64;
            if nat_eq(a1190, a1191) {
                f268(v2)
            } else {
                let m1192 = f322(v3.clone())?;
                match m1192 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f365(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1193 = f268(v2)?;
                            match m1193 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f366(v5), {
                                        let a1194 = v3;
                                        let a1195 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1194, a1195)
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

pub fn f269() -> Bytes {
    let a1196 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1196)
}

pub fn f270(p1201: List<Bytes>) -> R<Option<Bytes>> {
    let m1200 = p1201;
    {
        let mut m1200 = m1200;
        loop {
            let step = {
                let v0 = m1200;
                let m1197 = v0;
                match m1197.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1198 = f322(f201(v1)?)?;
                        match m1198 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1199 = f94(v3.clone(), f8(Str::lit("source")));
                                match m1199 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f367(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m1200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f271(v0: Bytes) -> Bytes {
    let a1206 = {
        let a1202 = Str::lit("binding ");
        utf8_encode(a1202)
    };
    let a1207 = {
        let a1204 = v0;
        let a1205 = {
            let a1203 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1203)
        };
        append_bytes(a1204, a1205)
    };
    append_bytes(a1206, a1207)
}

pub fn f272(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1208 = v0;
    match m1208.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f201(v1)?;
            let a1210 = {
                let a1209 = v3.clone();
                length_bytes(a1209)
            };
            let a1211 = 0u64;
            if nat_eq(a1210, a1211) {
                f272(v2)
            } else {
                let m1212 = f43(v3.clone())?;
                match m1212 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f368(v4.clone()) {
                            let m1213 = f272(v2)?;
                            match m1213 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f251(), List::cons(f8(Str::lit("/")), List::cons(f369(f281(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1214 = v3;
                                        let a1215 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1214, a1215)
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

pub fn f273(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m1216 = v0;
    match m1216.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f201(v2)?;
            let a1218 = {
                let a1217 = v4.clone();
                length_bytes(a1217)
            };
            let a1219 = 0u64;
            if nat_eq(a1218, a1219) {
                f273(v3, v1)
            } else {
                let m1220 = f322(v4)?;
                match m1220 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f370(v5.clone());
                        let a1222 = {
                            let a1221 = v6.clone();
                            length_bytes(a1221)
                        };
                        let a1223 = 0u64;
                        if nat_eq(a1222, a1223) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m1224 = f273(v3, v1.clone())?;
                            match m1224 {
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

pub fn f274(p1227: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1226 = p1227;
    {
        let mut m1226 = m1226;
        loop {
            let step = {
                let v0 = m1226;
                let m1225 = v0;
                match m1225.uncons() {
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
                Ok(next) => { m1226 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f276(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f101(Adt14::C3(std::rc::Rc::new(f102(v1))))?))
}

pub fn f277(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f101(Adt14::C3(std::rc::Rc::new(f371(f153(f126(v1, f150(f97(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f278(p1233: Adt2, v1: Bytes) -> Option<Adt2> {
    let m1232 = p1233;
    {
        let mut m1232 = m1232;
        loop {
            let step = {
                let v0 = m1232;
                let m1228 = v0;
                match m1228 {
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
                    Adt2::C5(r1229) => {
                        let v5 = (*r1229).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C6 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C7(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C8(v2, r1230, r1231) => {
                        let v3 = (*r1230).clone();
                        let v4 = (*r1231).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1234 = f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m1234.uncons() {
        None => {
            let m1236 = f274(f132(List::cons(Adt14::C4(std::rc::Rc::new({
                let m1235 = f372(v4.clone());
                match m1235 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1236 {
                None => {
                    f373(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f275(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f280(v0: Adt2) -> R<Bytes> {
    let a1237 = f374(v0)?;
    let a1238 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a1237, a1238))
}

pub fn f281(v0: Adt2, v1: Bytes) -> Bytes {
    let m1239 = f375(f278(v0, v1));
    match m1239 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f282(v0: List<Adt2>) -> List<Bytes> {
    let m1240 = v0;
    match m1240.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1241 = f375(Some(v1));
            match m1241 {
                None => {
                    f282(v2)
                }
                Some(v3) => {
                    List::cons(v3, f282(v2))
                }
            }
        }
    }
}

pub fn f283(v0: Option<Adt2>) -> List<Adt2> {
    let m1242 = v0;
    match m1242 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f376(v1)
        }
    }
}

pub fn f284() -> Bytes {
    f377()
}

pub fn f285(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f290(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f378(v1, v4)?)?), List::cons(f277(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f286(v0: Bytes) -> R<Bytes> {
    Ok(f379(f380(v0)?))
}

pub fn f287(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1243 = v0;
    match m1243.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f288() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f289(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f346(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f290(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f381(f158(f96(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f382(f383(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f291(p1246: List<Adt14>) -> bool {
    let m1245 = p1246;
    {
        let mut m1245 = m1245;
        loop {
            let step = {
                let v0 = m1245;
                let m1244 = v0;
                match m1244.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f149(f384(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f292(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m1247 = v0;
    match m1247.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f201(v1)?;
            let a1249 = {
                let a1248 = v3.clone();
                length_bytes(a1248)
            };
            let a1250 = 0u64;
            if nat_eq(a1249, a1250) {
                f292(v2)
            } else {
                let m1251 = f322(v3)?;
                match m1251 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f222(v4.clone(), f8(Str::lit("agent"))) {
                            if f222(v4.clone(), f8(Str::lit("at"))) {
                                if f222(v4.clone(), f8(Str::lit("quote"))) {
                                    f385(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1252 = f292(v2)?;
                            match m1252 {
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

pub fn f293(v0: u64) -> bool {
    let a1253 = v0;
    let a1254 = 127u64;
    nat_le(a1253, a1254)
}

pub fn f294(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1255, _) = v1.clone();
        h1255
    };
    let v3: u64 = {
        let (h1256, _) = {
            let (_, h1257) = v1.clone();
            h1257
        };
        h1256
    };
    let v4: List<Bytes> = {
        let (_, h1258) = {
            let (_, h1259) = v1;
            h1259
        };
        h1258
    };
    let m1260 = f211(v0.clone(), v2);
    match m1260 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1261 = v5;
            let a1262 = 10u8;
            if equal(a1261, a1262) {
                Ok(Some(({
                    let a1263 = v2;
                    let a1264 = 1u64;
                    nat_add(a1263, a1264)?
                }, ({
                    let a1265 = v2;
                    let a1266 = 1u64;
                    nat_add(a1265, a1266)?
                }, List::cons(f386(f46(v0, v3, {
                    let a1267 = v2;
                    let a1268 = v3;
                    nat_sub(a1267, a1268)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1269 = v2;
                    let a1270 = 1u64;
                    nat_add(a1269, a1270)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f295(v0: Adt14) -> List<Adt14> {
    let m1271 = v0;
    match m1271 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1272) => {
            let v3 = (*r1272).clone();
            f295(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1273, r1274) => {
            let v1 = (*r1273).clone();
            let v2 = (*r1274).clone();
            List::cons(v1, f295(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f296(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1275 = v0;
    match m1275 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r1276) => {
            let v10 = (*r1276).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r1277) => {
            let v5 = (*r1277).clone();
            Adt14::C4(std::rc::Rc::new(f296(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1278, r1279) => {
            let v11 = (*r1278).clone();
            let v12 = (*r1279).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r1280, r1281) => {
            let v3 = (*r1280).clone();
            let v4 = (*r1281).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt14::C7(v2, std::rc::Rc::new(f387(v3, v1.clone())), std::rc::Rc::new(f296(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f296(v4, v1)))
            }
        }
    }
}

pub fn f297(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1282 = 0u64;
        let a1283 = v1;
        nat_lt(a1282, a1283)
    } else {
        false
    } {
        let a1284 = Bytes::lit(&[0x2d]);
        let a1285 = f388(v1)?;
        Ok(append_bytes(a1284, a1285))
    } else {
        f388(v1)
    }
}

pub fn f298(v0: Bytes) -> R<Bytes> {
    let a1291 = {
        let a1289 = Bytes::lit(&[0x22]);
        let a1290 = {
            let (_, h1286) = {
                let (h1287, _) = f389(Fn5::F390(v0.clone()), {
                    let a1288 = v0;
                    length_bytes(a1288)
                }, (0u64, Bytes::lit(&[])))?;
                h1287
            };
            h1286
        };
        append_bytes(a1289, a1290)
    };
    let a1292 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1291, a1292))
}

pub fn f299(v0: List<Bytes>) -> Bytes {
    let a1295 = {
        let a1293 = Bytes::lit(&[0x5b]);
        let a1294 = f123(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1293, a1294)
    };
    let a1296 = Bytes::lit(&[0x5d]);
    append_bytes(a1295, a1296)
}

pub fn f300(v0: Adt14) -> R<List<Bytes>> {
    let m1297 = v0;
    match m1297 {
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
        Adt14::C6(r1298, r1299) => {
            let v7 = (*r1298).clone();
            let v8 = (*r1299).clone();
            Ok(List::cons(f193(v7)?, f300(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f301(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f391(Fn6::F392, List::<(Bytes, Bytes)>::nil(), f393(v0)?);
    let v2: List<Bytes> = f394(Fn7::F395, List::<Bytes>::nil(), v1)?;
    let a1302 = {
        let a1300 = Bytes::lit(&[0x7b]);
        let a1301 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1300, a1301)
    };
    let a1303 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1302, a1303))
}

pub fn f302(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1304 = v0;
    match m1304 {
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
        Adt14::C7(v9, r1305, r1306) => {
            let v10 = (*r1305).clone();
            let v11 = (*r1306).clone();
            Ok(List::cons((v9, f193(v10)?), f302(v11)?))
        }
    }
}

pub fn f303() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f304(v0: Adt13) -> Bytes {
    let m1307 = v0;
    match m1307 {
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

pub fn f305(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f396(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f396(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f396(f8(Str::lit("name")), f397(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f396(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f396(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f396(f8(Str::lit("statement")), f398(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f396(f8(Str::lit("media_type")), f399(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f396(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f396(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f396(f8(Str::lit("url")), f196(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f306(v0: Bytes) -> R<u64> {
    let (h1308, _) = f309(Fn3::F400(v0.clone()), {
        let a1309 = v0.clone();
        length_bytes(a1309)
    }, {
        let a1310 = v0;
        length_bytes(a1310)
    })?;
    Ok(h1308)
}

pub fn f307(v0: Bytes, v1: u64) -> R<u64> {
    let (h1311, _) = f309(Fn3::F401(v1, v0), v1, 0u64)?;
    Ok(h1311)
}

pub fn f308(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f116(v0.clone(), {
        let a1312 = v1;
        let a1313 = 1u64;
        nat_add(a1312, a1313)?
    })?;
    let v4: Bytes = {
        let a1314 = v3;
        let a1315 = 117u64;
        if nat_eq(a1314, a1315) {
            f402(v0, v1)?
        } else {
            f403(v3)
        }
    };
    let a1316 = v4.clone();
    let a1317 = Bytes::lit(&[]);
    if equal(a1316, a1317) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1318 = v3;
            let a1319 = 117u64;
            if nat_eq(a1318, a1319) {
                let a1320 = v1;
                let a1321 = 6u64;
                nat_add(a1320, a1321)?
            } else {
                let a1322 = v1;
                let a1323 = 2u64;
                nat_add(a1322, a1323)?
            }
        }, ({
            let a1324 = v2;
            let a1325 = v4;
            append_bytes(a1324, a1325)
        }, 0u64))))
    }
}

pub fn f309(v0: Fn3, p1330: u64, p1331: u64) -> R<(u64, bool)> {
    let m1329 = (p1330, p1331);
    {
        let mut m1329 = m1329;
        loop {
            let step = {
                let (v1, v2) = m1329;
                let m1326 = v1;
                if m1326 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1326 - 1;
                    let m1328 = {
                        let c1327: Fn3 = v0.clone();
                        c1327.apply(v2)?
                    };
                    match m1328 {
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
                Ok(next) => { m1329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f404(v0, v1)
}

pub fn f311(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f405(v0, v1)
}

pub fn f312(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1332 = v0.clone();
        length_bytes(a1332)
    };
    let a1333 = 0u64;
    let a1334 = v1;
    if nat_lt(a1333, a1334) {
        let a1335 = f406(v0.clone())?;
        let a1336 = v1;
        if nat_eq(a1335, a1336) {
            let a1337 = 1u64;
            let a1338 = v1;
            if nat_lt(a1337, a1338) {
                let m1341 = {
                    let a1339 = v0;
                    let a1340 = 0u64;
                    index_bytes(a1339, a1340)
                };
                match m1341 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1342 = v2;
                        let a1343 = 48u8;
                        Ok(!equal(a1342, a1343))
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

pub fn f313(v0: Fn4, p1348: u64, p1349: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1347 = (p1348, p1349);
    {
        let mut m1347 = m1347;
        loop {
            let step = {
                let (v1, v2) = m1347;
                let m1344 = v1;
                if m1344 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1344 - 1;
                    let m1346 = {
                        let c1345: Fn4 = v0.clone();
                        c1345.apply(v2)?
                    };
                    match m1346 {
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
                Ok(next) => { m1347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f314(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f407(v0, v1)
}

pub fn f315(v0: Bytes) -> R<u64> {
    let (h1350, _) = f309(Fn3::F408(v0.clone()), {
        let a1352 = {
            let a1351 = v0;
            length_bytes(a1351)
        };
        let a1353 = 1u64;
        nat_add(a1352, a1353)?
    }, 0u64)?;
    Ok(h1350)
}

pub fn f316(v0: u64) -> Bytes {
    let m1354 = v0;
    if m1354 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1354 - 1;
        let a1355 = f8(Str::lit("0"));
        let a1356 = f316(v1);
        append_bytes(a1355, a1356)
    }
}

pub fn f317(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1357 = v0;
    match m1357.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1358 = f94(v1, f8(Str::lit("std")));
            match m1358 {
                None => {
                    f317(v2)
                }
                Some(v3) => {
                    if f222(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f317(v2))
                    } else {
                        f317(v2)
                    }
                }
            }
        }
    }
}

pub fn f318(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1359 = v1;
    match m1359.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt15::C0(h1360, _, _) => {
                    h1360
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1361, _) => {
                        h1361
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f318(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt15::C0(h1362, _, _) => {
                        h1362
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1363, _) => {
                            h1363
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f318(v0, v3))
                } else {
                    f318(v0, v3)
                }
            }
        }
    }
}

pub fn f319(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt19>> {
    let m1364 = v0;
    match m1364.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v4, v5)) => {
            if if f409(v4.clone(), v2.clone()) {
                true
            } else {
                f409(v4.clone(), v3.clone())
            } {
                f319(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt19::C0({
                    let (h1365, _) = v4.clone();
                    h1365
                }, {
                    let a1368 = {
                        let a1367 = f153(f410({
                            let (h1366, _) = v4.clone();
                            h1366
                        }, v1.clone()))?;
                        length_list(a1367)
                    };
                    let a1369 = 1u64;
                    if nat_eq(a1368, a1369) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1370) = v4.clone();
                    h1370
                }), f319(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f320(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1371 = v1;
    match m1371.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1372, _) = v2.clone();
                h1372
            }, v0.clone()) {
                List::cons({
                    let (_, h1373) = v2;
                    h1373
                }, f320(v0, v3))
            } else {
                if f52({
                    let (_, h1374) = v2;
                    h1374
                }, v0.clone()) {
                    List::cons({
                        let (h1375, _) = v2;
                        h1375
                    }, f320(v0, v3))
                } else {
                    f320(v0, v3)
                }
            }
        }
    }
}

pub fn f321(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt15::C0(h1376, _, _) => {
            h1376
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1377, _) => {
                h1377
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt15::C0(_, _, h1378) => {
                    h1378
                }
            }) {
                true
            } else {
                if f222(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1379, _) => {
                            h1379
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f322(v0: Bytes) -> R<Option<Adt14>> {
    let m1380 = f43(v0)?;
    match m1380 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v1) => {
            Ok(f92(v1))
        }
    }
}

pub fn f323(v0: Adt14) -> bool {
    if f222(v0.clone(), f8(Str::lit("domain"))) {
        if f222(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f411()) {
                if f412(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f412(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f412(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f412(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f324(v0: Adt14) -> bool {
    if f222(v0.clone(), f8(Str::lit("subject"))) {
        if f222(v0.clone(), f8(Str::lit("object"))) {
            if f222(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f413())
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

pub fn f325(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f326(v0: Bytes) -> R<bool> {
    let v1: Bytes = f201(v0)?;
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
        let a1382 = {
            let a1381 = v1;
            length_bytes(a1381)
        };
        let a1383 = v2;
        nat_sub(a1382, a1383)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1384 = f230(v3.clone(), f8(Str::lit(")")))?;
        match m1384 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1385 = v4;
                    let a1386 = 1u64;
                    nat_add(a1385, a1386)?
                }, {
                    let a1390 = {
                        let a1387 = v3;
                        length_bytes(a1387)
                    };
                    let a1391 = {
                        let a1388 = v4;
                        let a1389 = 1u64;
                        nat_add(a1388, a1389)?
                    };
                    nat_sub(a1390, a1391)
                })
            }
        }
    } else {
        v3
    };
    let a1392 = 0u64;
    let a1393 = v2;
    if nat_lt(a1392, a1393) {
        f414(v5)
    } else {
        Ok(false)
    }
}

pub fn f327(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f328(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1396 = {
            let a1394 = v4;
            let a1395 = v1;
            nat_add(a1394, a1395)?
        };
        let a1397 = v3;
        if nat_lt(a1396, a1397) {
            Ok(Some({
                let a1398 = v4;
                let a1399 = 1u64;
                nat_add(a1398, a1399)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f328(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1404 = {
        let a1401 = v0;
        let a1402 = v2;
        let a1403 = {
            let a1400 = v1.clone();
            length_bytes(a1400)
        };
        slice_bytes(a1401, a1402, a1403)
    };
    match m1404 {
        None => {
            false
        }
        Some(v3) => {
            let a1405 = v3;
            let a1406 = v1;
            equal(a1405, a1406)
        }
    }
}

pub fn f329(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1407 = v0;
    match m1407.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f329(v3, {
                let a1408 = v1;
                let a1409 = 1u64;
                nat_add(a1408, a1409)?
            })?))
        }
    }
}

pub fn f330(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1410 = v2;
    match m1410 {
        Adt21::C0(v3, v4) => {
            if f336(v3, match v0.clone() {
                Adt18::C0(h1411, _, _, _) => {
                    h1411
                }
            }) {
                f415(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f336(v5, match v0 {
                Adt18::C0(_, _, h1412, _) => {
                    h1412
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f332(v7, v1) {
                f336(v8, match v0 {
                    Adt18::C0(_, _, _, h1413) => {
                        h1413
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f331(v0: Adt16, v1: Adt14) -> R<Adt21> {
    let m1414 = f94(v1.clone(), f8(Str::lit("std")));
    match m1414 {
        None => {
            let m1419 = f93(f94(v1.clone(), f8(Str::lit("req"))));
            match m1419 {
                None => {
                    let m1421 = f94(v1, f8(Str::lit("der")));
                    match m1421 {
                        None => {
                            Ok(f416(v0))
                        }
                        Some(v5) => {
                            let m1422 = f417(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1422 {
                                None => {
                                    Ok(f416(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f238(f150(f191(f94(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt16::C0(_, _, _, _, h1423) => {
                                            h1423
                                        }
                                    })?, f334(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt16::C0(_, _, _, h1424, _) => {
                                            h1424
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f334(v4, match v0 {
                        Adt16::C0(_, _, h1420, _, _) => {
                            h1420
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1415 = f93(f94(v2.clone(), f8(Str::lit("row"))));
            match m1415 {
                None => {
                    Ok(Adt21::C0(f334(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt16::C0(h1416, _, _, _, _) => {
                            h1416
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f334(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt16::C0(h1417, _, _, _, _) => {
                            h1417
                        }
                    })?, Some(f334(v3, match v0 {
                        Adt16::C0(_, h1418, _, _, _) => {
                            h1418
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f332(p1427: List<u64>, v1: List<u64>) -> bool {
    let m1426 = p1427;
    {
        let mut m1426 = m1426;
        loop {
            let step = {
                let v0 = m1426;
                let m1425 = v0;
                match m1425.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f336(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f333(p1432: List<u64>) -> bool {
    let m1431 = p1432;
    {
        let mut m1431 = m1431;
        loop {
            let step = {
                let v0 = m1431;
                let m1428 = v0;
                match m1428.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1429 = v1;
                        let a1430 = 7u64;
                        if nat_le(a1429, a1430) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f334(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1433 = f418(v0, v1.clone(), 0u64)?;
    match m1433 {
        None => {
            let a1434 = v1;
            Ok(length_list(a1434))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f335(v0: Adt14) -> u64 {
    let m1435 = f419(f94(v0, f8(Str::lit("stratum"))));
    match m1435 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f336(v0: u64, p1440: List<u64>) -> bool {
    let m1439 = p1440;
    {
        let mut m1439 = m1439;
        loop {
            let step = {
                let v1 = m1439;
                let m1436 = v1;
                match m1436.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1437 = v2;
                        let a1438 = v0;
                        if nat_eq(a1437, a1438) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: Bytes, p1498: u64, p1499: u64, p1500: Adt23) -> R<Adt23> {
    let m1497 = (p1498, (p1499, p1500));
    {
        let mut m1497 = m1497;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1497;
                let m1441 = v2;
                if m1441 == 0 {
                    Err::<(u64, (u64, Adt23)), R<Adt23>>(Ok(v3))
                } else {
                    let v4 = m1441 - 1;
                    let v5: u64 = f420(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt23::C0(_, h1442, _, _, _) => {
                            h1442
                        }
                    } {
                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                            let a1443 = v1;
                            let a1444 = 1u64;
                            nat_add(a1443, a1444)?
                        }, (v4, Adt23::C0(true, false, match v3.clone() {
                            Adt23::C0(_, _, h1445, _, _) => {
                                h1445
                            }
                        }, match v3.clone() {
                            Adt23::C0(_, _, _, h1446, _) => {
                                h1446
                            }
                        }, match v3 {
                            Adt23::C0(_, _, _, _, h1447) => {
                                h1447
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt23::C0(h1448, _, _, _, _) => {
                                h1448
                            }
                        } {
                            let a1449 = v5;
                            let a1450 = 34u64;
                            if nat_eq(a1449, a1450) {
                                let a1453 = f420(v0.clone(), {
                                    let a1451 = v1;
                                    let a1452 = 1u64;
                                    nat_add(a1451, a1452)?
                                })?;
                                let a1454 = 34u64;
                                if nat_eq(a1453, a1454) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1455 = v1;
                                        let a1456 = 1u64;
                                        nat_add(a1455, a1456)?
                                    }, (v4, Adt23::C0(true, true, f421(match v3.clone() {
                                        Adt23::C0(_, _, h1457, _, _) => {
                                            h1457
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt23::C0(_, _, _, h1458, _) => {
                                            h1458
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1459) => {
                                            h1459
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1460 = v1;
                                        let a1461 = 1u64;
                                        nat_add(a1460, a1461)?
                                    }, (v4, Adt23::C0(false, false, match v3.clone() {
                                        Adt23::C0(_, _, h1462, _, _) => {
                                            h1462
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1463, _) => {
                                            h1463
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1464) => {
                                            h1464
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1465 = v1;
                                    let a1466 = 1u64;
                                    nat_add(a1465, a1466)?
                                }, (v4, Adt23::C0(true, false, f421(match v3.clone() {
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
                            }
                        } else {
                            let a1470 = v5;
                            let a1471 = 34u64;
                            if nat_eq(a1470, a1471) {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1472 = v1;
                                    let a1473 = 1u64;
                                    nat_add(a1472, a1473)?
                                }, (v4, Adt23::C0(true, false, match v3.clone() {
                                    Adt23::C0(_, _, h1474, _, _) => {
                                        h1474
                                    }
                                }, match v3.clone() {
                                    Adt23::C0(_, _, _, h1475, _) => {
                                        h1475
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1476) => {
                                        h1476
                                    }
                                }))))
                            } else {
                                let a1477 = v5;
                                let a1478 = 44u64;
                                if nat_eq(a1477, a1478) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1479 = v1;
                                        let a1480 = 1u64;
                                        nat_add(a1479, a1480)?
                                    }, (v4, Adt23::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt23::C0(_, _, h1481, _, _) => {
                                            h1481
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1482, _) => {
                                            h1482
                                        }
                                    }), match v3 {
                                        Adt23::C0(_, _, _, _, h1483) => {
                                            h1483
                                        }
                                    }))))
                                } else {
                                    let a1484 = v5;
                                    let a1485 = 13u64;
                                    if nat_eq(a1484, a1485) {
                                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                            let a1486 = v1;
                                            let a1487 = 1u64;
                                            nat_add(a1486, a1487)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1488 = v5;
                                        let a1489 = 10u64;
                                        if nat_eq(a1488, a1489) {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1490 = v1;
                                                let a1491 = 1u64;
                                                nat_add(a1490, a1491)?
                                            }, (v4, f338(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1492 = v1;
                                                let a1493 = 1u64;
                                                nat_add(a1492, a1493)?
                                            }, (v4, Adt23::C0(false, false, f421(match v3.clone() {
                                                Adt23::C0(_, _, h1494, _, _) => {
                                                    h1494
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt23::C0(_, _, _, h1495, _) => {
                                                    h1495
                                                }
                                            }, match v3 {
                                                Adt23::C0(_, _, _, _, h1496) => {
                                                    h1496
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
                Ok(next) => { m1497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: Adt23) -> Adt23 {
    Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f422(List::cons(match v0.clone() {
        Adt23::C0(_, _, h1501, _, _) => {
            h1501
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h1502, _) => {
            h1502
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt23::C0(_, _, _, _, h1503) => {
            h1503
        }
    }))
}

pub fn f339(p1506: List<List<Bytes>>, p1507: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1505 = (p1506, p1507);
    {
        let mut m1505 = m1505;
        loop {
            let step = {
                let (v0, v1) = m1505;
                let m1504 = v0;
                match m1504.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1505 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f340(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f341(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1508 = v0.clone();
    match m1508.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1509 = v2;
            let a1510 = v1.clone();
            if equal(a1509, a1510) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f342(v0: List<(Bytes, ())>) -> List<Bytes> {
    f244(f423(Fn8::F424, List::<Bytes>::nil(), v0))
}

pub fn f343(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f425(f426(Fn9::F427, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1515 = {
        let (h1511, _) = f428(Fn10::F429, {
            let a1513 = {
                let a1512 = v0;
                length_list(a1512)
            };
            let a1514 = 1u64;
            nat_add(a1513, a1514)?
        }, v1)?;
        h1511
    };
    match m1515.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f344(v0: List<Bytes>) -> List<(Bytes, ())> {
    f430(f431(Fn11::F432, List::<(Bytes, ())>::nil(), v0))
}

pub fn f345(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1516 = f44(v1, 34u8)?;
    match m1516.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1517 = v3;
            match m1517.uncons() {
                None => {
                    Ok(f433(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f346(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f434(v0.clone(), 0u64, {
        let a1518 = v0;
        length_bytes(a1518)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f347(v0: Adt14, v1: Bytes) -> Bytes {
    if f32(f435(f94(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f348(v0: Bytes, v1: Bytes, p1531: u64, p1532: u64) -> R<bool> {
    let m1530 = (p1531, p1532);
    {
        let mut m1530 = m1530;
        loop {
            let step = {
                let (v2, v3) = m1530;
                let m1519 = v3;
                if m1519 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1519 - 1;
                    let m1522 = {
                        let a1520 = v1.clone();
                        let a1521 = v2;
                        index_bytes(a1520, a1521)
                    };
                    match m1522 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1525 = {
                                let a1523 = v0.clone();
                                let a1524 = v2;
                                index_bytes(a1523, a1524)
                            };
                            match m1525 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1526 = v5;
                                    let a1527 = v6;
                                    if equal(a1526, a1527) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1528 = v2;
                                            let a1529 = 1u64;
                                            nat_add(a1528, a1529)?
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
                Ok(next) => { m1530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1533 = f91(v0, v1)?;
    match m1533 {
        None => {
            Ok(Adt2::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f350(v0: Bytes) -> R<Bytes> {
    let a1534 = f8(Str::lit("Catalog."));
    let a1535 = f369(v0)?;
    Ok(append_bytes(a1534, a1535))
}

pub fn f351(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f369(f281(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f352(v0: List<Bytes>) -> List<Bytes> {
    let m1536 = v0;
    match m1536.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f247(f265(), v1), f352(v2))
            } else {
                f352(v2)
            }
        }
    }
}

pub fn f353(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1537 = v1;
    match m1537.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f436(v0.clone(), v2)?;
            let (h1538, _) = v4.clone();
            if h1538 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f353(v0, v3)?;
                let (h1539, _) = v5.clone();
                if h1539 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1542 = {
                            let (_, h1540) = v4;
                            h1540
                        };
                        let a1543 = {
                            let (_, h1541) = v5;
                            h1541
                        };
                        append_bytes(a1542, a1543)
                    }))
                }
            }
        }
    }
}

pub fn f354(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1544 = v1;
    match m1544.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f222(v2.clone(), f8(Str::lit("subject"))) {
                if f223(v0.clone(), v2.clone()) {
                    f354(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f354(v0, v3))
                }
            } else {
                if f360(v0.clone(), v2.clone()) {
                    f354(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f361(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f354(v0, v3))
                }
            }
        }
    }
}

pub fn f355(v0: Adt1) -> R<Bytes> {
    let a1546 = {
        let a1545 = f129(f130(f243(f257(f242(f64(v0.clone(), f216()))?)), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f132(f97(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1545)
    };
    let a1547 = 0u64;
    if nat_eq(a1546, a1547) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f356(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1548 = v0;
    match m1548.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f123(f150(f191(f94(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f437(f96(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f356(v2)?))
        }
    }
}

pub fn f357(v0: Bytes, v1: List<Adt19>) -> List<Bytes> {
    let m1549 = v1;
    match m1549.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt19::C0(h1550, _, _) => {
                    h1550
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt19::C0(_, h1551, _) => {
                        h1551
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt19::C0(_, _, h1552) => {
                        h1552
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f357(v0, v3))
            } else {
                f357(v0, v3)
            }
        }
    }
}

pub fn f358(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1553 = v1;
    match m1553.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f358(v0, v3))
            } else {
                if f32(f58(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f58(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f358(v0, v3))
                } else {
                    f358(v0, v3)
                }
            }
        }
    }
}

pub fn f359(p1558: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1557 = p1558;
    {
        let mut m1557 = m1557;
        loop {
            let step = {
                let v0 = m1557;
                let m1554 = v0;
                match m1554.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1555 = v2;
                        match m1555.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1556 = v5;
                                    match m1556.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f438(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1557 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f360(v0: Adt15, v1: Adt14) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1559, _, _) => {
            h1559
        }
    }) {
        f52(f361(v1), match v0 {
            Adt15::C0(_, h1560, _) => {
                h1560
            }
        })
    } else {
        false
    }
}

pub fn f361(v0: Adt14) -> Bytes {
    let m1561 = f94(v0, f8(Str::lit("evidence")));
    match m1561 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f362(v0: Adt14) -> Bytes {
    if f222(v0.clone(), f8(Str::lit("req"))) {
        let a1562 = f8(Str::lit("req "));
        let a1563 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1562, a1563)
    } else {
        let m1564 = f94(v0.clone(), f8(Str::lit("std")));
        match m1564 {
            None => {
                let m1567 = f94(v0, f8(Str::lit("der")));
                match m1567 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f123(f150(f191(f94(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f222(v1.clone(), f8(Str::lit("row"))) {
                    let a1565 = f8(Str::lit(" row "));
                    let a1566 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1565, a1566)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f363(v0: Option<List<Adt14>>) -> bool {
    let m1568 = v0;
    match m1568 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f364(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1569 = v0;
    match m1569 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f365(v0: Bytes, v1: Bytes) -> bool {
    let a1570 = v0;
    let a1571 = f377();
    if equal(a1570, a1571) {
        let a1575 = {
            let a1573 = {
                let a1572 = v1;
                length_bytes(a1572)
            };
            let a1574 = 0u64;
            nat_eq(a1573, a1574)
        };
        bool_not(a1575)
    } else {
        false
    }
}

pub fn f366(v0: Bytes) -> Bytes {
    f7(List::cons(f152(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f367(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1577 = v0;
    let a1578 = {
        let a1576 = Str::lit("redistributable");
        utf8_encode(a1576)
    };
    if equal(a1577, a1578) {
        f439(v1)
    } else {
        Ok(false)
    }
}

pub fn f368(v0: Adt2) -> bool {
    if f440(v0.clone(), f8(Str::lit("name"))) {
        if f441(v0.clone(), f8(Str::lit("schema"))) {
            let m1579 = f372(f282(f283(f278(v0, f8(Str::lit("citation"))))));
            match m1579 {
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

pub fn f369(v0: Bytes) -> R<Bytes> {
    f442(v0.clone(), 0u64, {
        let a1580 = v0;
        length_bytes(a1580)
    }, true, Bytes::lit(&[]))
}

pub fn f370(v0: Adt14) -> Bytes {
    if f222(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f222(v0.clone(), f8(Str::lit("id"))) {
            if f222(v0.clone(), f8(Str::lit("statement"))) {
                if f222(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f385(v0, f8(Str::lit("evidence")))
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

pub fn f371(v0: List<Bytes>) -> Adt14 {
    let m1581 = v0;
    match m1581.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f371(v2)))
        }
    }
}

pub fn f372(v0: List<Bytes>) -> Option<Adt14> {
    let m1582 = v0;
    match m1582.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1583 = v2;
                match m1583.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1584 = v2;
                    match m1584.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1585 = v6;
                                match m1585.uncons() {
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
                        let m1586 = v2;
                        match m1586.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1587 = v10;
                                match m1587.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f371(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f373(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1588 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1588.uncons() {
        None => {
            let m1589 = f372(v4);
            match m1589 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1590 = f8(Str::lit("Catalog."));
                        let a1591 = f369(v3.clone())?;
                        append_bytes(a1590, a1591)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f369(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1592 = f443(v2, v9.clone(), f369(v3.clone())?)?;
                    match m1592 {
                        Ok(v12) => {
                            Ok(Adt12::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f369(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f444(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f444(f97(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1596 = f445(f64(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v10.clone(), 4u64, {
                                    let a1594 = {
                                        let a1593 = v10;
                                        length_bytes(a1593)
                                    };
                                    let a1595 = 4u64;
                                    nat_sub(a1594, a1595)
                                }))?;
                                let a1597 = f8(Str::lit("\u{a}"));
                                append_bytes(a1596, a1597)
                            }), List::cons(f277(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f374(v0: Adt2) -> R<Bytes> {
    let m1598 = v0;
    match m1598 {
        Adt2::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt2::C1(v2, v3) => {
            f297(v2, v3)
        }
        Adt2::C2(v4, v5, v6) => {
            f446(v4, v5, v6)
        }
        Adt2::C3(v7) => {
            f447(v7)
        }
        Adt2::C4(r1599) => {
            let v8 = (*r1599).clone();
            Ok(f299(f448(v8)?))
        }
        Adt2::C5(r1600) => {
            let v9 = (*r1600).clone();
            f449(f450(v9)?)
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

pub fn f375(v0: Option<Adt2>) -> Option<Bytes> {
    let m1601 = v0;
    match m1601 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1602 = v1;
            match m1602 {
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

pub fn f376(v0: Adt2) -> List<Adt2> {
    let m1603 = v0;
    match m1603 {
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
        Adt2::C4(r1604) => {
            let v3 = (*r1604).clone();
            f376(v3)
        }
        Adt2::C5(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C6 => {
            List::<Adt2>::nil()
        }
        Adt2::C7(r1605, r1606) => {
            let v1 = (*r1605).clone();
            let v2 = (*r1606).clone();
            List::cons(v1, f376(v2))
        }
        Adt2::C8(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f377() -> Bytes {
    let a1607 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1607)
}

pub fn f378(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f94(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f289(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f289(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f289(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f289(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f289(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f289(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f289(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f289(Str::lit("url"), f435(v2.clone(), f8(Str::lit("url"))))?, List::cons(f289(Str::lit("media_type"), f435(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f289(Str::lit("license"), f435(v2.clone(), f8(Str::lit("license"))))?, List::cons(f289(Str::lit("redistribution"), f435(v2, f8(Str::lit("redistribution"))))?, List::cons(f289(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1608 = v1;
        match m1608 {
            None => {
                f7(List::cons(f289(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f289(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f289(Str::lit("artifact"), {
                    let (h1609, _) = v3;
                    h1609
                })?, List::cons(f289(Str::lit("checksum"), {
                    let (_, h1610) = v3;
                    h1610
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f451(f150(f191(f94(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f379(v0: Adt24) -> Bytes {
    let a1631 = f452(match v0.clone() {
        Adt24::C0(h1611, _, _, _, _, _, _, _) => {
            h1611
        }
    });
    let a1632 = {
        let a1629 = f452(match v0.clone() {
            Adt24::C0(_, h1612, _, _, _, _, _, _) => {
                h1612
            }
        });
        let a1630 = {
            let a1627 = f452(match v0.clone() {
                Adt24::C0(_, _, h1613, _, _, _, _, _) => {
                    h1613
                }
            });
            let a1628 = {
                let a1625 = f452(match v0.clone() {
                    Adt24::C0(_, _, _, h1614, _, _, _, _) => {
                        h1614
                    }
                });
                let a1626 = {
                    let a1623 = f452(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1615, _, _, _) => {
                            h1615
                        }
                    });
                    let a1624 = {
                        let a1621 = f452(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1616, _, _) => {
                                h1616
                            }
                        });
                        let a1622 = {
                            let a1619 = f452(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1617, _) => {
                                    h1617
                                }
                            });
                            let a1620 = f452(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1618) => {
                                    h1618
                                }
                            });
                            append_bytes(a1619, a1620)
                        };
                        append_bytes(a1621, a1622)
                    };
                    append_bytes(a1623, a1624)
                };
                append_bytes(a1625, a1626)
            };
            append_bytes(a1627, a1628)
        };
        append_bytes(a1629, a1630)
    };
    append_bytes(a1631, a1632)
}

pub fn f380(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1633 = v0.clone();
        length_bytes(a1633)
    };
    let v2: u64 = {
        let a1639 = {
            let a1636 = {
                let a1634 = v1;
                let a1635 = 72u64;
                nat_add(a1634, a1635)?
            };
            let a1637 = 64u64;
            let a1638 = 0u64;
            nat_quot(a1636, a1637, a1638)
        };
        let a1640 = 64u64;
        nat_mul(a1639, a1640)?
    };
    let (_, h1641) = f453(Fn12::F454(v0, v1, v2), {
        let a1642 = v2;
        let a1643 = 64u64;
        let a1644 = 0u64;
        nat_quot(a1642, a1643, a1644)
    }, (0u64, f455()))?;
    Ok(h1641)
}

pub fn f381(p1647: List<List<Bytes>>, v1: Bytes, p1648: List<Bytes>) -> R<List<Bytes>> {
    let m1646 = (p1647, p1648);
    {
        let mut m1646 = m1646;
        loop {
            let step = {
                let (v0, v2) = m1646;
                let m1645 = v0;
                match m1645.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f456(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f383(f457(v3), f8(Str::lit("\u{a}"))), v2)))
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

pub fn f382(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f306(v0)?))
}

pub fn f383(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1649 = v0;
    match m1649.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1650 = v3.clone();
            match m1650.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1653 = v2;
                    let a1654 = {
                        let a1651 = v1.clone();
                        let a1652 = f383(v3, v1);
                        append_bytes(a1651, a1652)
                    };
                    append_bytes(a1653, a1654)
                }
            }
        }
    }
}

pub fn f384(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1655 = f94(v0, v1);
    match m1655 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f191(f94(v3, v2))
        }
    }
}

pub fn f385(v0: Adt14, v1: Bytes) -> bool {
    let m1656 = f94(v0, v1);
    match m1656 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f386(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1658 = {
                let a1657 = v0;
                length_bytes(a1657)
            };
            let a1659 = 1u64;
            nat_sub(a1658, a1659)
        })
    } else {
        v0
    }
}

pub fn f387(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1660 = v0;
    match m1660 {
        Adt14::C0(v3) => {
            Adt14::C0(v3)
        }
        Adt14::C1(v4, v5) => {
            Adt14::C1(v4, v5)
        }
        Adt14::C2(v2) => {
            Adt14::C2(f458(f60(), v2, v1))
        }
        Adt14::C3(r1661) => {
            let v6 = (*r1661).clone();
            Adt14::C3(std::rc::Rc::new(v6))
        }
        Adt14::C4(r1662) => {
            let v7 = (*r1662).clone();
            Adt14::C4(std::rc::Rc::new(v7))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1663, r1664) => {
            let v8 = (*r1663).clone();
            let v9 = (*r1664).clone();
            Adt14::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt14::C7(v10, r1665, r1666) => {
            let v11 = (*r1665).clone();
            let v12 = (*r1666).clone();
            Adt14::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f388(v0: u64) -> R<Bytes> {
    f459(f460(), 10u64, v0)
}

pub fn f389(v0: Fn5, p1671: u64, p1672: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1670 = (p1671, p1672);
    {
        let mut m1670 = m1670;
        loop {
            let step = {
                let (v1, v2) = m1670;
                let m1667 = v1;
                if m1667 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1667 - 1;
                    let m1669 = {
                        let c1668: Fn5 = v0.clone();
                        c1668.apply(v2.clone())?
                    };
                    match m1669 {
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
                Ok(next) => { m1670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f461(v0, v1)
}

pub fn f391(v0: Fn6, p1676: List<(Bytes, Bytes)>, p1677: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1675 = (p1676, p1677);
    {
        let mut m1675 = m1675;
        loop {
            let step = {
                let (v1, v2) = m1675;
                let m1673 = v2;
                match m1673.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1674: Fn6 = v0.clone();
                            c1674.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1675 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f462(v0, v1)
}

pub fn f393(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f463(v0)
}

pub fn f394(v0: Fn7, p1681: List<Bytes>, p1682: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1680 = (p1681, p1682);
    {
        let mut m1680 = m1680;
        loop {
            let step = {
                let (v1, v2) = m1680;
                let m1678 = v2;
                match m1678.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1679: Fn7 = v0.clone();
                            c1679.apply(v1, v3)?
                        }, v4))
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

pub fn f395(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f464(v1)?, v0))
}

pub fn f396(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f298(v0)?, List::cons(f8(Str::lit(":")), List::cons(f298(v1)?, List::<Bytes>::nil())))))
}

pub fn f397(v0: Adt13) -> Bytes {
    let m1683 = v0;
    match m1683 {
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

pub fn f398(v0: Adt13) -> Bytes {
    let m1684 = v0;
    match m1684 {
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

pub fn f399(v0: Adt13) -> Bytes {
    let m1685 = v0;
    match m1685 {
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

pub fn f400(v0: Bytes, v1: u64) -> Option<u64> {
    let a1686 = 0u64;
    let a1687 = v1;
    if nat_lt(a1686, a1687) {
        if f465(v0, {
            let a1688 = v1;
            let a1689 = 1u64;
            nat_sub(a1688, a1689)
        }) {
            Some({
                let a1690 = v1;
                let a1691 = 1u64;
                nat_sub(a1690, a1691)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f401(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1692 = v2;
    let a1693 = v0;
    if nat_lt(a1692, a1693) {
        if f465(v1, v2) {
            Ok(Some({
                let a1694 = v2;
                let a1695 = 1u64;
                nat_add(a1694, a1695)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f402(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1698 = f46(v0.clone(), {
        let a1696 = v1;
        let a1697 = 2u64;
        nat_add(a1696, a1697)?
    }, 2u64);
    let a1699 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1698, a1699) {
        let m1702 = f466(f46(v0, {
            let a1700 = v1;
            let a1701 = 4u64;
            nat_add(a1700, a1701)?
        }, 2u64))?;
        match m1702 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f116(v2.clone(), 0u64)?;
                let a1703 = v3;
                let a1704 = 32u64;
                if nat_lt(a1703, a1704) {
                    if f467(v3) {
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

pub fn f403(v0: u64) -> Bytes {
    let a1705 = v0;
    let a1706 = 34u64;
    if nat_eq(a1705, a1706) {
        Bytes::lit(&[0x22])
    } else {
        let a1707 = v0;
        let a1708 = 92u64;
        if nat_eq(a1707, a1708) {
            Bytes::lit(&[0x5c])
        } else {
            let a1709 = v0;
            let a1710 = 98u64;
            if nat_eq(a1709, a1710) {
                Bytes::lit(&[0x08])
            } else {
                let a1711 = v0;
                let a1712 = 102u64;
                if nat_eq(a1711, a1712) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1713 = v0;
                    let a1714 = 110u64;
                    if nat_eq(a1713, a1714) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1715 = v0;
                        let a1716 = 114u64;
                        if nat_eq(a1715, a1716) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1717 = v0;
                            let a1718 = 116u64;
                            if nat_eq(a1717, a1718) {
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

pub fn f404(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f116(v0, v1)?;
    let a1719 = 47u64;
    let a1720 = v2;
    if if nat_lt(a1719, a1720) {
        let a1721 = v2;
        let a1722 = 58u64;
        nat_lt(a1721, a1722)
    } else {
        false
    } {
        Ok(Some({
            let a1723 = v1;
            let a1724 = 1u64;
            nat_add(a1723, a1724)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f405(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1725 = f116(v0, v1)?;
    let a1726 = 48u64;
    if nat_eq(a1725, a1726) {
        Ok(Some({
            let a1727 = v1;
            let a1728 = 1u64;
            nat_add(a1727, a1728)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f406(v0: Bytes) -> R<u64> {
    let (h1729, _) = f309(Fn3::F468(v0.clone()), {
        let a1730 = v0;
        length_bytes(a1730)
    }, 0u64)?;
    Ok(h1729)
}

pub fn f407(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1734 = {
        let a1732 = v0;
        let a1733 = {
            let (h1731, _) = v1;
            h1731
        };
        index_bytes(a1732, a1733)
    };
    match m1734 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1736 = {
                    let (h1735, _) = v1;
                    h1735
                };
                let a1737 = 1u64;
                nat_add(a1736, a1737)?
            }, {
                let a1741 = {
                    let a1739 = {
                        let (_, h1738) = v1;
                        h1738
                    };
                    let a1740 = 10u64;
                    nat_mul(a1739, a1740)?
                };
                let a1742 = f469(v2)?;
                nat_add(a1741, a1742)?
            })))
        }
    }
}

pub fn f408(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f470(v0, v1)
}

pub fn f409(v0: (Bytes, Bytes), p1745: List<(Bytes, Bytes)>) -> bool {
    let m1744 = p1745;
    {
        let mut m1744 = m1744;
        loop {
            let step = {
                let v1 = m1744;
                let m1743 = v1;
                match m1743.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f471(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1744 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1746 = v1;
    match m1746.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1747, _) = v2;
                h1747
            }, v0.clone()) {
                List::cons({
                    let (_, h1748) = v2;
                    h1748
                }, f410(v0, v3))
            } else {
                f410(v0, v3)
            }
        }
    }
}

pub fn f411() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f412(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1749 = f94(v0, v1);
    match m1749 {
        None => {
            false
        }
        Some(v3) => {
            f222(v3, v2)
        }
    }
}

pub fn f413() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f414(v0: Bytes) -> R<bool> {
    let m1750 = f230(v0.clone(), f8(Str::lit("&&")))?;
    if match m1750 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1751 = f230(v0, f8(Str::lit("||")))?;
        match m1751 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f415(v0: Adt18, v1: Option<u64>) -> bool {
    let m1752 = v1;
    match m1752 {
        None => {
            true
        }
        Some(v2) => {
            f336(v2, match v0 {
                Adt18::C0(_, h1753, _, _) => {
                    h1753
                }
            })
        }
    }
}

pub fn f416(v0: Adt16) -> Adt21 {
    Adt21::C1({
        let a1755 = match v0 {
            Adt16::C0(_, _, h1754, _, _) => {
                h1754
            }
        };
        length_list(a1755)
    })
}

pub fn f417(v0: Bytes) -> Option<Adt22> {
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

pub fn f418(v0: Bytes, p1760: List<Bytes>, p1761: u64) -> R<Option<u64>> {
    let m1759 = (p1760, p1761);
    {
        let mut m1759 = m1759;
        loop {
            let step = {
                let (v1, v2) = m1759;
                let m1756 = v1;
                match m1756.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1757 = v2;
                                let a1758 = 1u64;
                                nat_add(a1757, a1758)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1759 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f419(v0: Option<Adt14>) -> Option<u64> {
    let m1762 = v0;
    match m1762 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1763 = v1;
            match m1763 {
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

pub fn f420(v0: Bytes, v1: u64) -> R<u64> {
    let m1766 = {
        let a1764 = v0;
        let a1765 = v1;
        index_bytes(a1764, a1765)
    };
    match m1766 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f421(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1770 = {
        let a1767 = v1;
        let a1768 = v2;
        let a1769 = 1u64;
        slice_bytes(a1767, a1768, a1769)
    };
    match m1770 {
        None => {
            v0
        }
        Some(v3) => {
            let a1771 = v0;
            let a1772 = v3;
            append_bytes(a1771, a1772)
        }
    }
}

pub fn f422(p1775: List<Bytes>, p1776: List<Bytes>) -> List<Bytes> {
    let m1774 = (p1775, p1776);
    {
        let mut m1774 = m1774;
        loop {
            let step = {
                let (v0, v1) = m1774;
                let m1773 = v0;
                match m1773.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f423(v0: Fn8, p1780: List<Bytes>, p1781: List<(Bytes, ())>) -> List<Bytes> {
    let m1779 = (p1780, p1781);
    {
        let mut m1779 = m1779;
        loop {
            let step = {
                let (v1, v2) = m1779;
                let m1777 = v2;
                match m1777.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1778: Fn8 = v0.clone();
                            c1778.apply(v1, v3)
                        }, v4))
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

pub fn f424(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1782, _) = v1;
        h1782
    }, v0)
}

pub fn f425(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f472(Fn13::F473, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f426(v0: Fn9, p1786: List<List<(Bytes, ())>>, p1787: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1785 = (p1786, p1787);
    {
        let mut m1785 = m1785;
        loop {
            let step = {
                let (v1, v2) = m1785;
                let m1783 = v2;
                match m1783.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1784: Fn9 = v0.clone();
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

pub fn f427(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f428(v0: Fn10, p1792: u64, p1793: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1791 = (p1792, p1793);
    {
        let mut m1791 = m1791;
        loop {
            let step = {
                let (v1, v2) = m1791;
                let m1788 = v1;
                if m1788 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1788 - 1;
                    let m1790 = {
                        let c1789: Fn10 = v0.clone();
                        c1789.apply(v2.clone())?
                    };
                    match m1790 {
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
                Ok(next) => { m1791 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f429(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f474(v0)
}

pub fn f430(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f475(Fn14::F476, List::<(Bytes, ())>::nil(), v0)
}

pub fn f431(v0: Fn11, p1797: List<(Bytes, ())>, p1798: List<Bytes>) -> List<(Bytes, ())> {
    let m1796 = (p1797, p1798);
    {
        let mut m1796 = m1796;
        loop {
            let step = {
                let (v1, v2) = m1796;
                let m1794 = v2;
                match m1794.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1795: Fn11 = v0.clone();
                            c1795.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f433(p1803: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1802 = p1803;
    {
        let mut m1802 = m1802;
        loop {
            let step = {
                let v0 = m1802;
                let m1799 = v0;
                match m1799.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1800, _) = v2;
                            h1800
                        }) {
                            let (_, h1801) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1801)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1802 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Bytes, p1829: u64, p1830: u64, p1831: Bytes) -> R<Bytes> {
    let m1828 = (p1829, (p1830, p1831));
    {
        let mut m1828 = m1828;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1828;
                let m1804 = v2;
                if m1804 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1804 - 1;
                    let m1807 = {
                        let a1805 = v0.clone();
                        let a1806 = v1;
                        index_bytes(a1805, a1806)
                    };
                    match m1807 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f206({
                                let a1808 = v6;
                                let a1809 = 34u64;
                                nat_eq(a1808, a1809)
                            }, {
                                let a1810 = v6;
                                let a1811 = 92u64;
                                nat_eq(a1810, a1811)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1812 = v1;
                                    let a1813 = 1u64;
                                    nat_add(a1812, a1813)?
                                }, (v4, {
                                    let a1816 = v3;
                                    let a1817 = {
                                        let a1814 = f8(Str::lit("\\"));
                                        let a1815 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1814, a1815)
                                    };
                                    append_bytes(a1816, a1817)
                                })))
                            } else {
                                let a1818 = v6;
                                let a1819 = 10u64;
                                if nat_eq(a1818, a1819) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1820 = v1;
                                        let a1821 = 1u64;
                                        nat_add(a1820, a1821)?
                                    }, (v4, {
                                        let a1822 = v3;
                                        let a1823 = f8(Str::lit("\\n"));
                                        append_bytes(a1822, a1823)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1824 = v1;
                                        let a1825 = 1u64;
                                        nat_add(a1824, a1825)?
                                    }, (v4, {
                                        let a1826 = v3;
                                        let a1827 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1826, a1827)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f435(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1832 = v0;
    match m1832 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f436(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
    let m1833 = f292(v2.clone())?;
    match m1833 {
        None => {
            f478(v0, v1, v2)
        }
        Some(v3) => {
            f477(v0, v1, v3)
        }
    }
}

pub fn f437(v0: List<Bytes>) -> List<Bytes> {
    let m1834 = v0;
    match m1834.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1836 = {
                    let a1835 = v1.clone();
                    length_bytes(a1835)
                };
                let a1837 = 0u64;
                if nat_eq(a1836, a1837) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f437(v2))
        }
    }
}

pub fn f438(v0: List<Bytes>) -> Bytes {
    let m1838 = v0;
    match m1838.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f439(v0: Bytes) -> R<bool> {
    let m1840 = {
        let a1839 = v0.clone();
        length_bytes(a1839)
    };
    if m1840 == 0 {
        Ok(true)
    } else {
        let v1 = m1840 - 1;
        let m1843 = {
            let a1841 = v0;
            let a1842 = v1;
            index_bytes(a1841, a1842)
        };
        match m1843 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f479({
                    let v3: u8 = v2;
                    let a1888 = {
                        let a1864 = {
                            let a1852 = {
                                let a1846 = {
                                    let a1844 = v3;
                                    let a1845 = 128u8;
                                    fixed_u8::bit_and(a1844, a1845)
                                };
                                let a1847 = 128u8;
                                if equal(a1846, a1847) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1853 = {
                                let a1850 = {
                                    let a1848 = v3;
                                    let a1849 = 64u8;
                                    fixed_u8::bit_and(a1848, a1849)
                                };
                                let a1851 = 64u8;
                                if equal(a1850, a1851) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1852, a1853)?
                        };
                        let a1865 = {
                            let a1862 = {
                                let a1856 = {
                                    let a1854 = v3;
                                    let a1855 = 32u8;
                                    fixed_u8::bit_and(a1854, a1855)
                                };
                                let a1857 = 32u8;
                                if equal(a1856, a1857) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1863 = {
                                let a1860 = {
                                    let a1858 = v3;
                                    let a1859 = 16u8;
                                    fixed_u8::bit_and(a1858, a1859)
                                };
                                let a1861 = 16u8;
                                if equal(a1860, a1861) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1862, a1863)?
                        };
                        nat_add(a1864, a1865)?
                    };
                    let a1889 = {
                        let a1886 = {
                            let a1874 = {
                                let a1868 = {
                                    let a1866 = v3;
                                    let a1867 = 8u8;
                                    fixed_u8::bit_and(a1866, a1867)
                                };
                                let a1869 = 8u8;
                                if equal(a1868, a1869) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1875 = {
                                let a1872 = {
                                    let a1870 = v3;
                                    let a1871 = 4u8;
                                    fixed_u8::bit_and(a1870, a1871)
                                };
                                let a1873 = 4u8;
                                if equal(a1872, a1873) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1874, a1875)?
                        };
                        let a1887 = {
                            let a1884 = {
                                let a1878 = {
                                    let a1876 = v3;
                                    let a1877 = 2u8;
                                    fixed_u8::bit_and(a1876, a1877)
                                };
                                let a1879 = 2u8;
                                if equal(a1878, a1879) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1885 = {
                                let a1882 = {
                                    let a1880 = v3;
                                    let a1881 = 1u8;
                                    fixed_u8::bit_and(a1880, a1881)
                                };
                                let a1883 = 1u8;
                                if equal(a1882, a1883) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1884, a1885)?
                        };
                        nat_add(a1886, a1887)?
                    };
                    nat_add(a1888, a1889)?
                }))
            }
        }
    }
}

pub fn f440(v0: Adt2, v1: Bytes) -> bool {
    let a1891 = {
        let a1890 = f281(v0, v1);
        length_bytes(a1890)
    };
    let a1892 = 0u64;
    !nat_eq(a1891, a1892)
}

pub fn f441(v0: Adt2, v1: Bytes) -> bool {
    let m1893 = f278(v0, v1);
    match m1893 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f442(v0: Bytes, p1914: u64, p1915: u64, p1916: bool, p1917: Bytes) -> R<Bytes> {
    let m1913 = (p1914, (p1915, (p1916, p1917)));
    {
        let mut m1913 = m1913;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1913;
                let m1894 = v2;
                if m1894 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1894 - 1;
                    let m1897 = {
                        let a1895 = v0.clone();
                        let a1896 = v1;
                        index_bytes(a1895, a1896)
                    };
                    match m1897 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f480(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1898 = v1;
                                    let a1899 = 1u64;
                                    nat_add(a1898, a1899)?
                                }, (v5, (true, v4))))
                            } else {
                                if f118(f481(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1900 = v1;
                                        let a1901 = 1u64;
                                        nat_add(a1900, a1901)?
                                    }, (v5, (false, {
                                        let a1905 = v4;
                                        let a1906 = f46({
                                            let a1902 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1902)
                                        }, {
                                            let a1903 = v7;
                                            let a1904 = 97u64;
                                            nat_sub(a1903, a1904)
                                        }, 1u64);
                                        append_bytes(a1905, a1906)
                                    }))))
                                } else {
                                    if f206(f481(v7), f206(f482(v7), f483(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1907 = v1;
                                            let a1908 = 1u64;
                                            nat_add(a1907, a1908)?
                                        }, (v5, (false, {
                                            let a1909 = v4;
                                            let a1910 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1909, a1910)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1911 = v1;
                                            let a1912 = 1u64;
                                            nat_add(a1911, a1912)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
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

pub fn f443(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f484(f485(f278(v0.clone(), f8(Str::lit("$defs")))), f485(f278(v0.clone(), f8(Str::lit("definitions")))));
    let m1918 = f486(4096u64, v3.clone(), List::<Bytes>::nil(), f487(v3)?)?;
    match m1918 {
        Ok(v5) => {
            let m1919 = f488(4096u64, f489(f490(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1919 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f383(f250(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f444(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1920 = v0;
    match m1920.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f32(f58(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f491(v5, v1, v2))
            } else {
                List::cons(v4, f444(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f445(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f383(f492(f96(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f446(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f388(v1)?;
    let v4: u64 = {
        let a1921 = v3.clone();
        length_bytes(a1921)
    };
    let v5: Bytes = {
        let a1922 = v2;
        let a1923 = v4;
        if nat_lt(a1922, a1923) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1924 = v4;
                let a1925 = v2;
                nat_sub(a1924, a1925)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1926 = v4;
                let a1927 = v2;
                nat_sub(a1926, a1927)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f316({
                let a1928 = v2;
                let a1929 = v4;
                nat_sub(a1928, a1929)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1930 = f8(Str::lit("-"));
        let a1931 = v5;
        Ok(append_bytes(a1930, a1931))
    } else {
        Ok(v5)
    }
}

pub fn f447(v0: Bytes) -> R<Bytes> {
    let a1936 = f203(v0.clone(), 0u64, {
        let a1933 = {
            let a1932 = v0.clone();
            length_bytes(a1932)
        };
        let a1934 = 1u64;
        nat_add(a1933, a1934)?
    })?;
    let a1937 = {
        let a1935 = v0.clone();
        length_bytes(a1935)
    };
    if nat_eq(a1936, a1937) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1943 = {
            let a1941 = Bytes::lit(&[0x22]);
            let a1942 = f493(v0.clone(), 0u64, {
                let a1939 = {
                    let a1938 = v0;
                    length_bytes(a1938)
                };
                let a1940 = 1u64;
                nat_add(a1939, a1940)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1941, a1942)
        };
        let a1944 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1943, a1944))
    }
}

pub fn f448(v0: Adt2) -> R<List<Bytes>> {
    let m1945 = v0;
    match m1945 {
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
        Adt2::C7(r1946, r1947) => {
            let v10 = (*r1946).clone();
            let v11 = (*r1947).clone();
            Ok(List::cons(f374(v10)?, f448(v11)?))
        }
        Adt2::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f449(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f391(Fn6::F494, List::<(Bytes, Bytes)>::nil(), f393(v0)?);
    let v2: List<Bytes> = f394(Fn7::F495, List::<Bytes>::nil(), v1)?;
    let a1950 = {
        let a1948 = Bytes::lit(&[0x7b]);
        let a1949 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1948, a1949)
    };
    let a1951 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1950, a1951))
}

pub fn f450(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1952 = v0;
    match m1952 {
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
        Adt2::C8(v12, r1953, r1954) => {
            let v13 = (*r1953).clone();
            let v14 = (*r1954).clone();
            Ok(List::cons((v12, f374(v13)?), f450(v14)?))
        }
    }
}

pub fn f451(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f383(f496(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f452(v0: u32) -> Bytes {
    let a1983 = f497({
        let a1955 = f498(v0, 28u32);
        let a1956 = 15u32;
        fixed_u32::bit_and(a1955, a1956)
    });
    let a1984 = {
        let a1981 = f497({
            let a1957 = f498(v0, 24u32);
            let a1958 = 15u32;
            fixed_u32::bit_and(a1957, a1958)
        });
        let a1982 = {
            let a1979 = f497({
                let a1959 = f498(v0, 20u32);
                let a1960 = 15u32;
                fixed_u32::bit_and(a1959, a1960)
            });
            let a1980 = {
                let a1977 = f497({
                    let a1961 = f498(v0, 16u32);
                    let a1962 = 15u32;
                    fixed_u32::bit_and(a1961, a1962)
                });
                let a1978 = {
                    let a1975 = f497({
                        let a1963 = f498(v0, 12u32);
                        let a1964 = 15u32;
                        fixed_u32::bit_and(a1963, a1964)
                    });
                    let a1976 = {
                        let a1973 = f497({
                            let a1965 = f498(v0, 8u32);
                            let a1966 = 15u32;
                            fixed_u32::bit_and(a1965, a1966)
                        });
                        let a1974 = {
                            let a1971 = f497({
                                let a1967 = f498(v0, 4u32);
                                let a1968 = 15u32;
                                fixed_u32::bit_and(a1967, a1968)
                            });
                            let a1972 = f497({
                                let a1969 = v0;
                                let a1970 = 15u32;
                                fixed_u32::bit_and(a1969, a1970)
                            });
                            append_bytes(a1971, a1972)
                        };
                        append_bytes(a1973, a1974)
                    };
                    append_bytes(a1975, a1976)
                };
                append_bytes(a1977, a1978)
            };
            append_bytes(a1979, a1980)
        };
        append_bytes(a1981, a1982)
    };
    append_bytes(a1983, a1984)
}

pub fn f453(v0: Fn12, p1988: u64, p1989: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1987 = (p1988, p1989);
    {
        let mut m1987 = m1987;
        loop {
            let step = {
                let (v1, v2) = m1987;
                let m1985 = v1;
                if m1985 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1985 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1986: Fn12 = v0.clone();
                        c1986.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1987 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1991 = {
            let (h1990, _) = v3.clone();
            h1990
        };
        let a1992 = 64u64;
        nat_add(a1991, a1992)?
    }, f499({
        let (_, h1993) = v3;
        h1993
    }, f500(v0, v1, v2, {
        let (h1994, _) = v3;
        h1994
    })?)?))
}

pub fn f455() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f456(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1995 = f44(v1, 34u8)?;
    match m1995.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1996 = v3;
            match m1996.uncons() {
                None => {
                    Ok(f501(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f457(v0: List<Bytes>) -> List<Bytes> {
    let m1997 = v0;
    match m1997.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f457(v2)
            } else {
                List::cons(v1, f457(v2))
            }
        }
    }
}

pub fn f458(p2000: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m1999 = p2000;
    {
        let mut m1999 = m1999;
        loop {
            let step = {
                let v0 = m1999;
                let m1998 = v0;
                match m1998.uncons() {
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
                Ok(next) => { m1999 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2001 = v2;
    let a2002 = 0u64;
    if nat_eq(a2001, a2002) {
        Ok(f502(v0, 0u64))
    } else {
        let (_, h2003) = {
            let (h2004, _) = f389(Fn5::F503(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2004
        };
        Ok(h2003)
    }
}

pub fn f460() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f461(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2008 = {
        let a2006 = v0.clone();
        let a2007 = {
            let (h2005, _) = v1.clone();
            h2005
        };
        index_bytes(a2006, a2007)
    };
    match m2008 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2010 = {
                    let (h2009, _) = v1.clone();
                    h2009
                };
                let a2011 = 1u64;
                nat_add(a2010, a2011)?
            }, {
                let a2014 = {
                    let (_, h2012) = v1;
                    h2012
                };
                let a2015 = f504(v0, {
                    let (h2013, _) = v1;
                    h2013
                }, v2)?;
                append_bytes(a2014, a2015)
            })))
        }
    }
}

pub fn f462(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2016 = v0.clone();
    match m2016.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2019 = {
                let (h2017, _) = v2;
                h2017
            };
            let a2020 = {
                let (h2018, _) = v1.clone();
                h2018
            };
            if equal(a2019, a2020) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f463(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f505(f506(Fn15::F507, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2025 = {
        let (h2021, _) = f508(Fn16::F509, {
            let a2023 = {
                let a2022 = v0;
                length_list(a2022)
            };
            let a2024 = 1u64;
            nat_add(a2023, a2024)?
        }, v1)?;
        h2021
    };
    match m2025.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f464(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2030 = {
        let a2027 = f298({
            let (h2026, _) = v0;
            h2026
        })?;
        let a2028 = Bytes::lit(&[0x3a]);
        append_bytes(a2027, a2028)
    };
    let a2031 = {
        let (_, h2029) = v0;
        h2029
    };
    Ok(append_bytes(a2030, a2031))
}

pub fn f465(v0: Bytes, v1: u64) -> bool {
    let m2032 = f211(v0, v1);
    match m2032 {
        None => {
            false
        }
        Some(v2) => {
            f510(v2)
        }
    }
}

pub fn f466(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2033 = v0.clone();
        length_bytes(a2033)
    };
    let a2037 = {
        let a2034 = v1;
        let a2035 = 2u64;
        let a2036 = 0u64;
        nat_rem(a2034, a2035, a2036)
    };
    let a2038 = 0u64;
    if nat_eq(a2037, a2038) {
        let (_, h2039) = {
            let (h2040, _) = f511(Fn17::F512(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2040
        };
        Ok(h2039)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f467(v0: u64) -> bool {
    let a2041 = v0;
    let a2042 = 8u64;
    if nat_eq(a2041, a2042) {
        true
    } else {
        let a2043 = v0;
        let a2044 = 9u64;
        if nat_eq(a2043, a2044) {
            true
        } else {
            let a2045 = v0;
            let a2046 = 10u64;
            if nat_eq(a2045, a2046) {
                true
            } else {
                let a2047 = v0;
                let a2048 = 12u64;
                if nat_eq(a2047, a2048) {
                    true
                } else {
                    let a2049 = v0;
                    let a2050 = 13u64;
                    nat_eq(a2049, a2050)
                }
            }
        }
    }
}

pub fn f468(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2053 = {
        let a2051 = v0;
        let a2052 = v1;
        index_bytes(a2051, a2052)
    };
    match m2053 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f513(v2)? {
                Ok(Some({
                    let a2054 = v1;
                    let a2055 = 1u64;
                    nat_add(a2054, a2055)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f469(v0: u8) -> R<u64> {
    let a2102 = {
        let v1: u8 = v0;
        let a2100 = {
            let a2076 = {
                let a2064 = {
                    let a2058 = {
                        let a2056 = v1;
                        let a2057 = 128u8;
                        fixed_u8::bit_and(a2056, a2057)
                    };
                    let a2059 = 128u8;
                    if equal(a2058, a2059) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2065 = {
                    let a2062 = {
                        let a2060 = v1;
                        let a2061 = 64u8;
                        fixed_u8::bit_and(a2060, a2061)
                    };
                    let a2063 = 64u8;
                    if equal(a2062, a2063) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2064, a2065)?
            };
            let a2077 = {
                let a2074 = {
                    let a2068 = {
                        let a2066 = v1;
                        let a2067 = 32u8;
                        fixed_u8::bit_and(a2066, a2067)
                    };
                    let a2069 = 32u8;
                    if equal(a2068, a2069) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2075 = {
                    let a2072 = {
                        let a2070 = v1;
                        let a2071 = 16u8;
                        fixed_u8::bit_and(a2070, a2071)
                    };
                    let a2073 = 16u8;
                    if equal(a2072, a2073) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2074, a2075)?
            };
            nat_add(a2076, a2077)?
        };
        let a2101 = {
            let a2098 = {
                let a2086 = {
                    let a2080 = {
                        let a2078 = v1;
                        let a2079 = 8u8;
                        fixed_u8::bit_and(a2078, a2079)
                    };
                    let a2081 = 8u8;
                    if equal(a2080, a2081) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2087 = {
                    let a2084 = {
                        let a2082 = v1;
                        let a2083 = 4u8;
                        fixed_u8::bit_and(a2082, a2083)
                    };
                    let a2085 = 4u8;
                    if equal(a2084, a2085) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2086, a2087)?
            };
            let a2099 = {
                let a2096 = {
                    let a2090 = {
                        let a2088 = v1;
                        let a2089 = 2u8;
                        fixed_u8::bit_and(a2088, a2089)
                    };
                    let a2091 = 2u8;
                    if equal(a2090, a2091) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2097 = {
                    let a2094 = {
                        let a2092 = v1;
                        let a2093 = 1u8;
                        fixed_u8::bit_and(a2092, a2093)
                    };
                    let a2095 = 1u8;
                    if equal(a2094, a2095) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2096, a2097)?
            };
            nat_add(a2098, a2099)?
        };
        nat_add(a2100, a2101)?
    };
    let a2103 = 48u64;
    Ok(nat_sub(a2102, a2103))
}

pub fn f470(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2104 = v0.clone();
        length_bytes(a2104)
    };
    let a2105 = v1;
    let a2106 = v2;
    if if nat_lt(a2105, a2106) {
        let a2111 = f116(v0, {
            let a2109 = v2;
            let a2110 = {
                let a2107 = v1;
                let a2108 = 1u64;
                nat_add(a2107, a2108)?
            };
            nat_sub(a2109, a2110)
        })?;
        let a2112 = 48u64;
        nat_eq(a2111, a2112)
    } else {
        false
    } {
        Ok(Some({
            let a2113 = v1;
            let a2114 = 1u64;
            nat_add(a2113, a2114)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f471(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f118(f32({
        let (h2115, _) = v0;
        h2115
    }, {
        let (h2116, _) = v1;
        h2116
    }), f32({
        let (_, h2117) = v0;
        h2117
    }, {
        let (_, h2118) = v1;
        h2118
    }))
}

pub fn f472(v0: Fn13, p2122: List<List<(Bytes, ())>>, p2123: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2121 = (p2122, p2123);
    {
        let mut m2121 = m2121;
        loop {
            let step = {
                let (v1, v2) = m2121;
                let m2119 = v2;
                match m2119.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2120: Fn13 = v0.clone();
                            c2120.apply(v1, v3)
                        }, v4))
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

pub fn f473(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f474(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2125 = {
        let a2124 = v0.clone();
        length_list(a2124)
    };
    let a2126 = 1u64;
    if nat_le(a2125, a2126) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f514(v0)?))
    }
}

pub fn f475(v0: Fn14, p2130: List<(Bytes, ())>, p2131: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2129 = (p2130, p2131);
    {
        let mut m2129 = m2129;
        loop {
            let step = {
                let (v1, v2) = m2129;
                let m2127 = v2;
                match m2127.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2128: Fn14 = v0.clone();
                            c2128.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f477(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f149(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f149(f97(v0, f161())?, f8(Str::lit("at"))));
    let a2133 = {
        let a2132 = v4.clone();
        length_list(a2132)
    };
    let a2134 = 0u64;
    if nat_eq(a2133, a2134) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2137 = {
            let a2135 = v4;
            length_list(a2135)
        };
        let a2138 = {
            let a2136 = v3;
            length_list(a2136)
        };
        if nat_eq(a2137, a2138) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f515(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f478(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f364(f273(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f364(f273(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f126(f149(v3.clone(), f8(Str::lit("id"))), f149(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f149(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f126(v6.clone(), f149(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2140 = {
        let a2139 = v7.clone();
        length_list(a2139)
    };
    let a2141 = 0u64;
    if nat_eq(a2140, a2141) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2144 = {
                let a2142 = v7;
                length_list(a2142)
            };
            let a2145 = {
                let a2143 = v5;
                length_list(a2143)
            };
            nat_eq(a2144, a2145)
        }) {
            Ok((false, f515(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f153(f126(f62(f516(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f517(v4), f126(v6, f149(v3, f8(Str::lit("id")))))))?;
            let a2147 = {
                let a2146 = v8.clone();
                length_list(a2146)
            };
            let a2148 = 0u64;
            if nat_eq(a2147, a2148) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f515(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f123(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f479(v0: u64) -> bool {
    let a2149 = v0;
    let a2150 = 47u64;
    nat_eq(a2149, a2150)
}

pub fn f480(v0: u64) -> bool {
    f206({
        let a2151 = v0;
        let a2152 = 95u64;
        nat_eq(a2151, a2152)
    }, f206({
        let a2153 = v0;
        let a2154 = 45u64;
        nat_eq(a2153, a2154)
    }, f206({
        let a2155 = v0;
        let a2156 = 32u64;
        nat_eq(a2155, a2156)
    }, f206({
        let a2157 = v0;
        let a2158 = 46u64;
        nat_eq(a2157, a2158)
    }, {
        let a2159 = v0;
        let a2160 = 47u64;
        nat_eq(a2159, a2160)
    }))))
}

pub fn f481(v0: u64) -> bool {
    f118({
        let a2161 = 97u64;
        let a2162 = v0;
        nat_le(a2161, a2162)
    }, {
        let a2163 = v0;
        let a2164 = 122u64;
        nat_le(a2163, a2164)
    })
}

pub fn f482(v0: u64) -> bool {
    f118({
        let a2165 = 65u64;
        let a2166 = v0;
        nat_le(a2165, a2166)
    }, {
        let a2167 = v0;
        let a2168 = 90u64;
        nat_le(a2167, a2168)
    })
}

pub fn f483(v0: u64) -> bool {
    f118({
        let a2169 = 48u64;
        let a2170 = v0;
        nat_le(a2169, a2170)
    }, {
        let a2171 = v0;
        let a2172 = 57u64;
        nat_le(a2171, a2172)
    })
}

pub fn f484(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m2173 = v0;
    match m2173.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f484(v3, v1))
        }
    }
}

pub fn f485(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m2174 = v0;
    match m2174 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f518(v1)
        }
    }
}

pub fn f486(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m2175 = v0;
    if m2175 == 0 {
        let m2176 = v1;
        match m2176.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2175 - 1;
        let m2177 = v1.clone();
        match m2177.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m2178 = f519(v1.clone(), v2.clone(), v3.clone())?;
                match m2178 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f383(f487(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2181 = f486(v6, f520(v1, {
                            let (h2179, _) = v9.clone();
                            h2179
                        }), List::cons(f521({
                            let (h2180, _) = v9.clone();
                            h2180
                        })?, v2), v3)?;
                        match m2181 {
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

pub fn f487(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m2182 = v0;
    match m2182.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f521({
                let (h2183, _) = v1;
                h2183
            })?, f487(v2)?))
        }
    }
}

pub fn f488(p2189: u64, p2190: List<Adt25>, p2191: List<Bytes>, p2192: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2188 = (p2189, (p2190, (p2191, p2192)));
    {
        let mut m2188 = m2188;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2188;
                let m2184 = v0;
                if m2184 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2184 - 1;
                    let m2185 = v1;
                    match m2185.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2186 = v5;
                            match m2186 {
                                Adt25::C0(v8, v9) => {
                                    if f32(v8.clone(), f522()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f52(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2187 = f524(v8.clone(), v9)?;
                                            match m2187 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f489(f525(v11.clone()), List::cons(Adt25::C1(f526(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f52(f522(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f522(), v2), List::cons(f523(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f489(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2193 = v0;
    match m2193.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f489(v3, v1))
        }
    }
}

pub fn f490(v0: List<(Bytes, Adt2)>) -> R<List<Adt25>> {
    let m2194 = v0;
    match m2194.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f521({
                let (h2195, _) = v1;
                h2195
            })?, {
                let (_, h2196) = v1;
                h2196
            }), f490(v2)?))
        }
    }
}

pub fn f491(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2197 = v0;
    match m2197.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f32(f58(v3.clone(), v1.clone()), v2.clone()) {
                f491(v4, v1, v2)
            } else {
                List::cons(v3, f491(v4, v1, v2))
            }
        }
    }
}

pub fn f492(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2198 = v0;
    match m2198.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f383(f496(f153(List::cons(v1, f527(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f492(v3, v1)?))
            }
        }
    }
}

pub fn f493(v0: Bytes, p2217: u64, p2218: u64, p2219: Bytes) -> R<Bytes> {
    let m2216 = (p2217, (p2218, p2219));
    {
        let mut m2216 = m2216;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2216;
                let m2199 = v2;
                if m2199 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2199 - 1;
                    let v5: u64 = f203(v0.clone(), v1, {
                        let a2203 = {
                            let a2201 = {
                                let a2200 = v0.clone();
                                length_bytes(a2200)
                            };
                            let a2202 = v1;
                            nat_sub(a2201, a2202)
                        };
                        let a2204 = 1u64;
                        nat_add(a2203, a2204)?
                    })?;
                    let v6: Bytes = {
                        let a2207 = v3;
                        let a2208 = f46(v0.clone(), v1, {
                            let a2205 = v5;
                            let a2206 = v1;
                            nat_sub(a2205, a2206)
                        });
                        append_bytes(a2207, a2208)
                    };
                    let m2211 = {
                        let a2209 = v0.clone();
                        let a2210 = v5;
                        index_bytes(a2209, a2210)
                    };
                    match m2211 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2212 = v5;
                                let a2213 = 1u64;
                                nat_add(a2212, a2213)?
                            }, (v4, {
                                let a2214 = v6;
                                let a2215 = f504(v0.clone(), v5, v7)?;
                                append_bytes(a2214, a2215)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2216 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f462(v0, v1)
}

pub fn f495(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f528(v1)?, v0))
}

pub fn f496(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2220 = v0;
    match m2220.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f346(v1)?, f496(v2)?))
        }
    }
}

pub fn f497(v0: u32) -> Bytes {
    let a2223 = {
        let a2221 = v0;
        let a2222 = 8u32;
        fixed_u32::bit_and(a2221, a2222)
    };
    let a2224 = 0u32;
    if equal(a2223, a2224) {
        let a2227 = {
            let a2225 = v0;
            let a2226 = 4u32;
            fixed_u32::bit_and(a2225, a2226)
        };
        let a2228 = 0u32;
        if equal(a2227, a2228) {
            let a2231 = {
                let a2229 = v0;
                let a2230 = 2u32;
                fixed_u32::bit_and(a2229, a2230)
            };
            let a2232 = 0u32;
            if equal(a2231, a2232) {
                let a2235 = {
                    let a2233 = v0;
                    let a2234 = 1u32;
                    fixed_u32::bit_and(a2233, a2234)
                };
                let a2236 = 0u32;
                if equal(a2235, a2236) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2239 = {
                    let a2237 = v0;
                    let a2238 = 1u32;
                    fixed_u32::bit_and(a2237, a2238)
                };
                let a2240 = 0u32;
                if equal(a2239, a2240) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2243 = {
                let a2241 = v0;
                let a2242 = 2u32;
                fixed_u32::bit_and(a2241, a2242)
            };
            let a2244 = 0u32;
            if equal(a2243, a2244) {
                let a2247 = {
                    let a2245 = v0;
                    let a2246 = 1u32;
                    fixed_u32::bit_and(a2245, a2246)
                };
                let a2248 = 0u32;
                if equal(a2247, a2248) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2251 = {
                    let a2249 = v0;
                    let a2250 = 1u32;
                    fixed_u32::bit_and(a2249, a2250)
                };
                let a2252 = 0u32;
                if equal(a2251, a2252) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2255 = {
            let a2253 = v0;
            let a2254 = 4u32;
            fixed_u32::bit_and(a2253, a2254)
        };
        let a2256 = 0u32;
        if equal(a2255, a2256) {
            let a2259 = {
                let a2257 = v0;
                let a2258 = 2u32;
                fixed_u32::bit_and(a2257, a2258)
            };
            let a2260 = 0u32;
            if equal(a2259, a2260) {
                let a2263 = {
                    let a2261 = v0;
                    let a2262 = 1u32;
                    fixed_u32::bit_and(a2261, a2262)
                };
                let a2264 = 0u32;
                if equal(a2263, a2264) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2267 = {
                    let a2265 = v0;
                    let a2266 = 1u32;
                    fixed_u32::bit_and(a2265, a2266)
                };
                let a2268 = 0u32;
                if equal(a2267, a2268) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2271 = {
                let a2269 = v0;
                let a2270 = 2u32;
                fixed_u32::bit_and(a2269, a2270)
            };
            let a2272 = 0u32;
            if equal(a2271, a2272) {
                let a2275 = {
                    let a2273 = v0;
                    let a2274 = 1u32;
                    fixed_u32::bit_and(a2273, a2274)
                };
                let a2276 = 0u32;
                if equal(a2275, a2276) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2279 = {
                    let a2277 = v0;
                    let a2278 = 1u32;
                    fixed_u32::bit_and(a2277, a2278)
                };
                let a2280 = 0u32;
                if equal(a2279, a2280) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f498(v0: u32, v1: u32) -> u32 {
    let m2283 = {
        let a2281 = v0;
        let a2282 = v1;
        fixed_u32::shift_right(a2281, a2282)
    };
    match m2283 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f499(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f529(Fn18::F530, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2284, _, _, _, _, _, _, _) => {
            h2284
        }
    }, match v0.clone() {
        Adt24::C0(_, h2285, _, _, _, _, _, _) => {
            h2285
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2286, _, _, _, _, _) => {
            h2286
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2287, _, _, _, _) => {
            h2287
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2288, _, _, _) => {
            h2288
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2289, _, _) => {
            h2289
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2290, _) => {
            h2290
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2291) => {
            h2291
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f531(match v0.clone() {
        Adt24::C0(h2292, _, _, _, _, _, _, _) => {
            h2292
        }
    }, match v2.clone() {
        Adt27::C0(h2293, _, _, _, _, _, _, _, _, _) => {
            h2293
        }
    }), f531(match v0.clone() {
        Adt24::C0(_, h2294, _, _, _, _, _, _) => {
            h2294
        }
    }, match v2.clone() {
        Adt27::C0(_, h2295, _, _, _, _, _, _, _, _) => {
            h2295
        }
    }), f531(match v0.clone() {
        Adt24::C0(_, _, h2296, _, _, _, _, _) => {
            h2296
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2297, _, _, _, _, _, _, _) => {
            h2297
        }
    }), f531(match v0.clone() {
        Adt24::C0(_, _, _, h2298, _, _, _, _) => {
            h2298
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2299, _, _, _, _, _, _) => {
            h2299
        }
    }), f531(match v0.clone() {
        Adt24::C0(_, _, _, _, h2300, _, _, _) => {
            h2300
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2301, _, _, _, _, _) => {
            h2301
        }
    }), f531(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2302, _, _) => {
            h2302
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2303, _, _, _, _) => {
            h2303
        }
    }), f531(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2304, _) => {
            h2304
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2305, _, _, _) => {
            h2305
        }
    }), f531(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2306) => {
            h2306
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2307, _, _) => {
            h2307
        }
    })))
}

pub fn f500(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f532(v0.clone(), v1, v2, v3)?, f532(v0.clone(), v1, v2, {
        let a2308 = v3;
        let a2309 = 4u64;
        nat_add(a2308, a2309)?
    })?, f532(v0.clone(), v1, v2, {
        let a2310 = v3;
        let a2311 = 8u64;
        nat_add(a2310, a2311)?
    })?, f532(v0.clone(), v1, v2, {
        let a2312 = v3;
        let a2313 = 12u64;
        nat_add(a2312, a2313)?
    })?, f532(v0.clone(), v1, v2, {
        let a2314 = v3;
        let a2315 = 16u64;
        nat_add(a2314, a2315)?
    })?, f532(v0.clone(), v1, v2, {
        let a2316 = v3;
        let a2317 = 20u64;
        nat_add(a2316, a2317)?
    })?, f532(v0.clone(), v1, v2, {
        let a2318 = v3;
        let a2319 = 24u64;
        nat_add(a2318, a2319)?
    })?, f532(v0.clone(), v1, v2, {
        let a2320 = v3;
        let a2321 = 28u64;
        nat_add(a2320, a2321)?
    })?, f532(v0.clone(), v1, v2, {
        let a2322 = v3;
        let a2323 = 32u64;
        nat_add(a2322, a2323)?
    })?, f532(v0.clone(), v1, v2, {
        let a2324 = v3;
        let a2325 = 36u64;
        nat_add(a2324, a2325)?
    })?, f532(v0.clone(), v1, v2, {
        let a2326 = v3;
        let a2327 = 40u64;
        nat_add(a2326, a2327)?
    })?, f532(v0.clone(), v1, v2, {
        let a2328 = v3;
        let a2329 = 44u64;
        nat_add(a2328, a2329)?
    })?, f532(v0.clone(), v1, v2, {
        let a2330 = v3;
        let a2331 = 48u64;
        nat_add(a2330, a2331)?
    })?, f532(v0.clone(), v1, v2, {
        let a2332 = v3;
        let a2333 = 52u64;
        nat_add(a2332, a2333)?
    })?, f532(v0.clone(), v1, v2, {
        let a2334 = v3;
        let a2335 = 56u64;
        nat_add(a2334, a2335)?
    })?, f532(v0, v1, v2, {
        let a2336 = v3;
        let a2337 = 60u64;
        nat_add(a2336, a2337)?
    })?))
}

pub fn f501(p2340: List<Bytes>, v1: Bytes) -> bool {
    let m2339 = p2340;
    {
        let mut m2339 = m2339;
        loop {
            let step = {
                let v0 = m2339;
                let m2338 = v0;
                match m2338.uncons() {
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
                Ok(next) => { m2339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(v0: Bytes, v1: u64) -> Bytes {
    let m2344 = {
        let a2341 = v0;
        let a2342 = v1;
        let a2343 = 1u64;
        slice_bytes(a2341, a2342, a2343)
    };
    match m2344 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f503(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f533(v1, v0, v2)
}

pub fn f504(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2389 = {
            let a2365 = {
                let a2353 = {
                    let a2347 = {
                        let a2345 = v3;
                        let a2346 = 128u8;
                        fixed_u8::bit_and(a2345, a2346)
                    };
                    let a2348 = 128u8;
                    if equal(a2347, a2348) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2354 = {
                    let a2351 = {
                        let a2349 = v3;
                        let a2350 = 64u8;
                        fixed_u8::bit_and(a2349, a2350)
                    };
                    let a2352 = 64u8;
                    if equal(a2351, a2352) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2353, a2354)?
            };
            let a2366 = {
                let a2363 = {
                    let a2357 = {
                        let a2355 = v3;
                        let a2356 = 32u8;
                        fixed_u8::bit_and(a2355, a2356)
                    };
                    let a2358 = 32u8;
                    if equal(a2357, a2358) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2364 = {
                    let a2361 = {
                        let a2359 = v3;
                        let a2360 = 16u8;
                        fixed_u8::bit_and(a2359, a2360)
                    };
                    let a2362 = 16u8;
                    if equal(a2361, a2362) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2363, a2364)?
            };
            nat_add(a2365, a2366)?
        };
        let a2390 = {
            let a2387 = {
                let a2375 = {
                    let a2369 = {
                        let a2367 = v3;
                        let a2368 = 8u8;
                        fixed_u8::bit_and(a2367, a2368)
                    };
                    let a2370 = 8u8;
                    if equal(a2369, a2370) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2376 = {
                    let a2373 = {
                        let a2371 = v3;
                        let a2372 = 4u8;
                        fixed_u8::bit_and(a2371, a2372)
                    };
                    let a2374 = 4u8;
                    if equal(a2373, a2374) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2375, a2376)?
            };
            let a2388 = {
                let a2385 = {
                    let a2379 = {
                        let a2377 = v3;
                        let a2378 = 2u8;
                        fixed_u8::bit_and(a2377, a2378)
                    };
                    let a2380 = 2u8;
                    if equal(a2379, a2380) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2386 = {
                    let a2383 = {
                        let a2381 = v3;
                        let a2382 = 1u8;
                        fixed_u8::bit_and(a2381, a2382)
                    };
                    let a2384 = 1u8;
                    if equal(a2383, a2384) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2385, a2386)?
            };
            nat_add(a2387, a2388)?
        };
        nat_add(a2389, a2390)?
    };
    let a2391 = v4;
    let a2392 = 34u64;
    if nat_eq(a2391, a2392) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2393 = v4;
        let a2394 = 92u64;
        if nat_eq(a2393, a2394) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2395 = v4;
            let a2396 = 8u64;
            if nat_eq(a2395, a2396) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2397 = v4;
                let a2398 = 12u64;
                if nat_eq(a2397, a2398) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2399 = v4;
                    let a2400 = 10u64;
                    if nat_eq(a2399, a2400) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2401 = v4;
                        let a2402 = 13u64;
                        if nat_eq(a2401, a2402) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2403 = v4;
                            let a2404 = 9u64;
                            if nat_eq(a2403, a2404) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2405 = v4;
                                let a2406 = 32u64;
                                if nat_lt(a2405, a2406) {
                                    let a2407 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2408 = f534(v2)?;
                                    Ok(append_bytes(a2407, a2408))
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

pub fn f505(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f535(Fn19::F536, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f506(v0: Fn15, p2412: List<List<(Bytes, Bytes)>>, p2413: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2411 = (p2412, p2413);
    {
        let mut m2411 = m2411;
        loop {
            let step = {
                let (v1, v2) = m2411;
                let m2409 = v2;
                match m2409.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2410: Fn15 = v0.clone();
                            c2410.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f508(v0: Fn16, p2418: u64, p2419: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2417 = (p2418, p2419);
    {
        let mut m2417 = m2417;
        loop {
            let step = {
                let (v1, v2) = m2417;
                let m2414 = v1;
                if m2414 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2414 - 1;
                    let m2416 = {
                        let c2415: Fn16 = v0.clone();
                        c2415.apply(v2.clone())?
                    };
                    match m2416 {
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
                Ok(next) => { m2417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f537(v0)
}

pub fn f510(v0: u8) -> bool {
    let a2420 = v0;
    let a2421 = 32u8;
    if if if equal(a2420, a2421) {
        true
    } else {
        let a2422 = v0;
        let a2423 = 9u8;
        equal(a2422, a2423)
    } {
        true
    } else {
        let a2424 = v0;
        let a2425 = 10u8;
        equal(a2424, a2425)
    } {
        true
    } else {
        let a2426 = v0;
        let a2427 = 12u8;
        if equal(a2426, a2427) {
            true
        } else {
            let a2428 = v0;
            let a2429 = 13u8;
            equal(a2428, a2429)
        }
    }
}

pub fn f511(v0: Fn17, p2434: u64, p2435: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2433 = (p2434, p2435);
    {
        let mut m2433 = m2433;
        loop {
            let step = {
                let (v1, v2) = m2433;
                let m2430 = v1;
                if m2430 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2430 - 1;
                    let m2432 = {
                        let c2431: Fn17 = v0.clone();
                        c2431.apply(v2.clone())?
                    };
                    match m2432 {
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
                Ok(next) => { m2433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f538(v0, v1)
}

pub fn f513(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2480 = {
            let a2456 = {
                let a2444 = {
                    let a2438 = {
                        let a2436 = v1;
                        let a2437 = 128u8;
                        fixed_u8::bit_and(a2436, a2437)
                    };
                    let a2439 = 128u8;
                    if equal(a2438, a2439) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2445 = {
                    let a2442 = {
                        let a2440 = v1;
                        let a2441 = 64u8;
                        fixed_u8::bit_and(a2440, a2441)
                    };
                    let a2443 = 64u8;
                    if equal(a2442, a2443) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2444, a2445)?
            };
            let a2457 = {
                let a2454 = {
                    let a2448 = {
                        let a2446 = v1;
                        let a2447 = 32u8;
                        fixed_u8::bit_and(a2446, a2447)
                    };
                    let a2449 = 32u8;
                    if equal(a2448, a2449) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2455 = {
                    let a2452 = {
                        let a2450 = v1;
                        let a2451 = 16u8;
                        fixed_u8::bit_and(a2450, a2451)
                    };
                    let a2453 = 16u8;
                    if equal(a2452, a2453) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2454, a2455)?
            };
            nat_add(a2456, a2457)?
        };
        let a2481 = {
            let a2478 = {
                let a2466 = {
                    let a2460 = {
                        let a2458 = v1;
                        let a2459 = 8u8;
                        fixed_u8::bit_and(a2458, a2459)
                    };
                    let a2461 = 8u8;
                    if equal(a2460, a2461) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2467 = {
                    let a2464 = {
                        let a2462 = v1;
                        let a2463 = 4u8;
                        fixed_u8::bit_and(a2462, a2463)
                    };
                    let a2465 = 4u8;
                    if equal(a2464, a2465) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2466, a2467)?
            };
            let a2479 = {
                let a2476 = {
                    let a2470 = {
                        let a2468 = v1;
                        let a2469 = 2u8;
                        fixed_u8::bit_and(a2468, a2469)
                    };
                    let a2471 = 2u8;
                    if equal(a2470, a2471) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2477 = {
                    let a2474 = {
                        let a2472 = v1;
                        let a2473 = 1u8;
                        fixed_u8::bit_and(a2472, a2473)
                    };
                    let a2475 = 1u8;
                    if equal(a2474, a2475) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2476, a2477)?
            };
            nat_add(a2478, a2479)?
        };
        nat_add(a2480, a2481)?
    };
    let a2482 = 48u64;
    let a2483 = v2;
    if nat_le(a2482, a2483) {
        let a2484 = v2;
        let a2485 = 57u64;
        Ok(nat_le(a2484, a2485))
    } else {
        Ok(false)
    }
}

pub fn f514(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f425({
        let (_, h2486) = {
            let (h2487, _) = f539(Fn20::F540, {
                let a2489 = {
                    let a2488 = v0.clone();
                    length_list(a2488)
                };
                let a2490 = 1u64;
                nat_add(a2489, a2490)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2487
        };
        h2486
    }))
}

pub fn f515(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f516(v0: List<Adt14>) -> List<Bytes> {
    let m2491 = v0;
    match m2491.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2492 = f94(v1, f8(Str::lit("std")));
            match m2492 {
                None => {
                    f516(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f516(v2))
                }
            }
        }
    }
}

pub fn f517(v0: List<Adt14>) -> List<Bytes> {
    let m2493 = v0;
    match m2493.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2494 = f93(f94(v1, f8(Str::lit("req"))));
            match m2494 {
                None => {
                    f517(v2)
                }
                Some(v3) => {
                    List::cons(v3, f517(v2))
                }
            }
        }
    }
}

pub fn f518(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m2495 = v0;
    match m2495 {
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
        Adt2::C5(r2496) => {
            let v4 = (*r2496).clone();
            f518(v4)
        }
        Adt2::C6 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C7(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C8(v1, r2497, r2498) => {
            let v2 = (*r2497).clone();
            let v3 = (*r2498).clone();
            List::cons((v1, v2), f518(v3))
        }
    }
}

pub fn f519(p2503: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m2502 = p2503;
    {
        let mut m2502 = m2502;
        loop {
            let step = {
                let v0 = m2502;
                let m2499 = v0;
                match m2499.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f541(f542({
                            let (_, h2500) = v3.clone();
                            h2500
                        }, List::<Bytes>::nil())?, f521({
                            let (h2501, _) = v3.clone();
                            h2501
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f520(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m2504 = v0;
    match m2504.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h2505, _) = v2.clone();
                h2505
            }, v1.clone()) {
                f520(v3, v1)
            } else {
                List::cons(v2, f520(v3, v1))
            }
        }
    }
}

pub fn f521(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f369(v0)?;
    if f52(v1.clone(), f543()) {
        f544(v1)
    } else {
        Ok(v1)
    }
}

pub fn f522() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f523() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f524(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    f545(v0, f546(v1))
}

pub fn f525(v0: Adt28) -> List<Adt25> {
    let m2506 = v0;
    match m2506 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f526(v0: Adt28) -> Bytes {
    let m2507 = v0;
    match m2507 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f527(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2508 = v0;
    match m2508.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f527(v3, false))
            } else {
                f527(v3, true)
            }
        }
    }
}

pub fn f528(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2513 = {
        let a2510 = f447({
            let (h2509, _) = v0;
            h2509
        })?;
        let a2511 = Bytes::lit(&[0x3a]);
        append_bytes(a2510, a2511)
    };
    let a2514 = {
        let (_, h2512) = v0;
        h2512
    };
    Ok(append_bytes(a2513, a2514))
}

pub fn f529(v0: Fn18, p2518: u64, p2519: Adt27) -> R<Adt27> {
    let m2517 = (p2518, p2519);
    {
        let mut m2517 = m2517;
        loop {
            let step = {
                let (v1, v2) = m2517;
                let m2515 = v1;
                if m2515 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2515 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2516: Fn18 = v0.clone();
                        c2516.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f530(v0: Adt27) -> R<Adt27> {
    f547(v0)
}

pub fn f531(v0: u32, v1: u32) -> u32 {
    let m2522 = {
        let a2520 = f548(v0);
        let a2521 = f548(v1);
        fixed_u64::checked_add(a2520, a2521)
    };
    match m2522 {
        None => {
            0u32
        }
        Some(v2) => {
            f549(v2)
        }
    }
}

pub fn f532(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2525 = {
        let a2523 = v3;
        let a2524 = 8u64;
        nat_add(a2523, a2524)?
    };
    let a2526 = v2;
    if nat_eq(a2525, a2526) {
        Ok(f550({
            let a2529 = {
                let a2527 = v1;
                let a2528 = 8u64;
                nat_mul(a2527, a2528)?
            };
            let a2530 = 4294967296u64;
            let a2531 = 0u64;
            nat_quot(a2529, a2530, a2531)
        }))
    } else {
        let a2534 = {
            let a2532 = v3;
            let a2533 = 4u64;
            nat_add(a2532, a2533)?
        };
        let a2535 = v2;
        if nat_eq(a2534, a2535) {
            Ok(f550({
                let a2538 = {
                    let a2536 = v1;
                    let a2537 = 8u64;
                    nat_mul(a2536, a2537)?
                };
                let a2539 = 4294967296u64;
                let a2540 = 0u64;
                nat_rem(a2538, a2539, a2540)
            }))
        } else {
            Ok(f551(f552(v0.clone(), v1, v3), f552(v0.clone(), v1, {
                let a2541 = v3;
                let a2542 = 1u64;
                nat_add(a2541, a2542)?
            }), f552(v0.clone(), v1, {
                let a2543 = v3;
                let a2544 = 2u64;
                nat_add(a2543, a2544)?
            }), f552(v0, v1, {
                let a2545 = v3;
                let a2546 = 3u64;
                nat_add(a2545, a2546)?
            })))
        }
    }
}

pub fn f533(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2547, _) = v2;
        h2547
    };
    let a2548 = v3;
    let a2549 = 0u64;
    if nat_eq(a2548, a2549) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2550 = v3;
            let a2551 = v1;
            let a2552 = 0u64;
            nat_quot(a2550, a2551, a2552)
        }, {
            let a2557 = f502(v0, {
                let a2553 = v3;
                let a2554 = v1;
                let a2555 = 0u64;
                nat_rem(a2553, a2554, a2555)
            });
            let a2558 = {
                let (_, h2556) = v2;
                h2556
            };
            append_bytes(a2557, a2558)
        }))
    }
}

pub fn f534(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2603 = {
            let a2579 = {
                let a2567 = {
                    let a2561 = {
                        let a2559 = v1;
                        let a2560 = 128u8;
                        fixed_u8::bit_and(a2559, a2560)
                    };
                    let a2562 = 128u8;
                    if equal(a2561, a2562) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2568 = {
                    let a2565 = {
                        let a2563 = v1;
                        let a2564 = 64u8;
                        fixed_u8::bit_and(a2563, a2564)
                    };
                    let a2566 = 64u8;
                    if equal(a2565, a2566) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2567, a2568)?
            };
            let a2580 = {
                let a2577 = {
                    let a2571 = {
                        let a2569 = v1;
                        let a2570 = 32u8;
                        fixed_u8::bit_and(a2569, a2570)
                    };
                    let a2572 = 32u8;
                    if equal(a2571, a2572) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2578 = {
                    let a2575 = {
                        let a2573 = v1;
                        let a2574 = 16u8;
                        fixed_u8::bit_and(a2573, a2574)
                    };
                    let a2576 = 16u8;
                    if equal(a2575, a2576) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2577, a2578)?
            };
            nat_add(a2579, a2580)?
        };
        let a2604 = {
            let a2601 = {
                let a2589 = {
                    let a2583 = {
                        let a2581 = v1;
                        let a2582 = 8u8;
                        fixed_u8::bit_and(a2581, a2582)
                    };
                    let a2584 = 8u8;
                    if equal(a2583, a2584) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2590 = {
                    let a2587 = {
                        let a2585 = v1;
                        let a2586 = 4u8;
                        fixed_u8::bit_and(a2585, a2586)
                    };
                    let a2588 = 4u8;
                    if equal(a2587, a2588) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2589, a2590)?
            };
            let a2602 = {
                let a2599 = {
                    let a2593 = {
                        let a2591 = v1;
                        let a2592 = 2u8;
                        fixed_u8::bit_and(a2591, a2592)
                    };
                    let a2594 = 2u8;
                    if equal(a2593, a2594) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2600 = {
                    let a2597 = {
                        let a2595 = v1;
                        let a2596 = 1u8;
                        fixed_u8::bit_and(a2595, a2596)
                    };
                    let a2598 = 1u8;
                    if equal(a2597, a2598) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2599, a2600)?
            };
            nat_add(a2601, a2602)?
        };
        nat_add(a2603, a2604)?
    };
    let a2611 = f502(f553(), {
        let a2605 = v2;
        let a2606 = 16u64;
        let a2607 = 0u64;
        nat_quot(a2605, a2606, a2607)
    });
    let a2612 = f502(f553(), {
        let a2608 = v2;
        let a2609 = 16u64;
        let a2610 = 0u64;
        nat_rem(a2608, a2609, a2610)
    });
    Ok(append_bytes(a2611, a2612))
}

pub fn f535(v0: Fn19, p2616: List<List<(Bytes, Bytes)>>, p2617: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2615 = (p2616, p2617);
    {
        let mut m2615 = m2615;
        loop {
            let step = {
                let (v1, v2) = m2615;
                let m2613 = v2;
                match m2613.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2614: Fn19 = v0.clone();
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

pub fn f536(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f537(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2619 = {
        let a2618 = v0.clone();
        length_list(a2618)
    };
    let a2620 = 1u64;
    if nat_le(a2619, a2620) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f554(v0)?))
    }
}

pub fn f538(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2621, _) = v1;
        h2621
    };
    let m2623 = {
        let (_, h2622) = v1;
        h2622
    };
    match m2623 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2626 = {
                let a2624 = v0.clone();
                let a2625 = v2;
                index_bytes(a2624, a2625)
            };
            match m2626 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2631 = {
                        let a2629 = v0;
                        let a2630 = {
                            let a2627 = v2;
                            let a2628 = 1u64;
                            nat_add(a2627, a2628)?
                        };
                        index_bytes(a2629, a2630)
                    };
                    match m2631 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f555(v4)? {
                                f555(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2632 = v2;
                                    let a2633 = 2u64;
                                    nat_add(a2632, a2633)?
                                }, Some({
                                    let a2638 = v3;
                                    let a2639 = f502(f556(), {
                                        let a2636 = {
                                            let a2634 = f557(v4)?;
                                            let a2635 = 16u64;
                                            nat_mul(a2634, a2635)?
                                        };
                                        let a2637 = f557(v5)?;
                                        nat_add(a2636, a2637)?
                                    });
                                    append_bytes(a2638, a2639)
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

pub fn f539(v0: Fn20, p2644: u64, p2645: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2643 = (p2644, p2645);
    {
        let mut m2643 = m2643;
        loop {
            let step = {
                let (v1, v2) = m2643;
                let m2640 = v1;
                if m2640 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2640 - 1;
                    let m2642 = {
                        let c2641: Fn20 = v0.clone();
                        c2641.apply(v2.clone())?
                    };
                    match m2642 {
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
                Ok(next) => { m2643 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f558(v0)
}

pub fn f541(p2648: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2647 = p2648;
    {
        let mut m2647 = m2647;
        loop {
            let step = {
                let v0 = m2647;
                let m2646 = v0;
                match m2646.uncons() {
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
                Ok(next) => { m2647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f542(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2649 = v0;
    match m2649 {
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
        Adt2::C4(r2652) => {
            let v2 = (*r2652).clone();
            f542(v2, v1)
        }
        Adt2::C5(r2653) => {
            let v3 = (*r2653).clone();
            f542(v3, v1)
        }
        Adt2::C6 => {
            Ok(v1)
        }
        Adt2::C7(r2654, r2655) => {
            let v4 = (*r2654).clone();
            let v5 = (*r2655).clone();
            f542(v4, f542(v5, v1)?)
        }
        Adt2::C8(v6, r2656, r2657) => {
            let v7 = (*r2656).clone();
            let v8 = (*r2657).clone();
            let v9: List<Bytes> = f542(v7.clone(), f542(v8, v1)?)?;
            if f32(v6, f8(Str::lit("$ref"))) {
                let m2650 = f375(Some(v7));
                match m2650 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2651 = f559(v10)?;
                        match m2651 {
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

pub fn f543() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f544(v0: Bytes) -> R<Bytes> {
    let m2660 = {
        let a2658 = v0.clone();
        let a2659 = 0u64;
        index_bytes(a2658, a2659)
    };
    match m2660 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2666 = {
                let a2661 = f8(Str::lit("u"));
                let a2662 = f560(8u64, f72(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2661, a2662)
            };
            let a2667 = f46(v0.clone(), 1u64, {
                let a2664 = {
                    let a2663 = v0;
                    length_bytes(a2663)
                };
                let a2665 = 1u64;
                nat_sub(a2664, a2665)
            });
            Ok(append_bytes(a2666, a2667))
        }
    }
}

pub fn f545(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    let m2668 = f278(v1.clone(), f8(Str::lit("enum")));
    match m2668 {
        None => {
            let m2670 = f278(v1.clone(), f8(Str::lit("oneOf")));
            match m2670 {
                None => {
                    if f565(v1.clone(), v0.clone())? {
                        let m2671 = f566(v0.clone(), f485(f278(v1.clone(), f8(Str::lit("properties")))), f282(f283(f278(v1, f8(Str::lit("required"))))), true)?;
                        match m2671 {
                            Ok(v7) => {
                                Ok(f563(f567(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f383(f568(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f569(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2672 = f485(f278(v1.clone(), f8(Str::lit("properties"))));
                        match m2672.uncons() {
                            None => {
                                if f570(v1.clone()) {
                                    Ok(f563(f571(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2675 = f572(v1, {
                                        let a2673 = v0.clone();
                                        let a2674 = f8(Str::lit("Value"));
                                        append_bytes(a2673, a2674)
                                    })?;
                                    match m2675 {
                                        Ok(v9) => {
                                            Ok(f563(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f526(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f525(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2676 = f566(v0.clone(), f485(f278(v1.clone(), f8(Str::lit("properties")))), f282(f283(f278(v1, f8(Str::lit("required"))))), false)?;
                                match m2676 {
                                    Ok(v13) => {
                                        Ok(f563(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2677 = f568(v13.clone());
                                            match m2677.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f383(f568(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f569(v13)))
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
                    f564(v0, f376(v5))
                }
            }
        }
        Some(v2) => {
            let m2669 = f561(f376(v2), List::<Bytes>::nil())?;
            match m2669 {
                Ok(v4) => {
                    Ok(f563(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f383(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f562(v3, v0))
                }
            }
        }
    }
}

pub fn f546(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m2678 = f278(v0.clone(), f8(Str::lit("anyOf")));
        match m2678 {
            None => {
                v0
            }
            Some(v1) => {
                if f573(f376(v1.clone())) {
                    f574(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f575(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f576(f376(v1.clone()))
                    } {
                        v0
                    } else {
                        f577(f574(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m2679 = f278(v2.clone(), f8(Str::lit("oneOf")));
        match m2679 {
            None => {
                v2
            }
            Some(v3) => {
                if f573(f376(v3)) {
                    f574(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2680 = f278(v4.clone(), f8(Str::lit("allOf")));
    match m2680 {
        None => {
            v4
        }
        Some(v5) => {
            if f578(f376(v5)) {
                f574(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f547(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f579(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2681) => {
            h2681
        }
    });
    let v2: u32 = f531(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2682, _, _) => {
            h2682
        }
    }, f531(f580(match v0.clone() {
        Adt27::C0(_, _, _, _, h2683, _, _, _, _, _) => {
            h2683
        }
    }), f531(f581(match v0.clone() {
        Adt27::C0(_, _, _, _, h2684, _, _, _, _, _) => {
            h2684
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2685, _, _, _, _) => {
            h2685
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2686, _, _, _) => {
            h2686
        }
    }), f531(v1, {
        let m2689 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2688, _) => {
                h2688
            }
        };
        match m2689 {
            Adt26::C0(h2687, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2687
            }
        }
    }))));
    let v3: u32 = f531(f582(match v0.clone() {
        Adt27::C0(h2690, _, _, _, _, _, _, _, _, _) => {
            h2690
        }
    }), f583(match v0.clone() {
        Adt27::C0(h2691, _, _, _, _, _, _, _, _, _) => {
            h2691
        }
    }, match v0.clone() {
        Adt27::C0(_, h2692, _, _, _, _, _, _, _, _) => {
            h2692
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2693, _, _, _, _, _, _, _) => {
            h2693
        }
    }));
    Ok(Adt27::C0(f531(v2, v3), match v0.clone() {
        Adt27::C0(h2694, _, _, _, _, _, _, _, _, _) => {
            h2694
        }
    }, match v0.clone() {
        Adt27::C0(_, h2695, _, _, _, _, _, _, _, _) => {
            h2695
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2696, _, _, _, _, _, _, _) => {
            h2696
        }
    }, f531(match v0.clone() {
        Adt27::C0(_, _, _, h2697, _, _, _, _, _, _) => {
            h2697
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2698, _, _, _, _, _) => {
            h2698
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2699, _, _, _, _) => {
            h2699
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2700, _, _, _) => {
            h2700
        }
    }, f584(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2701, _) => {
            h2701
        }
    }), {
        let a2703 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2702) => {
                h2702
            }
        };
        let a2704 = 1u64;
        nat_add(a2703, a2704)?
    }))
}

pub fn f548(v0: u32) -> u64 {
    let m2706 = {
        let a2705 = v0;
        fixed_u64::convert(i128::from(a2705))
    };
    match m2706 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f549(v0: u64) -> u32 {
    let m2710 = {
        let a2709 = {
            let a2707 = v0;
            let a2708 = 4294967295u64;
            fixed_u64::bit_and(a2707, a2708)
        };
        fixed_u32::convert(i128::from(a2709))
    };
    match m2710 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f550(v0: u64) -> u32 {
    let (_, h2711) = {
        let (_, h2712) = f585(Fn21::F586, 32u64, (v0, (1u32, 0u32)));
        h2712
    };
    h2711
}

pub fn f551(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2717 = f587(v0, 24u32);
    let a2718 = {
        let a2715 = f587(v1, 16u32);
        let a2716 = {
            let a2713 = f587(v2, 8u32);
            let a2714 = v3;
            fixed_u32::bit_or(a2713, a2714)
        };
        fixed_u32::bit_or(a2715, a2716)
    };
    fixed_u32::bit_or(a2717, a2718)
}

pub fn f552(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2719 = v2;
    let a2720 = v1;
    if nat_lt(a2719, a2720) {
        let m2723 = {
            let a2721 = v0;
            let a2722 = v2;
            index_bytes(a2721, a2722)
        };
        match m2723 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2725 = {
                    let a2724 = v3;
                    fixed_u32::convert(i128::from(a2724))
                };
                match m2725 {
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
        let a2726 = v2;
        let a2727 = v1;
        if nat_eq(a2726, a2727) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f553() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f554(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f505({
        let (_, h2728) = {
            let (h2729, _) = f588(Fn22::F589, {
                let a2731 = {
                    let a2730 = v0.clone();
                    length_list(a2730)
                };
                let a2732 = 1u64;
                nat_add(a2731, a2732)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2729
        };
        h2728
    }))
}

pub fn f555(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2777 = {
            let a2753 = {
                let a2741 = {
                    let a2735 = {
                        let a2733 = v1;
                        let a2734 = 128u8;
                        fixed_u8::bit_and(a2733, a2734)
                    };
                    let a2736 = 128u8;
                    if equal(a2735, a2736) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2742 = {
                    let a2739 = {
                        let a2737 = v1;
                        let a2738 = 64u8;
                        fixed_u8::bit_and(a2737, a2738)
                    };
                    let a2740 = 64u8;
                    if equal(a2739, a2740) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2741, a2742)?
            };
            let a2754 = {
                let a2751 = {
                    let a2745 = {
                        let a2743 = v1;
                        let a2744 = 32u8;
                        fixed_u8::bit_and(a2743, a2744)
                    };
                    let a2746 = 32u8;
                    if equal(a2745, a2746) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2752 = {
                    let a2749 = {
                        let a2747 = v1;
                        let a2748 = 16u8;
                        fixed_u8::bit_and(a2747, a2748)
                    };
                    let a2750 = 16u8;
                    if equal(a2749, a2750) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2751, a2752)?
            };
            nat_add(a2753, a2754)?
        };
        let a2778 = {
            let a2775 = {
                let a2763 = {
                    let a2757 = {
                        let a2755 = v1;
                        let a2756 = 8u8;
                        fixed_u8::bit_and(a2755, a2756)
                    };
                    let a2758 = 8u8;
                    if equal(a2757, a2758) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2764 = {
                    let a2761 = {
                        let a2759 = v1;
                        let a2760 = 4u8;
                        fixed_u8::bit_and(a2759, a2760)
                    };
                    let a2762 = 4u8;
                    if equal(a2761, a2762) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2763, a2764)?
            };
            let a2776 = {
                let a2773 = {
                    let a2767 = {
                        let a2765 = v1;
                        let a2766 = 2u8;
                        fixed_u8::bit_and(a2765, a2766)
                    };
                    let a2768 = 2u8;
                    if equal(a2767, a2768) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2774 = {
                    let a2771 = {
                        let a2769 = v1;
                        let a2770 = 1u8;
                        fixed_u8::bit_and(a2769, a2770)
                    };
                    let a2772 = 1u8;
                    if equal(a2771, a2772) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2773, a2774)?
            };
            nat_add(a2775, a2776)?
        };
        nat_add(a2777, a2778)?
    };
    let a2779 = 48u64;
    let a2780 = v2;
    if if nat_le(a2779, a2780) {
        let a2781 = v2;
        let a2782 = 57u64;
        nat_le(a2781, a2782)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2783 = 97u64;
        let a2784 = v2;
        if nat_le(a2783, a2784) {
            let a2785 = v2;
            let a2786 = 102u64;
            Ok(nat_le(a2785, a2786))
        } else {
            Ok(false)
        }
    }
}

pub fn f556() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f557(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2831 = {
            let a2807 = {
                let a2795 = {
                    let a2789 = {
                        let a2787 = v1;
                        let a2788 = 128u8;
                        fixed_u8::bit_and(a2787, a2788)
                    };
                    let a2790 = 128u8;
                    if equal(a2789, a2790) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2796 = {
                    let a2793 = {
                        let a2791 = v1;
                        let a2792 = 64u8;
                        fixed_u8::bit_and(a2791, a2792)
                    };
                    let a2794 = 64u8;
                    if equal(a2793, a2794) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2795, a2796)?
            };
            let a2808 = {
                let a2805 = {
                    let a2799 = {
                        let a2797 = v1;
                        let a2798 = 32u8;
                        fixed_u8::bit_and(a2797, a2798)
                    };
                    let a2800 = 32u8;
                    if equal(a2799, a2800) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2806 = {
                    let a2803 = {
                        let a2801 = v1;
                        let a2802 = 16u8;
                        fixed_u8::bit_and(a2801, a2802)
                    };
                    let a2804 = 16u8;
                    if equal(a2803, a2804) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2805, a2806)?
            };
            nat_add(a2807, a2808)?
        };
        let a2832 = {
            let a2829 = {
                let a2817 = {
                    let a2811 = {
                        let a2809 = v1;
                        let a2810 = 8u8;
                        fixed_u8::bit_and(a2809, a2810)
                    };
                    let a2812 = 8u8;
                    if equal(a2811, a2812) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2818 = {
                    let a2815 = {
                        let a2813 = v1;
                        let a2814 = 4u8;
                        fixed_u8::bit_and(a2813, a2814)
                    };
                    let a2816 = 4u8;
                    if equal(a2815, a2816) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2817, a2818)?
            };
            let a2830 = {
                let a2827 = {
                    let a2821 = {
                        let a2819 = v1;
                        let a2820 = 2u8;
                        fixed_u8::bit_and(a2819, a2820)
                    };
                    let a2822 = 2u8;
                    if equal(a2821, a2822) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2828 = {
                    let a2825 = {
                        let a2823 = v1;
                        let a2824 = 1u8;
                        fixed_u8::bit_and(a2823, a2824)
                    };
                    let a2826 = 1u8;
                    if equal(a2825, a2826) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2827, a2828)?
            };
            nat_add(a2829, a2830)?
        };
        nat_add(a2831, a2832)?
    };
    let a2833 = v2;
    let a2834 = 57u64;
    if nat_le(a2833, a2834) {
        let a2835 = v2;
        let a2836 = 48u64;
        Ok(nat_sub(a2835, a2836))
    } else {
        let a2837 = v2;
        let a2838 = 87u64;
        Ok(nat_sub(a2837, a2838))
    }
}

pub fn f558(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2839, _) = v0;
        h2839
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2840) = v0;
        h2840
    };
    let m2841 = v1;
    match m2841.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2842 = v4;
            match m2842.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f590(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f559(v0: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f521(f46(v0.clone(), 8u64, {
            let a2844 = {
                let a2843 = v0;
                length_bytes(a2843)
            };
            let a2845 = 8u64;
            nat_sub(a2844, a2845)
        }))?))
    } else {
        if f85(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f521(f46(v0.clone(), 14u64, {
                let a2847 = {
                    let a2846 = v0;
                    length_bytes(a2846)
                };
                let a2848 = 14u64;
                nat_sub(a2847, a2848)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f560(p2865: u64, p2866: u64, p2867: u64, p2868: Bytes) -> Bytes {
    let m2864 = (p2865, (p2866, (p2867, p2868)));
    {
        let mut m2864 = m2864;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2864;
                let m2849 = v0;
                if m2849 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2849 - 1;
                    if f118({
                        let a2850 = v1;
                        let a2851 = 0u64;
                        nat_eq(a2850, a2851)
                    }, {
                        let a2852 = v2;
                        let a2853 = 0u64;
                        nat_eq(a2852, a2853)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2854 = v1;
                            let a2855 = 16u64;
                            let a2856 = 0u64;
                            nat_quot(a2854, a2855, a2856)
                        }, ({
                            let a2857 = v2;
                            let a2858 = 1u64;
                            nat_sub(a2857, a2858)
                        }, {
                            let a2862 = f591({
                                let a2859 = v1;
                                let a2860 = 16u64;
                                let a2861 = 0u64;
                                nat_rem(a2859, a2860, a2861)
                            });
                            let a2863 = v3;
                            append_bytes(a2862, a2863)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2864 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f592(f593(f282(v0.clone()))?, v0, v1)
}

pub fn f562(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f563(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f564(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt28, Bytes>> {
    let m2869 = v1.clone();
    match m2869.uncons() {
        None => {
            Ok(f562(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2870 = f594(f485(f278(v2, f8(Str::lit("properties")))), v1.clone());
            match m2870.uncons() {
                None => {
                    let m2871 = f595(f593(f596(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2871 {
                        Ok(v5) => {
                            Ok(f563(f567(v0, f568(v5.clone())), f569(v5)))
                        }
                        Err(v4) => {
                            Ok(f562(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2872 = f597(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2872 {
                        Ok(v9) => {
                            Ok(f563(f567(v0, f568(v9.clone())), f569(v9)))
                        }
                        Err(v8) => {
                            Ok(f562(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f565(v0: Adt2, v1: Bytes) -> R<bool> {
    let m2873 = v0;
    match m2873 {
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
        Adt2::C4(r2874) => {
            let v2 = (*r2874).clone();
            f565(v2, v1)
        }
        Adt2::C5(r2875) => {
            let v3 = (*r2875).clone();
            f565(v3, v1)
        }
        Adt2::C6 => {
            Ok(false)
        }
        Adt2::C7(r2876, r2877) => {
            let v4 = (*r2876).clone();
            let v5 = (*r2877).clone();
            if f565(v4, v1.clone())? {
                Ok(true)
            } else {
                f565(v5, v1)
            }
        }
        Adt2::C8(v6, r2878, r2879) => {
            let v7 = (*r2878).clone();
            let v8 = (*r2879).clone();
            if if f32(v6, f8(Str::lit("$ref"))) {
                f598(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f565(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f565(v8, v1)
                }
            }
        }
    }
}

pub fn f566(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f599(f593(f600(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f567(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f383(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f568(v0: Adt29) -> List<Bytes> {
    let m2880 = v0;
    match m2880 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f569(v0: Adt29) -> List<Adt25> {
    let m2881 = v0;
    match m2881 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f570(v0: Adt2) -> bool {
    let m2882 = f485(f278(v0.clone(), f8(Str::lit("properties"))));
    match m2882.uncons() {
        None => {
            let m2883 = f601(v0);
            match m2883 {
                None => {
                    false
                }
                Some(v3) => {
                    f602(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f571(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f572(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2884 = f603(v0.clone());
    match m2884 {
        None => {
            if f32(f281(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2885 = f278(v0, f8(Str::lit("items")));
                match m2885 {
                    None => {
                        Ok(f604(f8(Str::lit("List")), f606()))
                    }
                    Some(v3) => {
                        Ok(f604(f8(Str::lit("List")), f605(v3, {
                            let a2886 = v1;
                            let a2887 = f8(Str::lit("Item"));
                            append_bytes(a2886, a2887)
                        })?))
                    }
                }
            } else {
                f605(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f604(f8(Str::lit("Option")), f605(v2, v1)?))
        }
    }
}

pub fn f573(p2890: List<Adt2>) -> bool {
    let m2889 = p2890;
    {
        let mut m2889 = m2889;
        loop {
            let step = {
                let v0 = m2889;
                let m2888 = v0;
                match m2888.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f607(f518(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
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

pub fn f574(v0: Adt2, v1: Bytes) -> Adt2 {
    let m2891 = v0;
    match m2891 {
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
        Adt2::C4(r2892) => {
            let v13 = (*r2892).clone();
            Adt2::C4(std::rc::Rc::new(v13))
        }
        Adt2::C5(r2893) => {
            let v5 = (*r2893).clone();
            Adt2::C5(std::rc::Rc::new(f574(v5, v1)))
        }
        Adt2::C6 => {
            Adt2::C6
        }
        Adt2::C7(r2894, r2895) => {
            let v14 = (*r2894).clone();
            let v15 = (*r2895).clone();
            Adt2::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt2::C8(v2, r2896, r2897) => {
            let v3 = (*r2896).clone();
            let v4 = (*r2897).clone();
            if f32(v2.clone(), v1.clone()) {
                f574(v4, v1)
            } else {
                Adt2::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f574(v4, v1)))
            }
        }
    }
}

pub fn f575(v0: Adt2, v1: Bytes) -> bool {
    let m2898 = f278(v0, v1);
    match m2898 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f576(v0: List<Adt2>) -> bool {
    let m2899 = v0.clone();
    match m2899.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2900 = f594(f485(f278(v1, f8(Str::lit("properties")))), v0.clone());
            match m2900.uncons() {
                None => {
                    f609(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f608(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f577(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m2901 = v0;
    match m2901 {
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
        Adt2::C4(r2902) => {
            let v14 = (*r2902).clone();
            Adt2::C4(std::rc::Rc::new(v14))
        }
        Adt2::C5(r2903) => {
            let v3 = (*r2903).clone();
            Adt2::C5(std::rc::Rc::new(Adt2::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C6 => {
            Adt2::C6
        }
        Adt2::C7(r2904, r2905) => {
            let v15 = (*r2904).clone();
            let v16 = (*r2905).clone();
            Adt2::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt2::C8(v4, r2906, r2907) => {
            let v5 = (*r2906).clone();
            let v6 = (*r2907).clone();
            Adt2::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f578(p2910: List<Adt2>) -> bool {
    let m2909 = p2910;
    {
        let mut m2909 = m2909;
        loop {
            let step = {
                let v0 = m2909;
                let m2908 = v0;
                match m2908.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f610(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: u64) -> u32 {
    let a2911 = v0;
    let a2912 = 32u64;
    if nat_lt(a2911, a2912) {
        let a2913 = v0;
        let a2914 = 16u64;
        if nat_lt(a2913, a2914) {
            let a2915 = v0;
            let a2916 = 8u64;
            if nat_lt(a2915, a2916) {
                let a2917 = v0;
                let a2918 = 4u64;
                if nat_lt(a2917, a2918) {
                    let a2919 = v0;
                    let a2920 = 2u64;
                    if nat_lt(a2919, a2920) {
                        let a2921 = v0;
                        let a2922 = 1u64;
                        if nat_lt(a2921, a2922) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2923 = v0;
                        let a2924 = 3u64;
                        if nat_lt(a2923, a2924) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2925 = v0;
                    let a2926 = 6u64;
                    if nat_lt(a2925, a2926) {
                        let a2927 = v0;
                        let a2928 = 5u64;
                        if nat_lt(a2927, a2928) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2929 = v0;
                        let a2930 = 7u64;
                        if nat_lt(a2929, a2930) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2931 = v0;
                let a2932 = 12u64;
                if nat_lt(a2931, a2932) {
                    let a2933 = v0;
                    let a2934 = 10u64;
                    if nat_lt(a2933, a2934) {
                        let a2935 = v0;
                        let a2936 = 9u64;
                        if nat_lt(a2935, a2936) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2937 = v0;
                        let a2938 = 11u64;
                        if nat_lt(a2937, a2938) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2939 = v0;
                    let a2940 = 14u64;
                    if nat_lt(a2939, a2940) {
                        let a2941 = v0;
                        let a2942 = 13u64;
                        if nat_lt(a2941, a2942) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2943 = v0;
                        let a2944 = 15u64;
                        if nat_lt(a2943, a2944) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2945 = v0;
            let a2946 = 24u64;
            if nat_lt(a2945, a2946) {
                let a2947 = v0;
                let a2948 = 20u64;
                if nat_lt(a2947, a2948) {
                    let a2949 = v0;
                    let a2950 = 18u64;
                    if nat_lt(a2949, a2950) {
                        let a2951 = v0;
                        let a2952 = 17u64;
                        if nat_lt(a2951, a2952) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2953 = v0;
                        let a2954 = 19u64;
                        if nat_lt(a2953, a2954) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2955 = v0;
                    let a2956 = 22u64;
                    if nat_lt(a2955, a2956) {
                        let a2957 = v0;
                        let a2958 = 21u64;
                        if nat_lt(a2957, a2958) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2959 = v0;
                        let a2960 = 23u64;
                        if nat_lt(a2959, a2960) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2961 = v0;
                let a2962 = 28u64;
                if nat_lt(a2961, a2962) {
                    let a2963 = v0;
                    let a2964 = 26u64;
                    if nat_lt(a2963, a2964) {
                        let a2965 = v0;
                        let a2966 = 25u64;
                        if nat_lt(a2965, a2966) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2967 = v0;
                        let a2968 = 27u64;
                        if nat_lt(a2967, a2968) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2969 = v0;
                    let a2970 = 30u64;
                    if nat_lt(a2969, a2970) {
                        let a2971 = v0;
                        let a2972 = 29u64;
                        if nat_lt(a2971, a2972) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2973 = v0;
                        let a2974 = 31u64;
                        if nat_lt(a2973, a2974) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2975 = v0;
        let a2976 = 48u64;
        if nat_lt(a2975, a2976) {
            let a2977 = v0;
            let a2978 = 40u64;
            if nat_lt(a2977, a2978) {
                let a2979 = v0;
                let a2980 = 36u64;
                if nat_lt(a2979, a2980) {
                    let a2981 = v0;
                    let a2982 = 34u64;
                    if nat_lt(a2981, a2982) {
                        let a2983 = v0;
                        let a2984 = 33u64;
                        if nat_lt(a2983, a2984) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2985 = v0;
                        let a2986 = 35u64;
                        if nat_lt(a2985, a2986) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2987 = v0;
                    let a2988 = 38u64;
                    if nat_lt(a2987, a2988) {
                        let a2989 = v0;
                        let a2990 = 37u64;
                        if nat_lt(a2989, a2990) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2991 = v0;
                        let a2992 = 39u64;
                        if nat_lt(a2991, a2992) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2993 = v0;
                let a2994 = 44u64;
                if nat_lt(a2993, a2994) {
                    let a2995 = v0;
                    let a2996 = 42u64;
                    if nat_lt(a2995, a2996) {
                        let a2997 = v0;
                        let a2998 = 41u64;
                        if nat_lt(a2997, a2998) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2999 = v0;
                        let a3000 = 43u64;
                        if nat_lt(a2999, a3000) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3001 = v0;
                    let a3002 = 46u64;
                    if nat_lt(a3001, a3002) {
                        let a3003 = v0;
                        let a3004 = 45u64;
                        if nat_lt(a3003, a3004) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3005 = v0;
                        let a3006 = 47u64;
                        if nat_lt(a3005, a3006) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3007 = v0;
            let a3008 = 56u64;
            if nat_lt(a3007, a3008) {
                let a3009 = v0;
                let a3010 = 52u64;
                if nat_lt(a3009, a3010) {
                    let a3011 = v0;
                    let a3012 = 50u64;
                    if nat_lt(a3011, a3012) {
                        let a3013 = v0;
                        let a3014 = 49u64;
                        if nat_lt(a3013, a3014) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3015 = v0;
                        let a3016 = 51u64;
                        if nat_lt(a3015, a3016) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3017 = v0;
                    let a3018 = 54u64;
                    if nat_lt(a3017, a3018) {
                        let a3019 = v0;
                        let a3020 = 53u64;
                        if nat_lt(a3019, a3020) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3021 = v0;
                        let a3022 = 55u64;
                        if nat_lt(a3021, a3022) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3023 = v0;
                let a3024 = 60u64;
                if nat_lt(a3023, a3024) {
                    let a3025 = v0;
                    let a3026 = 58u64;
                    if nat_lt(a3025, a3026) {
                        let a3027 = v0;
                        let a3028 = 57u64;
                        if nat_lt(a3027, a3028) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3029 = v0;
                        let a3030 = 59u64;
                        if nat_lt(a3029, a3030) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3031 = v0;
                    let a3032 = 62u64;
                    if nat_lt(a3031, a3032) {
                        let a3033 = v0;
                        let a3034 = 61u64;
                        if nat_lt(a3033, a3034) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3035 = v0;
                        let a3036 = 63u64;
                        if nat_lt(a3035, a3036) {
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

pub fn f580(v0: u32) -> u32 {
    let a3039 = {
        let a3037 = f611(v0, 6u32, 26u32);
        let a3038 = f611(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3037, a3038)
    };
    let a3040 = f611(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3039, a3040)
}

pub fn f581(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3046 = {
        let a3041 = v0;
        let a3042 = v1;
        fixed_u32::bit_and(a3041, a3042)
    };
    let a3047 = {
        let a3044 = {
            let a3043 = v0;
            fixed_u32::bit_not(a3043)
        };
        let a3045 = v2;
        fixed_u32::bit_and(a3044, a3045)
    };
    fixed_u32::bit_xor(a3046, a3047)
}

pub fn f582(v0: u32) -> u32 {
    let a3050 = {
        let a3048 = f611(v0, 2u32, 30u32);
        let a3049 = f611(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3048, a3049)
    };
    let a3051 = f611(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3050, a3051)
}

pub fn f583(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3060 = {
        let a3056 = {
            let a3052 = v0;
            let a3053 = v1;
            fixed_u32::bit_and(a3052, a3053)
        };
        let a3057 = {
            let a3054 = v0;
            let a3055 = v2;
            fixed_u32::bit_and(a3054, a3055)
        };
        fixed_u32::bit_xor(a3056, a3057)
    };
    let a3061 = {
        let a3058 = v1;
        let a3059 = v2;
        fixed_u32::bit_and(a3058, a3059)
    };
    fixed_u32::bit_xor(a3060, a3061)
}

pub fn f584(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3062, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3062
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3063, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3063
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3064, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3064
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3065, _, _, _, _, _, _, _, _, _, _, _) => {
            h3065
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3066, _, _, _, _, _, _, _, _, _, _) => {
            h3066
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3067, _, _, _, _, _, _, _, _, _) => {
            h3067
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3068, _, _, _, _, _, _, _, _) => {
            h3068
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3069, _, _, _, _, _, _, _) => {
            h3069
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3070, _, _, _, _, _, _) => {
            h3070
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3071, _, _, _, _, _) => {
            h3071
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3072, _, _, _, _) => {
            h3072
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3073, _, _, _) => {
            h3073
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3074, _, _) => {
            h3074
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3075, _) => {
            h3075
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3076) => {
            h3076
        }
    }, f531(f612(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3077, _) => {
            h3077
        }
    }), f531(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3078, _, _, _, _, _, _) => {
            h3078
        }
    }, f531(f613(match v0.clone() {
        Adt26::C0(_, h3079, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3079
        }
    }), match v0 {
        Adt26::C0(h3080, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3080
        }
    }))))
}

pub fn f585(v0: Fn21, p3084: u64, p3085: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3083 = (p3084, p3085);
    {
        let mut m3083 = m3083;
        loop {
            let step = {
                let (v1, v2) = m3083;
                let m3081 = v1;
                if m3081 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3081 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3082: Fn21 = v0.clone();
                        c3082.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3086, _) = v0;
        h3086
    };
    let v2: u32 = {
        let (h3087, _) = {
            let (_, h3088) = v0;
            h3088
        };
        h3087
    };
    let v3: u32 = {
        let (_, h3089) = {
            let (_, h3090) = v0;
            h3090
        };
        h3089
    };
    ({
        let a3091 = v1;
        let a3092 = 2u64;
        let a3093 = 0u64;
        nat_quot(a3091, a3092, a3093)
    }, (f587(v2, 1u32), {
        let a3097 = {
            let a3094 = v1;
            let a3095 = 2u64;
            let a3096 = 0u64;
            nat_rem(a3094, a3095, a3096)
        };
        let a3098 = 1u64;
        if nat_eq(a3097, a3098) {
            let a3099 = v3;
            let a3100 = v2;
            fixed_u32::bit_or(a3099, a3100)
        } else {
            v3
        }
    }))
}

pub fn f587(v0: u32, v1: u32) -> u32 {
    let m3103 = {
        let a3101 = v0;
        let a3102 = v1;
        fixed_u32::shift_left(a3101, a3102)
    };
    match m3103 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f588(v0: Fn22, p3108: u64, p3109: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3107 = (p3108, p3109);
    {
        let mut m3107 = m3107;
        loop {
            let step = {
                let (v1, v2) = m3107;
                let m3104 = v1;
                if m3104 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3104 - 1;
                    let m3106 = {
                        let c3105: Fn22 = v0.clone();
                        c3105.apply(v2.clone())?
                    };
                    match m3106 {
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
                Ok(next) => { m3107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f589(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f614(v0)
}

pub fn f590(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3114 = {
            let a3112 = {
                let a3110 = v0.clone();
                length_list(a3110)
            };
            let a3113 = {
                let a3111 = v1.clone();
                length_list(a3111)
            };
            nat_add(a3112, a3113)?
        };
        let a3115 = 1u64;
        nat_add(a3114, a3115)?
    };
    Ok(f430({
        let (_, h3116) = {
            let (_, h3117) = {
                let (h3118, _) = f615(Fn23::F616, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3118
            };
            h3117
        };
        h3116
    }))
}

pub fn f591(v0: u64) -> Bytes {
    f46({
        let a3119 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3119)
    }, v0, 1u64)
}

pub fn f592(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3120 = v1;
    match m3120.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3121 = f617(v0, v3.clone())?;
            match m3121 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f618(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f52(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3122 = f592(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3122 {
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

pub fn f593(p3126: List<Bytes>) -> R<bool> {
    let m3125 = p3126;
    {
        let mut m3125 = m3125;
        loop {
            let step = {
                let v0 = m3125;
                let m3123 = v0;
                match m3123.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3124 = f619(v1)?;
                        match m3124 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f52(v3, f620(false, v2.clone())?) {
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
                Ok(next) => { m3125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f594(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m3127 = v0;
    match m3127.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f621(v1.clone(), {
                let (h3128, _) = v2.clone();
                h3128
            }) {
                List::cons({
                    let (h3129, _) = v2;
                    h3129
                }, f594(v3, v1))
            } else {
                f594(v3, v1)
            }
        }
    }
}

pub fn f595(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3130 = v2;
    match m3130.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3131 = f622(v4.clone())?;
            match m3131 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3132 = f623(v0, v6.clone())?;
                    match m3132 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f52(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3135 = f624(v4, {
                                    let a3133 = v1.clone();
                                    let a3134 = f369(v6)?;
                                    append_bytes(a3133, a3134)
                                })?;
                                match m3135 {
                                    Ok(v9) => {
                                        let m3136 = f595(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3136 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f526(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f568(v11.clone())), f489(f525(v9), f569(v11)))))
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

pub fn f596(v0: List<Adt2>) -> R<List<Bytes>> {
    let m3137 = v0;
    match m3137.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3138 = f622(v1)?;
            match m3138 {
                None => {
                    f596(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f596(v2)?))
                }
            }
        }
    }
}

pub fn f597(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f625(f593(f626(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f598(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3139 = f375(Some(v0));
    match m3139 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3140 = f559(v2)?;
            match m3140 {
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

pub fn f599(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m3141 = v2;
    match m3141.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3143 = f627(v0, {
                let (h3142, _) = v5.clone();
                h3142
            })?;
            match m3143 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3144, _) = v5;
                        h3144
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f52(v7.clone(), f628(v0, f600(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3149 = f572({
                            let (_, h3145) = v5.clone();
                            h3145
                        }, {
                            let a3147 = v1.clone();
                            let a3148 = f369({
                                let (h3146, _) = v5.clone();
                                h3146
                            })?;
                            append_bytes(a3147, a3148)
                        })?;
                        match m3149 {
                            Ok(v9) => {
                                let v10: Bytes = if f52({
                                    let (h3150, _) = v5;
                                    h3150
                                }, v3.clone()) {
                                    f526(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f526(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3151 = f599(v0, v1, v6, v3, v4)?;
                                match m3151 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f568(v12.clone())), f489(f525(v9), f569(v12)))))
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

pub fn f600(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m3152 = v0;
    match m3152.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h3153, _) = v1;
                h3153
            }, f600(v2))
        }
    }
}

pub fn f601(v0: Adt2) -> Option<Adt2> {
    let m3154 = f278(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3154 {
        None => {
            f629(v0)
        }
        Some(v1) => {
            if f602(v1.clone()) {
                let m3155 = f629(v0);
                match m3155 {
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

pub fn f602(v0: Adt2) -> bool {
    let m3156 = v0;
    match m3156 {
        Adt2::C0(v1) => {
            !v1
        }
        Adt2::C1(_, _) => {
            false
        }
        Adt2::C2(_, _, _) => {
            false
        }
        Adt2::C3(_) => {
            false
        }
        Adt2::C4(_) => {
            false
        }
        Adt2::C5(_) => {
            false
        }
        Adt2::C6 => {
            false
        }
        Adt2::C7(_, _) => {
            false
        }
        Adt2::C8(_, _, _) => {
            false
        }
    }
}

pub fn f603(v0: Adt2) -> Option<Adt2> {
    let m3157 = f283(f278(v0, f8(Str::lit("oneOf"))));
    match m3157.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3158 = v2;
            match m3158.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m3159 = v4;
                    match m3159.uncons() {
                        None => {
                            if f630(v3.clone()) {
                                Some(v1)
                            } else {
                                if f630(v1) {
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

pub fn f604(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3160 = v1;
    match m3160 {
        Ok(v3) => {
            f563(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f526(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f525(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f605(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3161 = f603(v0.clone());
    match m3161 {
        None => {
            if f32(f281(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3162 = f278(v0, f8(Str::lit("items")));
                match m3162 {
                    None => {
                        Ok(f604(f8(Str::lit("List")), f606()))
                    }
                    Some(v3) => {
                        Ok(f604(f8(Str::lit("List")), f624(v3, {
                            let a3163 = v1;
                            let a3164 = f8(Str::lit("Item"));
                            append_bytes(a3163, a3164)
                        })?))
                    }
                }
            } else {
                f624(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f604(f8(Str::lit("Option")), f624(v2, v1)?))
        }
    }
}

pub fn f606() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f522(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f607(p3168: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m3167 = p3168;
    {
        let mut m3167 = m3167;
        loop {
            let step = {
                let v0 = m3167;
                let m3165 = v0;
                match m3165.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f52({
                            let (h3166, _) = v2;
                            h3166
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f608(p3172: List<Adt2>, v1: Bytes, p3173: List<Bytes>) -> bool {
    let m3171 = (p3172, p3173);
    {
        let mut m3171 = m3171;
        loop {
            let step = {
                let (v0, v2) = m3171;
                let m3169 = v0;
                match m3169.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3170 = f631(v3, v1.clone());
                        match m3170 {
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
                Ok(next) => { m3171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f609(p3176: List<Adt2>, p3177: List<Bytes>) -> bool {
    let m3175 = (p3176, p3177);
    {
        let mut m3175 = m3175;
        loop {
            let step = {
                let (v0, v1) = m3175;
                let m3174 = v0;
                match m3174.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f281(v2, f8(Str::lit("type")));
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
                Ok(next) => { m3175 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f610(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m3178 = f278(v0.clone(), f8(Str::lit("anyOf")));
        match m3178 {
            None => {
                Adt2::C6
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f607(f518(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f573(f376(v2))
    } else {
        false
    }
}

pub fn f611(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3179 = f498(v0, v1);
    let a3180 = f587(v0, v2);
    fixed_u32::bit_or(a3179, a3180)
}

pub fn f612(v0: u32) -> u32 {
    let a3183 = {
        let a3181 = f611(v0, 17u32, 15u32);
        let a3182 = f611(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3181, a3182)
    };
    let a3184 = f498(v0, 10u32);
    fixed_u32::bit_xor(a3183, a3184)
}

pub fn f613(v0: u32) -> u32 {
    let a3187 = {
        let a3185 = f611(v0, 7u32, 25u32);
        let a3186 = f611(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3185, a3186)
    };
    let a3188 = f498(v0, 3u32);
    fixed_u32::bit_xor(a3187, a3188)
}

pub fn f614(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3189, _) = v0;
        h3189
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3190) = v0;
        h3190
    };
    let m3191 = v1;
    match m3191.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3192 = v4;
            match m3192.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f632(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f615(v0: Fn23, p3197: u64, p3198: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3196 = (p3197, p3198);
    {
        let mut m3196 = m3196;
        loop {
            let step = {
                let (v1, v2) = m3196;
                let m3193 = v1;
                if m3193 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3193 - 1;
                    let m3195 = {
                        let c3194: Fn23 = v0.clone();
                        c3194.apply(v2.clone())
                    };
                    match m3195 {
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
                Ok(next) => { m3196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f616(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f633(v0)
}

pub fn f617(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3199 = v1;
    match m3199 {
        Adt2::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(v7, v8) => {
            Ok(Some({
                let a3210 = if f118(v7, {
                    let a3208 = 0u64;
                    let a3209 = v8;
                    nat_lt(a3208, a3209)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3211 = f297(false, v8)?;
                append_bytes(a3210, a3211)
            }))
        }
        Adt2::C2(v3, v4, v5) => {
            let m3202 = f634({
                let a3200 = f8(Str::lit("n"));
                let a3201 = f446(false, v4, v5)?;
                append_bytes(a3200, a3201)
            })?;
            match m3202 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3206 = f8(Str::lit("nMinus"));
                        let a3207 = f46(v6.clone(), 1u64, {
                            let a3204 = {
                                let a3203 = v6;
                                length_bytes(a3203)
                            };
                            let a3205 = 1u64;
                            nat_sub(a3204, a3205)
                        });
                        append_bytes(a3206, a3207)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C3(v2) => {
            f623(v0, v2)
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

pub fn f618(v0: Adt2) -> R<Bytes> {
    f374(v0)
}

pub fn f619(v0: Bytes) -> R<Option<Bytes>> {
    let m3212 = f635(v0)?;
    match m3212 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f636(v1)?))
        }
    }
}

pub fn f620(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3213 = v1;
    match m3213.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3214 = f623(v0, v2)?;
            match m3214 {
                None => {
                    f620(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f620(v0, v3)?))
                }
            }
        }
    }
}

pub fn f621(p3218: List<Adt2>, v1: Bytes) -> bool {
    let m3217 = p3218;
    {
        let mut m3217 = m3217;
        loop {
            let step = {
                let v0 = m3217;
                let m3215 = v0;
                match m3215.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3216 = f631(v2, v1.clone());
                        if match m3216 {
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
                Ok(next) => { m3217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(v0: Adt2) -> R<Option<Bytes>> {
    let m3219 = f375(f278(v0.clone(), f8(Str::lit("$ref"))));
    match m3219 {
        None => {
            Ok(f375(f278(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f559(v1)
        }
    }
}

pub fn f623(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f634(v1)
    } else {
        f619(v1)
    }
}

pub fn f624(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f637(f546(v0), v1)
}

pub fn f625(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3220 = v2;
    match m3220.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3221 = f631(v5.clone(), v3.clone());
            match m3221 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3222 = f623(v0, v7.clone())?;
                    match m3222 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f52(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3225 = f566({
                                    let a3223 = v1.clone();
                                    let a3224 = f369(v7)?;
                                    append_bytes(a3223, a3224)
                                }, f520(f485(f278(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f282(f283(f278(v5, f8(Str::lit("required"))))), true)?;
                                match m3225 {
                                    Ok(v10) => {
                                        let m3226 = f625(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3226 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f383(f568(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f568(v12.clone())), f489(f569(v10), f569(v12)))))
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

pub fn f626(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3227 = v0;
    match m3227.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3228 = f631(v2, v1.clone());
            match m3228 {
                None => {
                    f626(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f626(v3, v1))
                }
            }
        }
    }
}

pub fn f627(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3229 = f623(v0, v1)?;
    match m3229 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f32(v2.clone(), f8(Str::lit("mk"))) {
                f544(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f628(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3230 = v1;
    match m3230.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3231 = f627(v0, v2)?;
            match m3231 {
                None => {
                    f628(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f628(v0, v3)?))
                }
            }
        }
    }
}

pub fn f629(v0: Adt2) -> Option<Adt2> {
    let m3232 = f485(f278(v0, f8(Str::lit("patternProperties"))));
    match m3232.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3233 = v2;
            match m3233.uncons() {
                None => {
                    Some({
                        let (_, h3234) = v1;
                        h3234
                    })
                }
                Some((_, _)) => {
                    None::<Adt2>
                }
            }
        }
    }
}

pub fn f630(v0: Adt2) -> bool {
    f32(f281(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f631(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f52(v1.clone(), f282(f283(f278(v0.clone(), f8(Str::lit("required")))))) {
        let m3235 = f278(v0, f8(Str::lit("properties")));
        match m3235 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3236 = f278(v2, v1);
                match m3236 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f375(f278(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f632(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3241 = {
            let a3239 = {
                let a3237 = v0.clone();
                length_list(a3237)
            };
            let a3240 = {
                let a3238 = v1.clone();
                length_list(a3238)
            };
            nat_add(a3239, a3240)?
        };
        let a3242 = 1u64;
        nat_add(a3241, a3242)?
    };
    Ok(f638({
        let (_, h3243) = {
            let (_, h3244) = {
                let (h3245, _) = f639(Fn24::F640, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3245
            };
            h3244
        };
        h3243
    }))
}

pub fn f633(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3246, _) = v0.clone();
        h3246
    };
    let v2: List<(Bytes, ())> = {
        let (h3247, _) = {
            let (_, h3248) = v0.clone();
            h3248
        };
        h3247
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3249) = {
            let (_, h3250) = v0;
            h3250
        };
        h3249
    };
    let m3251 = v1.clone();
    match m3251.uncons() {
        None => {
            let m3252 = v2;
            match m3252.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3253 = v2.clone();
            match m3253.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f641({
                        let (h3254, _) = v8.clone();
                        h3254
                    }, {
                        let (h3255, _) = v6.clone();
                        h3255
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

pub fn f634(v0: Bytes) -> R<Option<Bytes>> {
    let m3257 = f642({
        let a3256 = v0.clone();
        length_bytes(a3256)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3257 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3259 = {
                let a3258 = v1.clone();
                length_bytes(a3258)
            };
            let a3260 = 0u64;
            if nat_eq(a3259, a3260) {
                let a3262 = {
                    let a3261 = v0;
                    length_bytes(a3261)
                };
                let a3263 = 0u64;
                if nat_eq(a3262, a3263) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f636(v1)?))
            }
        }
    }
}

pub fn f635(v0: Bytes) -> R<Option<Bytes>> {
    if f643({
        let a3264 = v0.clone();
        length_bytes(a3264)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f644(v0)?;
        if f645(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3267 = {
                let a3265 = v1.clone();
                let a3266 = 0u64;
                index_bytes(a3265, a3266)
            };
            match m3267 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f483(f72(v2)?) {
                        Ok(Some({
                            let a3268 = f8(Str::lit("n"));
                            let a3269 = v1;
                            append_bytes(a3268, a3269)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f646({
            let a3270 = v0.clone();
            length_bytes(a3270)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f636(v0: Bytes) -> R<Bytes> {
    if f52(v0.clone(), f647()) {
        f544(v0)
    } else {
        Ok(v0)
    }
}

pub fn f637(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3271 = v0.clone();
    match m3271 {
        Adt2::C0(v2) => {
            if v2 {
                Ok(f606())
            } else {
                Ok(f562(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C1(_, _) => {
            Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(_, _, _) => {
            Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C3(_) => {
            Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_) => {
            Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_) => {
            let m3272 = f648(f518(v0.clone()));
            match m3272 {
                None => {
                    let m3273 = f375(f278(v0.clone(), f8(Str::lit("$ref"))));
                    match m3273 {
                        None => {
                            let m3275 = f278(v0.clone(), f8(Str::lit("allOf")));
                            match m3275 {
                                None => {
                                    let m3280 = f278(v0.clone(), f8(Str::lit("enum")));
                                    match m3280 {
                                        None => {
                                            let m3281 = f278(v0.clone(), f8(Str::lit("const")));
                                            match m3281 {
                                                None => {
                                                    let m3283 = f278(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3283 {
                                                        None => {
                                                            let v30: Bytes = f281(v0.clone(), f8(Str::lit("type")));
                                                            if if f32(v30.clone(), Bytes::lit(&[])) {
                                                                !f649(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f606())
                                                            } else {
                                                                if f32(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f563(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f32(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f563(f650(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f32(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f563(f651(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f32(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f563(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f32(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m3284 = f485(f278(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3284.uncons() {
                                                                                        None => {
                                                                                            if f570(v0.clone()) {
                                                                                                Ok(f563(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3285 = f601(v0);
                                                                                                match m3285 {
                                                                                                    None => {
                                                                                                        Ok(f604(f8(Str::lit("List")), f652(f606())))
                                                                                                    }
                                                                                                    Some(v33) => {
                                                                                                        Ok(f604(f8(Str::lit("List")), f652(f653(v33, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f563(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f562(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f563(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m3282 = f375(Some(v27));
                                                    match m3282 {
                                                        None => {
                                                            Ok(f562(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f563(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f563(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m3276 = f376(v19);
                                    match m3276.uncons() {
                                        None => {
                                            Ok(f562(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m3277 = v21;
                                            match m3277.uncons() {
                                                None => {
                                                    let m3278 = f375(f278(v20, f8(Str::lit("$ref"))));
                                                    match m3278 {
                                                        None => {
                                                            Ok(f562(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m3279 = f559(v24)?;
                                                            match m3279 {
                                                                None => {
                                                                    Ok(f562(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f563(v25, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f562(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m3274 = f559(v17)?;
                            match m3274 {
                                None => {
                                    Ok(f562(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f563(v18, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f562(if f32(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C6 => {
            Ok(f606())
        }
        Adt2::C7(_, _) => {
            Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C8(_, _, _) => {
            Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f638(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f391(Fn6::F654, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f639(v0: Fn24, p3290: u64, p3291: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3289 = (p3290, p3291);
    {
        let mut m3289 = m3289;
        loop {
            let step = {
                let (v1, v2) = m3289;
                let m3286 = v1;
                if m3286 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3286 - 1;
                    let m3288 = {
                        let c3287: Fn24 = v0.clone();
                        c3287.apply(v2.clone())
                    };
                    match m3288 {
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
                Ok(next) => { m3289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f655(v0)
}

pub fn f641(v0: Bytes, v1: Bytes) -> bool {
    let a3296 = {
        let a3292 = v0;
        let a3293 = v1;
        compare_bytes(a3292, a3293)
    };
    let a3297 = {
        let a3294 = Bytes::lit(&[]);
        let a3295 = Bytes::lit(&[0x01]);
        compare_bytes(a3294, a3295)
    };
    equal(a3296, a3297)
}

pub fn f642(p3310: u64, v1: Bytes, p3311: u64, p3312: Bytes) -> R<Option<Bytes>> {
    let m3309 = (p3310, (p3311, p3312));
    {
        let mut m3309 = m3309;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3309;
                let m3298 = v0;
                if m3298 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3298 - 1;
                    let a3300 = {
                        let a3299 = v1.clone();
                        length_bytes(a3299)
                    };
                    let a3301 = v2;
                    if nat_le(a3300, a3301) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3302 = f656(v1.clone(), v2)?;
                        match m3302 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3304 = v2;
                                    let a3305 = {
                                        let (_, h3303) = v5;
                                        h3303
                                    };
                                    nat_add(a3304, a3305)?
                                }, {
                                    let a3307 = v3;
                                    let a3308 = f657(v1.clone(), v2, {
                                        let (h3306, _) = v5;
                                        h3306
                                    });
                                    append_bytes(a3307, a3308)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3309 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f643(p3322: u64, v1: Bytes, p3323: u64) -> R<bool> {
    let m3321 = (p3322, p3323);
    {
        let mut m3321 = m3321;
        loop {
            let step = {
                let (v0, v2) = m3321;
                let m3313 = v0;
                if m3313 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3313 - 1;
                    let m3316 = {
                        let a3314 = v1.clone();
                        let a3315 = v2;
                        index_bytes(a3314, a3315)
                    };
                    match m3316 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3317 = f72(v4)?;
                            let a3318 = 128u64;
                            if nat_lt(a3317, a3318) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3319 = v2;
                                    let a3320 = 1u64;
                                    nat_add(a3319, a3320)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f442(v0.clone(), 0u64, {
        let a3324 = v0;
        length_bytes(a3324)
    }, false, Bytes::lit(&[]))?;
    let m3327 = {
        let a3325 = v1.clone();
        let a3326 = 0u64;
        index_bytes(a3325, a3326)
    };
    match m3327 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f482(v3) {
                let a3334 = f46({
                    let a3328 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3328)
                }, {
                    let a3329 = v3;
                    let a3330 = 65u64;
                    nat_sub(a3329, a3330)
                }, 1u64);
                let a3335 = f46(v1.clone(), 1u64, {
                    let a3332 = {
                        let a3331 = v1;
                        length_bytes(a3331)
                    };
                    let a3333 = 1u64;
                    nat_sub(a3332, a3333)
                });
                Ok(append_bytes(a3334, a3335))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f645(v0: Bytes) -> R<bool> {
    let m3338 = {
        let a3336 = v0;
        let a3337 = 0u64;
        index_bytes(a3336, a3337)
    };
    match m3338 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f206(f481(f72(v1)?), f482(f72(v1)?)))
        }
    }
}

pub fn f646(p3351: u64, v1: Bytes, p3352: u64, p3353: Bytes) -> R<Option<Bytes>> {
    let m3350 = (p3351, (p3352, p3353));
    {
        let mut m3350 = m3350;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3350;
                let m3339 = v0;
                if m3339 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3339 - 1;
                    let a3341 = {
                        let a3340 = v1.clone();
                        length_bytes(a3340)
                    };
                    let a3342 = v2;
                    if nat_le(a3341, a3342) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3343 = f656(v1.clone(), v2)?;
                        match m3343 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3345 = v2;
                                    let a3346 = {
                                        let (_, h3344) = v5;
                                        h3344
                                    };
                                    nat_add(a3345, a3346)?
                                }, {
                                    let a3348 = v3;
                                    let a3349 = f658({
                                        let (h3347, _) = v5;
                                        h3347
                                    });
                                    append_bytes(a3348, a3349)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f647() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f648(p3358: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m3357 = p3358;
    {
        let mut m3357 = m3357;
        loop {
            let step = {
                let v0 = m3357;
                let m3354 = v0;
                match m3354.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h3355, _) = v1.clone();
                            h3355
                        }, f659()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h3356, _) = v1;
                                h3356
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f649(v0: Adt2) -> bool {
    if f575(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f575(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f575(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f575(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f575(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f575(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f650(v0: Adt2) -> Bytes {
    let m3359 = f660(f278(v0, f8(Str::lit("minimum"))));
    match m3359 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f651() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f652(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3360 = v0;
    match m3360 {
        Ok(v2) => {
            f563(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f526(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f525(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f653(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f661(16u64, v0, v1)
}

pub fn f654(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f655(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3361, _) = v0.clone();
        h3361
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3362, _) = {
            let (_, h3363) = v0.clone();
            h3363
        };
        h3362
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3364) = {
            let (_, h3365) = v0;
            h3365
        };
        h3364
    };
    let m3366 = v1.clone();
    match m3366.uncons() {
        None => {
            let m3367 = v2;
            match m3367.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3368 = v2.clone();
            match m3368.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f641({
                        let (h3369, _) = v8.clone();
                        h3369
                    }, {
                        let (h3370, _) = v6.clone();
                        h3370
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

pub fn f656(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3373 = {
        let a3371 = v0.clone();
        let a3372 = v1;
        index_bytes(a3371, a3372)
    };
    match m3373 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            let a3374 = v3;
            let a3375 = 128u64;
            if nat_lt(a3374, a3375) {
                Ok(Some((v3, 1u64)))
            } else {
                if f118({
                    let a3376 = 194u64;
                    let a3377 = v3;
                    nat_le(a3376, a3377)
                }, {
                    let a3378 = v3;
                    let a3379 = 223u64;
                    nat_le(a3378, a3379)
                }) {
                    let m3382 = f662(v0, {
                        let a3380 = v1;
                        let a3381 = 1u64;
                        nat_add(a3380, a3381)?
                    })?;
                    match m3382 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f663({
                                let a3387 = {
                                    let a3385 = {
                                        let a3383 = v3;
                                        let a3384 = 192u64;
                                        nat_sub(a3383, a3384)
                                    };
                                    let a3386 = 64u64;
                                    nat_mul(a3385, a3386)?
                                };
                                let a3388 = v4;
                                nat_add(a3387, a3388)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f118({
                        let a3389 = 224u64;
                        let a3390 = v3;
                        nat_le(a3389, a3390)
                    }, {
                        let a3391 = v3;
                        let a3392 = 239u64;
                        nat_le(a3391, a3392)
                    }) {
                        let m3395 = f662(v0.clone(), {
                            let a3393 = v1;
                            let a3394 = 1u64;
                            nat_add(a3393, a3394)?
                        })?;
                        match m3395 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3398 = f662(v0, {
                                    let a3396 = v1;
                                    let a3397 = 2u64;
                                    nat_add(a3396, a3397)?
                                })?;
                                match m3398 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f663({
                                            let a3407 = {
                                                let a3401 = {
                                                    let a3399 = v3;
                                                    let a3400 = 224u64;
                                                    nat_sub(a3399, a3400)
                                                };
                                                let a3402 = 4096u64;
                                                nat_mul(a3401, a3402)?
                                            };
                                            let a3408 = {
                                                let a3405 = {
                                                    let a3403 = v5;
                                                    let a3404 = 64u64;
                                                    nat_mul(a3403, a3404)?
                                                };
                                                let a3406 = v6;
                                                nat_add(a3405, a3406)?
                                            };
                                            nat_add(a3407, a3408)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f118({
                            let a3409 = 240u64;
                            let a3410 = v3;
                            nat_le(a3409, a3410)
                        }, {
                            let a3411 = v3;
                            let a3412 = 244u64;
                            nat_le(a3411, a3412)
                        }) {
                            let m3415 = f662(v0.clone(), {
                                let a3413 = v1;
                                let a3414 = 1u64;
                                nat_add(a3413, a3414)?
                            })?;
                            match m3415 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3418 = f662(v0.clone(), {
                                        let a3416 = v1;
                                        let a3417 = 2u64;
                                        nat_add(a3416, a3417)?
                                    })?;
                                    match m3418 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3421 = f662(v0, {
                                                let a3419 = v1;
                                                let a3420 = 3u64;
                                                nat_add(a3419, a3420)?
                                            })?;
                                            match m3421 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f663({
                                                        let a3434 = {
                                                            let a3424 = {
                                                                let a3422 = v3;
                                                                let a3423 = 240u64;
                                                                nat_sub(a3422, a3423)
                                                            };
                                                            let a3425 = 262144u64;
                                                            nat_mul(a3424, a3425)?
                                                        };
                                                        let a3435 = {
                                                            let a3432 = {
                                                                let a3426 = v7;
                                                                let a3427 = 4096u64;
                                                                nat_mul(a3426, a3427)?
                                                            };
                                                            let a3433 = {
                                                                let a3430 = {
                                                                    let a3428 = v8;
                                                                    let a3429 = 64u64;
                                                                    nat_mul(a3428, a3429)?
                                                                };
                                                                let a3431 = v9;
                                                                nat_add(a3430, a3431)?
                                                            };
                                                            nat_add(a3432, a3433)?
                                                        };
                                                        nat_add(a3434, a3435)?
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

pub fn f657(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f206(f118(f206(f481(v2), f482(v2)), f118({
        let a3436 = v2;
        let a3437 = 117u64;
        !nat_eq(a3436, a3437)
    }, {
        let a3438 = v2;
        let a3439 = 85u64;
        !nat_eq(a3438, a3439)
    })), f118(f483(v2), {
        let a3440 = 0u64;
        let a3441 = v1;
        nat_lt(a3440, a3441)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a3442 = v2;
        let a3443 = 65536u64;
        if nat_lt(a3442, a3443) {
            let a3444 = f8(Str::lit("u"));
            let a3445 = f560(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3444, a3445)
        } else {
            let a3446 = f8(Str::lit("U"));
            let a3447 = f560(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3446, a3447)
        }
    }
}

pub fn f658(v0: u64) -> Bytes {
    let a3448 = f8(Str::lit("u"));
    let a3449 = f560(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3448, a3449)
}

pub fn f659() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f660(v0: Option<Adt2>) -> Option<u64> {
    let m3450 = v0;
    match m3450 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m3451 = v1;
            match m3451 {
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

pub fn f661(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3452 = v0;
    if m3452 == 0 {
        Ok(f562(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3452 - 1;
        let m3453 = v1.clone();
        match m3453 {
            Adt2::C0(v4) => {
                if v4 {
                    Ok(f606())
                } else {
                    Ok(f562(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C1(_, _) => {
                Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(_, _, _) => {
                Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C3(_) => {
                Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_) => {
                Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_) => {
                let m3454 = f375(f278(v1.clone(), f8(Str::lit("$ref"))));
                match m3454 {
                    None => {
                        if if f575(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f575(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f575(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f563(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v20: Bytes = f281(v1.clone(), f8(Str::lit("type")));
                            if if f32(v20.clone(), Bytes::lit(&[])) {
                                !f649(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f606())
                            } else {
                                if f32(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f563(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f32(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f563(f650(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f32(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f563(f651(), List::<Adt25>::nil()))
                                        } else {
                                            if f32(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f563(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f32(v20.clone(), f8(Str::lit("array"))) {
                                                    let m3456 = f278(v1, f8(Str::lit("items")));
                                                    match m3456 {
                                                        None => {
                                                            Ok(f604(f8(Str::lit("List")), f606()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f604(f8(Str::lit("List")), f661(v3, v21, {
                                                                let a3457 = v2;
                                                                let a3458 = f8(Str::lit("Item"));
                                                                append_bytes(a3457, a3458)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v20.clone(), f8(Str::lit("object"))) {
                                                        if f570(v1.clone()) {
                                                            Ok(f563(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3459 = f601(v1);
                                                            match m3459 {
                                                                None => {
                                                                    Ok(f604(f8(Str::lit("List")), f652(f606())))
                                                                }
                                                                Some(v22) => {
                                                                    Ok(f604(f8(Str::lit("List")), f652(f661(v3, v22, {
                                                                        let a3460 = v2;
                                                                        let a3461 = f8(Str::lit("Value"));
                                                                        append_bytes(a3460, a3461)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f562(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
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
                        let m3455 = f559(v18)?;
                        match m3455 {
                            None => {
                                Ok(f562(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f563(v19, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C6 => {
                Ok(f606())
            }
            Adt2::C7(_, _) => {
                Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C8(_, _, _) => {
                Ok(f562(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f662(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3464 = {
        let a3462 = v0;
        let a3463 = v1;
        index_bytes(a3462, a3463)
    };
    match m3464 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f118({
                let a3465 = 128u64;
                let a3466 = v3;
                nat_le(a3465, a3466)
            }, {
                let a3467 = v3;
                let a3468 = 191u64;
                nat_le(a3467, a3468)
            }) {
                Ok(Some({
                    let a3469 = v3;
                    let a3470 = 128u64;
                    nat_sub(a3469, a3470)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f663(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f118({
        let a3471 = v1;
        let a3472 = v0;
        nat_le(a3471, a3472)
    }, f118({
        let a3473 = v0;
        let a3474 = v2;
        nat_le(a3473, a3474)
    }, f206({
        let a3475 = v0;
        let a3476 = 55296u64;
        nat_lt(a3475, a3476)
    }, {
        let a3477 = 57343u64;
        let a3478 = v0;
        nat_lt(a3477, a3478)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
