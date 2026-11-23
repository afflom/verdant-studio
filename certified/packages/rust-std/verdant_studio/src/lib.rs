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
    C0(Bytes, Adt14),
    C1(Bytes),
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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
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
    F109(u8, Bytes),
    F148(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F109(k0, k1) => f109(*k0, k1.clone(), p0),
            Fn0::F148(k0) => f148(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F127,
    F162,
    F173,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F127 => f127(p0, p1),
            Fn1::F162 => f162(p0, p1),
            Fn1::F173 => f173(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F144(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F144(k0) => f144(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F187(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt21)) -> R<(u64, Adt21)> {
        match self {
            Fn3::F187(k0, k1, k2) => f187(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F222,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn4::F222 => f222(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F225,
}

impl Fn5 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn5::F225 => f225(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F227,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn6::F227 => f227(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F230,
}

impl Fn7 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn7::F230 => f230(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F236(Bytes),
    F237(u64, Bytes),
    F288(Bytes),
    F331(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn8::F236(k0) => Ok(f236(k0.clone(), p0)),
            Fn8::F237(k0, k1) => f237(*k0, k1.clone(), p0),
            Fn8::F288(k0) => f288(k0.clone(), p0),
            Fn8::F331(k0) => f331(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F249(Bytes),
    F296(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F249(k0) => f249(k0.clone(), p0),
            Fn9::F296(k0, k1) => Ok(f296(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F251,
    F361,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F251 => f251(p0, p1),
            Fn10::F361 => f361(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F254,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F254 => f254(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F256,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F256 => f256(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F259,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F259 => f259(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F261,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt23) -> R<Adt23> {
        match self {
            Fn14::F261 => f261(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F266(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F266(k0) => f266(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F291(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F291(k0) => f291(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F300,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F300 => f300(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F302,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F302 => f302(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F304,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F304 => f304(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F312,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F312 => f312(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F323,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F323 => f323(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F345,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F345 => f345(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F347,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F347 => f347(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F349(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F349(k0) => f349(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F360,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F360 => f360(p0),
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
                }), Adt9::C0(f4(f5()))))
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
                        Ok(f12(v3))
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
                                    }), {
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
                                }), Adt9::C0(f4(f19()))))
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
                    Ok((f14(v0, 1u64, v2.clone()), Adt9::C0(f4(v2))))
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
                    Ok((f14(v0, 2u64, v5.clone()), Adt9::C2(f4(v5))))
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

pub fn f4(v0: Bytes) -> Str {
    let m78 = {
        let a77 = v0;
        utf8_decode(a77)
    };
    match m78 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f5() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.err"))
}

pub fn f6(v0: Adt0, v1: u8, v2: Bytes, v3: Bytes) -> (Adt0, Adt9) {
    (f14(v0, 9u64, Bytes::lit(&[])), Adt9::C13(v1, v2, v3))
}

pub fn f7(v0: List<Bytes>) -> Bytes {
    let m79 = v0;
    match m79.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, v2)) => {
            let a80 = v1;
            let a81 = f7(v2);
            append_bytes(a80, a81)
        }
    }
}

pub fn f8(v0: Str) -> Bytes {
    let a82 = v0;
    utf8_encode(a82)
}

pub fn f9(v0: Adt0, v1: Option<Bytes>) -> R<(Adt0, Adt9)> {
    let v2: Adt1 = match v0.clone() {
        Adt0::C0(_, _, h83, _, _, _, _, _, _, _) => {
            h83
        }
    };
    f2(f21(v0.clone(), Adt1::C0(List::cons((match v0 {
        Adt0::C0(_, _, _, _, h84, _, _, _, _, _) => {
            h84
        }
    }, v1), match v2.clone() {
        Adt1::C0(h85, _, _) => {
            h85
        }
    }), match v2.clone() {
        Adt1::C0(_, h86, _) => {
            h86
        }
    }, match v2 {
        Adt1::C0(_, _, h87) => {
            h87
        }
    })))
}

pub fn f10(v0: Adt0, v1: List<Bytes>) -> R<(Adt0, Adt9)> {
    let v2: Adt1 = match v0.clone() {
        Adt0::C0(_, _, h88, _, _, _, _, _, _, _) => {
            h88
        }
    };
    f2(f21(v0.clone(), Adt1::C0(match v2.clone() {
        Adt1::C0(h89, _, _) => {
            h89
        }
    }, List::cons((match v0 {
        Adt0::C0(_, _, _, _, h90, _, _, _, _, _) => {
            h90
        }
    }, v1), match v2.clone() {
        Adt1::C0(_, h91, _) => {
            h91
        }
    }), match v2 {
        Adt1::C0(_, _, h92) => {
            h92
        }
    })))
}

pub fn f11(v0: Adt0, v1: Bytes) -> (Adt0, Adt9) {
    f6(v0.clone(), 1u8, Bytes::lit(&[]), f7(List::cons(v1, List::cons(f8(Str::lit(": ")), List::cons(match v0 {
        Adt0::C0(_, _, _, _, h93, _, _, _, _, _) => {
            h93
        }
    }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f12(v0: Adt0) -> (Adt0, Adt9) {
    (f14(v0.clone(), 4u64, match v0.clone() {
        Adt0::C0(_, _, _, _, h94, _, _, _, _, _) => {
            h94
        }
    }), Adt9::C10(Adt10::C0(f4(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h95, _, _, _, _) => {
            h95
        }
    }), f22(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h96, _, _, _) => {
            h96
        }
    }), Str::lit("."), f23(match v0 {
        Adt0::C0(_, h97, _, _, _, _, _, _, _, _) => {
            h97
        }
    }), f4(f19()), f4(f5()))))
}

pub fn f13(v0: Adt0) -> R<(Adt0, Adt9)> {
    let m99 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h98, _) => {
            h98
        }
    };
    match m99.uncons() {
        None => {
            Ok(f6(v0.clone(), 0u8, match v0 {
                Adt0::C0(_, _, _, _, _, _, _, _, _, h100) => {
                    h100
                }
            }, Bytes::lit(&[])))
        }
        Some((v1, _)) => {
            Ok((f14(v0, 5u64, {
                let (h101, _) = v1.clone();
                h101
            }), Adt9::C4(f4(f24({
                let (h102, _) = v1;
                h102
            })?))))
        }
    }
}

pub fn f14(v0: Adt0, v1: u64, v2: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h103, _, _, _, _, _, _, _, _, _) => {
            h103
        }
    }, match v0.clone() {
        Adt0::C0(_, h104, _, _, _, _, _, _, _, _) => {
            h104
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h105, _, _, _, _, _, _, _) => {
            h105
        }
    }, v1, v2, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h106, _, _, _, _) => {
            h106
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h107, _, _, _) => {
            h107
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h108, _, _) => {
            h108
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h109, _) => {
            h109
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h110) => {
            h110
        }
    })
}

pub fn f15(v0: Adt0) -> (Adt0, Adt9) {
    (v0.clone(), Adt9::C11(List::cons(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h111, _, _) => {
            h111
        }
    }, List::<u64>::nil()), 600000u64))
}

pub fn f16(p118: List<(u64, Option<i32>)>, v1: u64) -> Option<Option<i32>> {
    let m117 = p118;
    {
        let mut m117 = m117;
        loop {
            let step = {
                let v0 = m117;
                let m112 = v0;
                match m112.uncons() {
                    None => {
                        Err::<List<(u64, Option<i32>)>, Option<Option<i32>>>(None::<Option<i32>>)
                    }
                    Some((v2, v3)) => {
                        let a114 = {
                            let (h113, _) = v2;
                            h113
                        };
                        let a115 = v1;
                        if nat_eq(a114, a115) {
                            Err::<List<(u64, Option<i32>)>, Option<Option<i32>>>(Some({
                                let (_, h116) = v2;
                                h116
                            }))
                        } else {
                            Ok::<List<(u64, Option<i32>)>, Option<Option<i32>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Option<i32>) -> bool {
    let m119 = v0;
    match m119 {
        None => {
            false
        }
        Some(v1) => {
            let a120 = v1;
            let a121 = 0i32;
            equal(a120, a121)
        }
    }
}

pub fn f18(v0: Adt0) -> R<(Adt0, Adt9)> {
    let v1: Adt1 = match v0.clone() {
        Adt0::C0(_, _, h122, _, _, _, _, _, _, _) => {
            h122
        }
    };
    f2(f21(v0.clone(), Adt1::C0(match v1.clone() {
        Adt1::C0(h123, _, _) => {
            h123
        }
    }, match v1.clone() {
        Adt1::C0(_, h124, _) => {
            h124
        }
    }, List::cons(match v0 {
        Adt0::C0(_, _, _, _, h125, _, _, _, _, _) => {
            h125
        }
    }, match v1 {
        Adt1::C0(_, _, h126) => {
            h126
        }
    }))))
}

pub fn f19() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f20(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m127 = v0;
    match m127.uncons() {
        None => {
            Ok(Adt11::C4(f25()))
        }
        Some((v2, v3)) => {
            if f26(v2.clone(), f8(Str::lit("check"))) {
                f27(v1)
            } else {
                if f26(v2.clone(), f8(Str::lit("next"))) {
                    f28(v1, v3)
                } else {
                    if f26(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f29(v1))
                    } else {
                        if f26(v2.clone(), f8(Str::lit("publish"))) {
                            f30(v1)
                        } else {
                            if f26(v2.clone(), f8(Str::lit("agenda"))) {
                                f31(v1)
                            } else {
                                let m128 = v3;
                                match m128.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f25()))
                                    }
                                    Some((v4, v5)) => {
                                        if f26(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f32(v1, v4)
                                        } else {
                                            if f26(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                f33(v1, v4)
                                            } else {
                                                if f26(v2.clone(), f8(Str::lit("admit"))) {
                                                    f34(v1, v4)
                                                } else {
                                                    if f26(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                        f35(v1, v4)
                                                    } else {
                                                        if f26(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                            f36(v1, v4)
                                                        } else {
                                                            if f26(v2, f8(Str::lit("propose-encoding"))) {
                                                                let m129 = v5;
                                                                match m129.uncons() {
                                                                    None => {
                                                                        Ok(Adt11::C4(f25()))
                                                                    }
                                                                    Some((v6, v7)) => {
                                                                        f37(v1, v4, v6, v7)
                                                                    }
                                                                }
                                                            } else {
                                                                Ok(Adt11::C4(f25()))
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

pub fn f21(v0: Adt0, v1: Adt1) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h130, _, _, _, _, _, _, _, _, _) => {
            h130
        }
    }, match v0.clone() {
        Adt0::C0(_, h131, _, _, _, _, _, _, _, _) => {
            h131
        }
    }, v1, 0u64, match v0.clone() {
        Adt0::C0(_, _, _, _, h132, _, _, _, _, _) => {
            h132
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h133, _, _, _, _) => {
            h133
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h134, _, _, _) => {
            h134
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h135, _, _) => {
            h135
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h136, _) => {
            h136
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h137) => {
            h137
        }
    })
}

pub fn f22(v0: List<Bytes>) -> List<Str> {
    let m138 = v0;
    match m138.uncons() {
        None => {
            List::<Str>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f4(v1), f22(v2))
        }
    }
}

pub fn f23(v0: List<(Bytes, Bytes)>) -> List<(Str, Str)> {
    let m139 = v0;
    match m139.uncons() {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f4({
                let (h140, _) = v1;
                h140
            }), f4({
                let (_, h141) = v1;
                h141
            })), f23(v2))
        }
    }
}

pub fn f24(v0: Bytes) -> R<Bytes> {
    let v1: List<Bytes> = f38(v0.clone(), 47u8)?;
    let v2: Bytes = f39(v1);
    let a144 = {
        let a142 = v0.clone();
        length_bytes(a142)
    };
    let a145 = {
        let a143 = v2.clone();
        length_bytes(a143)
    };
    if nat_eq(a144, a145) {
        Ok(f8(Str::lit(".")))
    } else {
        Ok(f40(v0.clone(), 0u64, {
            let a150 = {
                let a146 = v0;
                length_bytes(a146)
            };
            let a151 = {
                let a148 = {
                    let a147 = v2;
                    length_bytes(a147)
                };
                let a149 = 1u64;
                nat_add(a148, a149)?
            };
            nat_sub(a150, a151)
        }))
    }
}

pub fn f25() -> Bytes {
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | admit JSONL | merge-gate RULES | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f26(v0: Bytes, v1: Bytes) -> bool {
    let a152 = v0;
    let a153 = v1;
    equal(a152, a153)
}

pub fn f27(v0: Adt1) -> R<Adt11> {
    let m154 = f41(v0.clone(), f42());
    match m154.uncons() {
        None => {
            let m155 = f43(v0.clone(), f42())?;
            match m155.uncons() {
                None => {
                    let m158 = f44(match v0.clone() {
                        Adt1::C0(_, h156, _) => {
                            h156
                        }
                    }, f45(f46(), f47(match v0.clone() {
                        Adt1::C0(_, h157, _) => {
                            h157
                        }
                    }, false)));
                    match m158.uncons() {
                        None => {
                            let v7: (bool, Bytes) = f48(Adt13::C0(Adt12::C0(f49(f50(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f51(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f49(f50(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f52(f53(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f53(v0.clone(), f8(Str::lit("model/admissions.json")))?, f53(v0.clone(), f8(Str::lit("model/strata.json")))?, f54(f53(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f47(match v0 {
                                Adt1::C0(_, h159, _) => {
                                    h159
                                }
                            }, true)))?;
                            let (h160, _) = v7;
                            if h160 {
                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                    let (_, h161) = v7;
                                    h161
                                }))
                            } else {
                                Ok(Adt11::C4({
                                    let (_, h162) = v7;
                                    h162
                                }))
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

pub fn f28(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f55(f56(match v0.clone() {
        Adt1::C0(_, h163, _) => {
            h163
        }
    }, f57())) {
        Ok(Adt11::C1(List::cons(f57(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f58(f59(f57(), f60(match v0.clone() {
            Adt1::C0(_, h164, _) => {
                h164
            }
        }, f57())))?;
        let m165 = f41(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m165.uncons() {
            None => {
                let m166 = f61(v0, f62(v2, v1))?;
                match m166 {
                    None => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[])))
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

pub fn f29(v0: Adt1) -> Adt11 {
    if f55(f63(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h167) => {
            h167
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f64(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f55(f63(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h168) => {
                h168
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f64(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f30(v0: Adt1) -> R<Adt11> {
    let m169 = f41(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil())));
    match m169.uncons() {
        None => {
            Ok(Adt11::C3(List::cons((f65(), f66()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f67()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f68()), List::cons((f8(Str::lit("docs/model.md")), f69(v0)?), List::<(Bytes, Bytes)>::nil())))), f8(Str::lit("published .github/workflows/pages.yml, .github/workflows/operate.yml, .github/CODEOWNERS and docs/model.md\u{a}"))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f31(v0: Adt1) -> R<Adt11> {
    let m170 = f41(v0.clone(), List::cons(f70(), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil())));
    match m170.uncons() {
        None => {
            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f71(f72(f73(f74(f50(v0.clone(), f70()))?)), f75(f53(v0, f8(Str::lit("model/admissions.json")))?)))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f32(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m171 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m171.uncons() {
        None => {
            let v4: Bytes = f50(v0.clone(), v1);
            let v5: List<List<Bytes>> = f74(v4.clone())?;
            if f55(f76(f77(v5.clone()), f78())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f79(Str::lit("id"), v6.clone())?, List::cons(f79(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f79(Str::lit("artifact"), f70())?, List::cons(f79(Str::lit("checksum"), f80(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f70(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f81(f50(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f82(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f83(f72(f73(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f33(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m172 = f41(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m172.uncons() {
        None => {
            let m173 = f84(f85(f50(v0, v1))?)?;
            match m173 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f83(f86(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f34(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m174 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil())))));
    match m174.uncons() {
        None => {
            let v4: List<Bytes> = f85(f50(v0.clone(), v1))?;
            let m175 = f87(v4.clone(), f8(Str::lit("requirement")))?;
            match m175 {
                None => {
                    Ok(Adt11::C4(f88()))
                }
                Some(v5) => {
                    let m176 = f87(v4, f8(Str::lit("admission")))?;
                    match m176 {
                        None => {
                            Ok(Adt11::C4(f88()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f53(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f53(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f45(f52(v5.clone(), f8(Str::lit("id"))), f52(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f58(f45(f52(v7.clone(), f8(Str::lit("id"))), f52(v8.clone(), f8(Str::lit("construct")))))?;
                            let a178 = {
                                let a177 = v9.clone();
                                length_list(a177)
                            };
                            let a179 = 0u64;
                            if nat_eq(a178, a179) {
                                Ok(Adt11::C4(f88()))
                            } else {
                                if f55({
                                    let a182 = {
                                        let a180 = f58(f45(v9.clone(), v10.clone()))?;
                                        length_list(a180)
                                    };
                                    let a183 = {
                                        let a181 = f45(v9.clone(), v10);
                                        length_list(a181)
                                    };
                                    nat_eq(a182, a183)
                                }) {
                                    Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                } else {
                                    Ok(Adt11::C3(List::cons(f89(f8(Str::lit("model/requirements.json")), f90(v7, v5))?, List::cons(f89(f8(Str::lit("model/admissions.json")), f90(v8, v6))?, List::cons(f82(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f83(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f35(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m184 = f41(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m184.uncons() {
        None => {
            let m185 = f91(v0, v1)?;
            match m185 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f92(f93(Some(v4))) {
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

pub fn f36(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m186 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m186.uncons() {
        None => {
            let m187 = f91(v0.clone(), v1.clone())?;
            match m187 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f55(f26(f94(v4.clone(), f8(Str::lit("schema"))), f95())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f94(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m188 = f96(v4.clone(), f8(Str::lit("source")));
                            match m188 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f97(f96(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f98(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f98(v9.clone(), f8(Str::lit("binding.json"))), f50(v0.clone(), v1));
                        let m189 = v8;
                        match m189 {
                            None => {
                                f99(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f55(f26(f94(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f99(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f98(v9, f100(f94(v7.clone(), f8(Str::lit("url"))))?);
                                    if f55(f63(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h190) => {
                                            h190
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f94(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m191 = f41(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m191.uncons() {
                                            None => {
                                                if f26(f80(f50(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f99(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f37(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m192 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil()))))));
    match m192.uncons() {
        None => {
            let m193 = f101(v3);
            match m193 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v6) => {
                    let m194 = f91(v0.clone(), v1.clone())?;
                    match m194 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                        }
                        Some(v7) => {
                            let v8: Bytes = {
                                let a195 = f8(Str::lit("Catalog."));
                                let a196 = f102(v2.clone())?;
                                append_bytes(a195, a196)
                            };
                            let v9: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f102(v2.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                            let m197 = f103(v7, v8.clone(), f102(v2.clone())?)?;
                            match m197 {
                                Ok(v11) => {
                                    Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f102(v2)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), f50(v0.clone(), v1)), List::cons((v9.clone(), v11), List::cons((f8(Str::lit("model/admissions.json")), f104(Adt14::C3(std::rc::Rc::new(f105(f106(f53(v0.clone(), f8(Str::lit("model/admissions.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v8.clone())), std::rc::Rc::new(v6)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f104(Adt14::C3(std::rc::Rc::new(f105(f106(f53(v0.clone(), f8(Str::lit("model/strata.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                        let a201 = f107(f50(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f40(v9.clone(), 4u64, {
                                            let a199 = {
                                                let a198 = v9;
                                                length_bytes(a198)
                                            };
                                            let a200 = 4u64;
                                            nat_sub(a199, a200)
                                        }))?;
                                        let a202 = Bytes::lit(&[0x0a]);
                                        append_bytes(a201, a202)
                                    }), List::cons(f82(v0, List::cons(v8.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v8, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                }
                                Err(v10) => {
                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v10, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f38(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a203 = v0.clone();
        length_bytes(a203)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h204, _) = f108(Fn0::F109(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h204
    };
    let v4: u64 = {
        let (h205, _) = {
            let (_, h206) = v3.clone();
            h206
        };
        h205
    };
    Ok(f110(List::cons(f40(v0, v4, {
        let a207 = v2;
        let a208 = v4;
        nat_sub(a207, a208)
    }), {
        let (_, h209) = {
            let (_, h210) = v3;
            h210
        };
        h209
    })))
}

pub fn f39(p214: List<Bytes>) -> Bytes {
    let m213 = p214;
    {
        let mut m213 = m213;
        loop {
            let step = {
                let v0 = m213;
                let m211 = v0;
                match m211.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m212 = v2.clone();
                        match m212.uncons() {
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
                Ok(next) => { m213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m218 = {
        let a215 = v0;
        let a216 = v1;
        let a217 = v2;
        slice_bytes(a215, a216, a217)
    };
    match m218 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f41(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m219 = v1;
    match m219.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m221 = f111(match v0.clone() {
                Adt1::C0(h220, _, _) => {
                    h220
                }
            }, v2.clone());
            match m221 {
                None => {
                    List::cons(v2, f41(v0, v3))
                }
                Some(_) => {
                    f41(v0, v3)
                }
            }
        }
    }
}

pub fn f42() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f70(), List::<Bytes>::nil())))))))
}

pub fn f43(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m222 = v1;
    match m222.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f112(v0.clone(), v2.clone()) {
                f113(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m223 = f91(v0.clone(), v2.clone())?;
                match m223 {
                    None => {
                        Ok(List::cons(v2, f43(v0, v3)?))
                    }
                    Some(_) => {
                        f43(v0, v3)
                    }
                }
            } else {
                f43(v0, v3)
            }
        }
    }
}

pub fn f44(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m224 = v1;
    match m224.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f56(v0.clone(), v2.clone()) {
                f44(v0, v3)
            } else {
                List::cons(v2, f44(v0, v3))
            }
        }
    }
}

pub fn f45(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m225 = v0;
    match m225.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f45(v3, v1))
        }
    }
}

pub fn f46() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f47(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m226 = v0;
    match m226.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f45(f114({
                let (h227, _) = v2;
                h227
            }, {
                let (_, h228) = v2;
                h228
            }, v1), f47(v3, v1))
        }
    }
}

pub fn f48(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h229, _, _, _, _) => {
            h229
        }
    };
    let v2: Adt15 = f115(v1.clone())?;
    let v3: List<Bytes> = f116(v1.clone());
    let v4: List<Bytes> = f52(match v0.clone() {
        Adt13::C0(_, _, h230, _, _) => {
            h230
        }
    }, f8(Str::lit("path")));
    let m232 = f117(v2, List::<u64>::nil(), f118(v1.clone(), match v0.clone() {
        Adt13::C0(_, h231, _, _, _) => {
            h231
        }
    }, 0u64)?);
    match m232 {
        None => {
            if f120(f121(match v0.clone() {
                Adt13::C0(_, _, _, _, h234) => {
                    h234
                }
            }, f45(match v0.clone() {
                Adt13::C0(_, _, _, _, h235) => {
                    h235
                }
            }, v4.clone()))?, f121(v4.clone(), f45(match v0.clone() {
                Adt13::C0(_, _, _, _, h236) => {
                    h236
                }
            }, v4))?, f122(match v0.clone() {
                Adt13::C0(_, _, h237, _, _) => {
                    h237
                }
            })) {
                let m241 = f123(f121(match v0.clone() {
                    Adt13::C0(_, _, _, h238, _) => {
                        h238
                    }
                }, f45(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h239, _) => {
                        h239
                    }
                }))?, f121(v3.clone(), f45(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h240, _) => {
                        h240
                    }
                }))?);
                match m241 {
                    None => {
                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f83(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f119(f45(v3, match v0 {
                            Adt13::C0(_, _, _, h242, _) => {
                                h242
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f119(match v1 {
                Adt12::C0(_, _, _, _, h233) => {
                    h233
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f49(v0: Bytes) -> R<List<Bytes>> {
    f124(f85(v0)?, f8(Str::lit("id = \"")))
}

pub fn f50(v0: Adt1, v1: Bytes) -> Bytes {
    let m244 = f111(match v0 {
        Adt1::C0(h243, _, _) => {
            h243
        }
    }, v1);
    match m244 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m245 = v2;
            match m245 {
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

pub fn f51(v0: Adt1) -> R<List<Bytes>> {
    let m246 = f74(f50(v0, f70()))?;
    match m246.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f72(v2))
        }
    }
}

pub fn f52(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m247 = v0;
    match m247.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f94(v2, v1.clone()), f52(v3, v1))
        }
    }
}

pub fn f53(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f93(f91(v0, v1)?))
}

pub fn f54(v0: List<Adt14>) -> List<Bytes> {
    let m248 = v0;
    match m248.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m249 = f97(Some(v1));
            match m249 {
                None => {
                    f54(v2)
                }
                Some(v3) => {
                    List::cons(v3, f54(v2))
                }
            }
        }
    }
}

pub fn f55(v0: bool) -> bool {
    !v0
}

pub fn f56(p253: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m252 = p253;
    {
        let mut m252 = m252;
        loop {
            let step = {
                let v0 = m252;
                let m250 = v0;
                match m250.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h251, _) = v2;
                            h251
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
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

pub fn f57() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f58(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f125(f126(Fn1::F127, List::<Bytes>::nil(), f128(v0)?)))
}

pub fn f59(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m254 = v1;
    match m254.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f113(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f98(v0.clone(), v2), f59(v0, v3))
            } else {
                f59(v0, v3)
            }
        }
    }
}

pub fn f60(p259: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m258 = p259;
    {
        let mut m258 = m258;
        loop {
            let step = {
                let v0 = m258;
                let m255 = v0;
                match m255.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h256, _) = v2;
                            h256
                        }, v1.clone()) {
                            let (_, h257) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h257)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt1, p262: List<Bytes>) -> R<Option<Bytes>> {
    let m261 = p262;
    {
        let mut m261 = m261;
        loop {
            let step = {
                let v1 = m261;
                let m260 = v1;
                match m260.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        if f129(v0.clone(), v2.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v2)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
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

pub fn f62(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m263 = v0;
    match m263.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f63(v2.clone(), v1.clone()) {
                f62(v3, v1)
            } else {
                List::cons(v2, f62(v3, v1))
            }
        }
    }
}

pub fn f63(v0: Bytes, p266: List<Bytes>) -> bool {
    let m265 = p266;
    {
        let mut m265 = m265;
        loop {
            let step = {
                let v1 = m265;
                let m264 = v1;
                match m264.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f26(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f65() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f66() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f67() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**', 'model/authorities.toml']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: operate\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          { echo \"proposed a change to the API's sources, exported again as $(basename studio/.lexlean/production/*/)\"; git diff --stat -- studio/src README.md; } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          if [ \"$ARGS\" = advance ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '[.[] | select(.title == \"verdant_studio advance\")] | length')\" != 0 ]; then\u{a}            echo \"::notice::an advance pull request is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}            exit 0\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#ingest-authority }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          git push origin \"HEAD:refs/heads/${branch}\"\u{a}          gh pr create --head \"$branch\" --base main --title \"verdant_studio $ARGS\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f68() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f69(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f130(f53(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f131(f53(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f70() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f71(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m267 = v0;
    match m267.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f63(v2.clone(), v1.clone()) {
                f71(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f71(v3, v1))
            }
        }
    }
}

pub fn f72(v0: List<List<Bytes>>) -> List<Bytes> {
    let m268 = v0;
    match m268.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m269 = v1;
            match m269.uncons() {
                None => {
                    f72(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f72(v2))
                }
            }
        }
    }
}

pub fn f73(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m270 = v0;
    match m270.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f74(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt16 = f132(v0.clone(), 0u64, {
        let a271 = v0;
        length_bytes(a271)
    }, Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt16 = {
        let a274 = {
            let a273 = match v1.clone() {
                Adt16::C0(_, _, h272, _, _) => {
                    h272
                }
            };
            length_bytes(a273)
        };
        let a275 = 0u64;
        if if nat_eq(a274, a275) {
            let m277 = match v1.clone() {
                Adt16::C0(_, _, _, h276, _) => {
                    h276
                }
            };
            match m277.uncons() {
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
            f133(v1)
        }
    };
    Ok(f134(match v4 {
        Adt16::C0(_, _, _, _, h278) => {
            h278
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f75(v0: List<Adt14>) -> List<Bytes> {
    let m279 = v0;
    match m279.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m280 = f96(v1, f8(Str::lit("std")));
            match m280 {
                None => {
                    f75(v2)
                }
                Some(v3) => {
                    List::cons(f94(v3, f8(Str::lit("row"))), f75(v2))
                }
            }
        }
    }
}

pub fn f76(p285: List<Bytes>, p286: List<Bytes>) -> bool {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v0, v1) = m284;
                let m281 = v0;
                match m281.uncons() {
                    None => {
                        let m282 = v1;
                        match m282.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m283 = v1;
                        match m283.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f26(v4, v6) {
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
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: List<List<Bytes>>) -> List<Bytes> {
    let m287 = v0;
    match m287.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f78() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f79(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f135(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f80(v0: Bytes) -> R<Bytes> {
    Ok(f136(f137(v0)?))
}

pub fn f81(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f138(f139(f85(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f140(f141(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f82(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f104(Adt14::C3(std::rc::Rc::new(f142(f58(f45(v1, f54(f53(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f83(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m288 = f143(Fn2::F144(v1), None::<Bytes>, v0);
    match m288 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f84(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m289 = v0;
    match m289.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f145(v1)?;
            let a291 = {
                let a290 = v3.clone();
                length_bytes(a290)
            };
            let a292 = 0u64;
            if nat_eq(a291, a292) {
                f84(v2)
            } else {
                let m293 = f146(v3.clone())?;
                match m293 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f94(v4.clone(), f8(Str::lit("id")));
                        if if f26(f94(v4, f8(Str::lit("schema"))), f95()) {
                            f55({
                                let a295 = {
                                    let a294 = v5.clone();
                                    length_bytes(a294)
                                };
                                let a296 = 0u64;
                                nat_eq(a295, a296)
                            })
                        } else {
                            false
                        } {
                            let m297 = f84(v2)?;
                            match m297 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f147(v5), {
                                        let a298 = v3;
                                        let a299 = Bytes::lit(&[0x0a]);
                                        append_bytes(a298, a299)
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

pub fn f85(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a300 = v0.clone();
        length_bytes(a300)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h301, _) = f108(Fn0::F148(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h301
    };
    let v3: u64 = {
        let (h302, _) = {
            let (_, h303) = v2.clone();
            h303
        };
        h302
    };
    let v4: List<Bytes> = {
        let (_, h304) = {
            let (_, h305) = v2;
            h305
        };
        h304
    };
    let a306 = v3;
    let a307 = v1;
    if nat_lt(a306, a307) {
        Ok(f110(List::cons(f40(v0, v3, {
            let a308 = v1;
            let a309 = v3;
            nat_sub(a308, a309)
        }), v4)))
    } else {
        Ok(f110(v4))
    }
}

pub fn f86(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m310 = v0;
    match m310.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h311, _) = v1;
                h311
            }, f86(v2))
        }
    }
}

pub fn f87(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m312 = v0;
    match m312.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f145(v2)?;
            let a314 = {
                let a313 = v4.clone();
                length_bytes(a313)
            };
            let a315 = 0u64;
            if nat_eq(a314, a315) {
                f87(v3, v1)
            } else {
                let m316 = f146(v4)?;
                match m316 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f149(v5.clone());
                        let a318 = {
                            let a317 = v6.clone();
                            length_bytes(a317)
                        };
                        let a319 = 0u64;
                        if nat_eq(a318, a319) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m320 = f87(v3, v1.clone())?;
                            match m320 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v7) => {
                                    if f26(v6, v1) {
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

pub fn f88() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f89(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f104(Adt14::C3(std::rc::Rc::new(f105(v1))))?))
}

pub fn f90(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m321 = v0;
    match m321.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f90(v3, v1))
        }
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f146(f50(v0, v1))
}

pub fn f92(p324: List<Adt14>) -> bool {
    let m323 = p324;
    {
        let mut m323 = m323;
        loop {
            let step = {
                let v0 = m323;
                let m322 = v0;
                match m322.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f26(f94(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f63(f8(Str::lit("studio")), f52(f150(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: Option<Adt14>) -> List<Adt14> {
    let m325 = v0;
    match m325 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f151(v1)
        }
    }
}

pub fn f94(v0: Adt14, v1: Bytes) -> Bytes {
    let m326 = f97(f96(v0, v1));
    match m326 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f95() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f96(p332: Adt14, v1: Bytes) -> Option<Adt14> {
    let m331 = p332;
    {
        let mut m331 = m331;
        loop {
            let step = {
                let v0 = m331;
                let m327 = v0;
                match m327 {
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
                    Adt14::C4(r328) => {
                        let v5 = (*r328).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r329, r330) => {
                        let v3 = (*r329).clone();
                        let v4 = (*r330).clone();
                        if f26(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m331 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: Option<Adt14>) -> Option<Bytes> {
    let m333 = v0;
    match m333 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m334 = v1;
            match m334 {
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

pub fn f98(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f99(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f81(f50(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f152(v1, v4)?)?), List::cons(f82(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f100(v0: Bytes) -> R<Bytes> {
    Ok(f39(f38(v0, 47u8)?))
}

pub fn f101(v0: List<Bytes>) -> Option<Adt14> {
    let m335 = v0;
    match m335.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f26(v1.clone(), f8(Str::lit("req"))) {
                let m336 = v2;
                match m336.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f26(v1.clone(), f8(Str::lit("std"))) {
                    let m337 = v2;
                    match m337.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m338 = v6;
                                match m338.uncons() {
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
                    if f26(v1, f8(Str::lit("der"))) {
                        let m339 = v2;
                        match m339.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m340 = v10;
                                match m340.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f142(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f102(v0: Bytes) -> R<Bytes> {
    f153(v0.clone(), 0u64, {
        let a341 = v0;
        length_bytes(a341)
    }, true, Bytes::lit(&[]))
}

pub fn f103(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m342 = f154(4096u64, f155(f156(f157(f96(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt17::C0(v2, v0), List::<Adt17>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m342 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f141(f158(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f104(v0: Adt14) -> R<Bytes> {
    let a343 = f159(v0)?;
    let a344 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a343, a344))
}

pub fn f105(v0: List<Adt14>) -> Adt14 {
    let m345 = v0;
    match m345.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f105(v2)))
        }
    }
}

pub fn f106(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    let m346 = v0;
    match m346.uncons() {
        None => {
            List::cons(v1, List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            List::cons(v2, f106(v3, v1))
        }
    }
}

pub fn f107(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f141(f160(f85(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f108(v0: Fn0, p351: u64, p352: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m350 = (p351, p352);
    {
        let mut m350 = m350;
        loop {
            let step = {
                let (v1, v2) = m350;
                let m347 = v1;
                if m347 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m347 - 1;
                    let m349 = {
                        let c348: Fn0 = v0.clone();
                        c348.apply(v2.clone())?
                    };
                    match m349 {
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
                Ok(next) => { m350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f161(v1, v0, v2)
}

pub fn f110(v0: List<Bytes>) -> List<Bytes> {
    f126(Fn1::F162, List::<Bytes>::nil(), v0)
}

pub fn f111(p357: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m356 = p357;
    {
        let mut m356 = m356;
        loop {
            let step = {
                let v0 = m356;
                let m353 = v0;
                match m353.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h354, _) = v2;
                            h354
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h355) = v2;
                                h355
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f112(v0: Adt1, v1: Bytes) -> bool {
    let m359 = f111(match v0 {
        Adt1::C0(h358, _, _) => {
            h358
        }
    }, v1);
    match m359 {
        None => {
            false
        }
        Some(v2) => {
            let m360 = v2;
            match m360 {
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

pub fn f113(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a361 = v0.clone();
        length_bytes(a361)
    };
    let v3: u64 = {
        let a362 = v1.clone();
        length_bytes(a362)
    };
    let a363 = v3;
    let a364 = v2;
    if nat_le(a363, a364) {
        let m370 = {
            let a367 = v0;
            let a368 = {
                let a365 = v2;
                let a366 = v3;
                nat_sub(a365, a366)
            };
            let a369 = v3;
            slice_bytes(a367, a368, a369)
        };
        match m370 {
            None => {
                false
            }
            Some(v4) => {
                let a371 = v4;
                let a372 = v1;
                equal(a371, a372)
            }
        }
    } else {
        false
    }
}

pub fn f114(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m373 = v1;
    match m373.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f163(v3.clone())
            } else {
                f55(f163(v3.clone()))
            } {
                List::cons(f98(v0.clone(), v3), f114(v0, v4, v2))
            } else {
                f114(v0, v4, v2)
            }
        }
    }
}

pub fn f115(v0: Adt12) -> R<Adt15> {
    Ok(Adt15::C0(f164(match v0.clone() {
        Adt12::C0(h374, _, _, _, _) => {
            h374
        }
    }, 0u64)?, f164(match v0.clone() {
        Adt12::C0(_, h375, _, _, _) => {
            h375
        }
    }, 0u64)?, f164(match v0.clone() {
        Adt12::C0(_, _, h376, _, _) => {
            h376
        }
    }, 0u64)?, f164(match v0 {
        Adt12::C0(_, _, _, h377, _) => {
            h377
        }
    }, 0u64)?))
}

pub fn f116(v0: Adt12) -> List<Bytes> {
    f45(match v0.clone() {
        Adt12::C0(h378, _, _, _, _) => {
            h378
        }
    }, f45(match v0.clone() {
        Adt12::C0(_, _, h379, _, _) => {
            h379
        }
    }, f45(match v0.clone() {
        Adt12::C0(_, _, _, h380, _) => {
            h380
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h381) => {
            h381
        }
    })))
}

pub fn f117(v0: Adt15, p387: List<u64>, p388: List<Adt18>) -> Option<u64> {
    let m386 = (p387, p388);
    {
        let mut m386 = m386;
        loop {
            let step = {
                let (v1, v2) = m386;
                let m382 = v2;
                match m382.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f165(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h383) => {
                                h383
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h384, _) => {
                                    h384
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h385, _) => {
                                    h385
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m389 = v1;
    match m389.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f166(v0.clone(), v3)?), f118(v0, v4, {
                let a390 = v2;
                let a391 = 1u64;
                nat_add(a390, a391)?
            })?))
        }
    }
}

pub fn f119(p395: List<Bytes>, p396: u64) -> Bytes {
    let m394 = (p395, p396);
    {
        let mut m394 = m394;
        loop {
            let step = {
                let (v0, v1) = m394;
                let m392 = v0;
                match m392.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m393 = v1;
                        if m393 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m393 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f167(v0.clone(), v1.clone()) {
        if f167(v1, v0) {
            f168(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f121(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m397 = v0;
    match m397.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f169(v2, v1.clone())?, f121(v3, v1)?))
        }
    }
}

pub fn f122(v0: List<Adt14>) -> List<u64> {
    let m398 = v0;
    match m398.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f170(v1), f122(v2))
        }
    }
}

pub fn f123(p401: List<u64>, v1: List<u64>) -> Option<u64> {
    let m400 = p401;
    {
        let mut m400 = m400;
        loop {
            let step = {
                let v0 = m400;
                let m399 = v0;
                match m399.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f171(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m400 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m402 = v0;
    match m402.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m403 = f172(v2, v1.clone())?;
            match m403 {
                None => {
                    f124(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f124(v3, v1)?))
                }
            }
        }
    }
}

pub fn f125(v0: List<Bytes>) -> List<Bytes> {
    f126(Fn1::F173, List::<Bytes>::nil(), v0)
}

pub fn f126(v0: Fn1, p407: List<Bytes>, p408: List<Bytes>) -> List<Bytes> {
    let m406 = (p407, p408);
    {
        let mut m406 = m406;
        loop {
            let step = {
                let (v1, v2) = m406;
                let m404 = v2;
                match m404.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c405: Fn1 = v0.clone();
                            c405.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m406 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f174(v0, v1)
}

pub fn f128(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f175(f176(f177(v0))?))
}

pub fn f129(v0: Adt1, v1: Bytes) -> R<bool> {
    let m409 = f91(v0.clone(), v1)?;
    match m409 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let v3: List<Bytes> = f178(f139(f85(f50(v0, f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f94(v2.clone(), f8(Str::lit("id"))))?;
            let m410 = v3.clone();
            match m410.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m412 = f97(f96({
                        let m411 = f96(v2.clone(), f8(Str::lit("source")));
                        match m411 {
                            None => {
                                Adt14::C5
                            }
                            Some(v6) => {
                                v6
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m412 {
                        None => {
                            Ok(false)
                        }
                        Some(v7) => {
                            Ok(f55(f63(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f135(f179(v2, v7))?, List::<Bytes>::nil()))), v3)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f130(v0: List<Adt14>) -> List<Bytes> {
    let m413 = v0;
    match m413.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f94(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f94(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f94(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f130(v2))
        }
    }
}

pub fn f131(v0: List<Adt14>) -> List<Bytes> {
    let m414 = v0;
    match m414.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f94(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f180(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f131(v2))
        }
    }
}

pub fn f132(v0: Bytes, p472: u64, p473: u64, p474: Adt16) -> R<Adt16> {
    let m471 = (p472, (p473, p474));
    {
        let mut m471 = m471;
        loop {
            let step = {
                let (v1, (v2, v3)) = m471;
                let m415 = v2;
                if m415 == 0 {
                    Err::<(u64, (u64, Adt16)), R<Adt16>>(Ok(v3))
                } else {
                    let v4 = m415 - 1;
                    let v5: u64 = f181(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt16::C0(_, h416, _, _, _) => {
                            h416
                        }
                    } {
                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                            let a417 = v1;
                            let a418 = 1u64;
                            nat_add(a417, a418)?
                        }, (v4, Adt16::C0(true, false, match v3.clone() {
                            Adt16::C0(_, _, h419, _, _) => {
                                h419
                            }
                        }, match v3.clone() {
                            Adt16::C0(_, _, _, h420, _) => {
                                h420
                            }
                        }, match v3 {
                            Adt16::C0(_, _, _, _, h421) => {
                                h421
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt16::C0(h422, _, _, _, _) => {
                                h422
                            }
                        } {
                            let a423 = v5;
                            let a424 = 34u64;
                            if nat_eq(a423, a424) {
                                let a427 = f181(v0.clone(), {
                                    let a425 = v1;
                                    let a426 = 1u64;
                                    nat_add(a425, a426)?
                                })?;
                                let a428 = 34u64;
                                if nat_eq(a427, a428) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a429 = v1;
                                        let a430 = 1u64;
                                        nat_add(a429, a430)?
                                    }, (v4, Adt16::C0(true, true, f182(match v3.clone() {
                                        Adt16::C0(_, _, h431, _, _) => {
                                            h431
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt16::C0(_, _, _, h432, _) => {
                                            h432
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h433) => {
                                            h433
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a434 = v1;
                                        let a435 = 1u64;
                                        nat_add(a434, a435)?
                                    }, (v4, Adt16::C0(false, false, match v3.clone() {
                                        Adt16::C0(_, _, h436, _, _) => {
                                            h436
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h437, _) => {
                                            h437
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h438) => {
                                            h438
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a439 = v1;
                                    let a440 = 1u64;
                                    nat_add(a439, a440)?
                                }, (v4, Adt16::C0(true, false, f182(match v3.clone() {
                                    Adt16::C0(_, _, h441, _, _) => {
                                        h441
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt16::C0(_, _, _, h442, _) => {
                                        h442
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h443) => {
                                        h443
                                    }
                                }))))
                            }
                        } else {
                            let a444 = v5;
                            let a445 = 34u64;
                            if nat_eq(a444, a445) {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a446 = v1;
                                    let a447 = 1u64;
                                    nat_add(a446, a447)?
                                }, (v4, Adt16::C0(true, false, match v3.clone() {
                                    Adt16::C0(_, _, h448, _, _) => {
                                        h448
                                    }
                                }, match v3.clone() {
                                    Adt16::C0(_, _, _, h449, _) => {
                                        h449
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h450) => {
                                        h450
                                    }
                                }))))
                            } else {
                                let a451 = v5;
                                let a452 = 44u64;
                                if nat_eq(a451, a452) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a453 = v1;
                                        let a454 = 1u64;
                                        nat_add(a453, a454)?
                                    }, (v4, Adt16::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt16::C0(_, _, h455, _, _) => {
                                            h455
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h456, _) => {
                                            h456
                                        }
                                    }), match v3 {
                                        Adt16::C0(_, _, _, _, h457) => {
                                            h457
                                        }
                                    }))))
                                } else {
                                    let a458 = v5;
                                    let a459 = 13u64;
                                    if nat_eq(a458, a459) {
                                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                            let a460 = v1;
                                            let a461 = 1u64;
                                            nat_add(a460, a461)?
                                        }, (v4, v3)))
                                    } else {
                                        let a462 = v5;
                                        let a463 = 10u64;
                                        if nat_eq(a462, a463) {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a464 = v1;
                                                let a465 = 1u64;
                                                nat_add(a464, a465)?
                                            }, (v4, f133(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a466 = v1;
                                                let a467 = 1u64;
                                                nat_add(a466, a467)?
                                            }, (v4, Adt16::C0(false, false, f182(match v3.clone() {
                                                Adt16::C0(_, _, h468, _, _) => {
                                                    h468
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt16::C0(_, _, _, h469, _) => {
                                                    h469
                                                }
                                            }, match v3 {
                                                Adt16::C0(_, _, _, _, h470) => {
                                                    h470
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
                Ok(next) => { m471 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: Adt16) -> Adt16 {
    Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f183(List::cons(match v0.clone() {
        Adt16::C0(_, _, h475, _, _) => {
            h475
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h476, _) => {
            h476
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt16::C0(_, _, _, _, h477) => {
            h477
        }
    }))
}

pub fn f134(p480: List<List<Bytes>>, p481: List<List<Bytes>>) -> List<List<Bytes>> {
    let m479 = (p480, p481);
    {
        let mut m479 = m479;
        loop {
            let step = {
                let (v0, v1) = m479;
                let m478 = v0;
                match m478.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f184(v0.clone(), 0u64, {
        let a482 = v0;
        length_bytes(a482)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f136(v0: Adt21) -> Bytes {
    let a503 = f185(match v0.clone() {
        Adt21::C0(h483, _, _, _, _, _, _, _) => {
            h483
        }
    });
    let a504 = {
        let a501 = f185(match v0.clone() {
            Adt21::C0(_, h484, _, _, _, _, _, _) => {
                h484
            }
        });
        let a502 = {
            let a499 = f185(match v0.clone() {
                Adt21::C0(_, _, h485, _, _, _, _, _) => {
                    h485
                }
            });
            let a500 = {
                let a497 = f185(match v0.clone() {
                    Adt21::C0(_, _, _, h486, _, _, _, _) => {
                        h486
                    }
                });
                let a498 = {
                    let a495 = f185(match v0.clone() {
                        Adt21::C0(_, _, _, _, h487, _, _, _) => {
                            h487
                        }
                    });
                    let a496 = {
                        let a493 = f185(match v0.clone() {
                            Adt21::C0(_, _, _, _, _, h488, _, _) => {
                                h488
                            }
                        });
                        let a494 = {
                            let a491 = f185(match v0.clone() {
                                Adt21::C0(_, _, _, _, _, _, h489, _) => {
                                    h489
                                }
                            });
                            let a492 = f185(match v0 {
                                Adt21::C0(_, _, _, _, _, _, _, h490) => {
                                    h490
                                }
                            });
                            append_bytes(a491, a492)
                        };
                        append_bytes(a493, a494)
                    };
                    append_bytes(a495, a496)
                };
                append_bytes(a497, a498)
            };
            append_bytes(a499, a500)
        };
        append_bytes(a501, a502)
    };
    append_bytes(a503, a504)
}

pub fn f137(v0: Bytes) -> R<Adt21> {
    let v1: u64 = {
        let a505 = v0.clone();
        length_bytes(a505)
    };
    let v2: u64 = {
        let a511 = {
            let a508 = {
                let a506 = v1;
                let a507 = 72u64;
                nat_add(a506, a507)?
            };
            let a509 = 64u64;
            let a510 = 0u64;
            nat_quot(a508, a509, a510)
        };
        let a512 = 64u64;
        nat_mul(a511, a512)?
    };
    let (_, h513) = f186(Fn3::F187(v0, v1, v2), {
        let a514 = v2;
        let a515 = 64u64;
        let a516 = 0u64;
        nat_quot(a514, a515, a516)
    }, (0u64, f188()))?;
    Ok(h513)
}

pub fn f138(p519: List<List<Bytes>>, v1: Bytes, p520: List<Bytes>) -> R<List<Bytes>> {
    let m518 = (p519, p520);
    {
        let mut m518 = m518;
        loop {
            let step = {
                let (v0, v2) = m518;
                let m517 = v0;
                match m517.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f189(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f141(f190(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(p523: List<Bytes>, p524: List<Bytes>, p525: List<List<Bytes>>) -> List<List<Bytes>> {
    let m522 = (p523, (p524, p525));
    {
        let mut m522 = m522;
        loop {
            let step = {
                let (v0, (v1, v2)) = m522;
                let m521 = v0;
                match m521.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f158(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f26(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f158(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
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

pub fn f140(v0: Bytes) -> R<Bytes> {
    Ok(f40(v0.clone(), 0u64, f191(v0)?))
}

pub fn f141(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m526 = v0;
    match m526.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m527 = v3.clone();
            match m527.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a530 = v2;
                    let a531 = {
                        let a528 = v1.clone();
                        let a529 = f141(v3, v1);
                        append_bytes(a528, a529)
                    };
                    append_bytes(a530, a531)
                }
            }
        }
    }
}

pub fn f142(v0: List<Bytes>) -> Adt14 {
    let m532 = v0;
    match m532.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f142(v2)))
        }
    }
}

pub fn f143(v0: Fn2, p536: Option<Bytes>, p537: List<Bytes>) -> Option<Bytes> {
    let m535 = (p536, p537);
    {
        let mut m535 = m535;
        loop {
            let step = {
                let (v1, v2) = m535;
                let m533 = v2;
                match m533.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c534: Fn2 = v0.clone();
                            c534.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m538 = v1;
    match m538 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a541 = {
                    let a539 = v3;
                    let a540 = v0;
                    append_bytes(a539, a540)
                };
                let a542 = v2;
                append_bytes(a541, a542)
            })
        }
    }
}

pub fn f145(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f191(v0.clone())?;
    let v2: u64 = f192(v0.clone(), v1)?;
    Ok(f40(v0, v2, {
        let a543 = v1;
        let a544 = v2;
        nat_sub(a543, a544)
    }))
}

pub fn f146(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f193(v0)?;
    let m548 = f194(v1.clone(), 0u64, {
        let a546 = {
            let a545 = v1.clone();
            length_bytes(a545)
        };
        let a547 = 2u64;
        nat_add(a546, a547)?
    }, 0u64)?;
    match m548 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a551 = {
                let (_, h549) = v2;
                h549
            };
            let a552 = {
                let a550 = v1;
                length_bytes(a550)
            };
            if nat_eq(a551, a552) {
                Ok(Some({
                    let (h553, _) = v2;
                    h553
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f147(v0: Bytes) -> Bytes {
    f7(List::cons(f57(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f148(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f195(v0, v1)
}

pub fn f149(v0: Adt14) -> Bytes {
    if f196(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f196(v0.clone(), f8(Str::lit("id"))) {
            if f196(v0.clone(), f8(Str::lit("statement"))) {
                if f196(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f197(v0, f8(Str::lit("evidence")))
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

pub fn f150(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m554 = f96(v0, v1);
    match m554 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f93(f96(v3, v2))
        }
    }
}

pub fn f151(v0: Adt14) -> List<Adt14> {
    let m555 = v0;
    match m555 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r556) => {
            let v3 = (*r556).clone();
            f151(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r557, r558) => {
            let v1 = (*r557).clone();
            let v2 = (*r558).clone();
            List::cons(v1, f151(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f152(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f96(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f79(Str::lit("id"), f94(v0.clone(), f8(Str::lit("id"))))?, List::cons(f79(Str::lit("name"), f94(v0.clone(), f8(Str::lit("name"))))?, List::cons(f79(Str::lit("issuer"), f94(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f79(Str::lit("edition"), f94(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f79(Str::lit("canonical_id"), f94(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f79(Str::lit("source_role"), f94(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f79(Str::lit("retrieval_date"), f94(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f79(Str::lit("url"), f198(v2.clone(), f8(Str::lit("url"))))?, List::cons(f79(Str::lit("media_type"), f198(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f79(Str::lit("license"), f198(v2.clone(), f8(Str::lit("license"))))?, List::cons(f79(Str::lit("redistribution"), f198(v2, f8(Str::lit("redistribution"))))?, List::cons(f79(Str::lit("statement"), f94(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m559 = v1;
        match m559 {
            None => {
                f7(List::cons(f79(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f79(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f79(Str::lit("artifact"), {
                    let (h560, _) = v3;
                    h560
                })?, List::cons(f79(Str::lit("checksum"), {
                    let (_, h561) = v3;
                    h561
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f199(f54(f93(f96(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f153(v0: Bytes, p628: u64, p629: u64, p630: bool, p631: Bytes) -> R<Bytes> {
    let m627 = (p628, (p629, (p630, p631)));
    {
        let mut m627 = m627;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m627;
                let m562 = v2;
                if m562 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m562 - 1;
                    let m565 = {
                        let a563 = v0.clone();
                        let a564 = v1;
                        index_bytes(a563, a564)
                    };
                    match m565 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a610 = {
                                    let a586 = {
                                        let a574 = {
                                            let a568 = {
                                                let a566 = v7;
                                                let a567 = 128u8;
                                                fixed_u8::bit_and(a566, a567)
                                            };
                                            let a569 = 128u8;
                                            if equal(a568, a569) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a575 = {
                                            let a572 = {
                                                let a570 = v7;
                                                let a571 = 64u8;
                                                fixed_u8::bit_and(a570, a571)
                                            };
                                            let a573 = 64u8;
                                            if equal(a572, a573) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a574, a575)?
                                    };
                                    let a587 = {
                                        let a584 = {
                                            let a578 = {
                                                let a576 = v7;
                                                let a577 = 32u8;
                                                fixed_u8::bit_and(a576, a577)
                                            };
                                            let a579 = 32u8;
                                            if equal(a578, a579) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a585 = {
                                            let a582 = {
                                                let a580 = v7;
                                                let a581 = 16u8;
                                                fixed_u8::bit_and(a580, a581)
                                            };
                                            let a583 = 16u8;
                                            if equal(a582, a583) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a584, a585)?
                                    };
                                    nat_add(a586, a587)?
                                };
                                let a611 = {
                                    let a608 = {
                                        let a596 = {
                                            let a590 = {
                                                let a588 = v7;
                                                let a589 = 8u8;
                                                fixed_u8::bit_and(a588, a589)
                                            };
                                            let a591 = 8u8;
                                            if equal(a590, a591) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a597 = {
                                            let a594 = {
                                                let a592 = v7;
                                                let a593 = 4u8;
                                                fixed_u8::bit_and(a592, a593)
                                            };
                                            let a595 = 4u8;
                                            if equal(a594, a595) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a596, a597)?
                                    };
                                    let a609 = {
                                        let a606 = {
                                            let a600 = {
                                                let a598 = v7;
                                                let a599 = 2u8;
                                                fixed_u8::bit_and(a598, a599)
                                            };
                                            let a601 = 2u8;
                                            if equal(a600, a601) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a607 = {
                                            let a604 = {
                                                let a602 = v7;
                                                let a603 = 1u8;
                                                fixed_u8::bit_and(a602, a603)
                                            };
                                            let a605 = 1u8;
                                            if equal(a604, a605) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a606, a607)?
                                    };
                                    nat_add(a608, a609)?
                                };
                                nat_add(a610, a611)?
                            };
                            if f200(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a612 = v1;
                                    let a613 = 1u64;
                                    nat_add(a612, a613)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f201(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a614 = v1;
                                        let a615 = 1u64;
                                        nat_add(a614, a615)?
                                    }, (v5, (false, {
                                        let a619 = v4;
                                        let a620 = f40({
                                            let a616 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a616)
                                        }, {
                                            let a617 = v8;
                                            let a618 = 97u64;
                                            nat_sub(a617, a618)
                                        }, 1u64);
                                        append_bytes(a619, a620)
                                    }))))
                                } else {
                                    if if f201(v8) {
                                        true
                                    } else {
                                        if f202(v8) {
                                            true
                                        } else {
                                            f203(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a621 = v1;
                                            let a622 = 1u64;
                                            nat_add(a621, a622)?
                                        }, (v5, (false, {
                                            let a623 = v4;
                                            let a624 = f40(v0.clone(), v1, 1u64);
                                            append_bytes(a623, a624)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a625 = v1;
                                            let a626 = 1u64;
                                            nat_add(a625, a626)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
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

pub fn f154(p639: u64, p640: List<Adt17>, p641: List<Bytes>, p642: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m638 = (p639, (p640, (p641, p642)));
    {
        let mut m638 = m638;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m638;
                let m632 = v0;
                if m632 == 0 {
                    Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m632 - 1;
                    let m633 = v1;
                    match m633.uncons() {
                        None => {
                            Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m634 = v5;
                            match m634 {
                                Adt17::C0(v8, v9) => {
                                    if f63(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m635 = f204(v8.clone(), v9)?;
                                        match m635 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f155({
                                                    let (_, h636) = v11;
                                                    h636
                                                }, List::cons(Adt17::C1({
                                                    let (h637, _) = v11;
                                                    h637
                                                }), v6)), (List::cons(v8, v2), v3))))
                                            }
                                            Err(v10) => {
                                                Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                            }
                                        }
                                    }
                                }
                                Adt17::C1(v7) => {
                                    Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                            }
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

pub fn f155(v0: List<Adt17>, v1: List<Adt17>) -> List<Adt17> {
    let m643 = v0;
    match m643.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f155(v3, v1))
        }
    }
}

pub fn f156(v0: List<(Bytes, Adt14)>) -> R<List<Adt17>> {
    let m644 = v0;
    match m644.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt17::C0(f102({
                let (h645, _) = v1;
                h645
            })?, {
                let (_, h646) = v1;
                h646
            }), f156(v2)?))
        }
    }
}

pub fn f157(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m647 = v0;
    match m647 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f205(v1)
        }
    }
}

pub fn f158(p650: List<Bytes>, p651: List<Bytes>) -> List<Bytes> {
    let m649 = (p650, p651);
    {
        let mut m649 = m649;
        loop {
            let step = {
                let (v0, v1) = m649;
                let m648 = v0;
                match m648.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: Adt14) -> R<Bytes> {
    let m652 = v0;
    match m652 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f206(v2, v3)
        }
        Adt14::C2(v4) => {
            f207(v4)
        }
        Adt14::C3(r653) => {
            let v5 = (*r653).clone();
            Ok(f208(f209(v5)?))
        }
        Adt14::C4(r654) => {
            let v6 = (*r654).clone();
            f210(f211(v6)?)
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

pub fn f160(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m655 = v0;
    match m655.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f212(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f141(f213(f58(List::cons(v1, f214(f38(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f160(v3, v1)?))
            }
        }
    }
}

pub fn f161(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h656, _) = v2.clone();
        h656
    };
    let v4: u64 = {
        let (h657, _) = {
            let (_, h658) = v2.clone();
            h658
        };
        h657
    };
    let v5: List<Bytes> = {
        let (_, h659) = {
            let (_, h660) = v2;
            h660
        };
        h659
    };
    let m661 = f215(v0.clone(), v3);
    match m661 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a662 = v6;
            let a663 = v1;
            if equal(a662, a663) {
                Ok(Some(({
                    let a664 = v3;
                    let a665 = 1u64;
                    nat_add(a664, a665)?
                }, ({
                    let a666 = v3;
                    let a667 = 1u64;
                    nat_add(a666, a667)?
                }, List::cons(f40(v0, v4, {
                    let a668 = v3;
                    let a669 = v4;
                    nat_sub(a668, a669)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a670 = v3;
                    let a671 = 1u64;
                    nat_add(a670, a671)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f162(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f163(v0: Bytes) -> bool {
    f113(v0, f8(Str::lit(".lex.tex")))
}

pub fn f164(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m672 = v0;
    match m672.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f164(v3, {
                let a673 = v1;
                let a674 = 1u64;
                nat_add(a673, a674)?
            })?))
        }
    }
}

pub fn f165(v0: Adt15, v1: List<u64>, v2: Adt19) -> bool {
    let m675 = v2;
    match m675 {
        Adt19::C0(v3, v4) => {
            if f171(v3, match v0.clone() {
                Adt15::C0(h676, _, _, _) => {
                    h676
                }
            }) {
                f216(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f171(v5, match v0 {
                Adt15::C0(_, _, h677, _) => {
                    h677
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f167(v7, v1) {
                f171(v8, match v0 {
                    Adt15::C0(_, _, _, h678) => {
                        h678
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f166(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m679 = f96(v1.clone(), f8(Str::lit("std")));
    match m679 {
        None => {
            let m684 = f97(f96(v1.clone(), f8(Str::lit("req"))));
            match m684 {
                None => {
                    let m686 = f96(v1, f8(Str::lit("der")));
                    match m686 {
                        None => {
                            Ok(f217(v0))
                        }
                        Some(v5) => {
                            let m687 = f218(f94(v5.clone(), f8(Str::lit("rule"))));
                            match m687 {
                                None => {
                                    Ok(f217(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f121(f54(f93(f96(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h688) => {
                                            h688
                                        }
                                    })?, f169(f94(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h689, _) => {
                                            h689
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f169(v4, match v0 {
                        Adt12::C0(_, _, h685, _, _) => {
                            h685
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m680 = f97(f96(v2.clone(), f8(Str::lit("row"))));
            match m680 {
                None => {
                    Ok(Adt19::C0(f169(f94(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h681, _, _, _, _) => {
                            h681
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f169(f94(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h682, _, _, _, _) => {
                            h682
                        }
                    })?, Some(f169(v3, match v0 {
                        Adt12::C0(_, h683, _, _, _) => {
                            h683
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f167(p692: List<u64>, v1: List<u64>) -> bool {
    let m691 = p692;
    {
        let mut m691 = m691;
        loop {
            let step = {
                let v0 = m691;
                let m690 = v0;
                match m690.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f171(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f168(p697: List<u64>) -> bool {
    let m696 = p697;
    {
        let mut m696 = m696;
        loop {
            let step = {
                let v0 = m696;
                let m693 = v0;
                match m693.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a694 = v1;
                        let a695 = 7u64;
                        if nat_le(a694, a695) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m696 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m698 = f219(v0, v1.clone(), 0u64)?;
    match m698 {
        None => {
            let a699 = v1;
            Ok(length_list(a699))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f170(v0: Adt14) -> u64 {
    let m700 = f220(f96(v0, f8(Str::lit("stratum"))));
    match m700 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f171(v0: u64, p705: List<u64>) -> bool {
    let m704 = p705;
    {
        let mut m704 = m704;
        loop {
            let step = {
                let v1 = m704;
                let m701 = v1;
                match m701.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a702 = v2;
                        let a703 = v0;
                        if nat_eq(a702, a703) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f212(v0.clone(), v1) {
        let m706 = f38(v0, 34u8)?;
        match m706.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m707 = v3;
                match m707.uncons() {
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

pub fn f173(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f174(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m708 = v0.clone();
    match m708.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a709 = v2;
            let a710 = v1.clone();
            if equal(a709, a710) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f175(v0: List<(Bytes, ())>) -> List<Bytes> {
    f125(f221(Fn4::F222, List::<Bytes>::nil(), v0))
}

pub fn f176(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f223(f224(Fn5::F225, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m715 = {
        let (h711, _) = f226(Fn6::F227, {
            let a713 = {
                let a712 = v0;
                length_list(a712)
            };
            let a714 = 1u64;
            nat_add(a713, a714)?
        }, v1)?;
        h711
    };
    match m715.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f177(v0: List<Bytes>) -> List<(Bytes, ())> {
    f228(f229(Fn7::F230, List::<(Bytes, ())>::nil(), v0))
}

pub fn f178(p718: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m717 = p718;
    {
        let mut m717 = m717;
        loop {
            let step = {
                let v0 = m717;
                let m716 = v0;
                match m716.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f189(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m717 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Adt14, v1: Bytes) -> Bytes {
    if f26(f198(f96(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f180(v0: Adt14) -> Bytes {
    if f196(v0.clone(), f8(Str::lit("req"))) {
        let a719 = f8(Str::lit("req "));
        let a720 = f94(v0, f8(Str::lit("req")));
        append_bytes(a719, a720)
    } else {
        let m721 = f96(v0.clone(), f8(Str::lit("std")));
        match m721 {
            None => {
                let m724 = f96(v0, f8(Str::lit("der")));
                match m724 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f94(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f83(f54(f93(f96(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f94(v1.clone(), f8(Str::lit("authority"))), List::cons(if f196(v1.clone(), f8(Str::lit("row"))) {
                    let a722 = f8(Str::lit(" row "));
                    let a723 = f94(v1, f8(Str::lit("row")));
                    append_bytes(a722, a723)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f181(v0: Bytes, v1: u64) -> R<u64> {
    let m727 = {
        let a725 = v0;
        let a726 = v1;
        index_bytes(a725, a726)
    };
    match m727 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a772 = {
                let a748 = {
                    let a736 = {
                        let a730 = {
                            let a728 = v3;
                            let a729 = 128u8;
                            fixed_u8::bit_and(a728, a729)
                        };
                        let a731 = 128u8;
                        if equal(a730, a731) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a737 = {
                        let a734 = {
                            let a732 = v3;
                            let a733 = 64u8;
                            fixed_u8::bit_and(a732, a733)
                        };
                        let a735 = 64u8;
                        if equal(a734, a735) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a736, a737)?
                };
                let a749 = {
                    let a746 = {
                        let a740 = {
                            let a738 = v3;
                            let a739 = 32u8;
                            fixed_u8::bit_and(a738, a739)
                        };
                        let a741 = 32u8;
                        if equal(a740, a741) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a747 = {
                        let a744 = {
                            let a742 = v3;
                            let a743 = 16u8;
                            fixed_u8::bit_and(a742, a743)
                        };
                        let a745 = 16u8;
                        if equal(a744, a745) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a746, a747)?
                };
                nat_add(a748, a749)?
            };
            let a773 = {
                let a770 = {
                    let a758 = {
                        let a752 = {
                            let a750 = v3;
                            let a751 = 8u8;
                            fixed_u8::bit_and(a750, a751)
                        };
                        let a753 = 8u8;
                        if equal(a752, a753) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a759 = {
                        let a756 = {
                            let a754 = v3;
                            let a755 = 4u8;
                            fixed_u8::bit_and(a754, a755)
                        };
                        let a757 = 4u8;
                        if equal(a756, a757) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a758, a759)?
                };
                let a771 = {
                    let a768 = {
                        let a762 = {
                            let a760 = v3;
                            let a761 = 2u8;
                            fixed_u8::bit_and(a760, a761)
                        };
                        let a763 = 2u8;
                        if equal(a762, a763) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a769 = {
                        let a766 = {
                            let a764 = v3;
                            let a765 = 1u8;
                            fixed_u8::bit_and(a764, a765)
                        };
                        let a767 = 1u8;
                        if equal(a766, a767) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a768, a769)?
                };
                nat_add(a770, a771)?
            };
            nat_add(a772, a773)
        }
    }
}

pub fn f182(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m777 = {
        let a774 = v1;
        let a775 = v2;
        let a776 = 1u64;
        slice_bytes(a774, a775, a776)
    };
    match m777 {
        None => {
            v0
        }
        Some(v3) => {
            let a778 = v0;
            let a779 = v3;
            append_bytes(a778, a779)
        }
    }
}

pub fn f183(p782: List<Bytes>, p783: List<Bytes>) -> List<Bytes> {
    let m781 = (p782, p783);
    {
        let mut m781 = m781;
        loop {
            let step = {
                let (v0, v1) = m781;
                let m780 = v0;
                match m780.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f184(v0: Bytes, p855: u64, p856: u64, p857: Bytes) -> R<Bytes> {
    let m854 = (p855, (p856, p857));
    {
        let mut m854 = m854;
        loop {
            let step = {
                let (v1, (v2, v3)) = m854;
                let m784 = v2;
                if m784 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m784 - 1;
                    let m787 = {
                        let a785 = v0.clone();
                        let a786 = v1;
                        index_bytes(a785, a786)
                    };
                    match m787 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a832 = {
                                    let a808 = {
                                        let a796 = {
                                            let a790 = {
                                                let a788 = v6;
                                                let a789 = 128u8;
                                                fixed_u8::bit_and(a788, a789)
                                            };
                                            let a791 = 128u8;
                                            if equal(a790, a791) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a797 = {
                                            let a794 = {
                                                let a792 = v6;
                                                let a793 = 64u8;
                                                fixed_u8::bit_and(a792, a793)
                                            };
                                            let a795 = 64u8;
                                            if equal(a794, a795) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a796, a797)?
                                    };
                                    let a809 = {
                                        let a806 = {
                                            let a800 = {
                                                let a798 = v6;
                                                let a799 = 32u8;
                                                fixed_u8::bit_and(a798, a799)
                                            };
                                            let a801 = 32u8;
                                            if equal(a800, a801) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a807 = {
                                            let a804 = {
                                                let a802 = v6;
                                                let a803 = 16u8;
                                                fixed_u8::bit_and(a802, a803)
                                            };
                                            let a805 = 16u8;
                                            if equal(a804, a805) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a806, a807)?
                                    };
                                    nat_add(a808, a809)?
                                };
                                let a833 = {
                                    let a830 = {
                                        let a818 = {
                                            let a812 = {
                                                let a810 = v6;
                                                let a811 = 8u8;
                                                fixed_u8::bit_and(a810, a811)
                                            };
                                            let a813 = 8u8;
                                            if equal(a812, a813) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a819 = {
                                            let a816 = {
                                                let a814 = v6;
                                                let a815 = 4u8;
                                                fixed_u8::bit_and(a814, a815)
                                            };
                                            let a817 = 4u8;
                                            if equal(a816, a817) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a818, a819)?
                                    };
                                    let a831 = {
                                        let a828 = {
                                            let a822 = {
                                                let a820 = v6;
                                                let a821 = 2u8;
                                                fixed_u8::bit_and(a820, a821)
                                            };
                                            let a823 = 2u8;
                                            if equal(a822, a823) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a829 = {
                                            let a826 = {
                                                let a824 = v6;
                                                let a825 = 1u8;
                                                fixed_u8::bit_and(a824, a825)
                                            };
                                            let a827 = 1u8;
                                            if equal(a826, a827) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a828, a829)?
                                    };
                                    nat_add(a830, a831)?
                                };
                                nat_add(a832, a833)?
                            };
                            let a834 = v7;
                            let a835 = 34u64;
                            if if nat_eq(a834, a835) {
                                true
                            } else {
                                let a836 = v7;
                                let a837 = 92u64;
                                nat_eq(a836, a837)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a838 = v1;
                                    let a839 = 1u64;
                                    nat_add(a838, a839)?
                                }, (v4, {
                                    let a842 = v3;
                                    let a843 = {
                                        let a840 = Bytes::lit(&[0x5c]);
                                        let a841 = f40(v0.clone(), v1, 1u64);
                                        append_bytes(a840, a841)
                                    };
                                    append_bytes(a842, a843)
                                })))
                            } else {
                                let a844 = v7;
                                let a845 = 10u64;
                                if nat_eq(a844, a845) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a846 = v1;
                                        let a847 = 1u64;
                                        nat_add(a846, a847)?
                                    }, (v4, {
                                        let a848 = v3;
                                        let a849 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a848, a849)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a850 = v1;
                                        let a851 = 1u64;
                                        nat_add(a850, a851)?
                                    }, (v4, {
                                        let a852 = v3;
                                        let a853 = f40(v0.clone(), v1, 1u64);
                                        append_bytes(a852, a853)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m854 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: u32) -> Bytes {
    let a886 = f231({
        let a858 = f232(v0, 28u32);
        let a859 = 15u32;
        fixed_u32::bit_and(a858, a859)
    });
    let a887 = {
        let a884 = f231({
            let a860 = f232(v0, 24u32);
            let a861 = 15u32;
            fixed_u32::bit_and(a860, a861)
        });
        let a885 = {
            let a882 = f231({
                let a862 = f232(v0, 20u32);
                let a863 = 15u32;
                fixed_u32::bit_and(a862, a863)
            });
            let a883 = {
                let a880 = f231({
                    let a864 = f232(v0, 16u32);
                    let a865 = 15u32;
                    fixed_u32::bit_and(a864, a865)
                });
                let a881 = {
                    let a878 = f231({
                        let a866 = f232(v0, 12u32);
                        let a867 = 15u32;
                        fixed_u32::bit_and(a866, a867)
                    });
                    let a879 = {
                        let a876 = f231({
                            let a868 = f232(v0, 8u32);
                            let a869 = 15u32;
                            fixed_u32::bit_and(a868, a869)
                        });
                        let a877 = {
                            let a874 = f231({
                                let a870 = f232(v0, 4u32);
                                let a871 = 15u32;
                                fixed_u32::bit_and(a870, a871)
                            });
                            let a875 = f231({
                                let a872 = v0;
                                let a873 = 15u32;
                                fixed_u32::bit_and(a872, a873)
                            });
                            append_bytes(a874, a875)
                        };
                        append_bytes(a876, a877)
                    };
                    append_bytes(a878, a879)
                };
                append_bytes(a880, a881)
            };
            append_bytes(a882, a883)
        };
        append_bytes(a884, a885)
    };
    append_bytes(a886, a887)
}

pub fn f186(v0: Fn3, p891: u64, p892: (u64, Adt21)) -> R<(u64, Adt21)> {
    let m890 = (p891, p892);
    {
        let mut m890 = m890;
        loop {
            let step = {
                let (v1, v2) = m890;
                let m888 = v1;
                if m888 == 0 {
                    Err::<(u64, (u64, Adt21)), R<(u64, Adt21)>>(Ok(v2))
                } else {
                    let v3 = m888 - 1;
                    Ok::<(u64, (u64, Adt21)), R<(u64, Adt21)>>((v3, {
                        let c889: Fn3 = v0.clone();
                        c889.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m890 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt21)) -> R<(u64, Adt21)> {
    Ok(({
        let a894 = {
            let (h893, _) = v3.clone();
            h893
        };
        let a895 = 64u64;
        nat_add(a894, a895)?
    }, f233({
        let (_, h896) = v3;
        h896
    }, f234(v0, v1, v2, {
        let (h897, _) = v3;
        h897
    })?)?))
}

pub fn f188() -> Adt21 {
    Adt21::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f189(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f63(v1, f49(f141(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f190(v0: List<Bytes>) -> List<Bytes> {
    let m898 = v0;
    match m898.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f26(v1.clone(), f8(Str::lit("authority = []"))) {
                f190(v2)
            } else {
                List::cons(v1, f190(v2))
            }
        }
    }
}

pub fn f191(v0: Bytes) -> R<u64> {
    let (h899, _) = f235(Fn8::F236(v0.clone()), {
        let a900 = v0.clone();
        length_bytes(a900)
    }, {
        let a901 = v0;
        length_bytes(a901)
    })?;
    Ok(h899)
}

pub fn f192(v0: Bytes, v1: u64) -> R<u64> {
    let (h902, _) = f235(Fn8::F237(v1, v0), v1, 0u64)?;
    Ok(h902)
}

pub fn f193(v0: Bytes) -> R<Bytes> {
    f238(v0.clone(), 0u64, {
        let a903 = v0;
        length_bytes(a903)
    }, false, false, Bytes::lit(&[]))
}

pub fn f194(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m904 = v2;
    if m904 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m904 - 1;
        let v5: u64 = f239(v0.clone(), v1)?;
        let a905 = v3;
        let a906 = 1u64;
        if nat_eq(a905, a906) {
            let m907 = f194(v0.clone(), v1, v4, 0u64)?;
            match m907 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f239(v0.clone(), {
                        let (_, h908) = v6.clone();
                        h908
                    })?;
                    let a909 = v7;
                    let a910 = 44u64;
                    if nat_eq(a909, a910) {
                        let m914 = f194(v0, {
                            let a912 = {
                                let (_, h911) = v6;
                                h911
                            };
                            let a913 = 1u64;
                            nat_add(a912, a913)?
                        }, v4, 1u64)?;
                        match m914 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h915, _) = v6;
                                    h915
                                }), std::rc::Rc::new({
                                    let (h916, _) = v8;
                                    h916
                                })), {
                                    let (_, h917) = v8;
                                    h917
                                })))
                            }
                        }
                    } else {
                        let a918 = v7;
                        let a919 = 93u64;
                        if nat_eq(a918, a919) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h920, _) = v6;
                                h920
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a922 = {
                                    let (_, h921) = v6;
                                    h921
                                };
                                let a923 = 1u64;
                                nat_add(a922, a923)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a924 = v3;
            let a925 = 2u64;
            if nat_eq(a924, a925) {
                let m928 = f240(v0.clone(), {
                    let a926 = v1;
                    let a927 = 1u64;
                    nat_add(a926, a927)?
                })?;
                match m928 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a929 = v5;
                        let a930 = 34u64;
                        if if nat_eq(a929, a930) {
                            let a932 = f239(v0.clone(), {
                                let (h931, _) = v9.clone();
                                h931
                            })?;
                            let a933 = 58u64;
                            nat_eq(a932, a933)
                        } else {
                            false
                        } {
                            let m937 = f194(v0.clone(), {
                                let a935 = {
                                    let (h934, _) = v9;
                                    h934
                                };
                                let a936 = 1u64;
                                nat_add(a935, a936)?
                            }, v4, 0u64)?;
                            match m937 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f239(v0.clone(), {
                                        let (_, h938) = v10.clone();
                                        h938
                                    })?;
                                    let a939 = v11;
                                    let a940 = 44u64;
                                    if nat_eq(a939, a940) {
                                        let m944 = f194(v0, {
                                            let a942 = {
                                                let (_, h941) = v10;
                                                h941
                                            };
                                            let a943 = 1u64;
                                            nat_add(a942, a943)?
                                        }, v4, 2u64)?;
                                        match m944 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h945) = v9;
                                                    h945
                                                }, std::rc::Rc::new({
                                                    let (h946, _) = v10;
                                                    h946
                                                }), std::rc::Rc::new({
                                                    let (h947, _) = v12;
                                                    h947
                                                })), {
                                                    let (_, h948) = v12;
                                                    h948
                                                })))
                                            }
                                        }
                                    } else {
                                        let a949 = v11;
                                        let a950 = 125u64;
                                        if nat_eq(a949, a950) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h951) = v9;
                                                h951
                                            }, std::rc::Rc::new({
                                                let (h952, _) = v10;
                                                h952
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a954 = {
                                                    let (_, h953) = v10;
                                                    h953
                                                };
                                                let a955 = 1u64;
                                                nat_add(a954, a955)?
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
                let a956 = v5;
                let a957 = 34u64;
                if nat_eq(a956, a957) {
                    let m960 = f240(v0, {
                        let a958 = v1;
                        let a959 = 1u64;
                        nat_add(a958, a959)?
                    })?;
                    match m960 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h961) = v13;
                                h961
                            }), {
                                let (h962, _) = v13;
                                h962
                            })))
                        }
                    }
                } else {
                    if f26(f40(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a963 = v1;
                            let a964 = 4u64;
                            nat_add(a963, a964)?
                        })))
                    } else {
                        if f26(f40(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a965 = v1;
                                let a966 = 5u64;
                                nat_add(a965, a966)?
                            })))
                        } else {
                            let a967 = v5;
                            let a968 = 91u64;
                            if nat_eq(a967, a968) {
                                let a971 = f239(v0.clone(), {
                                    let a969 = v1;
                                    let a970 = 1u64;
                                    nat_add(a969, a970)?
                                })?;
                                let a972 = 93u64;
                                if nat_eq(a971, a972) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a973 = v1;
                                        let a974 = 2u64;
                                        nat_add(a973, a974)?
                                    })))
                                } else {
                                    let m977 = f194(v0, {
                                        let a975 = v1;
                                        let a976 = 1u64;
                                        nat_add(a975, a976)?
                                    }, v4, 1u64)?;
                                    match m977 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h978, _) = v14;
                                                h978
                                            })), {
                                                let (_, h979) = v14;
                                                h979
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a980 = v5;
                                let a981 = 123u64;
                                if nat_eq(a980, a981) {
                                    let a984 = f239(v0.clone(), {
                                        let a982 = v1;
                                        let a983 = 1u64;
                                        nat_add(a982, a983)?
                                    })?;
                                    let a985 = 125u64;
                                    if nat_eq(a984, a985) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a986 = v1;
                                            let a987 = 2u64;
                                            nat_add(a986, a987)?
                                        })))
                                    } else {
                                        let m990 = f194(v0, {
                                            let a988 = v1;
                                            let a989 = 1u64;
                                            nat_add(a988, a989)?
                                        }, v4, 2u64)?;
                                        match m990 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h991, _) = v15;
                                                    h991
                                                })), {
                                                    let (_, h992) = v15;
                                                    h992
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m993 = f241(v0, v1)?;
                                    match m993 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h994) = v16;
                                                h994
                                            }, {
                                                let (h995, _) = v16;
                                                h995
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

pub fn f195(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h996, _) = v1.clone();
        h996
    };
    let v3: u64 = {
        let (h997, _) = {
            let (_, h998) = v1.clone();
            h998
        };
        h997
    };
    let v4: List<Bytes> = {
        let (_, h999) = {
            let (_, h1000) = v1;
            h1000
        };
        h999
    };
    let m1001 = f215(v0.clone(), v2);
    match m1001 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1002 = v5;
            let a1003 = 10u8;
            if equal(a1002, a1003) {
                Ok(Some(({
                    let a1004 = v2;
                    let a1005 = 1u64;
                    nat_add(a1004, a1005)?
                }, ({
                    let a1006 = v2;
                    let a1007 = 1u64;
                    nat_add(a1006, a1007)?
                }, List::cons(f242(f40(v0, v3, {
                    let a1008 = v2;
                    let a1009 = v3;
                    nat_sub(a1008, a1009)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1010 = v2;
                    let a1011 = 1u64;
                    nat_add(a1010, a1011)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f196(v0: Adt14, v1: Bytes) -> bool {
    f55({
        let a1013 = {
            let a1012 = f94(v0, v1);
            length_bytes(a1012)
        };
        let a1014 = 0u64;
        nat_eq(a1013, a1014)
    })
}

pub fn f197(v0: Adt14, v1: Bytes) -> bool {
    let m1015 = f96(v0, v1);
    match m1015 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f198(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1016 = v0;
    match m1016 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f94(v2, v1)
        }
    }
}

pub fn f199(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f141(f213(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f200(v0: u64) -> bool {
    let a1017 = v0;
    let a1018 = 95u64;
    if nat_eq(a1017, a1018) {
        true
    } else {
        let a1019 = v0;
        let a1020 = 45u64;
        if nat_eq(a1019, a1020) {
            true
        } else {
            let a1021 = v0;
            let a1022 = 32u64;
            if nat_eq(a1021, a1022) {
                true
            } else {
                let a1023 = v0;
                let a1024 = 46u64;
                if nat_eq(a1023, a1024) {
                    true
                } else {
                    let a1025 = v0;
                    let a1026 = 47u64;
                    nat_eq(a1025, a1026)
                }
            }
        }
    }
}

pub fn f201(v0: u64) -> bool {
    let a1027 = 97u64;
    let a1028 = v0;
    if nat_le(a1027, a1028) {
        let a1029 = v0;
        let a1030 = 122u64;
        nat_le(a1029, a1030)
    } else {
        false
    }
}

pub fn f202(v0: u64) -> bool {
    let a1031 = 65u64;
    let a1032 = v0;
    if nat_le(a1031, a1032) {
        let a1033 = v0;
        let a1034 = 90u64;
        nat_le(a1033, a1034)
    } else {
        false
    }
}

pub fn f203(v0: u64) -> bool {
    let a1035 = 48u64;
    let a1036 = v0;
    if nat_le(a1035, a1036) {
        let a1037 = v0;
        let a1038 = 57u64;
        nat_le(a1037, a1038)
    } else {
        false
    }
}

pub fn f204(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1039 = f96(v1.clone(), f8(Str::lit("enum")));
    match m1039 {
        None => {
            let m1041 = f246(v0.clone(), f157(f96(v1.clone(), f8(Str::lit("properties")))), f54(f93(f96(v1, f8(Str::lit("required"))))))?;
            match m1041 {
                Ok(v6) => {
                    Ok(f245(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m1043 = {
                            let (h1042, _) = v6.clone();
                            h1042
                        };
                        match m1043.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f141({
                                    let (h1044, _) = v6;
                                    h1044
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1045) = v6;
                        h1045
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt17>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m1040 = f243(f54(f151(v2)))?;
            match m1040 {
                Ok(v4) => {
                    Ok(f245(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f141(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt17>::nil()))
                }
                Err(v3) => {
                    Ok(f244(v3, v0))
                }
            }
        }
    }
}

pub fn f205(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1046 = v0;
    match m1046 {
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
        Adt14::C4(r1047) => {
            let v4 = (*r1047).clone();
            f205(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1048, r1049) => {
            let v2 = (*r1048).clone();
            let v3 = (*r1049).clone();
            List::cons((v1, v2), f205(v3))
        }
    }
}

pub fn f206(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1050 = 0u64;
        let a1051 = v1;
        nat_lt(a1050, a1051)
    } else {
        false
    } {
        let a1052 = Bytes::lit(&[0x2d]);
        let a1053 = f247(v1)?;
        Ok(append_bytes(a1052, a1053))
    } else {
        f247(v1)
    }
}

pub fn f207(v0: Bytes) -> R<Bytes> {
    let a1059 = {
        let a1057 = Bytes::lit(&[0x22]);
        let a1058 = {
            let (_, h1054) = {
                let (h1055, _) = f248(Fn9::F249(v0.clone()), {
                    let a1056 = v0;
                    length_bytes(a1056)
                }, (0u64, Bytes::lit(&[])))?;
                h1055
            };
            h1054
        };
        append_bytes(a1057, a1058)
    };
    let a1060 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1059, a1060))
}

pub fn f208(v0: List<Bytes>) -> Bytes {
    let a1063 = {
        let a1061 = Bytes::lit(&[0x5b]);
        let a1062 = f83(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1061, a1062)
    };
    let a1064 = Bytes::lit(&[0x5d]);
    append_bytes(a1063, a1064)
}

pub fn f209(v0: Adt14) -> R<List<Bytes>> {
    let m1065 = v0;
    match m1065 {
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
        Adt14::C6(r1066, r1067) => {
            let v7 = (*r1066).clone();
            let v8 = (*r1067).clone();
            Ok(List::cons(f159(v7)?, f209(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f210(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f250(Fn10::F251, List::<(Bytes, Bytes)>::nil(), f252(v0)?);
    let v2: List<Bytes> = f253(Fn11::F254, List::<Bytes>::nil(), v1)?;
    let a1070 = {
        let a1068 = Bytes::lit(&[0x7b]);
        let a1069 = f83(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1068, a1069)
    };
    let a1071 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1070, a1071))
}

pub fn f211(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1072 = v0;
    match m1072 {
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
        Adt14::C7(v9, r1073, r1074) => {
            let v10 = (*r1073).clone();
            let v11 = (*r1074).clone();
            Ok(List::cons((v9, f159(v10)?), f211(v11)?))
        }
    }
}

pub fn f212(v0: Bytes, v1: Bytes) -> bool {
    let m1079 = {
        let a1076 = v0;
        let a1077 = 0u64;
        let a1078 = {
            let a1075 = v1.clone();
            length_bytes(a1075)
        };
        slice_bytes(a1076, a1077, a1078)
    };
    match m1079 {
        None => {
            false
        }
        Some(v2) => {
            let a1080 = v2;
            let a1081 = v1;
            equal(a1080, a1081)
        }
    }
}

pub fn f213(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1082 = v0;
    match m1082.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f135(v1)?, f213(v2)?))
        }
    }
}

pub fn f214(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1083 = v0;
    match m1083.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f214(v3, false))
            } else {
                f214(v3, true)
            }
        }
    }
}

pub fn f215(v0: Bytes, v1: u64) -> Option<u8> {
    let a1084 = v0;
    let a1085 = v1;
    index_bytes(a1084, a1085)
}

pub fn f216(v0: Adt15, v1: Option<u64>) -> bool {
    let m1086 = v1;
    match m1086 {
        None => {
            true
        }
        Some(v2) => {
            f171(v2, match v0 {
                Adt15::C0(_, h1087, _, _) => {
                    h1087
                }
            })
        }
    }
}

pub fn f217(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a1089 = match v0 {
            Adt12::C0(_, _, h1088, _, _) => {
                h1088
            }
        };
        length_list(a1089)
    })
}

pub fn f218(v0: Bytes) -> Option<Adt20> {
    if f26(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt20::C0)
    } else {
        if f26(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt20::C1)
        } else {
            if f26(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt20::C2)
            } else {
                None::<Adt20>
            }
        }
    }
}

pub fn f219(v0: Bytes, p1094: List<Bytes>, p1095: u64) -> R<Option<u64>> {
    let m1093 = (p1094, p1095);
    {
        let mut m1093 = m1093;
        loop {
            let step = {
                let (v1, v2) = m1093;
                let m1090 = v1;
                match m1090.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f26(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1091 = v2;
                                let a1092 = 1u64;
                                nat_add(a1091, a1092)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1093 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: Option<Adt14>) -> Option<u64> {
    let m1096 = v0;
    match m1096 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1097 = v1;
            match m1097 {
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

pub fn f221(v0: Fn4, p1101: List<Bytes>, p1102: List<(Bytes, ())>) -> List<Bytes> {
    let m1100 = (p1101, p1102);
    {
        let mut m1100 = m1100;
        loop {
            let step = {
                let (v1, v2) = m1100;
                let m1098 = v2;
                match m1098.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1099: Fn4 = v0.clone();
                            c1099.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1103, _) = v1;
        h1103
    }, v0)
}

pub fn f223(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f255(Fn12::F256, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f224(v0: Fn5, p1107: List<List<(Bytes, ())>>, p1108: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1106 = (p1107, p1108);
    {
        let mut m1106 = m1106;
        loop {
            let step = {
                let (v1, v2) = m1106;
                let m1104 = v2;
                match m1104.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1105: Fn5 = v0.clone();
                            c1105.apply(v1, v3)
                        }, v4))
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

pub fn f225(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f226(v0: Fn6, p1113: u64, p1114: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1112 = (p1113, p1114);
    {
        let mut m1112 = m1112;
        loop {
            let step = {
                let (v1, v2) = m1112;
                let m1109 = v1;
                if m1109 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1109 - 1;
                    let m1111 = {
                        let c1110: Fn6 = v0.clone();
                        c1110.apply(v2.clone())?
                    };
                    match m1111 {
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
                Ok(next) => { m1112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f257(v0)
}

pub fn f228(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f258(Fn13::F259, List::<(Bytes, ())>::nil(), v0)
}

pub fn f229(v0: Fn7, p1118: List<(Bytes, ())>, p1119: List<Bytes>) -> List<(Bytes, ())> {
    let m1117 = (p1118, p1119);
    {
        let mut m1117 = m1117;
        loop {
            let step = {
                let (v1, v2) = m1117;
                let m1115 = v2;
                match m1115.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1116: Fn7 = v0.clone();
                            c1116.apply(v1, v3)
                        }, v4))
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

pub fn f230(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f231(v0: u32) -> Bytes {
    let a1122 = {
        let a1120 = v0;
        let a1121 = 8u32;
        fixed_u32::bit_and(a1120, a1121)
    };
    let a1123 = 0u32;
    if equal(a1122, a1123) {
        let a1126 = {
            let a1124 = v0;
            let a1125 = 4u32;
            fixed_u32::bit_and(a1124, a1125)
        };
        let a1127 = 0u32;
        if equal(a1126, a1127) {
            let a1130 = {
                let a1128 = v0;
                let a1129 = 2u32;
                fixed_u32::bit_and(a1128, a1129)
            };
            let a1131 = 0u32;
            if equal(a1130, a1131) {
                let a1134 = {
                    let a1132 = v0;
                    let a1133 = 1u32;
                    fixed_u32::bit_and(a1132, a1133)
                };
                let a1135 = 0u32;
                if equal(a1134, a1135) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1138 = {
                    let a1136 = v0;
                    let a1137 = 1u32;
                    fixed_u32::bit_and(a1136, a1137)
                };
                let a1139 = 0u32;
                if equal(a1138, a1139) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1142 = {
                let a1140 = v0;
                let a1141 = 2u32;
                fixed_u32::bit_and(a1140, a1141)
            };
            let a1143 = 0u32;
            if equal(a1142, a1143) {
                let a1146 = {
                    let a1144 = v0;
                    let a1145 = 1u32;
                    fixed_u32::bit_and(a1144, a1145)
                };
                let a1147 = 0u32;
                if equal(a1146, a1147) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1150 = {
                    let a1148 = v0;
                    let a1149 = 1u32;
                    fixed_u32::bit_and(a1148, a1149)
                };
                let a1151 = 0u32;
                if equal(a1150, a1151) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1154 = {
            let a1152 = v0;
            let a1153 = 4u32;
            fixed_u32::bit_and(a1152, a1153)
        };
        let a1155 = 0u32;
        if equal(a1154, a1155) {
            let a1158 = {
                let a1156 = v0;
                let a1157 = 2u32;
                fixed_u32::bit_and(a1156, a1157)
            };
            let a1159 = 0u32;
            if equal(a1158, a1159) {
                let a1162 = {
                    let a1160 = v0;
                    let a1161 = 1u32;
                    fixed_u32::bit_and(a1160, a1161)
                };
                let a1163 = 0u32;
                if equal(a1162, a1163) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1166 = {
                    let a1164 = v0;
                    let a1165 = 1u32;
                    fixed_u32::bit_and(a1164, a1165)
                };
                let a1167 = 0u32;
                if equal(a1166, a1167) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1170 = {
                let a1168 = v0;
                let a1169 = 2u32;
                fixed_u32::bit_and(a1168, a1169)
            };
            let a1171 = 0u32;
            if equal(a1170, a1171) {
                let a1174 = {
                    let a1172 = v0;
                    let a1173 = 1u32;
                    fixed_u32::bit_and(a1172, a1173)
                };
                let a1175 = 0u32;
                if equal(a1174, a1175) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1178 = {
                    let a1176 = v0;
                    let a1177 = 1u32;
                    fixed_u32::bit_and(a1176, a1177)
                };
                let a1179 = 0u32;
                if equal(a1178, a1179) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f232(v0: u32, v1: u32) -> u32 {
    let m1182 = {
        let a1180 = v0;
        let a1181 = v1;
        fixed_u32::shift_right(a1180, a1181)
    };
    match m1182 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f233(v0: Adt21, v1: Adt22) -> R<Adt21> {
    let v2: Adt23 = f260(Fn14::F261, 64u64, Adt23::C0(match v0.clone() {
        Adt21::C0(h1183, _, _, _, _, _, _, _) => {
            h1183
        }
    }, match v0.clone() {
        Adt21::C0(_, h1184, _, _, _, _, _, _) => {
            h1184
        }
    }, match v0.clone() {
        Adt21::C0(_, _, h1185, _, _, _, _, _) => {
            h1185
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, h1186, _, _, _, _) => {
            h1186
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, h1187, _, _, _) => {
            h1187
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1188, _, _) => {
            h1188
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1189, _) => {
            h1189
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, h1190) => {
            h1190
        }
    }, v1, 0u64))?;
    Ok(Adt21::C0(f262(match v0.clone() {
        Adt21::C0(h1191, _, _, _, _, _, _, _) => {
            h1191
        }
    }, match v2.clone() {
        Adt23::C0(h1192, _, _, _, _, _, _, _, _, _) => {
            h1192
        }
    }), f262(match v0.clone() {
        Adt21::C0(_, h1193, _, _, _, _, _, _) => {
            h1193
        }
    }, match v2.clone() {
        Adt23::C0(_, h1194, _, _, _, _, _, _, _, _) => {
            h1194
        }
    }), f262(match v0.clone() {
        Adt21::C0(_, _, h1195, _, _, _, _, _) => {
            h1195
        }
    }, match v2.clone() {
        Adt23::C0(_, _, h1196, _, _, _, _, _, _, _) => {
            h1196
        }
    }), f262(match v0.clone() {
        Adt21::C0(_, _, _, h1197, _, _, _, _) => {
            h1197
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, h1198, _, _, _, _, _, _) => {
            h1198
        }
    }), f262(match v0.clone() {
        Adt21::C0(_, _, _, _, h1199, _, _, _) => {
            h1199
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, h1200, _, _, _, _, _) => {
            h1200
        }
    }), f262(match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1201, _, _) => {
            h1201
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, h1202, _, _, _, _) => {
            h1202
        }
    }), f262(match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1203, _) => {
            h1203
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, _, h1204, _, _, _) => {
            h1204
        }
    }), f262(match v0 {
        Adt21::C0(_, _, _, _, _, _, _, h1205) => {
            h1205
        }
    }, match v2 {
        Adt23::C0(_, _, _, _, _, _, _, h1206, _, _) => {
            h1206
        }
    })))
}

pub fn f234(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    Ok(Adt22::C0(f263(v0.clone(), v1, v2, v3)?, f263(v0.clone(), v1, v2, {
        let a1207 = v3;
        let a1208 = 4u64;
        nat_add(a1207, a1208)?
    })?, f263(v0.clone(), v1, v2, {
        let a1209 = v3;
        let a1210 = 8u64;
        nat_add(a1209, a1210)?
    })?, f263(v0.clone(), v1, v2, {
        let a1211 = v3;
        let a1212 = 12u64;
        nat_add(a1211, a1212)?
    })?, f263(v0.clone(), v1, v2, {
        let a1213 = v3;
        let a1214 = 16u64;
        nat_add(a1213, a1214)?
    })?, f263(v0.clone(), v1, v2, {
        let a1215 = v3;
        let a1216 = 20u64;
        nat_add(a1215, a1216)?
    })?, f263(v0.clone(), v1, v2, {
        let a1217 = v3;
        let a1218 = 24u64;
        nat_add(a1217, a1218)?
    })?, f263(v0.clone(), v1, v2, {
        let a1219 = v3;
        let a1220 = 28u64;
        nat_add(a1219, a1220)?
    })?, f263(v0.clone(), v1, v2, {
        let a1221 = v3;
        let a1222 = 32u64;
        nat_add(a1221, a1222)?
    })?, f263(v0.clone(), v1, v2, {
        let a1223 = v3;
        let a1224 = 36u64;
        nat_add(a1223, a1224)?
    })?, f263(v0.clone(), v1, v2, {
        let a1225 = v3;
        let a1226 = 40u64;
        nat_add(a1225, a1226)?
    })?, f263(v0.clone(), v1, v2, {
        let a1227 = v3;
        let a1228 = 44u64;
        nat_add(a1227, a1228)?
    })?, f263(v0.clone(), v1, v2, {
        let a1229 = v3;
        let a1230 = 48u64;
        nat_add(a1229, a1230)?
    })?, f263(v0.clone(), v1, v2, {
        let a1231 = v3;
        let a1232 = 52u64;
        nat_add(a1231, a1232)?
    })?, f263(v0.clone(), v1, v2, {
        let a1233 = v3;
        let a1234 = 56u64;
        nat_add(a1233, a1234)?
    })?, f263(v0, v1, v2, {
        let a1235 = v3;
        let a1236 = 60u64;
        nat_add(a1235, a1236)?
    })?))
}

pub fn f235(v0: Fn8, p1241: u64, p1242: u64) -> R<(u64, bool)> {
    let m1240 = (p1241, p1242);
    {
        let mut m1240 = m1240;
        loop {
            let step = {
                let (v1, v2) = m1240;
                let m1237 = v1;
                if m1237 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1237 - 1;
                    let m1239 = {
                        let c1238: Fn8 = v0.clone();
                        c1238.apply(v2)?
                    };
                    match m1239 {
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
                Ok(next) => { m1240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: Bytes, v1: u64) -> Option<u64> {
    let a1243 = 0u64;
    let a1244 = v1;
    if nat_lt(a1243, a1244) {
        if f264(v0, {
            let a1245 = v1;
            let a1246 = 1u64;
            nat_sub(a1245, a1246)
        }) {
            Some({
                let a1247 = v1;
                let a1248 = 1u64;
                nat_sub(a1247, a1248)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f237(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1249 = v2;
    let a1250 = v0;
    if nat_lt(a1249, a1250) {
        if f264(v1, v2) {
            Ok(Some({
                let a1251 = v2;
                let a1252 = 1u64;
                nat_add(a1251, a1252)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f238(v0: Bytes, p1332: u64, p1333: u64, p1334: bool, p1335: bool, p1336: Bytes) -> R<Bytes> {
    let m1331 = (p1332, (p1333, (p1334, (p1335, p1336))));
    {
        let mut m1331 = m1331;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1331;
                let m1253 = v2;
                if m1253 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1253 - 1;
                    let m1256 = {
                        let a1254 = v0.clone();
                        let a1255 = v1;
                        index_bytes(a1254, a1255)
                    };
                    match m1256 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1301 = {
                                    let a1277 = {
                                        let a1265 = {
                                            let a1259 = {
                                                let a1257 = v8;
                                                let a1258 = 128u8;
                                                fixed_u8::bit_and(a1257, a1258)
                                            };
                                            let a1260 = 128u8;
                                            if equal(a1259, a1260) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1266 = {
                                            let a1263 = {
                                                let a1261 = v8;
                                                let a1262 = 64u8;
                                                fixed_u8::bit_and(a1261, a1262)
                                            };
                                            let a1264 = 64u8;
                                            if equal(a1263, a1264) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1265, a1266)?
                                    };
                                    let a1278 = {
                                        let a1275 = {
                                            let a1269 = {
                                                let a1267 = v8;
                                                let a1268 = 32u8;
                                                fixed_u8::bit_and(a1267, a1268)
                                            };
                                            let a1270 = 32u8;
                                            if equal(a1269, a1270) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1276 = {
                                            let a1273 = {
                                                let a1271 = v8;
                                                let a1272 = 16u8;
                                                fixed_u8::bit_and(a1271, a1272)
                                            };
                                            let a1274 = 16u8;
                                            if equal(a1273, a1274) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1275, a1276)?
                                    };
                                    nat_add(a1277, a1278)?
                                };
                                let a1302 = {
                                    let a1299 = {
                                        let a1287 = {
                                            let a1281 = {
                                                let a1279 = v8;
                                                let a1280 = 8u8;
                                                fixed_u8::bit_and(a1279, a1280)
                                            };
                                            let a1282 = 8u8;
                                            if equal(a1281, a1282) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1288 = {
                                            let a1285 = {
                                                let a1283 = v8;
                                                let a1284 = 4u8;
                                                fixed_u8::bit_and(a1283, a1284)
                                            };
                                            let a1286 = 4u8;
                                            if equal(a1285, a1286) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1287, a1288)?
                                    };
                                    let a1300 = {
                                        let a1297 = {
                                            let a1291 = {
                                                let a1289 = v8;
                                                let a1290 = 2u8;
                                                fixed_u8::bit_and(a1289, a1290)
                                            };
                                            let a1292 = 2u8;
                                            if equal(a1291, a1292) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1298 = {
                                            let a1295 = {
                                                let a1293 = v8;
                                                let a1294 = 1u8;
                                                fixed_u8::bit_and(a1293, a1294)
                                            };
                                            let a1296 = 1u8;
                                            if equal(a1295, a1296) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1297, a1298)?
                                    };
                                    nat_add(a1299, a1300)?
                                };
                                nat_add(a1301, a1302)?
                            };
                            let v10: Bytes = f40(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1303 = v1;
                                    let a1304 = 1u64;
                                    nat_add(a1303, a1304)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1305 = v9;
                                    let a1306 = 34u64;
                                    !nat_eq(a1305, a1306)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1307 = v9;
                                    let a1308 = 92u64;
                                    nat_eq(a1307, a1308)
                                }, {
                                    let a1309 = v5;
                                    let a1310 = v10;
                                    append_bytes(a1309, a1310)
                                })))))
                            } else {
                                let a1311 = v9;
                                let a1312 = 32u64;
                                if if if if nat_eq(a1311, a1312) {
                                    true
                                } else {
                                    let a1313 = v9;
                                    let a1314 = 9u64;
                                    nat_eq(a1313, a1314)
                                } {
                                    true
                                } else {
                                    let a1315 = v9;
                                    let a1316 = 10u64;
                                    nat_eq(a1315, a1316)
                                } {
                                    true
                                } else {
                                    let a1317 = v9;
                                    let a1318 = 13u64;
                                    nat_eq(a1317, a1318)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1319 = v1;
                                        let a1320 = 1u64;
                                        nat_add(a1319, a1320)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f26(f40(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1321 = v1;
                                            let a1322 = 4u64;
                                            nat_add(a1321, a1322)?
                                        }, (v6, (false, (false, {
                                            let a1323 = v5;
                                            let a1324 = f8(Str::lit("[]"));
                                            append_bytes(a1323, a1324)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1325 = v1;
                                            let a1326 = 1u64;
                                            nat_add(a1325, a1326)?
                                        }, (v6, ({
                                            let a1327 = v9;
                                            let a1328 = 34u64;
                                            nat_eq(a1327, a1328)
                                        }, (false, {
                                            let a1329 = v5;
                                            let a1330 = v10;
                                            append_bytes(a1329, a1330)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1331 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(v0: Bytes, v1: u64) -> R<u64> {
    let m1339 = {
        let a1337 = v0;
        let a1338 = v1;
        index_bytes(a1337, a1338)
    };
    match m1339 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1384 = {
                let a1360 = {
                    let a1348 = {
                        let a1342 = {
                            let a1340 = v3;
                            let a1341 = 128u8;
                            fixed_u8::bit_and(a1340, a1341)
                        };
                        let a1343 = 128u8;
                        if equal(a1342, a1343) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1349 = {
                        let a1346 = {
                            let a1344 = v3;
                            let a1345 = 64u8;
                            fixed_u8::bit_and(a1344, a1345)
                        };
                        let a1347 = 64u8;
                        if equal(a1346, a1347) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1348, a1349)?
                };
                let a1361 = {
                    let a1358 = {
                        let a1352 = {
                            let a1350 = v3;
                            let a1351 = 32u8;
                            fixed_u8::bit_and(a1350, a1351)
                        };
                        let a1353 = 32u8;
                        if equal(a1352, a1353) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1359 = {
                        let a1356 = {
                            let a1354 = v3;
                            let a1355 = 16u8;
                            fixed_u8::bit_and(a1354, a1355)
                        };
                        let a1357 = 16u8;
                        if equal(a1356, a1357) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1358, a1359)?
                };
                nat_add(a1360, a1361)?
            };
            let a1385 = {
                let a1382 = {
                    let a1370 = {
                        let a1364 = {
                            let a1362 = v3;
                            let a1363 = 8u8;
                            fixed_u8::bit_and(a1362, a1363)
                        };
                        let a1365 = 8u8;
                        if equal(a1364, a1365) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1371 = {
                        let a1368 = {
                            let a1366 = v3;
                            let a1367 = 4u8;
                            fixed_u8::bit_and(a1366, a1367)
                        };
                        let a1369 = 4u8;
                        if equal(a1368, a1369) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1370, a1371)?
                };
                let a1383 = {
                    let a1380 = {
                        let a1374 = {
                            let a1372 = v3;
                            let a1373 = 2u8;
                            fixed_u8::bit_and(a1372, a1373)
                        };
                        let a1375 = 2u8;
                        if equal(a1374, a1375) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1381 = {
                        let a1378 = {
                            let a1376 = v3;
                            let a1377 = 1u8;
                            fixed_u8::bit_and(a1376, a1377)
                        };
                        let a1379 = 1u8;
                        if equal(a1378, a1379) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1380, a1381)?
                };
                nat_add(a1382, a1383)?
            };
            nat_add(a1384, a1385)
        }
    }
}

pub fn f240(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f265(Fn15::F266(v0.clone()), {
        let a1387 = {
            let a1386 = v0;
            length_bytes(a1386)
        };
        let a1388 = 1u64;
        nat_add(a1387, a1388)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1392 = {
        let (_, h1389) = {
            let (_, h1390) = {
                let (h1391, _) = v2.clone();
                h1391
            };
            h1390
        };
        h1389
    };
    let a1393 = 1u64;
    if nat_eq(a1392, a1393) {
        Ok(Some(({
            let (h1394, _) = {
                let (h1395, _) = v2.clone();
                h1395
            };
            h1394
        }, {
            let (h1396, _) = {
                let (_, h1397) = {
                    let (h1398, _) = v2;
                    h1398
                };
                h1397
            };
            h1396
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f241(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1399 = f239(v0.clone(), v1)?;
        let a1400 = 45u64;
        nat_eq(a1399, a1400)
    };
    let v3: u64 = if v2 {
        let a1401 = v1;
        let a1402 = 1u64;
        nat_add(a1401, a1402)?
    } else {
        v1
    };
    let v4: u64 = f267(v0.clone(), v3)?;
    let a1405 = 19u64;
    let a1406 = {
        let a1403 = v4;
        let a1404 = v3;
        nat_sub(a1403, a1404)
    };
    if nat_lt(a1405, a1406) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1409 = f268(f40(v0, v3, {
            let a1407 = v4;
            let a1408 = v3;
            nat_sub(a1407, a1408)
        }))?;
        match m1409 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f269(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f242(v0: Bytes) -> Bytes {
    if f113(v0.clone(), Bytes::lit(&[0x0d])) {
        f40(v0.clone(), 0u64, {
            let a1411 = {
                let a1410 = v0;
                length_bytes(a1410)
            };
            let a1412 = 1u64;
            nat_sub(a1411, a1412)
        })
    } else {
        v0
    }
}

pub fn f243(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1413 = v0;
    match m1413.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f270(v1.clone())?;
            if f271(v3.clone())? {
                let m1414 = f243(v2)?;
                match m1414 {
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

pub fn f244(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt17>), Bytes> {
    Err::<(Bytes, List<Adt17>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f245(v0: Bytes, v1: List<Adt17>) -> Result<(Bytes, List<Adt17>), Bytes> {
    Ok::<(Bytes, List<Adt17>), Bytes>((v0, v1))
}

pub fn f246(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt17>), Bytes>> {
    let m1415 = v1;
    match m1415.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt17>), Bytes>((List::<Bytes>::nil(), List::<Adt17>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f270({
                let (h1416, _) = v3.clone();
                h1416
            })?;
            if f271(v5.clone())? {
                let m1421 = f272({
                    let (_, h1417) = v3.clone();
                    h1417
                }, {
                    let a1419 = v0.clone();
                    let a1420 = f102({
                        let (h1418, _) = v3.clone();
                        h1418
                    })?;
                    append_bytes(a1419, a1420)
                })?;
                match m1421 {
                    Ok(v7) => {
                        let v8: Bytes = if f63({
                            let (h1422, _) = v3;
                            h1422
                        }, v2.clone()) {
                            let (h1423, _) = v7;
                            h1423
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1424, _) = v7;
                                h1424
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1425 = f246(v0, v4, v2)?;
                        match m1425 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt17>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1426, _) = v10;
                                    h1426
                                }), f155({
                                    let (_, h1427) = v7;
                                    h1427
                                }, {
                                    let (_, h1428) = v10;
                                    h1428
                                }))))
                            }
                            Err(v9) => {
                                Ok(Err::<(List<Bytes>, List<Adt17>), Bytes>(v9))
                            }
                        }
                    }
                    Err(v6) => {
                        Ok(Err::<(List<Bytes>, List<Adt17>), Bytes>(v6))
                    }
                }
            } else {
                Ok(Err::<(List<Bytes>, List<Adt17>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                    let (h1429, _) = v3;
                    h1429
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f247(v0: u64) -> R<Bytes> {
    f273(f274(), 10u64, v0)
}

pub fn f248(v0: Fn9, p1434: u64, p1435: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1433 = (p1434, p1435);
    {
        let mut m1433 = m1433;
        loop {
            let step = {
                let (v1, v2) = m1433;
                let m1430 = v1;
                if m1430 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1430 - 1;
                    let m1432 = {
                        let c1431: Fn9 = v0.clone();
                        c1431.apply(v2.clone())?
                    };
                    match m1432 {
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
                Ok(next) => { m1433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f249(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f275(v0, v1)
}

pub fn f250(v0: Fn10, p1439: List<(Bytes, Bytes)>, p1440: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1438 = (p1439, p1440);
    {
        let mut m1438 = m1438;
        loop {
            let step = {
                let (v1, v2) = m1438;
                let m1436 = v2;
                match m1436.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1437: Fn10 = v0.clone();
                            c1437.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f276(v0, v1)
}

pub fn f252(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f277(v0)
}

pub fn f253(v0: Fn11, p1444: List<Bytes>, p1445: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1443 = (p1444, p1445);
    {
        let mut m1443 = m1443;
        loop {
            let step = {
                let (v1, v2) = m1443;
                let m1441 = v2;
                match m1441.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1442: Fn11 = v0.clone();
                            c1442.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f254(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f278(v1)?, v0))
}

pub fn f255(v0: Fn12, p1449: List<List<(Bytes, ())>>, p1450: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1448 = (p1449, p1450);
    {
        let mut m1448 = m1448;
        loop {
            let step = {
                let (v1, v2) = m1448;
                let m1446 = v2;
                match m1446.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1447: Fn12 = v0.clone();
                            c1447.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f257(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1452 = {
        let a1451 = v0.clone();
        length_list(a1451)
    };
    let a1453 = 1u64;
    if nat_le(a1452, a1453) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f279(v0)?))
    }
}

pub fn f258(v0: Fn13, p1457: List<(Bytes, ())>, p1458: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1456 = (p1457, p1458);
    {
        let mut m1456 = m1456;
        loop {
            let step = {
                let (v1, v2) = m1456;
                let m1454 = v2;
                match m1454.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1455: Fn13 = v0.clone();
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

pub fn f259(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f260(v0: Fn14, p1462: u64, p1463: Adt23) -> R<Adt23> {
    let m1461 = (p1462, p1463);
    {
        let mut m1461 = m1461;
        loop {
            let step = {
                let (v1, v2) = m1461;
                let m1459 = v1;
                if m1459 == 0 {
                    Err::<(u64, Adt23), R<Adt23>>(Ok(v2))
                } else {
                    let v3 = m1459 - 1;
                    Ok::<(u64, Adt23), R<Adt23>>((v3, {
                        let c1460: Fn14 = v0.clone();
                        c1460.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: Adt23) -> R<Adt23> {
    f280(v0)
}

pub fn f262(v0: u32, v1: u32) -> u32 {
    let m1466 = {
        let a1464 = f281(v0);
        let a1465 = f281(v1);
        fixed_u64::checked_add(a1464, a1465)
    };
    match m1466 {
        None => {
            0u32
        }
        Some(v2) => {
            f282(v2)
        }
    }
}

pub fn f263(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1469 = {
        let a1467 = v3;
        let a1468 = 8u64;
        nat_add(a1467, a1468)?
    };
    let a1470 = v2;
    if nat_eq(a1469, a1470) {
        Ok(f283({
            let a1473 = {
                let a1471 = v1;
                let a1472 = 8u64;
                nat_mul(a1471, a1472)?
            };
            let a1474 = 4294967296u64;
            let a1475 = 0u64;
            nat_quot(a1473, a1474, a1475)
        }))
    } else {
        let a1478 = {
            let a1476 = v3;
            let a1477 = 4u64;
            nat_add(a1476, a1477)?
        };
        let a1479 = v2;
        if nat_eq(a1478, a1479) {
            Ok(f283({
                let a1482 = {
                    let a1480 = v1;
                    let a1481 = 8u64;
                    nat_mul(a1480, a1481)?
                };
                let a1483 = 4294967296u64;
                let a1484 = 0u64;
                nat_rem(a1482, a1483, a1484)
            }))
        } else {
            Ok(f284(f285(v0.clone(), v1, v3), f285(v0.clone(), v1, {
                let a1485 = v3;
                let a1486 = 1u64;
                nat_add(a1485, a1486)?
            }), f285(v0.clone(), v1, {
                let a1487 = v3;
                let a1488 = 2u64;
                nat_add(a1487, a1488)?
            }), f285(v0, v1, {
                let a1489 = v3;
                let a1490 = 3u64;
                nat_add(a1489, a1490)?
            })))
        }
    }
}

pub fn f264(v0: Bytes, v1: u64) -> bool {
    let m1491 = f215(v0, v1);
    match m1491 {
        None => {
            false
        }
        Some(v2) => {
            f286(v2)
        }
    }
}

pub fn f265(v0: Fn15, p1496: u64, p1497: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1495 = (p1496, p1497);
    {
        let mut m1495 = m1495;
        loop {
            let step = {
                let (v1, v2) = m1495;
                let m1492 = v1;
                if m1492 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1492 - 1;
                    let m1494 = {
                        let c1493: Fn15 = v0.clone();
                        c1493.apply(v2.clone())?
                    };
                    match m1494 {
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
                Ok(next) => { m1495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f266(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f287(v0, v1)
}

pub fn f267(v0: Bytes, v1: u64) -> R<u64> {
    let (h1498, _) = f235(Fn8::F288(v0.clone()), {
        let a1500 = {
            let a1499 = v0;
            length_bytes(a1499)
        };
        let a1501 = 1u64;
        nat_add(a1500, a1501)?
    }, v1)?;
    Ok(h1498)
}

pub fn f268(v0: Bytes) -> R<Option<u64>> {
    if f289(v0.clone())? {
        Ok(Some({
            let (_, h1502) = {
                let (h1503, _) = f290(Fn16::F291(v0.clone()), {
                    let a1504 = v0;
                    length_bytes(a1504)
                }, (0u64, 0u64))?;
                h1503
            };
            h1502
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f269(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1505 = 0u64;
        let a1506 = v1;
        if nat_lt(a1505, a1506) {
            let a1507 = v1;
            let a1508 = 9223372036854775808u64;
            nat_le(a1507, a1508)
        } else {
            false
        }
    } else {
        let a1509 = v1;
        let a1510 = 9223372036854775807u64;
        nat_le(a1509, a1510)
    }
}

pub fn f270(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f153(v0.clone(), 0u64, {
        let a1511 = v0;
        length_bytes(a1511)
    }, false, Bytes::lit(&[]))?;
    let m1514 = {
        let a1512 = v1.clone();
        let a1513 = 0u64;
        index_bytes(a1512, a1513)
    };
    match m1514 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1559 = {
                    let a1535 = {
                        let a1523 = {
                            let a1517 = {
                                let a1515 = v3;
                                let a1516 = 128u8;
                                fixed_u8::bit_and(a1515, a1516)
                            };
                            let a1518 = 128u8;
                            if equal(a1517, a1518) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1524 = {
                            let a1521 = {
                                let a1519 = v3;
                                let a1520 = 64u8;
                                fixed_u8::bit_and(a1519, a1520)
                            };
                            let a1522 = 64u8;
                            if equal(a1521, a1522) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1523, a1524)?
                    };
                    let a1536 = {
                        let a1533 = {
                            let a1527 = {
                                let a1525 = v3;
                                let a1526 = 32u8;
                                fixed_u8::bit_and(a1525, a1526)
                            };
                            let a1528 = 32u8;
                            if equal(a1527, a1528) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1534 = {
                            let a1531 = {
                                let a1529 = v3;
                                let a1530 = 16u8;
                                fixed_u8::bit_and(a1529, a1530)
                            };
                            let a1532 = 16u8;
                            if equal(a1531, a1532) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1533, a1534)?
                    };
                    nat_add(a1535, a1536)?
                };
                let a1560 = {
                    let a1557 = {
                        let a1545 = {
                            let a1539 = {
                                let a1537 = v3;
                                let a1538 = 8u8;
                                fixed_u8::bit_and(a1537, a1538)
                            };
                            let a1540 = 8u8;
                            if equal(a1539, a1540) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1546 = {
                            let a1543 = {
                                let a1541 = v3;
                                let a1542 = 4u8;
                                fixed_u8::bit_and(a1541, a1542)
                            };
                            let a1544 = 4u8;
                            if equal(a1543, a1544) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1545, a1546)?
                    };
                    let a1558 = {
                        let a1555 = {
                            let a1549 = {
                                let a1547 = v3;
                                let a1548 = 2u8;
                                fixed_u8::bit_and(a1547, a1548)
                            };
                            let a1550 = 2u8;
                            if equal(a1549, a1550) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1556 = {
                            let a1553 = {
                                let a1551 = v3;
                                let a1552 = 1u8;
                                fixed_u8::bit_and(a1551, a1552)
                            };
                            let a1554 = 1u8;
                            if equal(a1553, a1554) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1555, a1556)?
                    };
                    nat_add(a1557, a1558)?
                };
                nat_add(a1559, a1560)?
            };
            if f202(v4) {
                let a1567 = f40({
                    let a1561 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a1561)
                }, {
                    let a1562 = v4;
                    let a1563 = 65u64;
                    nat_sub(a1562, a1563)
                }, 1u64);
                let a1568 = f40(v1.clone(), 1u64, {
                    let a1565 = {
                        let a1564 = v1;
                        length_bytes(a1564)
                    };
                    let a1566 = 1u64;
                    nat_sub(a1565, a1566)
                });
                Ok(append_bytes(a1567, a1568))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f271(v0: Bytes) -> R<bool> {
    let m1571 = {
        let a1569 = v0;
        let a1570 = 0u64;
        index_bytes(a1569, a1570)
    };
    match m1571 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f201({
                let v2: u8 = v1;
                let a1616 = {
                    let a1592 = {
                        let a1580 = {
                            let a1574 = {
                                let a1572 = v2;
                                let a1573 = 128u8;
                                fixed_u8::bit_and(a1572, a1573)
                            };
                            let a1575 = 128u8;
                            if equal(a1574, a1575) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1581 = {
                            let a1578 = {
                                let a1576 = v2;
                                let a1577 = 64u8;
                                fixed_u8::bit_and(a1576, a1577)
                            };
                            let a1579 = 64u8;
                            if equal(a1578, a1579) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1580, a1581)?
                    };
                    let a1593 = {
                        let a1590 = {
                            let a1584 = {
                                let a1582 = v2;
                                let a1583 = 32u8;
                                fixed_u8::bit_and(a1582, a1583)
                            };
                            let a1585 = 32u8;
                            if equal(a1584, a1585) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1591 = {
                            let a1588 = {
                                let a1586 = v2;
                                let a1587 = 16u8;
                                fixed_u8::bit_and(a1586, a1587)
                            };
                            let a1589 = 16u8;
                            if equal(a1588, a1589) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1590, a1591)?
                    };
                    nat_add(a1592, a1593)?
                };
                let a1617 = {
                    let a1614 = {
                        let a1602 = {
                            let a1596 = {
                                let a1594 = v2;
                                let a1595 = 8u8;
                                fixed_u8::bit_and(a1594, a1595)
                            };
                            let a1597 = 8u8;
                            if equal(a1596, a1597) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1603 = {
                            let a1600 = {
                                let a1598 = v2;
                                let a1599 = 4u8;
                                fixed_u8::bit_and(a1598, a1599)
                            };
                            let a1601 = 4u8;
                            if equal(a1600, a1601) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1602, a1603)?
                    };
                    let a1615 = {
                        let a1612 = {
                            let a1606 = {
                                let a1604 = v2;
                                let a1605 = 2u8;
                                fixed_u8::bit_and(a1604, a1605)
                            };
                            let a1607 = 2u8;
                            if equal(a1606, a1607) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1613 = {
                            let a1610 = {
                                let a1608 = v2;
                                let a1609 = 1u8;
                                fixed_u8::bit_and(a1608, a1609)
                            };
                            let a1611 = 1u8;
                            if equal(a1610, a1611) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1612, a1613)?
                    };
                    nat_add(a1614, a1615)?
                };
                nat_add(a1616, a1617)?
            }) {
                Ok(true)
            } else {
                Ok(f202({
                    let v3: u8 = v1;
                    let a1662 = {
                        let a1638 = {
                            let a1626 = {
                                let a1620 = {
                                    let a1618 = v3;
                                    let a1619 = 128u8;
                                    fixed_u8::bit_and(a1618, a1619)
                                };
                                let a1621 = 128u8;
                                if equal(a1620, a1621) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1627 = {
                                let a1624 = {
                                    let a1622 = v3;
                                    let a1623 = 64u8;
                                    fixed_u8::bit_and(a1622, a1623)
                                };
                                let a1625 = 64u8;
                                if equal(a1624, a1625) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1626, a1627)?
                        };
                        let a1639 = {
                            let a1636 = {
                                let a1630 = {
                                    let a1628 = v3;
                                    let a1629 = 32u8;
                                    fixed_u8::bit_and(a1628, a1629)
                                };
                                let a1631 = 32u8;
                                if equal(a1630, a1631) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1637 = {
                                let a1634 = {
                                    let a1632 = v3;
                                    let a1633 = 16u8;
                                    fixed_u8::bit_and(a1632, a1633)
                                };
                                let a1635 = 16u8;
                                if equal(a1634, a1635) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1636, a1637)?
                        };
                        nat_add(a1638, a1639)?
                    };
                    let a1663 = {
                        let a1660 = {
                            let a1648 = {
                                let a1642 = {
                                    let a1640 = v3;
                                    let a1641 = 8u8;
                                    fixed_u8::bit_and(a1640, a1641)
                                };
                                let a1643 = 8u8;
                                if equal(a1642, a1643) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1649 = {
                                let a1646 = {
                                    let a1644 = v3;
                                    let a1645 = 4u8;
                                    fixed_u8::bit_and(a1644, a1645)
                                };
                                let a1647 = 4u8;
                                if equal(a1646, a1647) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1648, a1649)?
                        };
                        let a1661 = {
                            let a1658 = {
                                let a1652 = {
                                    let a1650 = v3;
                                    let a1651 = 2u8;
                                    fixed_u8::bit_and(a1650, a1651)
                                };
                                let a1653 = 2u8;
                                if equal(a1652, a1653) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1659 = {
                                let a1656 = {
                                    let a1654 = v3;
                                    let a1655 = 1u8;
                                    fixed_u8::bit_and(a1654, a1655)
                                };
                                let a1657 = 1u8;
                                if equal(a1656, a1657) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1658, a1659)?
                        };
                        nat_add(a1660, a1661)?
                    };
                    nat_add(a1662, a1663)?
                }))
            }
        }
    }
}

pub fn f272(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1664 = f292(v0.clone());
    match m1664 {
        None => {
            if f26(f94(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1665 = f96(v0, f8(Str::lit("items")));
                match m1665 {
                    None => {
                        Ok(f244(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f293(f8(Str::lit("List")), f294(v3, {
                            let a1666 = v1;
                            let a1667 = f8(Str::lit("Item"));
                            append_bytes(a1666, a1667)
                        })?))
                    }
                }
            } else {
                f294(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f293(f8(Str::lit("Option")), f294(v2, v1)?))
        }
    }
}

pub fn f273(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1668 = v2;
    let a1669 = 0u64;
    if nat_eq(a1668, a1669) {
        Ok(f295(v0, 0u64))
    } else {
        let (_, h1670) = {
            let (h1671, _) = f248(Fn9::F296(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1671
        };
        Ok(h1670)
    }
}

pub fn f274() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f275(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1675 = {
        let a1673 = v0.clone();
        let a1674 = {
            let (h1672, _) = v1.clone();
            h1672
        };
        index_bytes(a1673, a1674)
    };
    match m1675 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1677 = {
                    let (h1676, _) = v1.clone();
                    h1676
                };
                let a1678 = 1u64;
                nat_add(a1677, a1678)?
            }, {
                let a1681 = {
                    let (_, h1679) = v1;
                    h1679
                };
                let a1682 = f297(v0, {
                    let (h1680, _) = v1;
                    h1680
                }, v2)?;
                append_bytes(a1681, a1682)
            })))
        }
    }
}

pub fn f276(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1683 = v0.clone();
    match m1683.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1686 = {
                let (h1684, _) = v2;
                h1684
            };
            let a1687 = {
                let (h1685, _) = v1.clone();
                h1685
            };
            if equal(a1686, a1687) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f277(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f298(f299(Fn17::F300, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1692 = {
        let (h1688, _) = f301(Fn18::F302, {
            let a1690 = {
                let a1689 = v0;
                length_list(a1689)
            };
            let a1691 = 1u64;
            nat_add(a1690, a1691)?
        }, v1)?;
        h1688
    };
    match m1692.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f278(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1697 = {
        let a1694 = f207({
            let (h1693, _) = v0;
            h1693
        })?;
        let a1695 = Bytes::lit(&[0x3a]);
        append_bytes(a1694, a1695)
    };
    let a1698 = {
        let (_, h1696) = v0;
        h1696
    };
    Ok(append_bytes(a1697, a1698))
}

pub fn f279(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f223({
        let (_, h1699) = {
            let (h1700, _) = f303(Fn19::F304, {
                let a1702 = {
                    let a1701 = v0.clone();
                    length_list(a1701)
                };
                let a1703 = 1u64;
                nat_add(a1702, a1703)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1700
        };
        h1699
    }))
}

pub fn f280(v0: Adt23) -> R<Adt23> {
    let v1: u32 = f305(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, _, h1704) => {
            h1704
        }
    });
    let v2: u32 = f262(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h1705, _, _) => {
            h1705
        }
    }, f262(f306(match v0.clone() {
        Adt23::C0(_, _, _, _, h1706, _, _, _, _, _) => {
            h1706
        }
    }), f262(f307(match v0.clone() {
        Adt23::C0(_, _, _, _, h1707, _, _, _, _, _) => {
            h1707
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1708, _, _, _, _) => {
            h1708
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1709, _, _, _) => {
            h1709
        }
    }), f262(v1, {
        let m1712 = match v0.clone() {
            Adt23::C0(_, _, _, _, _, _, _, _, h1711, _) => {
                h1711
            }
        };
        match m1712 {
            Adt22::C0(h1710, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1710
            }
        }
    }))));
    let v3: u32 = f262(f308(match v0.clone() {
        Adt23::C0(h1713, _, _, _, _, _, _, _, _, _) => {
            h1713
        }
    }), f309(match v0.clone() {
        Adt23::C0(h1714, _, _, _, _, _, _, _, _, _) => {
            h1714
        }
    }, match v0.clone() {
        Adt23::C0(_, h1715, _, _, _, _, _, _, _, _) => {
            h1715
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1716, _, _, _, _, _, _, _) => {
            h1716
        }
    }));
    Ok(Adt23::C0(f262(v2, v3), match v0.clone() {
        Adt23::C0(h1717, _, _, _, _, _, _, _, _, _) => {
            h1717
        }
    }, match v0.clone() {
        Adt23::C0(_, h1718, _, _, _, _, _, _, _, _) => {
            h1718
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1719, _, _, _, _, _, _, _) => {
            h1719
        }
    }, f262(match v0.clone() {
        Adt23::C0(_, _, _, h1720, _, _, _, _, _, _) => {
            h1720
        }
    }, v2), match v0.clone() {
        Adt23::C0(_, _, _, _, h1721, _, _, _, _, _) => {
            h1721
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1722, _, _, _, _) => {
            h1722
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1723, _, _, _) => {
            h1723
        }
    }, f310(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, h1724, _) => {
            h1724
        }
    }), {
        let a1726 = match v0 {
            Adt23::C0(_, _, _, _, _, _, _, _, _, h1725) => {
                h1725
            }
        };
        let a1727 = 1u64;
        nat_add(a1726, a1727)?
    }))
}

pub fn f281(v0: u32) -> u64 {
    let m1729 = {
        let a1728 = v0;
        fixed_u64::convert(i128::from(a1728))
    };
    match m1729 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f282(v0: u64) -> u32 {
    let m1733 = {
        let a1732 = {
            let a1730 = v0;
            let a1731 = 4294967295u64;
            fixed_u64::bit_and(a1730, a1731)
        };
        fixed_u32::convert(i128::from(a1732))
    };
    match m1733 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f283(v0: u64) -> u32 {
    let (_, h1734) = {
        let (_, h1735) = f311(Fn20::F312, 32u64, (v0, (1u32, 0u32)));
        h1735
    };
    h1734
}

pub fn f284(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1740 = f313(v0, 24u32);
    let a1741 = {
        let a1738 = f313(v1, 16u32);
        let a1739 = {
            let a1736 = f313(v2, 8u32);
            let a1737 = v3;
            fixed_u32::bit_or(a1736, a1737)
        };
        fixed_u32::bit_or(a1738, a1739)
    };
    fixed_u32::bit_or(a1740, a1741)
}

pub fn f285(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1742 = v2;
    let a1743 = v1;
    if nat_lt(a1742, a1743) {
        let m1746 = {
            let a1744 = v0;
            let a1745 = v2;
            index_bytes(a1744, a1745)
        };
        match m1746 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1748 = {
                    let a1747 = v3;
                    fixed_u32::convert(i128::from(a1747))
                };
                match m1748 {
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
        let a1749 = v2;
        let a1750 = v1;
        if nat_eq(a1749, a1750) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f286(v0: u8) -> bool {
    let a1751 = v0;
    let a1752 = 32u8;
    if if if equal(a1751, a1752) {
        true
    } else {
        let a1753 = v0;
        let a1754 = 9u8;
        equal(a1753, a1754)
    } {
        true
    } else {
        let a1755 = v0;
        let a1756 = 10u8;
        equal(a1755, a1756)
    } {
        true
    } else {
        let a1757 = v0;
        let a1758 = 12u8;
        if equal(a1757, a1758) {
            true
        } else {
            let a1759 = v0;
            let a1760 = 13u8;
            equal(a1759, a1760)
        }
    }
}

pub fn f287(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1761, _) = v1.clone();
        h1761
    };
    let v3: Bytes = {
        let (h1762, _) = {
            let (_, h1763) = v1.clone();
            h1763
        };
        h1762
    };
    let a1766 = {
        let (_, h1764) = {
            let (_, h1765) = v1;
            h1765
        };
        h1764
    };
    let a1767 = 0u64;
    if nat_eq(a1766, a1767) {
        let v4: u64 = f239(v0.clone(), v2)?;
        let a1768 = v4;
        let a1769 = 34u64;
        if nat_eq(a1768, a1769) {
            Ok(Some(({
                let a1770 = v2;
                let a1771 = 1u64;
                nat_add(a1770, a1771)?
            }, (v3, 1u64))))
        } else {
            let a1772 = v4;
            let a1773 = 92u64;
            if nat_eq(a1772, a1773) {
                f314(v0, v2, v3)
            } else {
                let a1774 = v4;
                let a1775 = 32u64;
                if if nat_lt(a1774, a1775) {
                    true
                } else {
                    let a1776 = v4;
                    let a1777 = 256u64;
                    nat_eq(a1776, a1777)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1778 = v2;
                        let a1779 = 1u64;
                        nat_add(a1778, a1779)?
                    }, ({
                        let a1780 = v3;
                        let a1781 = f40(v0, v2, 1u64);
                        append_bytes(a1780, a1781)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f288(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f315(v0, v1)
}

pub fn f289(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1782 = v0.clone();
        length_bytes(a1782)
    };
    let a1783 = 0u64;
    let a1784 = v1;
    if nat_lt(a1783, a1784) {
        let a1785 = f316(v0.clone())?;
        let a1786 = v1;
        if nat_eq(a1785, a1786) {
            let a1787 = 1u64;
            let a1788 = v1;
            if nat_lt(a1787, a1788) {
                let m1791 = {
                    let a1789 = v0;
                    let a1790 = 0u64;
                    index_bytes(a1789, a1790)
                };
                match m1791 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1792 = v2;
                        let a1793 = 48u8;
                        Ok(!equal(a1792, a1793))
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

pub fn f290(v0: Fn16, p1798: u64, p1799: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1797 = (p1798, p1799);
    {
        let mut m1797 = m1797;
        loop {
            let step = {
                let (v1, v2) = m1797;
                let m1794 = v1;
                if m1794 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1794 - 1;
                    let m1796 = {
                        let c1795: Fn16 = v0.clone();
                        c1795.apply(v2)?
                    };
                    match m1796 {
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
                Ok(next) => { m1797 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f317(v0, v1)
}

pub fn f292(v0: Adt14) -> Option<Adt14> {
    let m1800 = f93(f96(v0, f8(Str::lit("oneOf"))));
    match m1800.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m1801 = v2;
            match m1801.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m1802 = v4;
                    match m1802.uncons() {
                        None => {
                            if f318(v3.clone()) {
                                Some(v1)
                            } else {
                                if f318(v1) {
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

pub fn f293(v0: Bytes, v1: Result<(Bytes, List<Adt17>), Bytes>) -> Result<(Bytes, List<Adt17>), Bytes> {
    let m1803 = v1;
    match m1803 {
        Ok(v3) => {
            f245(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h1804, _) = v3;
                h1804
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h1805) = v3;
                h1805
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt17>), Bytes>(v2)
        }
    }
}

pub fn f294(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1806 = f292(v0.clone());
    match m1806 {
        None => {
            if f26(f94(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1807 = f96(v0, f8(Str::lit("items")));
                match m1807 {
                    None => {
                        Ok(f244(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f293(f8(Str::lit("List")), f319(v3, {
                            let a1808 = v1;
                            let a1809 = f8(Str::lit("Item"));
                            append_bytes(a1808, a1809)
                        })?))
                    }
                }
            } else {
                f319(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f293(f8(Str::lit("Option")), f319(v2, v1)?))
        }
    }
}

pub fn f295(v0: Bytes, v1: u64) -> Bytes {
    let m1813 = {
        let a1810 = v0;
        let a1811 = v1;
        let a1812 = 1u64;
        slice_bytes(a1810, a1811, a1812)
    };
    match m1813 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f296(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f320(v1, v0, v2)
}

pub fn f297(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a1858 = {
            let a1834 = {
                let a1822 = {
                    let a1816 = {
                        let a1814 = v3;
                        let a1815 = 128u8;
                        fixed_u8::bit_and(a1814, a1815)
                    };
                    let a1817 = 128u8;
                    if equal(a1816, a1817) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1823 = {
                    let a1820 = {
                        let a1818 = v3;
                        let a1819 = 64u8;
                        fixed_u8::bit_and(a1818, a1819)
                    };
                    let a1821 = 64u8;
                    if equal(a1820, a1821) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1822, a1823)?
            };
            let a1835 = {
                let a1832 = {
                    let a1826 = {
                        let a1824 = v3;
                        let a1825 = 32u8;
                        fixed_u8::bit_and(a1824, a1825)
                    };
                    let a1827 = 32u8;
                    if equal(a1826, a1827) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1833 = {
                    let a1830 = {
                        let a1828 = v3;
                        let a1829 = 16u8;
                        fixed_u8::bit_and(a1828, a1829)
                    };
                    let a1831 = 16u8;
                    if equal(a1830, a1831) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1832, a1833)?
            };
            nat_add(a1834, a1835)?
        };
        let a1859 = {
            let a1856 = {
                let a1844 = {
                    let a1838 = {
                        let a1836 = v3;
                        let a1837 = 8u8;
                        fixed_u8::bit_and(a1836, a1837)
                    };
                    let a1839 = 8u8;
                    if equal(a1838, a1839) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1845 = {
                    let a1842 = {
                        let a1840 = v3;
                        let a1841 = 4u8;
                        fixed_u8::bit_and(a1840, a1841)
                    };
                    let a1843 = 4u8;
                    if equal(a1842, a1843) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1844, a1845)?
            };
            let a1857 = {
                let a1854 = {
                    let a1848 = {
                        let a1846 = v3;
                        let a1847 = 2u8;
                        fixed_u8::bit_and(a1846, a1847)
                    };
                    let a1849 = 2u8;
                    if equal(a1848, a1849) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1855 = {
                    let a1852 = {
                        let a1850 = v3;
                        let a1851 = 1u8;
                        fixed_u8::bit_and(a1850, a1851)
                    };
                    let a1853 = 1u8;
                    if equal(a1852, a1853) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1854, a1855)?
            };
            nat_add(a1856, a1857)?
        };
        nat_add(a1858, a1859)?
    };
    let a1860 = v4;
    let a1861 = 34u64;
    if nat_eq(a1860, a1861) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a1862 = v4;
        let a1863 = 92u64;
        if nat_eq(a1862, a1863) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a1864 = v4;
            let a1865 = 8u64;
            if nat_eq(a1864, a1865) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a1866 = v4;
                let a1867 = 12u64;
                if nat_eq(a1866, a1867) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a1868 = v4;
                    let a1869 = 10u64;
                    if nat_eq(a1868, a1869) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a1870 = v4;
                        let a1871 = 13u64;
                        if nat_eq(a1870, a1871) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a1872 = v4;
                            let a1873 = 9u64;
                            if nat_eq(a1872, a1873) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a1874 = v4;
                                let a1875 = 32u64;
                                if nat_lt(a1874, a1875) {
                                    let a1876 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a1877 = f321(v2)?;
                                    Ok(append_bytes(a1876, a1877))
                                } else {
                                    Ok(f40(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f298(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f322(Fn21::F323, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f299(v0: Fn17, p1881: List<List<(Bytes, Bytes)>>, p1882: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m1880 = (p1881, p1882);
    {
        let mut m1880 = m1880;
        loop {
            let step = {
                let (v1, v2) = m1880;
                let m1878 = v2;
                match m1878.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c1879: Fn17 = v0.clone();
                            c1879.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1880 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f301(v0: Fn18, p1887: u64, p1888: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m1886 = (p1887, p1888);
    {
        let mut m1886 = m1886;
        loop {
            let step = {
                let (v1, v2) = m1886;
                let m1883 = v1;
                if m1883 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1883 - 1;
                    let m1885 = {
                        let c1884: Fn18 = v0.clone();
                        c1884.apply(v2.clone())?
                    };
                    match m1885 {
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
                Ok(next) => { m1886 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f324(v0)
}

pub fn f303(v0: Fn19, p1893: u64, p1894: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m1892 = (p1893, p1894);
    {
        let mut m1892 = m1892;
        loop {
            let step = {
                let (v1, v2) = m1892;
                let m1889 = v1;
                if m1889 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1889 - 1;
                    let m1891 = {
                        let c1890: Fn19 = v0.clone();
                        c1890.apply(v2.clone())?
                    };
                    match m1891 {
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
                Ok(next) => { m1892 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f325(v0)
}

pub fn f305(v0: u64) -> u32 {
    let a1895 = v0;
    let a1896 = 32u64;
    if nat_lt(a1895, a1896) {
        let a1897 = v0;
        let a1898 = 16u64;
        if nat_lt(a1897, a1898) {
            let a1899 = v0;
            let a1900 = 8u64;
            if nat_lt(a1899, a1900) {
                let a1901 = v0;
                let a1902 = 4u64;
                if nat_lt(a1901, a1902) {
                    let a1903 = v0;
                    let a1904 = 2u64;
                    if nat_lt(a1903, a1904) {
                        let a1905 = v0;
                        let a1906 = 1u64;
                        if nat_lt(a1905, a1906) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1907 = v0;
                        let a1908 = 3u64;
                        if nat_lt(a1907, a1908) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1909 = v0;
                    let a1910 = 6u64;
                    if nat_lt(a1909, a1910) {
                        let a1911 = v0;
                        let a1912 = 5u64;
                        if nat_lt(a1911, a1912) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1913 = v0;
                        let a1914 = 7u64;
                        if nat_lt(a1913, a1914) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1915 = v0;
                let a1916 = 12u64;
                if nat_lt(a1915, a1916) {
                    let a1917 = v0;
                    let a1918 = 10u64;
                    if nat_lt(a1917, a1918) {
                        let a1919 = v0;
                        let a1920 = 9u64;
                        if nat_lt(a1919, a1920) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1921 = v0;
                        let a1922 = 11u64;
                        if nat_lt(a1921, a1922) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1923 = v0;
                    let a1924 = 14u64;
                    if nat_lt(a1923, a1924) {
                        let a1925 = v0;
                        let a1926 = 13u64;
                        if nat_lt(a1925, a1926) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1927 = v0;
                        let a1928 = 15u64;
                        if nat_lt(a1927, a1928) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1929 = v0;
            let a1930 = 24u64;
            if nat_lt(a1929, a1930) {
                let a1931 = v0;
                let a1932 = 20u64;
                if nat_lt(a1931, a1932) {
                    let a1933 = v0;
                    let a1934 = 18u64;
                    if nat_lt(a1933, a1934) {
                        let a1935 = v0;
                        let a1936 = 17u64;
                        if nat_lt(a1935, a1936) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1937 = v0;
                        let a1938 = 19u64;
                        if nat_lt(a1937, a1938) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1939 = v0;
                    let a1940 = 22u64;
                    if nat_lt(a1939, a1940) {
                        let a1941 = v0;
                        let a1942 = 21u64;
                        if nat_lt(a1941, a1942) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1943 = v0;
                        let a1944 = 23u64;
                        if nat_lt(a1943, a1944) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1945 = v0;
                let a1946 = 28u64;
                if nat_lt(a1945, a1946) {
                    let a1947 = v0;
                    let a1948 = 26u64;
                    if nat_lt(a1947, a1948) {
                        let a1949 = v0;
                        let a1950 = 25u64;
                        if nat_lt(a1949, a1950) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1951 = v0;
                        let a1952 = 27u64;
                        if nat_lt(a1951, a1952) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1953 = v0;
                    let a1954 = 30u64;
                    if nat_lt(a1953, a1954) {
                        let a1955 = v0;
                        let a1956 = 29u64;
                        if nat_lt(a1955, a1956) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1957 = v0;
                        let a1958 = 31u64;
                        if nat_lt(a1957, a1958) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1959 = v0;
        let a1960 = 48u64;
        if nat_lt(a1959, a1960) {
            let a1961 = v0;
            let a1962 = 40u64;
            if nat_lt(a1961, a1962) {
                let a1963 = v0;
                let a1964 = 36u64;
                if nat_lt(a1963, a1964) {
                    let a1965 = v0;
                    let a1966 = 34u64;
                    if nat_lt(a1965, a1966) {
                        let a1967 = v0;
                        let a1968 = 33u64;
                        if nat_lt(a1967, a1968) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1969 = v0;
                        let a1970 = 35u64;
                        if nat_lt(a1969, a1970) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1971 = v0;
                    let a1972 = 38u64;
                    if nat_lt(a1971, a1972) {
                        let a1973 = v0;
                        let a1974 = 37u64;
                        if nat_lt(a1973, a1974) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1975 = v0;
                        let a1976 = 39u64;
                        if nat_lt(a1975, a1976) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1977 = v0;
                let a1978 = 44u64;
                if nat_lt(a1977, a1978) {
                    let a1979 = v0;
                    let a1980 = 42u64;
                    if nat_lt(a1979, a1980) {
                        let a1981 = v0;
                        let a1982 = 41u64;
                        if nat_lt(a1981, a1982) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1983 = v0;
                        let a1984 = 43u64;
                        if nat_lt(a1983, a1984) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1985 = v0;
                    let a1986 = 46u64;
                    if nat_lt(a1985, a1986) {
                        let a1987 = v0;
                        let a1988 = 45u64;
                        if nat_lt(a1987, a1988) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1989 = v0;
                        let a1990 = 47u64;
                        if nat_lt(a1989, a1990) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1991 = v0;
            let a1992 = 56u64;
            if nat_lt(a1991, a1992) {
                let a1993 = v0;
                let a1994 = 52u64;
                if nat_lt(a1993, a1994) {
                    let a1995 = v0;
                    let a1996 = 50u64;
                    if nat_lt(a1995, a1996) {
                        let a1997 = v0;
                        let a1998 = 49u64;
                        if nat_lt(a1997, a1998) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1999 = v0;
                        let a2000 = 51u64;
                        if nat_lt(a1999, a2000) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2001 = v0;
                    let a2002 = 54u64;
                    if nat_lt(a2001, a2002) {
                        let a2003 = v0;
                        let a2004 = 53u64;
                        if nat_lt(a2003, a2004) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2005 = v0;
                        let a2006 = 55u64;
                        if nat_lt(a2005, a2006) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2007 = v0;
                let a2008 = 60u64;
                if nat_lt(a2007, a2008) {
                    let a2009 = v0;
                    let a2010 = 58u64;
                    if nat_lt(a2009, a2010) {
                        let a2011 = v0;
                        let a2012 = 57u64;
                        if nat_lt(a2011, a2012) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2013 = v0;
                        let a2014 = 59u64;
                        if nat_lt(a2013, a2014) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2015 = v0;
                    let a2016 = 62u64;
                    if nat_lt(a2015, a2016) {
                        let a2017 = v0;
                        let a2018 = 61u64;
                        if nat_lt(a2017, a2018) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2019 = v0;
                        let a2020 = 63u64;
                        if nat_lt(a2019, a2020) {
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

pub fn f306(v0: u32) -> u32 {
    let a2023 = {
        let a2021 = f326(v0, 6u32, 26u32);
        let a2022 = f326(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2021, a2022)
    };
    let a2024 = f326(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2023, a2024)
}

pub fn f307(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2030 = {
        let a2025 = v0;
        let a2026 = v1;
        fixed_u32::bit_and(a2025, a2026)
    };
    let a2031 = {
        let a2028 = {
            let a2027 = v0;
            fixed_u32::bit_not(a2027)
        };
        let a2029 = v2;
        fixed_u32::bit_and(a2028, a2029)
    };
    fixed_u32::bit_xor(a2030, a2031)
}

pub fn f308(v0: u32) -> u32 {
    let a2034 = {
        let a2032 = f326(v0, 2u32, 30u32);
        let a2033 = f326(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2032, a2033)
    };
    let a2035 = f326(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2034, a2035)
}

pub fn f309(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2044 = {
        let a2040 = {
            let a2036 = v0;
            let a2037 = v1;
            fixed_u32::bit_and(a2036, a2037)
        };
        let a2041 = {
            let a2038 = v0;
            let a2039 = v2;
            fixed_u32::bit_and(a2038, a2039)
        };
        fixed_u32::bit_xor(a2040, a2041)
    };
    let a2045 = {
        let a2042 = v1;
        let a2043 = v2;
        fixed_u32::bit_and(a2042, a2043)
    };
    fixed_u32::bit_xor(a2044, a2045)
}

pub fn f310(v0: Adt22) -> Adt22 {
    Adt22::C0(match v0.clone() {
        Adt22::C0(_, h2046, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2046
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2047, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2047
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h2048, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2048
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h2049, _, _, _, _, _, _, _, _, _, _, _) => {
            h2049
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2050, _, _, _, _, _, _, _, _, _, _) => {
            h2050
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2051, _, _, _, _, _, _, _, _, _) => {
            h2051
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2052, _, _, _, _, _, _, _, _) => {
            h2052
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2053, _, _, _, _, _, _, _) => {
            h2053
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2054, _, _, _, _, _, _) => {
            h2054
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, h2055, _, _, _, _, _) => {
            h2055
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, h2056, _, _, _, _) => {
            h2056
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2057, _, _, _) => {
            h2057
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2058, _, _) => {
            h2058
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2059, _) => {
            h2059
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2060) => {
            h2060
        }
    }, f262(f327(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2061, _) => {
            h2061
        }
    }), f262(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2062, _, _, _, _, _, _) => {
            h2062
        }
    }, f262(f328(match v0.clone() {
        Adt22::C0(_, h2063, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2063
        }
    }), match v0 {
        Adt22::C0(h2064, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2064
        }
    }))))
}

pub fn f311(v0: Fn20, p2068: u64, p2069: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2067 = (p2068, p2069);
    {
        let mut m2067 = m2067;
        loop {
            let step = {
                let (v1, v2) = m2067;
                let m2065 = v1;
                if m2065 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2065 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2066: Fn20 = v0.clone();
                        c2066.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2067 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2070, _) = v0;
        h2070
    };
    let v2: u32 = {
        let (h2071, _) = {
            let (_, h2072) = v0;
            h2072
        };
        h2071
    };
    let v3: u32 = {
        let (_, h2073) = {
            let (_, h2074) = v0;
            h2074
        };
        h2073
    };
    ({
        let a2075 = v1;
        let a2076 = 2u64;
        let a2077 = 0u64;
        nat_quot(a2075, a2076, a2077)
    }, (f313(v2, 1u32), {
        let a2081 = {
            let a2078 = v1;
            let a2079 = 2u64;
            let a2080 = 0u64;
            nat_rem(a2078, a2079, a2080)
        };
        let a2082 = 1u64;
        if nat_eq(a2081, a2082) {
            let a2083 = v3;
            let a2084 = v2;
            fixed_u32::bit_or(a2083, a2084)
        } else {
            v3
        }
    }))
}

pub fn f313(v0: u32, v1: u32) -> u32 {
    let m2087 = {
        let a2085 = v0;
        let a2086 = v1;
        fixed_u32::shift_left(a2085, a2086)
    };
    match m2087 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f314(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f239(v0.clone(), {
        let a2088 = v1;
        let a2089 = 1u64;
        nat_add(a2088, a2089)?
    })?;
    let v4: Bytes = {
        let a2090 = v3;
        let a2091 = 117u64;
        if nat_eq(a2090, a2091) {
            f329(v0, v1)?
        } else {
            f330(v3)
        }
    };
    let a2092 = v4.clone();
    let a2093 = Bytes::lit(&[]);
    if equal(a2092, a2093) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2094 = v3;
            let a2095 = 117u64;
            if nat_eq(a2094, a2095) {
                let a2096 = v1;
                let a2097 = 6u64;
                nat_add(a2096, a2097)?
            } else {
                let a2098 = v1;
                let a2099 = 2u64;
                nat_add(a2098, a2099)?
            }
        }, ({
            let a2100 = v2;
            let a2101 = v4;
            append_bytes(a2100, a2101)
        }, 0u64))))
    }
}

pub fn f315(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f239(v0, v1)?;
    let a2102 = 47u64;
    let a2103 = v2;
    if if nat_lt(a2102, a2103) {
        let a2104 = v2;
        let a2105 = 58u64;
        nat_lt(a2104, a2105)
    } else {
        false
    } {
        Ok(Some({
            let a2106 = v1;
            let a2107 = 1u64;
            nat_add(a2106, a2107)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f316(v0: Bytes) -> R<u64> {
    let (h2108, _) = f235(Fn8::F331(v0.clone()), {
        let a2109 = v0;
        length_bytes(a2109)
    }, 0u64)?;
    Ok(h2108)
}

pub fn f317(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2113 = {
        let a2111 = v0;
        let a2112 = {
            let (h2110, _) = v1;
            h2110
        };
        index_bytes(a2111, a2112)
    };
    match m2113 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2115 = {
                    let (h2114, _) = v1;
                    h2114
                };
                let a2116 = 1u64;
                nat_add(a2115, a2116)?
            }, {
                let a2120 = {
                    let a2118 = {
                        let (_, h2117) = v1;
                        h2117
                    };
                    let a2119 = 10u64;
                    nat_mul(a2118, a2119)?
                };
                let a2121 = f332(v2)?;
                nat_add(a2120, a2121)?
            })))
        }
    }
}

pub fn f318(v0: Adt14) -> bool {
    f26(f94(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f319(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m2122 = f333(f205(v0.clone()));
    match m2122 {
        None => {
            let m2123 = f97(f96(v0.clone(), f8(Str::lit("$ref"))));
            match m2123 {
                None => {
                    let m2125 = f96(v0.clone(), f8(Str::lit("enum")));
                    match m2125 {
                        None => {
                            let m2126 = f96(v0.clone(), f8(Str::lit("const")));
                            match m2126 {
                                None => {
                                    let v8: Bytes = f94(v0.clone(), f8(Str::lit("type")));
                                    if f26(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f245(f8(Str::lit("Bytes")), List::<Adt17>::nil()))
                                    } else {
                                        if f26(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f245(f335(v0), List::<Adt17>::nil()))
                                        } else {
                                            if f26(v8.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f245(f8(Str::lit("Bool")), List::<Adt17>::nil()))
                                            } else {
                                                if f26(v8.clone(), f8(Str::lit("object"))) {
                                                    Ok(f245(v1.clone(), List::cons(Adt17::C0(v1, v0), List::<Adt17>::nil())))
                                                } else {
                                                    Ok(f244(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2127 = f97(Some(v6));
                                    match m2127 {
                                        None => {
                                            Ok(f244(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f245(f8(Str::lit("Bytes")), List::<Adt17>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f245(v1.clone(), List::cons(Adt17::C0(v1, v0), List::<Adt17>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2124 = f334(v3)?;
                    match m2124 {
                        None => {
                            Ok(f244(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f245(v4, List::<Adt17>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f244(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f320(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2128, _) = v2;
        h2128
    };
    let a2129 = v3;
    let a2130 = 0u64;
    if nat_eq(a2129, a2130) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2131 = v3;
            let a2132 = v1;
            let a2133 = 0u64;
            nat_quot(a2131, a2132, a2133)
        }, {
            let a2138 = f295(v0, {
                let a2134 = v3;
                let a2135 = v1;
                let a2136 = 0u64;
                nat_rem(a2134, a2135, a2136)
            });
            let a2139 = {
                let (_, h2137) = v2;
                h2137
            };
            append_bytes(a2138, a2139)
        }))
    }
}

pub fn f321(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2184 = {
            let a2160 = {
                let a2148 = {
                    let a2142 = {
                        let a2140 = v1;
                        let a2141 = 128u8;
                        fixed_u8::bit_and(a2140, a2141)
                    };
                    let a2143 = 128u8;
                    if equal(a2142, a2143) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2149 = {
                    let a2146 = {
                        let a2144 = v1;
                        let a2145 = 64u8;
                        fixed_u8::bit_and(a2144, a2145)
                    };
                    let a2147 = 64u8;
                    if equal(a2146, a2147) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2148, a2149)?
            };
            let a2161 = {
                let a2158 = {
                    let a2152 = {
                        let a2150 = v1;
                        let a2151 = 32u8;
                        fixed_u8::bit_and(a2150, a2151)
                    };
                    let a2153 = 32u8;
                    if equal(a2152, a2153) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2159 = {
                    let a2156 = {
                        let a2154 = v1;
                        let a2155 = 16u8;
                        fixed_u8::bit_and(a2154, a2155)
                    };
                    let a2157 = 16u8;
                    if equal(a2156, a2157) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2158, a2159)?
            };
            nat_add(a2160, a2161)?
        };
        let a2185 = {
            let a2182 = {
                let a2170 = {
                    let a2164 = {
                        let a2162 = v1;
                        let a2163 = 8u8;
                        fixed_u8::bit_and(a2162, a2163)
                    };
                    let a2165 = 8u8;
                    if equal(a2164, a2165) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2171 = {
                    let a2168 = {
                        let a2166 = v1;
                        let a2167 = 4u8;
                        fixed_u8::bit_and(a2166, a2167)
                    };
                    let a2169 = 4u8;
                    if equal(a2168, a2169) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2170, a2171)?
            };
            let a2183 = {
                let a2180 = {
                    let a2174 = {
                        let a2172 = v1;
                        let a2173 = 2u8;
                        fixed_u8::bit_and(a2172, a2173)
                    };
                    let a2175 = 2u8;
                    if equal(a2174, a2175) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2181 = {
                    let a2178 = {
                        let a2176 = v1;
                        let a2177 = 1u8;
                        fixed_u8::bit_and(a2176, a2177)
                    };
                    let a2179 = 1u8;
                    if equal(a2178, a2179) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2180, a2181)?
            };
            nat_add(a2182, a2183)?
        };
        nat_add(a2184, a2185)?
    };
    let a2192 = f295(f336(), {
        let a2186 = v2;
        let a2187 = 16u64;
        let a2188 = 0u64;
        nat_quot(a2186, a2187, a2188)
    });
    let a2193 = f295(f336(), {
        let a2189 = v2;
        let a2190 = 16u64;
        let a2191 = 0u64;
        nat_rem(a2189, a2190, a2191)
    });
    Ok(append_bytes(a2192, a2193))
}

pub fn f322(v0: Fn21, p2197: List<List<(Bytes, Bytes)>>, p2198: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2196 = (p2197, p2198);
    {
        let mut m2196 = m2196;
        loop {
            let step = {
                let (v1, v2) = m2196;
                let m2194 = v2;
                match m2194.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2195: Fn21 = v0.clone();
                            c2195.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f323(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f324(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2200 = {
        let a2199 = v0.clone();
        length_list(a2199)
    };
    let a2201 = 1u64;
    if nat_le(a2200, a2201) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f337(v0)?))
    }
}

pub fn f325(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2202, _) = v0;
        h2202
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2203) = v0;
        h2203
    };
    let m2204 = v1;
    match m2204.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2205 = v4;
            match m2205.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f338(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f326(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2206 = f232(v0, v1);
    let a2207 = f313(v0, v2);
    fixed_u32::bit_or(a2206, a2207)
}

pub fn f327(v0: u32) -> u32 {
    let a2210 = {
        let a2208 = f326(v0, 17u32, 15u32);
        let a2209 = f326(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2208, a2209)
    };
    let a2211 = f232(v0, 10u32);
    fixed_u32::bit_xor(a2210, a2211)
}

pub fn f328(v0: u32) -> u32 {
    let a2214 = {
        let a2212 = f326(v0, 7u32, 25u32);
        let a2213 = f326(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2212, a2213)
    };
    let a2215 = f232(v0, 3u32);
    fixed_u32::bit_xor(a2214, a2215)
}

pub fn f329(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2218 = f40(v0.clone(), {
        let a2216 = v1;
        let a2217 = 2u64;
        nat_add(a2216, a2217)?
    }, 2u64);
    let a2219 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2218, a2219) {
        let m2222 = f339(f40(v0, {
            let a2220 = v1;
            let a2221 = 4u64;
            nat_add(a2220, a2221)?
        }, 2u64))?;
        match m2222 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f239(v2.clone(), 0u64)?;
                let a2223 = v3;
                let a2224 = 32u64;
                if nat_lt(a2223, a2224) {
                    if f340(v3) {
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

pub fn f330(v0: u64) -> Bytes {
    let a2225 = v0;
    let a2226 = 34u64;
    if nat_eq(a2225, a2226) {
        Bytes::lit(&[0x22])
    } else {
        let a2227 = v0;
        let a2228 = 92u64;
        if nat_eq(a2227, a2228) {
            Bytes::lit(&[0x5c])
        } else {
            let a2229 = v0;
            let a2230 = 98u64;
            if nat_eq(a2229, a2230) {
                Bytes::lit(&[0x08])
            } else {
                let a2231 = v0;
                let a2232 = 102u64;
                if nat_eq(a2231, a2232) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2233 = v0;
                    let a2234 = 110u64;
                    if nat_eq(a2233, a2234) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2235 = v0;
                        let a2236 = 114u64;
                        if nat_eq(a2235, a2236) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2237 = v0;
                            let a2238 = 116u64;
                            if nat_eq(a2237, a2238) {
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

pub fn f331(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2241 = {
        let a2239 = v0;
        let a2240 = v1;
        index_bytes(a2239, a2240)
    };
    match m2241 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f341(v2)? {
                Ok(Some({
                    let a2242 = v1;
                    let a2243 = 1u64;
                    nat_add(a2242, a2243)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f332(v0: u8) -> R<u64> {
    let a2290 = {
        let v1: u8 = v0;
        let a2288 = {
            let a2264 = {
                let a2252 = {
                    let a2246 = {
                        let a2244 = v1;
                        let a2245 = 128u8;
                        fixed_u8::bit_and(a2244, a2245)
                    };
                    let a2247 = 128u8;
                    if equal(a2246, a2247) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2253 = {
                    let a2250 = {
                        let a2248 = v1;
                        let a2249 = 64u8;
                        fixed_u8::bit_and(a2248, a2249)
                    };
                    let a2251 = 64u8;
                    if equal(a2250, a2251) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2252, a2253)?
            };
            let a2265 = {
                let a2262 = {
                    let a2256 = {
                        let a2254 = v1;
                        let a2255 = 32u8;
                        fixed_u8::bit_and(a2254, a2255)
                    };
                    let a2257 = 32u8;
                    if equal(a2256, a2257) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2263 = {
                    let a2260 = {
                        let a2258 = v1;
                        let a2259 = 16u8;
                        fixed_u8::bit_and(a2258, a2259)
                    };
                    let a2261 = 16u8;
                    if equal(a2260, a2261) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2262, a2263)?
            };
            nat_add(a2264, a2265)?
        };
        let a2289 = {
            let a2286 = {
                let a2274 = {
                    let a2268 = {
                        let a2266 = v1;
                        let a2267 = 8u8;
                        fixed_u8::bit_and(a2266, a2267)
                    };
                    let a2269 = 8u8;
                    if equal(a2268, a2269) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2275 = {
                    let a2272 = {
                        let a2270 = v1;
                        let a2271 = 4u8;
                        fixed_u8::bit_and(a2270, a2271)
                    };
                    let a2273 = 4u8;
                    if equal(a2272, a2273) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2274, a2275)?
            };
            let a2287 = {
                let a2284 = {
                    let a2278 = {
                        let a2276 = v1;
                        let a2277 = 2u8;
                        fixed_u8::bit_and(a2276, a2277)
                    };
                    let a2279 = 2u8;
                    if equal(a2278, a2279) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2285 = {
                    let a2282 = {
                        let a2280 = v1;
                        let a2281 = 1u8;
                        fixed_u8::bit_and(a2280, a2281)
                    };
                    let a2283 = 1u8;
                    if equal(a2282, a2283) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2284, a2285)?
            };
            nat_add(a2286, a2287)?
        };
        nat_add(a2288, a2289)?
    };
    let a2291 = 48u64;
    Ok(nat_sub(a2290, a2291))
}

pub fn f333(p2297: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2296 = p2297;
    {
        let mut m2296 = m2296;
        loop {
            let step = {
                let v0 = m2296;
                let m2292 = v0;
                match m2292.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f63({
                            let (h2293, _) = v1.clone();
                            h2293
                        }, f342()) {
                            true
                        } else {
                            f63({
                                let (h2294, _) = v1.clone();
                                h2294
                            }, f343())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2295, _) = v1;
                                h2295
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f334(v0: Bytes) -> R<Option<Bytes>> {
    if f212(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f102(f40(v0.clone(), 8u64, {
            let a2299 = {
                let a2298 = v0;
                length_bytes(a2298)
            };
            let a2300 = 8u64;
            nat_sub(a2299, a2300)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f335(v0: Adt14) -> Bytes {
    let m2301 = f220(f96(v0, f8(Str::lit("minimum"))));
    match m2301 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f336() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f337(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f298({
        let (_, h2302) = {
            let (h2303, _) = f344(Fn22::F345, {
                let a2305 = {
                    let a2304 = v0.clone();
                    length_list(a2304)
                };
                let a2306 = 1u64;
                nat_add(a2305, a2306)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2303
        };
        h2302
    }))
}

pub fn f338(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2311 = {
            let a2309 = {
                let a2307 = v0.clone();
                length_list(a2307)
            };
            let a2310 = {
                let a2308 = v1.clone();
                length_list(a2308)
            };
            nat_add(a2309, a2310)?
        };
        let a2312 = 1u64;
        nat_add(a2311, a2312)?
    };
    Ok(f228({
        let (_, h2313) = {
            let (_, h2314) = {
                let (h2315, _) = f346(Fn23::F347, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2315
            };
            h2314
        };
        h2313
    }))
}

pub fn f339(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2316 = v0.clone();
        length_bytes(a2316)
    };
    let a2320 = {
        let a2317 = v1;
        let a2318 = 2u64;
        let a2319 = 0u64;
        nat_rem(a2317, a2318, a2319)
    };
    let a2321 = 0u64;
    if nat_eq(a2320, a2321) {
        let (_, h2322) = {
            let (h2323, _) = f348(Fn24::F349(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2323
        };
        Ok(h2322)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f340(v0: u64) -> bool {
    let a2324 = v0;
    let a2325 = 8u64;
    if nat_eq(a2324, a2325) {
        true
    } else {
        let a2326 = v0;
        let a2327 = 9u64;
        if nat_eq(a2326, a2327) {
            true
        } else {
            let a2328 = v0;
            let a2329 = 10u64;
            if nat_eq(a2328, a2329) {
                true
            } else {
                let a2330 = v0;
                let a2331 = 12u64;
                if nat_eq(a2330, a2331) {
                    true
                } else {
                    let a2332 = v0;
                    let a2333 = 13u64;
                    nat_eq(a2332, a2333)
                }
            }
        }
    }
}

pub fn f341(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2378 = {
            let a2354 = {
                let a2342 = {
                    let a2336 = {
                        let a2334 = v1;
                        let a2335 = 128u8;
                        fixed_u8::bit_and(a2334, a2335)
                    };
                    let a2337 = 128u8;
                    if equal(a2336, a2337) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2343 = {
                    let a2340 = {
                        let a2338 = v1;
                        let a2339 = 64u8;
                        fixed_u8::bit_and(a2338, a2339)
                    };
                    let a2341 = 64u8;
                    if equal(a2340, a2341) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2342, a2343)?
            };
            let a2355 = {
                let a2352 = {
                    let a2346 = {
                        let a2344 = v1;
                        let a2345 = 32u8;
                        fixed_u8::bit_and(a2344, a2345)
                    };
                    let a2347 = 32u8;
                    if equal(a2346, a2347) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2353 = {
                    let a2350 = {
                        let a2348 = v1;
                        let a2349 = 16u8;
                        fixed_u8::bit_and(a2348, a2349)
                    };
                    let a2351 = 16u8;
                    if equal(a2350, a2351) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2352, a2353)?
            };
            nat_add(a2354, a2355)?
        };
        let a2379 = {
            let a2376 = {
                let a2364 = {
                    let a2358 = {
                        let a2356 = v1;
                        let a2357 = 8u8;
                        fixed_u8::bit_and(a2356, a2357)
                    };
                    let a2359 = 8u8;
                    if equal(a2358, a2359) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2365 = {
                    let a2362 = {
                        let a2360 = v1;
                        let a2361 = 4u8;
                        fixed_u8::bit_and(a2360, a2361)
                    };
                    let a2363 = 4u8;
                    if equal(a2362, a2363) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2364, a2365)?
            };
            let a2377 = {
                let a2374 = {
                    let a2368 = {
                        let a2366 = v1;
                        let a2367 = 2u8;
                        fixed_u8::bit_and(a2366, a2367)
                    };
                    let a2369 = 2u8;
                    if equal(a2368, a2369) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2375 = {
                    let a2372 = {
                        let a2370 = v1;
                        let a2371 = 1u8;
                        fixed_u8::bit_and(a2370, a2371)
                    };
                    let a2373 = 1u8;
                    if equal(a2372, a2373) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2374, a2375)?
            };
            nat_add(a2376, a2377)?
        };
        nat_add(a2378, a2379)?
    };
    let a2380 = 48u64;
    let a2381 = v2;
    if nat_le(a2380, a2381) {
        let a2382 = v2;
        let a2383 = 57u64;
        Ok(nat_le(a2382, a2383))
    } else {
        Ok(false)
    }
}

pub fn f342() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f343() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f344(v0: Fn22, p2388: u64, p2389: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2387 = (p2388, p2389);
    {
        let mut m2387 = m2387;
        loop {
            let step = {
                let (v1, v2) = m2387;
                let m2384 = v1;
                if m2384 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2384 - 1;
                    let m2386 = {
                        let c2385: Fn22 = v0.clone();
                        c2385.apply(v2.clone())?
                    };
                    match m2386 {
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
                Ok(next) => { m2387 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f350(v0)
}

pub fn f346(v0: Fn23, p2394: u64, p2395: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2393 = (p2394, p2395);
    {
        let mut m2393 = m2393;
        loop {
            let step = {
                let (v1, v2) = m2393;
                let m2390 = v1;
                if m2390 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2390 - 1;
                    let m2392 = {
                        let c2391: Fn23 = v0.clone();
                        c2391.apply(v2.clone())
                    };
                    match m2392 {
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
                Ok(next) => { m2393 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f351(v0)
}

pub fn f348(v0: Fn24, p2400: u64, p2401: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2399 = (p2400, p2401);
    {
        let mut m2399 = m2399;
        loop {
            let step = {
                let (v1, v2) = m2399;
                let m2396 = v1;
                if m2396 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2396 - 1;
                    let m2398 = {
                        let c2397: Fn24 = v0.clone();
                        c2397.apply(v2.clone())?
                    };
                    match m2398 {
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
                Ok(next) => { m2399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f352(v0, v1)
}

pub fn f350(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2402, _) = v0;
        h2402
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2403) = v0;
        h2403
    };
    let m2404 = v1;
    match m2404.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2405 = v4;
            match m2405.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f353(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f351(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2406, _) = v0.clone();
        h2406
    };
    let v2: List<(Bytes, ())> = {
        let (h2407, _) = {
            let (_, h2408) = v0.clone();
            h2408
        };
        h2407
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2409) = {
            let (_, h2410) = v0;
            h2410
        };
        h2409
    };
    let m2411 = v1.clone();
    match m2411.uncons() {
        None => {
            let m2412 = v2;
            match m2412.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2413 = v2.clone();
            match m2413.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f354({
                        let (h2414, _) = v8.clone();
                        h2414
                    }, {
                        let (h2415, _) = v6.clone();
                        h2415
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

pub fn f352(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2416, _) = v1;
        h2416
    };
    let m2418 = {
        let (_, h2417) = v1;
        h2417
    };
    match m2418 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2421 = {
                let a2419 = v0.clone();
                let a2420 = v2;
                index_bytes(a2419, a2420)
            };
            match m2421 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2426 = {
                        let a2424 = v0;
                        let a2425 = {
                            let a2422 = v2;
                            let a2423 = 1u64;
                            nat_add(a2422, a2423)?
                        };
                        index_bytes(a2424, a2425)
                    };
                    match m2426 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f355(v4)? {
                                f355(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2427 = v2;
                                    let a2428 = 2u64;
                                    nat_add(a2427, a2428)?
                                }, Some({
                                    let a2433 = v3;
                                    let a2434 = f295(f356(), {
                                        let a2431 = {
                                            let a2429 = f357(v4)?;
                                            let a2430 = 16u64;
                                            nat_mul(a2429, a2430)?
                                        };
                                        let a2432 = f357(v5)?;
                                        nat_add(a2431, a2432)?
                                    });
                                    append_bytes(a2433, a2434)
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

pub fn f353(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a2439 = {
            let a2437 = {
                let a2435 = v0.clone();
                length_list(a2435)
            };
            let a2438 = {
                let a2436 = v1.clone();
                length_list(a2436)
            };
            nat_add(a2437, a2438)?
        };
        let a2440 = 1u64;
        nat_add(a2439, a2440)?
    };
    Ok(f358({
        let (_, h2441) = {
            let (_, h2442) = {
                let (h2443, _) = f359(Fn25::F360, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h2443
            };
            h2442
        };
        h2441
    }))
}

pub fn f354(v0: Bytes, v1: Bytes) -> bool {
    let a2448 = {
        let a2444 = v0;
        let a2445 = v1;
        compare_bytes(a2444, a2445)
    };
    let a2449 = {
        let a2446 = Bytes::lit(&[]);
        let a2447 = Bytes::lit(&[0x01]);
        compare_bytes(a2446, a2447)
    };
    equal(a2448, a2449)
}

pub fn f355(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2494 = {
            let a2470 = {
                let a2458 = {
                    let a2452 = {
                        let a2450 = v1;
                        let a2451 = 128u8;
                        fixed_u8::bit_and(a2450, a2451)
                    };
                    let a2453 = 128u8;
                    if equal(a2452, a2453) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2459 = {
                    let a2456 = {
                        let a2454 = v1;
                        let a2455 = 64u8;
                        fixed_u8::bit_and(a2454, a2455)
                    };
                    let a2457 = 64u8;
                    if equal(a2456, a2457) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2458, a2459)?
            };
            let a2471 = {
                let a2468 = {
                    let a2462 = {
                        let a2460 = v1;
                        let a2461 = 32u8;
                        fixed_u8::bit_and(a2460, a2461)
                    };
                    let a2463 = 32u8;
                    if equal(a2462, a2463) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2469 = {
                    let a2466 = {
                        let a2464 = v1;
                        let a2465 = 16u8;
                        fixed_u8::bit_and(a2464, a2465)
                    };
                    let a2467 = 16u8;
                    if equal(a2466, a2467) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2468, a2469)?
            };
            nat_add(a2470, a2471)?
        };
        let a2495 = {
            let a2492 = {
                let a2480 = {
                    let a2474 = {
                        let a2472 = v1;
                        let a2473 = 8u8;
                        fixed_u8::bit_and(a2472, a2473)
                    };
                    let a2475 = 8u8;
                    if equal(a2474, a2475) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2481 = {
                    let a2478 = {
                        let a2476 = v1;
                        let a2477 = 4u8;
                        fixed_u8::bit_and(a2476, a2477)
                    };
                    let a2479 = 4u8;
                    if equal(a2478, a2479) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2480, a2481)?
            };
            let a2493 = {
                let a2490 = {
                    let a2484 = {
                        let a2482 = v1;
                        let a2483 = 2u8;
                        fixed_u8::bit_and(a2482, a2483)
                    };
                    let a2485 = 2u8;
                    if equal(a2484, a2485) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2491 = {
                    let a2488 = {
                        let a2486 = v1;
                        let a2487 = 1u8;
                        fixed_u8::bit_and(a2486, a2487)
                    };
                    let a2489 = 1u8;
                    if equal(a2488, a2489) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2490, a2491)?
            };
            nat_add(a2492, a2493)?
        };
        nat_add(a2494, a2495)?
    };
    let a2496 = 48u64;
    let a2497 = v2;
    if if nat_le(a2496, a2497) {
        let a2498 = v2;
        let a2499 = 57u64;
        nat_le(a2498, a2499)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2500 = 97u64;
        let a2501 = v2;
        if nat_le(a2500, a2501) {
            let a2502 = v2;
            let a2503 = 102u64;
            Ok(nat_le(a2502, a2503))
        } else {
            Ok(false)
        }
    }
}

pub fn f356() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f357(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2548 = {
            let a2524 = {
                let a2512 = {
                    let a2506 = {
                        let a2504 = v1;
                        let a2505 = 128u8;
                        fixed_u8::bit_and(a2504, a2505)
                    };
                    let a2507 = 128u8;
                    if equal(a2506, a2507) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2513 = {
                    let a2510 = {
                        let a2508 = v1;
                        let a2509 = 64u8;
                        fixed_u8::bit_and(a2508, a2509)
                    };
                    let a2511 = 64u8;
                    if equal(a2510, a2511) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2512, a2513)?
            };
            let a2525 = {
                let a2522 = {
                    let a2516 = {
                        let a2514 = v1;
                        let a2515 = 32u8;
                        fixed_u8::bit_and(a2514, a2515)
                    };
                    let a2517 = 32u8;
                    if equal(a2516, a2517) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2523 = {
                    let a2520 = {
                        let a2518 = v1;
                        let a2519 = 16u8;
                        fixed_u8::bit_and(a2518, a2519)
                    };
                    let a2521 = 16u8;
                    if equal(a2520, a2521) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2522, a2523)?
            };
            nat_add(a2524, a2525)?
        };
        let a2549 = {
            let a2546 = {
                let a2534 = {
                    let a2528 = {
                        let a2526 = v1;
                        let a2527 = 8u8;
                        fixed_u8::bit_and(a2526, a2527)
                    };
                    let a2529 = 8u8;
                    if equal(a2528, a2529) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2535 = {
                    let a2532 = {
                        let a2530 = v1;
                        let a2531 = 4u8;
                        fixed_u8::bit_and(a2530, a2531)
                    };
                    let a2533 = 4u8;
                    if equal(a2532, a2533) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2534, a2535)?
            };
            let a2547 = {
                let a2544 = {
                    let a2538 = {
                        let a2536 = v1;
                        let a2537 = 2u8;
                        fixed_u8::bit_and(a2536, a2537)
                    };
                    let a2539 = 2u8;
                    if equal(a2538, a2539) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2545 = {
                    let a2542 = {
                        let a2540 = v1;
                        let a2541 = 1u8;
                        fixed_u8::bit_and(a2540, a2541)
                    };
                    let a2543 = 1u8;
                    if equal(a2542, a2543) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2544, a2545)?
            };
            nat_add(a2546, a2547)?
        };
        nat_add(a2548, a2549)?
    };
    let a2550 = v2;
    let a2551 = 57u64;
    if nat_le(a2550, a2551) {
        let a2552 = v2;
        let a2553 = 48u64;
        Ok(nat_sub(a2552, a2553))
    } else {
        let a2554 = v2;
        let a2555 = 87u64;
        Ok(nat_sub(a2554, a2555))
    }
}

pub fn f358(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f250(Fn10::F361, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f359(v0: Fn25, p2560: u64, p2561: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m2559 = (p2560, p2561);
    {
        let mut m2559 = m2559;
        loop {
            let step = {
                let (v1, v2) = m2559;
                let m2556 = v1;
                if m2556 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m2556 - 1;
                    let m2558 = {
                        let c2557: Fn25 = v0.clone();
                        c2557.apply(v2.clone())
                    };
                    match m2558 {
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
                Ok(next) => { m2559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f360(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f362(v0)
}

pub fn f361(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f362(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h2562, _) = v0.clone();
        h2562
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h2563, _) = {
            let (_, h2564) = v0.clone();
            h2564
        };
        h2563
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h2565) = {
            let (_, h2566) = v0;
            h2566
        };
        h2565
    };
    let m2567 = v1.clone();
    match m2567.uncons() {
        None => {
            let m2568 = v2;
            match m2568.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2569 = v2.clone();
            match m2569.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f354({
                        let (h2570, _) = v8.clone();
                        h2570
                    }, {
                        let (h2571, _) = v6.clone();
                        h2571
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
