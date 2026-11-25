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
    C0(Bytes, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C1(Bytes, Bytes, std::rc::Rc<Adt18>),
    C2(Bytes),
    C3(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C4,
}

#[derive(Clone)]
pub enum Adt19 {
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt19>),
    C5(std::rc::Rc<Adt19>),
    C6,
    C7(std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
    C8(Bytes, std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
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
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Bytes, Adt19),
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
pub enum Fn0 {
    F59(u8, Bytes),
    F198(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F59(k0, k1) => f59(*k0, k1.clone(), p0),
            Fn0::F198(k0) => f198(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F173,
    F194,
    F291,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F173 => f173(p0, p1),
            Fn1::F194 => f194(p0, p1),
            Fn1::F291 => f291(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F230(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F230(k0) => f230(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F281(Bytes, u64, Bytes, u64),
    F332(Bytes),
    F404(Bytes),
    F409(Bytes, Bytes),
    F410(Bytes),
    F411(Bytes, u64),
    F425(u64, Bytes),
    F472(bool, u64, Bytes),
    F526(Bytes),
    F527(Bytes),
    F563(Bytes),
    F606(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F281(k0, k1, k2, k3) => f281(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F332(k0) => f332(k0.clone(), p0),
            Fn3::F404(k0) => Ok(f404(k0.clone(), p0)),
            Fn3::F409(k0, k1) => f409(k0.clone(), k1.clone(), p0),
            Fn3::F410(k0) => f410(k0.clone(), p0),
            Fn3::F411(k0, k1) => f411(k0.clone(), *k1, p0),
            Fn3::F425(k0, k1) => f425(*k0, k1.clone(), p0),
            Fn3::F472(k0, k1, k2) => f472(*k0, *k1, k2.clone(), p0),
            Fn3::F526(k0) => f526(k0.clone(), p0),
            Fn3::F527(k0) => f527(k0.clone(), p0),
            Fn3::F563(k0) => f563(k0.clone(), p0),
            Fn3::F606(k0) => f606(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F324(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn4::F324(k0, k1, k2) => f324(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F331(Bytes),
    F344(u64, Bytes),
    F445(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F331(k0) => f331(k0.clone(), p0),
            Fn5::F344(k0, k1) => Ok(f344(*k0, k1.clone(), p0)),
            Fn5::F445(k0) => f445(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F384,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn6::F384 => f384(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F387,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn7::F387 => f387(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F389,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn8::F389 => f389(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F392,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn9::F392 => f392(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F447,
    F664,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F447 => f447(p0, p1),
            Fn10::F664 => f664(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F450,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F450 => f450(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F453,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F453 => f453(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F456,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F456 => f456(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F461,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn14::F461 => f461(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F479(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F479(k0) => f479(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F514(bool, Bytes),
    F529(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F514(k0, k1) => f514(*k0, k1.clone(), p0),
            Fn16::F529(k0) => f529(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F538,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F538 => f538(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F540,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F540 => f540(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F542,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F542 => f542(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F550,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F550 => f550(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F580,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F580 => f580(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F638,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F638 => f638(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F640,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F640 => f640(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F642(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F642(k0) => f642(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F663,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F663 => f663(p0),
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
            Ok(Adt11::C4(f35(v9)))
        }
    }
}

pub fn f28(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m177 = v0;
    match m177.uncons() {
        None => {
            Ok(Adt11::C4(f36()))
        }
        Some((v2, v3)) => {
            if f37(v2.clone(), f8(Str::lit("check"))) {
                f38(v1)
            } else {
                if f37(v2.clone(), f8(Str::lit("next"))) {
                    f39(v1, v3)
                } else {
                    if f37(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f40(v1))
                    } else {
                        if f37(v2.clone(), f8(Str::lit("publish"))) {
                            f41(v1)
                        } else {
                            if f37(v2.clone(), f8(Str::lit("agenda"))) {
                                f42(v1)
                            } else {
                                let m178 = v3;
                                match m178.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f36()))
                                    }
                                    Some((v4, v5)) => {
                                        if f37(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f43(v1, v4)
                                        } else {
                                            if f37(v2.clone(), f8(Str::lit("read-xml"))) {
                                                f44(v1, v4)
                                            } else {
                                                if f37(v2.clone(), f8(Str::lit("read-xsd"))) {
                                                    f45(v1, v4)
                                                } else {
                                                    if f37(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                        f46(v1.clone(), v4.clone(), f47(v1, v4)?)
                                                    } else {
                                                        if f37(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                            f46(v1.clone(), v4.clone(), f48(v1, v4, false)?)
                                                        } else {
                                                            if f37(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                                f46(v1.clone(), v4.clone(), f48(v1, v4, true)?)
                                                            } else {
                                                                if f37(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                                    f46(v1.clone(), v4.clone(), f49(v1, v4, false)?)
                                                                } else {
                                                                    if f37(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                        f46(v1.clone(), v4.clone(), f49(v1, v4, true)?)
                                                                    } else {
                                                                        if f37(v2.clone(), f8(Str::lit("admit"))) {
                                                                            f50(v1, v4)
                                                                        } else {
                                                                            if f37(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                                f46(v1.clone(), v4.clone(), f51(v1, v4)?)
                                                                            } else {
                                                                                if f37(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                                    f52(v1, v4)
                                                                                } else {
                                                                                    if f37(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                        f53(v1, v4)
                                                                                    } else {
                                                                                        if f37(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                            f54(v1, v4)
                                                                                        } else {
                                                                                            if f37(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                                f55(v1, v4)
                                                                                            } else {
                                                                                                if f37(v2, f8(Str::lit("propose-encoding"))) {
                                                                                                    let m179 = v5;
                                                                                                    match m179.uncons() {
                                                                                                        None => {
                                                                                                            Ok(Adt11::C4(f36()))
                                                                                                        }
                                                                                                        Some((v6, v7)) => {
                                                                                                            f56(v1, v4, v6, v7)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    Ok(Adt11::C4(f36()))
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
    }
}

pub fn f29(v0: u8) -> R<Str> {
    let m228 = {
        let a226 = f57();
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
        let (h230, _) = f58(Fn0::F59(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h230
    };
    let v4: u64 = {
        let (h231, _) = {
            let (_, h232) = v3.clone();
            h232
        };
        h231
    };
    Ok(f60(List::cons(f32(v0, v4, {
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

pub fn f33(p247: List<Bytes>) -> R<Option<Bytes>> {
    let m246 = p247;
    {
        let mut m246 = m246;
        loop {
            let step = {
                let v0 = m246;
                let m245 = v0;
                match m245.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f61(v1.clone())? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m248 = v0;
    match m248.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h249, _) = v1;
                h249
            }, f34(v2))
        }
    }
}

pub fn f35(v0: Bytes) -> Bytes {
    let a254 = {
        let a250 = Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ");
        utf8_encode(a250)
    };
    let a255 = {
        let a252 = v0;
        let a253 = {
            let a251 = Str::lit(" is not\u{a}");
            utf8_encode(a251)
        };
        append_bytes(a252, a253)
    };
    append_bytes(a254, a255)
}

pub fn f36() -> Bytes {
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | stage-mappings JSONL | stage-matrix JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | read-xml FILE | read-xsd FILE | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f37(v0: Bytes, v1: Bytes) -> bool {
    let a256 = v0;
    let a257 = v1;
    equal(a256, a257)
}

pub fn f38(v0: Adt1) -> R<Adt11> {
    let m258 = f62(v0.clone(), f63());
    match m258.uncons() {
        None => {
            let m259 = f64(v0.clone(), f63())?;
            match m259.uncons() {
                None => {
                    let m262 = f65(match v0.clone() {
                        Adt1::C0(_, h260, _) => {
                            h260
                        }
                    }, f66(f67(), f68(match v0.clone() {
                        Adt1::C0(_, h261, _) => {
                            h261
                        }
                    }, false)));
                    match m262.uncons() {
                        None => {
                            let m264 = f62(v0.clone(), f68(match v0.clone() {
                                Adt1::C0(_, h263, _) => {
                                    h263
                                }
                            }, true));
                            match m264.uncons() {
                                None => {
                                    let m265 = f62(v0.clone(), f69());
                                    match m265.uncons() {
                                        None => {
                                            let m266 = f70(v0.clone())?;
                                            match m266 {
                                                None => {
                                                    let m268 = f71(v0.clone(), f68(match v0.clone() {
                                                        Adt1::C0(_, h267, _) => {
                                                            h267
                                                        }
                                                    }, true))?;
                                                    match m268 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f72(Adt13::C0(Adt12::C0(f73(f74(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f75(v0.clone())?, f76(f77(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f73(f74(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f76(f77(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f77(v0.clone(), f8(Str::lit("model/admissions.json")))?, f77(v0.clone(), f8(Str::lit("model/strata.json")))?, f78(f77(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f68(match v0 {
                                                                Adt1::C0(_, h269, _) => {
                                                                    h269
                                                                }
                                                            }, true)))?;
                                                            let (h270, _) = v13;
                                                            if h270 {
                                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h271) = v13;
                                                                    h271
                                                                }))
                                                            } else {
                                                                Ok(Adt11::C4({
                                                                    let (_, h272) = v13;
                                                                    h272
                                                                }))
                                                            }
                                                        }
                                                        Some(v12) => {
                                                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: classical choice is an assumption, and ")), List::cons(v12, List::cons(f8(Str::lit(" depends on it\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                    }
                                                }
                                                Some(v11) => {
                                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: a negated guard in a generated workflow is one command, because `if !` negates only the first command of a chain: ")), List::cons(v11, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("register is not JSON: ")), List::cons(v3, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f39(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f79(f80(match v0.clone() {
        Adt1::C0(_, h273, _) => {
            h273
        }
    }, f81())) {
        Ok(Adt11::C1(List::cons(f81(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f82(f83(f81(), f84(match v0.clone() {
            Adt1::C0(_, h274, _) => {
                h274
            }
        }, f81())))?;
        let m275 = f62(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m275.uncons() {
            None => {
                let m276 = f85(v0.clone(), f86(f87(f74(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f88(v2, v1.clone()))?;
                match m276 {
                    None => {
                        f89(v0, v1)
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

pub fn f40(v0: Adt1) -> Adt11 {
    if f79(f90(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h277) => {
            h277
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f91(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f79(f90(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h278) => {
                h278
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f91(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f41(v0: Adt1) -> R<Adt11> {
    let m279 = f92(v0.clone());
    match m279.uncons() {
        None => {
            let m280 = f66(f62(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f93(), List::<Bytes>::nil())))), f94(v0.clone())?);
            match m280.uncons() {
                None => {
                    Ok(Adt11::C3(f95(f96(v0.clone())?, List::cons((f97(), f98()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f99()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f100()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f101()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f102()), List::cons((f8(Str::lit("docs/catalog.md")), f103(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f104(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f105(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f106()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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

pub fn f42(v0: Adt1) -> R<Adt11> {
    let m281 = f92(v0.clone());
    match m281.uncons() {
        None => {
            let m282 = f66(f62(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f94(v0.clone())?);
            match m282.uncons() {
                None => {
                    let v5: Adt15 = f107(v0.clone())?;
                    let v6: List<Adt14> = f77(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f66(f108(f109(match v5.clone() {
                        Adt15::C0(h283, _, _) => {
                            h283
                        }
                    }, f110(f77(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f111(v6.clone())), f112(f109(match v5.clone() {
                        Adt15::C0(h284, _, _) => {
                            h284
                        }
                    }, f110(f77(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f113(v5.clone(), v6, f114(v5, f115(v0.clone(), f116(v0)?)?))?)))))
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

pub fn f43(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m285 = f62(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m285.uncons() {
        None => {
            let v4: Bytes = f74(v0.clone(), v1);
            let v5: List<List<Bytes>> = f117(v4.clone())?;
            if f79(f118(f119(v5.clone()), f120())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f121(Str::lit("id"), v6.clone())?, List::cons(f121(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f121(Str::lit("artifact"), f122())?, List::cons(f121(Str::lit("checksum"), f123(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f122(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f124(f74(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f125(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f126(f127(f128(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f44(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m286 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m286.uncons() {
        None => {
            let m287 = f129(f74(v0, v1))?;
            match m287 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f130(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f131(f132(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
                }
                Err(v4) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v4, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f45(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m288 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m288.uncons() {
        None => {
            let m289 = f133(f74(v0, v1))?;
            match m289 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f131({
                        let (h290, _) = v5;
                        h290
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h291) = v5;
                        h291
                    }, List::<Bytes>::nil())))))))
                }
                Err(v4) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v4, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f46(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m292 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m292.uncons() {
        None => {
            if f134(f87(f74(v0, v1))?)? {
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

pub fn f47(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m293 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m293.uncons() {
        None => {
            let m294 = f135(f87(f74(v0.clone(), v1.clone()))?)?;
            match m294 {
                None => {
                    Ok(Adt11::C4(f136()))
                }
                Some(v4) => {
                    let m295 = f137(f87(f74(v0, v1))?)?;
                    match m295 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f126(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f138(v5)))
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

pub fn f48(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m296 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m296.uncons() {
        None => {
            let v5: Bytes = f139(if v2 {
                f140()
            } else {
                f141()
            }, f142(v1.clone())?);
            if f79(f143(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m297 = f144(f87(f74(v0.clone(), v1.clone()))?, v2)?;
                match m297 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a299 = {
                            let a298 = v6;
                            length_list(a298)
                        };
                        let a300 = 0u64;
                        if nat_eq(a299, a300) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f74(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f49(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m301 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m301.uncons() {
        None => {
            let v5: List<Bytes> = f87(f74(v0.clone(), v1.clone()))?;
            if f79(f143(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f145(v5)?
                } else {
                    f146(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f139(f147(), f142(v1.clone())?), f74(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f139(f147(), f142(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f148()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f50(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m302 = f62(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m302.uncons() {
        None => {
            let v4: List<Bytes> = f87(f74(v0.clone(), v1))?;
            let m303 = f149(v4.clone(), f8(Str::lit("requirement")))?;
            match m303 {
                None => {
                    Ok(Adt11::C4(f148()))
                }
                Some(v5) => {
                    let m304 = f149(v4, f8(Str::lit("admission")))?;
                    match m304 {
                        None => {
                            Ok(Adt11::C4(f148()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f77(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f77(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f66(f76(v5.clone(), f8(Str::lit("id"))), f76(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f82(f66(f76(v7.clone(), f8(Str::lit("id"))), f76(v8.clone(), f8(Str::lit("construct")))))?;
                            let m305 = f150(f111(v6.clone()), f110(f77(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m305 {
                                None => {
                                    let a307 = {
                                        let a306 = v9.clone();
                                        length_list(a306)
                                    };
                                    let a308 = 0u64;
                                    if nat_eq(a307, a308) {
                                        Ok(Adt11::C4(f148()))
                                    } else {
                                        if f79({
                                            let a311 = {
                                                let a309 = f82(f66(v9.clone(), v10.clone()))?;
                                                length_list(a309)
                                            };
                                            let a312 = {
                                                let a310 = f66(v9.clone(), v10);
                                                length_list(a310)
                                            };
                                            nat_eq(a311, a312)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f152(f8(Str::lit("model/requirements.json")), f153(v7, v5))?, List::cons(f152(f8(Str::lit("model/admissions.json")), f153(v8, v6))?, List::cons(f125(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f126(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f151(v11))
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

pub fn f51(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m313 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m313.uncons() {
        None => {
            let m314 = f154(f87(f74(v0, v1))?)?;
            match m314 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f126(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f52(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m315 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m315.uncons() {
        None => {
            let m316 = f155(v0.clone(), v1.clone())?;
            match m316 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m317 = f156(v4.clone(), f8(Str::lit("schema")));
                    match m317 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f157(v0, f158(v5.clone())?, v5, f159(v4.clone(), f8(Str::lit("name"))), f160(f161(f156(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f53(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m318 = f62(v0.clone(), List::cons(v1.clone(), List::cons(f93(), List::<Bytes>::nil())));
    match m318.uncons() {
        None => {
            let m319 = f162(f87(f74(v0.clone(), v1))?)?;
            match m319 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f77(v0, f93())?;
                    let v6: List<Bytes> = f76(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f82(f76(v5.clone(), f8(Str::lit("at"))))?;
                    let a321 = {
                        let a320 = v6.clone();
                        length_list(a320)
                    };
                    let a322 = 0u64;
                    if if nat_eq(a321, a322) {
                        true
                    } else {
                        f79({
                            let a325 = {
                                let a323 = f82(f66(v6.clone(), v7.clone()))?;
                                length_list(a323)
                            };
                            let a326 = {
                                let a324 = f66(v6.clone(), v7);
                                length_list(a324)
                            };
                            nat_eq(a325, a326)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f152(f93(), f153(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f126(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f54(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m327 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m327.uncons() {
        None => {
            let m328 = f163(v0, v1)?;
            match m328 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f164(f165(Some(v4))) {
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

pub fn f55(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m329 = f62(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m329.uncons() {
        None => {
            let m330 = f163(v0.clone(), v1.clone())?;
            match m330 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f79(f37(f166(v4.clone(), f8(Str::lit("schema"))), f167())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f166(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m331 = f168(v4.clone(), f8(Str::lit("source")));
                            match m331 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f169(f168(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f139(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f139(v9.clone(), f8(Str::lit("binding.json"))), f74(v0.clone(), v1));
                        let m332 = v8;
                        match m332 {
                            None => {
                                f170(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f79(f37(f166(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f170(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f139(v9, f142(f166(v7.clone(), f8(Str::lit("url"))))?);
                                    if f79(f90(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h333) => {
                                            h333
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f166(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m334 = f62(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m334.uncons() {
                                            None => {
                                                if f37(f123(f74(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f170(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f56(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m335 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m335.uncons() {
        None => {
            let m336 = f155(v0.clone(), v1.clone())?;
            match m336 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f157(v0.clone(), f74(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f57() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f58(v0: Fn0, p341: u64, p342: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m340 = (p341, p342);
    {
        let mut m340 = m340;
        loop {
            let step = {
                let (v1, v2) = m340;
                let m337 = v1;
                if m337 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m337 - 1;
                    let m339 = {
                        let c338: Fn0 = v0.clone();
                        c338.apply(v2.clone())?
                    };
                    match m339 {
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
                Ok(next) => { m340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f171(v1, v0, v2)
}

pub fn f60(v0: List<Bytes>) -> List<Bytes> {
    f172(Fn1::F173, List::<Bytes>::nil(), v0)
}

pub fn f61(v0: Bytes) -> R<bool> {
    f174(v0.clone(), 0u64, {
        let a343 = v0;
        length_bytes(a343)
    })
}

pub fn f62(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m344 = v1;
    match m344.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m346 = f175(match v0.clone() {
                Adt1::C0(h345, _, _) => {
                    h345
                }
            }, v2.clone());
            match m346 {
                None => {
                    List::cons(v2, f62(v0, v3))
                }
                Some(_) => {
                    f62(v0, v3)
                }
            }
        }
    }
}

pub fn f63() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f122(), List::<Bytes>::nil())))))))
}

pub fn f64(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m347 = v1;
    match m347.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f176(v0.clone(), v2.clone()) {
                f143(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m348 = f163(v0.clone(), v2.clone())?;
                match m348 {
                    None => {
                        Ok(List::cons(v2, f64(v0, v3)?))
                    }
                    Some(_) => {
                        f64(v0, v3)
                    }
                }
            } else {
                f64(v0, v3)
            }
        }
    }
}

pub fn f65(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m349 = v1;
    match m349.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f80(v0.clone(), v2.clone()) {
                f65(v0, v3)
            } else {
                List::cons(v2, f65(v0, v3))
            }
        }
    }
}

pub fn f66(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m350 = v0;
    match m350.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f66(v3, v1))
        }
    }
}

pub fn f67() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f68(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m351 = v0;
    match m351.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f66(f177({
                let (h352, _) = v2;
                h352
            }, {
                let (_, h353) = v2;
                h353
            }, v1), f68(v3, v1))
        }
    }
}

pub fn f69() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f70(v0: Adt1) -> R<Option<Bytes>> {
    f178(f179(v0, f69())?)
}

pub fn f71(v0: Adt1, p357: List<Bytes>) -> R<Option<Bytes>> {
    let m356 = p357;
    {
        let mut m356 = m356;
        loop {
            let step = {
                let v1 = m356;
                let m354 = v1;
                match m354.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m355 = f180(f74(v0.clone(), v2.clone()), f181())?;
                        match m355 {
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
                Ok(next) => { m356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h358, _, _, _, _) => {
            h358
        }
    };
    let v2: Adt16 = f182(v1.clone())?;
    let v3: List<Bytes> = f183(v1.clone());
    let v4: List<Bytes> = f76(match v0.clone() {
        Adt13::C0(_, _, h359, _, _) => {
            h359
        }
    }, f8(Str::lit("path")));
    let m361 = f184(v2, List::<u64>::nil(), f185(v1.clone(), match v0.clone() {
        Adt13::C0(_, h360, _, _, _) => {
            h360
        }
    }, 0u64)?);
    match m361 {
        None => {
            if f187(f188(match v0.clone() {
                Adt13::C0(_, _, _, _, h363) => {
                    h363
                }
            }, f66(match v0.clone() {
                Adt13::C0(_, _, _, _, h364) => {
                    h364
                }
            }, v4.clone()))?, f188(v4.clone(), f66(match v0.clone() {
                Adt13::C0(_, _, _, _, h365) => {
                    h365
                }
            }, v4.clone()))?, f189(match v0.clone() {
                Adt13::C0(_, _, h366, _, _) => {
                    h366
                }
            })) {
                let m370 = f190(f188(match v0.clone() {
                    Adt13::C0(_, _, _, h367, _) => {
                        h367
                    }
                }, f66(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h368, _) => {
                        h368
                    }
                }))?, f188(v3.clone(), f66(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h369, _) => {
                        h369
                    }
                }))?);
                match m370 {
                    None => {
                        let m373 = f191(f76(match v0 {
                            Adt13::C0(_, h372, _, _, _) => {
                                h372
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m373 {
                            None => {
                                let m374 = f191(v4, List::<Bytes>::nil());
                                match m374 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f126(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f186(f66(v3, match v0 {
                            Adt13::C0(_, _, _, h371, _) => {
                                h371
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f186(match v1 {
                Adt12::C0(_, _, _, _, h362) => {
                    h362
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f73(v0: Bytes) -> R<List<Bytes>> {
    f192(f87(v0)?, f8(Str::lit("id = \"")))
}

pub fn f74(v0: Adt1, v1: Bytes) -> Bytes {
    let m376 = f175(match v0 {
        Adt1::C0(h375, _, _) => {
            h375
        }
    }, v1);
    match m376 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m377 = v2;
            match m377 {
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

pub fn f75(v0: Adt1) -> R<List<Bytes>> {
    let m378 = f117(f74(v0, f122()))?;
    match m378.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f127(v2))
        }
    }
}

pub fn f76(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m379 = v0;
    match m379.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f166(v2, v1.clone()), f76(v3, v1))
        }
    }
}

pub fn f77(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f165(f163(v0, v1)?))
}

pub fn f78(v0: List<Adt14>) -> List<Bytes> {
    let m380 = v0;
    match m380.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m381 = f169(Some(v1));
            match m381 {
                None => {
                    f78(v2)
                }
                Some(v3) => {
                    List::cons(v3, f78(v2))
                }
            }
        }
    }
}

pub fn f79(v0: bool) -> bool {
    !v0
}

pub fn f80(p385: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m384 = p385;
    {
        let mut m384 = m384;
        loop {
            let step = {
                let v0 = m384;
                let m382 = v0;
                match m382.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h383, _) = v2;
                            h383
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m384 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f82(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f193(f172(Fn1::F194, List::<Bytes>::nil(), f195(v0)?)))
}

pub fn f83(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m386 = v1;
    match m386.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f143(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f139(v0.clone(), v2), f83(v0, v3))
            } else {
                f83(v0, v3)
            }
        }
    }
}

pub fn f84(p391: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m390 = p391;
    {
        let mut m390 = m390;
        loop {
            let step = {
                let v0 = m390;
                let m387 = v0;
                match m387.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h388, _) = v2;
                            h388
                        }, v1.clone()) {
                            let (_, h389) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h389)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Adt1, v1: List<List<Bytes>>, p394: List<Bytes>) -> R<Option<Bytes>> {
    let m393 = p394;
    {
        let mut m393 = m393;
        loop {
            let step = {
                let v2 = m393;
                let m392 = v2;
                match m392.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f196(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
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

pub fn f86(p397: List<Bytes>, p398: List<Bytes>, p399: List<List<Bytes>>) -> List<List<Bytes>> {
    let m396 = (p397, (p398, p399));
    {
        let mut m396 = m396;
        loop {
            let step = {
                let (v0, (v1, v2)) = m396;
                let m395 = v0;
                match m395.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f197(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f37(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f197(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a400 = v0.clone();
        length_bytes(a400)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h401, _) = f58(Fn0::F198(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h401
    };
    let v3: u64 = {
        let (h402, _) = {
            let (_, h403) = v2.clone();
            h403
        };
        h402
    };
    let v4: List<Bytes> = {
        let (_, h404) = {
            let (_, h405) = v2;
            h405
        };
        h404
    };
    let a406 = v3;
    let a407 = v1;
    if nat_lt(a406, a407) {
        Ok(f60(List::cons(f32(v0, v3, {
            let a408 = v1;
            let a409 = v3;
            nat_sub(a408, a409)
        }), v4)))
    } else {
        Ok(f60(v4))
    }
}

pub fn f88(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m410 = v0;
    match m410.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f90(v2.clone(), v1.clone()) {
                f88(v3, v1)
            } else {
                List::cons(v2, f88(v3, v1))
            }
        }
    }
}

pub fn f89(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f79(f80(match v0.clone() {
        Adt1::C0(_, h411, _) => {
            h411
        }
    }, f199())) {
        Ok(Adt11::C1(List::cons(f199(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f88(f82(f83(f199(), f84(match v0.clone() {
            Adt1::C0(_, h412, _) => {
                h412
            }
        }, f199())))?, v1.clone());
        let m413 = f62(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m413.uncons() {
            None => {
                let v5: List<Bytes> = f76(f77(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m414 = f62(v0.clone(), f200(v0.clone(), v2.clone(), v5.clone())?);
                match m414.uncons() {
                    None => {
                        let m415 = f201(v0.clone(), v2, v5)?;
                        match m415 {
                            None => {
                                f202(v0, v1)
                            }
                            Some(v8) => {
                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("admit-encoding ")), List::cons(v8, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f90(v0: Bytes, p418: List<Bytes>) -> bool {
    let m417 = p418;
    {
        let mut m417 = m417;
        loop {
            let step = {
                let v1 = m417;
                let m416 = v1;
                match m416.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f37(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f92(v0: Adt1) -> List<Bytes> {
    if f80(match v0.clone() {
        Adt1::C0(_, h419, _) => {
            h419
        }
    }, f141()) {
        if f80(match v0 {
            Adt1::C0(_, h420, _) => {
                h420
            }
        }, f140()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f140(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f141(), List::<Bytes>::nil())
    }
}

pub fn f93() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f94(v0: Adt1) -> R<List<Bytes>> {
    Ok(f62(v0.clone(), List::cons(f122(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f116(v0)?)))))
}

pub fn f95(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m421 = v0;
    match m421.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f95(v3, v1))
        }
    }
}

pub fn f96(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m422 = f77(v0.clone(), f93())?;
    match m422.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f203(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f97() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f98() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f99() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f204(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # main may have moved while this ran, so refresh checks the new pull request at once.\u{a}            gh workflow run refresh.yml --repo \"$GITHUB_REPOSITORY\" --ref main || echo \"::warning::refresh could not be started; it runs on the next push to main\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f100() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: operate starts this workflow\u{a}# right after it opens one, so every regeneration takes this one path.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f204(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f101() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}(cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log\" 2>&1\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}"))
}

pub fn f102() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f103(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f107(v0.clone())?;
    let v2: List<Adt14> = f77(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f114(v1.clone(), f115(v0.clone(), f116(v0.clone())?)?);
    let v4: List<List<Bytes>> = f128(f117(f74(v0, f122()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f205(v4.clone(), f127(v4), f206(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f113(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f104(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f207(f107(v0.clone())?, f115(v0.clone(), f116(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f105(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f208(f77(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f209(f77(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f106() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f107(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f75(v0.clone())?, f73(f74(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f76(f77(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f108(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m423 = v0;
    match m423.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f90(v2.clone(), v1.clone()) {
                f108(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f108(v3, v1))
            }
        }
    }
}

pub fn f109(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m424 = v0;
    match m424.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f90(v2.clone(), v1.clone()) {
                f109(v3, v1)
            } else {
                List::cons(v2, f109(v3, v1))
            }
        }
    }
}

pub fn f110(v0: List<Adt14>) -> List<Bytes> {
    let m425 = v0;
    match m425.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f66(f78(f165(f168(v1, f8(Str::lit("defers"))))), f110(v2))
        }
    }
}

pub fn f111(v0: List<Adt14>) -> List<Bytes> {
    let m426 = v0;
    match m426.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m427 = f168(v1, f8(Str::lit("std")));
            match m427 {
                None => {
                    f111(v2)
                }
                Some(v3) => {
                    List::cons(f166(v3, f8(Str::lit("row"))), f111(v2))
                }
            }
        }
    }
}

pub fn f112(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m428 = v0;
    match m428.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f90(v2.clone(), v1.clone()) {
                f112(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f112(v3, v1))
            }
        }
    }
}

pub fn f113(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f95(f210(v2.clone()), f211(f206(v0, v1.clone(), v2)?));
    f212(f82(f213(v1))?, v3.clone(), {
        let a430 = {
            let a429 = v3;
            length_list(a429)
        };
        let a431 = 1u64;
        nat_add(a430, a431)?
    })
}

pub fn f114(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m432 = v1;
    match m432.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f214(v2.clone(), f8(Str::lit("subject"))) {
                f215(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f114(v0, v3))
            } else {
                f114(v0, v3)
            }
        }
    }
}

pub fn f115(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m433 = v1;
    match m433.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m434 = f144(f87(f74(v0.clone(), v2.clone()))?, f216(v2, f140()))?;
            match m434 {
                None => {
                    f115(v0, v3)
                }
                Some(v4) => {
                    Ok(f153(v4, f115(v0, v3)?))
                }
            }
        }
    }
}

pub fn f116(v0: Adt1) -> R<List<Bytes>> {
    Ok(f66(f82(f217(f141(), f84(match v0.clone() {
        Adt1::C0(_, h435, _) => {
            h435
        }
    }, f141())))?, f82(f217(f140(), f84(match v0 {
        Adt1::C0(_, h436, _) => {
            h436
        }
    }, f140())))?))
}

pub fn f117(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f218(v0.clone(), 0u64, {
        let a437 = v0;
        length_bytes(a437)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a440 = {
            let a439 = match v1.clone() {
                Adt17::C0(_, _, h438, _, _) => {
                    h438
                }
            };
            length_bytes(a439)
        };
        let a441 = 0u64;
        if if nat_eq(a440, a441) {
            let m443 = match v1.clone() {
                Adt17::C0(_, _, _, h442, _) => {
                    h442
                }
            };
            match m443.uncons() {
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
            f219(v1)
        }
    };
    Ok(f220(match v4 {
        Adt17::C0(_, _, _, _, h444) => {
            h444
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f118(p449: List<Bytes>, p450: List<Bytes>) -> bool {
    let m448 = (p449, p450);
    {
        let mut m448 = m448;
        loop {
            let step = {
                let (v0, v1) = m448;
                let m445 = v0;
                match m445.uncons() {
                    None => {
                        let m446 = v1;
                        match m446.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m447 = v1;
                        match m447.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f37(v4, v6) {
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
                Ok(next) => { m448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: List<List<Bytes>>) -> List<Bytes> {
    let m451 = v0;
    match m451.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f120() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f121(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f221(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f122() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f123(v0: Bytes) -> R<Bytes> {
    Ok(f222(f223(v0)?))
}

pub fn f124(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f224(f86(f87(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f225(f226(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f125(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f227(Adt14::C3(std::rc::Rc::new(f228(f82(f66(v1, f78(f77(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f126(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m452 = f229(Fn2::F230(v1), None::<Bytes>, v0);
    match m452 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f127(v0: List<List<Bytes>>) -> List<Bytes> {
    let m453 = v0;
    match m453.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m454 = v1;
            match m454.uncons() {
                None => {
                    f127(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f127(v2))
                }
            }
        }
    }
}

pub fn f128(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m455 = v0;
    match m455.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f129(v0: Bytes) -> R<Result<Adt18, Bytes>> {
    let v1: Bytes = if f231(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f32(v0.clone(), 3u64, {
            let a457 = {
                let a456 = v0;
                length_bytes(a456)
            };
            let a458 = 3u64;
            nat_sub(a457, a458)
        })
    } else {
        v0
    };
    let v2: Bytes = f232(v1)?;
    let v3: u64 = {
        let a459 = v2.clone();
        length_bytes(a459)
    };
    let v4: u64 = f233(v2.clone())?;
    let a460 = v4;
    let a461 = v3;
    if nat_lt(a460, a461) {
        Ok(Err::<Adt18, Bytes>(f234(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m462 = f235(v2.clone())?;
        match m462 {
            Ok(v6) => {
                let m465 = f236(v2.clone(), v6, {
                    let a463 = v3;
                    let a464 = 2u64;
                    nat_add(a463, a464)?
                })?;
                match m465 {
                    Ok(v8) => {
                        if f237(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(Err::<Adt18, Bytes>(f234(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a466 = f238(v2.clone(), v8)?;
                            let a467 = 60u64;
                            if !nat_eq(a466, a467) {
                                Ok(Err::<Adt18, Bytes>(f234(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            } else {
                                let m470 = f239(v2.clone(), v8, {
                                    let a468 = v3;
                                    let a469 = 2u64;
                                    nat_add(a468, a469)?
                                }, 0u64)?;
                                match m470 {
                                    Ok(v10) => {
                                        let m474 = f236(v2, {
                                            let (_, h471) = v10;
                                            h471
                                        }, {
                                            let a472 = v3;
                                            let a473 = 2u64;
                                            nat_add(a472, a473)?
                                        })?;
                                        match m474 {
                                            Ok(v12) => {
                                                let a475 = v12;
                                                let a476 = v3;
                                                if nat_eq(a475, a476) {
                                                    Ok(Ok::<Adt18, Bytes>({
                                                        let (h477, _) = v10;
                                                        h477
                                                    }))
                                                } else {
                                                    Ok(Err::<Adt18, Bytes>(f234(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt18, Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<Adt18, Bytes>(v9))
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt18, Bytes>(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(Err::<Adt18, Bytes>(v5))
            }
        }
    }
}

pub fn f130(v0: Adt18) -> Bytes {
    let m478 = v0;
    match m478 {
        Adt18::C0(v1, _, _) => {
            v1
        }
        Adt18::C1(_, _, _) => {
            Bytes::lit(&[])
        }
        Adt18::C2(_) => {
            Bytes::lit(&[])
        }
        Adt18::C3(_, _) => {
            Bytes::lit(&[])
        }
        Adt18::C4 => {
            Bytes::lit(&[])
        }
    }
}

pub fn f131(v0: u64) -> R<Bytes> {
    f240(f241(), 10u64, v0)
}

pub fn f132(v0: Adt18) -> R<u64> {
    let m479 = v0;
    match m479 {
        Adt18::C0(_, _, r484) => {
            let v3 = (*r484).clone();
            let a480 = 1u64;
            let a481 = f132(v3)?;
            nat_add(a480, a481)
        }
        Adt18::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt18::C2(_) => {
            Ok(0u64)
        }
        Adt18::C3(r485, r486) => {
            let v8 = (*r485).clone();
            let v9 = (*r486).clone();
            let a482 = f132(v8)?;
            let a483 = f132(v9)?;
            nat_add(a482, a483)
        }
        Adt18::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f133(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m487 = f129(v0)?;
    match m487 {
        Ok(v2) => {
            let m488 = v2;
            match m488 {
                Adt18::C0(v3, r500, r501) => {
                    let v4 = (*r500).clone();
                    let v5 = (*r501).clone();
                    let m489 = f242(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m489 {
                        Ok(v7) => {
                            let m490 = f243(v7.clone(), v3)?;
                            match m490 {
                                Ok(v9) => {
                                    if f244(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m491 = f245(v4, f8(Str::lit("targetNamespace")));
                                            match m491 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m492 = f246(v5, v7, v11)?;
                                        match m492 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f247(v13.clone());
                                                let m493 = f248(v14.clone());
                                                match m493 {
                                                    None => {
                                                        let m494 = f249(v13.clone(), v14.clone());
                                                        match m494 {
                                                            None => {
                                                                let m498 = f250({
                                                                    let a496 = {
                                                                        let a495 = v14.clone();
                                                                        length_list(a495)
                                                                    };
                                                                    let a497 = 1u64;
                                                                    nat_add(a496, a497)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m498 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a499 = v14;
                                                                            length_list(a499)
                                                                        }, f226(f251(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(Err::<(u64, Bytes), Bytes>(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(Err::<(u64, Bytes), Bytes>(v12))
                                            }
                                        }
                                    } else {
                                        Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(Err::<(u64, Bytes), Bytes>(v6))
                        }
                    }
                }
                Adt18::C1(_, _, _) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C2(_) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C3(_, _) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C4 => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f134(p507: List<Bytes>) -> R<bool> {
    let m506 = p507;
    {
        let mut m506 = m506;
        loop {
            let step = {
                let v0 = m506;
                let m502 = v0;
                match m502.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a504 = {
                            let a503 = f252(v1)?;
                            length_bytes(a503)
                        };
                        let a505 = 0u64;
                        if nat_eq(a504, a505) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
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

pub fn f135(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m508 = v0;
    match m508.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f252(v1)?;
            let a510 = {
                let a509 = v3.clone();
                length_bytes(a509)
            };
            let a511 = 0u64;
            if nat_eq(a510, a511) {
                f135(v2)
            } else {
                let m512 = f253(v3.clone())?;
                match m512 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f166(v4.clone(), f8(Str::lit("id")));
                        if f254(f166(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m513 = f135(v2)?;
                            match m513 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f255(v5), {
                                        let a514 = v3;
                                        let a515 = Bytes::lit(&[0x0a]);
                                        append_bytes(a514, a515)
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

pub fn f136() -> Bytes {
    let a516 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a516)
}

pub fn f137(p521: List<Bytes>) -> R<Option<Bytes>> {
    let m520 = p521;
    {
        let mut m520 = m520;
        loop {
            let step = {
                let v0 = m520;
                let m517 = v0;
                match m517.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m518 = f253(f252(v1)?)?;
                        match m518 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m519 = f168(v3.clone(), f8(Str::lit("source")));
                                match m519 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f256(f166(v4.clone(), f8(Str::lit("redistribution"))), f166(v4, f8(Str::lit("url"))))? {
                                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f166(v3, f8(Str::lit("id"))))))
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
                Ok(next) => { m520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(v0: Bytes) -> Bytes {
    let a526 = {
        let a522 = Str::lit("binding ");
        utf8_encode(a522)
    };
    let a527 = {
        let a524 = v0;
        let a525 = {
            let a523 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a523)
        };
        append_bytes(a524, a525)
    };
    append_bytes(a526, a527)
}

pub fn f139(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f140() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f141() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f142(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f143(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a528 = v0.clone();
        length_bytes(a528)
    };
    let v3: u64 = {
        let a529 = v1.clone();
        length_bytes(a529)
    };
    let a530 = v3;
    let a531 = v2;
    if nat_le(a530, a531) {
        let m537 = {
            let a534 = v0;
            let a535 = {
                let a532 = v2;
                let a533 = v3;
                nat_sub(a532, a533)
            };
            let a536 = v3;
            slice_bytes(a534, a535, a536)
        };
        match m537 {
            None => {
                false
            }
            Some(v4) => {
                let a538 = v4;
                let a539 = v1;
                equal(a538, a539)
            }
        }
    } else {
        false
    }
}

pub fn f144(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m540 = v0;
    match m540.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f252(v2)?;
            let a542 = {
                let a541 = v4.clone();
                length_bytes(a541)
            };
            let a543 = 0u64;
            if nat_eq(a542, a543) {
                f144(v3, v1)
            } else {
                let m544 = f253(v4)?;
                match m544 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f257(v5.clone())
                        } else {
                            f258(v5.clone())
                        } {
                            let m545 = f144(v3, v1)?;
                            match m545 {
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

pub fn f145(v0: List<Bytes>) -> R<bool> {
    if f259(f162(v0.clone())?) {
        Ok(f79({
            let a547 = {
                let a546 = f260(f162(v0)?);
                length_list(a546)
            };
            let a548 = 0u64;
            nat_eq(a547, a548)
        }))
    } else {
        Ok(false)
    }
}

pub fn f146(v0: List<Bytes>) -> R<bool> {
    if f259(f149(v0.clone(), f8(Str::lit("requirement")))?) {
        if f259(f149(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f79({
                let a550 = {
                    let a549 = f153(f260(f149(v0.clone(), f8(Str::lit("requirement")))?), f260(f149(v0, f8(Str::lit("admission")))?));
                    length_list(a549)
                };
                let a551 = 0u64;
                nat_eq(a550, a551)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f147() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f148() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f149(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m552 = v0;
    match m552.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f252(v2)?;
            let a554 = {
                let a553 = v4.clone();
                length_bytes(a553)
            };
            let a555 = 0u64;
            if nat_eq(a554, a555) {
                f149(v3, v1)
            } else {
                let m556 = f253(v4)?;
                match m556 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f261(v5.clone());
                        let a558 = {
                            let a557 = v6.clone();
                            length_bytes(a557)
                        };
                        let a559 = 0u64;
                        if nat_eq(a558, a559) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m560 = f149(v3, v1.clone())?;
                            match m560 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v7) => {
                                    if f37(v6, v1) {
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

pub fn f150(p563: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m562 = p563;
    {
        let mut m562 = m562;
        loop {
            let step = {
                let v0 = m562;
                let m561 = v0;
                match m561.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f90(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m562 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f152(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f227(Adt14::C3(std::rc::Rc::new(f262(v1))))?))
}

pub fn f153(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m564 = v0;
    match m564.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f153(v3, v1))
        }
    }
}

pub fn f154(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m565 = v0;
    match m565.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f252(v1)?;
            let a567 = {
                let a566 = v3.clone();
                length_bytes(a566)
            };
            let a568 = 0u64;
            if nat_eq(a567, a568) {
                f154(v2)
            } else {
                let m569 = f263(v3.clone())?;
                match m569 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f264(v4.clone()) {
                            let m570 = f154(v2)?;
                            match m570 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f199(), List::cons(f8(Str::lit("/")), List::cons(f265(f159(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a571 = v3;
                                        let a572 = Bytes::lit(&[0x0a]);
                                        append_bytes(a571, a572)
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

pub fn f155(v0: Adt1, v1: Bytes) -> R<Option<Adt19>> {
    f263(f74(v0, v1))
}

pub fn f156(p578: Adt19, v1: Bytes) -> Option<Adt19> {
    let m577 = p578;
    {
        let mut m577 = m577;
        loop {
            let step = {
                let v0 = m577;
                let m573 = v0;
                match m573 {
                    Adt19::C0(_) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C1(_, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C2(_, _, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C3(_) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C4(_) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C5(r574) => {
                        let v5 = (*r574).clone();
                        Ok::<Adt19, Option<Adt19>>(v5)
                    }
                    Adt19::C6 => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C7(_, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C8(v2, r575, r576) => {
                        let v3 = (*r575).clone();
                        let v4 = (*r576).clone();
                        if f37(v2, v1.clone()) {
                            Err::<Adt19, Option<Adt19>>(Some(v3))
                        } else {
                            Ok::<Adt19, Option<Adt19>>(v4)
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

pub fn f157(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m579 = f62(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m579.uncons() {
        None => {
            let m581 = f150(f111(List::cons(Adt14::C4(std::rc::Rc::new({
                let m580 = f266(v4.clone());
                match m580 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f110(f77(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m581 {
                None => {
                    f267(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f151(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f158(v0: Adt19) -> R<Bytes> {
    let a582 = f268(v0)?;
    let a583 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a582, a583))
}

pub fn f159(v0: Adt19, v1: Bytes) -> Bytes {
    let m584 = f269(f156(v0, v1));
    match m584 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f160(v0: List<Adt19>) -> List<Bytes> {
    let m585 = v0;
    match m585.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m586 = f269(Some(v1));
            match m586 {
                None => {
                    f160(v2)
                }
                Some(v3) => {
                    List::cons(v3, f160(v2))
                }
            }
        }
    }
}

pub fn f161(v0: Option<Adt19>) -> List<Adt19> {
    let m587 = v0;
    match m587 {
        None => {
            List::<Adt19>::nil()
        }
        Some(v1) => {
            f270(v1)
        }
    }
}

pub fn f162(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m588 = v0;
    match m588.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f252(v1)?;
            let a590 = {
                let a589 = v3.clone();
                length_bytes(a589)
            };
            let a591 = 0u64;
            if nat_eq(a590, a591) {
                f162(v2)
            } else {
                let m592 = f253(v3)?;
                match m592 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f214(v4.clone(), f8(Str::lit("agent"))) {
                            if f214(v4.clone(), f8(Str::lit("at"))) {
                                if f214(v4.clone(), f8(Str::lit("quote"))) {
                                    f271(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m593 = f162(v2)?;
                            match m593 {
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

pub fn f163(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f253(f74(v0, v1))
}

pub fn f164(p596: List<Adt14>) -> bool {
    let m595 = p596;
    {
        let mut m595 = m595;
        loop {
            let step = {
                let v0 = m595;
                let m594 = v0;
                match m594.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f37(f166(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f90(f8(Str::lit("studio")), f76(f272(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m595 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: Option<Adt14>) -> List<Adt14> {
    let m597 = v0;
    match m597 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f273(v1)
        }
    }
}

pub fn f166(v0: Adt14, v1: Bytes) -> Bytes {
    let m598 = f169(f168(v0, v1));
    match m598 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f167() -> Bytes {
    f274()
}

pub fn f168(p604: Adt14, v1: Bytes) -> Option<Adt14> {
    let m603 = p604;
    {
        let mut m603 = m603;
        loop {
            let step = {
                let v0 = m603;
                let m599 = v0;
                match m599 {
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
                    Adt14::C4(r600) => {
                        let v5 = (*r600).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r601, r602) => {
                        let v3 = (*r601).clone();
                        let v4 = (*r602).clone();
                        if f37(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m603 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Option<Adt14>) -> Option<Bytes> {
    let m605 = v0;
    match m605 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m606 = v1;
            match m606 {
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

pub fn f170(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f124(f74(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f275(v1, v4)?)?), List::cons(f125(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f171(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h607, _) = v2.clone();
        h607
    };
    let v4: u64 = {
        let (h608, _) = {
            let (_, h609) = v2.clone();
            h609
        };
        h608
    };
    let v5: List<Bytes> = {
        let (_, h610) = {
            let (_, h611) = v2;
            h611
        };
        h610
    };
    let m612 = f276(v0.clone(), v3);
    match m612 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a613 = v6;
            let a614 = v1;
            if equal(a613, a614) {
                Ok(Some(({
                    let a615 = v3;
                    let a616 = 1u64;
                    nat_add(a615, a616)?
                }, ({
                    let a617 = v3;
                    let a618 = 1u64;
                    nat_add(a617, a618)?
                }, List::cons(f32(v0, v4, {
                    let a619 = v3;
                    let a620 = v4;
                    nat_sub(a619, a620)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a621 = v3;
                    let a622 = 1u64;
                    nat_add(a621, a622)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f172(v0: Fn1, p626: List<Bytes>, p627: List<Bytes>) -> List<Bytes> {
    let m625 = (p626, p627);
    {
        let mut m625 = m625;
        loop {
            let step = {
                let (v1, v2) = m625;
                let m623 = v2;
                match m623.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c624: Fn1 = v0.clone();
                            c624.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f174(v0: Bytes, p681: u64, p682: u64) -> R<bool> {
    let m680 = (p681, p682);
    {
        let mut m680 = m680;
        loop {
            let step = {
                let (v1, v2) = m680;
                let m628 = v2;
                if m628 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m628 - 1;
                    let m631 = {
                        let a629 = v0.clone();
                        let a630 = v1;
                        index_bytes(a629, a630)
                    };
                    match m631 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f277({
                                let v5: u8 = v4;
                                let a676 = {
                                    let a652 = {
                                        let a640 = {
                                            let a634 = {
                                                let a632 = v5;
                                                let a633 = 128u8;
                                                fixed_u8::bit_and(a632, a633)
                                            };
                                            let a635 = 128u8;
                                            if equal(a634, a635) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a641 = {
                                            let a638 = {
                                                let a636 = v5;
                                                let a637 = 64u8;
                                                fixed_u8::bit_and(a636, a637)
                                            };
                                            let a639 = 64u8;
                                            if equal(a638, a639) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a640, a641)?
                                    };
                                    let a653 = {
                                        let a650 = {
                                            let a644 = {
                                                let a642 = v5;
                                                let a643 = 32u8;
                                                fixed_u8::bit_and(a642, a643)
                                            };
                                            let a645 = 32u8;
                                            if equal(a644, a645) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a651 = {
                                            let a648 = {
                                                let a646 = v5;
                                                let a647 = 16u8;
                                                fixed_u8::bit_and(a646, a647)
                                            };
                                            let a649 = 16u8;
                                            if equal(a648, a649) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a650, a651)?
                                    };
                                    nat_add(a652, a653)?
                                };
                                let a677 = {
                                    let a674 = {
                                        let a662 = {
                                            let a656 = {
                                                let a654 = v5;
                                                let a655 = 8u8;
                                                fixed_u8::bit_and(a654, a655)
                                            };
                                            let a657 = 8u8;
                                            if equal(a656, a657) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a663 = {
                                            let a660 = {
                                                let a658 = v5;
                                                let a659 = 4u8;
                                                fixed_u8::bit_and(a658, a659)
                                            };
                                            let a661 = 4u8;
                                            if equal(a660, a661) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a662, a663)?
                                    };
                                    let a675 = {
                                        let a672 = {
                                            let a666 = {
                                                let a664 = v5;
                                                let a665 = 2u8;
                                                fixed_u8::bit_and(a664, a665)
                                            };
                                            let a667 = 2u8;
                                            if equal(a666, a667) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a673 = {
                                            let a670 = {
                                                let a668 = v5;
                                                let a669 = 1u8;
                                                fixed_u8::bit_and(a668, a669)
                                            };
                                            let a671 = 1u8;
                                            if equal(a670, a671) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a672, a673)?
                                    };
                                    nat_add(a674, a675)?
                                };
                                nat_add(a676, a677)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a678 = v1;
                                    let a679 = 1u64;
                                    nat_add(a678, a679)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(p687: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m686 = p687;
    {
        let mut m686 = m686;
        loop {
            let step = {
                let v0 = m686;
                let m683 = v0;
                match m683.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h684, _) = v2;
                            h684
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h685) = v2;
                                h685
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m686 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: Adt1, v1: Bytes) -> bool {
    let m689 = f175(match v0 {
        Adt1::C0(h688, _, _) => {
            h688
        }
    }, v1);
    match m689 {
        None => {
            false
        }
        Some(v2) => {
            let m690 = v2;
            match m690 {
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

pub fn f177(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m691 = v1;
    match m691.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f278(v3.clone())
            } else {
                f79(f278(v3.clone()))
            } {
                List::cons(f139(v0.clone(), v3), f177(v0, v4, v2))
            } else {
                f177(v0, v4, v2)
            }
        }
    }
}

pub fn f178(p694: List<Bytes>) -> R<Option<Bytes>> {
    let m693 = p694;
    {
        let mut m693 = m693;
        loop {
            let step = {
                let v0 = m693;
                let m692 = v0;
                match m692.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f279(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f252(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m693 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m695 = v1;
    match m695.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f66(f87(f74(v0.clone(), v2))?, f179(v0, v3)?))
        }
    }
}

pub fn f180(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a696 = v0.clone();
        length_bytes(a696)
    };
    let v3: u64 = {
        let a697 = v1.clone();
        length_bytes(a697)
    };
    let v4: u64 = {
        let (h698, _) = f280(Fn3::F281(v1.clone(), v3, v0.clone(), v2), {
            let a699 = v2;
            let a700 = 1u64;
            nat_add(a699, a700)?
        }, 0u64)?;
        h698
    };
    if f231(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f181() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f182(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f282(match v0.clone() {
        Adt12::C0(h701, _, _, _, _) => {
            h701
        }
    }, 0u64)?, f282(match v0.clone() {
        Adt12::C0(_, h702, _, _, _) => {
            h702
        }
    }, 0u64)?, f282(match v0.clone() {
        Adt12::C0(_, _, h703, _, _) => {
            h703
        }
    }, 0u64)?, f282(match v0 {
        Adt12::C0(_, _, _, h704, _) => {
            h704
        }
    }, 0u64)?))
}

pub fn f183(v0: Adt12) -> List<Bytes> {
    f66(match v0.clone() {
        Adt12::C0(h705, _, _, _, _) => {
            h705
        }
    }, f66(match v0.clone() {
        Adt12::C0(_, _, h706, _, _) => {
            h706
        }
    }, f66(match v0.clone() {
        Adt12::C0(_, _, _, h707, _) => {
            h707
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h708) => {
            h708
        }
    })))
}

pub fn f184(v0: Adt16, p714: List<u64>, p715: List<Adt20>) -> Option<u64> {
    let m713 = (p714, p715);
    {
        let mut m713 = m713;
        loop {
            let step = {
                let (v1, v2) = m713;
                let m709 = v2;
                match m709.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f283(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h710) => {
                                h710
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h711, _) => {
                                    h711
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h712, _) => {
                                    h712
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m713 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m716 = v1;
    match m716.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f284(v0.clone(), v3)?), f185(v0, v4, {
                let a717 = v2;
                let a718 = 1u64;
                nat_add(a717, a718)?
            })?))
        }
    }
}

pub fn f186(p722: List<Bytes>, p723: u64) -> Bytes {
    let m721 = (p722, p723);
    {
        let mut m721 = m721;
        loop {
            let step = {
                let (v0, v1) = m721;
                let m719 = v0;
                match m719.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m720 = v1;
                        if m720 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m720 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m721 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f285(v0.clone(), v1.clone()) {
        if f285(v1, v0) {
            f286(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f188(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m724 = v0;
    match m724.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f287(v2, v1.clone())?, f188(v3, v1)?))
        }
    }
}

pub fn f189(v0: List<Adt14>) -> List<u64> {
    let m725 = v0;
    match m725.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f288(v1), f189(v2))
        }
    }
}

pub fn f190(p728: List<u64>, v1: List<u64>) -> Option<u64> {
    let m727 = p728;
    {
        let mut m727 = m727;
        loop {
            let step = {
                let v0 = m727;
                let m726 = v0;
                match m726.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f289(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
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

pub fn f191(p731: List<Bytes>, p732: List<Bytes>) -> Option<Bytes> {
    let m730 = (p731, p732);
    {
        let mut m730 = m730;
        loop {
            let step = {
                let (v0, v1) = m730;
                let m729 = v0;
                match m729.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f90(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m730 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m733 = v0;
    match m733.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m734 = f290(v2, v1.clone())?;
            match m734 {
                None => {
                    f192(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f192(v3, v1)?))
                }
            }
        }
    }
}

pub fn f193(v0: List<Bytes>) -> List<Bytes> {
    f172(Fn1::F291, List::<Bytes>::nil(), v0)
}

pub fn f194(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f292(v0, v1)
}

pub fn f195(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f293(f294(f295(v0))?))
}

pub fn f196(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m735 = f163(v0, v2)?;
    match m735 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f296(v1, f166(v3.clone(), f8(Str::lit("id"))))?;
            let m736 = v4.clone();
            match m736.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m738 = f169(f168({
                        let m737 = f168(v3.clone(), f8(Str::lit("source")));
                        match m737 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m738 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f79(f90(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f221(f297(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f197(p741: List<Bytes>, p742: List<Bytes>) -> List<Bytes> {
    let m740 = (p741, p742);
    {
        let mut m740 = m740;
        loop {
            let step = {
                let (v0, v1) = m740;
                let m739 = v0;
                match m739.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m740 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f298(v0, v1)
}

pub fn f199() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f200(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m743 = v1;
    match m743.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt19 = f299(v0.clone(), v3)?;
            if f90(f300(f159(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f301(v5)?, f200(v0, v4, v2)?))
            } else {
                f200(v0, v4, v2)
            }
        }
    }
}

pub fn f201(v0: Adt1, p747: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m746 = p747;
    {
        let mut m746 = m746;
        loop {
            let step = {
                let v1 = m746;
                let m744 = v1;
                match m744.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt19 = f299(v0.clone(), v3.clone())?;
                        if if f90(f300(f159(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f37(f158({
                                let m745 = f156(v5.clone(), f8(Str::lit("schema")));
                                match m745 {
                                    None => {
                                        Adt19::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f74(v0.clone(), f301(v5)?))
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
                Ok(next) => { m746 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f79(f80(match v0.clone() {
        Adt1::C0(_, h748, _) => {
            h748
        }
    }, f147())) {
        Ok(Adt11::C1(List::cons(f147(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f88(f82(f302(f84(match v0.clone() {
            Adt1::C0(_, h749, _) => {
                h749
            }
        }, f147())))?, v1);
        let m750 = f62(v0.clone(), List::cons(f93(), List::cons(f122(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m750.uncons() {
            None => {
                let v5: (bool, Bytes) = f303(v0.clone(), v2)?;
                let (h751, _) = v5;
                if h751 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h752) = v5;
                        h752
                    }))
                } else {
                    let m753 = f92(v0.clone());
                    match m753.uncons() {
                        None => {
                            let m754 = f94(v0.clone())?;
                            match m754.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h755) = v5;
                                        h755
                                    }, List::cons(f7(f82(f304(f107(v0.clone())?, f115(v0.clone(), f116(v0.clone())?)?))?), List::cons(f305(v0)?, List::<Bytes>::nil()))))))
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

pub fn f203(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f306(f77(v0, f93())?)?)))
}

pub fn f204() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f205(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt23>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m756 = v1;
    match m756.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f66(f307(v5.clone(), v2.clone()), f308(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f309(v0.clone(), v5.clone()), List::cons(if f90(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a758 = {
                    let a757 = v7.clone();
                    length_list(a757)
                };
                let a759 = 0u64;
                if nat_eq(a758, a759) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f126(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f205(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f206(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt23>> {
    let v3: List<(Bytes, Bytes)> = f310(v1);
    let v4: List<(Bytes, Bytes)> = f311(v0, v2);
    f312(v3.clone(), f95(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f207(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m760 = v1;
    match m760.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f214(v2.clone(), f8(Str::lit("domain"))) {
                f313(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f166(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f166(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m761 = f168(v2.clone(), f8(Str::lit("role")));
                    match m761 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f166(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f166(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f166(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f314(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m762 = f168(v2.clone(), f8(Str::lit("origin")));
                    match m762 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f166(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m763 = f168(v2, f8(Str::lit("evidence")));
                    match m763 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f166(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f207(v0, v3))
            } else {
                f207(v0, v3)
            }
        }
    }
}

pub fn f208(v0: List<Adt14>) -> List<Bytes> {
    let m764 = v0;
    match m764.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f166(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f166(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f166(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f208(v2))
        }
    }
}

pub fn f209(v0: List<Adt14>) -> List<Bytes> {
    let m765 = v0;
    match m765.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f166(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f315(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f209(v2))
        }
    }
}

pub fn f210(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m766 = v0;
    match m766.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f166(v1.clone(), f8(Str::lit("subject"))), f166(v1, f8(Str::lit("object")))), f210(v2))
        }
    }
}

pub fn f211(v0: List<Adt23>) -> List<(Bytes, Bytes)> {
    let m767 = v0;
    match m767.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt23::C0(h768, _, _) => {
                    h768
                }
            }, match v1 {
                Adt23::C0(_, _, h769) => {
                    h769
                }
            }), f211(v2))
        }
    }
}

pub fn f212(p776: List<Bytes>, v1: List<(Bytes, Bytes)>, p777: u64) -> R<List<Bytes>> {
    let m775 = (p776, p777);
    {
        let mut m775 = m775;
        loop {
            let step = {
                let (v0, v2) = m775;
                let m770 = v2;
                if m770 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m770 - 1;
                    let v4: List<Bytes> = f82(f66(v0.clone(), f316(v0.clone(), v1.clone())))?;
                    let a773 = {
                        let a771 = v4.clone();
                        length_list(a771)
                    };
                    let a774 = {
                        let a772 = v0.clone();
                        length_list(a772)
                    };
                    if nat_eq(a773, a774) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m775 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: List<Adt14>) -> List<Bytes> {
    let m778 = v0;
    match m778.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m779 = f168(v1, f8(Str::lit("std")));
            match m779 {
                None => {
                    f213(v2)
                }
                Some(v3) => {
                    List::cons(f166(v3.clone(), f8(Str::lit("authority"))), if f214(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f166(v3, f8(Str::lit("row"))), f213(v2))
                    } else {
                        f213(v2)
                    })
                }
            }
        }
    }
}

pub fn f214(v0: Adt14, v1: Bytes) -> bool {
    f79({
        let a781 = {
            let a780 = f166(v0, v1);
            length_bytes(a780)
        };
        let a782 = 0u64;
        nat_eq(a781, a782)
    })
}

pub fn f215(v0: Adt15, v1: Adt14) -> bool {
    if f317(v0.clone(), v1.clone(), f166(v1.clone(), f8(Str::lit("subject")))) {
        f317(v0, v1.clone(), f166(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f216(v0: Bytes, v1: Bytes) -> bool {
    let m787 = {
        let a784 = v0;
        let a785 = 0u64;
        let a786 = {
            let a783 = v1.clone();
            length_bytes(a783)
        };
        slice_bytes(a784, a785, a786)
    };
    match m787 {
        None => {
            false
        }
        Some(v2) => {
            let a788 = v2;
            let a789 = v1;
            equal(a788, a789)
        }
    }
}

pub fn f217(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m790 = v1;
    match m790.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f143(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f139(v0.clone(), v2), f217(v0, v3))
            } else {
                f217(v0, v3)
            }
        }
    }
}

pub fn f218(v0: Bytes, p848: u64, p849: u64, p850: Adt17) -> R<Adt17> {
    let m847 = (p848, (p849, p850));
    {
        let mut m847 = m847;
        loop {
            let step = {
                let (v1, (v2, v3)) = m847;
                let m791 = v2;
                if m791 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m791 - 1;
                    let v5: u64 = f318(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h792, _, _, _) => {
                            h792
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a793 = v1;
                            let a794 = 1u64;
                            nat_add(a793, a794)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h795, _, _) => {
                                h795
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h796, _) => {
                                h796
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h797) => {
                                h797
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h798, _, _, _, _) => {
                                h798
                            }
                        } {
                            let a799 = v5;
                            let a800 = 34u64;
                            if nat_eq(a799, a800) {
                                let a803 = f318(v0.clone(), {
                                    let a801 = v1;
                                    let a802 = 1u64;
                                    nat_add(a801, a802)?
                                })?;
                                let a804 = 34u64;
                                if nat_eq(a803, a804) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a805 = v1;
                                        let a806 = 1u64;
                                        nat_add(a805, a806)?
                                    }, (v4, Adt17::C0(true, true, f319(match v3.clone() {
                                        Adt17::C0(_, _, h807, _, _) => {
                                            h807
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h808, _) => {
                                            h808
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h809) => {
                                            h809
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a810 = v1;
                                        let a811 = 1u64;
                                        nat_add(a810, a811)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h812, _, _) => {
                                            h812
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h813, _) => {
                                            h813
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h814) => {
                                            h814
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a815 = v1;
                                    let a816 = 1u64;
                                    nat_add(a815, a816)?
                                }, (v4, Adt17::C0(true, false, f319(match v3.clone() {
                                    Adt17::C0(_, _, h817, _, _) => {
                                        h817
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h818, _) => {
                                        h818
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h819) => {
                                        h819
                                    }
                                }))))
                            }
                        } else {
                            let a820 = v5;
                            let a821 = 34u64;
                            if nat_eq(a820, a821) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a822 = v1;
                                    let a823 = 1u64;
                                    nat_add(a822, a823)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h824, _, _) => {
                                        h824
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h825, _) => {
                                        h825
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h826) => {
                                        h826
                                    }
                                }))))
                            } else {
                                let a827 = v5;
                                let a828 = 44u64;
                                if nat_eq(a827, a828) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a829 = v1;
                                        let a830 = 1u64;
                                        nat_add(a829, a830)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h831, _, _) => {
                                            h831
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h832, _) => {
                                            h832
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h833) => {
                                            h833
                                        }
                                    }))))
                                } else {
                                    let a834 = v5;
                                    let a835 = 13u64;
                                    if nat_eq(a834, a835) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a836 = v1;
                                            let a837 = 1u64;
                                            nat_add(a836, a837)?
                                        }, (v4, v3)))
                                    } else {
                                        let a838 = v5;
                                        let a839 = 10u64;
                                        if nat_eq(a838, a839) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a840 = v1;
                                                let a841 = 1u64;
                                                nat_add(a840, a841)?
                                            }, (v4, f219(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a842 = v1;
                                                let a843 = 1u64;
                                                nat_add(a842, a843)?
                                            }, (v4, Adt17::C0(false, false, f319(match v3.clone() {
                                                Adt17::C0(_, _, h844, _, _) => {
                                                    h844
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h845, _) => {
                                                    h845
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h846) => {
                                                    h846
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
                Ok(next) => { m847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f320(List::cons(match v0.clone() {
        Adt17::C0(_, _, h851, _, _) => {
            h851
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h852, _) => {
            h852
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h853) => {
            h853
        }
    }))
}

pub fn f220(p856: List<List<Bytes>>, p857: List<List<Bytes>>) -> List<List<Bytes>> {
    let m855 = (p856, p857);
    {
        let mut m855 = m855;
        loop {
            let step = {
                let (v0, v1) = m855;
                let m854 = v0;
                match m854.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m855 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f321(v0.clone(), 0u64, {
        let a858 = v0;
        length_bytes(a858)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f222(v0: Adt24) -> Bytes {
    let a879 = f322(match v0.clone() {
        Adt24::C0(h859, _, _, _, _, _, _, _) => {
            h859
        }
    });
    let a880 = {
        let a877 = f322(match v0.clone() {
            Adt24::C0(_, h860, _, _, _, _, _, _) => {
                h860
            }
        });
        let a878 = {
            let a875 = f322(match v0.clone() {
                Adt24::C0(_, _, h861, _, _, _, _, _) => {
                    h861
                }
            });
            let a876 = {
                let a873 = f322(match v0.clone() {
                    Adt24::C0(_, _, _, h862, _, _, _, _) => {
                        h862
                    }
                });
                let a874 = {
                    let a871 = f322(match v0.clone() {
                        Adt24::C0(_, _, _, _, h863, _, _, _) => {
                            h863
                        }
                    });
                    let a872 = {
                        let a869 = f322(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h864, _, _) => {
                                h864
                            }
                        });
                        let a870 = {
                            let a867 = f322(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h865, _) => {
                                    h865
                                }
                            });
                            let a868 = f322(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h866) => {
                                    h866
                                }
                            });
                            append_bytes(a867, a868)
                        };
                        append_bytes(a869, a870)
                    };
                    append_bytes(a871, a872)
                };
                append_bytes(a873, a874)
            };
            append_bytes(a875, a876)
        };
        append_bytes(a877, a878)
    };
    append_bytes(a879, a880)
}

pub fn f223(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a881 = v0.clone();
        length_bytes(a881)
    };
    let v2: u64 = {
        let a887 = {
            let a884 = {
                let a882 = v1;
                let a883 = 72u64;
                nat_add(a882, a883)?
            };
            let a885 = 64u64;
            let a886 = 0u64;
            nat_quot(a884, a885, a886)
        };
        let a888 = 64u64;
        nat_mul(a887, a888)?
    };
    let (_, h889) = f323(Fn4::F324(v0, v1, v2), {
        let a890 = v2;
        let a891 = 64u64;
        let a892 = 0u64;
        nat_quot(a890, a891, a892)
    }, (0u64, f325()))?;
    Ok(h889)
}

pub fn f224(p895: List<List<Bytes>>, v1: Bytes, p896: List<Bytes>) -> R<List<Bytes>> {
    let m894 = (p895, p896);
    {
        let mut m894 = m894;
        loop {
            let step = {
                let (v0, v2) = m894;
                let m893 = v0;
                match m893.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f326(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f226(f327(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f328(v0)?))
}

pub fn f226(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m897 = v0;
    match m897.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m898 = v3.clone();
            match m898.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a901 = v2;
                    let a902 = {
                        let a899 = v1.clone();
                        let a900 = f226(v3, v1);
                        append_bytes(a899, a900)
                    };
                    append_bytes(a901, a902)
                }
            }
        }
    }
}

pub fn f227(v0: Adt14) -> R<Bytes> {
    let a903 = f329(v0)?;
    let a904 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a903, a904))
}

pub fn f228(v0: List<Bytes>) -> Adt14 {
    let m905 = v0;
    match m905.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f228(v2)))
        }
    }
}

pub fn f229(v0: Fn2, p909: Option<Bytes>, p910: List<Bytes>) -> Option<Bytes> {
    let m908 = (p909, p910);
    {
        let mut m908 = m908;
        loop {
            let step = {
                let (v1, v2) = m908;
                let m906 = v2;
                match m906.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c907: Fn2 = v0.clone();
                            c907.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m911 = v1;
    match m911 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a914 = {
                    let a912 = v3;
                    let a913 = v0;
                    append_bytes(a912, a913)
                };
                let a915 = v2;
                append_bytes(a914, a915)
            })
        }
    }
}

pub fn f231(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m920 = {
        let a917 = v0;
        let a918 = v2;
        let a919 = {
            let a916 = v1.clone();
            length_bytes(a916)
        };
        slice_bytes(a917, a918, a919)
    };
    match m920 {
        None => {
            false
        }
        Some(v3) => {
            let a921 = v3;
            let a922 = v1;
            equal(a921, a922)
        }
    }
}

pub fn f232(v0: Bytes) -> R<Bytes> {
    let (_, h923) = {
        let (h924, _) = f330(Fn5::F331(v0.clone()), {
            let a926 = {
                let a925 = v0;
                length_bytes(a925)
            };
            let a927 = 1u64;
            nat_add(a926, a927)?
        }, (0u64, Bytes::lit(&[])))?;
        h924
    };
    Ok(h923)
}

pub fn f233(v0: Bytes) -> R<u64> {
    let (h928, _) = f280(Fn3::F332(v0.clone()), {
        let a930 = {
            let a929 = v0;
            length_bytes(a929)
        };
        let a931 = 1u64;
        nat_add(a930, a931)?
    }, 0u64)?;
    Ok(h928)
}

pub fn f234(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a932 = v0;
        utf8_encode(a932)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f131(v1)?, List::<Bytes>::nil())))))
}

pub fn f235(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f237(v0.clone(), 0u64, Str::lit("<?xml")) {
        f333(f238(v0.clone(), 5u64)?)
    } else {
        false
    } {
        let m934 = f334(v0.clone(), {
            let a933 = Str::lit("?>");
            utf8_encode(a933)
        }, 5u64)?;
        match m934 {
            None => {
                Ok(Err::<u64, Bytes>(f234(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m936 = f334(f32(v0.clone(), 0u64, v1), {
                    let a935 = Str::lit("encoding");
                    utf8_encode(a935)
                }, 5u64)?;
                match m936 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a937 = v1;
                            let a938 = 2u64;
                            nat_add(a937, a938)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f335(v0.clone(), {
                            let a939 = v2;
                            let a940 = 8u64;
                            nat_add(a939, a940)?
                        })?;
                        let v4: u64 = f335(v0.clone(), {
                            let a941 = v3;
                            let a942 = 1u64;
                            nat_add(a941, a942)?
                        })?;
                        let _: u64 = f238(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m945 = f334(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a943 = v4;
                                let a944 = 1u64;
                                nat_add(a943, a944)?
                            })?;
                            match m945 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f336(f32(v0, {
                            let a946 = v4;
                            let a947 = 1u64;
                            nat_add(a946, a947)?
                        }, {
                            let a950 = v7;
                            let a951 = {
                                let a948 = v4;
                                let a949 = 1u64;
                                nat_add(a948, a949)?
                            };
                            nat_sub(a950, a951)
                        }))? {
                            Ok(Ok::<u64, Bytes>({
                                let a952 = v1;
                                let a953 = 2u64;
                                nat_add(a952, a953)?
                            }))
                        } else {
                            Ok(Err::<u64, Bytes>(f234(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f236(v0: Bytes, p958: u64, p959: u64) -> R<Result<u64, Bytes>> {
    let m957 = (p958, p959);
    {
        let mut m957 = m957;
        loop {
            let step = {
                let (v1, v2) = m957;
                let m954 = v2;
                if m954 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(f234(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m954 - 1;
                    let v4: u64 = f335(v0.clone(), v1)?;
                    if f237(v0.clone(), v4, Str::lit("<!--")) {
                        let m955 = f337(v0.clone(), v4)?;
                        match m955 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v5)))
                            }
                        }
                    } else {
                        if f237(v0.clone(), v4, Str::lit("<?")) {
                            let m956 = f338(v0.clone(), v4)?;
                            match m956 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m957 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Bytes, v1: u64, v2: Str) -> bool {
    f231(v0, {
        let a960 = v2;
        utf8_encode(a960)
    }, v1)
}

pub fn f238(v0: Bytes, v1: u64) -> R<u64> {
    f339(v0, v1)
}

pub fn f239(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt18, u64), Bytes>> {
    let m961 = v2;
    if m961 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m961 - 1;
        let a962 = v3;
        let a963 = 0u64;
        if nat_eq(a962, a963) {
            let v5: u64 = f340(v0.clone(), {
                let a964 = v1;
                let a965 = 1u64;
                nat_add(a964, a965)?
            })?;
            let a968 = v5;
            let a969 = {
                let a966 = v1;
                let a967 = 1u64;
                nat_add(a966, a967)?
            };
            if nat_eq(a968, a969) {
                Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a970 = v1;
                    let a971 = 1u64;
                    nat_add(a970, a971)?
                }, {
                    let a974 = v5;
                    let a975 = {
                        let a972 = v1;
                        let a973 = 1u64;
                        nat_add(a972, a973)?
                    };
                    nat_sub(a974, a975)
                });
                let m976 = f341(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m976 {
                    Ok(v8) => {
                        if f237(v0.clone(), {
                            let (_, h977) = v8.clone();
                            h977
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                let (h978, _) = v8;
                                h978
                            }), std::rc::Rc::new(Adt18::C4)), {
                                let a980 = {
                                    let (_, h979) = v8;
                                    h979
                                };
                                let a981 = 2u64;
                                nat_add(a980, a981)?
                            })))
                        } else {
                            let m985 = f239(v0.clone(), {
                                let a983 = {
                                    let (_, h982) = v8;
                                    h982
                                };
                                let a984 = 1u64;
                                nat_add(a983, a984)?
                            }, v4, 1u64)?;
                            match m985 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a987 = {
                                            let (_, h986) = v10.clone();
                                            h986
                                        };
                                        let a988 = 2u64;
                                        nat_add(a987, a988)?
                                    };
                                    let v12: u64 = f340(v0.clone(), v11)?;
                                    if !f37(f32(v0.clone(), v11, {
                                        let a989 = v12;
                                        let a990 = v11;
                                        nat_sub(a989, a990)
                                    }), v6.clone()) {
                                        Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h991) = v10;
                                            h991
                                        })?))
                                    } else {
                                        let v13: u64 = f335(v0.clone(), v12)?;
                                        let a992 = f238(v0, v13)?;
                                        let a993 = 62u64;
                                        if nat_eq(a992, a993) {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                                let (h994, _) = v8;
                                                h994
                                            }), std::rc::Rc::new({
                                                let (h995, _) = v10;
                                                h995
                                            })), {
                                                let a996 = v13;
                                                let a997 = 1u64;
                                                nat_add(a996, a997)?
                                            })))
                                        } else {
                                            Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<(Adt18, u64), Bytes>(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<(Adt18, u64), Bytes>(v7))
                    }
                }
            }
        } else {
            let a998 = f238(v0.clone(), v1)?;
            let a999 = 256u64;
            if nat_eq(a998, a999) {
                Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f237(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v1)))
                } else {
                    if f237(v0.clone(), v1, Str::lit("<!--")) {
                        let m1000 = f337(v0.clone(), v1)?;
                        match m1000 {
                            Ok(v15) => {
                                f239(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(Err::<(Adt18, u64), Bytes>(v14))
                            }
                        }
                    } else {
                        if f237(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m1004 = f334(v0.clone(), {
                                let a1001 = Str::lit("]]>");
                                utf8_encode(a1001)
                            }, {
                                let a1002 = v1;
                                let a1003 = 9u64;
                                nat_add(a1002, a1003)?
                            })?;
                            match m1004 {
                                None => {
                                    Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m1007 = f239(v0.clone(), {
                                        let a1005 = v16;
                                        let a1006 = 3u64;
                                        nat_add(a1005, a1006)?
                                    }, v4, 1u64)?;
                                    match m1007 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2(f32(v0, {
                                                let a1008 = v1;
                                                let a1009 = 9u64;
                                                nat_add(a1008, a1009)?
                                            }, {
                                                let a1012 = v16;
                                                let a1013 = {
                                                    let a1010 = v1;
                                                    let a1011 = 9u64;
                                                    nat_add(a1010, a1011)?
                                                };
                                                nat_sub(a1012, a1013)
                                            }))), std::rc::Rc::new({
                                                let (h1014, _) = v18;
                                                h1014
                                            })), {
                                                let (_, h1015) = v18;
                                                h1015
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(Err::<(Adt18, u64), Bytes>(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f237(v0.clone(), v1, Str::lit("<?")) {
                                let m1016 = f338(v0.clone(), v1)?;
                                match m1016 {
                                    Ok(v20) => {
                                        f239(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v19))
                                    }
                                }
                            } else {
                                if f237(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a1017 = f238(v0.clone(), v1)?;
                                    let a1018 = 60u64;
                                    if nat_eq(a1017, a1018) {
                                        let m1019 = f239(v0.clone(), v1, v4, 0u64)?;
                                        match m1019 {
                                            Ok(v22) => {
                                                let m1021 = f239(v0, {
                                                    let (_, h1020) = v22;
                                                    h1020
                                                }, v4, 1u64)?;
                                                match m1021 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new({
                                                            let (h1022, _) = v22;
                                                            h1022
                                                        }), std::rc::Rc::new({
                                                            let (h1023, _) = v24;
                                                            h1023
                                                        })), {
                                                            let (_, h1024) = v24;
                                                            h1024
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<(Adt18, u64), Bytes>(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(Err::<(Adt18, u64), Bytes>(v21))
                                            }
                                        }
                                    } else {
                                        let m1025 = f342(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m1025 {
                                            Ok(v26) => {
                                                let m1027 = f239(v0, {
                                                    let (_, h1026) = v26;
                                                    h1026
                                                }, v4, 1u64)?;
                                                match m1027 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2({
                                                            let (h1028, _) = v26;
                                                            h1028
                                                        })), std::rc::Rc::new({
                                                            let (h1029, _) = v28;
                                                            h1029
                                                        })), {
                                                            let (_, h1030) = v28;
                                                            h1030
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(Err::<(Adt18, u64), Bytes>(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(Err::<(Adt18, u64), Bytes>(v25))
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

pub fn f240(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1031 = v2;
    let a1032 = 0u64;
    if nat_eq(a1031, a1032) {
        Ok(f343(v0, 0u64))
    } else {
        let (_, h1033) = {
            let (h1034, _) = f330(Fn5::F344(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1034
        };
        Ok(h1033)
    }
}

pub fn f241() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f242(v0: Adt18, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m1035 = v0;
    match m1035 {
        Adt18::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C1(v2, v3, r1040) => {
            let v4 = (*r1040).clone();
            let m1036 = f242(v4, v1);
            match m1036 {
                Ok(v6) => {
                    if f37(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f37(v3.clone(), f345()) {
                            true
                        } else {
                            f37(v3.clone(), f346())
                        } {
                            Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f216(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f32(v2.clone(), 6u64, {
                                let a1038 = {
                                    let a1037 = v2;
                                    length_bytes(a1037)
                                };
                                let a1039 = 6u64;
                                nat_sub(a1038, a1039)
                            });
                            if f37(v7.clone(), f8(Str::lit("xmlns"))) {
                                Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f37(v7.clone(), f8(Str::lit("xml"))) {
                                    !f37(v3.clone(), f345())
                                } else {
                                    false
                                } {
                                    Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f37(v3.clone(), f345()) {
                                        true
                                    } else {
                                        f37(v3.clone(), f346())
                                    } {
                                        !f37(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f37(v3.clone(), Bytes::lit(&[])) {
                                            Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
                                        } else {
                                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((v7, v3), v6))
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(v6)
                        }
                    }
                }
                Err(v5) => {
                    Err::<List<(Bytes, Bytes)>, Bytes>(v5)
                }
            }
        }
        Adt18::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f243(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f347(v1.clone())?;
    if f37(v2.clone(), Bytes::lit(&[])) {
        let m1041 = f348(v0, Bytes::lit(&[]));
        match m1041 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f37(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f345(), f349(v1)?)))
        } else {
            let m1042 = f348(v0, v2.clone());
            match m1042 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f349(v1)?)))
                }
            }
        }
    }
}

pub fn f244(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f37({
        let (h1043, _) = v0;
        h1043
    }, f350()) {
        f37({
            let (_, h1044) = v0;
            h1044
        }, {
            let a1045 = v1;
            utf8_encode(a1045)
        })
    } else {
        false
    }
}

pub fn f245(p1049: Adt18, v1: Bytes) -> Option<Bytes> {
    let m1048 = p1049;
    {
        let mut m1048 = m1048;
        loop {
            let step = {
                let v0 = m1048;
                let m1046 = v0;
                match m1046 {
                    Adt18::C0(_, _, _) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C1(v2, v3, r1047) => {
                        let v4 = (*r1047).clone();
                        if f37(v2, v1.clone()) {
                            Err::<Adt18, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt18, Option<Bytes>>(v4)
                        }
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C3(_, _) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C4 => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                }
            };
            match step {
                Ok(next) => { m1048 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: Adt18, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m1050 = v0;
    match m1050 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C3(r1061, r1062) => {
            let v3 = (*r1061).clone();
            let v4 = (*r1062).clone();
            let m1051 = f246(v4, v1.clone(), v2.clone())?;
            match m1051 {
                Ok(v6) => {
                    let m1052 = v3;
                    match m1052 {
                        Adt18::C0(v7, r1059, r1060) => {
                            let v8 = (*r1059).clone();
                            let v9 = (*r1060).clone();
                            let m1053 = f242(v8.clone(), v1);
                            match m1053 {
                                Ok(v11) => {
                                    let m1054 = f243(v11.clone(), v7)?;
                                    match m1054 {
                                        Ok(v13) => {
                                            if f244(v13, Str::lit("simpleType")) {
                                                let m1055 = f245(v8, f8(Str::lit("name")));
                                                match m1055 {
                                                    None => {
                                                        Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m1056 = f351(v14.clone(), v9, v11, v2)?;
                                                        match m1056 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f265(v14)?, ({
                                                                    let (_, h1057) = v16;
                                                                    h1057
                                                                }, {
                                                                    let (h1058, _) = v16;
                                                                    h1058
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v10))
                                }
                            }
                        }
                        Adt18::C1(_, _, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C2(_) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C3(_, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C4 => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v5))
                }
            }
        }
        Adt18::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f247(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m1063 = v0;
    match m1063.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1064, _) = v1;
                h1064
            }, f247(v2))
        }
    }
}

pub fn f248(p1067: List<Bytes>) -> Option<Bytes> {
    let m1066 = p1067;
    {
        let mut m1066 = m1066;
        loop {
            let step = {
                let v0 = m1066;
                let m1065 = v0;
                match m1065.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f90(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f249(p1073: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m1072 = p1073;
    {
        let mut m1072 = m1072;
        loop {
            let step = {
                let v0 = m1072;
                let m1068 = v0;
                match m1068.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1071 = f352({
                            let (h1069, _) = {
                                let (_, h1070) = v2;
                                h1070
                            };
                            h1069
                        }, v1.clone());
                        match m1071 {
                            None => {
                                Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(Some(v4))
                            }
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

pub fn f250(p1084: u64, p1085: List<(Bytes, (List<Bytes>, Bytes))>, p1086: List<Bytes>, p1087: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1083 = (p1084, (p1085, (p1086, p1087)));
    {
        let mut m1083 = m1083;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1083;
                let m1074 = v0;
                if m1074 == 0 {
                    let m1075 = v1;
                    match m1075.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m1074 - 1;
                    let m1076 = v1.clone();
                    match m1076.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m1077 = f353(v1, v2.clone());
                            match m1077 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h1078) = v9.clone();
                                        h1078
                                    }, (List::cons({
                                        let (h1079, _) = {
                                            let (h1080, _) = v9.clone();
                                            h1080
                                        };
                                        h1079
                                    }, v2), List::cons({
                                        let (_, h1081) = {
                                            let (h1082, _) = v9;
                                            h1082
                                        };
                                        h1081
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(p1090: List<Bytes>, p1091: List<Bytes>) -> List<Bytes> {
    let m1089 = (p1090, p1091);
    {
        let mut m1089 = m1089;
        loop {
            let step = {
                let (v0, v1) = m1089;
                let m1088 = v0;
                match m1088.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1089 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f328(v0.clone())?;
    let v2: u64 = f354(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a1092 = v1;
        let a1093 = v2;
        nat_sub(a1092, a1093)
    }))
}

pub fn f253(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f355(v0)?;
    let m1097 = f356(v1.clone(), 0u64, {
        let a1095 = {
            let a1094 = v1.clone();
            length_bytes(a1094)
        };
        let a1096 = 2u64;
        nat_add(a1095, a1096)?
    }, 0u64)?;
    match m1097 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a1100 = {
                let (_, h1098) = v2;
                h1098
            };
            let a1101 = {
                let a1099 = v1;
                length_bytes(a1099)
            };
            if nat_eq(a1100, a1101) {
                Ok(Some({
                    let (h1102, _) = v2;
                    h1102
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f254(v0: Bytes, v1: Bytes) -> bool {
    let a1103 = v0;
    let a1104 = f274();
    if equal(a1103, a1104) {
        let a1108 = {
            let a1106 = {
                let a1105 = v1;
                length_bytes(a1105)
            };
            let a1107 = 0u64;
            nat_eq(a1106, a1107)
        };
        bool_not(a1108)
    } else {
        false
    }
}

pub fn f255(v0: Bytes) -> Bytes {
    f7(List::cons(f81(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f256(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1110 = v0;
    let a1111 = {
        let a1109 = Str::lit("redistributable");
        utf8_encode(a1109)
    };
    if equal(a1110, a1111) {
        f357(v1)
    } else {
        Ok(false)
    }
}

pub fn f257(v0: Adt14) -> bool {
    if f214(v0.clone(), f8(Str::lit("domain"))) {
        if f214(v0.clone(), f8(Str::lit("row"))) {
            if f90(f166(v0.clone(), f8(Str::lit("relation"))), f358()) {
                if f359(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f359(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f359(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f359(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f258(v0: Adt14) -> bool {
    if f214(v0.clone(), f8(Str::lit("subject"))) {
        if f214(v0.clone(), f8(Str::lit("object"))) {
            if f214(v0.clone(), f8(Str::lit("evidence"))) {
                f90(f166(v0, f8(Str::lit("relation"))), f360())
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

pub fn f259(v0: Option<List<Adt14>>) -> bool {
    let m1112 = v0;
    match m1112 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f260(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1113 = v0;
    match m1113 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f261(v0: Adt14) -> Bytes {
    if f214(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f214(v0.clone(), f8(Str::lit("id"))) {
            if f214(v0.clone(), f8(Str::lit("statement"))) {
                if f214(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f271(v0, f8(Str::lit("evidence")))
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

pub fn f262(v0: List<Adt14>) -> Adt14 {
    let m1114 = v0;
    match m1114.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f262(v2)))
        }
    }
}

pub fn f263(v0: Bytes) -> R<Option<Adt19>> {
    let v1: Bytes = f355(v0)?;
    let m1118 = f361(v1.clone(), 0u64, {
        let a1116 = {
            let a1115 = v1.clone();
            length_bytes(a1115)
        };
        let a1117 = 2u64;
        nat_add(a1116, a1117)?
    }, 0u64)?;
    match m1118 {
        None => {
            Ok(None::<Adt19>)
        }
        Some(v2) => {
            let a1121 = {
                let (_, h1119) = v2;
                h1119
            };
            let a1122 = {
                let a1120 = v1;
                length_bytes(a1120)
            };
            if nat_eq(a1121, a1122) {
                Ok(Some({
                    let (h1123, _) = v2;
                    h1123
                }))
            } else {
                Ok(None::<Adt19>)
            }
        }
    }
}

pub fn f264(v0: Adt19) -> bool {
    if f362(v0.clone(), f8(Str::lit("name"))) {
        if f363(v0.clone(), f8(Str::lit("schema"))) {
            let m1124 = f266(f160(f161(f156(v0, f8(Str::lit("citation"))))));
            match m1124 {
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

pub fn f265(v0: Bytes) -> R<Bytes> {
    f364(v0.clone(), 0u64, {
        let a1125 = v0;
        length_bytes(a1125)
    }, true, Bytes::lit(&[]))
}

pub fn f266(v0: List<Bytes>) -> Option<Adt14> {
    let m1126 = v0;
    match m1126.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f37(v1.clone(), f8(Str::lit("req"))) {
                let m1127 = v2;
                match m1127.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f37(v1.clone(), f8(Str::lit("std"))) {
                    let m1128 = v2;
                    match m1128.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1129 = v6;
                                match m1129.uncons() {
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
                    if f37(v1, f8(Str::lit("der"))) {
                        let m1130 = v2;
                        match m1130.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1131 = v10;
                                match m1131.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f228(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f267(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1132 = f62(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1132.uncons() {
        None => {
            let m1133 = f266(v4);
            match m1133 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1134 = f8(Str::lit("Catalog."));
                        let a1135 = f265(v3.clone())?;
                        append_bytes(a1134, a1135)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f265(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1136 = f365(v2, v9.clone(), f265(v3.clone())?)?;
                    match m1136 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f265(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f227(Adt14::C3(std::rc::Rc::new(f262(f366(f77(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f227(Adt14::C3(std::rc::Rc::new(f262(f366(f77(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1140 = f367(f74(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a1138 = {
                                        let a1137 = v10;
                                        length_bytes(a1137)
                                    };
                                    let a1139 = 4u64;
                                    nat_sub(a1138, a1139)
                                }))?;
                                let a1141 = Bytes::lit(&[0x0a]);
                                append_bytes(a1140, a1141)
                            }), List::cons(f125(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f268(v0: Adt19) -> R<Bytes> {
    let m1142 = v0;
    match m1142 {
        Adt19::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt19::C1(v2, v3) => {
            f368(v2, v3)
        }
        Adt19::C2(v4, v5, v6) => {
            f369(v4, v5, v6)
        }
        Adt19::C3(v7) => {
            f370(v7)
        }
        Adt19::C4(r1143) => {
            let v8 = (*r1143).clone();
            Ok(f371(f372(v8)?))
        }
        Adt19::C5(r1144) => {
            let v9 = (*r1144).clone();
            f373(f374(v9)?)
        }
        Adt19::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f269(v0: Option<Adt19>) -> Option<Bytes> {
    let m1145 = v0;
    match m1145 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1146 = v1;
            match m1146 {
                Adt19::C0(_) => {
                    None::<Bytes>
                }
                Adt19::C1(_, _) => {
                    None::<Bytes>
                }
                Adt19::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt19::C3(v2) => {
                    Some(v2)
                }
                Adt19::C4(_) => {
                    None::<Bytes>
                }
                Adt19::C5(_) => {
                    None::<Bytes>
                }
                Adt19::C6 => {
                    None::<Bytes>
                }
                Adt19::C7(_, _) => {
                    None::<Bytes>
                }
                Adt19::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f270(v0: Adt19) -> List<Adt19> {
    let m1147 = v0;
    match m1147 {
        Adt19::C0(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C1(_, _) => {
            List::<Adt19>::nil()
        }
        Adt19::C2(_, _, _) => {
            List::<Adt19>::nil()
        }
        Adt19::C3(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C4(r1148) => {
            let v3 = (*r1148).clone();
            f270(v3)
        }
        Adt19::C5(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C6 => {
            List::<Adt19>::nil()
        }
        Adt19::C7(r1149, r1150) => {
            let v1 = (*r1149).clone();
            let v2 = (*r1150).clone();
            List::cons(v1, f270(v2))
        }
        Adt19::C8(_, _, _) => {
            List::<Adt19>::nil()
        }
    }
}

pub fn f271(v0: Adt14, v1: Bytes) -> bool {
    let m1151 = f168(v0, v1);
    match m1151 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f272(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1152 = f168(v0, v1);
    match m1152 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f165(f168(v3, v2))
        }
    }
}

pub fn f273(v0: Adt14) -> List<Adt14> {
    let m1153 = v0;
    match m1153 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1154) => {
            let v3 = (*r1154).clone();
            f273(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1155, r1156) => {
            let v1 = (*r1155).clone();
            let v2 = (*r1156).clone();
            List::cons(v1, f273(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f274() -> Bytes {
    let a1157 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1157)
}

pub fn f275(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f168(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f121(Str::lit("id"), f166(v0.clone(), f8(Str::lit("id"))))?, List::cons(f121(Str::lit("name"), f166(v0.clone(), f8(Str::lit("name"))))?, List::cons(f121(Str::lit("issuer"), f166(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f121(Str::lit("edition"), f166(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f121(Str::lit("canonical_id"), f166(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f121(Str::lit("source_role"), f166(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f121(Str::lit("retrieval_date"), f166(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f121(Str::lit("url"), f375(v2.clone(), f8(Str::lit("url"))))?, List::cons(f121(Str::lit("media_type"), f375(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f121(Str::lit("license"), f375(v2.clone(), f8(Str::lit("license"))))?, List::cons(f121(Str::lit("redistribution"), f375(v2, f8(Str::lit("redistribution"))))?, List::cons(f121(Str::lit("statement"), f166(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1158 = v1;
        match m1158 {
            None => {
                f7(List::cons(f121(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f121(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f121(Str::lit("artifact"), {
                    let (h1159, _) = v3;
                    h1159
                })?, List::cons(f121(Str::lit("checksum"), {
                    let (_, h1160) = v3;
                    h1160
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f376(f78(f165(f168(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f276(v0: Bytes, v1: u64) -> Option<u8> {
    let a1161 = v0;
    let a1162 = v1;
    index_bytes(a1161, a1162)
}

pub fn f277(v0: u64) -> bool {
    let a1163 = v0;
    let a1164 = 127u64;
    nat_le(a1163, a1164)
}

pub fn f278(v0: Bytes) -> bool {
    f143(v0, f8(Str::lit(".lex.tex")))
}

pub fn f279(v0: Bytes) -> R<bool> {
    let v1: Bytes = f252(v0)?;
    let v2: u64 = if f216(v1.clone(), f8(Str::lit("if ! "))) {
        5u64
    } else {
        if f216(v1.clone(), f8(Str::lit("elif ! "))) {
            7u64
        } else {
            0u64
        }
    };
    let v3: Bytes = f32(v1.clone(), v2, {
        let a1166 = {
            let a1165 = v1;
            length_bytes(a1165)
        };
        let a1167 = v2;
        nat_sub(a1166, a1167)
    });
    let v5: Bytes = if f216(v3.clone(), f8(Str::lit("("))) {
        let m1168 = f180(v3.clone(), f8(Str::lit(")")))?;
        match m1168 {
            None => {
                v3
            }
            Some(v4) => {
                f32(v3.clone(), {
                    let a1169 = v4;
                    let a1170 = 1u64;
                    nat_add(a1169, a1170)?
                }, {
                    let a1174 = {
                        let a1171 = v3;
                        length_bytes(a1171)
                    };
                    let a1175 = {
                        let a1172 = v4;
                        let a1173 = 1u64;
                        nat_add(a1172, a1173)?
                    };
                    nat_sub(a1174, a1175)
                })
            }
        }
    } else {
        v3
    };
    let a1176 = 0u64;
    let a1177 = v2;
    if nat_lt(a1176, a1177) {
        f377(v5)
    } else {
        Ok(false)
    }
}

pub fn f280(v0: Fn3, p1182: u64, p1183: u64) -> R<(u64, bool)> {
    let m1181 = (p1182, p1183);
    {
        let mut m1181 = m1181;
        loop {
            let step = {
                let (v1, v2) = m1181;
                let m1178 = v1;
                if m1178 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1178 - 1;
                    let m1180 = {
                        let c1179: Fn3 = v0.clone();
                        c1179.apply(v2)?
                    };
                    match m1180 {
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
                Ok(next) => { m1181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f231(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1186 = {
            let a1184 = v4;
            let a1185 = v1;
            nat_add(a1184, a1185)?
        };
        let a1187 = v3;
        if nat_lt(a1186, a1187) {
            Ok(Some({
                let a1188 = v4;
                let a1189 = 1u64;
                nat_add(a1188, a1189)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f282(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1190 = v0;
    match m1190.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f282(v3, {
                let a1191 = v1;
                let a1192 = 1u64;
                nat_add(a1191, a1192)?
            })?))
        }
    }
}

pub fn f283(v0: Adt16, v1: List<u64>, v2: Adt21) -> bool {
    let m1193 = v2;
    match m1193 {
        Adt21::C0(v3, v4) => {
            if f289(v3, match v0.clone() {
                Adt16::C0(h1194, _, _, _) => {
                    h1194
                }
            }) {
                f378(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f289(v5, match v0 {
                Adt16::C0(_, _, h1195, _) => {
                    h1195
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f285(v7, v1) {
                f289(v8, match v0 {
                    Adt16::C0(_, _, _, h1196) => {
                        h1196
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f284(v0: Adt12, v1: Adt14) -> R<Adt21> {
    let m1197 = f168(v1.clone(), f8(Str::lit("std")));
    match m1197 {
        None => {
            let m1202 = f169(f168(v1.clone(), f8(Str::lit("req"))));
            match m1202 {
                None => {
                    let m1204 = f168(v1, f8(Str::lit("der")));
                    match m1204 {
                        None => {
                            Ok(f379(v0))
                        }
                        Some(v5) => {
                            let m1205 = f380(f166(v5.clone(), f8(Str::lit("rule"))));
                            match m1205 {
                                None => {
                                    Ok(f379(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f188(f78(f165(f168(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h1206) => {
                                            h1206
                                        }
                                    })?, f287(f166(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h1207, _) => {
                                            h1207
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f287(v4, match v0 {
                        Adt12::C0(_, _, h1203, _, _) => {
                            h1203
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1198 = f169(f168(v2.clone(), f8(Str::lit("row"))));
            match m1198 {
                None => {
                    Ok(Adt21::C0(f287(f166(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h1199, _, _, _, _) => {
                            h1199
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f287(f166(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h1200, _, _, _, _) => {
                            h1200
                        }
                    })?, Some(f287(v3, match v0 {
                        Adt12::C0(_, h1201, _, _, _) => {
                            h1201
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f285(p1210: List<u64>, v1: List<u64>) -> bool {
    let m1209 = p1210;
    {
        let mut m1209 = m1209;
        loop {
            let step = {
                let v0 = m1209;
                let m1208 = v0;
                match m1208.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f289(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f286(p1215: List<u64>) -> bool {
    let m1214 = p1215;
    {
        let mut m1214 = m1214;
        loop {
            let step = {
                let v0 = m1214;
                let m1211 = v0;
                match m1211.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1212 = v1;
                        let a1213 = 7u64;
                        if nat_le(a1212, a1213) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1216 = f381(v0, v1.clone(), 0u64)?;
    match m1216 {
        None => {
            let a1217 = v1;
            Ok(length_list(a1217))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f288(v0: Adt14) -> u64 {
    let m1218 = f382(f168(v0, f8(Str::lit("stratum"))));
    match m1218 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f289(v0: u64, p1223: List<u64>) -> bool {
    let m1222 = p1223;
    {
        let mut m1222 = m1222;
        loop {
            let step = {
                let v1 = m1222;
                let m1219 = v1;
                match m1219.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1220 = v2;
                        let a1221 = v0;
                        if nat_eq(a1220, a1221) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f216(v0.clone(), v1) {
        let m1224 = f30(v0, 34u8)?;
        match m1224.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1225 = v3;
                match m1225.uncons() {
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

pub fn f291(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f292(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1226 = v0.clone();
    match m1226.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1227 = v2;
            let a1228 = v1.clone();
            if equal(a1227, a1228) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f293(v0: List<(Bytes, ())>) -> List<Bytes> {
    f193(f383(Fn6::F384, List::<Bytes>::nil(), v0))
}

pub fn f294(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f385(f386(Fn7::F387, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1233 = {
        let (h1229, _) = f388(Fn8::F389, {
            let a1231 = {
                let a1230 = v0;
                length_list(a1230)
            };
            let a1232 = 1u64;
            nat_add(a1231, a1232)?
        }, v1)?;
        h1229
    };
    match m1233.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f295(v0: List<Bytes>) -> List<(Bytes, ())> {
    f390(f391(Fn9::F392, List::<(Bytes, ())>::nil(), v0))
}

pub fn f296(p1236: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1235 = p1236;
    {
        let mut m1235 = m1235;
        loop {
            let step = {
                let v0 = m1235;
                let m1234 = v0;
                match m1234.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f326(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: Adt14, v1: Bytes) -> Bytes {
    if f37(f375(f168(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f298(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1237, _) = v1.clone();
        h1237
    };
    let v3: u64 = {
        let (h1238, _) = {
            let (_, h1239) = v1.clone();
            h1239
        };
        h1238
    };
    let v4: List<Bytes> = {
        let (_, h1240) = {
            let (_, h1241) = v1;
            h1241
        };
        h1240
    };
    let m1242 = f276(v0.clone(), v2);
    match m1242 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1243 = v5;
            let a1244 = 10u8;
            if equal(a1243, a1244) {
                Ok(Some(({
                    let a1245 = v2;
                    let a1246 = 1u64;
                    nat_add(a1245, a1246)?
                }, ({
                    let a1247 = v2;
                    let a1248 = 1u64;
                    nat_add(a1247, a1248)?
                }, List::cons(f393(f32(v0, v3, {
                    let a1249 = v2;
                    let a1250 = v3;
                    nat_sub(a1249, a1250)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1251 = v2;
                    let a1252 = 1u64;
                    nat_add(a1251, a1252)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f299(v0: Adt1, v1: Bytes) -> R<Adt19> {
    let m1253 = f155(v0, v1)?;
    match m1253 {
        None => {
            Ok(Adt19::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f300(v0: Bytes) -> R<Bytes> {
    let a1254 = f8(Str::lit("Catalog."));
    let a1255 = f265(v0)?;
    Ok(append_bytes(a1254, a1255))
}

pub fn f301(v0: Adt19) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f265(f159(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f302(v0: List<Bytes>) -> List<Bytes> {
    let m1256 = v0;
    match m1256.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f143(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f139(f147(), v1), f302(v2))
            } else {
                f302(v2)
            }
        }
    }
}

pub fn f303(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1257 = v1;
    match m1257.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f394(v0.clone(), v2)?;
            let (h1258, _) = v4.clone();
            if h1258 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f303(v0, v3)?;
                let (h1259, _) = v5.clone();
                if h1259 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1262 = {
                            let (_, h1260) = v4;
                            h1260
                        };
                        let a1263 = {
                            let (_, h1261) = v5;
                            h1261
                        };
                        append_bytes(a1262, a1263)
                    }))
                }
            }
        }
    }
}

pub fn f304(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1264 = v1;
    match m1264.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f214(v2.clone(), f8(Str::lit("subject"))) {
                if f215(v0.clone(), v2.clone()) {
                    f304(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f166(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f166(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f166(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f304(v0, v3))
                }
            } else {
                if f313(v0.clone(), v2.clone()) {
                    f304(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f166(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f314(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f304(v0, v3))
                }
            }
        }
    }
}

pub fn f305(v0: Adt1) -> R<Bytes> {
    let a1266 = {
        let a1265 = f108(f109(f127(f128(f117(f74(v0.clone(), f122()))?)), f110(f77(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f111(f77(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1265)
    };
    let a1267 = 0u64;
    if nat_eq(a1266, a1267) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f306(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1268 = v0;
    match m1268.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f126(f78(f165(f168(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f166(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f166(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f395(f87(f166(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f306(v2)?))
        }
    }
}

pub fn f307(v0: Bytes, v1: List<Adt23>) -> List<Bytes> {
    let m1269 = v1;
    match m1269.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37(match v2.clone() {
                Adt23::C0(h1270, _, _) => {
                    h1270
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt23::C0(_, h1271, _) => {
                        h1271
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt23::C0(_, _, h1272) => {
                        h1272
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f307(v0, v3))
            } else {
                f307(v0, v3)
            }
        }
    }
}

pub fn f308(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1273 = v1;
    match m1273.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37(f166(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f166(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f166(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f308(v0, v3))
            } else {
                if f37(f166(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f166(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f166(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f308(v0, v3))
                } else {
                    f308(v0, v3)
                }
            }
        }
    }
}

pub fn f309(p1278: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1277 = p1278;
    {
        let mut m1277 = m1277;
        loop {
            let step = {
                let v0 = m1277;
                let m1274 = v0;
                match m1274.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1275 = v2;
                        match m1275.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f37(v4, v1.clone()) {
                                    let m1276 = v5;
                                    match m1276.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f396(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1279 = v0;
    match m1279.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1280 = f168(v1, f8(Str::lit("std")));
            match m1280 {
                None => {
                    f310(v2)
                }
                Some(v3) => {
                    if f214(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f166(v3.clone(), f8(Str::lit("row"))), f166(v3, f8(Str::lit("authority")))), f310(v2))
                    } else {
                        f310(v2)
                    }
                }
            }
        }
    }
}

pub fn f311(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1281 = v1;
    match m1281.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f166(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f166(v2, f8(Str::lit("object")));
            if if f90(v4.clone(), match v0.clone() {
                Adt15::C0(h1282, _, _) => {
                    h1282
                }
            }) {
                f90(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1283, _) => {
                        h1283
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f311(v0, v3))
            } else {
                if if f90(v5.clone(), match v0.clone() {
                    Adt15::C0(h1284, _, _) => {
                        h1284
                    }
                }) {
                    f90(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1285, _) => {
                            h1285
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f311(v0, v3))
                } else {
                    f311(v0, v3)
                }
            }
        }
    }
}

pub fn f312(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt23>> {
    let m1286 = v0;
    match m1286.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v4, v5)) => {
            if if f397(v4.clone(), v2.clone()) {
                true
            } else {
                f397(v4.clone(), v3.clone())
            } {
                f312(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt23::C0({
                    let (h1287, _) = v4.clone();
                    h1287
                }, {
                    let a1290 = {
                        let a1289 = f82(f398({
                            let (h1288, _) = v4.clone();
                            h1288
                        }, v1.clone()))?;
                        length_list(a1289)
                    };
                    let a1291 = 1u64;
                    if nat_eq(a1290, a1291) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1292) = v4.clone();
                    h1292
                }), f312(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f313(v0: Adt15, v1: Adt14) -> bool {
    if f90(f166(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1293, _, _) => {
            h1293
        }
    }) {
        f90(f314(v1), match v0 {
            Adt15::C0(_, h1294, _) => {
                h1294
            }
        })
    } else {
        false
    }
}

pub fn f314(v0: Adt14) -> Bytes {
    let m1295 = f168(v0, f8(Str::lit("evidence")));
    match m1295 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f166(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f315(v0: Adt14) -> Bytes {
    if f214(v0.clone(), f8(Str::lit("req"))) {
        let a1296 = f8(Str::lit("req "));
        let a1297 = f166(v0, f8(Str::lit("req")));
        append_bytes(a1296, a1297)
    } else {
        let m1298 = f168(v0.clone(), f8(Str::lit("std")));
        match m1298 {
            None => {
                let m1301 = f168(v0, f8(Str::lit("der")));
                match m1301 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f166(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f126(f78(f165(f168(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f166(v1.clone(), f8(Str::lit("authority"))), List::cons(if f214(v1.clone(), f8(Str::lit("row"))) {
                    let a1299 = f8(Str::lit(" row "));
                    let a1300 = f166(v1, f8(Str::lit("row")));
                    append_bytes(a1299, a1300)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f316(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1302 = v1;
    match m1302.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f90({
                let (h1303, _) = v2.clone();
                h1303
            }, v0.clone()) {
                List::cons({
                    let (_, h1304) = v2;
                    h1304
                }, f316(v0, v3))
            } else {
                if f90({
                    let (_, h1305) = v2;
                    h1305
                }, v0.clone()) {
                    List::cons({
                        let (h1306, _) = v2;
                        h1306
                    }, f316(v0, v3))
                } else {
                    f316(v0, v3)
                }
            }
        }
    }
}

pub fn f317(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f90(v2.clone(), match v0.clone() {
        Adt15::C0(h1307, _, _) => {
            h1307
        }
    }) {
        true
    } else {
        if f90(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1308, _) => {
                h1308
            }
        }) {
            true
        } else {
            if f90(v2, match v0.clone() {
                Adt15::C0(_, _, h1309) => {
                    h1309
                }
            }) {
                true
            } else {
                if f214(v1.clone(), f8(Str::lit("scheme"))) {
                    f90(f166(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1310, _) => {
                            h1310
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f318(v0: Bytes, v1: u64) -> R<u64> {
    let m1313 = {
        let a1311 = v0;
        let a1312 = v1;
        index_bytes(a1311, a1312)
    };
    match m1313 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1358 = {
                let a1334 = {
                    let a1322 = {
                        let a1316 = {
                            let a1314 = v3;
                            let a1315 = 128u8;
                            fixed_u8::bit_and(a1314, a1315)
                        };
                        let a1317 = 128u8;
                        if equal(a1316, a1317) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1323 = {
                        let a1320 = {
                            let a1318 = v3;
                            let a1319 = 64u8;
                            fixed_u8::bit_and(a1318, a1319)
                        };
                        let a1321 = 64u8;
                        if equal(a1320, a1321) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1322, a1323)?
                };
                let a1335 = {
                    let a1332 = {
                        let a1326 = {
                            let a1324 = v3;
                            let a1325 = 32u8;
                            fixed_u8::bit_and(a1324, a1325)
                        };
                        let a1327 = 32u8;
                        if equal(a1326, a1327) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1333 = {
                        let a1330 = {
                            let a1328 = v3;
                            let a1329 = 16u8;
                            fixed_u8::bit_and(a1328, a1329)
                        };
                        let a1331 = 16u8;
                        if equal(a1330, a1331) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1332, a1333)?
                };
                nat_add(a1334, a1335)?
            };
            let a1359 = {
                let a1356 = {
                    let a1344 = {
                        let a1338 = {
                            let a1336 = v3;
                            let a1337 = 8u8;
                            fixed_u8::bit_and(a1336, a1337)
                        };
                        let a1339 = 8u8;
                        if equal(a1338, a1339) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1345 = {
                        let a1342 = {
                            let a1340 = v3;
                            let a1341 = 4u8;
                            fixed_u8::bit_and(a1340, a1341)
                        };
                        let a1343 = 4u8;
                        if equal(a1342, a1343) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1344, a1345)?
                };
                let a1357 = {
                    let a1354 = {
                        let a1348 = {
                            let a1346 = v3;
                            let a1347 = 2u8;
                            fixed_u8::bit_and(a1346, a1347)
                        };
                        let a1349 = 2u8;
                        if equal(a1348, a1349) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1355 = {
                        let a1352 = {
                            let a1350 = v3;
                            let a1351 = 1u8;
                            fixed_u8::bit_and(a1350, a1351)
                        };
                        let a1353 = 1u8;
                        if equal(a1352, a1353) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1354, a1355)?
                };
                nat_add(a1356, a1357)?
            };
            nat_add(a1358, a1359)
        }
    }
}

pub fn f319(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1363 = {
        let a1360 = v1;
        let a1361 = v2;
        let a1362 = 1u64;
        slice_bytes(a1360, a1361, a1362)
    };
    match m1363 {
        None => {
            v0
        }
        Some(v3) => {
            let a1364 = v0;
            let a1365 = v3;
            append_bytes(a1364, a1365)
        }
    }
}

pub fn f320(p1368: List<Bytes>, p1369: List<Bytes>) -> List<Bytes> {
    let m1367 = (p1368, p1369);
    {
        let mut m1367 = m1367;
        loop {
            let step = {
                let (v0, v1) = m1367;
                let m1366 = v0;
                match m1366.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1367 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f321(v0: Bytes, p1441: u64, p1442: u64, p1443: Bytes) -> R<Bytes> {
    let m1440 = (p1441, (p1442, p1443));
    {
        let mut m1440 = m1440;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1440;
                let m1370 = v2;
                if m1370 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1370 - 1;
                    let m1373 = {
                        let a1371 = v0.clone();
                        let a1372 = v1;
                        index_bytes(a1371, a1372)
                    };
                    match m1373 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1418 = {
                                    let a1394 = {
                                        let a1382 = {
                                            let a1376 = {
                                                let a1374 = v6;
                                                let a1375 = 128u8;
                                                fixed_u8::bit_and(a1374, a1375)
                                            };
                                            let a1377 = 128u8;
                                            if equal(a1376, a1377) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1383 = {
                                            let a1380 = {
                                                let a1378 = v6;
                                                let a1379 = 64u8;
                                                fixed_u8::bit_and(a1378, a1379)
                                            };
                                            let a1381 = 64u8;
                                            if equal(a1380, a1381) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1382, a1383)?
                                    };
                                    let a1395 = {
                                        let a1392 = {
                                            let a1386 = {
                                                let a1384 = v6;
                                                let a1385 = 32u8;
                                                fixed_u8::bit_and(a1384, a1385)
                                            };
                                            let a1387 = 32u8;
                                            if equal(a1386, a1387) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1393 = {
                                            let a1390 = {
                                                let a1388 = v6;
                                                let a1389 = 16u8;
                                                fixed_u8::bit_and(a1388, a1389)
                                            };
                                            let a1391 = 16u8;
                                            if equal(a1390, a1391) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1392, a1393)?
                                    };
                                    nat_add(a1394, a1395)?
                                };
                                let a1419 = {
                                    let a1416 = {
                                        let a1404 = {
                                            let a1398 = {
                                                let a1396 = v6;
                                                let a1397 = 8u8;
                                                fixed_u8::bit_and(a1396, a1397)
                                            };
                                            let a1399 = 8u8;
                                            if equal(a1398, a1399) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1405 = {
                                            let a1402 = {
                                                let a1400 = v6;
                                                let a1401 = 4u8;
                                                fixed_u8::bit_and(a1400, a1401)
                                            };
                                            let a1403 = 4u8;
                                            if equal(a1402, a1403) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1404, a1405)?
                                    };
                                    let a1417 = {
                                        let a1414 = {
                                            let a1408 = {
                                                let a1406 = v6;
                                                let a1407 = 2u8;
                                                fixed_u8::bit_and(a1406, a1407)
                                            };
                                            let a1409 = 2u8;
                                            if equal(a1408, a1409) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1415 = {
                                            let a1412 = {
                                                let a1410 = v6;
                                                let a1411 = 1u8;
                                                fixed_u8::bit_and(a1410, a1411)
                                            };
                                            let a1413 = 1u8;
                                            if equal(a1412, a1413) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1414, a1415)?
                                    };
                                    nat_add(a1416, a1417)?
                                };
                                nat_add(a1418, a1419)?
                            };
                            let a1420 = v7;
                            let a1421 = 34u64;
                            if if nat_eq(a1420, a1421) {
                                true
                            } else {
                                let a1422 = v7;
                                let a1423 = 92u64;
                                nat_eq(a1422, a1423)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1424 = v1;
                                    let a1425 = 1u64;
                                    nat_add(a1424, a1425)?
                                }, (v4, {
                                    let a1428 = v3;
                                    let a1429 = {
                                        let a1426 = Bytes::lit(&[0x5c]);
                                        let a1427 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1426, a1427)
                                    };
                                    append_bytes(a1428, a1429)
                                })))
                            } else {
                                let a1430 = v7;
                                let a1431 = 10u64;
                                if nat_eq(a1430, a1431) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1432 = v1;
                                        let a1433 = 1u64;
                                        nat_add(a1432, a1433)?
                                    }, (v4, {
                                        let a1434 = v3;
                                        let a1435 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1434, a1435)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1436 = v1;
                                        let a1437 = 1u64;
                                        nat_add(a1436, a1437)?
                                    }, (v4, {
                                        let a1438 = v3;
                                        let a1439 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1438, a1439)
                                    })))
                                }
                            }
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

pub fn f322(v0: u32) -> Bytes {
    let a1472 = f399({
        let a1444 = f400(v0, 28u32);
        let a1445 = 15u32;
        fixed_u32::bit_and(a1444, a1445)
    });
    let a1473 = {
        let a1470 = f399({
            let a1446 = f400(v0, 24u32);
            let a1447 = 15u32;
            fixed_u32::bit_and(a1446, a1447)
        });
        let a1471 = {
            let a1468 = f399({
                let a1448 = f400(v0, 20u32);
                let a1449 = 15u32;
                fixed_u32::bit_and(a1448, a1449)
            });
            let a1469 = {
                let a1466 = f399({
                    let a1450 = f400(v0, 16u32);
                    let a1451 = 15u32;
                    fixed_u32::bit_and(a1450, a1451)
                });
                let a1467 = {
                    let a1464 = f399({
                        let a1452 = f400(v0, 12u32);
                        let a1453 = 15u32;
                        fixed_u32::bit_and(a1452, a1453)
                    });
                    let a1465 = {
                        let a1462 = f399({
                            let a1454 = f400(v0, 8u32);
                            let a1455 = 15u32;
                            fixed_u32::bit_and(a1454, a1455)
                        });
                        let a1463 = {
                            let a1460 = f399({
                                let a1456 = f400(v0, 4u32);
                                let a1457 = 15u32;
                                fixed_u32::bit_and(a1456, a1457)
                            });
                            let a1461 = f399({
                                let a1458 = v0;
                                let a1459 = 15u32;
                                fixed_u32::bit_and(a1458, a1459)
                            });
                            append_bytes(a1460, a1461)
                        };
                        append_bytes(a1462, a1463)
                    };
                    append_bytes(a1464, a1465)
                };
                append_bytes(a1466, a1467)
            };
            append_bytes(a1468, a1469)
        };
        append_bytes(a1470, a1471)
    };
    append_bytes(a1472, a1473)
}

pub fn f323(v0: Fn4, p1477: u64, p1478: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1476 = (p1477, p1478);
    {
        let mut m1476 = m1476;
        loop {
            let step = {
                let (v1, v2) = m1476;
                let m1474 = v1;
                if m1474 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1474 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1475: Fn4 = v0.clone();
                        c1475.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1476 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1480 = {
            let (h1479, _) = v3.clone();
            h1479
        };
        let a1481 = 64u64;
        nat_add(a1480, a1481)?
    }, f401({
        let (_, h1482) = v3;
        h1482
    }, f402(v0, v1, v2, {
        let (h1483, _) = v3;
        h1483
    })?)?))
}

pub fn f325() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f326(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1484 = f30(v1, 34u8)?;
    match m1484.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1485 = v3;
            match m1485.uncons() {
                None => {
                    Ok(f403(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f327(v0: List<Bytes>) -> List<Bytes> {
    let m1486 = v0;
    match m1486.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f37(v1.clone(), f8(Str::lit("authority = []"))) {
                f327(v2)
            } else {
                List::cons(v1, f327(v2))
            }
        }
    }
}

pub fn f328(v0: Bytes) -> R<u64> {
    let (h1487, _) = f280(Fn3::F404(v0.clone()), {
        let a1488 = v0.clone();
        length_bytes(a1488)
    }, {
        let a1489 = v0;
        length_bytes(a1489)
    })?;
    Ok(h1487)
}

pub fn f329(v0: Adt14) -> R<Bytes> {
    let m1490 = v0;
    match m1490 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f368(v2, v3)
        }
        Adt14::C2(v4) => {
            f370(v4)
        }
        Adt14::C3(r1491) => {
            let v5 = (*r1491).clone();
            Ok(f371(f405(v5)?))
        }
        Adt14::C4(r1492) => {
            let v6 = (*r1492).clone();
            f373(f406(v6)?)
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

pub fn f330(v0: Fn5, p1497: u64, p1498: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1496 = (p1497, p1498);
    {
        let mut m1496 = m1496;
        loop {
            let step = {
                let (v1, v2) = m1496;
                let m1493 = v1;
                if m1493 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1493 - 1;
                    let m1495 = {
                        let c1494: Fn5 = v0.clone();
                        c1494.apply(v2.clone())?
                    };
                    match m1495 {
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
                Ok(next) => { m1496 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f407(v0, v1)
}

pub fn f332(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f408(v0, v1)
}

pub fn f333(v0: u64) -> bool {
    let a1499 = v0;
    let a1500 = 32u64;
    if if nat_eq(a1499, a1500) {
        true
    } else {
        let a1501 = v0;
        let a1502 = 9u64;
        nat_eq(a1501, a1502)
    } {
        true
    } else {
        let a1503 = v0;
        let a1504 = 10u64;
        if nat_eq(a1503, a1504) {
            true
        } else {
            let a1505 = v0;
            let a1506 = 13u64;
            nat_eq(a1505, a1506)
        }
    }
}

pub fn f334(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1507, _) = f280(Fn3::F409(v1.clone(), v0.clone()), {
            let a1509 = {
                let a1508 = v0.clone();
                length_bytes(a1508)
            };
            let a1510 = 1u64;
            nat_add(a1509, a1510)?
        }, v2)?;
        h1507
    };
    if f231(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f335(v0: Bytes, v1: u64) -> R<u64> {
    let (h1511, _) = f280(Fn3::F410(v0.clone()), {
        let a1513 = {
            let a1512 = v0;
            length_bytes(a1512)
        };
        let a1514 = 1u64;
        nat_add(a1513, a1514)?
    }, v1)?;
    Ok(h1511)
}

pub fn f336(v0: Bytes) -> R<bool> {
    let a1516 = {
        let a1515 = v0.clone();
        length_bytes(a1515)
    };
    let a1517 = 5u64;
    if nat_eq(a1516, a1517) {
        let a1518 = f339(v0.clone(), 0u64)?;
        let a1519 = 85u64;
        if if nat_eq(a1518, a1519) {
            true
        } else {
            let a1520 = f339(v0.clone(), 0u64)?;
            let a1521 = 117u64;
            nat_eq(a1520, a1521)
        } {
            let a1522 = f339(v0.clone(), 1u64)?;
            let a1523 = 84u64;
            if if nat_eq(a1522, a1523) {
                true
            } else {
                let a1524 = f339(v0.clone(), 1u64)?;
                let a1525 = 116u64;
                nat_eq(a1524, a1525)
            } {
                let a1526 = f339(v0.clone(), 2u64)?;
                let a1527 = 70u64;
                if if nat_eq(a1526, a1527) {
                    true
                } else {
                    let a1528 = f339(v0.clone(), 2u64)?;
                    let a1529 = 102u64;
                    nat_eq(a1528, a1529)
                } {
                    let a1530 = f339(v0.clone(), 3u64)?;
                    let a1531 = 45u64;
                    if nat_eq(a1530, a1531) {
                        let a1532 = f339(v0, 4u64)?;
                        let a1533 = 56u64;
                        Ok(nat_eq(a1532, a1533))
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f337(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1537 = f334(v0.clone(), {
        let a1534 = Str::lit("--");
        utf8_encode(a1534)
    }, {
        let a1535 = v1;
        let a1536 = 4u64;
        nat_add(a1535, a1536)?
    })?;
    match m1537 {
        None => {
            Ok(Err::<u64, Bytes>(f234(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1540 = f238(v0, {
                let a1538 = v2;
                let a1539 = 2u64;
                nat_add(a1538, a1539)?
            })?;
            let a1541 = 62u64;
            if nat_eq(a1540, a1541) {
                Ok(Ok::<u64, Bytes>({
                    let a1542 = v2;
                    let a1543 = 3u64;
                    nat_add(a1542, a1543)?
                }))
            } else {
                Ok(Err::<u64, Bytes>(f234(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f338(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f340(v0.clone(), {
        let a1544 = v1;
        let a1545 = 2u64;
        nat_add(a1544, a1545)?
    })?;
    let a1548 = v2;
    let a1549 = {
        let a1546 = v1;
        let a1547 = 2u64;
        nat_add(a1546, a1547)?
    };
    if nat_eq(a1548, a1549) {
        Ok(Err::<u64, Bytes>(f234(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1552 = v2;
        let a1553 = {
            let a1550 = v1;
            let a1551 = 5u64;
            nat_add(a1550, a1551)?
        };
        if if nat_eq(a1552, a1553) {
            let a1556 = f238(v0.clone(), {
                let a1554 = v1;
                let a1555 = 2u64;
                nat_add(a1554, a1555)?
            })?;
            let a1557 = 120u64;
            if if nat_eq(a1556, a1557) {
                true
            } else {
                let a1560 = f238(v0.clone(), {
                    let a1558 = v1;
                    let a1559 = 2u64;
                    nat_add(a1558, a1559)?
                })?;
                let a1561 = 88u64;
                nat_eq(a1560, a1561)
            } {
                let a1564 = f238(v0.clone(), {
                    let a1562 = v1;
                    let a1563 = 3u64;
                    nat_add(a1562, a1563)?
                })?;
                let a1565 = 109u64;
                if if nat_eq(a1564, a1565) {
                    true
                } else {
                    let a1568 = f238(v0.clone(), {
                        let a1566 = v1;
                        let a1567 = 3u64;
                        nat_add(a1566, a1567)?
                    })?;
                    let a1569 = 77u64;
                    nat_eq(a1568, a1569)
                } {
                    let a1572 = f238(v0.clone(), {
                        let a1570 = v1;
                        let a1571 = 4u64;
                        nat_add(a1570, a1571)?
                    })?;
                    let a1573 = 108u64;
                    if nat_eq(a1572, a1573) {
                        true
                    } else {
                        let a1576 = f238(v0.clone(), {
                            let a1574 = v1;
                            let a1575 = 4u64;
                            nat_add(a1574, a1575)?
                        })?;
                        let a1577 = 76u64;
                        nat_eq(a1576, a1577)
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f234(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1579 = f334(v0, {
                let a1578 = Str::lit("?>");
                utf8_encode(a1578)
            }, v2)?;
            match m1579 {
                None => {
                    Ok(Err::<u64, Bytes>(f234(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1580 = v3;
                        let a1581 = 2u64;
                        nat_add(a1580, a1581)?
                    }))
                }
            }
        }
    }
}

pub fn f339(v0: Bytes, v1: u64) -> R<u64> {
    let m1584 = {
        let a1582 = v0;
        let a1583 = v1;
        index_bytes(a1582, a1583)
    };
    match m1584 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1629 = {
                let a1605 = {
                    let a1593 = {
                        let a1587 = {
                            let a1585 = v3;
                            let a1586 = 128u8;
                            fixed_u8::bit_and(a1585, a1586)
                        };
                        let a1588 = 128u8;
                        if equal(a1587, a1588) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1594 = {
                        let a1591 = {
                            let a1589 = v3;
                            let a1590 = 64u8;
                            fixed_u8::bit_and(a1589, a1590)
                        };
                        let a1592 = 64u8;
                        if equal(a1591, a1592) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1593, a1594)?
                };
                let a1606 = {
                    let a1603 = {
                        let a1597 = {
                            let a1595 = v3;
                            let a1596 = 32u8;
                            fixed_u8::bit_and(a1595, a1596)
                        };
                        let a1598 = 32u8;
                        if equal(a1597, a1598) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1604 = {
                        let a1601 = {
                            let a1599 = v3;
                            let a1600 = 16u8;
                            fixed_u8::bit_and(a1599, a1600)
                        };
                        let a1602 = 16u8;
                        if equal(a1601, a1602) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1603, a1604)?
                };
                nat_add(a1605, a1606)?
            };
            let a1630 = {
                let a1627 = {
                    let a1615 = {
                        let a1609 = {
                            let a1607 = v3;
                            let a1608 = 8u8;
                            fixed_u8::bit_and(a1607, a1608)
                        };
                        let a1610 = 8u8;
                        if equal(a1609, a1610) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1616 = {
                        let a1613 = {
                            let a1611 = v3;
                            let a1612 = 4u8;
                            fixed_u8::bit_and(a1611, a1612)
                        };
                        let a1614 = 4u8;
                        if equal(a1613, a1614) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1615, a1616)?
                };
                let a1628 = {
                    let a1625 = {
                        let a1619 = {
                            let a1617 = v3;
                            let a1618 = 2u8;
                            fixed_u8::bit_and(a1617, a1618)
                        };
                        let a1620 = 2u8;
                        if equal(a1619, a1620) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1626 = {
                        let a1623 = {
                            let a1621 = v3;
                            let a1622 = 1u8;
                            fixed_u8::bit_and(a1621, a1622)
                        };
                        let a1624 = 1u8;
                        if equal(a1623, a1624) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1625, a1626)?
                };
                nat_add(a1627, a1628)?
            };
            nat_add(a1629, a1630)
        }
    }
}

pub fn f340(v0: Bytes, v1: u64) -> R<u64> {
    let (h1631, _) = f280(Fn3::F411(v0.clone(), v1), {
        let a1633 = {
            let a1632 = v0;
            length_bytes(a1632)
        };
        let a1634 = 1u64;
        nat_add(a1633, a1634)?
    }, v1)?;
    Ok(h1631)
}

pub fn f341(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt18, u64), Bytes>> {
    let m1635 = v2;
    if m1635 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1635 - 1;
        let v5: u64 = f335(v0.clone(), v1)?;
        let a1636 = f238(v0.clone(), v5)?;
        let a1637 = 62u64;
        if if nat_eq(a1636, a1637) {
            true
        } else {
            f237(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v5)))
        } else {
            let a1638 = v5;
            let a1639 = v1;
            if nat_eq(a1638, a1639) {
                Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f340(v0.clone(), v5)?;
                let a1640 = v6;
                let a1641 = v5;
                if nat_eq(a1640, a1641) {
                    Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a1642 = v6;
                        let a1643 = v5;
                        nat_sub(a1642, a1643)
                    });
                    if f412(v7.clone(), v3.clone()) {
                        Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f335(v0.clone(), v6)?;
                        let a1644 = f238(v0.clone(), v8)?;
                        let a1645 = 61u64;
                        if !nat_eq(a1644, a1645) {
                            Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        } else {
                            let v9: u64 = f335(v0.clone(), {
                                let a1646 = v8;
                                let a1647 = 1u64;
                                nat_add(a1646, a1647)?
                            })?;
                            let v10: u64 = f238(v0.clone(), v9)?;
                            let a1648 = v10;
                            let a1649 = 34u64;
                            if !if nat_eq(a1648, a1649) {
                                true
                            } else {
                                let a1650 = v10;
                                let a1651 = 39u64;
                                nat_eq(a1650, a1651)
                            } {
                                Ok(Err::<(Adt18, u64), Bytes>(f234(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            } else {
                                let m1654 = f342(v0.clone(), {
                                    let a1652 = v9;
                                    let a1653 = 1u64;
                                    nat_add(a1652, a1653)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1654 {
                                    Ok(v12) => {
                                        let m1656 = f341(v0, {
                                            let (_, h1655) = v12;
                                            h1655
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1656 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C1(v7, {
                                                    let (h1657, _) = v12;
                                                    h1657
                                                }, std::rc::Rc::new({
                                                    let (h1658, _) = v14;
                                                    h1658
                                                })), {
                                                    let (_, h1659) = v14;
                                                    h1659
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt18, u64), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v11))
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

pub fn f342(v0: Bytes, p1699: u64, p1700: u64, p1701: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1698 = (p1699, (p1700, p1701));
    {
        let mut m1698 = m1698;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1698;
                let m1660 = v2;
                if m1660 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f234(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1660 - 1;
                    let v7: u64 = f238(v0.clone(), v1)?;
                    if if v4 {
                        let a1661 = v7;
                        let a1662 = v5;
                        nat_eq(a1661, a1662)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1663 = v1;
                            let a1664 = 1u64;
                            nat_add(a1663, a1664)?
                        }))))
                    } else {
                        let a1665 = v7;
                        let a1666 = 256u64;
                        if nat_eq(a1665, a1666) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f234(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1667 = v7;
                            let a1668 = 60u64;
                            if nat_eq(a1667, a1668) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f234(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1669 = v7;
                                let a1670 = 38u64;
                                if nat_eq(a1669, a1670) {
                                    let m1671 = f413(v0.clone(), v1)?;
                                    match m1671 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1672) = v9;
                                                h1672
                                            }, (v6, {
                                                let a1674 = v3;
                                                let a1675 = {
                                                    let (h1673, _) = v9;
                                                    h1673
                                                };
                                                append_bytes(a1674, a1675)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1676 = v7;
                                        let a1677 = 9u64;
                                        if if nat_eq(a1676, a1677) {
                                            true
                                        } else {
                                            let a1678 = v7;
                                            let a1679 = 10u64;
                                            nat_eq(a1678, a1679)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1680 = v1;
                                                let a1681 = 1u64;
                                                nat_add(a1680, a1681)?
                                            }, (v6, {
                                                let a1682 = v3;
                                                let a1683 = Bytes::lit(&[0x20]);
                                                append_bytes(a1682, a1683)
                                            })))
                                        } else {
                                            let v10: u64 = f414(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1686 = v3;
                                                let a1687 = f32(v0.clone(), v1, {
                                                    let a1684 = v10;
                                                    let a1685 = v1;
                                                    nat_sub(a1684, a1685)
                                                });
                                                append_bytes(a1686, a1687)
                                            })))
                                        }
                                    } else {
                                        let a1688 = v7;
                                        let a1689 = 93u64;
                                        if nat_eq(a1688, a1689) {
                                            if f237(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f234(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1690 = v1;
                                                    let a1691 = 1u64;
                                                    nat_add(a1690, a1691)?
                                                }, (v6, {
                                                    let a1692 = v3;
                                                    let a1693 = Bytes::lit(&[0x5d]);
                                                    append_bytes(a1692, a1693)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f414(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1696 = v3;
                                                let a1697 = f32(v0.clone(), v1, {
                                                    let a1694 = v11;
                                                    let a1695 = v1;
                                                    nat_sub(a1694, a1695)
                                                });
                                                append_bytes(a1696, a1697)
                                            })))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: Bytes, v1: u64) -> Bytes {
    let m1705 = {
        let a1702 = v0;
        let a1703 = v1;
        let a1704 = 1u64;
        slice_bytes(a1702, a1703, a1704)
    };
    match m1705 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f344(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f415(v1, v0, v2)
}

pub fn f345() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f346() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f347(v0: Bytes) -> R<Bytes> {
    let m1706 = f334(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1706 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f348(p1711: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1710 = p1711;
    {
        let mut m1710 = m1710;
        loop {
            let step = {
                let v0 = m1710;
                let m1707 = v0;
                match m1707.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h1708, _) = v2;
                            h1708
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1709) = v2;
                                h1709
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1710 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(v0: Bytes) -> R<Bytes> {
    let m1712 = f334(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1712 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1713 = v1;
                let a1714 = 1u64;
                nat_add(a1713, a1714)?
            }, {
                let a1718 = {
                    let a1715 = v0;
                    length_bytes(a1715)
                };
                let a1719 = {
                    let a1716 = v1;
                    let a1717 = 1u64;
                    nat_add(a1716, a1717)?
                };
                nat_sub(a1718, a1719)
            }))
        }
    }
}

pub fn f350() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f351(v0: Bytes, v1: Adt18, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v4: Bytes = f265(v0.clone())?;
    let m1720 = f416(v1, v2)?;
    match m1720 {
        Ok(v6) => {
            let m1721 = v6;
            match m1721 {
                None => {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f417(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = {
                        let (h1722, _) = v7.clone();
                        h1722
                    };
                    let v9: Adt18 = {
                        let (h1723, _) = {
                            let (_, h1724) = v7.clone();
                            h1724
                        };
                        h1723
                    };
                    let v10: Adt18 = {
                        let (h1725, _) = {
                            let (_, h1726) = {
                                let (_, h1727) = v7.clone();
                                h1727
                            };
                            h1726
                        };
                        h1725
                    };
                    let v11: List<(Bytes, Bytes)> = {
                        let (_, h1728) = {
                            let (_, h1729) = {
                                let (_, h1730) = v7;
                                h1730
                            };
                            h1729
                        };
                        h1728
                    };
                    if f37(v8.clone(), f8(Str::lit("union"))) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f417(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f37(v8, f8(Str::lit("list"))) {
                            let m1731 = f245(v9, f8(Str::lit("itemType")));
                            match m1731 {
                                None => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f417(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1732 = f418(v11, v3, v12)?;
                                    match m1732 {
                                        Ok(v14) => {
                                            if f216({
                                                let (h1733, _) = v14.clone();
                                                h1733
                                            }, f8(Str::lit("(List "))) {
                                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f417(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f419(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons({
                                                    let (h1734, _) = v14;
                                                    h1734
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), {
                                                    let (_, h1735) = v14;
                                                    h1735
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f417(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1736 = f245(v9, f8(Str::lit("base")));
                            match m1736 {
                                None => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f417(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1737 = f418(v11.clone(), v3, v15)?;
                                    match m1737 {
                                        Ok(v17) => {
                                            let m1738 = f420(v10, v11)?;
                                            match m1738 {
                                                Ok(v19) => {
                                                    let v20: List<Adt19> = f421(v19, List::<Bytes>::nil())?;
                                                    let m1739 = v20.clone();
                                                    match m1739.uncons() {
                                                        None => {
                                                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f419(v4, {
                                                                let (h1740, _) = v17;
                                                                h1740
                                                            }), {
                                                                let (_, h1741) = v17;
                                                                h1741
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1742 = f422(f423(f160(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1742 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f424(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f417(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f417(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f417(v16, v0)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(v5) => {
            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f417(v5, v0)))
        }
    }
}

pub fn f352(p1745: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1744 = p1745;
    {
        let mut m1744 = m1744;
        loop {
            let step = {
                let v0 = m1744;
                let m1743 = v0;
                match m1743.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f90(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
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

pub fn f353(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1746 = v0;
    match m1746.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1749 = f352({
                let (h1747, _) = {
                    let (_, h1748) = v2.clone();
                    h1748
                };
                h1747
            }, v1.clone());
            match m1749 {
                None => {
                    Some((({
                        let (h1750, _) = v2;
                        h1750
                    }, {
                        let (_, h1751) = {
                            let (_, h1752) = v2;
                            h1752
                        };
                        h1751
                    }), v3))
                }
                Some(_) => {
                    let m1753 = f353(v3, v1);
                    match m1753 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1754, _) = v5;
                                h1754
                            }, List::cons(v2, {
                                let (_, h1755) = v5;
                                h1755
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f354(v0: Bytes, v1: u64) -> R<u64> {
    let (h1756, _) = f280(Fn3::F425(v1, v0), v1, 0u64)?;
    Ok(h1756)
}

pub fn f355(v0: Bytes) -> R<Bytes> {
    f426(v0.clone(), 0u64, {
        let a1757 = v0;
        length_bytes(a1757)
    }, false, false, Bytes::lit(&[]))
}

pub fn f356(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1758 = v2;
    if m1758 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1758 - 1;
        let v5: u64 = f339(v0.clone(), v1)?;
        let a1759 = v3;
        let a1760 = 1u64;
        if nat_eq(a1759, a1760) {
            let m1761 = f356(v0.clone(), v1, v4, 0u64)?;
            match m1761 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f339(v0.clone(), {
                        let (_, h1762) = v6.clone();
                        h1762
                    })?;
                    let a1763 = v7;
                    let a1764 = 44u64;
                    if nat_eq(a1763, a1764) {
                        let m1768 = f356(v0, {
                            let a1766 = {
                                let (_, h1765) = v6;
                                h1765
                            };
                            let a1767 = 1u64;
                            nat_add(a1766, a1767)?
                        }, v4, 1u64)?;
                        match m1768 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1769, _) = v6;
                                    h1769
                                }), std::rc::Rc::new({
                                    let (h1770, _) = v8;
                                    h1770
                                })), {
                                    let (_, h1771) = v8;
                                    h1771
                                })))
                            }
                        }
                    } else {
                        let a1772 = v7;
                        let a1773 = 93u64;
                        if nat_eq(a1772, a1773) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1774, _) = v6;
                                h1774
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1776 = {
                                    let (_, h1775) = v6;
                                    h1775
                                };
                                let a1777 = 1u64;
                                nat_add(a1776, a1777)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1778 = v3;
            let a1779 = 2u64;
            if nat_eq(a1778, a1779) {
                let m1782 = f427(v0.clone(), {
                    let a1780 = v1;
                    let a1781 = 1u64;
                    nat_add(a1780, a1781)?
                })?;
                match m1782 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1783 = v5;
                        let a1784 = 34u64;
                        if if nat_eq(a1783, a1784) {
                            let a1786 = f339(v0.clone(), {
                                let (h1785, _) = v9.clone();
                                h1785
                            })?;
                            let a1787 = 58u64;
                            nat_eq(a1786, a1787)
                        } else {
                            false
                        } {
                            let m1791 = f356(v0.clone(), {
                                let a1789 = {
                                    let (h1788, _) = v9;
                                    h1788
                                };
                                let a1790 = 1u64;
                                nat_add(a1789, a1790)?
                            }, v4, 0u64)?;
                            match m1791 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f339(v0.clone(), {
                                        let (_, h1792) = v10.clone();
                                        h1792
                                    })?;
                                    let a1793 = v11;
                                    let a1794 = 44u64;
                                    if nat_eq(a1793, a1794) {
                                        let m1798 = f356(v0, {
                                            let a1796 = {
                                                let (_, h1795) = v10;
                                                h1795
                                            };
                                            let a1797 = 1u64;
                                            nat_add(a1796, a1797)?
                                        }, v4, 2u64)?;
                                        match m1798 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1799) = v9;
                                                    h1799
                                                }, std::rc::Rc::new({
                                                    let (h1800, _) = v10;
                                                    h1800
                                                }), std::rc::Rc::new({
                                                    let (h1801, _) = v12;
                                                    h1801
                                                })), {
                                                    let (_, h1802) = v12;
                                                    h1802
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1803 = v11;
                                        let a1804 = 125u64;
                                        if nat_eq(a1803, a1804) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1805) = v9;
                                                h1805
                                            }, std::rc::Rc::new({
                                                let (h1806, _) = v10;
                                                h1806
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1808 = {
                                                    let (_, h1807) = v10;
                                                    h1807
                                                };
                                                let a1809 = 1u64;
                                                nat_add(a1808, a1809)?
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
                let a1810 = v5;
                let a1811 = 34u64;
                if nat_eq(a1810, a1811) {
                    let m1814 = f427(v0, {
                        let a1812 = v1;
                        let a1813 = 1u64;
                        nat_add(a1812, a1813)?
                    })?;
                    match m1814 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1815) = v13;
                                h1815
                            }), {
                                let (h1816, _) = v13;
                                h1816
                            })))
                        }
                    }
                } else {
                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1817 = v1;
                            let a1818 = 4u64;
                            nat_add(a1817, a1818)?
                        })))
                    } else {
                        if f37(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1819 = v1;
                                let a1820 = 5u64;
                                nat_add(a1819, a1820)?
                            })))
                        } else {
                            let a1821 = v5;
                            let a1822 = 91u64;
                            if nat_eq(a1821, a1822) {
                                let a1825 = f339(v0.clone(), {
                                    let a1823 = v1;
                                    let a1824 = 1u64;
                                    nat_add(a1823, a1824)?
                                })?;
                                let a1826 = 93u64;
                                if nat_eq(a1825, a1826) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1827 = v1;
                                        let a1828 = 2u64;
                                        nat_add(a1827, a1828)?
                                    })))
                                } else {
                                    let m1831 = f356(v0, {
                                        let a1829 = v1;
                                        let a1830 = 1u64;
                                        nat_add(a1829, a1830)?
                                    }, v4, 1u64)?;
                                    match m1831 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1832, _) = v14;
                                                h1832
                                            })), {
                                                let (_, h1833) = v14;
                                                h1833
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1834 = v5;
                                let a1835 = 123u64;
                                if nat_eq(a1834, a1835) {
                                    let a1838 = f339(v0.clone(), {
                                        let a1836 = v1;
                                        let a1837 = 1u64;
                                        nat_add(a1836, a1837)?
                                    })?;
                                    let a1839 = 125u64;
                                    if nat_eq(a1838, a1839) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1840 = v1;
                                            let a1841 = 2u64;
                                            nat_add(a1840, a1841)?
                                        })))
                                    } else {
                                        let m1844 = f356(v0, {
                                            let a1842 = v1;
                                            let a1843 = 1u64;
                                            nat_add(a1842, a1843)?
                                        }, v4, 2u64)?;
                                        match m1844 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1845, _) = v15;
                                                    h1845
                                                })), {
                                                    let (_, h1846) = v15;
                                                    h1846
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1847 = f428(v0, v1)?;
                                    match m1847 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1848) = v16;
                                                h1848
                                            }, {
                                                let (h1849, _) = v16;
                                                h1849
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

pub fn f357(v0: Bytes) -> R<bool> {
    let m1851 = {
        let a1850 = v0.clone();
        length_bytes(a1850)
    };
    if m1851 == 0 {
        Ok(true)
    } else {
        let v1 = m1851 - 1;
        let m1854 = {
            let a1852 = v0;
            let a1853 = v1;
            index_bytes(a1852, a1853)
        };
        match m1854 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f429({
                    let v3: u8 = v2;
                    let a1899 = {
                        let a1875 = {
                            let a1863 = {
                                let a1857 = {
                                    let a1855 = v3;
                                    let a1856 = 128u8;
                                    fixed_u8::bit_and(a1855, a1856)
                                };
                                let a1858 = 128u8;
                                if equal(a1857, a1858) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1864 = {
                                let a1861 = {
                                    let a1859 = v3;
                                    let a1860 = 64u8;
                                    fixed_u8::bit_and(a1859, a1860)
                                };
                                let a1862 = 64u8;
                                if equal(a1861, a1862) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1863, a1864)?
                        };
                        let a1876 = {
                            let a1873 = {
                                let a1867 = {
                                    let a1865 = v3;
                                    let a1866 = 32u8;
                                    fixed_u8::bit_and(a1865, a1866)
                                };
                                let a1868 = 32u8;
                                if equal(a1867, a1868) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1874 = {
                                let a1871 = {
                                    let a1869 = v3;
                                    let a1870 = 16u8;
                                    fixed_u8::bit_and(a1869, a1870)
                                };
                                let a1872 = 16u8;
                                if equal(a1871, a1872) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1873, a1874)?
                        };
                        nat_add(a1875, a1876)?
                    };
                    let a1900 = {
                        let a1897 = {
                            let a1885 = {
                                let a1879 = {
                                    let a1877 = v3;
                                    let a1878 = 8u8;
                                    fixed_u8::bit_and(a1877, a1878)
                                };
                                let a1880 = 8u8;
                                if equal(a1879, a1880) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1886 = {
                                let a1883 = {
                                    let a1881 = v3;
                                    let a1882 = 4u8;
                                    fixed_u8::bit_and(a1881, a1882)
                                };
                                let a1884 = 4u8;
                                if equal(a1883, a1884) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1885, a1886)?
                        };
                        let a1898 = {
                            let a1895 = {
                                let a1889 = {
                                    let a1887 = v3;
                                    let a1888 = 2u8;
                                    fixed_u8::bit_and(a1887, a1888)
                                };
                                let a1890 = 2u8;
                                if equal(a1889, a1890) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1896 = {
                                let a1893 = {
                                    let a1891 = v3;
                                    let a1892 = 1u8;
                                    fixed_u8::bit_and(a1891, a1892)
                                };
                                let a1894 = 1u8;
                                if equal(a1893, a1894) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1895, a1896)?
                        };
                        nat_add(a1897, a1898)?
                    };
                    nat_add(a1899, a1900)?
                }))
            }
        }
    }
}

pub fn f358() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f359(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1901 = f168(v0, v1);
    match m1901 {
        None => {
            false
        }
        Some(v3) => {
            f214(v3, v2)
        }
    }
}

pub fn f360() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f361(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt19, u64)>> {
    let m1902 = v2;
    if m1902 == 0 {
        Ok(None::<(Adt19, u64)>)
    } else {
        let v4 = m1902 - 1;
        let v5: u64 = f339(v0.clone(), v1)?;
        let a1903 = v3;
        let a1904 = 1u64;
        if nat_eq(a1903, a1904) {
            let m1905 = f361(v0.clone(), v1, v4, 0u64)?;
            match m1905 {
                None => {
                    Ok(None::<(Adt19, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f339(v0.clone(), {
                        let (_, h1906) = v6.clone();
                        h1906
                    })?;
                    let a1907 = v7;
                    let a1908 = 44u64;
                    if nat_eq(a1907, a1908) {
                        let m1912 = f361(v0, {
                            let a1910 = {
                                let (_, h1909) = v6;
                                h1909
                            };
                            let a1911 = 1u64;
                            nat_add(a1910, a1911)?
                        }, v4, 1u64)?;
                        match m1912 {
                            None => {
                                Ok(None::<(Adt19, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt19::C7(std::rc::Rc::new({
                                    let (h1913, _) = v6;
                                    h1913
                                }), std::rc::Rc::new({
                                    let (h1914, _) = v8;
                                    h1914
                                })), {
                                    let (_, h1915) = v8;
                                    h1915
                                })))
                            }
                        }
                    } else {
                        let a1916 = v7;
                        let a1917 = 93u64;
                        if nat_eq(a1916, a1917) {
                            Ok(Some((Adt19::C7(std::rc::Rc::new({
                                let (h1918, _) = v6;
                                h1918
                            }), std::rc::Rc::new(Adt19::C6)), {
                                let a1920 = {
                                    let (_, h1919) = v6;
                                    h1919
                                };
                                let a1921 = 1u64;
                                nat_add(a1920, a1921)?
                            })))
                        } else {
                            Ok(None::<(Adt19, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1922 = v3;
            let a1923 = 2u64;
            if nat_eq(a1922, a1923) {
                let m1926 = f427(v0.clone(), {
                    let a1924 = v1;
                    let a1925 = 1u64;
                    nat_add(a1924, a1925)?
                })?;
                match m1926 {
                    None => {
                        Ok(None::<(Adt19, u64)>)
                    }
                    Some(v9) => {
                        let a1927 = v5;
                        let a1928 = 34u64;
                        if if nat_eq(a1927, a1928) {
                            let a1930 = f339(v0.clone(), {
                                let (h1929, _) = v9.clone();
                                h1929
                            })?;
                            let a1931 = 58u64;
                            nat_eq(a1930, a1931)
                        } else {
                            false
                        } {
                            let m1935 = f361(v0.clone(), {
                                let a1933 = {
                                    let (h1932, _) = v9;
                                    h1932
                                };
                                let a1934 = 1u64;
                                nat_add(a1933, a1934)?
                            }, v4, 0u64)?;
                            match m1935 {
                                None => {
                                    Ok(None::<(Adt19, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f339(v0.clone(), {
                                        let (_, h1936) = v10.clone();
                                        h1936
                                    })?;
                                    let a1937 = v11;
                                    let a1938 = 44u64;
                                    if nat_eq(a1937, a1938) {
                                        let m1942 = f361(v0, {
                                            let a1940 = {
                                                let (_, h1939) = v10;
                                                h1939
                                            };
                                            let a1941 = 1u64;
                                            nat_add(a1940, a1941)?
                                        }, v4, 2u64)?;
                                        match m1942 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt19::C8({
                                                    let (_, h1943) = v9;
                                                    h1943
                                                }, std::rc::Rc::new({
                                                    let (h1944, _) = v10;
                                                    h1944
                                                }), std::rc::Rc::new({
                                                    let (h1945, _) = v12;
                                                    h1945
                                                })), {
                                                    let (_, h1946) = v12;
                                                    h1946
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1947 = v11;
                                        let a1948 = 125u64;
                                        if nat_eq(a1947, a1948) {
                                            Ok(Some((Adt19::C8({
                                                let (_, h1949) = v9;
                                                h1949
                                            }, std::rc::Rc::new({
                                                let (h1950, _) = v10;
                                                h1950
                                            }), std::rc::Rc::new(Adt19::C6)), {
                                                let a1952 = {
                                                    let (_, h1951) = v10;
                                                    h1951
                                                };
                                                let a1953 = 1u64;
                                                nat_add(a1952, a1953)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt19, u64)>)
                        }
                    }
                }
            } else {
                let a1954 = v5;
                let a1955 = 34u64;
                if nat_eq(a1954, a1955) {
                    let m1958 = f427(v0, {
                        let a1956 = v1;
                        let a1957 = 1u64;
                        nat_add(a1956, a1957)?
                    })?;
                    match m1958 {
                        None => {
                            Ok(None::<(Adt19, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt19::C3({
                                let (_, h1959) = v13;
                                h1959
                            }), {
                                let (h1960, _) = v13;
                                h1960
                            })))
                        }
                    }
                } else {
                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt19::C0(true), {
                            let a1961 = v1;
                            let a1962 = 4u64;
                            nat_add(a1961, a1962)?
                        })))
                    } else {
                        if f37(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt19::C0(false), {
                                let a1963 = v1;
                                let a1964 = 5u64;
                                nat_add(a1963, a1964)?
                            })))
                        } else {
                            let a1965 = v5;
                            let a1966 = 91u64;
                            if nat_eq(a1965, a1966) {
                                let a1969 = f339(v0.clone(), {
                                    let a1967 = v1;
                                    let a1968 = 1u64;
                                    nat_add(a1967, a1968)?
                                })?;
                                let a1970 = 93u64;
                                if nat_eq(a1969, a1970) {
                                    Ok(Some((Adt19::C4(std::rc::Rc::new(Adt19::C6)), {
                                        let a1971 = v1;
                                        let a1972 = 2u64;
                                        nat_add(a1971, a1972)?
                                    })))
                                } else {
                                    let m1975 = f361(v0, {
                                        let a1973 = v1;
                                        let a1974 = 1u64;
                                        nat_add(a1973, a1974)?
                                    }, v4, 1u64)?;
                                    match m1975 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt19::C4(std::rc::Rc::new({
                                                let (h1976, _) = v14;
                                                h1976
                                            })), {
                                                let (_, h1977) = v14;
                                                h1977
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1978 = v5;
                                let a1979 = 123u64;
                                if nat_eq(a1978, a1979) {
                                    let a1982 = f339(v0.clone(), {
                                        let a1980 = v1;
                                        let a1981 = 1u64;
                                        nat_add(a1980, a1981)?
                                    })?;
                                    let a1983 = 125u64;
                                    if nat_eq(a1982, a1983) {
                                        Ok(Some((Adt19::C5(std::rc::Rc::new(Adt19::C6)), {
                                            let a1984 = v1;
                                            let a1985 = 2u64;
                                            nat_add(a1984, a1985)?
                                        })))
                                    } else {
                                        let m1988 = f361(v0, {
                                            let a1986 = v1;
                                            let a1987 = 1u64;
                                            nat_add(a1986, a1987)?
                                        }, v4, 2u64)?;
                                        match m1988 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt19::C5(std::rc::Rc::new({
                                                    let (h1989, _) = v15;
                                                    h1989
                                                })), {
                                                    let (_, h1990) = v15;
                                                    h1990
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1991 = f430(v0, v1)?;
                                    match m1991 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1992) = v16;
                                                h1992
                                            }, {
                                                let (h1993, _) = v16;
                                                h1993
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

pub fn f362(v0: Adt19, v1: Bytes) -> bool {
    let a1995 = {
        let a1994 = f159(v0, v1);
        length_bytes(a1994)
    };
    let a1996 = 0u64;
    !nat_eq(a1995, a1996)
}

pub fn f363(v0: Adt19, v1: Bytes) -> bool {
    let m1997 = f156(v0, v1);
    match m1997 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f364(v0: Bytes, p2064: u64, p2065: u64, p2066: bool, p2067: Bytes) -> R<Bytes> {
    let m2063 = (p2064, (p2065, (p2066, p2067)));
    {
        let mut m2063 = m2063;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m2063;
                let m1998 = v2;
                if m1998 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1998 - 1;
                    let m2001 = {
                        let a1999 = v0.clone();
                        let a2000 = v1;
                        index_bytes(a1999, a2000)
                    };
                    match m2001 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a2046 = {
                                    let a2022 = {
                                        let a2010 = {
                                            let a2004 = {
                                                let a2002 = v7;
                                                let a2003 = 128u8;
                                                fixed_u8::bit_and(a2002, a2003)
                                            };
                                            let a2005 = 128u8;
                                            if equal(a2004, a2005) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2011 = {
                                            let a2008 = {
                                                let a2006 = v7;
                                                let a2007 = 64u8;
                                                fixed_u8::bit_and(a2006, a2007)
                                            };
                                            let a2009 = 64u8;
                                            if equal(a2008, a2009) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2010, a2011)?
                                    };
                                    let a2023 = {
                                        let a2020 = {
                                            let a2014 = {
                                                let a2012 = v7;
                                                let a2013 = 32u8;
                                                fixed_u8::bit_and(a2012, a2013)
                                            };
                                            let a2015 = 32u8;
                                            if equal(a2014, a2015) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2021 = {
                                            let a2018 = {
                                                let a2016 = v7;
                                                let a2017 = 16u8;
                                                fixed_u8::bit_and(a2016, a2017)
                                            };
                                            let a2019 = 16u8;
                                            if equal(a2018, a2019) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2020, a2021)?
                                    };
                                    nat_add(a2022, a2023)?
                                };
                                let a2047 = {
                                    let a2044 = {
                                        let a2032 = {
                                            let a2026 = {
                                                let a2024 = v7;
                                                let a2025 = 8u8;
                                                fixed_u8::bit_and(a2024, a2025)
                                            };
                                            let a2027 = 8u8;
                                            if equal(a2026, a2027) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2033 = {
                                            let a2030 = {
                                                let a2028 = v7;
                                                let a2029 = 4u8;
                                                fixed_u8::bit_and(a2028, a2029)
                                            };
                                            let a2031 = 4u8;
                                            if equal(a2030, a2031) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2032, a2033)?
                                    };
                                    let a2045 = {
                                        let a2042 = {
                                            let a2036 = {
                                                let a2034 = v7;
                                                let a2035 = 2u8;
                                                fixed_u8::bit_and(a2034, a2035)
                                            };
                                            let a2037 = 2u8;
                                            if equal(a2036, a2037) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2043 = {
                                            let a2040 = {
                                                let a2038 = v7;
                                                let a2039 = 1u8;
                                                fixed_u8::bit_and(a2038, a2039)
                                            };
                                            let a2041 = 1u8;
                                            if equal(a2040, a2041) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2042, a2043)?
                                    };
                                    nat_add(a2044, a2045)?
                                };
                                nat_add(a2046, a2047)?
                            };
                            if f431(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a2048 = v1;
                                    let a2049 = 1u64;
                                    nat_add(a2048, a2049)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f432(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a2050 = v1;
                                        let a2051 = 1u64;
                                        nat_add(a2050, a2051)?
                                    }, (v5, (false, {
                                        let a2055 = v4;
                                        let a2056 = f32({
                                            let a2052 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a2052)
                                        }, {
                                            let a2053 = v8;
                                            let a2054 = 97u64;
                                            nat_sub(a2053, a2054)
                                        }, 1u64);
                                        append_bytes(a2055, a2056)
                                    }))))
                                } else {
                                    if if f432(v8) {
                                        true
                                    } else {
                                        if f433(v8) {
                                            true
                                        } else {
                                            f434(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2057 = v1;
                                            let a2058 = 1u64;
                                            nat_add(a2057, a2058)?
                                        }, (v5, (false, {
                                            let a2059 = v4;
                                            let a2060 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a2059, a2060)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2061 = v1;
                                            let a2062 = 1u64;
                                            nat_add(a2061, a2062)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f365(v0: Adt19, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt19)> = f435(f436(f156(v0.clone(), f8(Str::lit("$defs")))), f436(f156(v0.clone(), f8(Str::lit("definitions")))));
    let m2068 = f437(4096u64, v3.clone(), List::<Bytes>::nil(), f438(v3)?)?;
    match m2068 {
        Ok(v5) => {
            let m2069 = f439(4096u64, f440(f441(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m2069 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f226(f197(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f366(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m2070 = v0;
    match m2070.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f37(f166(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f442(v5, v1, v2))
            } else {
                List::cons(v4, f366(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f367(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f226(f443(f87(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f368(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a2071 = 0u64;
        let a2072 = v1;
        nat_lt(a2071, a2072)
    } else {
        false
    } {
        let a2073 = Bytes::lit(&[0x2d]);
        let a2074 = f131(v1)?;
        Ok(append_bytes(a2073, a2074))
    } else {
        f131(v1)
    }
}

pub fn f369(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f131(v1)?;
    let v4: u64 = {
        let a2075 = v3.clone();
        length_bytes(a2075)
    };
    let v5: Bytes = {
        let a2076 = v2;
        let a2077 = v4;
        if nat_lt(a2076, a2077) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a2078 = v4;
                let a2079 = v2;
                nat_sub(a2078, a2079)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a2080 = v4;
                let a2081 = v2;
                nat_sub(a2080, a2081)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f444({
                let a2082 = v2;
                let a2083 = v4;
                nat_sub(a2082, a2083)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a2084 = f8(Str::lit("-"));
        let a2085 = v5;
        Ok(append_bytes(a2084, a2085))
    } else {
        Ok(v5)
    }
}

pub fn f370(v0: Bytes) -> R<Bytes> {
    let a2091 = {
        let a2089 = Bytes::lit(&[0x22]);
        let a2090 = {
            let (_, h2086) = {
                let (h2087, _) = f330(Fn5::F445(v0.clone()), {
                    let a2088 = v0;
                    length_bytes(a2088)
                }, (0u64, Bytes::lit(&[])))?;
                h2087
            };
            h2086
        };
        append_bytes(a2089, a2090)
    };
    let a2092 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2091, a2092))
}

pub fn f371(v0: List<Bytes>) -> Bytes {
    let a2095 = {
        let a2093 = Bytes::lit(&[0x5b]);
        let a2094 = f126(v0, Bytes::lit(&[0x2c]));
        append_bytes(a2093, a2094)
    };
    let a2096 = Bytes::lit(&[0x5d]);
    append_bytes(a2095, a2096)
}

pub fn f372(v0: Adt19) -> R<List<Bytes>> {
    let m2097 = v0;
    match m2097 {
        Adt19::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C7(r2098, r2099) => {
            let v10 = (*r2098).clone();
            let v11 = (*r2099).clone();
            Ok(List::cons(f268(v10)?, f372(v11)?))
        }
        Adt19::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f373(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f446(Fn10::F447, List::<(Bytes, Bytes)>::nil(), f448(v0)?);
    let v2: List<Bytes> = f449(Fn11::F450, List::<Bytes>::nil(), v1)?;
    let a2102 = {
        let a2100 = Bytes::lit(&[0x7b]);
        let a2101 = f126(v2, Bytes::lit(&[0x2c]));
        append_bytes(a2100, a2101)
    };
    let a2103 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a2102, a2103))
}

pub fn f374(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m2104 = v0;
    match m2104 {
        Adt19::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C8(v12, r2105, r2106) => {
            let v13 = (*r2105).clone();
            let v14 = (*r2106).clone();
            Ok(List::cons((v12, f268(v13)?), f374(v14)?))
        }
    }
}

pub fn f375(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m2107 = v0;
    match m2107 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f166(v2, v1)
        }
    }
}

pub fn f376(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f226(f451(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f377(v0: Bytes) -> R<bool> {
    let m2108 = f180(v0.clone(), f8(Str::lit("&&")))?;
    if match m2108 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2109 = f180(v0, f8(Str::lit("||")))?;
        match m2109 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f378(v0: Adt16, v1: Option<u64>) -> bool {
    let m2110 = v1;
    match m2110 {
        None => {
            true
        }
        Some(v2) => {
            f289(v2, match v0 {
                Adt16::C0(_, h2111, _, _) => {
                    h2111
                }
            })
        }
    }
}

pub fn f379(v0: Adt12) -> Adt21 {
    Adt21::C1({
        let a2113 = match v0 {
            Adt12::C0(_, _, h2112, _, _) => {
                h2112
            }
        };
        length_list(a2113)
    })
}

pub fn f380(v0: Bytes) -> Option<Adt22> {
    if f37(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt22::C0)
    } else {
        if f37(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt22::C1)
        } else {
            if f37(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt22::C2)
            } else {
                None::<Adt22>
            }
        }
    }
}

pub fn f381(v0: Bytes, p2118: List<Bytes>, p2119: u64) -> R<Option<u64>> {
    let m2117 = (p2118, p2119);
    {
        let mut m2117 = m2117;
        loop {
            let step = {
                let (v1, v2) = m2117;
                let m2114 = v1;
                match m2114.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f37(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2115 = v2;
                                let a2116 = 1u64;
                                nat_add(a2115, a2116)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382(v0: Option<Adt14>) -> Option<u64> {
    let m2120 = v0;
    match m2120 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2121 = v1;
            match m2121 {
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

pub fn f383(v0: Fn6, p2125: List<Bytes>, p2126: List<(Bytes, ())>) -> List<Bytes> {
    let m2124 = (p2125, p2126);
    {
        let mut m2124 = m2124;
        loop {
            let step = {
                let (v1, v2) = m2124;
                let m2122 = v2;
                match m2122.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c2123: Fn6 = v0.clone();
                            c2123.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2124 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h2127, _) = v1;
        h2127
    }, v0)
}

pub fn f385(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f452(Fn12::F453, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f386(v0: Fn7, p2131: List<List<(Bytes, ())>>, p2132: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m2130 = (p2131, p2132);
    {
        let mut m2130 = m2130;
        loop {
            let step = {
                let (v1, v2) = m2130;
                let m2128 = v2;
                match m2128.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c2129: Fn7 = v0.clone();
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

pub fn f387(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f388(v0: Fn8, p2137: u64, p2138: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m2136 = (p2137, p2138);
    {
        let mut m2136 = m2136;
        loop {
            let step = {
                let (v1, v2) = m2136;
                let m2133 = v1;
                if m2133 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2133 - 1;
                    let m2135 = {
                        let c2134: Fn8 = v0.clone();
                        c2134.apply(v2.clone())?
                    };
                    match m2135 {
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
                Ok(next) => { m2136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f389(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f454(v0)
}

pub fn f390(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f455(Fn13::F456, List::<(Bytes, ())>::nil(), v0)
}

pub fn f391(v0: Fn9, p2142: List<(Bytes, ())>, p2143: List<Bytes>) -> List<(Bytes, ())> {
    let m2141 = (p2142, p2143);
    {
        let mut m2141 = m2141;
        loop {
            let step = {
                let (v1, v2) = m2141;
                let m2139 = v2;
                match m2139.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c2140: Fn9 = v0.clone();
                            c2140.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f393(v0: Bytes) -> Bytes {
    if f143(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a2145 = {
                let a2144 = v0;
                length_bytes(a2144)
            };
            let a2146 = 1u64;
            nat_sub(a2145, a2146)
        })
    } else {
        v0
    }
}

pub fn f394(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f87(f74(v0.clone(), v1.clone()))?;
    let m2147 = f162(v2.clone())?;
    match m2147 {
        None => {
            f458(v0, v1, v2)
        }
        Some(v3) => {
            f457(v0, v1, v3)
        }
    }
}

pub fn f395(v0: List<Bytes>) -> List<Bytes> {
    let m2148 = v0;
    match m2148.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a2150 = {
                    let a2149 = v1.clone();
                    length_bytes(a2149)
                };
                let a2151 = 0u64;
                if nat_eq(a2150, a2151) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f395(v2))
        }
    }
}

pub fn f396(v0: List<Bytes>) -> Bytes {
    let m2152 = v0;
    match m2152.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f397(v0: (Bytes, Bytes), p2155: List<(Bytes, Bytes)>) -> bool {
    let m2154 = p2155;
    {
        let mut m2154 = m2154;
        loop {
            let step = {
                let v1 = m2154;
                let m2153 = v1;
                match m2153.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f459(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m2156 = v1;
    match m2156.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37({
                let (h2157, _) = v2;
                h2157
            }, v0.clone()) {
                List::cons({
                    let (_, h2158) = v2;
                    h2158
                }, f398(v0, v3))
            } else {
                f398(v0, v3)
            }
        }
    }
}

pub fn f399(v0: u32) -> Bytes {
    let a2161 = {
        let a2159 = v0;
        let a2160 = 8u32;
        fixed_u32::bit_and(a2159, a2160)
    };
    let a2162 = 0u32;
    if equal(a2161, a2162) {
        let a2165 = {
            let a2163 = v0;
            let a2164 = 4u32;
            fixed_u32::bit_and(a2163, a2164)
        };
        let a2166 = 0u32;
        if equal(a2165, a2166) {
            let a2169 = {
                let a2167 = v0;
                let a2168 = 2u32;
                fixed_u32::bit_and(a2167, a2168)
            };
            let a2170 = 0u32;
            if equal(a2169, a2170) {
                let a2173 = {
                    let a2171 = v0;
                    let a2172 = 1u32;
                    fixed_u32::bit_and(a2171, a2172)
                };
                let a2174 = 0u32;
                if equal(a2173, a2174) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2177 = {
                    let a2175 = v0;
                    let a2176 = 1u32;
                    fixed_u32::bit_and(a2175, a2176)
                };
                let a2178 = 0u32;
                if equal(a2177, a2178) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2181 = {
                let a2179 = v0;
                let a2180 = 2u32;
                fixed_u32::bit_and(a2179, a2180)
            };
            let a2182 = 0u32;
            if equal(a2181, a2182) {
                let a2185 = {
                    let a2183 = v0;
                    let a2184 = 1u32;
                    fixed_u32::bit_and(a2183, a2184)
                };
                let a2186 = 0u32;
                if equal(a2185, a2186) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2189 = {
                    let a2187 = v0;
                    let a2188 = 1u32;
                    fixed_u32::bit_and(a2187, a2188)
                };
                let a2190 = 0u32;
                if equal(a2189, a2190) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2193 = {
            let a2191 = v0;
            let a2192 = 4u32;
            fixed_u32::bit_and(a2191, a2192)
        };
        let a2194 = 0u32;
        if equal(a2193, a2194) {
            let a2197 = {
                let a2195 = v0;
                let a2196 = 2u32;
                fixed_u32::bit_and(a2195, a2196)
            };
            let a2198 = 0u32;
            if equal(a2197, a2198) {
                let a2201 = {
                    let a2199 = v0;
                    let a2200 = 1u32;
                    fixed_u32::bit_and(a2199, a2200)
                };
                let a2202 = 0u32;
                if equal(a2201, a2202) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2205 = {
                    let a2203 = v0;
                    let a2204 = 1u32;
                    fixed_u32::bit_and(a2203, a2204)
                };
                let a2206 = 0u32;
                if equal(a2205, a2206) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2209 = {
                let a2207 = v0;
                let a2208 = 2u32;
                fixed_u32::bit_and(a2207, a2208)
            };
            let a2210 = 0u32;
            if equal(a2209, a2210) {
                let a2213 = {
                    let a2211 = v0;
                    let a2212 = 1u32;
                    fixed_u32::bit_and(a2211, a2212)
                };
                let a2214 = 0u32;
                if equal(a2213, a2214) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2217 = {
                    let a2215 = v0;
                    let a2216 = 1u32;
                    fixed_u32::bit_and(a2215, a2216)
                };
                let a2218 = 0u32;
                if equal(a2217, a2218) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f400(v0: u32, v1: u32) -> u32 {
    let m2221 = {
        let a2219 = v0;
        let a2220 = v1;
        fixed_u32::shift_right(a2219, a2220)
    };
    match m2221 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f401(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f460(Fn14::F461, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2222, _, _, _, _, _, _, _) => {
            h2222
        }
    }, match v0.clone() {
        Adt24::C0(_, h2223, _, _, _, _, _, _) => {
            h2223
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2224, _, _, _, _, _) => {
            h2224
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2225, _, _, _, _) => {
            h2225
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2226, _, _, _) => {
            h2226
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2227, _, _) => {
            h2227
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2228, _) => {
            h2228
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2229) => {
            h2229
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f462(match v0.clone() {
        Adt24::C0(h2230, _, _, _, _, _, _, _) => {
            h2230
        }
    }, match v2.clone() {
        Adt27::C0(h2231, _, _, _, _, _, _, _, _, _) => {
            h2231
        }
    }), f462(match v0.clone() {
        Adt24::C0(_, h2232, _, _, _, _, _, _) => {
            h2232
        }
    }, match v2.clone() {
        Adt27::C0(_, h2233, _, _, _, _, _, _, _, _) => {
            h2233
        }
    }), f462(match v0.clone() {
        Adt24::C0(_, _, h2234, _, _, _, _, _) => {
            h2234
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2235, _, _, _, _, _, _, _) => {
            h2235
        }
    }), f462(match v0.clone() {
        Adt24::C0(_, _, _, h2236, _, _, _, _) => {
            h2236
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2237, _, _, _, _, _, _) => {
            h2237
        }
    }), f462(match v0.clone() {
        Adt24::C0(_, _, _, _, h2238, _, _, _) => {
            h2238
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2239, _, _, _, _, _) => {
            h2239
        }
    }), f462(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2240, _, _) => {
            h2240
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2241, _, _, _, _) => {
            h2241
        }
    }), f462(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2242, _) => {
            h2242
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2243, _, _, _) => {
            h2243
        }
    }), f462(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2244) => {
            h2244
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2245, _, _) => {
            h2245
        }
    })))
}

pub fn f402(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f463(v0.clone(), v1, v2, v3)?, f463(v0.clone(), v1, v2, {
        let a2246 = v3;
        let a2247 = 4u64;
        nat_add(a2246, a2247)?
    })?, f463(v0.clone(), v1, v2, {
        let a2248 = v3;
        let a2249 = 8u64;
        nat_add(a2248, a2249)?
    })?, f463(v0.clone(), v1, v2, {
        let a2250 = v3;
        let a2251 = 12u64;
        nat_add(a2250, a2251)?
    })?, f463(v0.clone(), v1, v2, {
        let a2252 = v3;
        let a2253 = 16u64;
        nat_add(a2252, a2253)?
    })?, f463(v0.clone(), v1, v2, {
        let a2254 = v3;
        let a2255 = 20u64;
        nat_add(a2254, a2255)?
    })?, f463(v0.clone(), v1, v2, {
        let a2256 = v3;
        let a2257 = 24u64;
        nat_add(a2256, a2257)?
    })?, f463(v0.clone(), v1, v2, {
        let a2258 = v3;
        let a2259 = 28u64;
        nat_add(a2258, a2259)?
    })?, f463(v0.clone(), v1, v2, {
        let a2260 = v3;
        let a2261 = 32u64;
        nat_add(a2260, a2261)?
    })?, f463(v0.clone(), v1, v2, {
        let a2262 = v3;
        let a2263 = 36u64;
        nat_add(a2262, a2263)?
    })?, f463(v0.clone(), v1, v2, {
        let a2264 = v3;
        let a2265 = 40u64;
        nat_add(a2264, a2265)?
    })?, f463(v0.clone(), v1, v2, {
        let a2266 = v3;
        let a2267 = 44u64;
        nat_add(a2266, a2267)?
    })?, f463(v0.clone(), v1, v2, {
        let a2268 = v3;
        let a2269 = 48u64;
        nat_add(a2268, a2269)?
    })?, f463(v0.clone(), v1, v2, {
        let a2270 = v3;
        let a2271 = 52u64;
        nat_add(a2270, a2271)?
    })?, f463(v0.clone(), v1, v2, {
        let a2272 = v3;
        let a2273 = 56u64;
        nat_add(a2272, a2273)?
    })?, f463(v0, v1, v2, {
        let a2274 = v3;
        let a2275 = 60u64;
        nat_add(a2274, a2275)?
    })?))
}

pub fn f403(p2278: List<Bytes>, v1: Bytes) -> bool {
    let m2277 = p2278;
    {
        let mut m2277 = m2277;
        loop {
            let step = {
                let v0 = m2277;
                let m2276 = v0;
                match m2276.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f216(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: Bytes, v1: u64) -> Option<u64> {
    let a2279 = 0u64;
    let a2280 = v1;
    if nat_lt(a2279, a2280) {
        if f464(v0, {
            let a2281 = v1;
            let a2282 = 1u64;
            nat_sub(a2281, a2282)
        }) {
            Some({
                let a2283 = v1;
                let a2284 = 1u64;
                nat_sub(a2283, a2284)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f405(v0: Adt14) -> R<List<Bytes>> {
    let m2285 = v0;
    match m2285 {
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
        Adt14::C6(r2286, r2287) => {
            let v7 = (*r2286).clone();
            let v8 = (*r2287).clone();
            Ok(List::cons(f329(v7)?, f405(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f406(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m2288 = v0;
    match m2288 {
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
        Adt14::C7(v9, r2289, r2290) => {
            let v10 = (*r2289).clone();
            let v11 = (*r2290).clone();
            Ok(List::cons((v9, f329(v10)?), f406(v11)?))
        }
    }
}

pub fn f407(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h2291, _) = v1;
        h2291
    };
    let a2293 = {
        let a2292 = v0.clone();
        length_bytes(a2292)
    };
    let a2294 = v2;
    if nat_le(a2293, a2294) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a2295 = f238(v0.clone(), v2)?;
        let a2296 = 13u64;
        if nat_eq(a2295, a2296) {
            Ok(Some(({
                let a2299 = f238(v0, {
                    let a2297 = v2;
                    let a2298 = 1u64;
                    nat_add(a2297, a2298)?
                })?;
                let a2300 = 10u64;
                if nat_eq(a2299, a2300) {
                    let a2301 = v2;
                    let a2302 = 2u64;
                    nat_add(a2301, a2302)?
                } else {
                    let a2303 = v2;
                    let a2304 = 1u64;
                    nat_add(a2303, a2304)?
                }
            }, {
                let a2306 = {
                    let (_, h2305) = v1;
                    h2305
                };
                let a2307 = Bytes::lit(&[0x0a]);
                append_bytes(a2306, a2307)
            })))
        } else {
            let v4: u64 = {
                let m2308 = f334(v0.clone(), Bytes::lit(&[0x0d]), v2)?;
                match m2308 {
                    None => {
                        let a2309 = v0.clone();
                        length_bytes(a2309)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2313 = {
                    let (_, h2310) = v1;
                    h2310
                };
                let a2314 = f32(v0, v2, {
                    let a2311 = v4;
                    let a2312 = v2;
                    nat_sub(a2311, a2312)
                });
                append_bytes(a2313, a2314)
            })))
        }
    }
}

pub fn f408(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2315 = f465(v0, v1)?;
    match m2315 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f466({
                let (h2316, _) = v2;
                h2316
            }) {
                Ok(Some({
                    let a2318 = v1;
                    let a2319 = {
                        let (_, h2317) = v2;
                        h2317
                    };
                    nat_add(a2318, a2319)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f409(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f467(v1, v0, v2)
}

pub fn f410(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f468(v0, v1)
}

pub fn f411(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f469(v0, v1, v2)
}

pub fn f412(v0: Bytes, p2322: List<Bytes>) -> bool {
    let m2321 = p2322;
    {
        let mut m2321 = m2321;
        loop {
            let step = {
                let v1 = m2321;
                let m2320 = v1;
                match m2320.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f37(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f237(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2325 = f238(v0.clone(), {
                let a2323 = v1;
                let a2324 = 2u64;
                nat_add(a2323, a2324)?
            })?;
            let a2326 = 120u64;
            nat_eq(a2325, a2326)
        };
        let v3: u64 = if v2 {
            let a2327 = v1;
            let a2328 = 3u64;
            nat_add(a2327, a2328)?
        } else {
            let a2329 = v1;
            let a2330 = 2u64;
            nat_add(a2329, a2330)?
        };
        let v4: (u64, u64) = f470(v0.clone(), v2, v3)?;
        let a2332 = {
            let (h2331, _) = v4;
            h2331
        };
        let a2333 = v3;
        if if nat_eq(a2332, a2333) {
            true
        } else {
            let a2335 = f238(v0, {
                let (h2334, _) = v4;
                h2334
            })?;
            let a2336 = 59u64;
            !nat_eq(a2335, a2336)
        } {
            Ok(Err::<(Bytes, u64), Bytes>(f234(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f466({
                let (_, h2337) = v4;
                h2337
            }) {
                Ok(Ok::<(Bytes, u64), Bytes>((f471({
                    let (_, h2338) = v4;
                    h2338
                })?, {
                    let a2340 = {
                        let (h2339, _) = v4;
                        h2339
                    };
                    let a2341 = 1u64;
                    nat_add(a2340, a2341)?
                })))
            } else {
                Ok(Err::<(Bytes, u64), Bytes>(f234(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f237(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3c]), {
                let a2342 = v1;
                let a2343 = 4u64;
                nat_add(a2342, a2343)?
            })))
        } else {
            if f237(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3e]), {
                    let a2344 = v1;
                    let a2345 = 4u64;
                    nat_add(a2344, a2345)?
                })))
            } else {
                if f237(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x26]), {
                        let a2346 = v1;
                        let a2347 = 5u64;
                        nat_add(a2346, a2347)?
                    })))
                } else {
                    if f237(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x27]), {
                            let a2348 = v1;
                            let a2349 = 6u64;
                            nat_add(a2348, a2349)?
                        })))
                    } else {
                        if f237(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x22]), {
                                let a2350 = v1;
                                let a2351 = 6u64;
                                nat_add(a2350, a2351)?
                            })))
                        } else {
                            Ok(Err::<(Bytes, u64), Bytes>(f234(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f414(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2352, _) = f280(Fn3::F472(v1, v2, v0.clone()), {
        let a2354 = {
            let a2353 = v0;
            length_bytes(a2353)
        };
        let a2355 = 1u64;
        nat_add(a2354, a2355)?
    }, v3)?;
    Ok(h2352)
}

pub fn f415(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2356, _) = v2;
        h2356
    };
    let a2357 = v3;
    let a2358 = 0u64;
    if nat_eq(a2357, a2358) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2359 = v3;
            let a2360 = v1;
            let a2361 = 0u64;
            nat_quot(a2359, a2360, a2361)
        }, {
            let a2366 = f343(v0, {
                let a2362 = v3;
                let a2363 = v1;
                let a2364 = 0u64;
                nat_rem(a2362, a2363, a2364)
            });
            let a2367 = {
                let (_, h2365) = v2;
                h2365
            };
            append_bytes(a2366, a2367)
        }))
    }
}

pub fn f416(p2378: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>> {
    let m2377 = p2378;
    {
        let mut m2377 = m2377;
        loop {
            let step = {
                let v0 = m2377;
                let m2368 = v0;
                match m2368 {
                    Adt18::C0(_, _, _) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C1(_, _, _) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C3(r2375, r2376) => {
                        let v2 = (*r2375).clone();
                        let v3 = (*r2376).clone();
                        let m2369 = v2;
                        match m2369 {
                            Adt18::C0(v4, r2373, r2374) => {
                                let v5 = (*r2373).clone();
                                let v6 = (*r2374).clone();
                                let m2370 = f242(v5.clone(), v1.clone());
                                match m2370 {
                                    Ok(v8) => {
                                        let m2371 = f243(v8.clone(), v4)?;
                                        match m2371 {
                                            Ok(v10) => {
                                                if if f244(v10.clone(), Str::lit("restriction")) {
                                                    true
                                                } else {
                                                    if f244(v10.clone(), Str::lit("list")) {
                                                        true
                                                    } else {
                                                        f244(v10.clone(), Str::lit("union"))
                                                    }
                                                } {
                                                    Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(Some(({
                                                        let (_, h2372) = v10;
                                                        h2372
                                                    }, (v5, (v6, v8)))))))
                                                } else {
                                                    Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Err::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Err::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(v7)))
                                    }
                                }
                            }
                            Adt18::C1(_, _, _) => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt18::C2(_) => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt18::C3(_, _) => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt18::C4 => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt18::C4 => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                }
            };
            match step {
                Ok(next) => { m2377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f418(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m2379 = f243(v0, v2.clone())?;
    match m2379 {
        Ok(v4) => {
            if f37({
                let (h2380, _) = v4.clone();
                h2380
            }, f350()) {
                let m2382 = f473({
                    let (_, h2381) = v4;
                    h2381
                }, 16u64);
                match m2382 {
                    Ok(v6) => {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v5))
                    }
                }
            } else {
                if f37({
                    let (h2383, _) = v4.clone();
                    h2383
                }, v1) {
                    let v7: Bytes = f265({
                        let (_, h2384) = v4;
                        h2384
                    })?;
                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h2385, _) = v4;
                        h2385
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(Bytes, List<Bytes>), Bytes>(v3))
        }
    }
}

pub fn f419(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f420(v0: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt19>, Bytes>> {
    let m2386 = v0;
    match m2386 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C3(r2393, r2394) => {
            let v2 = (*r2393).clone();
            let v3 = (*r2394).clone();
            let m2387 = f420(v3, v1.clone())?;
            match m2387 {
                Ok(v5) => {
                    let m2388 = v2;
                    match m2388 {
                        Adt18::C0(v6, r2392, _) => {
                            let v7 = (*r2392).clone();
                            let m2389 = f242(v7.clone(), v1);
                            match m2389 {
                                Ok(v10) => {
                                    let m2390 = f243(v10, v6)?;
                                    match m2390 {
                                        Ok(v12) => {
                                            if f244(v12, Str::lit("enumeration")) {
                                                let m2391 = f245(v7, f8(Str::lit("value")));
                                                match m2391 {
                                                    None => {
                                                        Ok(Err::<List<Adt19>, Bytes>(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt19>, Bytes>(List::cons(Adt19::C3(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt19>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(Err::<List<Adt19>, Bytes>(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<List<Adt19>, Bytes>(v9))
                                }
                            }
                        }
                        Adt18::C1(_, _, _) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt18::C2(_) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt18::C3(_, _) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt18::C4 => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt19>, Bytes>(v4))
                }
            }
        }
        Adt18::C4 => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
    }
}

pub fn f421(v0: List<Adt19>, v1: List<Bytes>) -> R<List<Adt19>> {
    let m2395 = v0;
    match m2395.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f474(v2.clone())?;
            if f90(v4.clone(), v1.clone()) {
                f421(v3, v1)
            } else {
                Ok(List::cons(v2, f421(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f422(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2396 = v1;
    match m2396.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2397 = f475(v0, v3.clone())?;
            match m2397 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f474(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f90(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2398 = f422(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2398 {
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

pub fn f423(p2402: List<Bytes>) -> R<bool> {
    let m2401 = p2402;
    {
        let mut m2401 = m2401;
        loop {
            let step = {
                let v0 = m2401;
                let m2399 = v0;
                match m2399.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2400 = f476(v1)?;
                        match m2400 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f90(v3, f477(false, v2.clone())?) {
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
                Ok(next) => { m2401 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f424(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f226(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f425(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a2403 = v2;
    let a2404 = v0;
    if nat_lt(a2403, a2404) {
        if f464(v1, v2) {
            Ok(Some({
                let a2405 = v2;
                let a2406 = 1u64;
                nat_add(a2405, a2406)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f426(v0: Bytes, p2486: u64, p2487: u64, p2488: bool, p2489: bool, p2490: Bytes) -> R<Bytes> {
    let m2485 = (p2486, (p2487, (p2488, (p2489, p2490))));
    {
        let mut m2485 = m2485;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m2485;
                let m2407 = v2;
                if m2407 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m2407 - 1;
                    let m2410 = {
                        let a2408 = v0.clone();
                        let a2409 = v1;
                        index_bytes(a2408, a2409)
                    };
                    match m2410 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a2455 = {
                                    let a2431 = {
                                        let a2419 = {
                                            let a2413 = {
                                                let a2411 = v8;
                                                let a2412 = 128u8;
                                                fixed_u8::bit_and(a2411, a2412)
                                            };
                                            let a2414 = 128u8;
                                            if equal(a2413, a2414) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2420 = {
                                            let a2417 = {
                                                let a2415 = v8;
                                                let a2416 = 64u8;
                                                fixed_u8::bit_and(a2415, a2416)
                                            };
                                            let a2418 = 64u8;
                                            if equal(a2417, a2418) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2419, a2420)?
                                    };
                                    let a2432 = {
                                        let a2429 = {
                                            let a2423 = {
                                                let a2421 = v8;
                                                let a2422 = 32u8;
                                                fixed_u8::bit_and(a2421, a2422)
                                            };
                                            let a2424 = 32u8;
                                            if equal(a2423, a2424) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2430 = {
                                            let a2427 = {
                                                let a2425 = v8;
                                                let a2426 = 16u8;
                                                fixed_u8::bit_and(a2425, a2426)
                                            };
                                            let a2428 = 16u8;
                                            if equal(a2427, a2428) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2429, a2430)?
                                    };
                                    nat_add(a2431, a2432)?
                                };
                                let a2456 = {
                                    let a2453 = {
                                        let a2441 = {
                                            let a2435 = {
                                                let a2433 = v8;
                                                let a2434 = 8u8;
                                                fixed_u8::bit_and(a2433, a2434)
                                            };
                                            let a2436 = 8u8;
                                            if equal(a2435, a2436) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2442 = {
                                            let a2439 = {
                                                let a2437 = v8;
                                                let a2438 = 4u8;
                                                fixed_u8::bit_and(a2437, a2438)
                                            };
                                            let a2440 = 4u8;
                                            if equal(a2439, a2440) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2441, a2442)?
                                    };
                                    let a2454 = {
                                        let a2451 = {
                                            let a2445 = {
                                                let a2443 = v8;
                                                let a2444 = 2u8;
                                                fixed_u8::bit_and(a2443, a2444)
                                            };
                                            let a2446 = 2u8;
                                            if equal(a2445, a2446) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2452 = {
                                            let a2449 = {
                                                let a2447 = v8;
                                                let a2448 = 1u8;
                                                fixed_u8::bit_and(a2447, a2448)
                                            };
                                            let a2450 = 1u8;
                                            if equal(a2449, a2450) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2451, a2452)?
                                    };
                                    nat_add(a2453, a2454)?
                                };
                                nat_add(a2455, a2456)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a2457 = v1;
                                    let a2458 = 1u64;
                                    nat_add(a2457, a2458)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a2459 = v9;
                                    let a2460 = 34u64;
                                    !nat_eq(a2459, a2460)
                                }, (if v4 {
                                    false
                                } else {
                                    let a2461 = v9;
                                    let a2462 = 92u64;
                                    nat_eq(a2461, a2462)
                                }, {
                                    let a2463 = v5;
                                    let a2464 = v10;
                                    append_bytes(a2463, a2464)
                                })))))
                            } else {
                                let a2465 = v9;
                                let a2466 = 32u64;
                                if if if if nat_eq(a2465, a2466) {
                                    true
                                } else {
                                    let a2467 = v9;
                                    let a2468 = 9u64;
                                    nat_eq(a2467, a2468)
                                } {
                                    true
                                } else {
                                    let a2469 = v9;
                                    let a2470 = 10u64;
                                    nat_eq(a2469, a2470)
                                } {
                                    true
                                } else {
                                    let a2471 = v9;
                                    let a2472 = 13u64;
                                    nat_eq(a2471, a2472)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a2473 = v1;
                                        let a2474 = 1u64;
                                        nat_add(a2473, a2474)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2475 = v1;
                                            let a2476 = 4u64;
                                            nat_add(a2475, a2476)?
                                        }, (v6, (false, (false, {
                                            let a2477 = v5;
                                            let a2478 = f8(Str::lit("[]"));
                                            append_bytes(a2477, a2478)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2479 = v1;
                                            let a2480 = 1u64;
                                            nat_add(a2479, a2480)?
                                        }, (v6, ({
                                            let a2481 = v9;
                                            let a2482 = 34u64;
                                            nat_eq(a2481, a2482)
                                        }, (false, {
                                            let a2483 = v5;
                                            let a2484 = v10;
                                            append_bytes(a2483, a2484)
                                        })))))
                                    }
                                }
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

pub fn f427(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f478(Fn15::F479(v0.clone()), {
        let a2492 = {
            let a2491 = v0;
            length_bytes(a2491)
        };
        let a2493 = 1u64;
        nat_add(a2492, a2493)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a2497 = {
        let (_, h2494) = {
            let (_, h2495) = {
                let (h2496, _) = v2.clone();
                h2496
            };
            h2495
        };
        h2494
    };
    let a2498 = 1u64;
    if nat_eq(a2497, a2498) {
        Ok(Some(({
            let (h2499, _) = {
                let (h2500, _) = v2.clone();
                h2500
            };
            h2499
        }, {
            let (h2501, _) = {
                let (_, h2502) = {
                    let (h2503, _) = v2;
                    h2503
                };
                h2502
            };
            h2501
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f428(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2504 = f339(v0.clone(), v1)?;
        let a2505 = 45u64;
        nat_eq(a2504, a2505)
    };
    let v3: u64 = if v2 {
        let a2506 = v1;
        let a2507 = 1u64;
        nat_add(a2506, a2507)?
    } else {
        v1
    };
    let v4: u64 = f480(v0.clone(), v3)?;
    let v5: u64 = {
        let a2508 = v4;
        let a2509 = v3;
        nat_sub(a2508, a2509)
    };
    let v6: bool = {
        let a2510 = f339(v0.clone(), v4)?;
        let a2511 = 46u64;
        nat_eq(a2510, a2511)
    };
    let v7: u64 = if v6 {
        let a2512 = v4;
        let a2513 = 1u64;
        nat_add(a2512, a2513)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f480(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f339(v0.clone(), v8)?;
    let v10: bool = {
        let a2514 = v9;
        let a2515 = 101u64;
        if nat_eq(a2514, a2515) {
            true
        } else {
            let a2516 = v9;
            let a2517 = 69u64;
            nat_eq(a2516, a2517)
        }
    };
    let v11: u64 = f339(v0.clone(), {
        let a2518 = v8;
        let a2519 = 1u64;
        nat_add(a2518, a2519)?
    })?;
    let v12: bool = if v10 {
        let a2520 = v11;
        let a2521 = 43u64;
        if nat_eq(a2520, a2521) {
            true
        } else {
            let a2522 = v11;
            let a2523 = 45u64;
            nat_eq(a2522, a2523)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2524 = v8;
            let a2525 = 2u64;
            nat_add(a2524, a2525)?
        } else {
            let a2526 = v8;
            let a2527 = 1u64;
            nat_add(a2526, a2527)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f480(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2528 = v14;
        let a2529 = v13;
        nat_sub(a2528, a2529)
    });
    let v16: u64 = f481(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2531 = {
            let a2530 = v15;
            length_bytes(a2530)
        };
        let a2532 = v16;
        nat_sub(a2531, a2532)
    });
    let a2533 = v5;
    let a2534 = 0u64;
    if if nat_eq(a2533, a2534) {
        true
    } else {
        let a2535 = 1u64;
        let a2536 = v5;
        if if nat_lt(a2535, a2536) {
            let a2537 = f339(v0.clone(), v3)?;
            let a2538 = 48u64;
            nat_eq(a2537, a2538)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2539 = v8;
                let a2540 = v7;
                nat_eq(a2539, a2540)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2541 = v14;
                    let a2542 = v13;
                    nat_eq(a2541, a2542)
                } else {
                    false
                } {
                    true
                } else {
                    let a2544 = 18u64;
                    let a2545 = {
                        let a2543 = v17.clone();
                        length_bytes(a2543)
                    };
                    nat_lt(a2544, a2545)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2549 = {
            let a2547 = {
                let a2546 = v17.clone();
                length_bytes(a2546)
            };
            let a2548 = 0u64;
            if nat_eq(a2547, a2548) {
                Some(0u64)
            } else {
                f482(v17)?
            }
        };
        match m2549 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2554 = f483(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2550 = v8;
                    let a2551 = v7;
                    nat_sub(a2550, a2551)
                }), if v12 {
                    let a2552 = v11;
                    let a2553 = 45u64;
                    nat_eq(a2552, a2553)
                } else {
                    false
                }, v18)?;
                match m2554 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2556 = 19u64;
                        let a2557 = {
                            let a2555 = v19.clone();
                            length_bytes(a2555)
                        };
                        if nat_lt(a2556, a2557) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2558 = f482(v19)?;
                            match m2558 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2559 = 0u64;
                                        let a2560 = v20;
                                        nat_lt(a2559, a2560)
                                    } else {
                                        false
                                    };
                                    if f484(v21, v20) {
                                        Ok(Some((v14, Adt14::C1(v21, v20))))
                                    } else {
                                        Ok(None::<(u64, Adt14)>)
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

pub fn f429(v0: u64) -> bool {
    let a2561 = v0;
    let a2562 = 47u64;
    nat_eq(a2561, a2562)
}

pub fn f430(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2563 = f339(v0.clone(), v1)?;
        let a2564 = 45u64;
        nat_eq(a2563, a2564)
    };
    let v3: u64 = if v2 {
        let a2565 = v1;
        let a2566 = 1u64;
        nat_add(a2565, a2566)?
    } else {
        v1
    };
    let v4: u64 = f480(v0.clone(), v3)?;
    let v5: u64 = {
        let a2567 = v4;
        let a2568 = v3;
        nat_sub(a2567, a2568)
    };
    let v6: bool = {
        let a2569 = f339(v0.clone(), v4)?;
        let a2570 = 46u64;
        nat_eq(a2569, a2570)
    };
    let v7: u64 = if v6 {
        let a2571 = v4;
        let a2572 = 1u64;
        nat_add(a2571, a2572)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f480(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f339(v0.clone(), v8)?;
    let v10: bool = {
        let a2573 = v9;
        let a2574 = 101u64;
        if nat_eq(a2573, a2574) {
            true
        } else {
            let a2575 = v9;
            let a2576 = 69u64;
            nat_eq(a2575, a2576)
        }
    };
    let v11: u64 = f339(v0.clone(), {
        let a2577 = v8;
        let a2578 = 1u64;
        nat_add(a2577, a2578)?
    })?;
    let v12: bool = if v10 {
        let a2579 = v11;
        let a2580 = 43u64;
        if nat_eq(a2579, a2580) {
            true
        } else {
            let a2581 = v11;
            let a2582 = 45u64;
            nat_eq(a2581, a2582)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2583 = v8;
            let a2584 = 2u64;
            nat_add(a2583, a2584)?
        } else {
            let a2585 = v8;
            let a2586 = 1u64;
            nat_add(a2585, a2586)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f480(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2587 = v14;
        let a2588 = v13;
        nat_sub(a2587, a2588)
    });
    let v16: u64 = f481(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2590 = {
            let a2589 = v15;
            length_bytes(a2589)
        };
        let a2591 = v16;
        nat_sub(a2590, a2591)
    });
    let a2592 = v5;
    let a2593 = 0u64;
    if if nat_eq(a2592, a2593) {
        true
    } else {
        let a2594 = 1u64;
        let a2595 = v5;
        if if nat_lt(a2594, a2595) {
            let a2596 = f339(v0.clone(), v3)?;
            let a2597 = 48u64;
            nat_eq(a2596, a2597)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2598 = v8;
                let a2599 = v7;
                nat_eq(a2598, a2599)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2600 = v14;
                    let a2601 = v13;
                    nat_eq(a2600, a2601)
                } else {
                    false
                } {
                    true
                } else {
                    let a2603 = 18u64;
                    let a2604 = {
                        let a2602 = v17.clone();
                        length_bytes(a2602)
                    };
                    nat_lt(a2603, a2604)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2608 = {
            let a2606 = {
                let a2605 = v17.clone();
                length_bytes(a2605)
            };
            let a2607 = 0u64;
            if nat_eq(a2606, a2607) {
                Some(0u64)
            } else {
                f482(v17)?
            }
        };
        match m2608 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2613 = f485(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2609 = v8;
                    let a2610 = v7;
                    nat_sub(a2609, a2610)
                }), if v12 {
                    let a2611 = v11;
                    let a2612 = 45u64;
                    nat_eq(a2611, a2612)
                } else {
                    false
                }, v18)?;
                match m2613 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2616 = 19u64;
                        let a2617 = {
                            let a2615 = {
                                let (h2614, _) = v19.clone();
                                h2614
                            };
                            length_bytes(a2615)
                        };
                        if nat_lt(a2616, a2617) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2619 = f482({
                                let (h2618, _) = v19.clone();
                                h2618
                            })?;
                            match m2619 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2620 = 0u64;
                                        let a2621 = v20;
                                        nat_lt(a2620, a2621)
                                    } else {
                                        false
                                    };
                                    if f484(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2623 = {
                                                let (_, h2622) = v19.clone();
                                                h2622
                                            };
                                            let a2624 = 0u64;
                                            if nat_eq(a2623, a2624) {
                                                Adt19::C1(v21, v20)
                                            } else {
                                                Adt19::C2(v21, v20, {
                                                    let (_, h2625) = v19;
                                                    h2625
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt19)>)
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

pub fn f431(v0: u64) -> bool {
    let a2626 = v0;
    let a2627 = 95u64;
    if nat_eq(a2626, a2627) {
        true
    } else {
        let a2628 = v0;
        let a2629 = 45u64;
        if nat_eq(a2628, a2629) {
            true
        } else {
            let a2630 = v0;
            let a2631 = 32u64;
            if nat_eq(a2630, a2631) {
                true
            } else {
                let a2632 = v0;
                let a2633 = 46u64;
                if nat_eq(a2632, a2633) {
                    true
                } else {
                    let a2634 = v0;
                    let a2635 = 47u64;
                    nat_eq(a2634, a2635)
                }
            }
        }
    }
}

pub fn f432(v0: u64) -> bool {
    let a2636 = 97u64;
    let a2637 = v0;
    if nat_le(a2636, a2637) {
        let a2638 = v0;
        let a2639 = 122u64;
        nat_le(a2638, a2639)
    } else {
        false
    }
}

pub fn f433(v0: u64) -> bool {
    let a2640 = 65u64;
    let a2641 = v0;
    if nat_le(a2640, a2641) {
        let a2642 = v0;
        let a2643 = 90u64;
        nat_le(a2642, a2643)
    } else {
        false
    }
}

pub fn f434(v0: u64) -> bool {
    let a2644 = 48u64;
    let a2645 = v0;
    if nat_le(a2644, a2645) {
        let a2646 = v0;
        let a2647 = 57u64;
        nat_le(a2646, a2647)
    } else {
        false
    }
}

pub fn f435(v0: List<(Bytes, Adt19)>, v1: List<(Bytes, Adt19)>) -> List<(Bytes, Adt19)> {
    let m2648 = v0;
    match m2648.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f435(v3, v1))
        }
    }
}

pub fn f436(v0: Option<Adt19>) -> List<(Bytes, Adt19)> {
    let m2649 = v0;
    match m2649 {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some(v1) => {
            f486(v1)
        }
    }
}

pub fn f437(v0: u64, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt19)>, Bytes>> {
    let m2650 = v0;
    if m2650 == 0 {
        let m2651 = v1;
        match m2651.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2650 - 1;
        let m2652 = v1.clone();
        match m2652.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                let m2653 = f487(v1.clone(), v2.clone(), v3.clone())?;
                match m2653 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f226(f438(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2656 = f437(v6, f488(v1, {
                            let (h2654, _) = v9.clone();
                            h2654
                        }), List::cons(f265({
                            let (h2655, _) = v9.clone();
                            h2655
                        })?, v2), v3)?;
                        match m2656 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt19)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f438(v0: List<(Bytes, Adt19)>) -> R<List<Bytes>> {
    let m2657 = v0;
    match m2657.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f265({
                let (h2658, _) = v1;
                h2658
            })?, f438(v2)?))
        }
    }
}

pub fn f439(p2666: u64, p2667: List<Adt25>, p2668: List<Bytes>, p2669: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2665 = (p2666, (p2667, (p2668, p2669)));
    {
        let mut m2665 = m2665;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2665;
                let m2659 = v0;
                if m2659 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2659 - 1;
                    let m2660 = v1;
                    match m2660.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2661 = v5;
                            match m2661 {
                                Adt25::C0(v8, v9) => {
                                    if f37(v8.clone(), f489()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f90(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2662 = f491(v8.clone(), v9)?;
                                            match m2662 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f440({
                                                        let (_, h2663) = v11;
                                                        h2663
                                                    }, List::cons(Adt25::C1({
                                                        let (h2664, _) = v11;
                                                        h2664
                                                    }), v6)), (List::cons(v8, v2), v3))))
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
                                    if f90(f489(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f489(), v2), List::cons(f490(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2665 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2670 = v0;
    match m2670.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f440(v3, v1))
        }
    }
}

pub fn f441(v0: List<(Bytes, Adt19)>) -> R<List<Adt25>> {
    let m2671 = v0;
    match m2671.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f265({
                let (h2672, _) = v1;
                h2672
            })?, {
                let (_, h2673) = v1;
                h2673
            }), f441(v2)?))
        }
    }
}

pub fn f442(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2674 = v0;
    match m2674.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f37(f166(v3.clone(), v1.clone()), v2.clone()) {
                f442(v4, v1, v2)
            } else {
                List::cons(v3, f442(v4, v1, v2))
            }
        }
    }
}

pub fn f443(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2675 = v0;
    match m2675.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f216(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f226(f451(f82(List::cons(v1, f492(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f443(v3, v1)?))
            }
        }
    }
}

pub fn f444(v0: u64) -> Bytes {
    let m2676 = v0;
    if m2676 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2676 - 1;
        let a2677 = f8(Str::lit("0"));
        let a2678 = f444(v1);
        append_bytes(a2677, a2678)
    }
}

pub fn f445(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f493(v0, v1)
}

pub fn f446(v0: Fn10, p2682: List<(Bytes, Bytes)>, p2683: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2681 = (p2682, p2683);
    {
        let mut m2681 = m2681;
        loop {
            let step = {
                let (v1, v2) = m2681;
                let m2679 = v2;
                match m2679.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2680: Fn10 = v0.clone();
                            c2680.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2681 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f494(v0, v1)
}

pub fn f448(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f495(v0)
}

pub fn f449(v0: Fn11, p2687: List<Bytes>, p2688: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2686 = (p2687, p2688);
    {
        let mut m2686 = m2686;
        loop {
            let step = {
                let (v1, v2) = m2686;
                let m2684 = v2;
                match m2684.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2685: Fn11 = v0.clone();
                            c2685.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2686 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f496(v1)?, v0))
}

pub fn f451(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2689 = v0;
    match m2689.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f221(v1)?, f451(v2)?))
        }
    }
}

pub fn f452(v0: Fn12, p2693: List<List<(Bytes, ())>>, p2694: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2692 = (p2693, p2694);
    {
        let mut m2692 = m2692;
        loop {
            let step = {
                let (v1, v2) = m2692;
                let m2690 = v2;
                match m2690.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2691: Fn12 = v0.clone();
                            c2691.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2692 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f454(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2696 = {
        let a2695 = v0.clone();
        length_list(a2695)
    };
    let a2697 = 1u64;
    if nat_le(a2696, a2697) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f497(v0)?))
    }
}

pub fn f455(v0: Fn13, p2701: List<(Bytes, ())>, p2702: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2700 = (p2701, p2702);
    {
        let mut m2700 = m2700;
        loop {
            let step = {
                let (v1, v2) = m2700;
                let m2698 = v2;
                match m2698.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2699: Fn13 = v0.clone();
                            c2699.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f457(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f76(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f498(v3.clone(), f76(f77(v0, f93())?, f8(Str::lit("at"))));
    let a2704 = {
        let a2703 = v4.clone();
        length_list(a2703)
    };
    let a2705 = 0u64;
    if nat_eq(a2704, a2705) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2708 = {
            let a2706 = v4;
            length_list(a2706)
        };
        let a2709 = {
            let a2707 = v3;
            length_list(a2707)
        };
        if nat_eq(a2708, a2709) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f499(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f458(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f260(f149(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f260(f149(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f66(f76(v3.clone(), f8(Str::lit("id"))), f76(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f76(f77(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f498(v5.clone(), f66(v6.clone(), f76(f77(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2711 = {
        let a2710 = v7.clone();
        length_list(a2710)
    };
    let a2712 = 0u64;
    if nat_eq(a2711, a2712) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f79({
            let a2715 = {
                let a2713 = v7;
                length_list(a2713)
            };
            let a2716 = {
                let a2714 = v5;
                length_list(a2714)
            };
            nat_eq(a2715, a2716)
        }) {
            Ok((false, f499(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f82(f66(f498(f500(v4.clone()), f73(f74(v0, f8(Str::lit("model/authorities.toml"))))?), f498(f501(v4), f66(v6, f76(v3, f8(Str::lit("id")))))))?;
            let a2718 = {
                let a2717 = v8.clone();
                length_list(a2717)
            };
            let a2719 = 0u64;
            if nat_eq(a2718, a2719) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f499(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f126(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f459(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f37({
        let (h2720, _) = v0;
        h2720
    }, {
        let (h2721, _) = v1;
        h2721
    }) {
        f37({
            let (_, h2722) = v0;
            h2722
        }, {
            let (_, h2723) = v1;
            h2723
        })
    } else {
        false
    }
}

pub fn f460(v0: Fn14, p2727: u64, p2728: Adt27) -> R<Adt27> {
    let m2726 = (p2727, p2728);
    {
        let mut m2726 = m2726;
        loop {
            let step = {
                let (v1, v2) = m2726;
                let m2724 = v1;
                if m2724 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2724 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2725: Fn14 = v0.clone();
                        c2725.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: Adt27) -> R<Adt27> {
    f502(v0)
}

pub fn f462(v0: u32, v1: u32) -> u32 {
    let m2731 = {
        let a2729 = f503(v0);
        let a2730 = f503(v1);
        fixed_u64::checked_add(a2729, a2730)
    };
    match m2731 {
        None => {
            0u32
        }
        Some(v2) => {
            f504(v2)
        }
    }
}

pub fn f463(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2734 = {
        let a2732 = v3;
        let a2733 = 8u64;
        nat_add(a2732, a2733)?
    };
    let a2735 = v2;
    if nat_eq(a2734, a2735) {
        Ok(f505({
            let a2738 = {
                let a2736 = v1;
                let a2737 = 8u64;
                nat_mul(a2736, a2737)?
            };
            let a2739 = 4294967296u64;
            let a2740 = 0u64;
            nat_quot(a2738, a2739, a2740)
        }))
    } else {
        let a2743 = {
            let a2741 = v3;
            let a2742 = 4u64;
            nat_add(a2741, a2742)?
        };
        let a2744 = v2;
        if nat_eq(a2743, a2744) {
            Ok(f505({
                let a2747 = {
                    let a2745 = v1;
                    let a2746 = 8u64;
                    nat_mul(a2745, a2746)?
                };
                let a2748 = 4294967296u64;
                let a2749 = 0u64;
                nat_rem(a2747, a2748, a2749)
            }))
        } else {
            Ok(f506(f507(v0.clone(), v1, v3), f507(v0.clone(), v1, {
                let a2750 = v3;
                let a2751 = 1u64;
                nat_add(a2750, a2751)?
            }), f507(v0.clone(), v1, {
                let a2752 = v3;
                let a2753 = 2u64;
                nat_add(a2752, a2753)?
            }), f507(v0, v1, {
                let a2754 = v3;
                let a2755 = 3u64;
                nat_add(a2754, a2755)?
            })))
        }
    }
}

pub fn f464(v0: Bytes, v1: u64) -> bool {
    let m2756 = f276(v0, v1);
    match m2756 {
        None => {
            false
        }
        Some(v2) => {
            f508(v2)
        }
    }
}

pub fn f465(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2759 = {
        let a2757 = v0.clone();
        let a2758 = v1;
        index_bytes(a2757, a2758)
    };
    match m2759 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2804 = {
                    let a2780 = {
                        let a2768 = {
                            let a2762 = {
                                let a2760 = v3;
                                let a2761 = 128u8;
                                fixed_u8::bit_and(a2760, a2761)
                            };
                            let a2763 = 128u8;
                            if equal(a2762, a2763) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2769 = {
                            let a2766 = {
                                let a2764 = v3;
                                let a2765 = 64u8;
                                fixed_u8::bit_and(a2764, a2765)
                            };
                            let a2767 = 64u8;
                            if equal(a2766, a2767) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2768, a2769)?
                    };
                    let a2781 = {
                        let a2778 = {
                            let a2772 = {
                                let a2770 = v3;
                                let a2771 = 32u8;
                                fixed_u8::bit_and(a2770, a2771)
                            };
                            let a2773 = 32u8;
                            if equal(a2772, a2773) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2779 = {
                            let a2776 = {
                                let a2774 = v3;
                                let a2775 = 16u8;
                                fixed_u8::bit_and(a2774, a2775)
                            };
                            let a2777 = 16u8;
                            if equal(a2776, a2777) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2778, a2779)?
                    };
                    nat_add(a2780, a2781)?
                };
                let a2805 = {
                    let a2802 = {
                        let a2790 = {
                            let a2784 = {
                                let a2782 = v3;
                                let a2783 = 8u8;
                                fixed_u8::bit_and(a2782, a2783)
                            };
                            let a2785 = 8u8;
                            if equal(a2784, a2785) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2791 = {
                            let a2788 = {
                                let a2786 = v3;
                                let a2787 = 4u8;
                                fixed_u8::bit_and(a2786, a2787)
                            };
                            let a2789 = 4u8;
                            if equal(a2788, a2789) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2790, a2791)?
                    };
                    let a2803 = {
                        let a2800 = {
                            let a2794 = {
                                let a2792 = v3;
                                let a2793 = 2u8;
                                fixed_u8::bit_and(a2792, a2793)
                            };
                            let a2795 = 2u8;
                            if equal(a2794, a2795) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2801 = {
                            let a2798 = {
                                let a2796 = v3;
                                let a2797 = 1u8;
                                fixed_u8::bit_and(a2796, a2797)
                            };
                            let a2799 = 1u8;
                            if equal(a2798, a2799) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2800, a2801)?
                    };
                    nat_add(a2802, a2803)?
                };
                nat_add(a2804, a2805)?
            };
            let a2806 = v4;
            let a2807 = 128u64;
            if nat_lt(a2806, a2807) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2808 = 194u64;
                let a2809 = v4;
                if if nat_le(a2808, a2809) {
                    let a2810 = v4;
                    let a2811 = 223u64;
                    nat_le(a2810, a2811)
                } else {
                    false
                } {
                    let m2814 = f509(v0, {
                        let a2812 = v1;
                        let a2813 = 1u64;
                        nat_add(a2812, a2813)?
                    })?;
                    match m2814 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f510({
                                let a2819 = {
                                    let a2817 = {
                                        let a2815 = v4;
                                        let a2816 = 192u64;
                                        nat_sub(a2815, a2816)
                                    };
                                    let a2818 = 64u64;
                                    nat_mul(a2817, a2818)?
                                };
                                let a2820 = v5;
                                nat_add(a2819, a2820)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2821 = 224u64;
                    let a2822 = v4;
                    if if nat_le(a2821, a2822) {
                        let a2823 = v4;
                        let a2824 = 239u64;
                        nat_le(a2823, a2824)
                    } else {
                        false
                    } {
                        let m2827 = f509(v0.clone(), {
                            let a2825 = v1;
                            let a2826 = 1u64;
                            nat_add(a2825, a2826)?
                        })?;
                        match m2827 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2830 = f509(v0, {
                                    let a2828 = v1;
                                    let a2829 = 2u64;
                                    nat_add(a2828, a2829)?
                                })?;
                                match m2830 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f510({
                                            let a2839 = {
                                                let a2833 = {
                                                    let a2831 = v4;
                                                    let a2832 = 224u64;
                                                    nat_sub(a2831, a2832)
                                                };
                                                let a2834 = 4096u64;
                                                nat_mul(a2833, a2834)?
                                            };
                                            let a2840 = {
                                                let a2837 = {
                                                    let a2835 = v6;
                                                    let a2836 = 64u64;
                                                    nat_mul(a2835, a2836)?
                                                };
                                                let a2838 = v7;
                                                nat_add(a2837, a2838)?
                                            };
                                            nat_add(a2839, a2840)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2841 = 240u64;
                        let a2842 = v4;
                        if if nat_le(a2841, a2842) {
                            let a2843 = v4;
                            let a2844 = 244u64;
                            nat_le(a2843, a2844)
                        } else {
                            false
                        } {
                            let m2847 = f509(v0.clone(), {
                                let a2845 = v1;
                                let a2846 = 1u64;
                                nat_add(a2845, a2846)?
                            })?;
                            match m2847 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2850 = f509(v0.clone(), {
                                        let a2848 = v1;
                                        let a2849 = 2u64;
                                        nat_add(a2848, a2849)?
                                    })?;
                                    match m2850 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m2853 = f509(v0, {
                                                let a2851 = v1;
                                                let a2852 = 3u64;
                                                nat_add(a2851, a2852)?
                                            })?;
                                            match m2853 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f510({
                                                        let a2866 = {
                                                            let a2856 = {
                                                                let a2854 = v4;
                                                                let a2855 = 240u64;
                                                                nat_sub(a2854, a2855)
                                                            };
                                                            let a2857 = 262144u64;
                                                            nat_mul(a2856, a2857)?
                                                        };
                                                        let a2867 = {
                                                            let a2864 = {
                                                                let a2858 = v8;
                                                                let a2859 = 4096u64;
                                                                nat_mul(a2858, a2859)?
                                                            };
                                                            let a2865 = {
                                                                let a2862 = {
                                                                    let a2860 = v9;
                                                                    let a2861 = 64u64;
                                                                    nat_mul(a2860, a2861)?
                                                                };
                                                                let a2863 = v10;
                                                                nat_add(a2862, a2863)?
                                                            };
                                                            nat_add(a2864, a2865)?
                                                        };
                                                        nat_add(a2866, a2867)?
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

pub fn f466(v0: u64) -> bool {
    let a2868 = v0;
    let a2869 = 9u64;
    if if if nat_eq(a2868, a2869) {
        true
    } else {
        let a2870 = v0;
        let a2871 = 10u64;
        nat_eq(a2870, a2871)
    } {
        true
    } else {
        let a2872 = v0;
        let a2873 = 13u64;
        nat_eq(a2872, a2873)
    } {
        true
    } else {
        let a2874 = 32u64;
        let a2875 = v0;
        if if if nat_le(a2874, a2875) {
            let a2876 = v0;
            let a2877 = 55295u64;
            nat_le(a2876, a2877)
        } else {
            false
        } {
            true
        } else {
            let a2878 = 57344u64;
            let a2879 = v0;
            if nat_le(a2878, a2879) {
                let a2880 = v0;
                let a2881 = 65533u64;
                nat_le(a2880, a2881)
            } else {
                false
            }
        } {
            true
        } else {
            let a2882 = 65536u64;
            let a2883 = v0;
            if nat_le(a2882, a2883) {
                let a2884 = v0;
                let a2885 = 1114111u64;
                nat_le(a2884, a2885)
            } else {
                false
            }
        }
    }
}

pub fn f467(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f231(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2887 = v2;
        let a2888 = {
            let a2886 = v0;
            length_bytes(a2886)
        };
        if nat_lt(a2887, a2888) {
            Ok(Some({
                let a2889 = v2;
                let a2890 = 1u64;
                nat_add(a2889, a2890)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f468(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f333(f238(v0, v1)?) {
        Ok(Some({
            let a2891 = v1;
            let a2892 = 1u64;
            nat_add(a2891, a2892)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f469(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2893 = f465(v0, v2)?;
    match m2893 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2894 = v2;
            let a2895 = v1;
            if if nat_eq(a2894, a2895) {
                f511({
                    let (h2896, _) = v3;
                    h2896
                })
            } else {
                f512({
                    let (h2897, _) = v3;
                    h2897
                })
            } {
                Ok(Some({
                    let a2899 = v2;
                    let a2900 = {
                        let (_, h2898) = v3;
                        h2898
                    };
                    nat_add(a2899, a2900)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f470(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2901, _) = f513(Fn16::F514(v1, v0.clone()), {
        let a2903 = {
            let a2902 = v0;
            length_bytes(a2902)
        };
        let a2904 = 1u64;
        nat_add(a2903, a2904)?
    }, (v2, 0u64))?;
    Ok(h2901)
}

pub fn f471(v0: u64) -> R<Bytes> {
    let a2905 = v0;
    let a2906 = 128u64;
    if nat_lt(a2905, a2906) {
        Ok(f515(v0))
    } else {
        let a2907 = v0;
        let a2908 = 2048u64;
        if nat_lt(a2907, a2908) {
            let a2919 = f515({
                let a2912 = 192u64;
                let a2913 = {
                    let a2909 = v0;
                    let a2910 = 64u64;
                    let a2911 = 0u64;
                    nat_quot(a2909, a2910, a2911)
                };
                nat_add(a2912, a2913)?
            });
            let a2920 = f515({
                let a2917 = 128u64;
                let a2918 = {
                    let a2914 = v0;
                    let a2915 = 64u64;
                    let a2916 = 0u64;
                    nat_rem(a2914, a2915, a2916)
                };
                nat_add(a2917, a2918)?
            });
            Ok(append_bytes(a2919, a2920))
        } else {
            let a2921 = v0;
            let a2922 = 65536u64;
            if nat_lt(a2921, a2922) {
                let a2943 = f515({
                    let a2926 = 224u64;
                    let a2927 = {
                        let a2923 = v0;
                        let a2924 = 4096u64;
                        let a2925 = 0u64;
                        nat_quot(a2923, a2924, a2925)
                    };
                    nat_add(a2926, a2927)?
                });
                let a2944 = {
                    let a2941 = f515({
                        let a2934 = 128u64;
                        let a2935 = {
                            let a2931 = {
                                let a2928 = v0;
                                let a2929 = 64u64;
                                let a2930 = 0u64;
                                nat_quot(a2928, a2929, a2930)
                            };
                            let a2932 = 64u64;
                            let a2933 = 0u64;
                            nat_rem(a2931, a2932, a2933)
                        };
                        nat_add(a2934, a2935)?
                    });
                    let a2942 = f515({
                        let a2939 = 128u64;
                        let a2940 = {
                            let a2936 = v0;
                            let a2937 = 64u64;
                            let a2938 = 0u64;
                            nat_rem(a2936, a2937, a2938)
                        };
                        nat_add(a2939, a2940)?
                    });
                    append_bytes(a2941, a2942)
                };
                Ok(append_bytes(a2943, a2944))
            } else {
                let a2975 = f515({
                    let a2948 = 240u64;
                    let a2949 = {
                        let a2945 = v0;
                        let a2946 = 262144u64;
                        let a2947 = 0u64;
                        nat_quot(a2945, a2946, a2947)
                    };
                    nat_add(a2948, a2949)?
                });
                let a2976 = {
                    let a2973 = f515({
                        let a2956 = 128u64;
                        let a2957 = {
                            let a2953 = {
                                let a2950 = v0;
                                let a2951 = 4096u64;
                                let a2952 = 0u64;
                                nat_quot(a2950, a2951, a2952)
                            };
                            let a2954 = 64u64;
                            let a2955 = 0u64;
                            nat_rem(a2953, a2954, a2955)
                        };
                        nat_add(a2956, a2957)?
                    });
                    let a2974 = {
                        let a2971 = f515({
                            let a2964 = 128u64;
                            let a2965 = {
                                let a2961 = {
                                    let a2958 = v0;
                                    let a2959 = 64u64;
                                    let a2960 = 0u64;
                                    nat_quot(a2958, a2959, a2960)
                                };
                                let a2962 = 64u64;
                                let a2963 = 0u64;
                                nat_rem(a2961, a2962, a2963)
                            };
                            nat_add(a2964, a2965)?
                        });
                        let a2972 = f515({
                            let a2969 = 128u64;
                            let a2970 = {
                                let a2966 = v0;
                                let a2967 = 64u64;
                                let a2968 = 0u64;
                                nat_rem(a2966, a2967, a2968)
                            };
                            nat_add(a2969, a2970)?
                        });
                        append_bytes(a2971, a2972)
                    };
                    append_bytes(a2973, a2974)
                };
                Ok(append_bytes(a2975, a2976))
            }
        }
    }
}

pub fn f472(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f516(v2, v0, v1, v3)
}

pub fn f473(p2980: Bytes, p2981: u64) -> Result<Bytes, Bytes> {
    let m2979 = (p2980, p2981);
    {
        let mut m2979 = m2979;
        loop {
            let step = {
                let (v0, v1) = m2979;
                let m2977 = v1;
                if m2977 == 0 {
                    Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines"))))
                } else {
                    let v2 = m2977 - 1;
                    if f412(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)"))))
                    } else {
                        if f37(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Nat"))))
                        } else {
                            if f37(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Int"))))
                            } else {
                                if f37(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f517()))
                                } else {
                                    if f37(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Bool"))))
                                    } else {
                                        if if f37(v0.clone(), f8(Str::lit("float"))) {
                                            true
                                        } else {
                                            f37(v0.clone(), f8(Str::lit("double")))
                                        } {
                                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil()))))))
                                        } else {
                                            if if f37(v0.clone(), f8(Str::lit("anySimpleType"))) {
                                                true
                                            } else {
                                                f37(v0.clone(), f8(Str::lit("anyAtomicType")))
                                            } {
                                                Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil()))))))
                                            } else {
                                                let m2978 = f518(v0.clone());
                                                match m2978 {
                                                    None => {
                                                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil()))))))
                                                    }
                                                    Some(v3) => {
                                                        if f37(v3.clone(), f8(Str::lit("anyAtomicType"))) {
                                                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Bytes"))))
                                                        } else {
                                                            Ok::<(Bytes, u64), Result<Bytes, Bytes>>((v3, v2))
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
            };
            match step {
                Ok(next) => { m2979 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f474(v0: Adt19) -> R<Bytes> {
    f268(v0)
}

pub fn f475(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m2982 = v1;
    match m2982 {
        Adt19::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C1(v7, v8) => {
            Ok(Some({
                let a2993 = if if v7 {
                    let a2991 = 0u64;
                    let a2992 = v8;
                    nat_lt(a2991, a2992)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2994 = f368(false, v8)?;
                append_bytes(a2993, a2994)
            }))
        }
        Adt19::C2(v3, v4, v5) => {
            let m2985 = f520({
                let a2983 = f8(Str::lit("n"));
                let a2984 = f369(false, v4, v5)?;
                append_bytes(a2983, a2984)
            })?;
            match m2985 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2989 = f8(Str::lit("nMinus"));
                        let a2990 = f32(v6.clone(), 1u64, {
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
        Adt19::C3(v2) => {
            f519(v0, v2)
        }
        Adt19::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C6 => {
            Ok(None::<Bytes>)
        }
        Adt19::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt19::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f476(v0: Bytes) -> R<Option<Bytes>> {
    if f521({
        let a2995 = v0.clone();
        length_bytes(a2995)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f522(v0)?;
        if f523(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2998 = {
                let a2996 = v1.clone();
                let a2997 = 0u64;
                index_bytes(a2996, a2997)
            };
            match m2998 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f434({
                        let v3: u8 = v2;
                        let a3043 = {
                            let a3019 = {
                                let a3007 = {
                                    let a3001 = {
                                        let a2999 = v3;
                                        let a3000 = 128u8;
                                        fixed_u8::bit_and(a2999, a3000)
                                    };
                                    let a3002 = 128u8;
                                    if equal(a3001, a3002) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3008 = {
                                    let a3005 = {
                                        let a3003 = v3;
                                        let a3004 = 64u8;
                                        fixed_u8::bit_and(a3003, a3004)
                                    };
                                    let a3006 = 64u8;
                                    if equal(a3005, a3006) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3007, a3008)?
                            };
                            let a3020 = {
                                let a3017 = {
                                    let a3011 = {
                                        let a3009 = v3;
                                        let a3010 = 32u8;
                                        fixed_u8::bit_and(a3009, a3010)
                                    };
                                    let a3012 = 32u8;
                                    if equal(a3011, a3012) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3018 = {
                                    let a3015 = {
                                        let a3013 = v3;
                                        let a3014 = 16u8;
                                        fixed_u8::bit_and(a3013, a3014)
                                    };
                                    let a3016 = 16u8;
                                    if equal(a3015, a3016) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3017, a3018)?
                            };
                            nat_add(a3019, a3020)?
                        };
                        let a3044 = {
                            let a3041 = {
                                let a3029 = {
                                    let a3023 = {
                                        let a3021 = v3;
                                        let a3022 = 8u8;
                                        fixed_u8::bit_and(a3021, a3022)
                                    };
                                    let a3024 = 8u8;
                                    if equal(a3023, a3024) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3030 = {
                                    let a3027 = {
                                        let a3025 = v3;
                                        let a3026 = 4u8;
                                        fixed_u8::bit_and(a3025, a3026)
                                    };
                                    let a3028 = 4u8;
                                    if equal(a3027, a3028) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3029, a3030)?
                            };
                            let a3042 = {
                                let a3039 = {
                                    let a3033 = {
                                        let a3031 = v3;
                                        let a3032 = 2u8;
                                        fixed_u8::bit_and(a3031, a3032)
                                    };
                                    let a3034 = 2u8;
                                    if equal(a3033, a3034) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3040 = {
                                    let a3037 = {
                                        let a3035 = v3;
                                        let a3036 = 1u8;
                                        fixed_u8::bit_and(a3035, a3036)
                                    };
                                    let a3038 = 1u8;
                                    if equal(a3037, a3038) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3039, a3040)?
                            };
                            nat_add(a3041, a3042)?
                        };
                        nat_add(a3043, a3044)?
                    }) {
                        Ok(Some({
                            let a3045 = f8(Str::lit("n"));
                            let a3046 = v1;
                            append_bytes(a3045, a3046)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f524({
            let a3047 = v0.clone();
            length_bytes(a3047)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f477(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3048 = v1;
    match m3048.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3049 = f519(v0, v2)?;
            match m3049 {
                None => {
                    f477(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f477(v0, v3)?))
                }
            }
        }
    }
}

pub fn f478(v0: Fn15, p3054: u64, p3055: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m3053 = (p3054, p3055);
    {
        let mut m3053 = m3053;
        loop {
            let step = {
                let (v1, v2) = m3053;
                let m3050 = v1;
                if m3050 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3050 - 1;
                    let m3052 = {
                        let c3051: Fn15 = v0.clone();
                        c3051.apply(v2.clone())?
                    };
                    match m3052 {
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
                Ok(next) => { m3053 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f525(v0, v1)
}

pub fn f480(v0: Bytes, v1: u64) -> R<u64> {
    let (h3056, _) = f280(Fn3::F526(v0.clone()), {
        let a3058 = {
            let a3057 = v0;
            length_bytes(a3057)
        };
        let a3059 = 1u64;
        nat_add(a3058, a3059)?
    }, v1)?;
    Ok(h3056)
}

pub fn f481(v0: Bytes) -> R<u64> {
    let (h3060, _) = f280(Fn3::F527(v0.clone()), {
        let a3062 = {
            let a3061 = v0;
            length_bytes(a3061)
        };
        let a3063 = 1u64;
        nat_add(a3062, a3063)?
    }, 0u64)?;
    Ok(h3060)
}

pub fn f482(v0: Bytes) -> R<Option<u64>> {
    if f528(v0.clone())? {
        Ok(Some({
            let (_, h3064) = {
                let (h3065, _) = f513(Fn16::F529(v0.clone()), {
                    let a3066 = v0;
                    length_bytes(a3066)
                }, (0u64, 0u64))?;
                h3065
            };
            h3064
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f483(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<Bytes>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a3068 = {
            let a3067 = v1.clone();
            length_bytes(a3067)
        };
        let a3069 = f530(v1)?;
        nat_sub(a3068, a3069)
    });
    let v5: Bytes = {
        let a3070 = v0;
        let a3071 = v4.clone();
        append_bytes(a3070, a3071)
    };
    let v6: u64 = f481(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a3073 = {
            let a3072 = v5;
            length_bytes(a3072)
        };
        let a3074 = v6;
        nat_sub(a3073, a3074)
    });
    let v8: u64 = {
        let a3075 = v4;
        length_bytes(a3075)
    };
    let v9: u64 = {
        let a3076 = v7.clone();
        length_bytes(a3076)
    };
    let a3077 = v9;
    let a3078 = 0u64;
    if nat_eq(a3077, a3078) {
        Ok(Some(f8(Str::lit("0"))))
    } else {
        if if v2 {
            true
        } else {
            let a3079 = v3;
            let a3080 = v8;
            nat_lt(a3079, a3080)
        } {
            let v10: u64 = if v2 {
                let a3081 = v3;
                let a3082 = v8;
                nat_add(a3081, a3082)?
            } else {
                let a3083 = v8;
                let a3084 = v3;
                nat_sub(a3083, a3084)
            };
            let a3085 = v10;
            let a3086 = f530(v7.clone())?;
            if nat_le(a3085, a3086) {
                Ok(Some(f32(v7, 0u64, {
                    let a3087 = v9;
                    let a3088 = v10;
                    nat_sub(a3087, a3088)
                })))
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            let v11: u64 = {
                let a3089 = v3;
                let a3090 = v8;
                nat_sub(a3089, a3090)
            };
            let a3093 = {
                let a3091 = v9;
                let a3092 = v11;
                nat_add(a3091, a3092)?
            };
            let a3094 = 19u64;
            if nat_le(a3093, a3094) {
                Ok(Some({
                    let a3095 = v7;
                    let a3096 = f444(v11);
                    append_bytes(a3095, a3096)
                }))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f484(v0: bool, v1: u64) -> bool {
    if v0 {
        let a3097 = 0u64;
        let a3098 = v1;
        if nat_lt(a3097, a3098) {
            let a3099 = v1;
            let a3100 = 9223372036854775808u64;
            nat_le(a3099, a3100)
        } else {
            false
        }
    } else {
        let a3101 = v1;
        let a3102 = 9223372036854775807u64;
        nat_le(a3101, a3102)
    }
}

pub fn f485(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a3104 = {
            let a3103 = v1.clone();
            length_bytes(a3103)
        };
        let a3105 = f530(v1)?;
        nat_sub(a3104, a3105)
    });
    let v5: Bytes = {
        let a3106 = v0;
        let a3107 = v4.clone();
        append_bytes(a3106, a3107)
    };
    let v6: u64 = f481(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a3109 = {
            let a3108 = v5;
            length_bytes(a3108)
        };
        let a3110 = v6;
        nat_sub(a3109, a3110)
    });
    let v8: u64 = {
        let a3111 = v4;
        length_bytes(a3111)
    };
    let v9: u64 = {
        let a3112 = v7.clone();
        length_bytes(a3112)
    };
    let a3113 = v9;
    let a3114 = 0u64;
    if nat_eq(a3113, a3114) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if if v2 {
            true
        } else {
            let a3115 = v3;
            let a3116 = v8;
            nat_lt(a3115, a3116)
        } {
            let v10: u64 = if v2 {
                let a3117 = v3;
                let a3118 = v8;
                nat_add(a3117, a3118)?
            } else {
                let a3119 = v8;
                let a3120 = v3;
                nat_sub(a3119, a3120)
            };
            let v11: u64 = f530(v7.clone())?;
            let a3121 = v10;
            let a3122 = v11;
            if nat_le(a3121, a3122) {
                Ok(Some((f32(v7, 0u64, {
                    let a3123 = v9;
                    let a3124 = v10;
                    nat_sub(a3123, a3124)
                }), 0u64)))
            } else {
                let a3127 = {
                    let a3125 = v10;
                    let a3126 = v11;
                    nat_sub(a3125, a3126)
                };
                let a3128 = 19u64;
                if nat_le(a3127, a3128) {
                    Ok(Some((f32(v7, 0u64, {
                        let a3129 = v9;
                        let a3130 = v11;
                        nat_sub(a3129, a3130)
                    }), {
                        let a3131 = v10;
                        let a3132 = v11;
                        nat_sub(a3131, a3132)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a3133 = v3;
                let a3134 = v8;
                nat_sub(a3133, a3134)
            };
            let a3137 = {
                let a3135 = v9;
                let a3136 = v12;
                nat_add(a3135, a3136)?
            };
            let a3138 = 19u64;
            if nat_le(a3137, a3138) {
                Ok(Some(({
                    let a3139 = v7;
                    let a3140 = f444(v12);
                    append_bytes(a3139, a3140)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f486(v0: Adt19) -> List<(Bytes, Adt19)> {
    let m3141 = v0;
    match m3141 {
        Adt19::C0(_) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C1(_, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C2(_, _, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C3(_) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C4(_) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C5(r3142) => {
            let v4 = (*r3142).clone();
            f486(v4)
        }
        Adt19::C6 => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C7(_, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C8(v1, r3143, r3144) => {
            let v2 = (*r3143).clone();
            let v3 = (*r3144).clone();
            List::cons((v1, v2), f486(v3))
        }
    }
}

pub fn f487(p3149: List<(Bytes, Adt19)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt19)>> {
    let m3148 = p3149;
    {
        let mut m3148 = m3148;
        loop {
            let step = {
                let v0 = m3148;
                let m3145 = v0;
                match m3145.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(None::<(Bytes, Adt19)>))
                    }
                    Some((v3, v4)) => {
                        if f531(f532({
                            let (_, h3146) = v3.clone();
                            h3146
                        }, List::<Bytes>::nil())?, f265({
                            let (h3147, _) = v3.clone();
                            h3147
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(v0: List<(Bytes, Adt19)>, v1: Bytes) -> List<(Bytes, Adt19)> {
    let m3150 = v0;
    match m3150.uncons() {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some((v2, v3)) => {
            if f37({
                let (h3151, _) = v2.clone();
                h3151
            }, v1.clone()) {
                f488(v3, v1)
            } else {
                List::cons(v2, f488(v3, v1))
            }
        }
    }
}

pub fn f489() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f490() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f491(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f533(v0, f534(v1))
}

pub fn f492(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m3152 = v0;
    match m3152.uncons() {
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

pub fn f493(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m3156 = {
        let a3154 = v0.clone();
        let a3155 = {
            let (h3153, _) = v1.clone();
            h3153
        };
        index_bytes(a3154, a3155)
    };
    match m3156 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3158 = {
                    let (h3157, _) = v1.clone();
                    h3157
                };
                let a3159 = 1u64;
                nat_add(a3158, a3159)?
            }, {
                let a3162 = {
                    let (_, h3160) = v1;
                    h3160
                };
                let a3163 = f535(v0, {
                    let (h3161, _) = v1;
                    h3161
                }, v2)?;
                append_bytes(a3162, a3163)
            })))
        }
    }
}

pub fn f494(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3164 = v0.clone();
    match m3164.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3167 = {
                let (h3165, _) = v2;
                h3165
            };
            let a3168 = {
                let (h3166, _) = v1.clone();
                h3166
            };
            if equal(a3167, a3168) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f495(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f536(f537(Fn17::F538, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3173 = {
        let (h3169, _) = f539(Fn18::F540, {
            let a3171 = {
                let a3170 = v0;
                length_list(a3170)
            };
            let a3172 = 1u64;
            nat_add(a3171, a3172)?
        }, v1)?;
        h3169
    };
    match m3173.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f496(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3178 = {
        let a3175 = f370({
            let (h3174, _) = v0;
            h3174
        })?;
        let a3176 = Bytes::lit(&[0x3a]);
        append_bytes(a3175, a3176)
    };
    let a3179 = {
        let (_, h3177) = v0;
        h3177
    };
    Ok(append_bytes(a3178, a3179))
}

pub fn f497(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f385({
        let (_, h3180) = {
            let (h3181, _) = f541(Fn19::F542, {
                let a3183 = {
                    let a3182 = v0.clone();
                    length_list(a3182)
                };
                let a3184 = 1u64;
                nat_add(a3183, a3184)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h3181
        };
        h3180
    }))
}

pub fn f498(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3185 = v0;
    match m3185.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f90(v2.clone(), v1.clone()) {
                f498(v3, v1)
            } else {
                List::cons(v2, f498(v3, v1))
            }
        }
    }
}

pub fn f499(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f500(v0: List<Adt14>) -> List<Bytes> {
    let m3186 = v0;
    match m3186.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m3187 = f168(v1, f8(Str::lit("std")));
            match m3187 {
                None => {
                    f500(v2)
                }
                Some(v3) => {
                    List::cons(f166(v3, f8(Str::lit("authority"))), f500(v2))
                }
            }
        }
    }
}

pub fn f501(v0: List<Adt14>) -> List<Bytes> {
    let m3188 = v0;
    match m3188.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m3189 = f169(f168(v1, f8(Str::lit("req"))));
            match m3189 {
                None => {
                    f501(v2)
                }
                Some(v3) => {
                    List::cons(v3, f501(v2))
                }
            }
        }
    }
}

pub fn f502(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f543(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h3190) => {
            h3190
        }
    });
    let v2: u32 = f462(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h3191, _, _) => {
            h3191
        }
    }, f462(f544(match v0.clone() {
        Adt27::C0(_, _, _, _, h3192, _, _, _, _, _) => {
            h3192
        }
    }), f462(f545(match v0.clone() {
        Adt27::C0(_, _, _, _, h3193, _, _, _, _, _) => {
            h3193
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h3194, _, _, _, _) => {
            h3194
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h3195, _, _, _) => {
            h3195
        }
    }), f462(v1, {
        let m3198 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h3197, _) => {
                h3197
            }
        };
        match m3198 {
            Adt26::C0(h3196, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3196
            }
        }
    }))));
    let v3: u32 = f462(f546(match v0.clone() {
        Adt27::C0(h3199, _, _, _, _, _, _, _, _, _) => {
            h3199
        }
    }), f547(match v0.clone() {
        Adt27::C0(h3200, _, _, _, _, _, _, _, _, _) => {
            h3200
        }
    }, match v0.clone() {
        Adt27::C0(_, h3201, _, _, _, _, _, _, _, _) => {
            h3201
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h3202, _, _, _, _, _, _, _) => {
            h3202
        }
    }));
    Ok(Adt27::C0(f462(v2, v3), match v0.clone() {
        Adt27::C0(h3203, _, _, _, _, _, _, _, _, _) => {
            h3203
        }
    }, match v0.clone() {
        Adt27::C0(_, h3204, _, _, _, _, _, _, _, _) => {
            h3204
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h3205, _, _, _, _, _, _, _) => {
            h3205
        }
    }, f462(match v0.clone() {
        Adt27::C0(_, _, _, h3206, _, _, _, _, _, _) => {
            h3206
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h3207, _, _, _, _, _) => {
            h3207
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h3208, _, _, _, _) => {
            h3208
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h3209, _, _, _) => {
            h3209
        }
    }, f548(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h3210, _) => {
            h3210
        }
    }), {
        let a3212 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h3211) => {
                h3211
            }
        };
        let a3213 = 1u64;
        nat_add(a3212, a3213)?
    }))
}

pub fn f503(v0: u32) -> u64 {
    let m3215 = {
        let a3214 = v0;
        fixed_u64::convert(i128::from(a3214))
    };
    match m3215 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f504(v0: u64) -> u32 {
    let m3219 = {
        let a3218 = {
            let a3216 = v0;
            let a3217 = 4294967295u64;
            fixed_u64::bit_and(a3216, a3217)
        };
        fixed_u32::convert(i128::from(a3218))
    };
    match m3219 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f505(v0: u64) -> u32 {
    let (_, h3220) = {
        let (_, h3221) = f549(Fn20::F550, 32u64, (v0, (1u32, 0u32)));
        h3221
    };
    h3220
}

pub fn f506(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3226 = f551(v0, 24u32);
    let a3227 = {
        let a3224 = f551(v1, 16u32);
        let a3225 = {
            let a3222 = f551(v2, 8u32);
            let a3223 = v3;
            fixed_u32::bit_or(a3222, a3223)
        };
        fixed_u32::bit_or(a3224, a3225)
    };
    fixed_u32::bit_or(a3226, a3227)
}

pub fn f507(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3228 = v2;
    let a3229 = v1;
    if nat_lt(a3228, a3229) {
        let m3232 = {
            let a3230 = v0;
            let a3231 = v2;
            index_bytes(a3230, a3231)
        };
        match m3232 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3234 = {
                    let a3233 = v3;
                    fixed_u32::convert(i128::from(a3233))
                };
                match m3234 {
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
        let a3235 = v2;
        let a3236 = v1;
        if nat_eq(a3235, a3236) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f508(v0: u8) -> bool {
    let a3237 = v0;
    let a3238 = 32u8;
    if if if equal(a3237, a3238) {
        true
    } else {
        let a3239 = v0;
        let a3240 = 9u8;
        equal(a3239, a3240)
    } {
        true
    } else {
        let a3241 = v0;
        let a3242 = 10u8;
        equal(a3241, a3242)
    } {
        true
    } else {
        let a3243 = v0;
        let a3244 = 12u8;
        if equal(a3243, a3244) {
            true
        } else {
            let a3245 = v0;
            let a3246 = 13u8;
            equal(a3245, a3246)
        }
    }
}

pub fn f509(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3249 = {
        let a3247 = v0;
        let a3248 = v1;
        index_bytes(a3247, a3248)
    };
    match m3249 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3294 = {
                    let a3270 = {
                        let a3258 = {
                            let a3252 = {
                                let a3250 = v3;
                                let a3251 = 128u8;
                                fixed_u8::bit_and(a3250, a3251)
                            };
                            let a3253 = 128u8;
                            if equal(a3252, a3253) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3259 = {
                            let a3256 = {
                                let a3254 = v3;
                                let a3255 = 64u8;
                                fixed_u8::bit_and(a3254, a3255)
                            };
                            let a3257 = 64u8;
                            if equal(a3256, a3257) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3258, a3259)?
                    };
                    let a3271 = {
                        let a3268 = {
                            let a3262 = {
                                let a3260 = v3;
                                let a3261 = 32u8;
                                fixed_u8::bit_and(a3260, a3261)
                            };
                            let a3263 = 32u8;
                            if equal(a3262, a3263) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3269 = {
                            let a3266 = {
                                let a3264 = v3;
                                let a3265 = 16u8;
                                fixed_u8::bit_and(a3264, a3265)
                            };
                            let a3267 = 16u8;
                            if equal(a3266, a3267) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3268, a3269)?
                    };
                    nat_add(a3270, a3271)?
                };
                let a3295 = {
                    let a3292 = {
                        let a3280 = {
                            let a3274 = {
                                let a3272 = v3;
                                let a3273 = 8u8;
                                fixed_u8::bit_and(a3272, a3273)
                            };
                            let a3275 = 8u8;
                            if equal(a3274, a3275) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3281 = {
                            let a3278 = {
                                let a3276 = v3;
                                let a3277 = 4u8;
                                fixed_u8::bit_and(a3276, a3277)
                            };
                            let a3279 = 4u8;
                            if equal(a3278, a3279) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3280, a3281)?
                    };
                    let a3293 = {
                        let a3290 = {
                            let a3284 = {
                                let a3282 = v3;
                                let a3283 = 2u8;
                                fixed_u8::bit_and(a3282, a3283)
                            };
                            let a3285 = 2u8;
                            if equal(a3284, a3285) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3291 = {
                            let a3288 = {
                                let a3286 = v3;
                                let a3287 = 1u8;
                                fixed_u8::bit_and(a3286, a3287)
                            };
                            let a3289 = 1u8;
                            if equal(a3288, a3289) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3290, a3291)?
                    };
                    nat_add(a3292, a3293)?
                };
                nat_add(a3294, a3295)?
            };
            let a3296 = 128u64;
            let a3297 = v4;
            if if nat_le(a3296, a3297) {
                let a3298 = v4;
                let a3299 = 191u64;
                nat_le(a3298, a3299)
            } else {
                false
            } {
                Ok(Some({
                    let a3300 = v4;
                    let a3301 = 128u64;
                    nat_sub(a3300, a3301)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f510(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3302 = v1;
    let a3303 = v0;
    if if nat_le(a3302, a3303) {
        let a3304 = v0;
        let a3305 = v2;
        if nat_le(a3304, a3305) {
            let a3306 = v0;
            let a3307 = 55296u64;
            if nat_lt(a3306, a3307) {
                true
            } else {
                let a3308 = 57343u64;
                let a3309 = v0;
                nat_lt(a3308, a3309)
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

pub fn f511(v0: u64) -> bool {
    let a3310 = v0;
    let a3311 = 58u64;
    if if if nat_eq(a3310, a3311) {
        true
    } else {
        let a3312 = v0;
        let a3313 = 95u64;
        nat_eq(a3312, a3313)
    } {
        true
    } else {
        let a3314 = 65u64;
        let a3315 = v0;
        if if nat_le(a3314, a3315) {
            let a3316 = v0;
            let a3317 = 90u64;
            nat_le(a3316, a3317)
        } else {
            false
        } {
            true
        } else {
            let a3318 = 97u64;
            let a3319 = v0;
            if nat_le(a3318, a3319) {
                let a3320 = v0;
                let a3321 = 122u64;
                nat_le(a3320, a3321)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a3322 = 192u64;
        let a3323 = v0;
        if if if if nat_le(a3322, a3323) {
            let a3324 = v0;
            let a3325 = 214u64;
            nat_le(a3324, a3325)
        } else {
            false
        } {
            true
        } else {
            let a3326 = 216u64;
            let a3327 = v0;
            if nat_le(a3326, a3327) {
                let a3328 = v0;
                let a3329 = 246u64;
                nat_le(a3328, a3329)
            } else {
                false
            }
        } {
            true
        } else {
            let a3330 = 248u64;
            let a3331 = v0;
            if if nat_le(a3330, a3331) {
                let a3332 = v0;
                let a3333 = 767u64;
                nat_le(a3332, a3333)
            } else {
                false
            } {
                true
            } else {
                let a3334 = 880u64;
                let a3335 = v0;
                if nat_le(a3334, a3335) {
                    let a3336 = v0;
                    let a3337 = 893u64;
                    nat_le(a3336, a3337)
                } else {
                    false
                }
            }
        } {
            true
        } else {
            let a3338 = 895u64;
            let a3339 = v0;
            if if if if nat_le(a3338, a3339) {
                let a3340 = v0;
                let a3341 = 8191u64;
                nat_le(a3340, a3341)
            } else {
                false
            } {
                true
            } else {
                let a3342 = 8204u64;
                let a3343 = v0;
                if nat_le(a3342, a3343) {
                    let a3344 = v0;
                    let a3345 = 8205u64;
                    nat_le(a3344, a3345)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3346 = 8304u64;
                let a3347 = v0;
                if if nat_le(a3346, a3347) {
                    let a3348 = v0;
                    let a3349 = 8591u64;
                    nat_le(a3348, a3349)
                } else {
                    false
                } {
                    true
                } else {
                    let a3350 = 11264u64;
                    let a3351 = v0;
                    if nat_le(a3350, a3351) {
                        let a3352 = v0;
                        let a3353 = 12271u64;
                        nat_le(a3352, a3353)
                    } else {
                        false
                    }
                }
            } {
                true
            } else {
                let a3354 = 12289u64;
                let a3355 = v0;
                if if if nat_le(a3354, a3355) {
                    let a3356 = v0;
                    let a3357 = 55295u64;
                    nat_le(a3356, a3357)
                } else {
                    false
                } {
                    true
                } else {
                    let a3358 = 63744u64;
                    let a3359 = v0;
                    if nat_le(a3358, a3359) {
                        let a3360 = v0;
                        let a3361 = 64975u64;
                        nat_le(a3360, a3361)
                    } else {
                        false
                    }
                } {
                    true
                } else {
                    let a3362 = 65008u64;
                    let a3363 = v0;
                    if if nat_le(a3362, a3363) {
                        let a3364 = v0;
                        let a3365 = 65533u64;
                        nat_le(a3364, a3365)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a3366 = 65536u64;
                        let a3367 = v0;
                        if nat_le(a3366, a3367) {
                            let a3368 = v0;
                            let a3369 = 983039u64;
                            nat_le(a3368, a3369)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f512(v0: u64) -> bool {
    if f511(v0) {
        true
    } else {
        let a3370 = v0;
        let a3371 = 45u64;
        if if if nat_eq(a3370, a3371) {
            true
        } else {
            let a3372 = v0;
            let a3373 = 46u64;
            nat_eq(a3372, a3373)
        } {
            true
        } else {
            let a3374 = 48u64;
            let a3375 = v0;
            if nat_le(a3374, a3375) {
                let a3376 = v0;
                let a3377 = 57u64;
                nat_le(a3376, a3377)
            } else {
                false
            }
        } {
            true
        } else {
            let a3378 = v0;
            let a3379 = 183u64;
            if if nat_eq(a3378, a3379) {
                true
            } else {
                let a3380 = 768u64;
                let a3381 = v0;
                if nat_le(a3380, a3381) {
                    let a3382 = v0;
                    let a3383 = 879u64;
                    nat_le(a3382, a3383)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3384 = 8255u64;
                let a3385 = v0;
                if nat_le(a3384, a3385) {
                    let a3386 = v0;
                    let a3387 = 8256u64;
                    nat_le(a3386, a3387)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f513(v0: Fn16, p3392: u64, p3393: (u64, u64)) -> R<((u64, u64), bool)> {
    let m3391 = (p3392, p3393);
    {
        let mut m3391 = m3391;
        loop {
            let step = {
                let (v1, v2) = m3391;
                let m3388 = v1;
                if m3388 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3388 - 1;
                    let m3390 = {
                        let c3389: Fn16 = v0.clone();
                        c3389.apply(v2)?
                    };
                    match m3390 {
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
                Ok(next) => { m3391 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f514(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f552(v1, v0, v2)
}

pub fn f515(v0: u64) -> Bytes {
    f553(f554({
        let a3394 = v0;
        let a3395 = 16u64;
        let a3396 = 0u64;
        nat_quot(a3394, a3395, a3396)
    }), {
        let a3397 = v0;
        let a3398 = 16u64;
        let a3399 = 0u64;
        nat_rem(a3397, a3398, a3399)
    })
}

pub fn f516(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f555(f238(v0, v3)?, v1, v2) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3400 = v3;
            let a3401 = 1u64;
            nat_add(a3400, a3401)?
        }))
    }
}

pub fn f517() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f518(v0: Bytes) -> Option<Bytes> {
    if f412(v0.clone(), List::cons(f8(Str::lit("string")), List::cons(f8(Str::lit("boolean")), List::cons(f8(Str::lit("decimal")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::cons(f8(Str::lit("duration")), List::cons(f8(Str::lit("dateTime")), List::cons(f8(Str::lit("time")), List::cons(f8(Str::lit("date")), List::cons(f8(Str::lit("gYearMonth")), List::cons(f8(Str::lit("gYear")), List::cons(f8(Str::lit("gMonthDay")), List::cons(f8(Str::lit("gDay")), List::cons(f8(Str::lit("gMonth")), List::cons(f8(Str::lit("hexBinary")), List::cons(f8(Str::lit("base64Binary")), List::cons(f8(Str::lit("anyURI")), List::cons(f8(Str::lit("QName")), List::cons(f8(Str::lit("NOTATION")), List::<Bytes>::nil())))))))))))))))))))) {
        Some(f8(Str::lit("anyAtomicType")))
    } else {
        if f37(v0.clone(), f8(Str::lit("normalizedString"))) {
            Some(f8(Str::lit("string")))
        } else {
            if f37(v0.clone(), f8(Str::lit("token"))) {
                Some(f8(Str::lit("normalizedString")))
            } else {
                if f412(v0.clone(), List::cons(f8(Str::lit("language")), List::cons(f8(Str::lit("NMTOKEN")), List::cons(f8(Str::lit("Name")), List::<Bytes>::nil())))) {
                    Some(f8(Str::lit("token")))
                } else {
                    if f37(v0.clone(), f8(Str::lit("NCName"))) {
                        Some(f8(Str::lit("Name")))
                    } else {
                        if f412(v0.clone(), List::cons(f8(Str::lit("ID")), List::cons(f8(Str::lit("IDREF")), List::cons(f8(Str::lit("ENTITY")), List::<Bytes>::nil())))) {
                            Some(f8(Str::lit("NCName")))
                        } else {
                            if f37(v0.clone(), f8(Str::lit("integer"))) {
                                Some(f8(Str::lit("decimal")))
                            } else {
                                if f412(v0.clone(), List::cons(f8(Str::lit("nonPositiveInteger")), List::cons(f8(Str::lit("long")), List::cons(f8(Str::lit("nonNegativeInteger")), List::<Bytes>::nil())))) {
                                    Some(f8(Str::lit("integer")))
                                } else {
                                    if f37(v0.clone(), f8(Str::lit("negativeInteger"))) {
                                        Some(f8(Str::lit("nonPositiveInteger")))
                                    } else {
                                        if f37(v0.clone(), f8(Str::lit("int"))) {
                                            Some(f8(Str::lit("long")))
                                        } else {
                                            if f37(v0.clone(), f8(Str::lit("short"))) {
                                                Some(f8(Str::lit("int")))
                                            } else {
                                                if f37(v0.clone(), f8(Str::lit("byte"))) {
                                                    Some(f8(Str::lit("short")))
                                                } else {
                                                    if f412(v0.clone(), List::cons(f8(Str::lit("unsignedLong")), List::cons(f8(Str::lit("positiveInteger")), List::<Bytes>::nil()))) {
                                                        Some(f8(Str::lit("nonNegativeInteger")))
                                                    } else {
                                                        if f37(v0.clone(), f8(Str::lit("unsignedInt"))) {
                                                            Some(f8(Str::lit("unsignedLong")))
                                                        } else {
                                                            if f37(v0.clone(), f8(Str::lit("unsignedShort"))) {
                                                                Some(f8(Str::lit("unsignedInt")))
                                                            } else {
                                                                if f37(v0.clone(), f8(Str::lit("unsignedByte"))) {
                                                                    Some(f8(Str::lit("unsignedShort")))
                                                                } else {
                                                                    if f412(v0.clone(), List::cons(f8(Str::lit("yearMonthDuration")), List::cons(f8(Str::lit("dayTimeDuration")), List::<Bytes>::nil()))) {
                                                                        Some(f8(Str::lit("duration")))
                                                                    } else {
                                                                        if f37(v0.clone(), f8(Str::lit("dateTimeStamp"))) {
                                                                            Some(f8(Str::lit("dateTime")))
                                                                        } else {
                                                                            if f412(v0, List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))))) {
                                                                                Some(f8(Str::lit("anySimpleType")))
                                                                            } else {
                                                                                None::<Bytes>
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

pub fn f519(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f520(v1)
    } else {
        f476(v1)
    }
}

pub fn f520(v0: Bytes) -> R<Option<Bytes>> {
    let m3403 = f556({
        let a3402 = v0.clone();
        length_bytes(a3402)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m3403 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3405 = {
                let a3404 = v1.clone();
                length_bytes(a3404)
            };
            let a3406 = 0u64;
            if nat_eq(a3405, a3406) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f521(p3462: u64, v1: Bytes, p3463: u64) -> R<bool> {
    let m3461 = (p3462, p3463);
    {
        let mut m3461 = m3461;
        loop {
            let step = {
                let (v0, v2) = m3461;
                let m3407 = v0;
                if m3407 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3407 - 1;
                    let m3410 = {
                        let a3408 = v1.clone();
                        let a3409 = v2;
                        index_bytes(a3408, a3409)
                    };
                    match m3410 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3457 = {
                                let v5: u8 = v4;
                                let a3455 = {
                                    let a3431 = {
                                        let a3419 = {
                                            let a3413 = {
                                                let a3411 = v5;
                                                let a3412 = 128u8;
                                                fixed_u8::bit_and(a3411, a3412)
                                            };
                                            let a3414 = 128u8;
                                            if equal(a3413, a3414) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3420 = {
                                            let a3417 = {
                                                let a3415 = v5;
                                                let a3416 = 64u8;
                                                fixed_u8::bit_and(a3415, a3416)
                                            };
                                            let a3418 = 64u8;
                                            if equal(a3417, a3418) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3419, a3420)?
                                    };
                                    let a3432 = {
                                        let a3429 = {
                                            let a3423 = {
                                                let a3421 = v5;
                                                let a3422 = 32u8;
                                                fixed_u8::bit_and(a3421, a3422)
                                            };
                                            let a3424 = 32u8;
                                            if equal(a3423, a3424) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3430 = {
                                            let a3427 = {
                                                let a3425 = v5;
                                                let a3426 = 16u8;
                                                fixed_u8::bit_and(a3425, a3426)
                                            };
                                            let a3428 = 16u8;
                                            if equal(a3427, a3428) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3429, a3430)?
                                    };
                                    nat_add(a3431, a3432)?
                                };
                                let a3456 = {
                                    let a3453 = {
                                        let a3441 = {
                                            let a3435 = {
                                                let a3433 = v5;
                                                let a3434 = 8u8;
                                                fixed_u8::bit_and(a3433, a3434)
                                            };
                                            let a3436 = 8u8;
                                            if equal(a3435, a3436) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3442 = {
                                            let a3439 = {
                                                let a3437 = v5;
                                                let a3438 = 4u8;
                                                fixed_u8::bit_and(a3437, a3438)
                                            };
                                            let a3440 = 4u8;
                                            if equal(a3439, a3440) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3441, a3442)?
                                    };
                                    let a3454 = {
                                        let a3451 = {
                                            let a3445 = {
                                                let a3443 = v5;
                                                let a3444 = 2u8;
                                                fixed_u8::bit_and(a3443, a3444)
                                            };
                                            let a3446 = 2u8;
                                            if equal(a3445, a3446) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3452 = {
                                            let a3449 = {
                                                let a3447 = v5;
                                                let a3448 = 1u8;
                                                fixed_u8::bit_and(a3447, a3448)
                                            };
                                            let a3450 = 1u8;
                                            if equal(a3449, a3450) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3451, a3452)?
                                    };
                                    nat_add(a3453, a3454)?
                                };
                                nat_add(a3455, a3456)?
                            };
                            let a3458 = 128u64;
                            if nat_lt(a3457, a3458) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3459 = v2;
                                    let a3460 = 1u64;
                                    nat_add(a3459, a3460)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f522(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f364(v0.clone(), 0u64, {
        let a3464 = v0;
        length_bytes(a3464)
    }, false, Bytes::lit(&[]))?;
    let m3467 = {
        let a3465 = v1.clone();
        let a3466 = 0u64;
        index_bytes(a3465, a3466)
    };
    match m3467 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3512 = {
                    let a3488 = {
                        let a3476 = {
                            let a3470 = {
                                let a3468 = v3;
                                let a3469 = 128u8;
                                fixed_u8::bit_and(a3468, a3469)
                            };
                            let a3471 = 128u8;
                            if equal(a3470, a3471) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3477 = {
                            let a3474 = {
                                let a3472 = v3;
                                let a3473 = 64u8;
                                fixed_u8::bit_and(a3472, a3473)
                            };
                            let a3475 = 64u8;
                            if equal(a3474, a3475) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3476, a3477)?
                    };
                    let a3489 = {
                        let a3486 = {
                            let a3480 = {
                                let a3478 = v3;
                                let a3479 = 32u8;
                                fixed_u8::bit_and(a3478, a3479)
                            };
                            let a3481 = 32u8;
                            if equal(a3480, a3481) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3487 = {
                            let a3484 = {
                                let a3482 = v3;
                                let a3483 = 16u8;
                                fixed_u8::bit_and(a3482, a3483)
                            };
                            let a3485 = 16u8;
                            if equal(a3484, a3485) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3486, a3487)?
                    };
                    nat_add(a3488, a3489)?
                };
                let a3513 = {
                    let a3510 = {
                        let a3498 = {
                            let a3492 = {
                                let a3490 = v3;
                                let a3491 = 8u8;
                                fixed_u8::bit_and(a3490, a3491)
                            };
                            let a3493 = 8u8;
                            if equal(a3492, a3493) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3499 = {
                            let a3496 = {
                                let a3494 = v3;
                                let a3495 = 4u8;
                                fixed_u8::bit_and(a3494, a3495)
                            };
                            let a3497 = 4u8;
                            if equal(a3496, a3497) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3498, a3499)?
                    };
                    let a3511 = {
                        let a3508 = {
                            let a3502 = {
                                let a3500 = v3;
                                let a3501 = 2u8;
                                fixed_u8::bit_and(a3500, a3501)
                            };
                            let a3503 = 2u8;
                            if equal(a3502, a3503) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3509 = {
                            let a3506 = {
                                let a3504 = v3;
                                let a3505 = 1u8;
                                fixed_u8::bit_and(a3504, a3505)
                            };
                            let a3507 = 1u8;
                            if equal(a3506, a3507) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3508, a3509)?
                    };
                    nat_add(a3510, a3511)?
                };
                nat_add(a3512, a3513)?
            };
            if f433(v4) {
                let a3520 = f32({
                    let a3514 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3514)
                }, {
                    let a3515 = v4;
                    let a3516 = 65u64;
                    nat_sub(a3515, a3516)
                }, 1u64);
                let a3521 = f32(v1.clone(), 1u64, {
                    let a3518 = {
                        let a3517 = v1;
                        length_bytes(a3517)
                    };
                    let a3519 = 1u64;
                    nat_sub(a3518, a3519)
                });
                Ok(append_bytes(a3520, a3521))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f523(v0: Bytes) -> R<bool> {
    let m3524 = {
        let a3522 = v0;
        let a3523 = 0u64;
        index_bytes(a3522, a3523)
    };
    match m3524 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f432({
                let v2: u8 = v1;
                let a3569 = {
                    let a3545 = {
                        let a3533 = {
                            let a3527 = {
                                let a3525 = v2;
                                let a3526 = 128u8;
                                fixed_u8::bit_and(a3525, a3526)
                            };
                            let a3528 = 128u8;
                            if equal(a3527, a3528) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3534 = {
                            let a3531 = {
                                let a3529 = v2;
                                let a3530 = 64u8;
                                fixed_u8::bit_and(a3529, a3530)
                            };
                            let a3532 = 64u8;
                            if equal(a3531, a3532) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3533, a3534)?
                    };
                    let a3546 = {
                        let a3543 = {
                            let a3537 = {
                                let a3535 = v2;
                                let a3536 = 32u8;
                                fixed_u8::bit_and(a3535, a3536)
                            };
                            let a3538 = 32u8;
                            if equal(a3537, a3538) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3544 = {
                            let a3541 = {
                                let a3539 = v2;
                                let a3540 = 16u8;
                                fixed_u8::bit_and(a3539, a3540)
                            };
                            let a3542 = 16u8;
                            if equal(a3541, a3542) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3543, a3544)?
                    };
                    nat_add(a3545, a3546)?
                };
                let a3570 = {
                    let a3567 = {
                        let a3555 = {
                            let a3549 = {
                                let a3547 = v2;
                                let a3548 = 8u8;
                                fixed_u8::bit_and(a3547, a3548)
                            };
                            let a3550 = 8u8;
                            if equal(a3549, a3550) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3556 = {
                            let a3553 = {
                                let a3551 = v2;
                                let a3552 = 4u8;
                                fixed_u8::bit_and(a3551, a3552)
                            };
                            let a3554 = 4u8;
                            if equal(a3553, a3554) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3555, a3556)?
                    };
                    let a3568 = {
                        let a3565 = {
                            let a3559 = {
                                let a3557 = v2;
                                let a3558 = 2u8;
                                fixed_u8::bit_and(a3557, a3558)
                            };
                            let a3560 = 2u8;
                            if equal(a3559, a3560) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3566 = {
                            let a3563 = {
                                let a3561 = v2;
                                let a3562 = 1u8;
                                fixed_u8::bit_and(a3561, a3562)
                            };
                            let a3564 = 1u8;
                            if equal(a3563, a3564) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3565, a3566)?
                    };
                    nat_add(a3567, a3568)?
                };
                nat_add(a3569, a3570)?
            }) {
                Ok(true)
            } else {
                Ok(f433({
                    let v3: u8 = v1;
                    let a3615 = {
                        let a3591 = {
                            let a3579 = {
                                let a3573 = {
                                    let a3571 = v3;
                                    let a3572 = 128u8;
                                    fixed_u8::bit_and(a3571, a3572)
                                };
                                let a3574 = 128u8;
                                if equal(a3573, a3574) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a3580 = {
                                let a3577 = {
                                    let a3575 = v3;
                                    let a3576 = 64u8;
                                    fixed_u8::bit_and(a3575, a3576)
                                };
                                let a3578 = 64u8;
                                if equal(a3577, a3578) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3579, a3580)?
                        };
                        let a3592 = {
                            let a3589 = {
                                let a3583 = {
                                    let a3581 = v3;
                                    let a3582 = 32u8;
                                    fixed_u8::bit_and(a3581, a3582)
                                };
                                let a3584 = 32u8;
                                if equal(a3583, a3584) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a3590 = {
                                let a3587 = {
                                    let a3585 = v3;
                                    let a3586 = 16u8;
                                    fixed_u8::bit_and(a3585, a3586)
                                };
                                let a3588 = 16u8;
                                if equal(a3587, a3588) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3589, a3590)?
                        };
                        nat_add(a3591, a3592)?
                    };
                    let a3616 = {
                        let a3613 = {
                            let a3601 = {
                                let a3595 = {
                                    let a3593 = v3;
                                    let a3594 = 8u8;
                                    fixed_u8::bit_and(a3593, a3594)
                                };
                                let a3596 = 8u8;
                                if equal(a3595, a3596) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a3602 = {
                                let a3599 = {
                                    let a3597 = v3;
                                    let a3598 = 4u8;
                                    fixed_u8::bit_and(a3597, a3598)
                                };
                                let a3600 = 4u8;
                                if equal(a3599, a3600) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3601, a3602)?
                        };
                        let a3614 = {
                            let a3611 = {
                                let a3605 = {
                                    let a3603 = v3;
                                    let a3604 = 2u8;
                                    fixed_u8::bit_and(a3603, a3604)
                                };
                                let a3606 = 2u8;
                                if equal(a3605, a3606) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a3612 = {
                                let a3609 = {
                                    let a3607 = v3;
                                    let a3608 = 1u8;
                                    fixed_u8::bit_and(a3607, a3608)
                                };
                                let a3610 = 1u8;
                                if equal(a3609, a3610) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3611, a3612)?
                        };
                        nat_add(a3613, a3614)?
                    };
                    nat_add(a3615, a3616)?
                }))
            }
        }
    }
}

pub fn f524(p3629: u64, v1: Bytes, p3630: u64, p3631: Bytes) -> R<Option<Bytes>> {
    let m3628 = (p3629, (p3630, p3631));
    {
        let mut m3628 = m3628;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3628;
                let m3617 = v0;
                if m3617 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3617 - 1;
                    let a3619 = {
                        let a3618 = v1.clone();
                        length_bytes(a3618)
                    };
                    let a3620 = v2;
                    if nat_le(a3619, a3620) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3621 = f465(v1.clone(), v2)?;
                        match m3621 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3623 = v2;
                                    let a3624 = {
                                        let (_, h3622) = v5;
                                        h3622
                                    };
                                    nat_add(a3623, a3624)?
                                }, {
                                    let a3626 = v3;
                                    let a3627 = f557({
                                        let (h3625, _) = v5;
                                        h3625
                                    });
                                    append_bytes(a3626, a3627)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3628 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h3632, _) = v1.clone();
        h3632
    };
    let v3: Bytes = {
        let (h3633, _) = {
            let (_, h3634) = v1.clone();
            h3634
        };
        h3633
    };
    let a3637 = {
        let (_, h3635) = {
            let (_, h3636) = v1;
            h3636
        };
        h3635
    };
    let a3638 = 0u64;
    if nat_eq(a3637, a3638) {
        let v4: u64 = f339(v0.clone(), v2)?;
        let a3639 = v4;
        let a3640 = 34u64;
        if nat_eq(a3639, a3640) {
            Ok(Some(({
                let a3641 = v2;
                let a3642 = 1u64;
                nat_add(a3641, a3642)?
            }, (v3, 1u64))))
        } else {
            let a3643 = v4;
            let a3644 = 92u64;
            if nat_eq(a3643, a3644) {
                f558(v0, v2, v3)
            } else {
                let a3645 = v4;
                let a3646 = 32u64;
                if if nat_lt(a3645, a3646) {
                    true
                } else {
                    let a3647 = v4;
                    let a3648 = 256u64;
                    nat_eq(a3647, a3648)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a3649 = v2;
                        let a3650 = 1u64;
                        nat_add(a3649, a3650)?
                    }, ({
                        let a3651 = v3;
                        let a3652 = f32(v0, v2, 1u64);
                        append_bytes(a3651, a3652)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f526(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f559(v0, v1)
}

pub fn f527(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f560(v0, v1)
}

pub fn f528(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3653 = v0.clone();
        length_bytes(a3653)
    };
    let a3654 = 0u64;
    let a3655 = v1;
    if nat_lt(a3654, a3655) {
        let a3656 = f561(v0.clone())?;
        let a3657 = v1;
        if nat_eq(a3656, a3657) {
            let a3658 = 1u64;
            let a3659 = v1;
            if nat_lt(a3658, a3659) {
                let m3662 = {
                    let a3660 = v0;
                    let a3661 = 0u64;
                    index_bytes(a3660, a3661)
                };
                match m3662 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3663 = v2;
                        let a3664 = 48u8;
                        Ok(!equal(a3663, a3664))
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

pub fn f529(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f562(v0, v1)
}

pub fn f530(v0: Bytes) -> R<u64> {
    let (h3665, _) = f280(Fn3::F563(v0.clone()), {
        let a3667 = {
            let a3666 = v0;
            length_bytes(a3666)
        };
        let a3668 = 1u64;
        nat_add(a3667, a3668)?
    }, 0u64)?;
    Ok(h3665)
}

pub fn f531(p3671: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m3670 = p3671;
    {
        let mut m3670 = m3670;
        loop {
            let step = {
                let v0 = m3670;
                let m3669 = v0;
                match m3669.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f37(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f90(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f90(v4, v3.clone())
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
                Ok(next) => { m3670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f532(v0: Adt19, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3672 = v0;
    match m3672 {
        Adt19::C0(_) => {
            Ok(v1)
        }
        Adt19::C1(_, _) => {
            Ok(v1)
        }
        Adt19::C2(_, _, _) => {
            Ok(v1)
        }
        Adt19::C3(_) => {
            Ok(v1)
        }
        Adt19::C4(r3675) => {
            let v2 = (*r3675).clone();
            f532(v2, v1)
        }
        Adt19::C5(r3676) => {
            let v3 = (*r3676).clone();
            f532(v3, v1)
        }
        Adt19::C6 => {
            Ok(v1)
        }
        Adt19::C7(r3677, r3678) => {
            let v4 = (*r3677).clone();
            let v5 = (*r3678).clone();
            f532(v4, f532(v5, v1)?)
        }
        Adt19::C8(v6, r3679, r3680) => {
            let v7 = (*r3679).clone();
            let v8 = (*r3680).clone();
            let v9: List<Bytes> = f532(v7.clone(), f532(v8, v1)?)?;
            if f37(v6, f8(Str::lit("$ref"))) {
                let m3673 = f269(Some(v7));
                match m3673 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m3674 = f564(v10)?;
                        match m3674 {
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

pub fn f533(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3681 = f156(v1.clone(), f8(Str::lit("enum")));
    match m3681 {
        None => {
            let m3683 = f156(v1.clone(), f8(Str::lit("oneOf")));
            match m3683 {
                None => {
                    if f569(v1.clone(), v0.clone())? {
                        let m3684 = f570(v0.clone(), f436(f156(v1.clone(), f8(Str::lit("properties")))), f160(f161(f156(v1, f8(Str::lit("required"))))), true)?;
                        match m3684 {
                            Ok(v7) => {
                                Ok(f567(f424(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f226({
                                    let (h3685, _) = v7;
                                    h3685
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h3686) = v7;
                                    h3686
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt25>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m3687 = f436(f156(v1.clone(), f8(Str::lit("properties"))));
                        match m3687.uncons() {
                            None => {
                                let m3690 = f571(v1, {
                                    let a3688 = v0.clone();
                                    let a3689 = f8(Str::lit("Value"));
                                    append_bytes(a3688, a3689)
                                })?;
                                match m3690 {
                                    Ok(v9) => {
                                        Ok(f567(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h3691, _) = v9;
                                            h3691
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3692) = v9;
                                            h3692
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt25>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m3693 = f570(v0.clone(), f436(f156(v1.clone(), f8(Str::lit("properties")))), f160(f161(f156(v1, f8(Str::lit("required"))))), false)?;
                                match m3693 {
                                    Ok(v13) => {
                                        Ok(f567(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m3695 = {
                                                let (h3694, _) = v13.clone();
                                                h3694
                                            };
                                            match m3695.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f226({
                                                        let (h3696, _) = v13;
                                                        h3696
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3697) = v13;
                                            h3697
                                        }))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<(Bytes, List<Adt25>), Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f568(v0, f270(v5))
                }
            }
        }
        Some(v2) => {
            let m3682 = f565(f270(v2), List::<Bytes>::nil())?;
            match m3682 {
                Ok(v4) => {
                    Ok(f567(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f226(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f566(v3, v0))
                }
            }
        }
    }
}

pub fn f534(v0: Adt19) -> Adt19 {
    let v2: Adt19 = {
        let m3698 = f156(v0.clone(), f8(Str::lit("anyOf")));
        match m3698 {
            None => {
                v0
            }
            Some(v1) => {
                if f572(f270(v1.clone())) {
                    f573(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f574(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f575(f270(v1.clone()))
                    } {
                        v0
                    } else {
                        f576(f573(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt19 = {
        let m3699 = f156(v2.clone(), f8(Str::lit("oneOf")));
        match m3699 {
            None => {
                v2
            }
            Some(v3) => {
                if f572(f270(v3)) {
                    f573(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m3700 = f156(v4.clone(), f8(Str::lit("allOf")));
    match m3700 {
        None => {
            v4
        }
        Some(v5) => {
            if f577(f270(v5)) {
                f573(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f535(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3745 = {
            let a3721 = {
                let a3709 = {
                    let a3703 = {
                        let a3701 = v3;
                        let a3702 = 128u8;
                        fixed_u8::bit_and(a3701, a3702)
                    };
                    let a3704 = 128u8;
                    if equal(a3703, a3704) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3710 = {
                    let a3707 = {
                        let a3705 = v3;
                        let a3706 = 64u8;
                        fixed_u8::bit_and(a3705, a3706)
                    };
                    let a3708 = 64u8;
                    if equal(a3707, a3708) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3709, a3710)?
            };
            let a3722 = {
                let a3719 = {
                    let a3713 = {
                        let a3711 = v3;
                        let a3712 = 32u8;
                        fixed_u8::bit_and(a3711, a3712)
                    };
                    let a3714 = 32u8;
                    if equal(a3713, a3714) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3720 = {
                    let a3717 = {
                        let a3715 = v3;
                        let a3716 = 16u8;
                        fixed_u8::bit_and(a3715, a3716)
                    };
                    let a3718 = 16u8;
                    if equal(a3717, a3718) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3719, a3720)?
            };
            nat_add(a3721, a3722)?
        };
        let a3746 = {
            let a3743 = {
                let a3731 = {
                    let a3725 = {
                        let a3723 = v3;
                        let a3724 = 8u8;
                        fixed_u8::bit_and(a3723, a3724)
                    };
                    let a3726 = 8u8;
                    if equal(a3725, a3726) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3732 = {
                    let a3729 = {
                        let a3727 = v3;
                        let a3728 = 4u8;
                        fixed_u8::bit_and(a3727, a3728)
                    };
                    let a3730 = 4u8;
                    if equal(a3729, a3730) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3731, a3732)?
            };
            let a3744 = {
                let a3741 = {
                    let a3735 = {
                        let a3733 = v3;
                        let a3734 = 2u8;
                        fixed_u8::bit_and(a3733, a3734)
                    };
                    let a3736 = 2u8;
                    if equal(a3735, a3736) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3742 = {
                    let a3739 = {
                        let a3737 = v3;
                        let a3738 = 1u8;
                        fixed_u8::bit_and(a3737, a3738)
                    };
                    let a3740 = 1u8;
                    if equal(a3739, a3740) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3741, a3742)?
            };
            nat_add(a3743, a3744)?
        };
        nat_add(a3745, a3746)?
    };
    let a3747 = v4;
    let a3748 = 34u64;
    if nat_eq(a3747, a3748) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3749 = v4;
        let a3750 = 92u64;
        if nat_eq(a3749, a3750) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3751 = v4;
            let a3752 = 8u64;
            if nat_eq(a3751, a3752) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3753 = v4;
                let a3754 = 12u64;
                if nat_eq(a3753, a3754) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3755 = v4;
                    let a3756 = 10u64;
                    if nat_eq(a3755, a3756) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3757 = v4;
                        let a3758 = 13u64;
                        if nat_eq(a3757, a3758) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3759 = v4;
                            let a3760 = 9u64;
                            if nat_eq(a3759, a3760) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3761 = v4;
                                let a3762 = 32u64;
                                if nat_lt(a3761, a3762) {
                                    let a3763 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3764 = f578(v2)?;
                                    Ok(append_bytes(a3763, a3764))
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

pub fn f536(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f579(Fn21::F580, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f537(v0: Fn17, p3768: List<List<(Bytes, Bytes)>>, p3769: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3767 = (p3768, p3769);
    {
        let mut m3767 = m3767;
        loop {
            let step = {
                let (v1, v2) = m3767;
                let m3765 = v2;
                match m3765.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3766: Fn17 = v0.clone();
                            c3766.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3767 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f539(v0: Fn18, p3774: u64, p3775: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3773 = (p3774, p3775);
    {
        let mut m3773 = m3773;
        loop {
            let step = {
                let (v1, v2) = m3773;
                let m3770 = v1;
                if m3770 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3770 - 1;
                    let m3772 = {
                        let c3771: Fn18 = v0.clone();
                        c3771.apply(v2.clone())?
                    };
                    match m3772 {
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
                Ok(next) => { m3773 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f581(v0)
}

pub fn f541(v0: Fn19, p3780: u64, p3781: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m3779 = (p3780, p3781);
    {
        let mut m3779 = m3779;
        loop {
            let step = {
                let (v1, v2) = m3779;
                let m3776 = v1;
                if m3776 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3776 - 1;
                    let m3778 = {
                        let c3777: Fn19 = v0.clone();
                        c3777.apply(v2.clone())?
                    };
                    match m3778 {
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
                Ok(next) => { m3779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f542(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f582(v0)
}

pub fn f543(v0: u64) -> u32 {
    let a3782 = v0;
    let a3783 = 32u64;
    if nat_lt(a3782, a3783) {
        let a3784 = v0;
        let a3785 = 16u64;
        if nat_lt(a3784, a3785) {
            let a3786 = v0;
            let a3787 = 8u64;
            if nat_lt(a3786, a3787) {
                let a3788 = v0;
                let a3789 = 4u64;
                if nat_lt(a3788, a3789) {
                    let a3790 = v0;
                    let a3791 = 2u64;
                    if nat_lt(a3790, a3791) {
                        let a3792 = v0;
                        let a3793 = 1u64;
                        if nat_lt(a3792, a3793) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3794 = v0;
                        let a3795 = 3u64;
                        if nat_lt(a3794, a3795) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3796 = v0;
                    let a3797 = 6u64;
                    if nat_lt(a3796, a3797) {
                        let a3798 = v0;
                        let a3799 = 5u64;
                        if nat_lt(a3798, a3799) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3800 = v0;
                        let a3801 = 7u64;
                        if nat_lt(a3800, a3801) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3802 = v0;
                let a3803 = 12u64;
                if nat_lt(a3802, a3803) {
                    let a3804 = v0;
                    let a3805 = 10u64;
                    if nat_lt(a3804, a3805) {
                        let a3806 = v0;
                        let a3807 = 9u64;
                        if nat_lt(a3806, a3807) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3808 = v0;
                        let a3809 = 11u64;
                        if nat_lt(a3808, a3809) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3810 = v0;
                    let a3811 = 14u64;
                    if nat_lt(a3810, a3811) {
                        let a3812 = v0;
                        let a3813 = 13u64;
                        if nat_lt(a3812, a3813) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3814 = v0;
                        let a3815 = 15u64;
                        if nat_lt(a3814, a3815) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3816 = v0;
            let a3817 = 24u64;
            if nat_lt(a3816, a3817) {
                let a3818 = v0;
                let a3819 = 20u64;
                if nat_lt(a3818, a3819) {
                    let a3820 = v0;
                    let a3821 = 18u64;
                    if nat_lt(a3820, a3821) {
                        let a3822 = v0;
                        let a3823 = 17u64;
                        if nat_lt(a3822, a3823) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3824 = v0;
                        let a3825 = 19u64;
                        if nat_lt(a3824, a3825) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3826 = v0;
                    let a3827 = 22u64;
                    if nat_lt(a3826, a3827) {
                        let a3828 = v0;
                        let a3829 = 21u64;
                        if nat_lt(a3828, a3829) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3830 = v0;
                        let a3831 = 23u64;
                        if nat_lt(a3830, a3831) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3832 = v0;
                let a3833 = 28u64;
                if nat_lt(a3832, a3833) {
                    let a3834 = v0;
                    let a3835 = 26u64;
                    if nat_lt(a3834, a3835) {
                        let a3836 = v0;
                        let a3837 = 25u64;
                        if nat_lt(a3836, a3837) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3838 = v0;
                        let a3839 = 27u64;
                        if nat_lt(a3838, a3839) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3840 = v0;
                    let a3841 = 30u64;
                    if nat_lt(a3840, a3841) {
                        let a3842 = v0;
                        let a3843 = 29u64;
                        if nat_lt(a3842, a3843) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3844 = v0;
                        let a3845 = 31u64;
                        if nat_lt(a3844, a3845) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3846 = v0;
        let a3847 = 48u64;
        if nat_lt(a3846, a3847) {
            let a3848 = v0;
            let a3849 = 40u64;
            if nat_lt(a3848, a3849) {
                let a3850 = v0;
                let a3851 = 36u64;
                if nat_lt(a3850, a3851) {
                    let a3852 = v0;
                    let a3853 = 34u64;
                    if nat_lt(a3852, a3853) {
                        let a3854 = v0;
                        let a3855 = 33u64;
                        if nat_lt(a3854, a3855) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3856 = v0;
                        let a3857 = 35u64;
                        if nat_lt(a3856, a3857) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3858 = v0;
                    let a3859 = 38u64;
                    if nat_lt(a3858, a3859) {
                        let a3860 = v0;
                        let a3861 = 37u64;
                        if nat_lt(a3860, a3861) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3862 = v0;
                        let a3863 = 39u64;
                        if nat_lt(a3862, a3863) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3864 = v0;
                let a3865 = 44u64;
                if nat_lt(a3864, a3865) {
                    let a3866 = v0;
                    let a3867 = 42u64;
                    if nat_lt(a3866, a3867) {
                        let a3868 = v0;
                        let a3869 = 41u64;
                        if nat_lt(a3868, a3869) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3870 = v0;
                        let a3871 = 43u64;
                        if nat_lt(a3870, a3871) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3872 = v0;
                    let a3873 = 46u64;
                    if nat_lt(a3872, a3873) {
                        let a3874 = v0;
                        let a3875 = 45u64;
                        if nat_lt(a3874, a3875) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3876 = v0;
                        let a3877 = 47u64;
                        if nat_lt(a3876, a3877) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3878 = v0;
            let a3879 = 56u64;
            if nat_lt(a3878, a3879) {
                let a3880 = v0;
                let a3881 = 52u64;
                if nat_lt(a3880, a3881) {
                    let a3882 = v0;
                    let a3883 = 50u64;
                    if nat_lt(a3882, a3883) {
                        let a3884 = v0;
                        let a3885 = 49u64;
                        if nat_lt(a3884, a3885) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3886 = v0;
                        let a3887 = 51u64;
                        if nat_lt(a3886, a3887) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3888 = v0;
                    let a3889 = 54u64;
                    if nat_lt(a3888, a3889) {
                        let a3890 = v0;
                        let a3891 = 53u64;
                        if nat_lt(a3890, a3891) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3892 = v0;
                        let a3893 = 55u64;
                        if nat_lt(a3892, a3893) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3894 = v0;
                let a3895 = 60u64;
                if nat_lt(a3894, a3895) {
                    let a3896 = v0;
                    let a3897 = 58u64;
                    if nat_lt(a3896, a3897) {
                        let a3898 = v0;
                        let a3899 = 57u64;
                        if nat_lt(a3898, a3899) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3900 = v0;
                        let a3901 = 59u64;
                        if nat_lt(a3900, a3901) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3902 = v0;
                    let a3903 = 62u64;
                    if nat_lt(a3902, a3903) {
                        let a3904 = v0;
                        let a3905 = 61u64;
                        if nat_lt(a3904, a3905) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3906 = v0;
                        let a3907 = 63u64;
                        if nat_lt(a3906, a3907) {
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

pub fn f544(v0: u32) -> u32 {
    let a3910 = {
        let a3908 = f583(v0, 6u32, 26u32);
        let a3909 = f583(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3908, a3909)
    };
    let a3911 = f583(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3910, a3911)
}

pub fn f545(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3917 = {
        let a3912 = v0;
        let a3913 = v1;
        fixed_u32::bit_and(a3912, a3913)
    };
    let a3918 = {
        let a3915 = {
            let a3914 = v0;
            fixed_u32::bit_not(a3914)
        };
        let a3916 = v2;
        fixed_u32::bit_and(a3915, a3916)
    };
    fixed_u32::bit_xor(a3917, a3918)
}

pub fn f546(v0: u32) -> u32 {
    let a3921 = {
        let a3919 = f583(v0, 2u32, 30u32);
        let a3920 = f583(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3919, a3920)
    };
    let a3922 = f583(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3921, a3922)
}

pub fn f547(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3931 = {
        let a3927 = {
            let a3923 = v0;
            let a3924 = v1;
            fixed_u32::bit_and(a3923, a3924)
        };
        let a3928 = {
            let a3925 = v0;
            let a3926 = v2;
            fixed_u32::bit_and(a3925, a3926)
        };
        fixed_u32::bit_xor(a3927, a3928)
    };
    let a3932 = {
        let a3929 = v1;
        let a3930 = v2;
        fixed_u32::bit_and(a3929, a3930)
    };
    fixed_u32::bit_xor(a3931, a3932)
}

pub fn f548(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3933, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3933
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3934, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3934
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3935, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3935
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3936, _, _, _, _, _, _, _, _, _, _, _) => {
            h3936
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3937, _, _, _, _, _, _, _, _, _, _) => {
            h3937
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3938, _, _, _, _, _, _, _, _, _) => {
            h3938
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3939, _, _, _, _, _, _, _, _) => {
            h3939
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3940, _, _, _, _, _, _, _) => {
            h3940
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3941, _, _, _, _, _, _) => {
            h3941
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3942, _, _, _, _, _) => {
            h3942
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3943, _, _, _, _) => {
            h3943
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3944, _, _, _) => {
            h3944
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3945, _, _) => {
            h3945
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3946, _) => {
            h3946
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3947) => {
            h3947
        }
    }, f462(f584(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3948, _) => {
            h3948
        }
    }), f462(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3949, _, _, _, _, _, _) => {
            h3949
        }
    }, f462(f585(match v0.clone() {
        Adt26::C0(_, h3950, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3950
        }
    }), match v0 {
        Adt26::C0(h3951, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3951
        }
    }))))
}

pub fn f549(v0: Fn20, p3955: u64, p3956: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3954 = (p3955, p3956);
    {
        let mut m3954 = m3954;
        loop {
            let step = {
                let (v1, v2) = m3954;
                let m3952 = v1;
                if m3952 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3952 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3953: Fn20 = v0.clone();
                        c3953.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3957, _) = v0;
        h3957
    };
    let v2: u32 = {
        let (h3958, _) = {
            let (_, h3959) = v0;
            h3959
        };
        h3958
    };
    let v3: u32 = {
        let (_, h3960) = {
            let (_, h3961) = v0;
            h3961
        };
        h3960
    };
    ({
        let a3962 = v1;
        let a3963 = 2u64;
        let a3964 = 0u64;
        nat_quot(a3962, a3963, a3964)
    }, (f551(v2, 1u32), {
        let a3968 = {
            let a3965 = v1;
            let a3966 = 2u64;
            let a3967 = 0u64;
            nat_rem(a3965, a3966, a3967)
        };
        let a3969 = 1u64;
        if nat_eq(a3968, a3969) {
            let a3970 = v3;
            let a3971 = v2;
            fixed_u32::bit_or(a3970, a3971)
        } else {
            v3
        }
    }))
}

pub fn f551(v0: u32, v1: u32) -> u32 {
    let m3974 = {
        let a3972 = v0;
        let a3973 = v1;
        fixed_u32::shift_left(a3972, a3973)
    };
    match m3974 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f552(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f238(v0, {
        let (h3975, _) = v2;
        h3975
    })?;
    let a3977 = 1114111u64;
    let a3978 = {
        let (_, h3976) = v2;
        h3976
    };
    if nat_lt(a3977, a3978) {
        Ok(None::<(u64, u64)>)
    } else {
        let a3979 = 48u64;
        let a3980 = v3;
        if if nat_le(a3979, a3980) {
            let a3981 = v3;
            let a3982 = 57u64;
            nat_le(a3981, a3982)
        } else {
            false
        } {
            Ok(Some(({
                let a3984 = {
                    let (h3983, _) = v2;
                    h3983
                };
                let a3985 = 1u64;
                nat_add(a3984, a3985)?
            }, {
                let a3991 = {
                    let a3987 = {
                        let (_, h3986) = v2;
                        h3986
                    };
                    let a3988 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3987, a3988)?
                };
                let a3992 = {
                    let a3989 = v3;
                    let a3990 = 48u64;
                    nat_sub(a3989, a3990)
                };
                nat_add(a3991, a3992)?
            })))
        } else {
            if if v1 {
                let a3993 = 97u64;
                let a3994 = v3;
                if nat_le(a3993, a3994) {
                    let a3995 = v3;
                    let a3996 = 102u64;
                    nat_le(a3995, a3996)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some(({
                    let a3998 = {
                        let (h3997, _) = v2;
                        h3997
                    };
                    let a3999 = 1u64;
                    nat_add(a3998, a3999)?
                }, {
                    let a4005 = {
                        let a4001 = {
                            let (_, h4000) = v2;
                            h4000
                        };
                        let a4002 = 16u64;
                        nat_mul(a4001, a4002)?
                    };
                    let a4006 = {
                        let a4003 = v3;
                        let a4004 = 87u64;
                        nat_sub(a4003, a4004)
                    };
                    nat_add(a4005, a4006)?
                })))
            } else {
                if if v1 {
                    let a4007 = 65u64;
                    let a4008 = v3;
                    if nat_le(a4007, a4008) {
                        let a4009 = v3;
                        let a4010 = 70u64;
                        nat_le(a4009, a4010)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(({
                        let a4012 = {
                            let (h4011, _) = v2;
                            h4011
                        };
                        let a4013 = 1u64;
                        nat_add(a4012, a4013)?
                    }, {
                        let a4019 = {
                            let a4015 = {
                                let (_, h4014) = v2;
                                h4014
                            };
                            let a4016 = 16u64;
                            nat_mul(a4015, a4016)?
                        };
                        let a4020 = {
                            let a4017 = v3;
                            let a4018 = 55u64;
                            nat_sub(a4017, a4018)
                        };
                        nat_add(a4019, a4020)?
                    })))
                } else {
                    Ok(None::<(u64, u64)>)
                }
            }
        }
    }
}

pub fn f553(p4027: List<Bytes>, p4028: u64) -> Bytes {
    let m4026 = (p4027, p4028);
    {
        let mut m4026 = m4026;
        loop {
            let step = {
                let (v0, v1) = m4026;
                let m4021 = v0;
                match m4021.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a4022 = v1;
                        let a4023 = 0u64;
                        if nat_eq(a4022, a4023) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a4024 = v1;
                                let a4025 = 1u64;
                                nat_sub(a4024, a4025)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: u64) -> List<Bytes> {
    let a4029 = v0;
    let a4030 = 0u64;
    if nat_eq(a4029, a4030) {
        f586()
    } else {
        let a4031 = v0;
        let a4032 = 1u64;
        if nat_eq(a4031, a4032) {
            f587()
        } else {
            let a4033 = v0;
            let a4034 = 2u64;
            if nat_eq(a4033, a4034) {
                f588()
            } else {
                let a4035 = v0;
                let a4036 = 3u64;
                if nat_eq(a4035, a4036) {
                    f589()
                } else {
                    let a4037 = v0;
                    let a4038 = 4u64;
                    if nat_eq(a4037, a4038) {
                        f590()
                    } else {
                        let a4039 = v0;
                        let a4040 = 5u64;
                        if nat_eq(a4039, a4040) {
                            f591()
                        } else {
                            let a4041 = v0;
                            let a4042 = 6u64;
                            if nat_eq(a4041, a4042) {
                                f592()
                            } else {
                                let a4043 = v0;
                                let a4044 = 7u64;
                                if nat_eq(a4043, a4044) {
                                    f593()
                                } else {
                                    let a4045 = v0;
                                    let a4046 = 8u64;
                                    if nat_eq(a4045, a4046) {
                                        f594()
                                    } else {
                                        let a4047 = v0;
                                        let a4048 = 9u64;
                                        if nat_eq(a4047, a4048) {
                                            f595()
                                        } else {
                                            let a4049 = v0;
                                            let a4050 = 10u64;
                                            if nat_eq(a4049, a4050) {
                                                f596()
                                            } else {
                                                let a4051 = v0;
                                                let a4052 = 11u64;
                                                if nat_eq(a4051, a4052) {
                                                    f597()
                                                } else {
                                                    let a4053 = v0;
                                                    let a4054 = 12u64;
                                                    if nat_eq(a4053, a4054) {
                                                        f598()
                                                    } else {
                                                        let a4055 = v0;
                                                        let a4056 = 13u64;
                                                        if nat_eq(a4055, a4056) {
                                                            f599()
                                                        } else {
                                                            let a4057 = v0;
                                                            let a4058 = 14u64;
                                                            if nat_eq(a4057, a4058) {
                                                                f600()
                                                            } else {
                                                                f601()
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

pub fn f555(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        let a4059 = v0;
        let a4060 = v2;
        if if if nat_eq(a4059, a4060) {
            true
        } else {
            let a4061 = v0;
            let a4062 = 60u64;
            nat_eq(a4061, a4062)
        } {
            true
        } else {
            let a4063 = v0;
            let a4064 = 38u64;
            if nat_eq(a4063, a4064) {
                true
            } else {
                let a4065 = v0;
                let a4066 = 256u64;
                nat_eq(a4065, a4066)
            }
        } {
            true
        } else {
            let a4067 = v0;
            let a4068 = 9u64;
            if nat_eq(a4067, a4068) {
                true
            } else {
                let a4069 = v0;
                let a4070 = 10u64;
                nat_eq(a4069, a4070)
            }
        }
    } else {
        let a4071 = v0;
        let a4072 = 60u64;
        if if nat_eq(a4071, a4072) {
            true
        } else {
            let a4073 = v0;
            let a4074 = 38u64;
            nat_eq(a4073, a4074)
        } {
            true
        } else {
            let a4075 = v0;
            let a4076 = 93u64;
            if nat_eq(a4075, a4076) {
                true
            } else {
                let a4077 = v0;
                let a4078 = 256u64;
                nat_eq(a4077, a4078)
            }
        }
    }
}

pub fn f556(p4091: u64, v1: Bytes, p4092: u64, p4093: Bytes) -> R<Option<Bytes>> {
    let m4090 = (p4091, (p4092, p4093));
    {
        let mut m4090 = m4090;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4090;
                let m4079 = v0;
                if m4079 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4079 - 1;
                    let a4081 = {
                        let a4080 = v1.clone();
                        length_bytes(a4080)
                    };
                    let a4082 = v2;
                    if nat_le(a4081, a4082) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4083 = f465(v1.clone(), v2)?;
                        match m4083 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4085 = v2;
                                    let a4086 = {
                                        let (_, h4084) = v5;
                                        h4084
                                    };
                                    nat_add(a4085, a4086)?
                                }, {
                                    let a4088 = v3;
                                    let a4089 = f602(v1.clone(), v2, {
                                        let (h4087, _) = v5;
                                        h4087
                                    });
                                    append_bytes(a4088, a4089)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4090 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f557(v0: u64) -> Bytes {
    let a4094 = f8(Str::lit("u"));
    let a4095 = f603(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4094, a4095)
}

pub fn f558(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f339(v0.clone(), {
        let a4096 = v1;
        let a4097 = 1u64;
        nat_add(a4096, a4097)?
    })?;
    let v4: Bytes = {
        let a4098 = v3;
        let a4099 = 117u64;
        if nat_eq(a4098, a4099) {
            f604(v0, v1)?
        } else {
            f605(v3)
        }
    };
    let a4100 = v4.clone();
    let a4101 = Bytes::lit(&[]);
    if equal(a4100, a4101) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a4102 = v3;
            let a4103 = 117u64;
            if nat_eq(a4102, a4103) {
                let a4104 = v1;
                let a4105 = 6u64;
                nat_add(a4104, a4105)?
            } else {
                let a4106 = v1;
                let a4107 = 2u64;
                nat_add(a4106, a4107)?
            }
        }, ({
            let a4108 = v2;
            let a4109 = v4;
            append_bytes(a4108, a4109)
        }, 0u64))))
    }
}

pub fn f559(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f339(v0, v1)?;
    let a4110 = 47u64;
    let a4111 = v2;
    if if nat_lt(a4110, a4111) {
        let a4112 = v2;
        let a4113 = 58u64;
        nat_lt(a4112, a4113)
    } else {
        false
    } {
        Ok(Some({
            let a4114 = v1;
            let a4115 = 1u64;
            nat_add(a4114, a4115)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f560(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a4116 = f339(v0, v1)?;
    let a4117 = 48u64;
    if nat_eq(a4116, a4117) {
        Ok(Some({
            let a4118 = v1;
            let a4119 = 1u64;
            nat_add(a4118, a4119)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f561(v0: Bytes) -> R<u64> {
    let (h4120, _) = f280(Fn3::F606(v0.clone()), {
        let a4121 = v0;
        length_bytes(a4121)
    }, 0u64)?;
    Ok(h4120)
}

pub fn f562(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m4125 = {
        let a4123 = v0;
        let a4124 = {
            let (h4122, _) = v1;
            h4122
        };
        index_bytes(a4123, a4124)
    };
    match m4125 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a4127 = {
                    let (h4126, _) = v1;
                    h4126
                };
                let a4128 = 1u64;
                nat_add(a4127, a4128)?
            }, {
                let a4132 = {
                    let a4130 = {
                        let (_, h4129) = v1;
                        h4129
                    };
                    let a4131 = 10u64;
                    nat_mul(a4130, a4131)?
                };
                let a4133 = f607(v2)?;
                nat_add(a4132, a4133)?
            })))
        }
    }
}

pub fn f563(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f608(v0, v1)
}

pub fn f564(v0: Bytes) -> R<Option<Bytes>> {
    if f216(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f265(f32(v0.clone(), 8u64, {
            let a4135 = {
                let a4134 = v0;
                length_bytes(a4134)
            };
            let a4136 = 8u64;
            nat_sub(a4135, a4136)
        }))?))
    } else {
        if f216(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f265(f32(v0.clone(), 14u64, {
                let a4138 = {
                    let a4137 = v0;
                    length_bytes(a4137)
                };
                let a4139 = 14u64;
                nat_sub(a4138, a4139)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f565(v0: List<Adt19>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f422(f423(f160(v0.clone()))?, v0, v1)
}

pub fn f566(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt25>), Bytes> {
    Err::<(Bytes, List<Adt25>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f567(v0: Bytes, v1: List<Adt25>) -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((v0, v1))
}

pub fn f568(v0: Bytes, v1: List<Adt19>) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4140 = v1.clone();
    match m4140.uncons() {
        None => {
            Ok(f566(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m4141 = f609(f436(f156(v2, f8(Str::lit("properties")))), v1.clone());
            match m4141.uncons() {
                None => {
                    let m4142 = f610(f423(f611(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m4142 {
                        Ok(v5) => {
                            Ok(f567(f424(v0, {
                                let (h4143, _) = v5;
                                h4143
                            }), {
                                let (_, h4144) = v5;
                                h4144
                            }))
                        }
                        Err(v4) => {
                            Ok(f566(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m4145 = f612(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m4145 {
                        Ok(v9) => {
                            Ok(f567(f424(v0, {
                                let (h4146, _) = v9;
                                h4146
                            }), {
                                let (_, h4147) = v9;
                                h4147
                            }))
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

pub fn f569(v0: Adt19, v1: Bytes) -> R<bool> {
    let m4148 = v0;
    match m4148 {
        Adt19::C0(_) => {
            Ok(false)
        }
        Adt19::C1(_, _) => {
            Ok(false)
        }
        Adt19::C2(_, _, _) => {
            Ok(false)
        }
        Adt19::C3(_) => {
            Ok(false)
        }
        Adt19::C4(r4149) => {
            let v2 = (*r4149).clone();
            f569(v2, v1)
        }
        Adt19::C5(r4150) => {
            let v3 = (*r4150).clone();
            f569(v3, v1)
        }
        Adt19::C6 => {
            Ok(false)
        }
        Adt19::C7(r4151, r4152) => {
            let v4 = (*r4151).clone();
            let v5 = (*r4152).clone();
            if f569(v4, v1.clone())? {
                Ok(true)
            } else {
                f569(v5, v1)
            }
        }
        Adt19::C8(v6, r4153, r4154) => {
            let v7 = (*r4153).clone();
            let v8 = (*r4154).clone();
            if if f37(v6, f8(Str::lit("$ref"))) {
                f613(v7.clone(), v1.clone())?
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

pub fn f570(v0: Bytes, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f614(f423(f615(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f571(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4155 = f616(v0.clone());
    match m4155 {
        None => {
            if f37(f159(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m4156 = f156(v0, f8(Str::lit("items")));
                match m4156 {
                    None => {
                        Ok(f617(f8(Str::lit("List")), f619()))
                    }
                    Some(v3) => {
                        Ok(f617(f8(Str::lit("List")), f618(v3, {
                            let a4157 = v1;
                            let a4158 = f8(Str::lit("Item"));
                            append_bytes(a4157, a4158)
                        })?))
                    }
                }
            } else {
                f618(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f617(f8(Str::lit("Option")), f618(v2, v1)?))
        }
    }
}

pub fn f572(p4161: List<Adt19>) -> bool {
    let m4160 = p4161;
    {
        let mut m4160 = m4160;
        loop {
            let step = {
                let v0 = m4160;
                let m4159 = v0;
                match m4159.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f620(f486(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt19>, bool>(v2)
                        } else {
                            Err::<List<Adt19>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4160 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(v0: Adt19, v1: Bytes) -> Adt19 {
    let m4162 = v0;
    match m4162 {
        Adt19::C0(v6) => {
            Adt19::C0(v6)
        }
        Adt19::C1(v10, v11) => {
            Adt19::C1(v10, v11)
        }
        Adt19::C2(v7, v8, v9) => {
            Adt19::C2(v7, v8, v9)
        }
        Adt19::C3(v12) => {
            Adt19::C3(v12)
        }
        Adt19::C4(r4163) => {
            let v13 = (*r4163).clone();
            Adt19::C4(std::rc::Rc::new(v13))
        }
        Adt19::C5(r4164) => {
            let v5 = (*r4164).clone();
            Adt19::C5(std::rc::Rc::new(f573(v5, v1)))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r4165, r4166) => {
            let v14 = (*r4165).clone();
            let v15 = (*r4166).clone();
            Adt19::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt19::C8(v2, r4167, r4168) => {
            let v3 = (*r4167).clone();
            let v4 = (*r4168).clone();
            if f37(v2.clone(), v1.clone()) {
                f573(v4, v1)
            } else {
                Adt19::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f573(v4, v1)))
            }
        }
    }
}

pub fn f574(v0: Adt19, v1: Bytes) -> bool {
    let m4169 = f156(v0, v1);
    match m4169 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f575(v0: List<Adt19>) -> bool {
    let m4170 = v0.clone();
    match m4170.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m4171 = f609(f436(f156(v1, f8(Str::lit("properties")))), v0.clone());
            match m4171.uncons() {
                None => {
                    f622(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f621(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f576(v0: Adt19, v1: Bytes, v2: Adt19) -> Adt19 {
    let m4172 = v0;
    match m4172 {
        Adt19::C0(v7) => {
            Adt19::C0(v7)
        }
        Adt19::C1(v11, v12) => {
            Adt19::C1(v11, v12)
        }
        Adt19::C2(v8, v9, v10) => {
            Adt19::C2(v8, v9, v10)
        }
        Adt19::C3(v13) => {
            Adt19::C3(v13)
        }
        Adt19::C4(r4173) => {
            let v14 = (*r4173).clone();
            Adt19::C4(std::rc::Rc::new(v14))
        }
        Adt19::C5(r4174) => {
            let v3 = (*r4174).clone();
            Adt19::C5(std::rc::Rc::new(Adt19::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r4175, r4176) => {
            let v15 = (*r4175).clone();
            let v16 = (*r4176).clone();
            Adt19::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt19::C8(v4, r4177, r4178) => {
            let v5 = (*r4177).clone();
            let v6 = (*r4178).clone();
            Adt19::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f577(p4182: List<Adt19>) -> bool {
    let m4181 = p4182;
    {
        let mut m4181 = m4181;
        loop {
            let step = {
                let v0 = m4181;
                let m4179 = v0;
                match m4179.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f620(f486(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f572(f270({
                                let m4180 = f156(v1, f8(Str::lit("anyOf")));
                                match m4180 {
                                    None => {
                                        Adt19::C6
                                    }
                                    Some(v3) => {
                                        v3
                                    }
                                }
                            })) {
                                Ok::<List<Adt19>, bool>(v2)
                            } else {
                                Err::<List<Adt19>, bool>(false)
                            }
                        } else {
                            Err::<List<Adt19>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4227 = {
            let a4203 = {
                let a4191 = {
                    let a4185 = {
                        let a4183 = v1;
                        let a4184 = 128u8;
                        fixed_u8::bit_and(a4183, a4184)
                    };
                    let a4186 = 128u8;
                    if equal(a4185, a4186) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4192 = {
                    let a4189 = {
                        let a4187 = v1;
                        let a4188 = 64u8;
                        fixed_u8::bit_and(a4187, a4188)
                    };
                    let a4190 = 64u8;
                    if equal(a4189, a4190) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4191, a4192)?
            };
            let a4204 = {
                let a4201 = {
                    let a4195 = {
                        let a4193 = v1;
                        let a4194 = 32u8;
                        fixed_u8::bit_and(a4193, a4194)
                    };
                    let a4196 = 32u8;
                    if equal(a4195, a4196) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4202 = {
                    let a4199 = {
                        let a4197 = v1;
                        let a4198 = 16u8;
                        fixed_u8::bit_and(a4197, a4198)
                    };
                    let a4200 = 16u8;
                    if equal(a4199, a4200) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4201, a4202)?
            };
            nat_add(a4203, a4204)?
        };
        let a4228 = {
            let a4225 = {
                let a4213 = {
                    let a4207 = {
                        let a4205 = v1;
                        let a4206 = 8u8;
                        fixed_u8::bit_and(a4205, a4206)
                    };
                    let a4208 = 8u8;
                    if equal(a4207, a4208) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4214 = {
                    let a4211 = {
                        let a4209 = v1;
                        let a4210 = 4u8;
                        fixed_u8::bit_and(a4209, a4210)
                    };
                    let a4212 = 4u8;
                    if equal(a4211, a4212) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4213, a4214)?
            };
            let a4226 = {
                let a4223 = {
                    let a4217 = {
                        let a4215 = v1;
                        let a4216 = 2u8;
                        fixed_u8::bit_and(a4215, a4216)
                    };
                    let a4218 = 2u8;
                    if equal(a4217, a4218) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4224 = {
                    let a4221 = {
                        let a4219 = v1;
                        let a4220 = 1u8;
                        fixed_u8::bit_and(a4219, a4220)
                    };
                    let a4222 = 1u8;
                    if equal(a4221, a4222) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4223, a4224)?
            };
            nat_add(a4225, a4226)?
        };
        nat_add(a4227, a4228)?
    };
    let a4235 = f343(f623(), {
        let a4229 = v2;
        let a4230 = 16u64;
        let a4231 = 0u64;
        nat_quot(a4229, a4230, a4231)
    });
    let a4236 = f343(f623(), {
        let a4232 = v2;
        let a4233 = 16u64;
        let a4234 = 0u64;
        nat_rem(a4232, a4233, a4234)
    });
    Ok(append_bytes(a4235, a4236))
}

pub fn f579(v0: Fn21, p4240: List<List<(Bytes, Bytes)>>, p4241: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4239 = (p4240, p4241);
    {
        let mut m4239 = m4239;
        loop {
            let step = {
                let (v1, v2) = m4239;
                let m4237 = v2;
                match m4237.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4238: Fn21 = v0.clone();
                            c4238.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f580(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f581(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4243 = {
        let a4242 = v0.clone();
        length_list(a4242)
    };
    let a4244 = 1u64;
    if nat_le(a4243, a4244) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f624(v0)?))
    }
}

pub fn f582(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h4245, _) = v0;
        h4245
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h4246) = v0;
        h4246
    };
    let m4247 = v1;
    match m4247.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m4248 = v4;
            match m4248.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f625(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f583(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4249 = f400(v0, v1);
    let a4250 = f551(v0, v2);
    fixed_u32::bit_or(a4249, a4250)
}

pub fn f584(v0: u32) -> u32 {
    let a4253 = {
        let a4251 = f583(v0, 17u32, 15u32);
        let a4252 = f583(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4251, a4252)
    };
    let a4254 = f400(v0, 10u32);
    fixed_u32::bit_xor(a4253, a4254)
}

pub fn f585(v0: u32) -> u32 {
    let a4257 = {
        let a4255 = f583(v0, 7u32, 25u32);
        let a4256 = f583(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4255, a4256)
    };
    let a4258 = f400(v0, 3u32);
    fixed_u32::bit_xor(a4257, a4258)
}

pub fn f586() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x00]), List::cons(Bytes::lit(&[0x01]), List::cons(Bytes::lit(&[0x02]), List::cons(Bytes::lit(&[0x03]), List::cons(Bytes::lit(&[0x04]), List::cons(Bytes::lit(&[0x05]), List::cons(Bytes::lit(&[0x06]), List::cons(Bytes::lit(&[0x07]), List::cons(Bytes::lit(&[0x08]), List::cons(Bytes::lit(&[0x09]), List::cons(Bytes::lit(&[0x0a]), List::cons(Bytes::lit(&[0x0b]), List::cons(Bytes::lit(&[0x0c]), List::cons(Bytes::lit(&[0x0d]), List::cons(Bytes::lit(&[0x0e]), List::cons(Bytes::lit(&[0x0f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f587() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x10]), List::cons(Bytes::lit(&[0x11]), List::cons(Bytes::lit(&[0x12]), List::cons(Bytes::lit(&[0x13]), List::cons(Bytes::lit(&[0x14]), List::cons(Bytes::lit(&[0x15]), List::cons(Bytes::lit(&[0x16]), List::cons(Bytes::lit(&[0x17]), List::cons(Bytes::lit(&[0x18]), List::cons(Bytes::lit(&[0x19]), List::cons(Bytes::lit(&[0x1a]), List::cons(Bytes::lit(&[0x1b]), List::cons(Bytes::lit(&[0x1c]), List::cons(Bytes::lit(&[0x1d]), List::cons(Bytes::lit(&[0x1e]), List::cons(Bytes::lit(&[0x1f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f588() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x20]), List::cons(Bytes::lit(&[0x21]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x27]), List::cons(Bytes::lit(&[0x28]), List::cons(Bytes::lit(&[0x29]), List::cons(Bytes::lit(&[0x2a]), List::cons(Bytes::lit(&[0x2b]), List::cons(Bytes::lit(&[0x2c]), List::cons(Bytes::lit(&[0x2d]), List::cons(Bytes::lit(&[0x2e]), List::cons(Bytes::lit(&[0x2f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f589() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x30]), List::cons(Bytes::lit(&[0x31]), List::cons(Bytes::lit(&[0x32]), List::cons(Bytes::lit(&[0x33]), List::cons(Bytes::lit(&[0x34]), List::cons(Bytes::lit(&[0x35]), List::cons(Bytes::lit(&[0x36]), List::cons(Bytes::lit(&[0x37]), List::cons(Bytes::lit(&[0x38]), List::cons(Bytes::lit(&[0x39]), List::cons(Bytes::lit(&[0x3a]), List::cons(Bytes::lit(&[0x3b]), List::cons(Bytes::lit(&[0x3c]), List::cons(Bytes::lit(&[0x3d]), List::cons(Bytes::lit(&[0x3e]), List::cons(Bytes::lit(&[0x3f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f590() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x40]), List::cons(Bytes::lit(&[0x41]), List::cons(Bytes::lit(&[0x42]), List::cons(Bytes::lit(&[0x43]), List::cons(Bytes::lit(&[0x44]), List::cons(Bytes::lit(&[0x45]), List::cons(Bytes::lit(&[0x46]), List::cons(Bytes::lit(&[0x47]), List::cons(Bytes::lit(&[0x48]), List::cons(Bytes::lit(&[0x49]), List::cons(Bytes::lit(&[0x4a]), List::cons(Bytes::lit(&[0x4b]), List::cons(Bytes::lit(&[0x4c]), List::cons(Bytes::lit(&[0x4d]), List::cons(Bytes::lit(&[0x4e]), List::cons(Bytes::lit(&[0x4f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f591() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x50]), List::cons(Bytes::lit(&[0x51]), List::cons(Bytes::lit(&[0x52]), List::cons(Bytes::lit(&[0x53]), List::cons(Bytes::lit(&[0x54]), List::cons(Bytes::lit(&[0x55]), List::cons(Bytes::lit(&[0x56]), List::cons(Bytes::lit(&[0x57]), List::cons(Bytes::lit(&[0x58]), List::cons(Bytes::lit(&[0x59]), List::cons(Bytes::lit(&[0x5a]), List::cons(Bytes::lit(&[0x5b]), List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x5d]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f592() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x60]), List::cons(Bytes::lit(&[0x61]), List::cons(Bytes::lit(&[0x62]), List::cons(Bytes::lit(&[0x63]), List::cons(Bytes::lit(&[0x64]), List::cons(Bytes::lit(&[0x65]), List::cons(Bytes::lit(&[0x66]), List::cons(Bytes::lit(&[0x67]), List::cons(Bytes::lit(&[0x68]), List::cons(Bytes::lit(&[0x69]), List::cons(Bytes::lit(&[0x6a]), List::cons(Bytes::lit(&[0x6b]), List::cons(Bytes::lit(&[0x6c]), List::cons(Bytes::lit(&[0x6d]), List::cons(Bytes::lit(&[0x6e]), List::cons(Bytes::lit(&[0x6f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f593() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x70]), List::cons(Bytes::lit(&[0x71]), List::cons(Bytes::lit(&[0x72]), List::cons(Bytes::lit(&[0x73]), List::cons(Bytes::lit(&[0x74]), List::cons(Bytes::lit(&[0x75]), List::cons(Bytes::lit(&[0x76]), List::cons(Bytes::lit(&[0x77]), List::cons(Bytes::lit(&[0x78]), List::cons(Bytes::lit(&[0x79]), List::cons(Bytes::lit(&[0x7a]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7c]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x7f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f594() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x80]), List::cons(Bytes::lit(&[0x81]), List::cons(Bytes::lit(&[0x82]), List::cons(Bytes::lit(&[0x83]), List::cons(Bytes::lit(&[0x84]), List::cons(Bytes::lit(&[0x85]), List::cons(Bytes::lit(&[0x86]), List::cons(Bytes::lit(&[0x87]), List::cons(Bytes::lit(&[0x88]), List::cons(Bytes::lit(&[0x89]), List::cons(Bytes::lit(&[0x8a]), List::cons(Bytes::lit(&[0x8b]), List::cons(Bytes::lit(&[0x8c]), List::cons(Bytes::lit(&[0x8d]), List::cons(Bytes::lit(&[0x8e]), List::cons(Bytes::lit(&[0x8f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f595() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x90]), List::cons(Bytes::lit(&[0x91]), List::cons(Bytes::lit(&[0x92]), List::cons(Bytes::lit(&[0x93]), List::cons(Bytes::lit(&[0x94]), List::cons(Bytes::lit(&[0x95]), List::cons(Bytes::lit(&[0x96]), List::cons(Bytes::lit(&[0x97]), List::cons(Bytes::lit(&[0x98]), List::cons(Bytes::lit(&[0x99]), List::cons(Bytes::lit(&[0x9a]), List::cons(Bytes::lit(&[0x9b]), List::cons(Bytes::lit(&[0x9c]), List::cons(Bytes::lit(&[0x9d]), List::cons(Bytes::lit(&[0x9e]), List::cons(Bytes::lit(&[0x9f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f596() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xa0]), List::cons(Bytes::lit(&[0xa1]), List::cons(Bytes::lit(&[0xa2]), List::cons(Bytes::lit(&[0xa3]), List::cons(Bytes::lit(&[0xa4]), List::cons(Bytes::lit(&[0xa5]), List::cons(Bytes::lit(&[0xa6]), List::cons(Bytes::lit(&[0xa7]), List::cons(Bytes::lit(&[0xa8]), List::cons(Bytes::lit(&[0xa9]), List::cons(Bytes::lit(&[0xaa]), List::cons(Bytes::lit(&[0xab]), List::cons(Bytes::lit(&[0xac]), List::cons(Bytes::lit(&[0xad]), List::cons(Bytes::lit(&[0xae]), List::cons(Bytes::lit(&[0xaf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f597() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xb0]), List::cons(Bytes::lit(&[0xb1]), List::cons(Bytes::lit(&[0xb2]), List::cons(Bytes::lit(&[0xb3]), List::cons(Bytes::lit(&[0xb4]), List::cons(Bytes::lit(&[0xb5]), List::cons(Bytes::lit(&[0xb6]), List::cons(Bytes::lit(&[0xb7]), List::cons(Bytes::lit(&[0xb8]), List::cons(Bytes::lit(&[0xb9]), List::cons(Bytes::lit(&[0xba]), List::cons(Bytes::lit(&[0xbb]), List::cons(Bytes::lit(&[0xbc]), List::cons(Bytes::lit(&[0xbd]), List::cons(Bytes::lit(&[0xbe]), List::cons(Bytes::lit(&[0xbf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f598() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xc0]), List::cons(Bytes::lit(&[0xc1]), List::cons(Bytes::lit(&[0xc2]), List::cons(Bytes::lit(&[0xc3]), List::cons(Bytes::lit(&[0xc4]), List::cons(Bytes::lit(&[0xc5]), List::cons(Bytes::lit(&[0xc6]), List::cons(Bytes::lit(&[0xc7]), List::cons(Bytes::lit(&[0xc8]), List::cons(Bytes::lit(&[0xc9]), List::cons(Bytes::lit(&[0xca]), List::cons(Bytes::lit(&[0xcb]), List::cons(Bytes::lit(&[0xcc]), List::cons(Bytes::lit(&[0xcd]), List::cons(Bytes::lit(&[0xce]), List::cons(Bytes::lit(&[0xcf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f599() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xd0]), List::cons(Bytes::lit(&[0xd1]), List::cons(Bytes::lit(&[0xd2]), List::cons(Bytes::lit(&[0xd3]), List::cons(Bytes::lit(&[0xd4]), List::cons(Bytes::lit(&[0xd5]), List::cons(Bytes::lit(&[0xd6]), List::cons(Bytes::lit(&[0xd7]), List::cons(Bytes::lit(&[0xd8]), List::cons(Bytes::lit(&[0xd9]), List::cons(Bytes::lit(&[0xda]), List::cons(Bytes::lit(&[0xdb]), List::cons(Bytes::lit(&[0xdc]), List::cons(Bytes::lit(&[0xdd]), List::cons(Bytes::lit(&[0xde]), List::cons(Bytes::lit(&[0xdf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f600() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xe0]), List::cons(Bytes::lit(&[0xe1]), List::cons(Bytes::lit(&[0xe2]), List::cons(Bytes::lit(&[0xe3]), List::cons(Bytes::lit(&[0xe4]), List::cons(Bytes::lit(&[0xe5]), List::cons(Bytes::lit(&[0xe6]), List::cons(Bytes::lit(&[0xe7]), List::cons(Bytes::lit(&[0xe8]), List::cons(Bytes::lit(&[0xe9]), List::cons(Bytes::lit(&[0xea]), List::cons(Bytes::lit(&[0xeb]), List::cons(Bytes::lit(&[0xec]), List::cons(Bytes::lit(&[0xed]), List::cons(Bytes::lit(&[0xee]), List::cons(Bytes::lit(&[0xef]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f601() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xf0]), List::cons(Bytes::lit(&[0xf1]), List::cons(Bytes::lit(&[0xf2]), List::cons(Bytes::lit(&[0xf3]), List::cons(Bytes::lit(&[0xf4]), List::cons(Bytes::lit(&[0xf5]), List::cons(Bytes::lit(&[0xf6]), List::cons(Bytes::lit(&[0xf7]), List::cons(Bytes::lit(&[0xf8]), List::cons(Bytes::lit(&[0xf9]), List::cons(Bytes::lit(&[0xfa]), List::cons(Bytes::lit(&[0xfb]), List::cons(Bytes::lit(&[0xfc]), List::cons(Bytes::lit(&[0xfd]), List::cons(Bytes::lit(&[0xfe]), List::cons(Bytes::lit(&[0xff]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f602(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f432(v2) {
        true
    } else {
        f433(v2)
    } {
        let a4259 = v2;
        let a4260 = 117u64;
        if !nat_eq(a4259, a4260) {
            let a4261 = v2;
            let a4262 = 85u64;
            !nat_eq(a4261, a4262)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f434(v2) {
            let a4263 = 0u64;
            let a4264 = v1;
            nat_lt(a4263, a4264)
        } else {
            false
        }
    } {
        f32(v0, v1, 1u64)
    } else {
        let a4265 = v2;
        let a4266 = 65536u64;
        if nat_lt(a4265, a4266) {
            let a4267 = f8(Str::lit("u"));
            let a4268 = f603(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4267, a4268)
        } else {
            let a4269 = f8(Str::lit("U"));
            let a4270 = f603(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4269, a4270)
        }
    }
}

pub fn f603(p4287: u64, p4288: u64, p4289: u64, p4290: Bytes) -> Bytes {
    let m4286 = (p4287, (p4288, (p4289, p4290)));
    {
        let mut m4286 = m4286;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m4286;
                let m4271 = v0;
                if m4271 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m4271 - 1;
                    let a4272 = v1;
                    let a4273 = 0u64;
                    if if nat_eq(a4272, a4273) {
                        let a4274 = v2;
                        let a4275 = 0u64;
                        nat_eq(a4274, a4275)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a4276 = v1;
                            let a4277 = 16u64;
                            let a4278 = 0u64;
                            nat_quot(a4276, a4277, a4278)
                        }, ({
                            let a4279 = v2;
                            let a4280 = 1u64;
                            nat_sub(a4279, a4280)
                        }, {
                            let a4284 = f626({
                                let a4281 = v1;
                                let a4282 = 16u64;
                                let a4283 = 0u64;
                                nat_rem(a4281, a4282, a4283)
                            });
                            let a4285 = v3;
                            append_bytes(a4284, a4285)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m4286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: Bytes, v1: u64) -> R<Bytes> {
    let a4293 = f32(v0.clone(), {
        let a4291 = v1;
        let a4292 = 2u64;
        nat_add(a4291, a4292)?
    }, 2u64);
    let a4294 = Bytes::lit(&[0x30, 0x30]);
    if equal(a4293, a4294) {
        let m4297 = f627(f32(v0, {
            let a4295 = v1;
            let a4296 = 4u64;
            nat_add(a4295, a4296)?
        }, 2u64))?;
        match m4297 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f339(v2.clone(), 0u64)?;
                let a4298 = v3;
                let a4299 = 32u64;
                if nat_lt(a4298, a4299) {
                    if f628(v3) {
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

pub fn f605(v0: u64) -> Bytes {
    let a4300 = v0;
    let a4301 = 34u64;
    if nat_eq(a4300, a4301) {
        Bytes::lit(&[0x22])
    } else {
        let a4302 = v0;
        let a4303 = 92u64;
        if nat_eq(a4302, a4303) {
            Bytes::lit(&[0x5c])
        } else {
            let a4304 = v0;
            let a4305 = 98u64;
            if nat_eq(a4304, a4305) {
                Bytes::lit(&[0x08])
            } else {
                let a4306 = v0;
                let a4307 = 102u64;
                if nat_eq(a4306, a4307) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a4308 = v0;
                    let a4309 = 110u64;
                    if nat_eq(a4308, a4309) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a4310 = v0;
                        let a4311 = 114u64;
                        if nat_eq(a4310, a4311) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a4312 = v0;
                            let a4313 = 116u64;
                            if nat_eq(a4312, a4313) {
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

pub fn f606(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m4316 = {
        let a4314 = v0;
        let a4315 = v1;
        index_bytes(a4314, a4315)
    };
    match m4316 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f629(v2)? {
                Ok(Some({
                    let a4317 = v1;
                    let a4318 = 1u64;
                    nat_add(a4317, a4318)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f607(v0: u8) -> R<u64> {
    let a4365 = {
        let v1: u8 = v0;
        let a4363 = {
            let a4339 = {
                let a4327 = {
                    let a4321 = {
                        let a4319 = v1;
                        let a4320 = 128u8;
                        fixed_u8::bit_and(a4319, a4320)
                    };
                    let a4322 = 128u8;
                    if equal(a4321, a4322) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4328 = {
                    let a4325 = {
                        let a4323 = v1;
                        let a4324 = 64u8;
                        fixed_u8::bit_and(a4323, a4324)
                    };
                    let a4326 = 64u8;
                    if equal(a4325, a4326) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4327, a4328)?
            };
            let a4340 = {
                let a4337 = {
                    let a4331 = {
                        let a4329 = v1;
                        let a4330 = 32u8;
                        fixed_u8::bit_and(a4329, a4330)
                    };
                    let a4332 = 32u8;
                    if equal(a4331, a4332) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4338 = {
                    let a4335 = {
                        let a4333 = v1;
                        let a4334 = 16u8;
                        fixed_u8::bit_and(a4333, a4334)
                    };
                    let a4336 = 16u8;
                    if equal(a4335, a4336) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4337, a4338)?
            };
            nat_add(a4339, a4340)?
        };
        let a4364 = {
            let a4361 = {
                let a4349 = {
                    let a4343 = {
                        let a4341 = v1;
                        let a4342 = 8u8;
                        fixed_u8::bit_and(a4341, a4342)
                    };
                    let a4344 = 8u8;
                    if equal(a4343, a4344) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4350 = {
                    let a4347 = {
                        let a4345 = v1;
                        let a4346 = 4u8;
                        fixed_u8::bit_and(a4345, a4346)
                    };
                    let a4348 = 4u8;
                    if equal(a4347, a4348) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4349, a4350)?
            };
            let a4362 = {
                let a4359 = {
                    let a4353 = {
                        let a4351 = v1;
                        let a4352 = 2u8;
                        fixed_u8::bit_and(a4351, a4352)
                    };
                    let a4354 = 2u8;
                    if equal(a4353, a4354) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4360 = {
                    let a4357 = {
                        let a4355 = v1;
                        let a4356 = 1u8;
                        fixed_u8::bit_and(a4355, a4356)
                    };
                    let a4358 = 1u8;
                    if equal(a4357, a4358) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4359, a4360)?
            };
            nat_add(a4361, a4362)?
        };
        nat_add(a4363, a4364)?
    };
    let a4366 = 48u64;
    Ok(nat_sub(a4365, a4366))
}

pub fn f608(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a4367 = v0.clone();
        length_bytes(a4367)
    };
    let a4368 = v1;
    let a4369 = v2;
    if if nat_lt(a4368, a4369) {
        let a4374 = f339(v0, {
            let a4372 = v2;
            let a4373 = {
                let a4370 = v1;
                let a4371 = 1u64;
                nat_add(a4370, a4371)?
            };
            nat_sub(a4372, a4373)
        })?;
        let a4375 = 48u64;
        nat_eq(a4374, a4375)
    } else {
        false
    } {
        Ok(Some({
            let a4376 = v1;
            let a4377 = 1u64;
            nat_add(a4376, a4377)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f609(v0: List<(Bytes, Adt19)>, v1: List<Adt19>) -> List<Bytes> {
    let m4378 = v0;
    match m4378.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f630(v1.clone(), {
                let (h4379, _) = v2.clone();
                h4379
            }) {
                List::cons({
                    let (h4380, _) = v2;
                    h4380
                }, f609(v3, v1))
            } else {
                f609(v3, v1)
            }
        }
    }
}

pub fn f610(v0: bool, v1: Bytes, v2: List<Adt19>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4381 = v2;
    match m4381.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m4382 = f631(v4.clone())?;
            match m4382 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m4383 = f519(v0, v6.clone())?;
                    match m4383 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f90(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4386 = f632(v4, {
                                    let a4384 = v1.clone();
                                    let a4385 = f265(v6)?;
                                    append_bytes(a4384, a4385)
                                })?;
                                match m4386 {
                                    Ok(v9) => {
                                        let m4387 = f610(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m4387 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h4388, _) = v9;
                                                    h4388
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4389, _) = v11;
                                                    h4389
                                                }), f440({
                                                    let (_, h4390) = v9;
                                                    h4390
                                                }, {
                                                    let (_, h4391) = v11;
                                                    h4391
                                                }))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v8))
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

pub fn f611(v0: List<Adt19>) -> R<List<Bytes>> {
    let m4392 = v0;
    match m4392.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m4393 = f631(v1)?;
            match m4393 {
                None => {
                    f611(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f611(v2)?))
                }
            }
        }
    }
}

pub fn f612(v0: Bytes, v1: List<Adt19>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f633(f423(f634(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f613(v0: Adt19, v1: Bytes) -> R<bool> {
    let m4394 = f269(Some(v0));
    match m4394 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m4395 = f564(v2)?;
            match m4395 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    Ok(f37(v3, v1))
                }
            }
        }
    }
}

pub fn f614(v0: bool, v1: Bytes, v2: List<(Bytes, Adt19)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4396 = v2;
    match m4396.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m4398 = f519(v0, {
                let (h4397, _) = v5.clone();
                h4397
            })?;
            match m4398 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h4399, _) = v5;
                        h4399
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f90(v7.clone(), f477(v0, f615(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m4404 = f571({
                            let (_, h4400) = v5.clone();
                            h4400
                        }, {
                            let a4402 = v1.clone();
                            let a4403 = f265({
                                let (h4401, _) = v5.clone();
                                h4401
                            })?;
                            append_bytes(a4402, a4403)
                        })?;
                        match m4404 {
                            Ok(v9) => {
                                let v10: Bytes = if f90({
                                    let (h4405, _) = v5;
                                    h4405
                                }, v3.clone()) {
                                    let (h4406, _) = v9;
                                    h4406
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h4407, _) = v9;
                                        h4407
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m4408 = f614(v0, v1, v6, v3, v4)?;
                                match m4408 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h4409, _) = v12;
                                            h4409
                                        }), f440({
                                            let (_, h4410) = v9;
                                            h4410
                                        }, {
                                            let (_, h4411) = v12;
                                            h4411
                                        }))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f615(v0: List<(Bytes, Adt19)>) -> List<Bytes> {
    let m4412 = v0;
    match m4412.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h4413, _) = v1;
                h4413
            }, f615(v2))
        }
    }
}

pub fn f616(v0: Adt19) -> Option<Adt19> {
    let m4414 = f161(f156(v0, f8(Str::lit("oneOf"))));
    match m4414.uncons() {
        None => {
            None::<Adt19>
        }
        Some((v1, v2)) => {
            let m4415 = v2;
            match m4415.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v3, v4)) => {
                    let m4416 = v4;
                    match m4416.uncons() {
                        None => {
                            if f635(v3.clone()) {
                                Some(v1)
                            } else {
                                if f635(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt19>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt19>
                        }
                    }
                }
            }
        }
    }
}

pub fn f617(v0: Bytes, v1: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4417 = v1;
    match m4417 {
        Ok(v3) => {
            f567(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h4418, _) = v3;
                h4418
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h4419) = v3;
                h4419
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v2)
        }
    }
}

pub fn f618(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4420 = f616(v0.clone());
    match m4420 {
        None => {
            if f37(f159(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m4421 = f156(v0, f8(Str::lit("items")));
                match m4421 {
                    None => {
                        Ok(f617(f8(Str::lit("List")), f619()))
                    }
                    Some(v3) => {
                        Ok(f617(f8(Str::lit("List")), f632(v3, {
                            let a4422 = v1;
                            let a4423 = f8(Str::lit("Item"));
                            append_bytes(a4422, a4423)
                        })?))
                    }
                }
            } else {
                f632(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f617(f8(Str::lit("Option")), f632(v2, v1)?))
        }
    }
}

pub fn f619() -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((f489(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f620(p4427: List<(Bytes, Adt19)>, v1: List<Bytes>) -> bool {
    let m4426 = p4427;
    {
        let mut m4426 = m4426;
        loop {
            let step = {
                let v0 = m4426;
                let m4424 = v0;
                match m4424.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f90({
                            let (h4425, _) = v2;
                            h4425
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt19)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt19)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(p4431: List<Adt19>, v1: Bytes, p4432: List<Bytes>) -> bool {
    let m4430 = (p4431, p4432);
    {
        let mut m4430 = m4430;
        loop {
            let step = {
                let (v0, v2) = m4430;
                let m4428 = v0;
                match m4428.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m4429 = f636(v3, v1.clone());
                        match m4429 {
                            None => {
                                Err::<(List<Adt19>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f90(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt19>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt19>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4430 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(p4435: List<Adt19>, p4436: List<Bytes>) -> bool {
    let m4434 = (p4435, p4436);
    {
        let mut m4434 = m4434;
        loop {
            let step = {
                let (v0, v1) = m4434;
                let m4433 = v0;
                match m4433.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f159(v2, f8(Str::lit("type")));
                        if !f37(v4.clone(), Bytes::lit(&[])) {
                            if !f90(v4.clone(), v1.clone()) {
                                if !if if f37(v4.clone(), f8(Str::lit("integer"))) {
                                    f90(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f37(v4.clone(), f8(Str::lit("number"))) {
                                        f90(f8(Str::lit("integer")), v1.clone())
                                    } else {
                                        false
                                    }
                                } {
                                    Ok::<(List<Adt19>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt19>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt19>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt19>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f623() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f624(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f536({
        let (_, h4437) = {
            let (h4438, _) = f637(Fn22::F638, {
                let a4440 = {
                    let a4439 = v0.clone();
                    length_list(a4439)
                };
                let a4441 = 1u64;
                nat_add(a4440, a4441)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4438
        };
        h4437
    }))
}

pub fn f625(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a4446 = {
            let a4444 = {
                let a4442 = v0.clone();
                length_list(a4442)
            };
            let a4445 = {
                let a4443 = v1.clone();
                length_list(a4443)
            };
            nat_add(a4444, a4445)?
        };
        let a4447 = 1u64;
        nat_add(a4446, a4447)?
    };
    Ok(f390({
        let (_, h4448) = {
            let (_, h4449) = {
                let (h4450, _) = f639(Fn23::F640, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h4450
            };
            h4449
        };
        h4448
    }))
}

pub fn f626(v0: u64) -> Bytes {
    f32({
        let a4451 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4451)
    }, v0, 1u64)
}

pub fn f627(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a4452 = v0.clone();
        length_bytes(a4452)
    };
    let a4456 = {
        let a4453 = v1;
        let a4454 = 2u64;
        let a4455 = 0u64;
        nat_rem(a4453, a4454, a4455)
    };
    let a4457 = 0u64;
    if nat_eq(a4456, a4457) {
        let (_, h4458) = {
            let (h4459, _) = f641(Fn24::F642(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h4459
        };
        Ok(h4458)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f628(v0: u64) -> bool {
    let a4460 = v0;
    let a4461 = 8u64;
    if nat_eq(a4460, a4461) {
        true
    } else {
        let a4462 = v0;
        let a4463 = 9u64;
        if nat_eq(a4462, a4463) {
            true
        } else {
            let a4464 = v0;
            let a4465 = 10u64;
            if nat_eq(a4464, a4465) {
                true
            } else {
                let a4466 = v0;
                let a4467 = 12u64;
                if nat_eq(a4466, a4467) {
                    true
                } else {
                    let a4468 = v0;
                    let a4469 = 13u64;
                    nat_eq(a4468, a4469)
                }
            }
        }
    }
}

pub fn f629(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4514 = {
            let a4490 = {
                let a4478 = {
                    let a4472 = {
                        let a4470 = v1;
                        let a4471 = 128u8;
                        fixed_u8::bit_and(a4470, a4471)
                    };
                    let a4473 = 128u8;
                    if equal(a4472, a4473) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4479 = {
                    let a4476 = {
                        let a4474 = v1;
                        let a4475 = 64u8;
                        fixed_u8::bit_and(a4474, a4475)
                    };
                    let a4477 = 64u8;
                    if equal(a4476, a4477) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4478, a4479)?
            };
            let a4491 = {
                let a4488 = {
                    let a4482 = {
                        let a4480 = v1;
                        let a4481 = 32u8;
                        fixed_u8::bit_and(a4480, a4481)
                    };
                    let a4483 = 32u8;
                    if equal(a4482, a4483) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4489 = {
                    let a4486 = {
                        let a4484 = v1;
                        let a4485 = 16u8;
                        fixed_u8::bit_and(a4484, a4485)
                    };
                    let a4487 = 16u8;
                    if equal(a4486, a4487) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4488, a4489)?
            };
            nat_add(a4490, a4491)?
        };
        let a4515 = {
            let a4512 = {
                let a4500 = {
                    let a4494 = {
                        let a4492 = v1;
                        let a4493 = 8u8;
                        fixed_u8::bit_and(a4492, a4493)
                    };
                    let a4495 = 8u8;
                    if equal(a4494, a4495) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4501 = {
                    let a4498 = {
                        let a4496 = v1;
                        let a4497 = 4u8;
                        fixed_u8::bit_and(a4496, a4497)
                    };
                    let a4499 = 4u8;
                    if equal(a4498, a4499) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4500, a4501)?
            };
            let a4513 = {
                let a4510 = {
                    let a4504 = {
                        let a4502 = v1;
                        let a4503 = 2u8;
                        fixed_u8::bit_and(a4502, a4503)
                    };
                    let a4505 = 2u8;
                    if equal(a4504, a4505) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4511 = {
                    let a4508 = {
                        let a4506 = v1;
                        let a4507 = 1u8;
                        fixed_u8::bit_and(a4506, a4507)
                    };
                    let a4509 = 1u8;
                    if equal(a4508, a4509) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4510, a4511)?
            };
            nat_add(a4512, a4513)?
        };
        nat_add(a4514, a4515)?
    };
    let a4516 = 48u64;
    let a4517 = v2;
    if nat_le(a4516, a4517) {
        let a4518 = v2;
        let a4519 = 57u64;
        Ok(nat_le(a4518, a4519))
    } else {
        Ok(false)
    }
}

pub fn f630(p4523: List<Adt19>, v1: Bytes) -> bool {
    let m4522 = p4523;
    {
        let mut m4522 = m4522;
        loop {
            let step = {
                let v0 = m4522;
                let m4520 = v0;
                match m4520.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m4521 = f636(v2, v1.clone());
                        if match m4521 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt19>, bool>(v3)
                        } else {
                            Err::<List<Adt19>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4522 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f631(v0: Adt19) -> R<Option<Bytes>> {
    let m4524 = f269(f156(v0.clone(), f8(Str::lit("$ref"))));
    match m4524 {
        None => {
            Ok(f269(f156(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f564(v1)
        }
    }
}

pub fn f632(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f643(f534(v0), v1)
}

pub fn f633(v0: bool, v1: Bytes, v2: List<Adt19>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4525 = v2;
    match m4525.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m4526 = f636(v5.clone(), v3.clone());
            match m4526 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m4527 = f519(v0, v7.clone())?;
                    match m4527 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f90(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4530 = f570({
                                    let a4528 = v1.clone();
                                    let a4529 = f265(v7)?;
                                    append_bytes(a4528, a4529)
                                }, f488(f436(f156(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f160(f161(f156(v5, f8(Str::lit("required"))))), true)?;
                                match m4530 {
                                    Ok(v10) => {
                                        let m4531 = f633(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m4531 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f226({
                                                    let (h4532, _) = v10;
                                                    h4532
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4533, _) = v12;
                                                    h4533
                                                }), f440({
                                                    let (_, h4534) = v10;
                                                    h4534
                                                }, {
                                                    let (_, h4535) = v12;
                                                    h4535
                                                }))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v9))
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

pub fn f634(v0: List<Adt19>, v1: Bytes) -> List<Bytes> {
    let m4536 = v0;
    match m4536.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4537 = f636(v2, v1.clone());
            match m4537 {
                None => {
                    f634(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f634(v3, v1))
                }
            }
        }
    }
}

pub fn f635(v0: Adt19) -> bool {
    f37(f159(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f636(v0: Adt19, v1: Bytes) -> Option<Bytes> {
    if f90(v1.clone(), f160(f161(f156(v0.clone(), f8(Str::lit("required")))))) {
        let m4538 = f156(v0, f8(Str::lit("properties")));
        match m4538 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m4539 = f156(v2, v1);
                match m4539 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f269(f156(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f637(v0: Fn22, p4544: u64, p4545: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4543 = (p4544, p4545);
    {
        let mut m4543 = m4543;
        loop {
            let step = {
                let (v1, v2) = m4543;
                let m4540 = v1;
                if m4540 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4540 - 1;
                    let m4542 = {
                        let c4541: Fn22 = v0.clone();
                        c4541.apply(v2.clone())?
                    };
                    match m4542 {
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
                Ok(next) => { m4543 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f638(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f644(v0)
}

pub fn f639(v0: Fn23, p4550: u64, p4551: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4549 = (p4550, p4551);
    {
        let mut m4549 = m4549;
        loop {
            let step = {
                let (v1, v2) = m4549;
                let m4546 = v1;
                if m4546 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m4546 - 1;
                    let m4548 = {
                        let c4547: Fn23 = v0.clone();
                        c4547.apply(v2.clone())
                    };
                    match m4548 {
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
                Ok(next) => { m4549 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f645(v0)
}

pub fn f641(v0: Fn24, p4556: u64, p4557: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4555 = (p4556, p4557);
    {
        let mut m4555 = m4555;
        loop {
            let step = {
                let (v1, v2) = m4555;
                let m4552 = v1;
                if m4552 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4552 - 1;
                    let m4554 = {
                        let c4553: Fn24 = v0.clone();
                        c4553.apply(v2.clone())?
                    };
                    match m4554 {
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
                Ok(next) => { m4555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f642(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f646(v0, v1)
}

pub fn f643(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4558 = v0.clone();
    match m4558 {
        Adt19::C0(v2) => {
            if v2 {
                Ok(f619())
            } else {
                Ok(f566(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt19::C1(_, _) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C2(_, _, _) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C3(_) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C4(_) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C5(_) => {
            let m4559 = f647(f486(v0.clone()));
            match m4559 {
                None => {
                    let m4560 = f269(f156(v0.clone(), f8(Str::lit("$ref"))));
                    match m4560 {
                        None => {
                            let m4562 = f156(v0.clone(), f8(Str::lit("allOf")));
                            match m4562 {
                                None => {
                                    let m4567 = f156(v0.clone(), f8(Str::lit("enum")));
                                    match m4567 {
                                        None => {
                                            let m4568 = f156(v0.clone(), f8(Str::lit("const")));
                                            match m4568 {
                                                None => {
                                                    let m4570 = f156(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m4570 {
                                                        None => {
                                                            let v30: Bytes = f159(v0.clone(), f8(Str::lit("type")));
                                                            if if f37(v30.clone(), Bytes::lit(&[])) {
                                                                !f648(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f619())
                                                            } else {
                                                                if f37(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f567(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f37(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f567(f649(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f37(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f567(f517(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f37(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f567(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f37(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m4571 = f436(f156(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m4571.uncons() {
                                                                                        None => {
                                                                                            let m4572 = f650(v0);
                                                                                            match m4572 {
                                                                                                None => {
                                                                                                    Ok(f617(f8(Str::lit("List")), f651(f619())))
                                                                                                }
                                                                                                Some(v33) => {
                                                                                                    Ok(f617(f8(Str::lit("List")), f651(f652(v33, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f567(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f566(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
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
                                                Some(v27) => {
                                                    let m4569 = f269(Some(v27));
                                                    match m4569 {
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
                                Some(v19) => {
                                    let m4563 = f270(v19);
                                    match m4563.uncons() {
                                        None => {
                                            Ok(f566(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m4564 = v21;
                                            match m4564.uncons() {
                                                None => {
                                                    let m4565 = f269(f156(v20, f8(Str::lit("$ref"))));
                                                    match m4565 {
                                                        None => {
                                                            Ok(f566(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m4566 = f564(v24)?;
                                                            match m4566 {
                                                                None => {
                                                                    Ok(f566(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f567(v25, List::<Adt25>::nil()))
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
                        Some(v17) => {
                            let m4561 = f564(v17)?;
                            match m4561 {
                                None => {
                                    Ok(f566(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f567(v18, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f566(if f37(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt19::C6 => {
            Ok(f619())
        }
        Adt19::C7(_, _) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C8(_, _, _) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f644(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4573, _) = v0;
        h4573
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4574) = v0;
        h4574
    };
    let m4575 = v1;
    match m4575.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4576 = v4;
            match m4576.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f653(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f645(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4577, _) = v0.clone();
        h4577
    };
    let v2: List<(Bytes, ())> = {
        let (h4578, _) = {
            let (_, h4579) = v0.clone();
            h4579
        };
        h4578
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4580) = {
            let (_, h4581) = v0;
            h4581
        };
        h4580
    };
    let m4582 = v1.clone();
    match m4582.uncons() {
        None => {
            let m4583 = v2;
            match m4583.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4584 = v2.clone();
            match m4584.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f654({
                        let (h4585, _) = v8.clone();
                        h4585
                    }, {
                        let (h4586, _) = v6.clone();
                        h4586
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

pub fn f646(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4587, _) = v1;
        h4587
    };
    let m4589 = {
        let (_, h4588) = v1;
        h4588
    };
    match m4589 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4592 = {
                let a4590 = v0.clone();
                let a4591 = v2;
                index_bytes(a4590, a4591)
            };
            match m4592 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4597 = {
                        let a4595 = v0;
                        let a4596 = {
                            let a4593 = v2;
                            let a4594 = 1u64;
                            nat_add(a4593, a4594)?
                        };
                        index_bytes(a4595, a4596)
                    };
                    match m4597 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f655(v4)? {
                                f655(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4598 = v2;
                                    let a4599 = 2u64;
                                    nat_add(a4598, a4599)?
                                }, Some({
                                    let a4604 = v3;
                                    let a4605 = f343(f656(), {
                                        let a4602 = {
                                            let a4600 = f657(v4)?;
                                            let a4601 = 16u64;
                                            nat_mul(a4600, a4601)?
                                        };
                                        let a4603 = f657(v5)?;
                                        nat_add(a4602, a4603)?
                                    });
                                    append_bytes(a4604, a4605)
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

pub fn f647(p4610: List<(Bytes, Adt19)>) -> Option<Bytes> {
    let m4609 = p4610;
    {
        let mut m4609 = m4609;
        loop {
            let step = {
                let v0 = m4609;
                let m4606 = v0;
                match m4606.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f90({
                            let (h4607, _) = v1.clone();
                            h4607
                        }, f658()) {
                            Err::<List<(Bytes, Adt19)>, Option<Bytes>>(Some({
                                let (h4608, _) = v1;
                                h4608
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4609 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648(v0: Adt19) -> bool {
    if f574(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f574(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f574(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f574(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f574(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f574(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f649(v0: Adt19) -> Bytes {
    let m4611 = f659(f156(v0, f8(Str::lit("minimum"))));
    match m4611 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f650(v0: Adt19) -> Option<Adt19> {
    let m4612 = f156(v0.clone(), f8(Str::lit("additionalProperties")));
    match m4612 {
        None => {
            let m4613 = f436(f156(v0, f8(Str::lit("patternProperties"))));
            match m4613.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v2, v3)) => {
                    let m4614 = v3;
                    match m4614.uncons() {
                        None => {
                            Some({
                                let (_, h4615) = v2;
                                h4615
                            })
                        }
                        Some((_, _)) => {
                            None::<Adt19>
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

pub fn f651(v0: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4616 = v0;
    match m4616 {
        Ok(v2) => {
            f567(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h4617, _) = v2;
                h4617
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h4618) = v2;
                h4618
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v1)
        }
    }
}

pub fn f652(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f660(16u64, v0, v1)
}

pub fn f653(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4623 = {
            let a4621 = {
                let a4619 = v0.clone();
                length_list(a4619)
            };
            let a4622 = {
                let a4620 = v1.clone();
                length_list(a4620)
            };
            nat_add(a4621, a4622)?
        };
        let a4624 = 1u64;
        nat_add(a4623, a4624)?
    };
    Ok(f661({
        let (_, h4625) = {
            let (_, h4626) = {
                let (h4627, _) = f662(Fn25::F663, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4627
            };
            h4626
        };
        h4625
    }))
}

pub fn f654(v0: Bytes, v1: Bytes) -> bool {
    let a4632 = {
        let a4628 = v0;
        let a4629 = v1;
        compare_bytes(a4628, a4629)
    };
    let a4633 = {
        let a4630 = Bytes::lit(&[]);
        let a4631 = Bytes::lit(&[0x01]);
        compare_bytes(a4630, a4631)
    };
    equal(a4632, a4633)
}

pub fn f655(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4678 = {
            let a4654 = {
                let a4642 = {
                    let a4636 = {
                        let a4634 = v1;
                        let a4635 = 128u8;
                        fixed_u8::bit_and(a4634, a4635)
                    };
                    let a4637 = 128u8;
                    if equal(a4636, a4637) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4643 = {
                    let a4640 = {
                        let a4638 = v1;
                        let a4639 = 64u8;
                        fixed_u8::bit_and(a4638, a4639)
                    };
                    let a4641 = 64u8;
                    if equal(a4640, a4641) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4642, a4643)?
            };
            let a4655 = {
                let a4652 = {
                    let a4646 = {
                        let a4644 = v1;
                        let a4645 = 32u8;
                        fixed_u8::bit_and(a4644, a4645)
                    };
                    let a4647 = 32u8;
                    if equal(a4646, a4647) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4653 = {
                    let a4650 = {
                        let a4648 = v1;
                        let a4649 = 16u8;
                        fixed_u8::bit_and(a4648, a4649)
                    };
                    let a4651 = 16u8;
                    if equal(a4650, a4651) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4652, a4653)?
            };
            nat_add(a4654, a4655)?
        };
        let a4679 = {
            let a4676 = {
                let a4664 = {
                    let a4658 = {
                        let a4656 = v1;
                        let a4657 = 8u8;
                        fixed_u8::bit_and(a4656, a4657)
                    };
                    let a4659 = 8u8;
                    if equal(a4658, a4659) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4665 = {
                    let a4662 = {
                        let a4660 = v1;
                        let a4661 = 4u8;
                        fixed_u8::bit_and(a4660, a4661)
                    };
                    let a4663 = 4u8;
                    if equal(a4662, a4663) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4664, a4665)?
            };
            let a4677 = {
                let a4674 = {
                    let a4668 = {
                        let a4666 = v1;
                        let a4667 = 2u8;
                        fixed_u8::bit_and(a4666, a4667)
                    };
                    let a4669 = 2u8;
                    if equal(a4668, a4669) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4675 = {
                    let a4672 = {
                        let a4670 = v1;
                        let a4671 = 1u8;
                        fixed_u8::bit_and(a4670, a4671)
                    };
                    let a4673 = 1u8;
                    if equal(a4672, a4673) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4674, a4675)?
            };
            nat_add(a4676, a4677)?
        };
        nat_add(a4678, a4679)?
    };
    let a4680 = 48u64;
    let a4681 = v2;
    if if nat_le(a4680, a4681) {
        let a4682 = v2;
        let a4683 = 57u64;
        nat_le(a4682, a4683)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4684 = 97u64;
        let a4685 = v2;
        if nat_le(a4684, a4685) {
            let a4686 = v2;
            let a4687 = 102u64;
            Ok(nat_le(a4686, a4687))
        } else {
            Ok(false)
        }
    }
}

pub fn f656() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f657(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4732 = {
            let a4708 = {
                let a4696 = {
                    let a4690 = {
                        let a4688 = v1;
                        let a4689 = 128u8;
                        fixed_u8::bit_and(a4688, a4689)
                    };
                    let a4691 = 128u8;
                    if equal(a4690, a4691) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4697 = {
                    let a4694 = {
                        let a4692 = v1;
                        let a4693 = 64u8;
                        fixed_u8::bit_and(a4692, a4693)
                    };
                    let a4695 = 64u8;
                    if equal(a4694, a4695) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4696, a4697)?
            };
            let a4709 = {
                let a4706 = {
                    let a4700 = {
                        let a4698 = v1;
                        let a4699 = 32u8;
                        fixed_u8::bit_and(a4698, a4699)
                    };
                    let a4701 = 32u8;
                    if equal(a4700, a4701) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4707 = {
                    let a4704 = {
                        let a4702 = v1;
                        let a4703 = 16u8;
                        fixed_u8::bit_and(a4702, a4703)
                    };
                    let a4705 = 16u8;
                    if equal(a4704, a4705) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4706, a4707)?
            };
            nat_add(a4708, a4709)?
        };
        let a4733 = {
            let a4730 = {
                let a4718 = {
                    let a4712 = {
                        let a4710 = v1;
                        let a4711 = 8u8;
                        fixed_u8::bit_and(a4710, a4711)
                    };
                    let a4713 = 8u8;
                    if equal(a4712, a4713) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4719 = {
                    let a4716 = {
                        let a4714 = v1;
                        let a4715 = 4u8;
                        fixed_u8::bit_and(a4714, a4715)
                    };
                    let a4717 = 4u8;
                    if equal(a4716, a4717) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4718, a4719)?
            };
            let a4731 = {
                let a4728 = {
                    let a4722 = {
                        let a4720 = v1;
                        let a4721 = 2u8;
                        fixed_u8::bit_and(a4720, a4721)
                    };
                    let a4723 = 2u8;
                    if equal(a4722, a4723) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4729 = {
                    let a4726 = {
                        let a4724 = v1;
                        let a4725 = 1u8;
                        fixed_u8::bit_and(a4724, a4725)
                    };
                    let a4727 = 1u8;
                    if equal(a4726, a4727) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4728, a4729)?
            };
            nat_add(a4730, a4731)?
        };
        nat_add(a4732, a4733)?
    };
    let a4734 = v2;
    let a4735 = 57u64;
    if nat_le(a4734, a4735) {
        let a4736 = v2;
        let a4737 = 48u64;
        Ok(nat_sub(a4736, a4737))
    } else {
        let a4738 = v2;
        let a4739 = 87u64;
        Ok(nat_sub(a4738, a4739))
    }
}

pub fn f658() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f659(v0: Option<Adt19>) -> Option<u64> {
    let m4740 = v0;
    match m4740 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m4741 = v1;
            match m4741 {
                Adt19::C0(_) => {
                    None::<u64>
                }
                Adt19::C1(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt19::C2(_, _, _) => {
                    None::<u64>
                }
                Adt19::C3(_) => {
                    None::<u64>
                }
                Adt19::C4(_) => {
                    None::<u64>
                }
                Adt19::C5(_) => {
                    None::<u64>
                }
                Adt19::C6 => {
                    None::<u64>
                }
                Adt19::C7(_, _) => {
                    None::<u64>
                }
                Adt19::C8(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f660(v0: u64, v1: Adt19, v2: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4742 = v0;
    if m4742 == 0 {
        Ok(f566(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4742 - 1;
        let m4743 = v1.clone();
        match m4743 {
            Adt19::C0(v4) => {
                if v4 {
                    Ok(f619())
                } else {
                    Ok(f566(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt19::C1(_, _) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C2(_, _, _) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C3(_) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C4(_) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C5(_) => {
                let m4744 = f269(f156(v1.clone(), f8(Str::lit("$ref"))));
                match m4744 {
                    None => {
                        if if f574(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f574(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f574(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f567(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v20: Bytes = f159(v1.clone(), f8(Str::lit("type")));
                            if if f37(v20.clone(), Bytes::lit(&[])) {
                                !f648(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f619())
                            } else {
                                if f37(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f567(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f37(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f567(f649(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f37(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f567(f517(), List::<Adt25>::nil()))
                                        } else {
                                            if f37(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f567(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f37(v20.clone(), f8(Str::lit("array"))) {
                                                    let m4746 = f156(v1, f8(Str::lit("items")));
                                                    match m4746 {
                                                        None => {
                                                            Ok(f617(f8(Str::lit("List")), f619()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f617(f8(Str::lit("List")), f660(v3, v21, {
                                                                let a4747 = v2;
                                                                let a4748 = f8(Str::lit("Item"));
                                                                append_bytes(a4747, a4748)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f37(v20.clone(), f8(Str::lit("object"))) {
                                                        let m4749 = f650(v1);
                                                        match m4749 {
                                                            None => {
                                                                Ok(f617(f8(Str::lit("List")), f651(f619())))
                                                            }
                                                            Some(v22) => {
                                                                Ok(f617(f8(Str::lit("List")), f651(f660(v3, v22, {
                                                                    let a4750 = v2;
                                                                    let a4751 = f8(Str::lit("Value"));
                                                                    append_bytes(a4750, a4751)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f566(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
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
                        let m4745 = f564(v18)?;
                        match m4745 {
                            None => {
                                Ok(f566(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f567(v19, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt19::C6 => {
                Ok(f619())
            }
            Adt19::C7(_, _) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C8(_, _, _) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f661(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f446(Fn10::F664, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f662(v0: Fn25, p4756: u64, p4757: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4755 = (p4756, p4757);
    {
        let mut m4755 = m4755;
        loop {
            let step = {
                let (v1, v2) = m4755;
                let m4752 = v1;
                if m4752 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4752 - 1;
                    let m4754 = {
                        let c4753: Fn25 = v0.clone();
                        c4753.apply(v2.clone())
                    };
                    match m4754 {
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
                Ok(next) => { m4755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f663(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f665(v0)
}

pub fn f664(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f665(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4758, _) = v0.clone();
        h4758
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4759, _) = {
            let (_, h4760) = v0.clone();
            h4760
        };
        h4759
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4761) = {
            let (_, h4762) = v0;
            h4762
        };
        h4761
    };
    let m4763 = v1.clone();
    match m4763.uncons() {
        None => {
            let m4764 = v2;
            match m4764.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4765 = v2.clone();
            match m4765.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f654({
                        let (h4766, _) = v8.clone();
                        h4766
                    }, {
                        let (h4767, _) = v6.clone();
                        h4767
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
