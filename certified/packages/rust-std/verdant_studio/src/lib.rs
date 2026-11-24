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
    F54(u8, Bytes),
    F155(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F54(k0, k1) => f54(*k0, k1.clone(), p0),
            Fn0::F155(k0) => f155(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F134,
    F151,
    F207,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F134 => f134(p0, p1),
            Fn1::F151 => f151(p0, p1),
            Fn1::F207 => f207(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F173(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F173(k0) => f173(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F196(Bytes, u64, Bytes, u64),
    F272(Bytes),
    F273(u64, Bytes),
    F334(Bytes),
    F380(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F196(k0, k1, k2, k3) => f196(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F272(k0) => Ok(f272(k0.clone(), p0)),
            Fn3::F273(k0, k1) => f273(*k0, k1.clone(), p0),
            Fn3::F334(k0) => f334(k0.clone(), p0),
            Fn3::F380(k0) => f380(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F227(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<(u64, Adt20)> {
        match self {
            Fn4::F227(k0, k1, k2) => f227(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F256,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn5::F256 => f256(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F259,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn6::F259 => f259(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F261,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn7::F261 => f261(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F264,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn8::F264 => f264(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F288(Bytes),
    F344(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F288(k0) => f288(k0.clone(), p0),
            Fn9::F344(k0, k1) => Ok(f344(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F290,
    F413,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F290 => f290(p0, p1),
            Fn10::F413 => f413(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F293,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F293 => f293(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F295,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F295 => f295(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F298,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F298 => f298(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F302,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt23) -> R<Adt23> {
        match self {
            Fn14::F302 => f302(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F307(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F307(k0) => f307(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F337(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F337(k0) => f337(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F348,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F348 => f348(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F350,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F350 => f350(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F352,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F352 => f352(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F360,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F360 => f360(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F372,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F372 => f372(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F395,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F395 => f395(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F397,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F397 => f397(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F399(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F399(k0) => f399(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F412,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F412 => f412(p0),
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
                                                f43(v1, v4)
                                            } else {
                                                if f36(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                    f44(v1, v4, false)
                                                } else {
                                                    if f36(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                        f44(v1, v4, true)
                                                    } else {
                                                        if f36(v2.clone(), f8(Str::lit("admit"))) {
                                                            f45(v1, v4)
                                                        } else {
                                                            if f36(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                f46(v1, v4)
                                                            } else {
                                                                if f36(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                    f47(v1, v4)
                                                                } else {
                                                                    if f36(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                        f48(v1, v4)
                                                                    } else {
                                                                        if f36(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                            f49(v1, v4)
                                                                        } else {
                                                                            if f36(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                f50(v1, v4)
                                                                            } else {
                                                                                if f36(v2, f8(Str::lit("propose-encoding"))) {
                                                                                    let m179 = v5;
                                                                                    match m179.uncons() {
                                                                                        None => {
                                                                                            Ok(Adt11::C4(f35()))
                                                                                        }
                                                                                        Some((v6, v7)) => {
                                                                                            f51(v1, v4, v6, v7)
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

pub fn f29(v0: u8) -> R<Str> {
    let m228 = {
        let a226 = f52();
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
        let (h230, _) = f53(Fn0::F54(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h230
    };
    let v4: u64 = {
        let (h231, _) = {
            let (_, h232) = v3.clone();
            h232
        };
        h231
    };
    Ok(f55(List::cons(f32(v0, v4, {
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
                        if f56(v1.clone(), 0u64, {
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
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f36(v0: Bytes, v1: Bytes) -> bool {
    let a251 = v0;
    let a252 = v1;
    equal(a251, a252)
}

pub fn f37(v0: Adt1) -> R<Adt11> {
    let m253 = f57(v0.clone(), f58());
    match m253.uncons() {
        None => {
            let m254 = f59(v0.clone(), f58())?;
            match m254.uncons() {
                None => {
                    let m257 = f60(match v0.clone() {
                        Adt1::C0(_, h255, _) => {
                            h255
                        }
                    }, f61(f62(), f63(match v0.clone() {
                        Adt1::C0(_, h256, _) => {
                            h256
                        }
                    }, false)));
                    match m257.uncons() {
                        None => {
                            let m259 = f57(v0.clone(), f63(match v0.clone() {
                                Adt1::C0(_, h258, _) => {
                                    h258
                                }
                            }, true));
                            match m259.uncons() {
                                None => {
                                    let m261 = f64(v0.clone(), f63(match v0.clone() {
                                        Adt1::C0(_, h260, _) => {
                                            h260
                                        }
                                    }, true))?;
                                    match m261 {
                                        None => {
                                            let v10: (bool, Bytes) = f65(Adt13::C0(Adt12::C0(f66(f67(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f68(v0.clone())?, f69(f70(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f66(f67(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f69(f70(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f70(v0.clone(), f8(Str::lit("model/admissions.json")))?, f70(v0.clone(), f8(Str::lit("model/strata.json")))?, f71(f70(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f63(match v0 {
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
    if f72(f73(match v0.clone() {
        Adt1::C0(_, h266, _) => {
            h266
        }
    }, f74())) {
        Ok(Adt11::C1(List::cons(f74(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f75(f76(f74(), f77(match v0.clone() {
            Adt1::C0(_, h267, _) => {
                h267
            }
        }, f74())))?;
        let m268 = f57(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m268.uncons() {
            None => {
                let m269 = f78(v0.clone(), f79(f80(f67(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f81(v2, v1.clone()))?;
                match m269 {
                    None => {
                        f82(v0, v1)
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
    if f72(f83(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h270) => {
            h270
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f84(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f72(f83(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h271) => {
                h271
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f84(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f40(v0: Adt1) -> R<Adt11> {
    let m272 = f57(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f85(), List::<Bytes>::nil()))));
    match m272.uncons() {
        None => {
            Ok(Adt11::C3(f86(f87(v0.clone())?, List::cons((f88(), f89()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f90()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f91()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f92()), List::cons((f8(Str::lit("docs/model.md")), f93(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f94()), List::<(Bytes, Bytes)>::nil()))))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md and docs/owner-decisions.md\u{a}"))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f41(v0: Adt1) -> R<Adt11> {
    let m273 = f57(v0.clone(), List::cons(f95(), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil())));
    match m273.uncons() {
        None => {
            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f96(f97(f98(f99(f67(v0.clone(), f95()))?)), f100(f70(v0, f8(Str::lit("model/admissions.json")))?)))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f42(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m274 = f57(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m274.uncons() {
        None => {
            let v4: Bytes = f67(v0.clone(), v1);
            let v5: List<List<Bytes>> = f99(v4.clone())?;
            if f72(f101(f102(v5.clone()), f103())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f104(Str::lit("id"), v6.clone())?, List::cons(f104(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f104(Str::lit("artifact"), f95())?, List::cons(f104(Str::lit("checksum"), f105(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f95(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f106(f67(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f107(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f108(f97(f98(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f43(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m275 = f57(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m275.uncons() {
        None => {
            let m276 = f109(f80(f67(v0, v1))?)?;
            match m276 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f108(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f44(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m277 = f57(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m277.uncons() {
        None => {
            let v5: List<Bytes> = f80(f67(v0.clone(), v1.clone()))?;
            if f72(f110(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f111(v5)?
                } else {
                    f112(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f113(f114(), f115(v1.clone())?), f67(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f113(f114(), f115(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f116()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f45(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m278 = f57(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil())))));
    match m278.uncons() {
        None => {
            let v4: List<Bytes> = f80(f67(v0.clone(), v1))?;
            let m279 = f117(v4.clone(), f8(Str::lit("requirement")))?;
            match m279 {
                None => {
                    Ok(Adt11::C4(f116()))
                }
                Some(v5) => {
                    let m280 = f117(v4, f8(Str::lit("admission")))?;
                    match m280 {
                        None => {
                            Ok(Adt11::C4(f116()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f70(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f70(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f61(f69(v5.clone(), f8(Str::lit("id"))), f69(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f75(f61(f69(v7.clone(), f8(Str::lit("id"))), f69(v8.clone(), f8(Str::lit("construct")))))?;
                            let a282 = {
                                let a281 = v9.clone();
                                length_list(a281)
                            };
                            let a283 = 0u64;
                            if nat_eq(a282, a283) {
                                Ok(Adt11::C4(f116()))
                            } else {
                                if f72({
                                    let a286 = {
                                        let a284 = f75(f61(v9.clone(), v10.clone()))?;
                                        length_list(a284)
                                    };
                                    let a287 = {
                                        let a285 = f61(v9.clone(), v10);
                                        length_list(a285)
                                    };
                                    nat_eq(a286, a287)
                                }) {
                                    Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                } else {
                                    Ok(Adt11::C3(List::cons(f118(f8(Str::lit("model/requirements.json")), f119(v7, v5))?, List::cons(f118(f8(Str::lit("model/admissions.json")), f119(v8, v6))?, List::cons(f107(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f108(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f46(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m288 = f57(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m288.uncons() {
        None => {
            let m289 = f120(f80(f67(v0, v1))?)?;
            match m289 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f108(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f47(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m290 = f57(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m290.uncons() {
        None => {
            let m291 = f121(v0.clone(), v1.clone())?;
            match m291 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m292 = f122(v4.clone(), f8(Str::lit("schema")));
                    match m292 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f123(v0, f124(v5.clone())?, v5, f125(v4.clone(), f8(Str::lit("name"))), f71(f126(f122(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
    let m293 = f57(v0.clone(), List::cons(v1.clone(), List::cons(f85(), List::<Bytes>::nil())));
    match m293.uncons() {
        None => {
            let m294 = f127(f80(f67(v0.clone(), v1))?)?;
            match m294 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f70(v0, f85())?;
                    let v6: List<Bytes> = f69(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f75(f69(v5.clone(), f8(Str::lit("at"))))?;
                    let a296 = {
                        let a295 = v6.clone();
                        length_list(a295)
                    };
                    let a297 = 0u64;
                    if if nat_eq(a296, a297) {
                        true
                    } else {
                        f72({
                            let a300 = {
                                let a298 = f75(f61(v6.clone(), v7.clone()))?;
                                length_list(a298)
                            };
                            let a301 = {
                                let a299 = f61(v6.clone(), v7);
                                length_list(a299)
                            };
                            nat_eq(a300, a301)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f118(f85(), f119(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f108(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m302 = f57(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m302.uncons() {
        None => {
            let m303 = f121(v0, v1)?;
            match m303 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f128(f126(Some(v4))) {
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

pub fn f50(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m304 = f57(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m304.uncons() {
        None => {
            let m305 = f121(v0.clone(), v1.clone())?;
            match m305 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f72(f36(f125(v4.clone(), f8(Str::lit("schema"))), f129())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f125(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m306 = f122(v4.clone(), f8(Str::lit("source")));
                            match m306 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f130(f122(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f113(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f113(v9.clone(), f8(Str::lit("binding.json"))), f67(v0.clone(), v1));
                        let m307 = v8;
                        match m307 {
                            None => {
                                f131(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f72(f36(f125(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f131(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f113(v9, f115(f125(v7.clone(), f8(Str::lit("url"))))?);
                                    if f72(f83(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h308) => {
                                            h308
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f125(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m309 = f57(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m309.uncons() {
                                            None => {
                                                if f36(f105(f67(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f131(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f51(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m310 = f57(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m310.uncons() {
        None => {
            let m311 = f121(v0.clone(), v1.clone())?;
            match m311 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f123(v0.clone(), f67(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f52() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f53(v0: Fn0, p316: u64, p317: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m315 = (p316, p317);
    {
        let mut m315 = m315;
        loop {
            let step = {
                let (v1, v2) = m315;
                let m312 = v1;
                if m312 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m312 - 1;
                    let m314 = {
                        let c313: Fn0 = v0.clone();
                        c313.apply(v2.clone())?
                    };
                    match m314 {
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
                Ok(next) => { m315 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f132(v1, v0, v2)
}

pub fn f55(v0: List<Bytes>) -> List<Bytes> {
    f133(Fn1::F134, List::<Bytes>::nil(), v0)
}

pub fn f56(v0: Bytes, p373: u64, p374: u64) -> R<bool> {
    let m372 = (p373, p374);
    {
        let mut m372 = m372;
        loop {
            let step = {
                let (v1, v2) = m372;
                let m318 = v2;
                if m318 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m318 - 1;
                    let m321 = {
                        let a319 = v0.clone();
                        let a320 = v1;
                        index_bytes(a319, a320)
                    };
                    match m321 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a368 = {
                                let v5: u8 = v4;
                                let a366 = {
                                    let a342 = {
                                        let a330 = {
                                            let a324 = {
                                                let a322 = v5;
                                                let a323 = 128u8;
                                                fixed_u8::bit_and(a322, a323)
                                            };
                                            let a325 = 128u8;
                                            if equal(a324, a325) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a331 = {
                                            let a328 = {
                                                let a326 = v5;
                                                let a327 = 64u8;
                                                fixed_u8::bit_and(a326, a327)
                                            };
                                            let a329 = 64u8;
                                            if equal(a328, a329) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a330, a331)?
                                    };
                                    let a343 = {
                                        let a340 = {
                                            let a334 = {
                                                let a332 = v5;
                                                let a333 = 32u8;
                                                fixed_u8::bit_and(a332, a333)
                                            };
                                            let a335 = 32u8;
                                            if equal(a334, a335) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a341 = {
                                            let a338 = {
                                                let a336 = v5;
                                                let a337 = 16u8;
                                                fixed_u8::bit_and(a336, a337)
                                            };
                                            let a339 = 16u8;
                                            if equal(a338, a339) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a340, a341)?
                                    };
                                    nat_add(a342, a343)?
                                };
                                let a367 = {
                                    let a364 = {
                                        let a352 = {
                                            let a346 = {
                                                let a344 = v5;
                                                let a345 = 8u8;
                                                fixed_u8::bit_and(a344, a345)
                                            };
                                            let a347 = 8u8;
                                            if equal(a346, a347) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a353 = {
                                            let a350 = {
                                                let a348 = v5;
                                                let a349 = 4u8;
                                                fixed_u8::bit_and(a348, a349)
                                            };
                                            let a351 = 4u8;
                                            if equal(a350, a351) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a352, a353)?
                                    };
                                    let a365 = {
                                        let a362 = {
                                            let a356 = {
                                                let a354 = v5;
                                                let a355 = 2u8;
                                                fixed_u8::bit_and(a354, a355)
                                            };
                                            let a357 = 2u8;
                                            if equal(a356, a357) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a363 = {
                                            let a360 = {
                                                let a358 = v5;
                                                let a359 = 1u8;
                                                fixed_u8::bit_and(a358, a359)
                                            };
                                            let a361 = 1u8;
                                            if equal(a360, a361) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a362, a363)?
                                    };
                                    nat_add(a364, a365)?
                                };
                                nat_add(a366, a367)?
                            };
                            let a369 = 127u64;
                            if nat_le(a368, a369) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a370 = v1;
                                    let a371 = 1u64;
                                    nat_add(a370, a371)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m372 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m375 = v1;
    match m375.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m377 = f135(match v0.clone() {
                Adt1::C0(h376, _, _) => {
                    h376
                }
            }, v2.clone());
            match m377 {
                None => {
                    List::cons(v2, f57(v0, v3))
                }
                Some(_) => {
                    f57(v0, v3)
                }
            }
        }
    }
}

pub fn f58() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f95(), List::<Bytes>::nil())))))))
}

pub fn f59(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m378 = v1;
    match m378.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f136(v0.clone(), v2.clone()) {
                f110(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m379 = f121(v0.clone(), v2.clone())?;
                match m379 {
                    None => {
                        Ok(List::cons(v2, f59(v0, v3)?))
                    }
                    Some(_) => {
                        f59(v0, v3)
                    }
                }
            } else {
                f59(v0, v3)
            }
        }
    }
}

pub fn f60(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m380 = v1;
    match m380.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f73(v0.clone(), v2.clone()) {
                f60(v0, v3)
            } else {
                List::cons(v2, f60(v0, v3))
            }
        }
    }
}

pub fn f61(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m381 = v0;
    match m381.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f61(v3, v1))
        }
    }
}

pub fn f62() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f63(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m382 = v0;
    match m382.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f61(f137({
                let (h383, _) = v2;
                h383
            }, {
                let (_, h384) = v2;
                h384
            }, v1), f63(v3, v1))
        }
    }
}

pub fn f64(v0: Adt1, p388: List<Bytes>) -> R<Option<Bytes>> {
    let m387 = p388;
    {
        let mut m387 = m387;
        loop {
            let step = {
                let v1 = m387;
                let m385 = v1;
                match m385.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m386 = f138(f67(v0.clone(), v2.clone()), f139())?;
                        match m386 {
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
                Ok(next) => { m387 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h389, _, _, _, _) => {
            h389
        }
    };
    let v2: Adt15 = f140(v1.clone())?;
    let v3: List<Bytes> = f141(v1.clone());
    let v4: List<Bytes> = f69(match v0.clone() {
        Adt13::C0(_, _, h390, _, _) => {
            h390
        }
    }, f8(Str::lit("path")));
    let m392 = f142(v2, List::<u64>::nil(), f143(v1.clone(), match v0.clone() {
        Adt13::C0(_, h391, _, _, _) => {
            h391
        }
    }, 0u64)?);
    match m392 {
        None => {
            if f145(f146(match v0.clone() {
                Adt13::C0(_, _, _, _, h394) => {
                    h394
                }
            }, f61(match v0.clone() {
                Adt13::C0(_, _, _, _, h395) => {
                    h395
                }
            }, v4.clone()))?, f146(v4.clone(), f61(match v0.clone() {
                Adt13::C0(_, _, _, _, h396) => {
                    h396
                }
            }, v4))?, f147(match v0.clone() {
                Adt13::C0(_, _, h397, _, _) => {
                    h397
                }
            })) {
                let m401 = f148(f146(match v0.clone() {
                    Adt13::C0(_, _, _, h398, _) => {
                        h398
                    }
                }, f61(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h399, _) => {
                        h399
                    }
                }))?, f146(v3.clone(), f61(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h400, _) => {
                        h400
                    }
                }))?);
                match m401 {
                    None => {
                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f108(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f144(f61(v3, match v0 {
                            Adt13::C0(_, _, _, h402, _) => {
                                h402
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f144(match v1 {
                Adt12::C0(_, _, _, _, h393) => {
                    h393
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f66(v0: Bytes) -> R<List<Bytes>> {
    f149(f80(v0)?, f8(Str::lit("id = \"")))
}

pub fn f67(v0: Adt1, v1: Bytes) -> Bytes {
    let m404 = f135(match v0 {
        Adt1::C0(h403, _, _) => {
            h403
        }
    }, v1);
    match m404 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m405 = v2;
            match m405 {
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

pub fn f68(v0: Adt1) -> R<List<Bytes>> {
    let m406 = f99(f67(v0, f95()))?;
    match m406.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f97(v2))
        }
    }
}

pub fn f69(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m407 = v0;
    match m407.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f125(v2, v1.clone()), f69(v3, v1))
        }
    }
}

pub fn f70(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f126(f121(v0, v1)?))
}

pub fn f71(v0: List<Adt14>) -> List<Bytes> {
    let m408 = v0;
    match m408.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m409 = f130(Some(v1));
            match m409 {
                None => {
                    f71(v2)
                }
                Some(v3) => {
                    List::cons(v3, f71(v2))
                }
            }
        }
    }
}

pub fn f72(v0: bool) -> bool {
    !v0
}

pub fn f73(p413: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
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
                        if f36({
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

pub fn f74() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f75(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f150(f133(Fn1::F151, List::<Bytes>::nil(), f152(v0)?)))
}

pub fn f76(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m414 = v1;
    match m414.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f110(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f113(v0.clone(), v2), f76(v0, v3))
            } else {
                f76(v0, v3)
            }
        }
    }
}

pub fn f77(p419: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m418 = p419;
    {
        let mut m418 = m418;
        loop {
            let step = {
                let v0 = m418;
                let m415 = v0;
                match m415.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h416, _) = v2;
                            h416
                        }, v1.clone()) {
                            let (_, h417) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h417)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m418 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: Adt1, v1: List<List<Bytes>>, p422: List<Bytes>) -> R<Option<Bytes>> {
    let m421 = p422;
    {
        let mut m421 = m421;
        loop {
            let step = {
                let v2 = m421;
                let m420 = v2;
                match m420.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f153(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m421 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(p425: List<Bytes>, p426: List<Bytes>, p427: List<List<Bytes>>) -> List<List<Bytes>> {
    let m424 = (p425, (p426, p427));
    {
        let mut m424 = m424;
        loop {
            let step = {
                let (v0, (v1, v2)) = m424;
                let m423 = v0;
                match m423.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f154(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f36(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f154(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m424 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a428 = v0.clone();
        length_bytes(a428)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h429, _) = f53(Fn0::F155(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h429
    };
    let v3: u64 = {
        let (h430, _) = {
            let (_, h431) = v2.clone();
            h431
        };
        h430
    };
    let v4: List<Bytes> = {
        let (_, h432) = {
            let (_, h433) = v2;
            h433
        };
        h432
    };
    let a434 = v3;
    let a435 = v1;
    if nat_lt(a434, a435) {
        Ok(f55(List::cons(f32(v0, v3, {
            let a436 = v1;
            let a437 = v3;
            nat_sub(a436, a437)
        }), v4)))
    } else {
        Ok(f55(v4))
    }
}

pub fn f81(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m438 = v0;
    match m438.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f83(v2.clone(), v1.clone()) {
                f81(v3, v1)
            } else {
                List::cons(v2, f81(v3, v1))
            }
        }
    }
}

pub fn f82(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f72(f73(match v0.clone() {
        Adt1::C0(_, h439, _) => {
            h439
        }
    }, f156())) {
        Ok(Adt11::C1(List::cons(f156(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f81(f75(f76(f156(), f77(match v0.clone() {
            Adt1::C0(_, h440, _) => {
                h440
            }
        }, f156())))?, v1.clone());
        let m441 = f57(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m441.uncons() {
            None => {
                let m442 = f157(v0.clone(), v2, f69(f70(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m442 {
                    None => {
                        f158(v0, v1)
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

pub fn f83(v0: Bytes, p445: List<Bytes>) -> bool {
    let m444 = p445;
    {
        let mut m444 = m444;
        loop {
            let step = {
                let v1 = m444;
                let m443 = v1;
                match m443.uncons() {
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
                Ok(next) => { m444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f85() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f86(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m446 = v0;
    match m446.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f86(v3, v1))
        }
    }
}

pub fn f87(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m447 = f70(v0.clone(), f85())?;
    match m447.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f159(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f88() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f89() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f90() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**', 'model/encodings/**', 'model/queue/**', 'model/authorities.toml']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            while read -r file line; do head -n \"$line\" \"$file\" | grep '^  name: ' | tail -n 1 | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }'; done < target/hunks.txt | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f91() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src and\u{a}# README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one.\u{a}name: refresh\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  refresh:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            state=unknown\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              state=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\u{a}              if [ \"$state\" != unknown ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$state\" = dirty ]; then stale=\"$stale $pr\"; fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply each change to the new main, export it again, and push it to its own pull request\u{a}        if: steps.stale.outputs.stale != ''\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ steps.stale.outputs.stale }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /'\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}"))
}

pub fn f92() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f93(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f160(f70(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f161(f70(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f94() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f95() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f96(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m448 = v0;
    match m448.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f83(v2.clone(), v1.clone()) {
                f96(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f96(v3, v1))
            }
        }
    }
}

pub fn f97(v0: List<List<Bytes>>) -> List<Bytes> {
    let m449 = v0;
    match m449.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m450 = v1;
            match m450.uncons() {
                None => {
                    f97(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f97(v2))
                }
            }
        }
    }
}

pub fn f98(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m451 = v0;
    match m451.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f99(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt16 = f162(v0.clone(), 0u64, {
        let a452 = v0;
        length_bytes(a452)
    }, Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt16 = {
        let a455 = {
            let a454 = match v1.clone() {
                Adt16::C0(_, _, h453, _, _) => {
                    h453
                }
            };
            length_bytes(a454)
        };
        let a456 = 0u64;
        if if nat_eq(a455, a456) {
            let m458 = match v1.clone() {
                Adt16::C0(_, _, _, h457, _) => {
                    h457
                }
            };
            match m458.uncons() {
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
            f163(v1)
        }
    };
    Ok(f164(match v4 {
        Adt16::C0(_, _, _, _, h459) => {
            h459
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f100(v0: List<Adt14>) -> List<Bytes> {
    let m460 = v0;
    match m460.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m461 = f122(v1, f8(Str::lit("std")));
            match m461 {
                None => {
                    f100(v2)
                }
                Some(v3) => {
                    List::cons(f125(v3, f8(Str::lit("row"))), f100(v2))
                }
            }
        }
    }
}

pub fn f101(p466: List<Bytes>, p467: List<Bytes>) -> bool {
    let m465 = (p466, p467);
    {
        let mut m465 = m465;
        loop {
            let step = {
                let (v0, v1) = m465;
                let m462 = v0;
                match m462.uncons() {
                    None => {
                        let m463 = v1;
                        match m463.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m464 = v1;
                        match m464.uncons() {
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
                Ok(next) => { m465 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: List<List<Bytes>>) -> List<Bytes> {
    let m468 = v0;
    match m468.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f103() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f104(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f165(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f105(v0: Bytes) -> R<Bytes> {
    Ok(f166(f167(v0)?))
}

pub fn f106(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f168(f79(f80(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f169(f170(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f107(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f124(Adt14::C3(std::rc::Rc::new(f171(f75(f61(v1, f71(f70(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f108(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m469 = f172(Fn2::F173(v1), None::<Bytes>, v0);
    match m469 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f109(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m470 = v0;
    match m470.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f174(v1)?;
            let a472 = {
                let a471 = v3.clone();
                length_bytes(a471)
            };
            let a473 = 0u64;
            if nat_eq(a472, a473) {
                f109(v2)
            } else {
                let m474 = f175(v3.clone())?;
                match m474 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f125(v4.clone(), f8(Str::lit("id")));
                        if if f36(f125(v4, f8(Str::lit("schema"))), f129()) {
                            f72({
                                let a476 = {
                                    let a475 = v5.clone();
                                    length_bytes(a475)
                                };
                                let a477 = 0u64;
                                nat_eq(a476, a477)
                            })
                        } else {
                            false
                        } {
                            let m478 = f109(v2)?;
                            match m478 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f176(v5), {
                                        let a479 = v3;
                                        let a480 = Bytes::lit(&[0x0a]);
                                        append_bytes(a479, a480)
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

pub fn f110(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a481 = v0.clone();
        length_bytes(a481)
    };
    let v3: u64 = {
        let a482 = v1.clone();
        length_bytes(a482)
    };
    let a483 = v3;
    let a484 = v2;
    if nat_le(a483, a484) {
        let m490 = {
            let a487 = v0;
            let a488 = {
                let a485 = v2;
                let a486 = v3;
                nat_sub(a485, a486)
            };
            let a489 = v3;
            slice_bytes(a487, a488, a489)
        };
        match m490 {
            None => {
                false
            }
            Some(v4) => {
                let a491 = v4;
                let a492 = v1;
                equal(a491, a492)
            }
        }
    } else {
        false
    }
}

pub fn f111(v0: List<Bytes>) -> R<bool> {
    if f177(f127(v0.clone())?) {
        Ok(f72({
            let a494 = {
                let a493 = f178(f127(v0)?);
                length_list(a493)
            };
            let a495 = 0u64;
            nat_eq(a494, a495)
        }))
    } else {
        Ok(false)
    }
}

pub fn f112(v0: List<Bytes>) -> R<bool> {
    if f177(f117(v0.clone(), f8(Str::lit("requirement")))?) {
        if f177(f117(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f72({
                let a497 = {
                    let a496 = f119(f178(f117(v0.clone(), f8(Str::lit("requirement")))?), f178(f117(v0, f8(Str::lit("admission")))?));
                    length_list(a496)
                };
                let a498 = 0u64;
                nat_eq(a497, a498)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f113(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f114() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f115(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f116() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f117(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m499 = v0;
    match m499.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f174(v2)?;
            let a501 = {
                let a500 = v4.clone();
                length_bytes(a500)
            };
            let a502 = 0u64;
            if nat_eq(a501, a502) {
                f117(v3, v1)
            } else {
                let m503 = f175(v4)?;
                match m503 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f179(v5.clone());
                        let a505 = {
                            let a504 = v6.clone();
                            length_bytes(a504)
                        };
                        let a506 = 0u64;
                        if nat_eq(a505, a506) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m507 = f117(v3, v1.clone())?;
                            match m507 {
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

pub fn f118(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f124(Adt14::C3(std::rc::Rc::new(f180(v1))))?))
}

pub fn f119(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m508 = v0;
    match m508.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f119(v3, v1))
        }
    }
}

pub fn f120(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m509 = v0;
    match m509.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f174(v1)?;
            let a511 = {
                let a510 = v3.clone();
                length_bytes(a510)
            };
            let a512 = 0u64;
            if nat_eq(a511, a512) {
                f120(v2)
            } else {
                let m513 = f175(v3.clone())?;
                match m513 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f181(v4.clone()) {
                            let m514 = f120(v2)?;
                            match m514 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f156(), List::cons(f8(Str::lit("/")), List::cons(f182(f125(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a515 = v3;
                                        let a516 = Bytes::lit(&[0x0a]);
                                        append_bytes(a515, a516)
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

pub fn f121(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f175(f67(v0, v1))
}

pub fn f122(p522: Adt14, v1: Bytes) -> Option<Adt14> {
    let m521 = p522;
    {
        let mut m521 = m521;
        loop {
            let step = {
                let v0 = m521;
                let m517 = v0;
                match m517 {
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
                    Adt14::C4(r518) => {
                        let v5 = (*r518).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r519, r520) => {
                        let v3 = (*r519).clone();
                        let v4 = (*r520).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m521 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m523 = f57(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m523.uncons() {
        None => {
            let m524 = f183(v4);
            match m524 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a525 = f8(Str::lit("Catalog."));
                        let a526 = f182(v3.clone())?;
                        append_bytes(a525, a526)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f182(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m527 = f184(v2, v9.clone(), f182(v3.clone())?)?;
                    match m527 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f182(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f124(Adt14::C3(std::rc::Rc::new(f180(f185(f70(v0.clone(), f8(Str::lit("model/admissions.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f124(Adt14::C3(std::rc::Rc::new(f180(f185(f70(v0.clone(), f8(Str::lit("model/strata.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a531 = f186(f67(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a529 = {
                                        let a528 = v10;
                                        length_bytes(a528)
                                    };
                                    let a530 = 4u64;
                                    nat_sub(a529, a530)
                                }))?;
                                let a532 = Bytes::lit(&[0x0a]);
                                append_bytes(a531, a532)
                            }), List::cons(f107(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f124(v0: Adt14) -> R<Bytes> {
    let a533 = f187(v0)?;
    let a534 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a533, a534))
}

pub fn f125(v0: Adt14, v1: Bytes) -> Bytes {
    let m535 = f130(f122(v0, v1));
    match m535 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f126(v0: Option<Adt14>) -> List<Adt14> {
    let m536 = v0;
    match m536 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f188(v1)
        }
    }
}

pub fn f127(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m537 = v0;
    match m537.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f174(v1)?;
            let a539 = {
                let a538 = v3.clone();
                length_bytes(a538)
            };
            let a540 = 0u64;
            if nat_eq(a539, a540) {
                f127(v2)
            } else {
                let m541 = f175(v3)?;
                match m541 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f189(v4.clone(), f8(Str::lit("agent"))) {
                            if f189(v4.clone(), f8(Str::lit("at"))) {
                                if f189(v4.clone(), f8(Str::lit("quote"))) {
                                    f190(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m542 = f127(v2)?;
                            match m542 {
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

pub fn f128(p545: List<Adt14>) -> bool {
    let m544 = p545;
    {
        let mut m544 = m544;
        loop {
            let step = {
                let v0 = m544;
                let m543 = v0;
                match m543.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f125(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f83(f8(Str::lit("studio")), f69(f191(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m544 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f130(v0: Option<Adt14>) -> Option<Bytes> {
    let m546 = v0;
    match m546 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m547 = v1;
            match m547 {
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

pub fn f131(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f106(f67(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f192(v1, v4)?)?), List::cons(f107(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f132(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h548, _) = v2.clone();
        h548
    };
    let v4: u64 = {
        let (h549, _) = {
            let (_, h550) = v2.clone();
            h550
        };
        h549
    };
    let v5: List<Bytes> = {
        let (_, h551) = {
            let (_, h552) = v2;
            h552
        };
        h551
    };
    let m553 = f193(v0.clone(), v3);
    match m553 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a554 = v6;
            let a555 = v1;
            if equal(a554, a555) {
                Ok(Some(({
                    let a556 = v3;
                    let a557 = 1u64;
                    nat_add(a556, a557)?
                }, ({
                    let a558 = v3;
                    let a559 = 1u64;
                    nat_add(a558, a559)?
                }, List::cons(f32(v0, v4, {
                    let a560 = v3;
                    let a561 = v4;
                    nat_sub(a560, a561)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a562 = v3;
                    let a563 = 1u64;
                    nat_add(a562, a563)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f133(v0: Fn1, p567: List<Bytes>, p568: List<Bytes>) -> List<Bytes> {
    let m566 = (p567, p568);
    {
        let mut m566 = m566;
        loop {
            let step = {
                let (v1, v2) = m566;
                let m564 = v2;
                match m564.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c565: Fn1 = v0.clone();
                            c565.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m566 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f135(p573: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m572 = p573;
    {
        let mut m572 = m572;
        loop {
            let step = {
                let v0 = m572;
                let m569 = v0;
                match m569.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h570, _) = v2;
                            h570
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h571) = v2;
                                h571
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m572 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: Adt1, v1: Bytes) -> bool {
    let m575 = f135(match v0 {
        Adt1::C0(h574, _, _) => {
            h574
        }
    }, v1);
    match m575 {
        None => {
            false
        }
        Some(v2) => {
            let m576 = v2;
            match m576 {
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

pub fn f137(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m577 = v1;
    match m577.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f194(v3.clone())
            } else {
                f72(f194(v3.clone()))
            } {
                List::cons(f113(v0.clone(), v3), f137(v0, v4, v2))
            } else {
                f137(v0, v4, v2)
            }
        }
    }
}

pub fn f138(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a578 = v0.clone();
        length_bytes(a578)
    };
    let v3: u64 = {
        let a579 = v1.clone();
        length_bytes(a579)
    };
    let v4: u64 = {
        let (h580, _) = f195(Fn3::F196(v1.clone(), v3, v0.clone(), v2), {
            let a581 = v2;
            let a582 = 1u64;
            nat_add(a581, a582)?
        }, 0u64)?;
        h580
    };
    if f197(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f139() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f140(v0: Adt12) -> R<Adt15> {
    Ok(Adt15::C0(f198(match v0.clone() {
        Adt12::C0(h583, _, _, _, _) => {
            h583
        }
    }, 0u64)?, f198(match v0.clone() {
        Adt12::C0(_, h584, _, _, _) => {
            h584
        }
    }, 0u64)?, f198(match v0.clone() {
        Adt12::C0(_, _, h585, _, _) => {
            h585
        }
    }, 0u64)?, f198(match v0 {
        Adt12::C0(_, _, _, h586, _) => {
            h586
        }
    }, 0u64)?))
}

pub fn f141(v0: Adt12) -> List<Bytes> {
    f61(match v0.clone() {
        Adt12::C0(h587, _, _, _, _) => {
            h587
        }
    }, f61(match v0.clone() {
        Adt12::C0(_, _, h588, _, _) => {
            h588
        }
    }, f61(match v0.clone() {
        Adt12::C0(_, _, _, h589, _) => {
            h589
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h590) => {
            h590
        }
    })))
}

pub fn f142(v0: Adt15, p596: List<u64>, p597: List<Adt17>) -> Option<u64> {
    let m595 = (p596, p597);
    {
        let mut m595 = m595;
        loop {
            let step = {
                let (v1, v2) = m595;
                let m591 = v2;
                match m591.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt17>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f199(v0.clone(), v1.clone(), match v3.clone() {
                            Adt17::C0(_, h592) => {
                                h592
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt17>), Option<u64>>((List::cons(match v3 {
                                Adt17::C0(h593, _) => {
                                    h593
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt17>), Option<u64>>(Some(match v3 {
                                Adt17::C0(h594, _) => {
                                    h594
                                }
                            }))
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

pub fn f143(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt17>> {
    let m598 = v1;
    match m598.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt17::C0(v2, f200(v0.clone(), v3)?), f143(v0, v4, {
                let a599 = v2;
                let a600 = 1u64;
                nat_add(a599, a600)?
            })?))
        }
    }
}

pub fn f144(p604: List<Bytes>, p605: u64) -> Bytes {
    let m603 = (p604, p605);
    {
        let mut m603 = m603;
        loop {
            let step = {
                let (v0, v1) = m603;
                let m601 = v0;
                match m601.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m602 = v1;
                        if m602 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m602 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
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

pub fn f145(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f201(v0.clone(), v1.clone()) {
        if f201(v1, v0) {
            f202(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f146(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m606 = v0;
    match m606.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f203(v2, v1.clone())?, f146(v3, v1)?))
        }
    }
}

pub fn f147(v0: List<Adt14>) -> List<u64> {
    let m607 = v0;
    match m607.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f204(v1), f147(v2))
        }
    }
}

pub fn f148(p610: List<u64>, v1: List<u64>) -> Option<u64> {
    let m609 = p610;
    {
        let mut m609 = m609;
        loop {
            let step = {
                let v0 = m609;
                let m608 = v0;
                match m608.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f205(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m609 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m611 = v0;
    match m611.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m612 = f206(v2, v1.clone())?;
            match m612 {
                None => {
                    f149(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f149(v3, v1)?))
                }
            }
        }
    }
}

pub fn f150(v0: List<Bytes>) -> List<Bytes> {
    f133(Fn1::F207, List::<Bytes>::nil(), v0)
}

pub fn f151(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f208(v0, v1)
}

pub fn f152(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f209(f210(f211(v0))?))
}

pub fn f153(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m613 = f121(v0, v2)?;
    match m613 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f212(v1, f125(v3.clone(), f8(Str::lit("id"))))?;
            let m614 = v4.clone();
            match m614.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m616 = f130(f122({
                        let m615 = f122(v3.clone(), f8(Str::lit("source")));
                        match m615 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m616 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f72(f83(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f165(f213(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f154(p619: List<Bytes>, p620: List<Bytes>) -> List<Bytes> {
    let m618 = (p619, p620);
    {
        let mut m618 = m618;
        loop {
            let step = {
                let (v0, v1) = m618;
                let m617 = v0;
                match m617.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m618 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f214(v0, v1)
}

pub fn f156() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f157(v0: Adt1, p624: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m623 = p624;
    {
        let mut m623 = m623;
        loop {
            let step = {
                let v1 = m623;
                let m621 = v1;
                match m621.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f83(f215(f125({
                            let m622 = f121(v0.clone(), v3.clone())?;
                            match m622 {
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
                Ok(next) => { m623 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f72(f73(match v0.clone() {
        Adt1::C0(_, h625, _) => {
            h625
        }
    }, f114())) {
        Ok(Adt11::C1(List::cons(f114(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f81(f75(f216(f77(match v0.clone() {
            Adt1::C0(_, h626, _) => {
                h626
            }
        }, f114())))?, v1);
        let m627 = f57(v0.clone(), List::cons(f85(), List::cons(f95(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m627.uncons() {
            None => {
                let v5: (bool, Bytes) = f217(v0.clone(), v2)?;
                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                    let (h628, _) = v5;
                    if h628 {
                        let (_, h629) = v5;
                        h629
                    } else {
                        let a631 = {
                            let (_, h630) = v5;
                            h630
                        };
                        let a632 = f218(v0)?;
                        append_bytes(a631, a632)
                    }
                }))
            }
            Some((v3, v4)) => {
                Ok(Adt11::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f159(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f219(f70(v0, f85())?)?)))
}

pub fn f160(v0: List<Adt14>) -> List<Bytes> {
    let m633 = v0;
    match m633.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f125(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f125(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f125(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f160(v2))
        }
    }
}

pub fn f161(v0: List<Adt14>) -> List<Bytes> {
    let m634 = v0;
    match m634.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f125(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f220(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f161(v2))
        }
    }
}

pub fn f162(v0: Bytes, p692: u64, p693: u64, p694: Adt16) -> R<Adt16> {
    let m691 = (p692, (p693, p694));
    {
        let mut m691 = m691;
        loop {
            let step = {
                let (v1, (v2, v3)) = m691;
                let m635 = v2;
                if m635 == 0 {
                    Err::<(u64, (u64, Adt16)), R<Adt16>>(Ok(v3))
                } else {
                    let v4 = m635 - 1;
                    let v5: u64 = f221(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt16::C0(_, h636, _, _, _) => {
                            h636
                        }
                    } {
                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                            let a637 = v1;
                            let a638 = 1u64;
                            nat_add(a637, a638)?
                        }, (v4, Adt16::C0(true, false, match v3.clone() {
                            Adt16::C0(_, _, h639, _, _) => {
                                h639
                            }
                        }, match v3.clone() {
                            Adt16::C0(_, _, _, h640, _) => {
                                h640
                            }
                        }, match v3 {
                            Adt16::C0(_, _, _, _, h641) => {
                                h641
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt16::C0(h642, _, _, _, _) => {
                                h642
                            }
                        } {
                            let a643 = v5;
                            let a644 = 34u64;
                            if nat_eq(a643, a644) {
                                let a647 = f221(v0.clone(), {
                                    let a645 = v1;
                                    let a646 = 1u64;
                                    nat_add(a645, a646)?
                                })?;
                                let a648 = 34u64;
                                if nat_eq(a647, a648) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a649 = v1;
                                        let a650 = 1u64;
                                        nat_add(a649, a650)?
                                    }, (v4, Adt16::C0(true, true, f222(match v3.clone() {
                                        Adt16::C0(_, _, h651, _, _) => {
                                            h651
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt16::C0(_, _, _, h652, _) => {
                                            h652
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h653) => {
                                            h653
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a654 = v1;
                                        let a655 = 1u64;
                                        nat_add(a654, a655)?
                                    }, (v4, Adt16::C0(false, false, match v3.clone() {
                                        Adt16::C0(_, _, h656, _, _) => {
                                            h656
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h657, _) => {
                                            h657
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h658) => {
                                            h658
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a659 = v1;
                                    let a660 = 1u64;
                                    nat_add(a659, a660)?
                                }, (v4, Adt16::C0(true, false, f222(match v3.clone() {
                                    Adt16::C0(_, _, h661, _, _) => {
                                        h661
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt16::C0(_, _, _, h662, _) => {
                                        h662
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h663) => {
                                        h663
                                    }
                                }))))
                            }
                        } else {
                            let a664 = v5;
                            let a665 = 34u64;
                            if nat_eq(a664, a665) {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a666 = v1;
                                    let a667 = 1u64;
                                    nat_add(a666, a667)?
                                }, (v4, Adt16::C0(true, false, match v3.clone() {
                                    Adt16::C0(_, _, h668, _, _) => {
                                        h668
                                    }
                                }, match v3.clone() {
                                    Adt16::C0(_, _, _, h669, _) => {
                                        h669
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h670) => {
                                        h670
                                    }
                                }))))
                            } else {
                                let a671 = v5;
                                let a672 = 44u64;
                                if nat_eq(a671, a672) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a673 = v1;
                                        let a674 = 1u64;
                                        nat_add(a673, a674)?
                                    }, (v4, Adt16::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt16::C0(_, _, h675, _, _) => {
                                            h675
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h676, _) => {
                                            h676
                                        }
                                    }), match v3 {
                                        Adt16::C0(_, _, _, _, h677) => {
                                            h677
                                        }
                                    }))))
                                } else {
                                    let a678 = v5;
                                    let a679 = 13u64;
                                    if nat_eq(a678, a679) {
                                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                            let a680 = v1;
                                            let a681 = 1u64;
                                            nat_add(a680, a681)?
                                        }, (v4, v3)))
                                    } else {
                                        let a682 = v5;
                                        let a683 = 10u64;
                                        if nat_eq(a682, a683) {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a684 = v1;
                                                let a685 = 1u64;
                                                nat_add(a684, a685)?
                                            }, (v4, f163(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a686 = v1;
                                                let a687 = 1u64;
                                                nat_add(a686, a687)?
                                            }, (v4, Adt16::C0(false, false, f222(match v3.clone() {
                                                Adt16::C0(_, _, h688, _, _) => {
                                                    h688
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt16::C0(_, _, _, h689, _) => {
                                                    h689
                                                }
                                            }, match v3 {
                                                Adt16::C0(_, _, _, _, h690) => {
                                                    h690
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
                Ok(next) => { m691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: Adt16) -> Adt16 {
    Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f223(List::cons(match v0.clone() {
        Adt16::C0(_, _, h695, _, _) => {
            h695
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h696, _) => {
            h696
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt16::C0(_, _, _, _, h697) => {
            h697
        }
    }))
}

pub fn f164(p700: List<List<Bytes>>, p701: List<List<Bytes>>) -> List<List<Bytes>> {
    let m699 = (p700, p701);
    {
        let mut m699 = m699;
        loop {
            let step = {
                let (v0, v1) = m699;
                let m698 = v0;
                match m698.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m699 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f224(v0.clone(), 0u64, {
        let a702 = v0;
        length_bytes(a702)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f166(v0: Adt20) -> Bytes {
    let a723 = f225(match v0.clone() {
        Adt20::C0(h703, _, _, _, _, _, _, _) => {
            h703
        }
    });
    let a724 = {
        let a721 = f225(match v0.clone() {
            Adt20::C0(_, h704, _, _, _, _, _, _) => {
                h704
            }
        });
        let a722 = {
            let a719 = f225(match v0.clone() {
                Adt20::C0(_, _, h705, _, _, _, _, _) => {
                    h705
                }
            });
            let a720 = {
                let a717 = f225(match v0.clone() {
                    Adt20::C0(_, _, _, h706, _, _, _, _) => {
                        h706
                    }
                });
                let a718 = {
                    let a715 = f225(match v0.clone() {
                        Adt20::C0(_, _, _, _, h707, _, _, _) => {
                            h707
                        }
                    });
                    let a716 = {
                        let a713 = f225(match v0.clone() {
                            Adt20::C0(_, _, _, _, _, h708, _, _) => {
                                h708
                            }
                        });
                        let a714 = {
                            let a711 = f225(match v0.clone() {
                                Adt20::C0(_, _, _, _, _, _, h709, _) => {
                                    h709
                                }
                            });
                            let a712 = f225(match v0 {
                                Adt20::C0(_, _, _, _, _, _, _, h710) => {
                                    h710
                                }
                            });
                            append_bytes(a711, a712)
                        };
                        append_bytes(a713, a714)
                    };
                    append_bytes(a715, a716)
                };
                append_bytes(a717, a718)
            };
            append_bytes(a719, a720)
        };
        append_bytes(a721, a722)
    };
    append_bytes(a723, a724)
}

pub fn f167(v0: Bytes) -> R<Adt20> {
    let v1: u64 = {
        let a725 = v0.clone();
        length_bytes(a725)
    };
    let v2: u64 = {
        let a731 = {
            let a728 = {
                let a726 = v1;
                let a727 = 72u64;
                nat_add(a726, a727)?
            };
            let a729 = 64u64;
            let a730 = 0u64;
            nat_quot(a728, a729, a730)
        };
        let a732 = 64u64;
        nat_mul(a731, a732)?
    };
    let (_, h733) = f226(Fn4::F227(v0, v1, v2), {
        let a734 = v2;
        let a735 = 64u64;
        let a736 = 0u64;
        nat_quot(a734, a735, a736)
    }, (0u64, f228()))?;
    Ok(h733)
}

pub fn f168(p739: List<List<Bytes>>, v1: Bytes, p740: List<Bytes>) -> R<List<Bytes>> {
    let m738 = (p739, p740);
    {
        let mut m738 = m738;
        loop {
            let step = {
                let (v0, v2) = m738;
                let m737 = v0;
                match m737.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f229(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f170(f230(v3), Bytes::lit(&[0x0a])), v2)))
                        }
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

pub fn f169(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f231(v0)?))
}

pub fn f170(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m741 = v0;
    match m741.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m742 = v3.clone();
            match m742.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a745 = v2;
                    let a746 = {
                        let a743 = v1.clone();
                        let a744 = f170(v3, v1);
                        append_bytes(a743, a744)
                    };
                    append_bytes(a745, a746)
                }
            }
        }
    }
}

pub fn f171(v0: List<Bytes>) -> Adt14 {
    let m747 = v0;
    match m747.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f171(v2)))
        }
    }
}

pub fn f172(v0: Fn2, p751: Option<Bytes>, p752: List<Bytes>) -> Option<Bytes> {
    let m750 = (p751, p752);
    {
        let mut m750 = m750;
        loop {
            let step = {
                let (v1, v2) = m750;
                let m748 = v2;
                match m748.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c749: Fn2 = v0.clone();
                            c749.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m750 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m753 = v1;
    match m753 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a756 = {
                    let a754 = v3;
                    let a755 = v0;
                    append_bytes(a754, a755)
                };
                let a757 = v2;
                append_bytes(a756, a757)
            })
        }
    }
}

pub fn f174(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f231(v0.clone())?;
    let v2: u64 = f232(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a758 = v1;
        let a759 = v2;
        nat_sub(a758, a759)
    }))
}

pub fn f175(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f233(v0)?;
    let m763 = f234(v1.clone(), 0u64, {
        let a761 = {
            let a760 = v1.clone();
            length_bytes(a760)
        };
        let a762 = 2u64;
        nat_add(a761, a762)?
    }, 0u64)?;
    match m763 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a766 = {
                let (_, h764) = v2;
                h764
            };
            let a767 = {
                let a765 = v1;
                length_bytes(a765)
            };
            if nat_eq(a766, a767) {
                Ok(Some({
                    let (h768, _) = v2;
                    h768
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f176(v0: Bytes) -> Bytes {
    f7(List::cons(f74(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f177(v0: Option<List<Adt14>>) -> bool {
    let m769 = v0;
    match m769 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f178(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m770 = v0;
    match m770 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f179(v0: Adt14) -> Bytes {
    if f189(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f189(v0.clone(), f8(Str::lit("id"))) {
            if f189(v0.clone(), f8(Str::lit("statement"))) {
                if f189(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f190(v0, f8(Str::lit("evidence")))
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

pub fn f180(v0: List<Adt14>) -> Adt14 {
    let m771 = v0;
    match m771.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f180(v2)))
        }
    }
}

pub fn f181(v0: Adt14) -> bool {
    if f189(v0.clone(), f8(Str::lit("name"))) {
        if f190(v0.clone(), f8(Str::lit("schema"))) {
            let m772 = f183(f71(f126(f122(v0, f8(Str::lit("citation"))))));
            match m772 {
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

pub fn f182(v0: Bytes) -> R<Bytes> {
    f235(v0.clone(), 0u64, {
        let a773 = v0;
        length_bytes(a773)
    }, true, Bytes::lit(&[]))
}

pub fn f183(v0: List<Bytes>) -> Option<Adt14> {
    let m774 = v0;
    match m774.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m775 = v2;
                match m775.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m776 = v2;
                    match m776.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m777 = v6;
                                match m777.uncons() {
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
                        let m778 = v2;
                        match m778.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m779 = v10;
                                match m779.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f171(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f184(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m780 = f236(4096u64, f237(f238(f239(f122(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt21::C0(v2, v0), List::<Adt21>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m780 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f170(f154(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f185(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    let m781 = v0;
    match m781.uncons() {
        None => {
            List::cons(v1, List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            List::cons(v2, f185(v3, v1))
        }
    }
}

pub fn f186(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f170(f240(f80(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f187(v0: Adt14) -> R<Bytes> {
    let m782 = v0;
    match m782 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f241(v2, v3)
        }
        Adt14::C2(v4) => {
            f242(v4)
        }
        Adt14::C3(r783) => {
            let v5 = (*r783).clone();
            Ok(f243(f244(v5)?))
        }
        Adt14::C4(r784) => {
            let v6 = (*r784).clone();
            f245(f246(v6)?)
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

pub fn f188(v0: Adt14) -> List<Adt14> {
    let m785 = v0;
    match m785 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r786) => {
            let v3 = (*r786).clone();
            f188(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r787, r788) => {
            let v1 = (*r787).clone();
            let v2 = (*r788).clone();
            List::cons(v1, f188(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f189(v0: Adt14, v1: Bytes) -> bool {
    f72({
        let a790 = {
            let a789 = f125(v0, v1);
            length_bytes(a789)
        };
        let a791 = 0u64;
        nat_eq(a790, a791)
    })
}

pub fn f190(v0: Adt14, v1: Bytes) -> bool {
    let m792 = f122(v0, v1);
    match m792 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f191(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m793 = f122(v0, v1);
    match m793 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f126(f122(v3, v2))
        }
    }
}

pub fn f192(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f122(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f104(Str::lit("id"), f125(v0.clone(), f8(Str::lit("id"))))?, List::cons(f104(Str::lit("name"), f125(v0.clone(), f8(Str::lit("name"))))?, List::cons(f104(Str::lit("issuer"), f125(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f104(Str::lit("edition"), f125(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f104(Str::lit("canonical_id"), f125(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f104(Str::lit("source_role"), f125(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f104(Str::lit("retrieval_date"), f125(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f104(Str::lit("url"), f247(v2.clone(), f8(Str::lit("url"))))?, List::cons(f104(Str::lit("media_type"), f247(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f104(Str::lit("license"), f247(v2.clone(), f8(Str::lit("license"))))?, List::cons(f104(Str::lit("redistribution"), f247(v2, f8(Str::lit("redistribution"))))?, List::cons(f104(Str::lit("statement"), f125(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m794 = v1;
        match m794 {
            None => {
                f7(List::cons(f104(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f104(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f104(Str::lit("artifact"), {
                    let (h795, _) = v3;
                    h795
                })?, List::cons(f104(Str::lit("checksum"), {
                    let (_, h796) = v3;
                    h796
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f248(f71(f126(f122(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f193(v0: Bytes, v1: u64) -> Option<u8> {
    let a797 = v0;
    let a798 = v1;
    index_bytes(a797, a798)
}

pub fn f194(v0: Bytes) -> bool {
    f110(v0, f8(Str::lit(".lex.tex")))
}

pub fn f195(v0: Fn3, p803: u64, p804: u64) -> R<(u64, bool)> {
    let m802 = (p803, p804);
    {
        let mut m802 = m802;
        loop {
            let step = {
                let (v1, v2) = m802;
                let m799 = v1;
                if m799 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m799 - 1;
                    let m801 = {
                        let c800: Fn3 = v0.clone();
                        c800.apply(v2)?
                    };
                    match m801 {
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
                Ok(next) => { m802 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f197(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a807 = {
            let a805 = v4;
            let a806 = v1;
            nat_add(a805, a806)?
        };
        let a808 = v3;
        if nat_lt(a807, a808) {
            Ok(Some({
                let a809 = v4;
                let a810 = 1u64;
                nat_add(a809, a810)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f197(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m815 = {
        let a812 = v0;
        let a813 = v2;
        let a814 = {
            let a811 = v1.clone();
            length_bytes(a811)
        };
        slice_bytes(a812, a813, a814)
    };
    match m815 {
        None => {
            false
        }
        Some(v3) => {
            let a816 = v3;
            let a817 = v1;
            equal(a816, a817)
        }
    }
}

pub fn f198(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m818 = v0;
    match m818.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f198(v3, {
                let a819 = v1;
                let a820 = 1u64;
                nat_add(a819, a820)?
            })?))
        }
    }
}

pub fn f199(v0: Adt15, v1: List<u64>, v2: Adt18) -> bool {
    let m821 = v2;
    match m821 {
        Adt18::C0(v3, v4) => {
            if f205(v3, match v0.clone() {
                Adt15::C0(h822, _, _, _) => {
                    h822
                }
            }) {
                f249(v0, v4)
            } else {
                false
            }
        }
        Adt18::C1(v5) => {
            f205(v5, match v0 {
                Adt15::C0(_, _, h823, _) => {
                    h823
                }
            })
        }
        Adt18::C2(_, v7, v8) => {
            if f201(v7, v1) {
                f205(v8, match v0 {
                    Adt15::C0(_, _, _, h824) => {
                        h824
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f200(v0: Adt12, v1: Adt14) -> R<Adt18> {
    let m825 = f122(v1.clone(), f8(Str::lit("std")));
    match m825 {
        None => {
            let m830 = f130(f122(v1.clone(), f8(Str::lit("req"))));
            match m830 {
                None => {
                    let m832 = f122(v1, f8(Str::lit("der")));
                    match m832 {
                        None => {
                            Ok(f250(v0))
                        }
                        Some(v5) => {
                            let m833 = f251(f125(v5.clone(), f8(Str::lit("rule"))));
                            match m833 {
                                None => {
                                    Ok(f250(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt18::C2(v6, f146(f71(f126(f122(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h834) => {
                                            h834
                                        }
                                    })?, f203(f125(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h835, _) => {
                                            h835
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt18::C1(f203(v4, match v0 {
                        Adt12::C0(_, _, h831, _, _) => {
                            h831
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m826 = f130(f122(v2.clone(), f8(Str::lit("row"))));
            match m826 {
                None => {
                    Ok(Adt18::C0(f203(f125(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h827, _, _, _, _) => {
                            h827
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt18::C0(f203(f125(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h828, _, _, _, _) => {
                            h828
                        }
                    })?, Some(f203(v3, match v0 {
                        Adt12::C0(_, h829, _, _, _) => {
                            h829
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f201(p838: List<u64>, v1: List<u64>) -> bool {
    let m837 = p838;
    {
        let mut m837 = m837;
        loop {
            let step = {
                let v0 = m837;
                let m836 = v0;
                match m836.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f205(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m837 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(p843: List<u64>) -> bool {
    let m842 = p843;
    {
        let mut m842 = m842;
        loop {
            let step = {
                let v0 = m842;
                let m839 = v0;
                match m839.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a840 = v1;
                        let a841 = 7u64;
                        if nat_le(a840, a841) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m844 = f252(v0, v1.clone(), 0u64)?;
    match m844 {
        None => {
            let a845 = v1;
            Ok(length_list(a845))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f204(v0: Adt14) -> u64 {
    let m846 = f253(f122(v0, f8(Str::lit("stratum"))));
    match m846 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f205(v0: u64, p851: List<u64>) -> bool {
    let m850 = p851;
    {
        let mut m850 = m850;
        loop {
            let step = {
                let v1 = m850;
                let m847 = v1;
                match m847.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a848 = v2;
                        let a849 = v0;
                        if nat_eq(a848, a849) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m850 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f254(v0.clone(), v1) {
        let m852 = f30(v0, 34u8)?;
        match m852.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m853 = v3;
                match m853.uncons() {
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

pub fn f207(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f208(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m854 = v0.clone();
    match m854.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a855 = v2;
            let a856 = v1.clone();
            if equal(a855, a856) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f209(v0: List<(Bytes, ())>) -> List<Bytes> {
    f150(f255(Fn5::F256, List::<Bytes>::nil(), v0))
}

pub fn f210(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f257(f258(Fn6::F259, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m861 = {
        let (h857, _) = f260(Fn7::F261, {
            let a859 = {
                let a858 = v0;
                length_list(a858)
            };
            let a860 = 1u64;
            nat_add(a859, a860)?
        }, v1)?;
        h857
    };
    match m861.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f211(v0: List<Bytes>) -> List<(Bytes, ())> {
    f262(f263(Fn8::F264, List::<(Bytes, ())>::nil(), v0))
}

pub fn f212(p864: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m863 = p864;
    {
        let mut m863 = m863;
        loop {
            let step = {
                let v0 = m863;
                let m862 = v0;
                match m862.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f229(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f247(f122(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f214(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h865, _) = v1.clone();
        h865
    };
    let v3: u64 = {
        let (h866, _) = {
            let (_, h867) = v1.clone();
            h867
        };
        h866
    };
    let v4: List<Bytes> = {
        let (_, h868) = {
            let (_, h869) = v1;
            h869
        };
        h868
    };
    let m870 = f193(v0.clone(), v2);
    match m870 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a871 = v5;
            let a872 = 10u8;
            if equal(a871, a872) {
                Ok(Some(({
                    let a873 = v2;
                    let a874 = 1u64;
                    nat_add(a873, a874)?
                }, ({
                    let a875 = v2;
                    let a876 = 1u64;
                    nat_add(a875, a876)?
                }, List::cons(f265(f32(v0, v3, {
                    let a877 = v2;
                    let a878 = v3;
                    nat_sub(a877, a878)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a879 = v2;
                    let a880 = 1u64;
                    nat_add(a879, a880)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f215(v0: Bytes) -> R<Bytes> {
    let a881 = f8(Str::lit("Catalog."));
    let a882 = f182(v0)?;
    Ok(append_bytes(a881, a882))
}

pub fn f216(v0: List<Bytes>) -> List<Bytes> {
    let m883 = v0;
    match m883.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f110(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f113(f114(), v1), f216(v2))
            } else {
                f216(v2)
            }
        }
    }
}

pub fn f217(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m884 = v1;
    match m884.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f266(v0.clone(), v2)?;
            let (h885, _) = v4.clone();
            if h885 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f217(v0, v3)?;
                let (h886, _) = v5.clone();
                if h886 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a889 = {
                            let (_, h887) = v4;
                            h887
                        };
                        let a890 = {
                            let (_, h888) = v5;
                            h888
                        };
                        append_bytes(a889, a890)
                    }))
                }
            }
        }
    }
}

pub fn f218(v0: Adt1) -> R<Bytes> {
    let a892 = {
        let a891 = f96(f97(f98(f99(f67(v0.clone(), f95()))?)), f100(f70(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a891)
    };
    let a893 = 0u64;
    if nat_eq(a892, a893) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f219(v0: List<Adt14>) -> R<List<Bytes>> {
    let m894 = v0;
    match m894.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f108(f71(f126(f122(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f125(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f125(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f267(f80(f125(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f219(v2)?))
        }
    }
}

pub fn f220(v0: Adt14) -> Bytes {
    if f189(v0.clone(), f8(Str::lit("req"))) {
        let a895 = f8(Str::lit("req "));
        let a896 = f125(v0, f8(Str::lit("req")));
        append_bytes(a895, a896)
    } else {
        let m897 = f122(v0.clone(), f8(Str::lit("std")));
        match m897 {
            None => {
                let m900 = f122(v0, f8(Str::lit("der")));
                match m900 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f125(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f108(f71(f126(f122(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f125(v1.clone(), f8(Str::lit("authority"))), List::cons(if f189(v1.clone(), f8(Str::lit("row"))) {
                    let a898 = f8(Str::lit(" row "));
                    let a899 = f125(v1, f8(Str::lit("row")));
                    append_bytes(a898, a899)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: u64) -> R<u64> {
    let m903 = {
        let a901 = v0;
        let a902 = v1;
        index_bytes(a901, a902)
    };
    match m903 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a948 = {
                let a924 = {
                    let a912 = {
                        let a906 = {
                            let a904 = v3;
                            let a905 = 128u8;
                            fixed_u8::bit_and(a904, a905)
                        };
                        let a907 = 128u8;
                        if equal(a906, a907) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a913 = {
                        let a910 = {
                            let a908 = v3;
                            let a909 = 64u8;
                            fixed_u8::bit_and(a908, a909)
                        };
                        let a911 = 64u8;
                        if equal(a910, a911) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a912, a913)?
                };
                let a925 = {
                    let a922 = {
                        let a916 = {
                            let a914 = v3;
                            let a915 = 32u8;
                            fixed_u8::bit_and(a914, a915)
                        };
                        let a917 = 32u8;
                        if equal(a916, a917) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a923 = {
                        let a920 = {
                            let a918 = v3;
                            let a919 = 16u8;
                            fixed_u8::bit_and(a918, a919)
                        };
                        let a921 = 16u8;
                        if equal(a920, a921) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a922, a923)?
                };
                nat_add(a924, a925)?
            };
            let a949 = {
                let a946 = {
                    let a934 = {
                        let a928 = {
                            let a926 = v3;
                            let a927 = 8u8;
                            fixed_u8::bit_and(a926, a927)
                        };
                        let a929 = 8u8;
                        if equal(a928, a929) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a935 = {
                        let a932 = {
                            let a930 = v3;
                            let a931 = 4u8;
                            fixed_u8::bit_and(a930, a931)
                        };
                        let a933 = 4u8;
                        if equal(a932, a933) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a934, a935)?
                };
                let a947 = {
                    let a944 = {
                        let a938 = {
                            let a936 = v3;
                            let a937 = 2u8;
                            fixed_u8::bit_and(a936, a937)
                        };
                        let a939 = 2u8;
                        if equal(a938, a939) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a945 = {
                        let a942 = {
                            let a940 = v3;
                            let a941 = 1u8;
                            fixed_u8::bit_and(a940, a941)
                        };
                        let a943 = 1u8;
                        if equal(a942, a943) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a944, a945)?
                };
                nat_add(a946, a947)?
            };
            nat_add(a948, a949)
        }
    }
}

pub fn f222(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m953 = {
        let a950 = v1;
        let a951 = v2;
        let a952 = 1u64;
        slice_bytes(a950, a951, a952)
    };
    match m953 {
        None => {
            v0
        }
        Some(v3) => {
            let a954 = v0;
            let a955 = v3;
            append_bytes(a954, a955)
        }
    }
}

pub fn f223(p958: List<Bytes>, p959: List<Bytes>) -> List<Bytes> {
    let m957 = (p958, p959);
    {
        let mut m957 = m957;
        loop {
            let step = {
                let (v0, v1) = m957;
                let m956 = v0;
                match m956.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f224(v0: Bytes, p1031: u64, p1032: u64, p1033: Bytes) -> R<Bytes> {
    let m1030 = (p1031, (p1032, p1033));
    {
        let mut m1030 = m1030;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1030;
                let m960 = v2;
                if m960 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m960 - 1;
                    let m963 = {
                        let a961 = v0.clone();
                        let a962 = v1;
                        index_bytes(a961, a962)
                    };
                    match m963 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1008 = {
                                    let a984 = {
                                        let a972 = {
                                            let a966 = {
                                                let a964 = v6;
                                                let a965 = 128u8;
                                                fixed_u8::bit_and(a964, a965)
                                            };
                                            let a967 = 128u8;
                                            if equal(a966, a967) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a973 = {
                                            let a970 = {
                                                let a968 = v6;
                                                let a969 = 64u8;
                                                fixed_u8::bit_and(a968, a969)
                                            };
                                            let a971 = 64u8;
                                            if equal(a970, a971) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a972, a973)?
                                    };
                                    let a985 = {
                                        let a982 = {
                                            let a976 = {
                                                let a974 = v6;
                                                let a975 = 32u8;
                                                fixed_u8::bit_and(a974, a975)
                                            };
                                            let a977 = 32u8;
                                            if equal(a976, a977) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a983 = {
                                            let a980 = {
                                                let a978 = v6;
                                                let a979 = 16u8;
                                                fixed_u8::bit_and(a978, a979)
                                            };
                                            let a981 = 16u8;
                                            if equal(a980, a981) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a982, a983)?
                                    };
                                    nat_add(a984, a985)?
                                };
                                let a1009 = {
                                    let a1006 = {
                                        let a994 = {
                                            let a988 = {
                                                let a986 = v6;
                                                let a987 = 8u8;
                                                fixed_u8::bit_and(a986, a987)
                                            };
                                            let a989 = 8u8;
                                            if equal(a988, a989) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a995 = {
                                            let a992 = {
                                                let a990 = v6;
                                                let a991 = 4u8;
                                                fixed_u8::bit_and(a990, a991)
                                            };
                                            let a993 = 4u8;
                                            if equal(a992, a993) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a994, a995)?
                                    };
                                    let a1007 = {
                                        let a1004 = {
                                            let a998 = {
                                                let a996 = v6;
                                                let a997 = 2u8;
                                                fixed_u8::bit_and(a996, a997)
                                            };
                                            let a999 = 2u8;
                                            if equal(a998, a999) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1005 = {
                                            let a1002 = {
                                                let a1000 = v6;
                                                let a1001 = 1u8;
                                                fixed_u8::bit_and(a1000, a1001)
                                            };
                                            let a1003 = 1u8;
                                            if equal(a1002, a1003) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1004, a1005)?
                                    };
                                    nat_add(a1006, a1007)?
                                };
                                nat_add(a1008, a1009)?
                            };
                            let a1010 = v7;
                            let a1011 = 34u64;
                            if if nat_eq(a1010, a1011) {
                                true
                            } else {
                                let a1012 = v7;
                                let a1013 = 92u64;
                                nat_eq(a1012, a1013)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1014 = v1;
                                    let a1015 = 1u64;
                                    nat_add(a1014, a1015)?
                                }, (v4, {
                                    let a1018 = v3;
                                    let a1019 = {
                                        let a1016 = Bytes::lit(&[0x5c]);
                                        let a1017 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1016, a1017)
                                    };
                                    append_bytes(a1018, a1019)
                                })))
                            } else {
                                let a1020 = v7;
                                let a1021 = 10u64;
                                if nat_eq(a1020, a1021) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1022 = v1;
                                        let a1023 = 1u64;
                                        nat_add(a1022, a1023)?
                                    }, (v4, {
                                        let a1024 = v3;
                                        let a1025 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1024, a1025)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1026 = v1;
                                        let a1027 = 1u64;
                                        nat_add(a1026, a1027)?
                                    }, (v4, {
                                        let a1028 = v3;
                                        let a1029 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1028, a1029)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1030 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: u32) -> Bytes {
    let a1062 = f268({
        let a1034 = f269(v0, 28u32);
        let a1035 = 15u32;
        fixed_u32::bit_and(a1034, a1035)
    });
    let a1063 = {
        let a1060 = f268({
            let a1036 = f269(v0, 24u32);
            let a1037 = 15u32;
            fixed_u32::bit_and(a1036, a1037)
        });
        let a1061 = {
            let a1058 = f268({
                let a1038 = f269(v0, 20u32);
                let a1039 = 15u32;
                fixed_u32::bit_and(a1038, a1039)
            });
            let a1059 = {
                let a1056 = f268({
                    let a1040 = f269(v0, 16u32);
                    let a1041 = 15u32;
                    fixed_u32::bit_and(a1040, a1041)
                });
                let a1057 = {
                    let a1054 = f268({
                        let a1042 = f269(v0, 12u32);
                        let a1043 = 15u32;
                        fixed_u32::bit_and(a1042, a1043)
                    });
                    let a1055 = {
                        let a1052 = f268({
                            let a1044 = f269(v0, 8u32);
                            let a1045 = 15u32;
                            fixed_u32::bit_and(a1044, a1045)
                        });
                        let a1053 = {
                            let a1050 = f268({
                                let a1046 = f269(v0, 4u32);
                                let a1047 = 15u32;
                                fixed_u32::bit_and(a1046, a1047)
                            });
                            let a1051 = f268({
                                let a1048 = v0;
                                let a1049 = 15u32;
                                fixed_u32::bit_and(a1048, a1049)
                            });
                            append_bytes(a1050, a1051)
                        };
                        append_bytes(a1052, a1053)
                    };
                    append_bytes(a1054, a1055)
                };
                append_bytes(a1056, a1057)
            };
            append_bytes(a1058, a1059)
        };
        append_bytes(a1060, a1061)
    };
    append_bytes(a1062, a1063)
}

pub fn f226(v0: Fn4, p1067: u64, p1068: (u64, Adt20)) -> R<(u64, Adt20)> {
    let m1066 = (p1067, p1068);
    {
        let mut m1066 = m1066;
        loop {
            let step = {
                let (v1, v2) = m1066;
                let m1064 = v1;
                if m1064 == 0 {
                    Err::<(u64, (u64, Adt20)), R<(u64, Adt20)>>(Ok(v2))
                } else {
                    let v3 = m1064 - 1;
                    Ok::<(u64, (u64, Adt20)), R<(u64, Adt20)>>((v3, {
                        let c1065: Fn4 = v0.clone();
                        c1065.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt20)) -> R<(u64, Adt20)> {
    Ok(({
        let a1070 = {
            let (h1069, _) = v3.clone();
            h1069
        };
        let a1071 = 64u64;
        nat_add(a1070, a1071)?
    }, f270({
        let (_, h1072) = v3;
        h1072
    }, f271(v0, v1, v2, {
        let (h1073, _) = v3;
        h1073
    })?)?))
}

pub fn f228() -> Adt20 {
    Adt20::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f229(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f83(v1, f66(f170(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f230(v0: List<Bytes>) -> List<Bytes> {
    let m1074 = v0;
    match m1074.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f230(v2)
            } else {
                List::cons(v1, f230(v2))
            }
        }
    }
}

pub fn f231(v0: Bytes) -> R<u64> {
    let (h1075, _) = f195(Fn3::F272(v0.clone()), {
        let a1076 = v0.clone();
        length_bytes(a1076)
    }, {
        let a1077 = v0;
        length_bytes(a1077)
    })?;
    Ok(h1075)
}

pub fn f232(v0: Bytes, v1: u64) -> R<u64> {
    let (h1078, _) = f195(Fn3::F273(v1, v0), v1, 0u64)?;
    Ok(h1078)
}

pub fn f233(v0: Bytes) -> R<Bytes> {
    f274(v0.clone(), 0u64, {
        let a1079 = v0;
        length_bytes(a1079)
    }, false, false, Bytes::lit(&[]))
}

pub fn f234(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1080 = v2;
    if m1080 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1080 - 1;
        let v5: u64 = f275(v0.clone(), v1)?;
        let a1081 = v3;
        let a1082 = 1u64;
        if nat_eq(a1081, a1082) {
            let m1083 = f234(v0.clone(), v1, v4, 0u64)?;
            match m1083 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f275(v0.clone(), {
                        let (_, h1084) = v6.clone();
                        h1084
                    })?;
                    let a1085 = v7;
                    let a1086 = 44u64;
                    if nat_eq(a1085, a1086) {
                        let m1090 = f234(v0, {
                            let a1088 = {
                                let (_, h1087) = v6;
                                h1087
                            };
                            let a1089 = 1u64;
                            nat_add(a1088, a1089)?
                        }, v4, 1u64)?;
                        match m1090 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1091, _) = v6;
                                    h1091
                                }), std::rc::Rc::new({
                                    let (h1092, _) = v8;
                                    h1092
                                })), {
                                    let (_, h1093) = v8;
                                    h1093
                                })))
                            }
                        }
                    } else {
                        let a1094 = v7;
                        let a1095 = 93u64;
                        if nat_eq(a1094, a1095) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1096, _) = v6;
                                h1096
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1098 = {
                                    let (_, h1097) = v6;
                                    h1097
                                };
                                let a1099 = 1u64;
                                nat_add(a1098, a1099)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1100 = v3;
            let a1101 = 2u64;
            if nat_eq(a1100, a1101) {
                let m1104 = f276(v0.clone(), {
                    let a1102 = v1;
                    let a1103 = 1u64;
                    nat_add(a1102, a1103)?
                })?;
                match m1104 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1105 = v5;
                        let a1106 = 34u64;
                        if if nat_eq(a1105, a1106) {
                            let a1108 = f275(v0.clone(), {
                                let (h1107, _) = v9.clone();
                                h1107
                            })?;
                            let a1109 = 58u64;
                            nat_eq(a1108, a1109)
                        } else {
                            false
                        } {
                            let m1113 = f234(v0.clone(), {
                                let a1111 = {
                                    let (h1110, _) = v9;
                                    h1110
                                };
                                let a1112 = 1u64;
                                nat_add(a1111, a1112)?
                            }, v4, 0u64)?;
                            match m1113 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f275(v0.clone(), {
                                        let (_, h1114) = v10.clone();
                                        h1114
                                    })?;
                                    let a1115 = v11;
                                    let a1116 = 44u64;
                                    if nat_eq(a1115, a1116) {
                                        let m1120 = f234(v0, {
                                            let a1118 = {
                                                let (_, h1117) = v10;
                                                h1117
                                            };
                                            let a1119 = 1u64;
                                            nat_add(a1118, a1119)?
                                        }, v4, 2u64)?;
                                        match m1120 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1121) = v9;
                                                    h1121
                                                }, std::rc::Rc::new({
                                                    let (h1122, _) = v10;
                                                    h1122
                                                }), std::rc::Rc::new({
                                                    let (h1123, _) = v12;
                                                    h1123
                                                })), {
                                                    let (_, h1124) = v12;
                                                    h1124
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1125 = v11;
                                        let a1126 = 125u64;
                                        if nat_eq(a1125, a1126) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1127) = v9;
                                                h1127
                                            }, std::rc::Rc::new({
                                                let (h1128, _) = v10;
                                                h1128
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1130 = {
                                                    let (_, h1129) = v10;
                                                    h1129
                                                };
                                                let a1131 = 1u64;
                                                nat_add(a1130, a1131)?
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
                let a1132 = v5;
                let a1133 = 34u64;
                if nat_eq(a1132, a1133) {
                    let m1136 = f276(v0, {
                        let a1134 = v1;
                        let a1135 = 1u64;
                        nat_add(a1134, a1135)?
                    })?;
                    match m1136 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1137) = v13;
                                h1137
                            }), {
                                let (h1138, _) = v13;
                                h1138
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1139 = v1;
                            let a1140 = 4u64;
                            nat_add(a1139, a1140)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1141 = v1;
                                let a1142 = 5u64;
                                nat_add(a1141, a1142)?
                            })))
                        } else {
                            let a1143 = v5;
                            let a1144 = 91u64;
                            if nat_eq(a1143, a1144) {
                                let a1147 = f275(v0.clone(), {
                                    let a1145 = v1;
                                    let a1146 = 1u64;
                                    nat_add(a1145, a1146)?
                                })?;
                                let a1148 = 93u64;
                                if nat_eq(a1147, a1148) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1149 = v1;
                                        let a1150 = 2u64;
                                        nat_add(a1149, a1150)?
                                    })))
                                } else {
                                    let m1153 = f234(v0, {
                                        let a1151 = v1;
                                        let a1152 = 1u64;
                                        nat_add(a1151, a1152)?
                                    }, v4, 1u64)?;
                                    match m1153 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1154, _) = v14;
                                                h1154
                                            })), {
                                                let (_, h1155) = v14;
                                                h1155
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1156 = v5;
                                let a1157 = 123u64;
                                if nat_eq(a1156, a1157) {
                                    let a1160 = f275(v0.clone(), {
                                        let a1158 = v1;
                                        let a1159 = 1u64;
                                        nat_add(a1158, a1159)?
                                    })?;
                                    let a1161 = 125u64;
                                    if nat_eq(a1160, a1161) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1162 = v1;
                                            let a1163 = 2u64;
                                            nat_add(a1162, a1163)?
                                        })))
                                    } else {
                                        let m1166 = f234(v0, {
                                            let a1164 = v1;
                                            let a1165 = 1u64;
                                            nat_add(a1164, a1165)?
                                        }, v4, 2u64)?;
                                        match m1166 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1167, _) = v15;
                                                    h1167
                                                })), {
                                                    let (_, h1168) = v15;
                                                    h1168
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1169 = f277(v0, v1)?;
                                    match m1169 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1170) = v16;
                                                h1170
                                            }, {
                                                let (h1171, _) = v16;
                                                h1171
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

pub fn f235(v0: Bytes, p1238: u64, p1239: u64, p1240: bool, p1241: Bytes) -> R<Bytes> {
    let m1237 = (p1238, (p1239, (p1240, p1241)));
    {
        let mut m1237 = m1237;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1237;
                let m1172 = v2;
                if m1172 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1172 - 1;
                    let m1175 = {
                        let a1173 = v0.clone();
                        let a1174 = v1;
                        index_bytes(a1173, a1174)
                    };
                    match m1175 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1220 = {
                                    let a1196 = {
                                        let a1184 = {
                                            let a1178 = {
                                                let a1176 = v7;
                                                let a1177 = 128u8;
                                                fixed_u8::bit_and(a1176, a1177)
                                            };
                                            let a1179 = 128u8;
                                            if equal(a1178, a1179) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1185 = {
                                            let a1182 = {
                                                let a1180 = v7;
                                                let a1181 = 64u8;
                                                fixed_u8::bit_and(a1180, a1181)
                                            };
                                            let a1183 = 64u8;
                                            if equal(a1182, a1183) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1184, a1185)?
                                    };
                                    let a1197 = {
                                        let a1194 = {
                                            let a1188 = {
                                                let a1186 = v7;
                                                let a1187 = 32u8;
                                                fixed_u8::bit_and(a1186, a1187)
                                            };
                                            let a1189 = 32u8;
                                            if equal(a1188, a1189) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1195 = {
                                            let a1192 = {
                                                let a1190 = v7;
                                                let a1191 = 16u8;
                                                fixed_u8::bit_and(a1190, a1191)
                                            };
                                            let a1193 = 16u8;
                                            if equal(a1192, a1193) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1194, a1195)?
                                    };
                                    nat_add(a1196, a1197)?
                                };
                                let a1221 = {
                                    let a1218 = {
                                        let a1206 = {
                                            let a1200 = {
                                                let a1198 = v7;
                                                let a1199 = 8u8;
                                                fixed_u8::bit_and(a1198, a1199)
                                            };
                                            let a1201 = 8u8;
                                            if equal(a1200, a1201) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1207 = {
                                            let a1204 = {
                                                let a1202 = v7;
                                                let a1203 = 4u8;
                                                fixed_u8::bit_and(a1202, a1203)
                                            };
                                            let a1205 = 4u8;
                                            if equal(a1204, a1205) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1206, a1207)?
                                    };
                                    let a1219 = {
                                        let a1216 = {
                                            let a1210 = {
                                                let a1208 = v7;
                                                let a1209 = 2u8;
                                                fixed_u8::bit_and(a1208, a1209)
                                            };
                                            let a1211 = 2u8;
                                            if equal(a1210, a1211) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1217 = {
                                            let a1214 = {
                                                let a1212 = v7;
                                                let a1213 = 1u8;
                                                fixed_u8::bit_and(a1212, a1213)
                                            };
                                            let a1215 = 1u8;
                                            if equal(a1214, a1215) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1216, a1217)?
                                    };
                                    nat_add(a1218, a1219)?
                                };
                                nat_add(a1220, a1221)?
                            };
                            if f278(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1222 = v1;
                                    let a1223 = 1u64;
                                    nat_add(a1222, a1223)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f279(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1224 = v1;
                                        let a1225 = 1u64;
                                        nat_add(a1224, a1225)?
                                    }, (v5, (false, {
                                        let a1229 = v4;
                                        let a1230 = f32({
                                            let a1226 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1226)
                                        }, {
                                            let a1227 = v8;
                                            let a1228 = 97u64;
                                            nat_sub(a1227, a1228)
                                        }, 1u64);
                                        append_bytes(a1229, a1230)
                                    }))))
                                } else {
                                    if if f279(v8) {
                                        true
                                    } else {
                                        if f280(v8) {
                                            true
                                        } else {
                                            f281(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1231 = v1;
                                            let a1232 = 1u64;
                                            nat_add(a1231, a1232)?
                                        }, (v5, (false, {
                                            let a1233 = v4;
                                            let a1234 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1233, a1234)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1235 = v1;
                                            let a1236 = 1u64;
                                            nat_add(a1235, a1236)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(p1249: u64, p1250: List<Adt21>, p1251: List<Bytes>, p1252: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1248 = (p1249, (p1250, (p1251, p1252)));
    {
        let mut m1248 = m1248;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1248;
                let m1242 = v0;
                if m1242 == 0 {
                    Err::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1242 - 1;
                    let m1243 = v1;
                    match m1243.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1244 = v5;
                            match m1244 {
                                Adt21::C0(v8, v9) => {
                                    if f83(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m1245 = f282(v8.clone(), v9)?;
                                        match m1245 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f237({
                                                    let (_, h1246) = v11;
                                                    h1246
                                                }, List::cons(Adt21::C1({
                                                    let (h1247, _) = v11;
                                                    h1247
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
                Ok(next) => { m1248 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: List<Adt21>, v1: List<Adt21>) -> List<Adt21> {
    let m1253 = v0;
    match m1253.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f237(v3, v1))
        }
    }
}

pub fn f238(v0: List<(Bytes, Adt14)>) -> R<List<Adt21>> {
    let m1254 = v0;
    match m1254.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt21::C0(f182({
                let (h1255, _) = v1;
                h1255
            })?, {
                let (_, h1256) = v1;
                h1256
            }), f238(v2)?))
        }
    }
}

pub fn f239(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1257 = v0;
    match m1257 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f283(v1)
        }
    }
}

pub fn f240(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1258 = v0;
    match m1258.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f254(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f170(f284(f75(List::cons(v1, f285(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f240(v3, v1)?))
            }
        }
    }
}

pub fn f241(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1259 = 0u64;
        let a1260 = v1;
        nat_lt(a1259, a1260)
    } else {
        false
    } {
        let a1261 = Bytes::lit(&[0x2d]);
        let a1262 = f286(v1)?;
        Ok(append_bytes(a1261, a1262))
    } else {
        f286(v1)
    }
}

pub fn f242(v0: Bytes) -> R<Bytes> {
    let a1268 = {
        let a1266 = Bytes::lit(&[0x22]);
        let a1267 = {
            let (_, h1263) = {
                let (h1264, _) = f287(Fn9::F288(v0.clone()), {
                    let a1265 = v0;
                    length_bytes(a1265)
                }, (0u64, Bytes::lit(&[])))?;
                h1264
            };
            h1263
        };
        append_bytes(a1266, a1267)
    };
    let a1269 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1268, a1269))
}

pub fn f243(v0: List<Bytes>) -> Bytes {
    let a1272 = {
        let a1270 = Bytes::lit(&[0x5b]);
        let a1271 = f108(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1270, a1271)
    };
    let a1273 = Bytes::lit(&[0x5d]);
    append_bytes(a1272, a1273)
}

pub fn f244(v0: Adt14) -> R<List<Bytes>> {
    let m1274 = v0;
    match m1274 {
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
        Adt14::C6(r1275, r1276) => {
            let v7 = (*r1275).clone();
            let v8 = (*r1276).clone();
            Ok(List::cons(f187(v7)?, f244(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f245(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f289(Fn10::F290, List::<(Bytes, Bytes)>::nil(), f291(v0)?);
    let v2: List<Bytes> = f292(Fn11::F293, List::<Bytes>::nil(), v1)?;
    let a1279 = {
        let a1277 = Bytes::lit(&[0x7b]);
        let a1278 = f108(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1277, a1278)
    };
    let a1280 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1279, a1280))
}

pub fn f246(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1281 = v0;
    match m1281 {
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
        Adt14::C7(v9, r1282, r1283) => {
            let v10 = (*r1282).clone();
            let v11 = (*r1283).clone();
            Ok(List::cons((v9, f187(v10)?), f246(v11)?))
        }
    }
}

pub fn f247(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1284 = v0;
    match m1284 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f125(v2, v1)
        }
    }
}

pub fn f248(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f170(f284(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f249(v0: Adt15, v1: Option<u64>) -> bool {
    let m1285 = v1;
    match m1285 {
        None => {
            true
        }
        Some(v2) => {
            f205(v2, match v0 {
                Adt15::C0(_, h1286, _, _) => {
                    h1286
                }
            })
        }
    }
}

pub fn f250(v0: Adt12) -> Adt18 {
    Adt18::C1({
        let a1288 = match v0 {
            Adt12::C0(_, _, h1287, _, _) => {
                h1287
            }
        };
        length_list(a1288)
    })
}

pub fn f251(v0: Bytes) -> Option<Adt19> {
    if f36(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt19::C0)
    } else {
        if f36(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt19::C1)
        } else {
            if f36(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt19::C2)
            } else {
                None::<Adt19>
            }
        }
    }
}

pub fn f252(v0: Bytes, p1293: List<Bytes>, p1294: u64) -> R<Option<u64>> {
    let m1292 = (p1293, p1294);
    {
        let mut m1292 = m1292;
        loop {
            let step = {
                let (v1, v2) = m1292;
                let m1289 = v1;
                match m1289.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1290 = v2;
                                let a1291 = 1u64;
                                nat_add(a1290, a1291)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: Option<Adt14>) -> Option<u64> {
    let m1295 = v0;
    match m1295 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1296 = v1;
            match m1296 {
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

pub fn f254(v0: Bytes, v1: Bytes) -> bool {
    let m1301 = {
        let a1298 = v0;
        let a1299 = 0u64;
        let a1300 = {
            let a1297 = v1.clone();
            length_bytes(a1297)
        };
        slice_bytes(a1298, a1299, a1300)
    };
    match m1301 {
        None => {
            false
        }
        Some(v2) => {
            let a1302 = v2;
            let a1303 = v1;
            equal(a1302, a1303)
        }
    }
}

pub fn f255(v0: Fn5, p1307: List<Bytes>, p1308: List<(Bytes, ())>) -> List<Bytes> {
    let m1306 = (p1307, p1308);
    {
        let mut m1306 = m1306;
        loop {
            let step = {
                let (v1, v2) = m1306;
                let m1304 = v2;
                match m1304.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1305: Fn5 = v0.clone();
                            c1305.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1309, _) = v1;
        h1309
    }, v0)
}

pub fn f257(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f294(Fn12::F295, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f258(v0: Fn6, p1313: List<List<(Bytes, ())>>, p1314: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1312 = (p1313, p1314);
    {
        let mut m1312 = m1312;
        loop {
            let step = {
                let (v1, v2) = m1312;
                let m1310 = v2;
                match m1310.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1311: Fn6 = v0.clone();
                            c1311.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f260(v0: Fn7, p1319: u64, p1320: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1318 = (p1319, p1320);
    {
        let mut m1318 = m1318;
        loop {
            let step = {
                let (v1, v2) = m1318;
                let m1315 = v1;
                if m1315 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1315 - 1;
                    let m1317 = {
                        let c1316: Fn7 = v0.clone();
                        c1316.apply(v2.clone())?
                    };
                    match m1317 {
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
                Ok(next) => { m1318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f296(v0)
}

pub fn f262(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f297(Fn13::F298, List::<(Bytes, ())>::nil(), v0)
}

pub fn f263(v0: Fn8, p1324: List<(Bytes, ())>, p1325: List<Bytes>) -> List<(Bytes, ())> {
    let m1323 = (p1324, p1325);
    {
        let mut m1323 = m1323;
        loop {
            let step = {
                let (v1, v2) = m1323;
                let m1321 = v2;
                match m1321.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1322: Fn8 = v0.clone();
                            c1322.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f265(v0: Bytes) -> Bytes {
    if f110(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1327 = {
                let a1326 = v0;
                length_bytes(a1326)
            };
            let a1328 = 1u64;
            nat_sub(a1327, a1328)
        })
    } else {
        v0
    }
}

pub fn f266(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f80(f67(v0.clone(), v1.clone()))?;
    let m1329 = f127(v2.clone())?;
    match m1329 {
        None => {
            f300(v0, v1, v2)
        }
        Some(v3) => {
            f299(v0, v1, v3)
        }
    }
}

pub fn f267(v0: List<Bytes>) -> List<Bytes> {
    let m1330 = v0;
    match m1330.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1332 = {
                    let a1331 = v1.clone();
                    length_bytes(a1331)
                };
                let a1333 = 0u64;
                if nat_eq(a1332, a1333) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f267(v2))
        }
    }
}

pub fn f268(v0: u32) -> Bytes {
    let a1336 = {
        let a1334 = v0;
        let a1335 = 8u32;
        fixed_u32::bit_and(a1334, a1335)
    };
    let a1337 = 0u32;
    if equal(a1336, a1337) {
        let a1340 = {
            let a1338 = v0;
            let a1339 = 4u32;
            fixed_u32::bit_and(a1338, a1339)
        };
        let a1341 = 0u32;
        if equal(a1340, a1341) {
            let a1344 = {
                let a1342 = v0;
                let a1343 = 2u32;
                fixed_u32::bit_and(a1342, a1343)
            };
            let a1345 = 0u32;
            if equal(a1344, a1345) {
                let a1348 = {
                    let a1346 = v0;
                    let a1347 = 1u32;
                    fixed_u32::bit_and(a1346, a1347)
                };
                let a1349 = 0u32;
                if equal(a1348, a1349) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1352 = {
                    let a1350 = v0;
                    let a1351 = 1u32;
                    fixed_u32::bit_and(a1350, a1351)
                };
                let a1353 = 0u32;
                if equal(a1352, a1353) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1356 = {
                let a1354 = v0;
                let a1355 = 2u32;
                fixed_u32::bit_and(a1354, a1355)
            };
            let a1357 = 0u32;
            if equal(a1356, a1357) {
                let a1360 = {
                    let a1358 = v0;
                    let a1359 = 1u32;
                    fixed_u32::bit_and(a1358, a1359)
                };
                let a1361 = 0u32;
                if equal(a1360, a1361) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1364 = {
                    let a1362 = v0;
                    let a1363 = 1u32;
                    fixed_u32::bit_and(a1362, a1363)
                };
                let a1365 = 0u32;
                if equal(a1364, a1365) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1368 = {
            let a1366 = v0;
            let a1367 = 4u32;
            fixed_u32::bit_and(a1366, a1367)
        };
        let a1369 = 0u32;
        if equal(a1368, a1369) {
            let a1372 = {
                let a1370 = v0;
                let a1371 = 2u32;
                fixed_u32::bit_and(a1370, a1371)
            };
            let a1373 = 0u32;
            if equal(a1372, a1373) {
                let a1376 = {
                    let a1374 = v0;
                    let a1375 = 1u32;
                    fixed_u32::bit_and(a1374, a1375)
                };
                let a1377 = 0u32;
                if equal(a1376, a1377) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1380 = {
                    let a1378 = v0;
                    let a1379 = 1u32;
                    fixed_u32::bit_and(a1378, a1379)
                };
                let a1381 = 0u32;
                if equal(a1380, a1381) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1384 = {
                let a1382 = v0;
                let a1383 = 2u32;
                fixed_u32::bit_and(a1382, a1383)
            };
            let a1385 = 0u32;
            if equal(a1384, a1385) {
                let a1388 = {
                    let a1386 = v0;
                    let a1387 = 1u32;
                    fixed_u32::bit_and(a1386, a1387)
                };
                let a1389 = 0u32;
                if equal(a1388, a1389) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1392 = {
                    let a1390 = v0;
                    let a1391 = 1u32;
                    fixed_u32::bit_and(a1390, a1391)
                };
                let a1393 = 0u32;
                if equal(a1392, a1393) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f269(v0: u32, v1: u32) -> u32 {
    let m1396 = {
        let a1394 = v0;
        let a1395 = v1;
        fixed_u32::shift_right(a1394, a1395)
    };
    match m1396 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f270(v0: Adt20, v1: Adt22) -> R<Adt20> {
    let v2: Adt23 = f301(Fn14::F302, 64u64, Adt23::C0(match v0.clone() {
        Adt20::C0(h1397, _, _, _, _, _, _, _) => {
            h1397
        }
    }, match v0.clone() {
        Adt20::C0(_, h1398, _, _, _, _, _, _) => {
            h1398
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h1399, _, _, _, _, _) => {
            h1399
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1400, _, _, _, _) => {
            h1400
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, h1401, _, _, _) => {
            h1401
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1402, _, _) => {
            h1402
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1403, _) => {
            h1403
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, _, h1404) => {
            h1404
        }
    }, v1, 0u64))?;
    Ok(Adt20::C0(f303(match v0.clone() {
        Adt20::C0(h1405, _, _, _, _, _, _, _) => {
            h1405
        }
    }, match v2.clone() {
        Adt23::C0(h1406, _, _, _, _, _, _, _, _, _) => {
            h1406
        }
    }), f303(match v0.clone() {
        Adt20::C0(_, h1407, _, _, _, _, _, _) => {
            h1407
        }
    }, match v2.clone() {
        Adt23::C0(_, h1408, _, _, _, _, _, _, _, _) => {
            h1408
        }
    }), f303(match v0.clone() {
        Adt20::C0(_, _, h1409, _, _, _, _, _) => {
            h1409
        }
    }, match v2.clone() {
        Adt23::C0(_, _, h1410, _, _, _, _, _, _, _) => {
            h1410
        }
    }), f303(match v0.clone() {
        Adt20::C0(_, _, _, h1411, _, _, _, _) => {
            h1411
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, h1412, _, _, _, _, _, _) => {
            h1412
        }
    }), f303(match v0.clone() {
        Adt20::C0(_, _, _, _, h1413, _, _, _) => {
            h1413
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, h1414, _, _, _, _, _) => {
            h1414
        }
    }), f303(match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1415, _, _) => {
            h1415
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, h1416, _, _, _, _) => {
            h1416
        }
    }), f303(match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1417, _) => {
            h1417
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, _, h1418, _, _, _) => {
            h1418
        }
    }), f303(match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h1419) => {
            h1419
        }
    }, match v2 {
        Adt23::C0(_, _, _, _, _, _, _, h1420, _, _) => {
            h1420
        }
    })))
}

pub fn f271(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    Ok(Adt22::C0(f304(v0.clone(), v1, v2, v3)?, f304(v0.clone(), v1, v2, {
        let a1421 = v3;
        let a1422 = 4u64;
        nat_add(a1421, a1422)?
    })?, f304(v0.clone(), v1, v2, {
        let a1423 = v3;
        let a1424 = 8u64;
        nat_add(a1423, a1424)?
    })?, f304(v0.clone(), v1, v2, {
        let a1425 = v3;
        let a1426 = 12u64;
        nat_add(a1425, a1426)?
    })?, f304(v0.clone(), v1, v2, {
        let a1427 = v3;
        let a1428 = 16u64;
        nat_add(a1427, a1428)?
    })?, f304(v0.clone(), v1, v2, {
        let a1429 = v3;
        let a1430 = 20u64;
        nat_add(a1429, a1430)?
    })?, f304(v0.clone(), v1, v2, {
        let a1431 = v3;
        let a1432 = 24u64;
        nat_add(a1431, a1432)?
    })?, f304(v0.clone(), v1, v2, {
        let a1433 = v3;
        let a1434 = 28u64;
        nat_add(a1433, a1434)?
    })?, f304(v0.clone(), v1, v2, {
        let a1435 = v3;
        let a1436 = 32u64;
        nat_add(a1435, a1436)?
    })?, f304(v0.clone(), v1, v2, {
        let a1437 = v3;
        let a1438 = 36u64;
        nat_add(a1437, a1438)?
    })?, f304(v0.clone(), v1, v2, {
        let a1439 = v3;
        let a1440 = 40u64;
        nat_add(a1439, a1440)?
    })?, f304(v0.clone(), v1, v2, {
        let a1441 = v3;
        let a1442 = 44u64;
        nat_add(a1441, a1442)?
    })?, f304(v0.clone(), v1, v2, {
        let a1443 = v3;
        let a1444 = 48u64;
        nat_add(a1443, a1444)?
    })?, f304(v0.clone(), v1, v2, {
        let a1445 = v3;
        let a1446 = 52u64;
        nat_add(a1445, a1446)?
    })?, f304(v0.clone(), v1, v2, {
        let a1447 = v3;
        let a1448 = 56u64;
        nat_add(a1447, a1448)?
    })?, f304(v0, v1, v2, {
        let a1449 = v3;
        let a1450 = 60u64;
        nat_add(a1449, a1450)?
    })?))
}

pub fn f272(v0: Bytes, v1: u64) -> Option<u64> {
    let a1451 = 0u64;
    let a1452 = v1;
    if nat_lt(a1451, a1452) {
        if f305(v0, {
            let a1453 = v1;
            let a1454 = 1u64;
            nat_sub(a1453, a1454)
        }) {
            Some({
                let a1455 = v1;
                let a1456 = 1u64;
                nat_sub(a1455, a1456)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f273(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1457 = v2;
    let a1458 = v0;
    if nat_lt(a1457, a1458) {
        if f305(v1, v2) {
            Ok(Some({
                let a1459 = v2;
                let a1460 = 1u64;
                nat_add(a1459, a1460)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f274(v0: Bytes, p1540: u64, p1541: u64, p1542: bool, p1543: bool, p1544: Bytes) -> R<Bytes> {
    let m1539 = (p1540, (p1541, (p1542, (p1543, p1544))));
    {
        let mut m1539 = m1539;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1539;
                let m1461 = v2;
                if m1461 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1461 - 1;
                    let m1464 = {
                        let a1462 = v0.clone();
                        let a1463 = v1;
                        index_bytes(a1462, a1463)
                    };
                    match m1464 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1509 = {
                                    let a1485 = {
                                        let a1473 = {
                                            let a1467 = {
                                                let a1465 = v8;
                                                let a1466 = 128u8;
                                                fixed_u8::bit_and(a1465, a1466)
                                            };
                                            let a1468 = 128u8;
                                            if equal(a1467, a1468) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1474 = {
                                            let a1471 = {
                                                let a1469 = v8;
                                                let a1470 = 64u8;
                                                fixed_u8::bit_and(a1469, a1470)
                                            };
                                            let a1472 = 64u8;
                                            if equal(a1471, a1472) {
                                                64u64
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
                                                let a1476 = 32u8;
                                                fixed_u8::bit_and(a1475, a1476)
                                            };
                                            let a1478 = 32u8;
                                            if equal(a1477, a1478) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1484 = {
                                            let a1481 = {
                                                let a1479 = v8;
                                                let a1480 = 16u8;
                                                fixed_u8::bit_and(a1479, a1480)
                                            };
                                            let a1482 = 16u8;
                                            if equal(a1481, a1482) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1483, a1484)?
                                    };
                                    nat_add(a1485, a1486)?
                                };
                                let a1510 = {
                                    let a1507 = {
                                        let a1495 = {
                                            let a1489 = {
                                                let a1487 = v8;
                                                let a1488 = 8u8;
                                                fixed_u8::bit_and(a1487, a1488)
                                            };
                                            let a1490 = 8u8;
                                            if equal(a1489, a1490) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1496 = {
                                            let a1493 = {
                                                let a1491 = v8;
                                                let a1492 = 4u8;
                                                fixed_u8::bit_and(a1491, a1492)
                                            };
                                            let a1494 = 4u8;
                                            if equal(a1493, a1494) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1495, a1496)?
                                    };
                                    let a1508 = {
                                        let a1505 = {
                                            let a1499 = {
                                                let a1497 = v8;
                                                let a1498 = 2u8;
                                                fixed_u8::bit_and(a1497, a1498)
                                            };
                                            let a1500 = 2u8;
                                            if equal(a1499, a1500) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1506 = {
                                            let a1503 = {
                                                let a1501 = v8;
                                                let a1502 = 1u8;
                                                fixed_u8::bit_and(a1501, a1502)
                                            };
                                            let a1504 = 1u8;
                                            if equal(a1503, a1504) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1505, a1506)?
                                    };
                                    nat_add(a1507, a1508)?
                                };
                                nat_add(a1509, a1510)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1511 = v1;
                                    let a1512 = 1u64;
                                    nat_add(a1511, a1512)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1513 = v9;
                                    let a1514 = 34u64;
                                    !nat_eq(a1513, a1514)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1515 = v9;
                                    let a1516 = 92u64;
                                    nat_eq(a1515, a1516)
                                }, {
                                    let a1517 = v5;
                                    let a1518 = v10;
                                    append_bytes(a1517, a1518)
                                })))))
                            } else {
                                let a1519 = v9;
                                let a1520 = 32u64;
                                if if if if nat_eq(a1519, a1520) {
                                    true
                                } else {
                                    let a1521 = v9;
                                    let a1522 = 9u64;
                                    nat_eq(a1521, a1522)
                                } {
                                    true
                                } else {
                                    let a1523 = v9;
                                    let a1524 = 10u64;
                                    nat_eq(a1523, a1524)
                                } {
                                    true
                                } else {
                                    let a1525 = v9;
                                    let a1526 = 13u64;
                                    nat_eq(a1525, a1526)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1527 = v1;
                                        let a1528 = 1u64;
                                        nat_add(a1527, a1528)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1529 = v1;
                                            let a1530 = 4u64;
                                            nat_add(a1529, a1530)?
                                        }, (v6, (false, (false, {
                                            let a1531 = v5;
                                            let a1532 = f8(Str::lit("[]"));
                                            append_bytes(a1531, a1532)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1533 = v1;
                                            let a1534 = 1u64;
                                            nat_add(a1533, a1534)?
                                        }, (v6, ({
                                            let a1535 = v9;
                                            let a1536 = 34u64;
                                            nat_eq(a1535, a1536)
                                        }, (false, {
                                            let a1537 = v5;
                                            let a1538 = v10;
                                            append_bytes(a1537, a1538)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1539 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Bytes, v1: u64) -> R<u64> {
    let m1547 = {
        let a1545 = v0;
        let a1546 = v1;
        index_bytes(a1545, a1546)
    };
    match m1547 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1592 = {
                let a1568 = {
                    let a1556 = {
                        let a1550 = {
                            let a1548 = v3;
                            let a1549 = 128u8;
                            fixed_u8::bit_and(a1548, a1549)
                        };
                        let a1551 = 128u8;
                        if equal(a1550, a1551) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1557 = {
                        let a1554 = {
                            let a1552 = v3;
                            let a1553 = 64u8;
                            fixed_u8::bit_and(a1552, a1553)
                        };
                        let a1555 = 64u8;
                        if equal(a1554, a1555) {
                            64u64
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
                            let a1559 = 32u8;
                            fixed_u8::bit_and(a1558, a1559)
                        };
                        let a1561 = 32u8;
                        if equal(a1560, a1561) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1567 = {
                        let a1564 = {
                            let a1562 = v3;
                            let a1563 = 16u8;
                            fixed_u8::bit_and(a1562, a1563)
                        };
                        let a1565 = 16u8;
                        if equal(a1564, a1565) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1566, a1567)?
                };
                nat_add(a1568, a1569)?
            };
            let a1593 = {
                let a1590 = {
                    let a1578 = {
                        let a1572 = {
                            let a1570 = v3;
                            let a1571 = 8u8;
                            fixed_u8::bit_and(a1570, a1571)
                        };
                        let a1573 = 8u8;
                        if equal(a1572, a1573) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1579 = {
                        let a1576 = {
                            let a1574 = v3;
                            let a1575 = 4u8;
                            fixed_u8::bit_and(a1574, a1575)
                        };
                        let a1577 = 4u8;
                        if equal(a1576, a1577) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1578, a1579)?
                };
                let a1591 = {
                    let a1588 = {
                        let a1582 = {
                            let a1580 = v3;
                            let a1581 = 2u8;
                            fixed_u8::bit_and(a1580, a1581)
                        };
                        let a1583 = 2u8;
                        if equal(a1582, a1583) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1589 = {
                        let a1586 = {
                            let a1584 = v3;
                            let a1585 = 1u8;
                            fixed_u8::bit_and(a1584, a1585)
                        };
                        let a1587 = 1u8;
                        if equal(a1586, a1587) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1588, a1589)?
                };
                nat_add(a1590, a1591)?
            };
            nat_add(a1592, a1593)
        }
    }
}

pub fn f276(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f306(Fn15::F307(v0.clone()), {
        let a1595 = {
            let a1594 = v0;
            length_bytes(a1594)
        };
        let a1596 = 1u64;
        nat_add(a1595, a1596)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1600 = {
        let (_, h1597) = {
            let (_, h1598) = {
                let (h1599, _) = v2.clone();
                h1599
            };
            h1598
        };
        h1597
    };
    let a1601 = 1u64;
    if nat_eq(a1600, a1601) {
        Ok(Some(({
            let (h1602, _) = {
                let (h1603, _) = v2.clone();
                h1603
            };
            h1602
        }, {
            let (h1604, _) = {
                let (_, h1605) = {
                    let (h1606, _) = v2;
                    h1606
                };
                h1605
            };
            h1604
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f277(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1607 = f275(v0.clone(), v1)?;
        let a1608 = 45u64;
        nat_eq(a1607, a1608)
    };
    let v3: u64 = if v2 {
        let a1609 = v1;
        let a1610 = 1u64;
        nat_add(a1609, a1610)?
    } else {
        v1
    };
    let v4: u64 = f308(v0.clone(), v3)?;
    let a1613 = 19u64;
    let a1614 = {
        let a1611 = v4;
        let a1612 = v3;
        nat_sub(a1611, a1612)
    };
    if nat_lt(a1613, a1614) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1617 = f309(f32(v0, v3, {
            let a1615 = v4;
            let a1616 = v3;
            nat_sub(a1615, a1616)
        }))?;
        match m1617 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f310(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f278(v0: u64) -> bool {
    let a1618 = v0;
    let a1619 = 95u64;
    if nat_eq(a1618, a1619) {
        true
    } else {
        let a1620 = v0;
        let a1621 = 45u64;
        if nat_eq(a1620, a1621) {
            true
        } else {
            let a1622 = v0;
            let a1623 = 32u64;
            if nat_eq(a1622, a1623) {
                true
            } else {
                let a1624 = v0;
                let a1625 = 46u64;
                if nat_eq(a1624, a1625) {
                    true
                } else {
                    let a1626 = v0;
                    let a1627 = 47u64;
                    nat_eq(a1626, a1627)
                }
            }
        }
    }
}

pub fn f279(v0: u64) -> bool {
    let a1628 = 97u64;
    let a1629 = v0;
    if nat_le(a1628, a1629) {
        let a1630 = v0;
        let a1631 = 122u64;
        nat_le(a1630, a1631)
    } else {
        false
    }
}

pub fn f280(v0: u64) -> bool {
    let a1632 = 65u64;
    let a1633 = v0;
    if nat_le(a1632, a1633) {
        let a1634 = v0;
        let a1635 = 90u64;
        nat_le(a1634, a1635)
    } else {
        false
    }
}

pub fn f281(v0: u64) -> bool {
    let a1636 = 48u64;
    let a1637 = v0;
    if nat_le(a1636, a1637) {
        let a1638 = v0;
        let a1639 = 57u64;
        nat_le(a1638, a1639)
    } else {
        false
    }
}

pub fn f282(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m1640 = f122(v1.clone(), f8(Str::lit("enum")));
    match m1640 {
        None => {
            let m1642 = f314(v0.clone(), f239(f122(v1.clone(), f8(Str::lit("properties")))), f71(f126(f122(v1, f8(Str::lit("required"))))))?;
            match m1642 {
                Ok(v6) => {
                    Ok(f313(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m1644 = {
                            let (h1643, _) = v6.clone();
                            h1643
                        };
                        match m1644.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f170({
                                    let (h1645, _) = v6;
                                    h1645
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1646) = v6;
                        h1646
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt21>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m1641 = f311(f188(v2), List::<Bytes>::nil())?;
            match m1641 {
                Ok(v4) => {
                    Ok(f313(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f170(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt21>::nil()))
                }
                Err(v3) => {
                    Ok(f312(v3, v0))
                }
            }
        }
    }
}

pub fn f283(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1647 = v0;
    match m1647 {
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
        Adt14::C4(r1648) => {
            let v4 = (*r1648).clone();
            f283(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1649, r1650) => {
            let v2 = (*r1649).clone();
            let v3 = (*r1650).clone();
            List::cons((v1, v2), f283(v3))
        }
    }
}

pub fn f284(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1651 = v0;
    match m1651.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f165(v1)?, f284(v2)?))
        }
    }
}

pub fn f285(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1652 = v0;
    match m1652.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f285(v3, false))
            } else {
                f285(v3, true)
            }
        }
    }
}

pub fn f286(v0: u64) -> R<Bytes> {
    f315(f316(), 10u64, v0)
}

pub fn f287(v0: Fn9, p1657: u64, p1658: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1656 = (p1657, p1658);
    {
        let mut m1656 = m1656;
        loop {
            let step = {
                let (v1, v2) = m1656;
                let m1653 = v1;
                if m1653 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1653 - 1;
                    let m1655 = {
                        let c1654: Fn9 = v0.clone();
                        c1654.apply(v2.clone())?
                    };
                    match m1655 {
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
                Ok(next) => { m1656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f317(v0, v1)
}

pub fn f289(v0: Fn10, p1662: List<(Bytes, Bytes)>, p1663: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1661 = (p1662, p1663);
    {
        let mut m1661 = m1661;
        loop {
            let step = {
                let (v1, v2) = m1661;
                let m1659 = v2;
                match m1659.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1660: Fn10 = v0.clone();
                            c1660.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f318(v0, v1)
}

pub fn f291(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f319(v0)
}

pub fn f292(v0: Fn11, p1667: List<Bytes>, p1668: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1666 = (p1667, p1668);
    {
        let mut m1666 = m1666;
        loop {
            let step = {
                let (v1, v2) = m1666;
                let m1664 = v2;
                match m1664.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1665: Fn11 = v0.clone();
                            c1665.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1666 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f293(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f320(v1)?, v0))
}

pub fn f294(v0: Fn12, p1672: List<List<(Bytes, ())>>, p1673: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1671 = (p1672, p1673);
    {
        let mut m1671 = m1671;
        loop {
            let step = {
                let (v1, v2) = m1671;
                let m1669 = v2;
                match m1669.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1670: Fn12 = v0.clone();
                            c1670.apply(v1, v3)
                        }, v4))
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

pub fn f295(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f296(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1675 = {
        let a1674 = v0.clone();
        length_list(a1674)
    };
    let a1676 = 1u64;
    if nat_le(a1675, a1676) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f321(v0)?))
    }
}

pub fn f297(v0: Fn13, p1680: List<(Bytes, ())>, p1681: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1679 = (p1680, p1681);
    {
        let mut m1679 = m1679;
        loop {
            let step = {
                let (v1, v2) = m1679;
                let m1677 = v2;
                match m1677.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1678: Fn13 = v0.clone();
                            c1678.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f299(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f69(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f322(v3.clone(), f69(f70(v0, f85())?, f8(Str::lit("at"))));
    let a1683 = {
        let a1682 = v4.clone();
        length_list(a1682)
    };
    let a1684 = 0u64;
    if nat_eq(a1683, a1684) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1687 = {
            let a1685 = v4;
            length_list(a1685)
        };
        let a1688 = {
            let a1686 = v3;
            length_list(a1686)
        };
        if nat_eq(a1687, a1688) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f323(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f300(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f178(f117(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f178(f117(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f61(f69(v3.clone(), f8(Str::lit("id"))), f69(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f69(f70(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f322(v5.clone(), f61(v6.clone(), f69(f70(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1690 = {
        let a1689 = v7.clone();
        length_list(a1689)
    };
    let a1691 = 0u64;
    if nat_eq(a1690, a1691) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f72({
            let a1694 = {
                let a1692 = v7;
                length_list(a1692)
            };
            let a1695 = {
                let a1693 = v5;
                length_list(a1693)
            };
            nat_eq(a1694, a1695)
        }) {
            Ok((false, f323(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f75(f61(f322(f324(v4.clone()), f66(f67(v0, f8(Str::lit("model/authorities.toml"))))?), f322(f325(v4), f61(v6, f69(v3, f8(Str::lit("id")))))))?;
            let a1697 = {
                let a1696 = v8.clone();
                length_list(a1696)
            };
            let a1698 = 0u64;
            if nat_eq(a1697, a1698) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f323(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f108(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f301(v0: Fn14, p1702: u64, p1703: Adt23) -> R<Adt23> {
    let m1701 = (p1702, p1703);
    {
        let mut m1701 = m1701;
        loop {
            let step = {
                let (v1, v2) = m1701;
                let m1699 = v1;
                if m1699 == 0 {
                    Err::<(u64, Adt23), R<Adt23>>(Ok(v2))
                } else {
                    let v3 = m1699 - 1;
                    Ok::<(u64, Adt23), R<Adt23>>((v3, {
                        let c1700: Fn14 = v0.clone();
                        c1700.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(v0: Adt23) -> R<Adt23> {
    f326(v0)
}

pub fn f303(v0: u32, v1: u32) -> u32 {
    let m1706 = {
        let a1704 = f327(v0);
        let a1705 = f327(v1);
        fixed_u64::checked_add(a1704, a1705)
    };
    match m1706 {
        None => {
            0u32
        }
        Some(v2) => {
            f328(v2)
        }
    }
}

pub fn f304(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1709 = {
        let a1707 = v3;
        let a1708 = 8u64;
        nat_add(a1707, a1708)?
    };
    let a1710 = v2;
    if nat_eq(a1709, a1710) {
        Ok(f329({
            let a1713 = {
                let a1711 = v1;
                let a1712 = 8u64;
                nat_mul(a1711, a1712)?
            };
            let a1714 = 4294967296u64;
            let a1715 = 0u64;
            nat_quot(a1713, a1714, a1715)
        }))
    } else {
        let a1718 = {
            let a1716 = v3;
            let a1717 = 4u64;
            nat_add(a1716, a1717)?
        };
        let a1719 = v2;
        if nat_eq(a1718, a1719) {
            Ok(f329({
                let a1722 = {
                    let a1720 = v1;
                    let a1721 = 8u64;
                    nat_mul(a1720, a1721)?
                };
                let a1723 = 4294967296u64;
                let a1724 = 0u64;
                nat_rem(a1722, a1723, a1724)
            }))
        } else {
            Ok(f330(f331(v0.clone(), v1, v3), f331(v0.clone(), v1, {
                let a1725 = v3;
                let a1726 = 1u64;
                nat_add(a1725, a1726)?
            }), f331(v0.clone(), v1, {
                let a1727 = v3;
                let a1728 = 2u64;
                nat_add(a1727, a1728)?
            }), f331(v0, v1, {
                let a1729 = v3;
                let a1730 = 3u64;
                nat_add(a1729, a1730)?
            })))
        }
    }
}

pub fn f305(v0: Bytes, v1: u64) -> bool {
    let m1731 = f193(v0, v1);
    match m1731 {
        None => {
            false
        }
        Some(v2) => {
            f332(v2)
        }
    }
}

pub fn f306(v0: Fn15, p1736: u64, p1737: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1735 = (p1736, p1737);
    {
        let mut m1735 = m1735;
        loop {
            let step = {
                let (v1, v2) = m1735;
                let m1732 = v1;
                if m1732 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1732 - 1;
                    let m1734 = {
                        let c1733: Fn15 = v0.clone();
                        c1733.apply(v2.clone())?
                    };
                    match m1734 {
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
                Ok(next) => { m1735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f307(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f333(v0, v1)
}

pub fn f308(v0: Bytes, v1: u64) -> R<u64> {
    let (h1738, _) = f195(Fn3::F334(v0.clone()), {
        let a1740 = {
            let a1739 = v0;
            length_bytes(a1739)
        };
        let a1741 = 1u64;
        nat_add(a1740, a1741)?
    }, v1)?;
    Ok(h1738)
}

pub fn f309(v0: Bytes) -> R<Option<u64>> {
    if f335(v0.clone())? {
        Ok(Some({
            let (_, h1742) = {
                let (h1743, _) = f336(Fn16::F337(v0.clone()), {
                    let a1744 = v0;
                    length_bytes(a1744)
                }, (0u64, 0u64))?;
                h1743
            };
            h1742
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f310(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1745 = 0u64;
        let a1746 = v1;
        if nat_lt(a1745, a1746) {
            let a1747 = v1;
            let a1748 = 9223372036854775808u64;
            nat_le(a1747, a1748)
        } else {
            false
        }
    } else {
        let a1749 = v1;
        let a1750 = 9223372036854775807u64;
        nat_le(a1749, a1750)
    }
}

pub fn f311(v0: List<Adt14>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1751 = v0;
    match m1751.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v2, v3)) => {
            let m1752 = f338(v2.clone())?;
            match m1752 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f339(v2)?, List::<Bytes>::nil())))))
                }
                Some(v4) => {
                    if f83(v4.clone(), v1.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v4, List::cons(f8(Str::lit(" (a string is named in camel case, an integer n is named nN, and a negative one nMinusN)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1753 = f311(v3, List::cons(v4.clone(), v1))?;
                        match m1753 {
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

pub fn f312(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt21>), Bytes> {
    Err::<(Bytes, List<Adt21>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f313(v0: Bytes, v1: List<Adt21>) -> Result<(Bytes, List<Adt21>), Bytes> {
    Ok::<(Bytes, List<Adt21>), Bytes>((v0, v1))
}

pub fn f314(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt21>), Bytes>> {
    let m1754 = v1;
    match m1754.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt21>), Bytes>((List::<Bytes>::nil(), List::<Adt21>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f340({
                let (h1755, _) = v3.clone();
                h1755
            })?;
            if f341(v5.clone())? {
                let m1760 = f342({
                    let (_, h1756) = v3.clone();
                    h1756
                }, {
                    let a1758 = v0.clone();
                    let a1759 = f182({
                        let (h1757, _) = v3.clone();
                        h1757
                    })?;
                    append_bytes(a1758, a1759)
                })?;
                match m1760 {
                    Ok(v7) => {
                        let v8: Bytes = if f83({
                            let (h1761, _) = v3;
                            h1761
                        }, v2.clone()) {
                            let (h1762, _) = v7;
                            h1762
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1763, _) = v7;
                                h1763
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1764 = f314(v0, v4, v2)?;
                        match m1764 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt21>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1765, _) = v10;
                                    h1765
                                }), f237({
                                    let (_, h1766) = v7;
                                    h1766
                                }, {
                                    let (_, h1767) = v10;
                                    h1767
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
                    let (h1768, _) = v3;
                    h1768
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f315(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1769 = v2;
    let a1770 = 0u64;
    if nat_eq(a1769, a1770) {
        Ok(f343(v0, 0u64))
    } else {
        let (_, h1771) = {
            let (h1772, _) = f287(Fn9::F344(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1772
        };
        Ok(h1771)
    }
}

pub fn f316() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f317(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1776 = {
        let a1774 = v0.clone();
        let a1775 = {
            let (h1773, _) = v1.clone();
            h1773
        };
        index_bytes(a1774, a1775)
    };
    match m1776 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1778 = {
                    let (h1777, _) = v1.clone();
                    h1777
                };
                let a1779 = 1u64;
                nat_add(a1778, a1779)?
            }, {
                let a1782 = {
                    let (_, h1780) = v1;
                    h1780
                };
                let a1783 = f345(v0, {
                    let (h1781, _) = v1;
                    h1781
                }, v2)?;
                append_bytes(a1782, a1783)
            })))
        }
    }
}

pub fn f318(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1784 = v0.clone();
    match m1784.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1787 = {
                let (h1785, _) = v2;
                h1785
            };
            let a1788 = {
                let (h1786, _) = v1.clone();
                h1786
            };
            if equal(a1787, a1788) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f319(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f346(f347(Fn17::F348, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1793 = {
        let (h1789, _) = f349(Fn18::F350, {
            let a1791 = {
                let a1790 = v0;
                length_list(a1790)
            };
            let a1792 = 1u64;
            nat_add(a1791, a1792)?
        }, v1)?;
        h1789
    };
    match m1793.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f320(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1798 = {
        let a1795 = f242({
            let (h1794, _) = v0;
            h1794
        })?;
        let a1796 = Bytes::lit(&[0x3a]);
        append_bytes(a1795, a1796)
    };
    let a1799 = {
        let (_, h1797) = v0;
        h1797
    };
    Ok(append_bytes(a1798, a1799))
}

pub fn f321(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f257({
        let (_, h1800) = {
            let (h1801, _) = f351(Fn19::F352, {
                let a1803 = {
                    let a1802 = v0.clone();
                    length_list(a1802)
                };
                let a1804 = 1u64;
                nat_add(a1803, a1804)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1801
        };
        h1800
    }))
}

pub fn f322(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1805 = v0;
    match m1805.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f83(v2.clone(), v1.clone()) {
                f322(v3, v1)
            } else {
                List::cons(v2, f322(v3, v1))
            }
        }
    }
}

pub fn f323(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f324(v0: List<Adt14>) -> List<Bytes> {
    let m1806 = v0;
    match m1806.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1807 = f122(v1, f8(Str::lit("std")));
            match m1807 {
                None => {
                    f324(v2)
                }
                Some(v3) => {
                    List::cons(f125(v3, f8(Str::lit("authority"))), f324(v2))
                }
            }
        }
    }
}

pub fn f325(v0: List<Adt14>) -> List<Bytes> {
    let m1808 = v0;
    match m1808.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1809 = f130(f122(v1, f8(Str::lit("req"))));
            match m1809 {
                None => {
                    f325(v2)
                }
                Some(v3) => {
                    List::cons(v3, f325(v2))
                }
            }
        }
    }
}

pub fn f326(v0: Adt23) -> R<Adt23> {
    let v1: u32 = f353(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, _, h1810) => {
            h1810
        }
    });
    let v2: u32 = f303(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h1811, _, _) => {
            h1811
        }
    }, f303(f354(match v0.clone() {
        Adt23::C0(_, _, _, _, h1812, _, _, _, _, _) => {
            h1812
        }
    }), f303(f355(match v0.clone() {
        Adt23::C0(_, _, _, _, h1813, _, _, _, _, _) => {
            h1813
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1814, _, _, _, _) => {
            h1814
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1815, _, _, _) => {
            h1815
        }
    }), f303(v1, {
        let m1818 = match v0.clone() {
            Adt23::C0(_, _, _, _, _, _, _, _, h1817, _) => {
                h1817
            }
        };
        match m1818 {
            Adt22::C0(h1816, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1816
            }
        }
    }))));
    let v3: u32 = f303(f356(match v0.clone() {
        Adt23::C0(h1819, _, _, _, _, _, _, _, _, _) => {
            h1819
        }
    }), f357(match v0.clone() {
        Adt23::C0(h1820, _, _, _, _, _, _, _, _, _) => {
            h1820
        }
    }, match v0.clone() {
        Adt23::C0(_, h1821, _, _, _, _, _, _, _, _) => {
            h1821
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1822, _, _, _, _, _, _, _) => {
            h1822
        }
    }));
    Ok(Adt23::C0(f303(v2, v3), match v0.clone() {
        Adt23::C0(h1823, _, _, _, _, _, _, _, _, _) => {
            h1823
        }
    }, match v0.clone() {
        Adt23::C0(_, h1824, _, _, _, _, _, _, _, _) => {
            h1824
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1825, _, _, _, _, _, _, _) => {
            h1825
        }
    }, f303(match v0.clone() {
        Adt23::C0(_, _, _, h1826, _, _, _, _, _, _) => {
            h1826
        }
    }, v2), match v0.clone() {
        Adt23::C0(_, _, _, _, h1827, _, _, _, _, _) => {
            h1827
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1828, _, _, _, _) => {
            h1828
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1829, _, _, _) => {
            h1829
        }
    }, f358(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, h1830, _) => {
            h1830
        }
    }), {
        let a1832 = match v0 {
            Adt23::C0(_, _, _, _, _, _, _, _, _, h1831) => {
                h1831
            }
        };
        let a1833 = 1u64;
        nat_add(a1832, a1833)?
    }))
}

pub fn f327(v0: u32) -> u64 {
    let m1835 = {
        let a1834 = v0;
        fixed_u64::convert(i128::from(a1834))
    };
    match m1835 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f328(v0: u64) -> u32 {
    let m1839 = {
        let a1838 = {
            let a1836 = v0;
            let a1837 = 4294967295u64;
            fixed_u64::bit_and(a1836, a1837)
        };
        fixed_u32::convert(i128::from(a1838))
    };
    match m1839 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f329(v0: u64) -> u32 {
    let (_, h1840) = {
        let (_, h1841) = f359(Fn20::F360, 32u64, (v0, (1u32, 0u32)));
        h1841
    };
    h1840
}

pub fn f330(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1846 = f361(v0, 24u32);
    let a1847 = {
        let a1844 = f361(v1, 16u32);
        let a1845 = {
            let a1842 = f361(v2, 8u32);
            let a1843 = v3;
            fixed_u32::bit_or(a1842, a1843)
        };
        fixed_u32::bit_or(a1844, a1845)
    };
    fixed_u32::bit_or(a1846, a1847)
}

pub fn f331(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1848 = v2;
    let a1849 = v1;
    if nat_lt(a1848, a1849) {
        let m1852 = {
            let a1850 = v0;
            let a1851 = v2;
            index_bytes(a1850, a1851)
        };
        match m1852 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1854 = {
                    let a1853 = v3;
                    fixed_u32::convert(i128::from(a1853))
                };
                match m1854 {
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
        let a1855 = v2;
        let a1856 = v1;
        if nat_eq(a1855, a1856) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f332(v0: u8) -> bool {
    let a1857 = v0;
    let a1858 = 32u8;
    if if if equal(a1857, a1858) {
        true
    } else {
        let a1859 = v0;
        let a1860 = 9u8;
        equal(a1859, a1860)
    } {
        true
    } else {
        let a1861 = v0;
        let a1862 = 10u8;
        equal(a1861, a1862)
    } {
        true
    } else {
        let a1863 = v0;
        let a1864 = 12u8;
        if equal(a1863, a1864) {
            true
        } else {
            let a1865 = v0;
            let a1866 = 13u8;
            equal(a1865, a1866)
        }
    }
}

pub fn f333(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1867, _) = v1.clone();
        h1867
    };
    let v3: Bytes = {
        let (h1868, _) = {
            let (_, h1869) = v1.clone();
            h1869
        };
        h1868
    };
    let a1872 = {
        let (_, h1870) = {
            let (_, h1871) = v1;
            h1871
        };
        h1870
    };
    let a1873 = 0u64;
    if nat_eq(a1872, a1873) {
        let v4: u64 = f275(v0.clone(), v2)?;
        let a1874 = v4;
        let a1875 = 34u64;
        if nat_eq(a1874, a1875) {
            Ok(Some(({
                let a1876 = v2;
                let a1877 = 1u64;
                nat_add(a1876, a1877)?
            }, (v3, 1u64))))
        } else {
            let a1878 = v4;
            let a1879 = 92u64;
            if nat_eq(a1878, a1879) {
                f362(v0, v2, v3)
            } else {
                let a1880 = v4;
                let a1881 = 32u64;
                if if nat_lt(a1880, a1881) {
                    true
                } else {
                    let a1882 = v4;
                    let a1883 = 256u64;
                    nat_eq(a1882, a1883)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1884 = v2;
                        let a1885 = 1u64;
                        nat_add(a1884, a1885)?
                    }, ({
                        let a1886 = v3;
                        let a1887 = f32(v0, v2, 1u64);
                        append_bytes(a1886, a1887)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f334(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f363(v0, v1)
}

pub fn f335(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1888 = v0.clone();
        length_bytes(a1888)
    };
    let a1889 = 0u64;
    let a1890 = v1;
    if nat_lt(a1889, a1890) {
        let a1891 = f364(v0.clone())?;
        let a1892 = v1;
        if nat_eq(a1891, a1892) {
            let a1893 = 1u64;
            let a1894 = v1;
            if nat_lt(a1893, a1894) {
                let m1897 = {
                    let a1895 = v0;
                    let a1896 = 0u64;
                    index_bytes(a1895, a1896)
                };
                match m1897 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1898 = v2;
                        let a1899 = 48u8;
                        Ok(!equal(a1898, a1899))
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

pub fn f336(v0: Fn16, p1904: u64, p1905: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1903 = (p1904, p1905);
    {
        let mut m1903 = m1903;
        loop {
            let step = {
                let (v1, v2) = m1903;
                let m1900 = v1;
                if m1900 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1900 - 1;
                    let m1902 = {
                        let c1901: Fn16 = v0.clone();
                        c1901.apply(v2)?
                    };
                    match m1902 {
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
                Ok(next) => { m1903 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f365(v0, v1)
}

pub fn f338(v0: Adt14) -> R<Option<Bytes>> {
    let m1906 = v0;
    match m1906 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v3, v4) => {
            Ok(Some({
                let a1909 = if if v3 {
                    let a1907 = 0u64;
                    let a1908 = v4;
                    nat_lt(a1907, a1908)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a1910 = f241(false, v4)?;
                append_bytes(a1909, a1910)
            }))
        }
        Adt14::C2(v1) => {
            let v2: Bytes = f340(v1)?;
            if f341(v2.clone())? {
                Ok(Some(v2))
            } else {
                Ok(None::<Bytes>)
            }
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

pub fn f339(v0: Adt14) -> R<Bytes> {
    f187(v0)
}

pub fn f340(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f235(v0.clone(), 0u64, {
        let a1911 = v0;
        length_bytes(a1911)
    }, false, Bytes::lit(&[]))?;
    let m1914 = {
        let a1912 = v1.clone();
        let a1913 = 0u64;
        index_bytes(a1912, a1913)
    };
    match m1914 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1959 = {
                    let a1935 = {
                        let a1923 = {
                            let a1917 = {
                                let a1915 = v3;
                                let a1916 = 128u8;
                                fixed_u8::bit_and(a1915, a1916)
                            };
                            let a1918 = 128u8;
                            if equal(a1917, a1918) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1924 = {
                            let a1921 = {
                                let a1919 = v3;
                                let a1920 = 64u8;
                                fixed_u8::bit_and(a1919, a1920)
                            };
                            let a1922 = 64u8;
                            if equal(a1921, a1922) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1923, a1924)?
                    };
                    let a1936 = {
                        let a1933 = {
                            let a1927 = {
                                let a1925 = v3;
                                let a1926 = 32u8;
                                fixed_u8::bit_and(a1925, a1926)
                            };
                            let a1928 = 32u8;
                            if equal(a1927, a1928) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1934 = {
                            let a1931 = {
                                let a1929 = v3;
                                let a1930 = 16u8;
                                fixed_u8::bit_and(a1929, a1930)
                            };
                            let a1932 = 16u8;
                            if equal(a1931, a1932) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1933, a1934)?
                    };
                    nat_add(a1935, a1936)?
                };
                let a1960 = {
                    let a1957 = {
                        let a1945 = {
                            let a1939 = {
                                let a1937 = v3;
                                let a1938 = 8u8;
                                fixed_u8::bit_and(a1937, a1938)
                            };
                            let a1940 = 8u8;
                            if equal(a1939, a1940) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1946 = {
                            let a1943 = {
                                let a1941 = v3;
                                let a1942 = 4u8;
                                fixed_u8::bit_and(a1941, a1942)
                            };
                            let a1944 = 4u8;
                            if equal(a1943, a1944) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1945, a1946)?
                    };
                    let a1958 = {
                        let a1955 = {
                            let a1949 = {
                                let a1947 = v3;
                                let a1948 = 2u8;
                                fixed_u8::bit_and(a1947, a1948)
                            };
                            let a1950 = 2u8;
                            if equal(a1949, a1950) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1956 = {
                            let a1953 = {
                                let a1951 = v3;
                                let a1952 = 1u8;
                                fixed_u8::bit_and(a1951, a1952)
                            };
                            let a1954 = 1u8;
                            if equal(a1953, a1954) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1955, a1956)?
                    };
                    nat_add(a1957, a1958)?
                };
                nat_add(a1959, a1960)?
            };
            if f280(v4) {
                let a1967 = f32({
                    let a1961 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a1961)
                }, {
                    let a1962 = v4;
                    let a1963 = 65u64;
                    nat_sub(a1962, a1963)
                }, 1u64);
                let a1968 = f32(v1.clone(), 1u64, {
                    let a1965 = {
                        let a1964 = v1;
                        length_bytes(a1964)
                    };
                    let a1966 = 1u64;
                    nat_sub(a1965, a1966)
                });
                Ok(append_bytes(a1967, a1968))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f341(v0: Bytes) -> R<bool> {
    let m1971 = {
        let a1969 = v0;
        let a1970 = 0u64;
        index_bytes(a1969, a1970)
    };
    match m1971 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f279({
                let v2: u8 = v1;
                let a2016 = {
                    let a1992 = {
                        let a1980 = {
                            let a1974 = {
                                let a1972 = v2;
                                let a1973 = 128u8;
                                fixed_u8::bit_and(a1972, a1973)
                            };
                            let a1975 = 128u8;
                            if equal(a1974, a1975) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1981 = {
                            let a1978 = {
                                let a1976 = v2;
                                let a1977 = 64u8;
                                fixed_u8::bit_and(a1976, a1977)
                            };
                            let a1979 = 64u8;
                            if equal(a1978, a1979) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1980, a1981)?
                    };
                    let a1993 = {
                        let a1990 = {
                            let a1984 = {
                                let a1982 = v2;
                                let a1983 = 32u8;
                                fixed_u8::bit_and(a1982, a1983)
                            };
                            let a1985 = 32u8;
                            if equal(a1984, a1985) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1991 = {
                            let a1988 = {
                                let a1986 = v2;
                                let a1987 = 16u8;
                                fixed_u8::bit_and(a1986, a1987)
                            };
                            let a1989 = 16u8;
                            if equal(a1988, a1989) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1990, a1991)?
                    };
                    nat_add(a1992, a1993)?
                };
                let a2017 = {
                    let a2014 = {
                        let a2002 = {
                            let a1996 = {
                                let a1994 = v2;
                                let a1995 = 8u8;
                                fixed_u8::bit_and(a1994, a1995)
                            };
                            let a1997 = 8u8;
                            if equal(a1996, a1997) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2003 = {
                            let a2000 = {
                                let a1998 = v2;
                                let a1999 = 4u8;
                                fixed_u8::bit_and(a1998, a1999)
                            };
                            let a2001 = 4u8;
                            if equal(a2000, a2001) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2002, a2003)?
                    };
                    let a2015 = {
                        let a2012 = {
                            let a2006 = {
                                let a2004 = v2;
                                let a2005 = 2u8;
                                fixed_u8::bit_and(a2004, a2005)
                            };
                            let a2007 = 2u8;
                            if equal(a2006, a2007) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2013 = {
                            let a2010 = {
                                let a2008 = v2;
                                let a2009 = 1u8;
                                fixed_u8::bit_and(a2008, a2009)
                            };
                            let a2011 = 1u8;
                            if equal(a2010, a2011) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2012, a2013)?
                    };
                    nat_add(a2014, a2015)?
                };
                nat_add(a2016, a2017)?
            }) {
                Ok(true)
            } else {
                Ok(f280({
                    let v3: u8 = v1;
                    let a2062 = {
                        let a2038 = {
                            let a2026 = {
                                let a2020 = {
                                    let a2018 = v3;
                                    let a2019 = 128u8;
                                    fixed_u8::bit_and(a2018, a2019)
                                };
                                let a2021 = 128u8;
                                if equal(a2020, a2021) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a2027 = {
                                let a2024 = {
                                    let a2022 = v3;
                                    let a2023 = 64u8;
                                    fixed_u8::bit_and(a2022, a2023)
                                };
                                let a2025 = 64u8;
                                if equal(a2024, a2025) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2026, a2027)?
                        };
                        let a2039 = {
                            let a2036 = {
                                let a2030 = {
                                    let a2028 = v3;
                                    let a2029 = 32u8;
                                    fixed_u8::bit_and(a2028, a2029)
                                };
                                let a2031 = 32u8;
                                if equal(a2030, a2031) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a2037 = {
                                let a2034 = {
                                    let a2032 = v3;
                                    let a2033 = 16u8;
                                    fixed_u8::bit_and(a2032, a2033)
                                };
                                let a2035 = 16u8;
                                if equal(a2034, a2035) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2036, a2037)?
                        };
                        nat_add(a2038, a2039)?
                    };
                    let a2063 = {
                        let a2060 = {
                            let a2048 = {
                                let a2042 = {
                                    let a2040 = v3;
                                    let a2041 = 8u8;
                                    fixed_u8::bit_and(a2040, a2041)
                                };
                                let a2043 = 8u8;
                                if equal(a2042, a2043) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a2049 = {
                                let a2046 = {
                                    let a2044 = v3;
                                    let a2045 = 4u8;
                                    fixed_u8::bit_and(a2044, a2045)
                                };
                                let a2047 = 4u8;
                                if equal(a2046, a2047) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2048, a2049)?
                        };
                        let a2061 = {
                            let a2058 = {
                                let a2052 = {
                                    let a2050 = v3;
                                    let a2051 = 2u8;
                                    fixed_u8::bit_and(a2050, a2051)
                                };
                                let a2053 = 2u8;
                                if equal(a2052, a2053) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2059 = {
                                let a2056 = {
                                    let a2054 = v3;
                                    let a2055 = 1u8;
                                    fixed_u8::bit_and(a2054, a2055)
                                };
                                let a2057 = 1u8;
                                if equal(a2056, a2057) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2058, a2059)?
                        };
                        nat_add(a2060, a2061)?
                    };
                    nat_add(a2062, a2063)?
                }))
            }
        }
    }
}

pub fn f342(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m2064 = f366(v0.clone());
    match m2064 {
        None => {
            if f36(f125(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2065 = f122(v0, f8(Str::lit("items")));
                match m2065 {
                    None => {
                        Ok(f312(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f367(f8(Str::lit("List")), f368(v3, {
                            let a2066 = v1;
                            let a2067 = f8(Str::lit("Item"));
                            append_bytes(a2066, a2067)
                        })?))
                    }
                }
            } else {
                f368(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f367(f8(Str::lit("Option")), f368(v2, v1)?))
        }
    }
}

pub fn f343(v0: Bytes, v1: u64) -> Bytes {
    let m2071 = {
        let a2068 = v0;
        let a2069 = v1;
        let a2070 = 1u64;
        slice_bytes(a2068, a2069, a2070)
    };
    match m2071 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f344(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f369(v1, v0, v2)
}

pub fn f345(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2116 = {
            let a2092 = {
                let a2080 = {
                    let a2074 = {
                        let a2072 = v3;
                        let a2073 = 128u8;
                        fixed_u8::bit_and(a2072, a2073)
                    };
                    let a2075 = 128u8;
                    if equal(a2074, a2075) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2081 = {
                    let a2078 = {
                        let a2076 = v3;
                        let a2077 = 64u8;
                        fixed_u8::bit_and(a2076, a2077)
                    };
                    let a2079 = 64u8;
                    if equal(a2078, a2079) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2080, a2081)?
            };
            let a2093 = {
                let a2090 = {
                    let a2084 = {
                        let a2082 = v3;
                        let a2083 = 32u8;
                        fixed_u8::bit_and(a2082, a2083)
                    };
                    let a2085 = 32u8;
                    if equal(a2084, a2085) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2091 = {
                    let a2088 = {
                        let a2086 = v3;
                        let a2087 = 16u8;
                        fixed_u8::bit_and(a2086, a2087)
                    };
                    let a2089 = 16u8;
                    if equal(a2088, a2089) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2090, a2091)?
            };
            nat_add(a2092, a2093)?
        };
        let a2117 = {
            let a2114 = {
                let a2102 = {
                    let a2096 = {
                        let a2094 = v3;
                        let a2095 = 8u8;
                        fixed_u8::bit_and(a2094, a2095)
                    };
                    let a2097 = 8u8;
                    if equal(a2096, a2097) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2103 = {
                    let a2100 = {
                        let a2098 = v3;
                        let a2099 = 4u8;
                        fixed_u8::bit_and(a2098, a2099)
                    };
                    let a2101 = 4u8;
                    if equal(a2100, a2101) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2102, a2103)?
            };
            let a2115 = {
                let a2112 = {
                    let a2106 = {
                        let a2104 = v3;
                        let a2105 = 2u8;
                        fixed_u8::bit_and(a2104, a2105)
                    };
                    let a2107 = 2u8;
                    if equal(a2106, a2107) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2113 = {
                    let a2110 = {
                        let a2108 = v3;
                        let a2109 = 1u8;
                        fixed_u8::bit_and(a2108, a2109)
                    };
                    let a2111 = 1u8;
                    if equal(a2110, a2111) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2112, a2113)?
            };
            nat_add(a2114, a2115)?
        };
        nat_add(a2116, a2117)?
    };
    let a2118 = v4;
    let a2119 = 34u64;
    if nat_eq(a2118, a2119) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2120 = v4;
        let a2121 = 92u64;
        if nat_eq(a2120, a2121) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2122 = v4;
            let a2123 = 8u64;
            if nat_eq(a2122, a2123) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2124 = v4;
                let a2125 = 12u64;
                if nat_eq(a2124, a2125) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2126 = v4;
                    let a2127 = 10u64;
                    if nat_eq(a2126, a2127) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2128 = v4;
                        let a2129 = 13u64;
                        if nat_eq(a2128, a2129) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2130 = v4;
                            let a2131 = 9u64;
                            if nat_eq(a2130, a2131) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2132 = v4;
                                let a2133 = 32u64;
                                if nat_lt(a2132, a2133) {
                                    let a2134 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2135 = f370(v2)?;
                                    Ok(append_bytes(a2134, a2135))
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

pub fn f346(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f371(Fn21::F372, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f347(v0: Fn17, p2139: List<List<(Bytes, Bytes)>>, p2140: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2138 = (p2139, p2140);
    {
        let mut m2138 = m2138;
        loop {
            let step = {
                let (v1, v2) = m2138;
                let m2136 = v2;
                match m2136.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2137: Fn17 = v0.clone();
                            c2137.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f348(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f349(v0: Fn18, p2145: u64, p2146: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2144 = (p2145, p2146);
    {
        let mut m2144 = m2144;
        loop {
            let step = {
                let (v1, v2) = m2144;
                let m2141 = v1;
                if m2141 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2141 - 1;
                    let m2143 = {
                        let c2142: Fn18 = v0.clone();
                        c2142.apply(v2.clone())?
                    };
                    match m2143 {
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
                Ok(next) => { m2144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f373(v0)
}

pub fn f351(v0: Fn19, p2151: u64, p2152: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2150 = (p2151, p2152);
    {
        let mut m2150 = m2150;
        loop {
            let step = {
                let (v1, v2) = m2150;
                let m2147 = v1;
                if m2147 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2147 - 1;
                    let m2149 = {
                        let c2148: Fn19 = v0.clone();
                        c2148.apply(v2.clone())?
                    };
                    match m2149 {
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
                Ok(next) => { m2150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f374(v0)
}

pub fn f353(v0: u64) -> u32 {
    let a2153 = v0;
    let a2154 = 32u64;
    if nat_lt(a2153, a2154) {
        let a2155 = v0;
        let a2156 = 16u64;
        if nat_lt(a2155, a2156) {
            let a2157 = v0;
            let a2158 = 8u64;
            if nat_lt(a2157, a2158) {
                let a2159 = v0;
                let a2160 = 4u64;
                if nat_lt(a2159, a2160) {
                    let a2161 = v0;
                    let a2162 = 2u64;
                    if nat_lt(a2161, a2162) {
                        let a2163 = v0;
                        let a2164 = 1u64;
                        if nat_lt(a2163, a2164) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2165 = v0;
                        let a2166 = 3u64;
                        if nat_lt(a2165, a2166) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2167 = v0;
                    let a2168 = 6u64;
                    if nat_lt(a2167, a2168) {
                        let a2169 = v0;
                        let a2170 = 5u64;
                        if nat_lt(a2169, a2170) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2171 = v0;
                        let a2172 = 7u64;
                        if nat_lt(a2171, a2172) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2173 = v0;
                let a2174 = 12u64;
                if nat_lt(a2173, a2174) {
                    let a2175 = v0;
                    let a2176 = 10u64;
                    if nat_lt(a2175, a2176) {
                        let a2177 = v0;
                        let a2178 = 9u64;
                        if nat_lt(a2177, a2178) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2179 = v0;
                        let a2180 = 11u64;
                        if nat_lt(a2179, a2180) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2181 = v0;
                    let a2182 = 14u64;
                    if nat_lt(a2181, a2182) {
                        let a2183 = v0;
                        let a2184 = 13u64;
                        if nat_lt(a2183, a2184) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2185 = v0;
                        let a2186 = 15u64;
                        if nat_lt(a2185, a2186) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2187 = v0;
            let a2188 = 24u64;
            if nat_lt(a2187, a2188) {
                let a2189 = v0;
                let a2190 = 20u64;
                if nat_lt(a2189, a2190) {
                    let a2191 = v0;
                    let a2192 = 18u64;
                    if nat_lt(a2191, a2192) {
                        let a2193 = v0;
                        let a2194 = 17u64;
                        if nat_lt(a2193, a2194) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2195 = v0;
                        let a2196 = 19u64;
                        if nat_lt(a2195, a2196) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2197 = v0;
                    let a2198 = 22u64;
                    if nat_lt(a2197, a2198) {
                        let a2199 = v0;
                        let a2200 = 21u64;
                        if nat_lt(a2199, a2200) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2201 = v0;
                        let a2202 = 23u64;
                        if nat_lt(a2201, a2202) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2203 = v0;
                let a2204 = 28u64;
                if nat_lt(a2203, a2204) {
                    let a2205 = v0;
                    let a2206 = 26u64;
                    if nat_lt(a2205, a2206) {
                        let a2207 = v0;
                        let a2208 = 25u64;
                        if nat_lt(a2207, a2208) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2209 = v0;
                        let a2210 = 27u64;
                        if nat_lt(a2209, a2210) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2211 = v0;
                    let a2212 = 30u64;
                    if nat_lt(a2211, a2212) {
                        let a2213 = v0;
                        let a2214 = 29u64;
                        if nat_lt(a2213, a2214) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2215 = v0;
                        let a2216 = 31u64;
                        if nat_lt(a2215, a2216) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2217 = v0;
        let a2218 = 48u64;
        if nat_lt(a2217, a2218) {
            let a2219 = v0;
            let a2220 = 40u64;
            if nat_lt(a2219, a2220) {
                let a2221 = v0;
                let a2222 = 36u64;
                if nat_lt(a2221, a2222) {
                    let a2223 = v0;
                    let a2224 = 34u64;
                    if nat_lt(a2223, a2224) {
                        let a2225 = v0;
                        let a2226 = 33u64;
                        if nat_lt(a2225, a2226) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2227 = v0;
                        let a2228 = 35u64;
                        if nat_lt(a2227, a2228) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2229 = v0;
                    let a2230 = 38u64;
                    if nat_lt(a2229, a2230) {
                        let a2231 = v0;
                        let a2232 = 37u64;
                        if nat_lt(a2231, a2232) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2233 = v0;
                        let a2234 = 39u64;
                        if nat_lt(a2233, a2234) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2235 = v0;
                let a2236 = 44u64;
                if nat_lt(a2235, a2236) {
                    let a2237 = v0;
                    let a2238 = 42u64;
                    if nat_lt(a2237, a2238) {
                        let a2239 = v0;
                        let a2240 = 41u64;
                        if nat_lt(a2239, a2240) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2241 = v0;
                        let a2242 = 43u64;
                        if nat_lt(a2241, a2242) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2243 = v0;
                    let a2244 = 46u64;
                    if nat_lt(a2243, a2244) {
                        let a2245 = v0;
                        let a2246 = 45u64;
                        if nat_lt(a2245, a2246) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2247 = v0;
                        let a2248 = 47u64;
                        if nat_lt(a2247, a2248) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2249 = v0;
            let a2250 = 56u64;
            if nat_lt(a2249, a2250) {
                let a2251 = v0;
                let a2252 = 52u64;
                if nat_lt(a2251, a2252) {
                    let a2253 = v0;
                    let a2254 = 50u64;
                    if nat_lt(a2253, a2254) {
                        let a2255 = v0;
                        let a2256 = 49u64;
                        if nat_lt(a2255, a2256) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2257 = v0;
                        let a2258 = 51u64;
                        if nat_lt(a2257, a2258) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2259 = v0;
                    let a2260 = 54u64;
                    if nat_lt(a2259, a2260) {
                        let a2261 = v0;
                        let a2262 = 53u64;
                        if nat_lt(a2261, a2262) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2263 = v0;
                        let a2264 = 55u64;
                        if nat_lt(a2263, a2264) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2265 = v0;
                let a2266 = 60u64;
                if nat_lt(a2265, a2266) {
                    let a2267 = v0;
                    let a2268 = 58u64;
                    if nat_lt(a2267, a2268) {
                        let a2269 = v0;
                        let a2270 = 57u64;
                        if nat_lt(a2269, a2270) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2271 = v0;
                        let a2272 = 59u64;
                        if nat_lt(a2271, a2272) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2273 = v0;
                    let a2274 = 62u64;
                    if nat_lt(a2273, a2274) {
                        let a2275 = v0;
                        let a2276 = 61u64;
                        if nat_lt(a2275, a2276) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2277 = v0;
                        let a2278 = 63u64;
                        if nat_lt(a2277, a2278) {
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

pub fn f354(v0: u32) -> u32 {
    let a2281 = {
        let a2279 = f375(v0, 6u32, 26u32);
        let a2280 = f375(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2279, a2280)
    };
    let a2282 = f375(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2281, a2282)
}

pub fn f355(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2288 = {
        let a2283 = v0;
        let a2284 = v1;
        fixed_u32::bit_and(a2283, a2284)
    };
    let a2289 = {
        let a2286 = {
            let a2285 = v0;
            fixed_u32::bit_not(a2285)
        };
        let a2287 = v2;
        fixed_u32::bit_and(a2286, a2287)
    };
    fixed_u32::bit_xor(a2288, a2289)
}

pub fn f356(v0: u32) -> u32 {
    let a2292 = {
        let a2290 = f375(v0, 2u32, 30u32);
        let a2291 = f375(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2290, a2291)
    };
    let a2293 = f375(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2292, a2293)
}

pub fn f357(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2302 = {
        let a2298 = {
            let a2294 = v0;
            let a2295 = v1;
            fixed_u32::bit_and(a2294, a2295)
        };
        let a2299 = {
            let a2296 = v0;
            let a2297 = v2;
            fixed_u32::bit_and(a2296, a2297)
        };
        fixed_u32::bit_xor(a2298, a2299)
    };
    let a2303 = {
        let a2300 = v1;
        let a2301 = v2;
        fixed_u32::bit_and(a2300, a2301)
    };
    fixed_u32::bit_xor(a2302, a2303)
}

pub fn f358(v0: Adt22) -> Adt22 {
    Adt22::C0(match v0.clone() {
        Adt22::C0(_, h2304, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2304
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2305, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2305
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h2306, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2306
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h2307, _, _, _, _, _, _, _, _, _, _, _) => {
            h2307
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2308, _, _, _, _, _, _, _, _, _, _) => {
            h2308
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2309, _, _, _, _, _, _, _, _, _) => {
            h2309
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2310, _, _, _, _, _, _, _, _) => {
            h2310
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2311, _, _, _, _, _, _, _) => {
            h2311
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2312, _, _, _, _, _, _) => {
            h2312
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, h2313, _, _, _, _, _) => {
            h2313
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, h2314, _, _, _, _) => {
            h2314
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2315, _, _, _) => {
            h2315
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2316, _, _) => {
            h2316
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2317, _) => {
            h2317
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2318) => {
            h2318
        }
    }, f303(f376(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2319, _) => {
            h2319
        }
    }), f303(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2320, _, _, _, _, _, _) => {
            h2320
        }
    }, f303(f377(match v0.clone() {
        Adt22::C0(_, h2321, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2321
        }
    }), match v0 {
        Adt22::C0(h2322, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2322
        }
    }))))
}

pub fn f359(v0: Fn20, p2326: u64, p2327: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2325 = (p2326, p2327);
    {
        let mut m2325 = m2325;
        loop {
            let step = {
                let (v1, v2) = m2325;
                let m2323 = v1;
                if m2323 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2323 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2324: Fn20 = v0.clone();
                        c2324.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2325 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f360(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2328, _) = v0;
        h2328
    };
    let v2: u32 = {
        let (h2329, _) = {
            let (_, h2330) = v0;
            h2330
        };
        h2329
    };
    let v3: u32 = {
        let (_, h2331) = {
            let (_, h2332) = v0;
            h2332
        };
        h2331
    };
    ({
        let a2333 = v1;
        let a2334 = 2u64;
        let a2335 = 0u64;
        nat_quot(a2333, a2334, a2335)
    }, (f361(v2, 1u32), {
        let a2339 = {
            let a2336 = v1;
            let a2337 = 2u64;
            let a2338 = 0u64;
            nat_rem(a2336, a2337, a2338)
        };
        let a2340 = 1u64;
        if nat_eq(a2339, a2340) {
            let a2341 = v3;
            let a2342 = v2;
            fixed_u32::bit_or(a2341, a2342)
        } else {
            v3
        }
    }))
}

pub fn f361(v0: u32, v1: u32) -> u32 {
    let m2345 = {
        let a2343 = v0;
        let a2344 = v1;
        fixed_u32::shift_left(a2343, a2344)
    };
    match m2345 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f362(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f275(v0.clone(), {
        let a2346 = v1;
        let a2347 = 1u64;
        nat_add(a2346, a2347)?
    })?;
    let v4: Bytes = {
        let a2348 = v3;
        let a2349 = 117u64;
        if nat_eq(a2348, a2349) {
            f378(v0, v1)?
        } else {
            f379(v3)
        }
    };
    let a2350 = v4.clone();
    let a2351 = Bytes::lit(&[]);
    if equal(a2350, a2351) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2352 = v3;
            let a2353 = 117u64;
            if nat_eq(a2352, a2353) {
                let a2354 = v1;
                let a2355 = 6u64;
                nat_add(a2354, a2355)?
            } else {
                let a2356 = v1;
                let a2357 = 2u64;
                nat_add(a2356, a2357)?
            }
        }, ({
            let a2358 = v2;
            let a2359 = v4;
            append_bytes(a2358, a2359)
        }, 0u64))))
    }
}

pub fn f363(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f275(v0, v1)?;
    let a2360 = 47u64;
    let a2361 = v2;
    if if nat_lt(a2360, a2361) {
        let a2362 = v2;
        let a2363 = 58u64;
        nat_lt(a2362, a2363)
    } else {
        false
    } {
        Ok(Some({
            let a2364 = v1;
            let a2365 = 1u64;
            nat_add(a2364, a2365)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f364(v0: Bytes) -> R<u64> {
    let (h2366, _) = f195(Fn3::F380(v0.clone()), {
        let a2367 = v0;
        length_bytes(a2367)
    }, 0u64)?;
    Ok(h2366)
}

pub fn f365(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2371 = {
        let a2369 = v0;
        let a2370 = {
            let (h2368, _) = v1;
            h2368
        };
        index_bytes(a2369, a2370)
    };
    match m2371 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2373 = {
                    let (h2372, _) = v1;
                    h2372
                };
                let a2374 = 1u64;
                nat_add(a2373, a2374)?
            }, {
                let a2378 = {
                    let a2376 = {
                        let (_, h2375) = v1;
                        h2375
                    };
                    let a2377 = 10u64;
                    nat_mul(a2376, a2377)?
                };
                let a2379 = f381(v2)?;
                nat_add(a2378, a2379)?
            })))
        }
    }
}

pub fn f366(v0: Adt14) -> Option<Adt14> {
    let m2380 = f126(f122(v0, f8(Str::lit("oneOf"))));
    match m2380.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2381 = v2;
            match m2381.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2382 = v4;
                    match m2382.uncons() {
                        None => {
                            if f382(v3.clone()) {
                                Some(v1)
                            } else {
                                if f382(v1) {
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

pub fn f367(v0: Bytes, v1: Result<(Bytes, List<Adt21>), Bytes>) -> Result<(Bytes, List<Adt21>), Bytes> {
    let m2383 = v1;
    match m2383 {
        Ok(v3) => {
            f313(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2384, _) = v3;
                h2384
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2385) = v3;
                h2385
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt21>), Bytes>(v2)
        }
    }
}

pub fn f368(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m2386 = f366(v0.clone());
    match m2386 {
        None => {
            if f36(f125(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2387 = f122(v0, f8(Str::lit("items")));
                match m2387 {
                    None => {
                        Ok(f312(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f367(f8(Str::lit("List")), f383(v3, {
                            let a2388 = v1;
                            let a2389 = f8(Str::lit("Item"));
                            append_bytes(a2388, a2389)
                        })?))
                    }
                }
            } else {
                f383(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f367(f8(Str::lit("Option")), f383(v2, v1)?))
        }
    }
}

pub fn f369(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2390, _) = v2;
        h2390
    };
    let a2391 = v3;
    let a2392 = 0u64;
    if nat_eq(a2391, a2392) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2393 = v3;
            let a2394 = v1;
            let a2395 = 0u64;
            nat_quot(a2393, a2394, a2395)
        }, {
            let a2400 = f343(v0, {
                let a2396 = v3;
                let a2397 = v1;
                let a2398 = 0u64;
                nat_rem(a2396, a2397, a2398)
            });
            let a2401 = {
                let (_, h2399) = v2;
                h2399
            };
            append_bytes(a2400, a2401)
        }))
    }
}

pub fn f370(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2446 = {
            let a2422 = {
                let a2410 = {
                    let a2404 = {
                        let a2402 = v1;
                        let a2403 = 128u8;
                        fixed_u8::bit_and(a2402, a2403)
                    };
                    let a2405 = 128u8;
                    if equal(a2404, a2405) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2411 = {
                    let a2408 = {
                        let a2406 = v1;
                        let a2407 = 64u8;
                        fixed_u8::bit_and(a2406, a2407)
                    };
                    let a2409 = 64u8;
                    if equal(a2408, a2409) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2410, a2411)?
            };
            let a2423 = {
                let a2420 = {
                    let a2414 = {
                        let a2412 = v1;
                        let a2413 = 32u8;
                        fixed_u8::bit_and(a2412, a2413)
                    };
                    let a2415 = 32u8;
                    if equal(a2414, a2415) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2421 = {
                    let a2418 = {
                        let a2416 = v1;
                        let a2417 = 16u8;
                        fixed_u8::bit_and(a2416, a2417)
                    };
                    let a2419 = 16u8;
                    if equal(a2418, a2419) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2420, a2421)?
            };
            nat_add(a2422, a2423)?
        };
        let a2447 = {
            let a2444 = {
                let a2432 = {
                    let a2426 = {
                        let a2424 = v1;
                        let a2425 = 8u8;
                        fixed_u8::bit_and(a2424, a2425)
                    };
                    let a2427 = 8u8;
                    if equal(a2426, a2427) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2433 = {
                    let a2430 = {
                        let a2428 = v1;
                        let a2429 = 4u8;
                        fixed_u8::bit_and(a2428, a2429)
                    };
                    let a2431 = 4u8;
                    if equal(a2430, a2431) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2432, a2433)?
            };
            let a2445 = {
                let a2442 = {
                    let a2436 = {
                        let a2434 = v1;
                        let a2435 = 2u8;
                        fixed_u8::bit_and(a2434, a2435)
                    };
                    let a2437 = 2u8;
                    if equal(a2436, a2437) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2443 = {
                    let a2440 = {
                        let a2438 = v1;
                        let a2439 = 1u8;
                        fixed_u8::bit_and(a2438, a2439)
                    };
                    let a2441 = 1u8;
                    if equal(a2440, a2441) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2442, a2443)?
            };
            nat_add(a2444, a2445)?
        };
        nat_add(a2446, a2447)?
    };
    let a2454 = f343(f384(), {
        let a2448 = v2;
        let a2449 = 16u64;
        let a2450 = 0u64;
        nat_quot(a2448, a2449, a2450)
    });
    let a2455 = f343(f384(), {
        let a2451 = v2;
        let a2452 = 16u64;
        let a2453 = 0u64;
        nat_rem(a2451, a2452, a2453)
    });
    Ok(append_bytes(a2454, a2455))
}

pub fn f371(v0: Fn21, p2459: List<List<(Bytes, Bytes)>>, p2460: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2458 = (p2459, p2460);
    {
        let mut m2458 = m2458;
        loop {
            let step = {
                let (v1, v2) = m2458;
                let m2456 = v2;
                match m2456.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2457: Fn21 = v0.clone();
                            c2457.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f373(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2462 = {
        let a2461 = v0.clone();
        length_list(a2461)
    };
    let a2463 = 1u64;
    if nat_le(a2462, a2463) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f385(v0)?))
    }
}

pub fn f374(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2464, _) = v0;
        h2464
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2465) = v0;
        h2465
    };
    let m2466 = v1;
    match m2466.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2467 = v4;
            match m2467.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f386(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f375(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2468 = f269(v0, v1);
    let a2469 = f361(v0, v2);
    fixed_u32::bit_or(a2468, a2469)
}

pub fn f376(v0: u32) -> u32 {
    let a2472 = {
        let a2470 = f375(v0, 17u32, 15u32);
        let a2471 = f375(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2470, a2471)
    };
    let a2473 = f269(v0, 10u32);
    fixed_u32::bit_xor(a2472, a2473)
}

pub fn f377(v0: u32) -> u32 {
    let a2476 = {
        let a2474 = f375(v0, 7u32, 25u32);
        let a2475 = f375(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2474, a2475)
    };
    let a2477 = f269(v0, 3u32);
    fixed_u32::bit_xor(a2476, a2477)
}

pub fn f378(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2480 = f32(v0.clone(), {
        let a2478 = v1;
        let a2479 = 2u64;
        nat_add(a2478, a2479)?
    }, 2u64);
    let a2481 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2480, a2481) {
        let m2484 = f387(f32(v0, {
            let a2482 = v1;
            let a2483 = 4u64;
            nat_add(a2482, a2483)?
        }, 2u64))?;
        match m2484 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f275(v2.clone(), 0u64)?;
                let a2485 = v3;
                let a2486 = 32u64;
                if nat_lt(a2485, a2486) {
                    if f388(v3) {
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

pub fn f379(v0: u64) -> Bytes {
    let a2487 = v0;
    let a2488 = 34u64;
    if nat_eq(a2487, a2488) {
        Bytes::lit(&[0x22])
    } else {
        let a2489 = v0;
        let a2490 = 92u64;
        if nat_eq(a2489, a2490) {
            Bytes::lit(&[0x5c])
        } else {
            let a2491 = v0;
            let a2492 = 98u64;
            if nat_eq(a2491, a2492) {
                Bytes::lit(&[0x08])
            } else {
                let a2493 = v0;
                let a2494 = 102u64;
                if nat_eq(a2493, a2494) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2495 = v0;
                    let a2496 = 110u64;
                    if nat_eq(a2495, a2496) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2497 = v0;
                        let a2498 = 114u64;
                        if nat_eq(a2497, a2498) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2499 = v0;
                            let a2500 = 116u64;
                            if nat_eq(a2499, a2500) {
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

pub fn f380(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2503 = {
        let a2501 = v0;
        let a2502 = v1;
        index_bytes(a2501, a2502)
    };
    match m2503 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f389(v2)? {
                Ok(Some({
                    let a2504 = v1;
                    let a2505 = 1u64;
                    nat_add(a2504, a2505)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f381(v0: u8) -> R<u64> {
    let a2552 = {
        let v1: u8 = v0;
        let a2550 = {
            let a2526 = {
                let a2514 = {
                    let a2508 = {
                        let a2506 = v1;
                        let a2507 = 128u8;
                        fixed_u8::bit_and(a2506, a2507)
                    };
                    let a2509 = 128u8;
                    if equal(a2508, a2509) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2515 = {
                    let a2512 = {
                        let a2510 = v1;
                        let a2511 = 64u8;
                        fixed_u8::bit_and(a2510, a2511)
                    };
                    let a2513 = 64u8;
                    if equal(a2512, a2513) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2514, a2515)?
            };
            let a2527 = {
                let a2524 = {
                    let a2518 = {
                        let a2516 = v1;
                        let a2517 = 32u8;
                        fixed_u8::bit_and(a2516, a2517)
                    };
                    let a2519 = 32u8;
                    if equal(a2518, a2519) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2525 = {
                    let a2522 = {
                        let a2520 = v1;
                        let a2521 = 16u8;
                        fixed_u8::bit_and(a2520, a2521)
                    };
                    let a2523 = 16u8;
                    if equal(a2522, a2523) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2524, a2525)?
            };
            nat_add(a2526, a2527)?
        };
        let a2551 = {
            let a2548 = {
                let a2536 = {
                    let a2530 = {
                        let a2528 = v1;
                        let a2529 = 8u8;
                        fixed_u8::bit_and(a2528, a2529)
                    };
                    let a2531 = 8u8;
                    if equal(a2530, a2531) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2537 = {
                    let a2534 = {
                        let a2532 = v1;
                        let a2533 = 4u8;
                        fixed_u8::bit_and(a2532, a2533)
                    };
                    let a2535 = 4u8;
                    if equal(a2534, a2535) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2536, a2537)?
            };
            let a2549 = {
                let a2546 = {
                    let a2540 = {
                        let a2538 = v1;
                        let a2539 = 2u8;
                        fixed_u8::bit_and(a2538, a2539)
                    };
                    let a2541 = 2u8;
                    if equal(a2540, a2541) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2547 = {
                    let a2544 = {
                        let a2542 = v1;
                        let a2543 = 1u8;
                        fixed_u8::bit_and(a2542, a2543)
                    };
                    let a2545 = 1u8;
                    if equal(a2544, a2545) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2546, a2547)?
            };
            nat_add(a2548, a2549)?
        };
        nat_add(a2550, a2551)?
    };
    let a2553 = 48u64;
    Ok(nat_sub(a2552, a2553))
}

pub fn f382(v0: Adt14) -> bool {
    f36(f125(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f383(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m2554 = f390(f283(v0.clone()));
    match m2554 {
        None => {
            let m2555 = f130(f122(v0.clone(), f8(Str::lit("$ref"))));
            match m2555 {
                None => {
                    let m2557 = f122(v0.clone(), f8(Str::lit("enum")));
                    match m2557 {
                        None => {
                            let m2558 = f122(v0.clone(), f8(Str::lit("const")));
                            match m2558 {
                                None => {
                                    let v8: Bytes = f125(v0.clone(), f8(Str::lit("type")));
                                    if f36(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f313(f8(Str::lit("Bytes")), List::<Adt21>::nil()))
                                    } else {
                                        if f36(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f313(f392(v0), List::<Adt21>::nil()))
                                        } else {
                                            if f36(v8.clone(), f8(Str::lit("number"))) {
                                                Ok(f313(f393(), List::<Adt21>::nil()))
                                            } else {
                                                if f36(v8.clone(), f8(Str::lit("boolean"))) {
                                                    Ok(f313(f8(Str::lit("Bool")), List::<Adt21>::nil()))
                                                } else {
                                                    if f36(v8.clone(), f8(Str::lit("object"))) {
                                                        Ok(f313(v1.clone(), List::cons(Adt21::C0(v1, v0), List::<Adt21>::nil())))
                                                    } else {
                                                        Ok(f312(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2559 = f130(Some(v6));
                                    match m2559 {
                                        None => {
                                            Ok(f312(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f313(f8(Str::lit("Bytes")), List::<Adt21>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f313(v1.clone(), List::cons(Adt21::C0(v1, v0), List::<Adt21>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2556 = f391(v3)?;
                    match m2556 {
                        None => {
                            Ok(f312(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f313(v4, List::<Adt21>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f312(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f384() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f385(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f346({
        let (_, h2560) = {
            let (h2561, _) = f394(Fn22::F395, {
                let a2563 = {
                    let a2562 = v0.clone();
                    length_list(a2562)
                };
                let a2564 = 1u64;
                nat_add(a2563, a2564)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2561
        };
        h2560
    }))
}

pub fn f386(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2569 = {
            let a2567 = {
                let a2565 = v0.clone();
                length_list(a2565)
            };
            let a2568 = {
                let a2566 = v1.clone();
                length_list(a2566)
            };
            nat_add(a2567, a2568)?
        };
        let a2570 = 1u64;
        nat_add(a2569, a2570)?
    };
    Ok(f262({
        let (_, h2571) = {
            let (_, h2572) = {
                let (h2573, _) = f396(Fn23::F397, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2573
            };
            h2572
        };
        h2571
    }))
}

pub fn f387(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2574 = v0.clone();
        length_bytes(a2574)
    };
    let a2578 = {
        let a2575 = v1;
        let a2576 = 2u64;
        let a2577 = 0u64;
        nat_rem(a2575, a2576, a2577)
    };
    let a2579 = 0u64;
    if nat_eq(a2578, a2579) {
        let (_, h2580) = {
            let (h2581, _) = f398(Fn24::F399(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2581
        };
        Ok(h2580)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f388(v0: u64) -> bool {
    let a2582 = v0;
    let a2583 = 8u64;
    if nat_eq(a2582, a2583) {
        true
    } else {
        let a2584 = v0;
        let a2585 = 9u64;
        if nat_eq(a2584, a2585) {
            true
        } else {
            let a2586 = v0;
            let a2587 = 10u64;
            if nat_eq(a2586, a2587) {
                true
            } else {
                let a2588 = v0;
                let a2589 = 12u64;
                if nat_eq(a2588, a2589) {
                    true
                } else {
                    let a2590 = v0;
                    let a2591 = 13u64;
                    nat_eq(a2590, a2591)
                }
            }
        }
    }
}

pub fn f389(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2636 = {
            let a2612 = {
                let a2600 = {
                    let a2594 = {
                        let a2592 = v1;
                        let a2593 = 128u8;
                        fixed_u8::bit_and(a2592, a2593)
                    };
                    let a2595 = 128u8;
                    if equal(a2594, a2595) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2601 = {
                    let a2598 = {
                        let a2596 = v1;
                        let a2597 = 64u8;
                        fixed_u8::bit_and(a2596, a2597)
                    };
                    let a2599 = 64u8;
                    if equal(a2598, a2599) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2600, a2601)?
            };
            let a2613 = {
                let a2610 = {
                    let a2604 = {
                        let a2602 = v1;
                        let a2603 = 32u8;
                        fixed_u8::bit_and(a2602, a2603)
                    };
                    let a2605 = 32u8;
                    if equal(a2604, a2605) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2611 = {
                    let a2608 = {
                        let a2606 = v1;
                        let a2607 = 16u8;
                        fixed_u8::bit_and(a2606, a2607)
                    };
                    let a2609 = 16u8;
                    if equal(a2608, a2609) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2610, a2611)?
            };
            nat_add(a2612, a2613)?
        };
        let a2637 = {
            let a2634 = {
                let a2622 = {
                    let a2616 = {
                        let a2614 = v1;
                        let a2615 = 8u8;
                        fixed_u8::bit_and(a2614, a2615)
                    };
                    let a2617 = 8u8;
                    if equal(a2616, a2617) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2623 = {
                    let a2620 = {
                        let a2618 = v1;
                        let a2619 = 4u8;
                        fixed_u8::bit_and(a2618, a2619)
                    };
                    let a2621 = 4u8;
                    if equal(a2620, a2621) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2622, a2623)?
            };
            let a2635 = {
                let a2632 = {
                    let a2626 = {
                        let a2624 = v1;
                        let a2625 = 2u8;
                        fixed_u8::bit_and(a2624, a2625)
                    };
                    let a2627 = 2u8;
                    if equal(a2626, a2627) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2633 = {
                    let a2630 = {
                        let a2628 = v1;
                        let a2629 = 1u8;
                        fixed_u8::bit_and(a2628, a2629)
                    };
                    let a2631 = 1u8;
                    if equal(a2630, a2631) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2632, a2633)?
            };
            nat_add(a2634, a2635)?
        };
        nat_add(a2636, a2637)?
    };
    let a2638 = 48u64;
    let a2639 = v2;
    if nat_le(a2638, a2639) {
        let a2640 = v2;
        let a2641 = 57u64;
        Ok(nat_le(a2640, a2641))
    } else {
        Ok(false)
    }
}

pub fn f390(p2647: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2646 = p2647;
    {
        let mut m2646 = m2646;
        loop {
            let step = {
                let v0 = m2646;
                let m2642 = v0;
                match m2642.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f83({
                            let (h2643, _) = v1.clone();
                            h2643
                        }, f400()) {
                            true
                        } else {
                            f83({
                                let (h2644, _) = v1.clone();
                                h2644
                            }, f401())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2645, _) = v1;
                                h2645
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2646 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: Bytes) -> R<Option<Bytes>> {
    if f254(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f182(f32(v0.clone(), 8u64, {
            let a2649 = {
                let a2648 = v0;
                length_bytes(a2648)
            };
            let a2650 = 8u64;
            nat_sub(a2649, a2650)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f392(v0: Adt14) -> Bytes {
    let m2651 = f253(f122(v0, f8(Str::lit("minimum"))));
    match m2651 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f393() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f394(v0: Fn22, p2656: u64, p2657: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2655 = (p2656, p2657);
    {
        let mut m2655 = m2655;
        loop {
            let step = {
                let (v1, v2) = m2655;
                let m2652 = v1;
                if m2652 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2652 - 1;
                    let m2654 = {
                        let c2653: Fn22 = v0.clone();
                        c2653.apply(v2.clone())?
                    };
                    match m2654 {
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
                Ok(next) => { m2655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f402(v0)
}

pub fn f396(v0: Fn23, p2662: u64, p2663: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2661 = (p2662, p2663);
    {
        let mut m2661 = m2661;
        loop {
            let step = {
                let (v1, v2) = m2661;
                let m2658 = v1;
                if m2658 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2658 - 1;
                    let m2660 = {
                        let c2659: Fn23 = v0.clone();
                        c2659.apply(v2.clone())
                    };
                    match m2660 {
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
                Ok(next) => { m2661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f403(v0)
}

pub fn f398(v0: Fn24, p2668: u64, p2669: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2667 = (p2668, p2669);
    {
        let mut m2667 = m2667;
        loop {
            let step = {
                let (v1, v2) = m2667;
                let m2664 = v1;
                if m2664 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2664 - 1;
                    let m2666 = {
                        let c2665: Fn24 = v0.clone();
                        c2665.apply(v2.clone())?
                    };
                    match m2666 {
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
                Ok(next) => { m2667 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f404(v0, v1)
}

pub fn f400() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f401() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f402(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2670, _) = v0;
        h2670
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2671) = v0;
        h2671
    };
    let m2672 = v1;
    match m2672.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2673 = v4;
            match m2673.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f405(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f403(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2674, _) = v0.clone();
        h2674
    };
    let v2: List<(Bytes, ())> = {
        let (h2675, _) = {
            let (_, h2676) = v0.clone();
            h2676
        };
        h2675
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2677) = {
            let (_, h2678) = v0;
            h2678
        };
        h2677
    };
    let m2679 = v1.clone();
    match m2679.uncons() {
        None => {
            let m2680 = v2;
            match m2680.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2681 = v2.clone();
            match m2681.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f406({
                        let (h2682, _) = v8.clone();
                        h2682
                    }, {
                        let (h2683, _) = v6.clone();
                        h2683
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

pub fn f404(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2684, _) = v1;
        h2684
    };
    let m2686 = {
        let (_, h2685) = v1;
        h2685
    };
    match m2686 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2689 = {
                let a2687 = v0.clone();
                let a2688 = v2;
                index_bytes(a2687, a2688)
            };
            match m2689 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2694 = {
                        let a2692 = v0;
                        let a2693 = {
                            let a2690 = v2;
                            let a2691 = 1u64;
                            nat_add(a2690, a2691)?
                        };
                        index_bytes(a2692, a2693)
                    };
                    match m2694 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f407(v4)? {
                                f407(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2695 = v2;
                                    let a2696 = 2u64;
                                    nat_add(a2695, a2696)?
                                }, Some({
                                    let a2701 = v3;
                                    let a2702 = f343(f408(), {
                                        let a2699 = {
                                            let a2697 = f409(v4)?;
                                            let a2698 = 16u64;
                                            nat_mul(a2697, a2698)?
                                        };
                                        let a2700 = f409(v5)?;
                                        nat_add(a2699, a2700)?
                                    });
                                    append_bytes(a2701, a2702)
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

pub fn f405(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a2707 = {
            let a2705 = {
                let a2703 = v0.clone();
                length_list(a2703)
            };
            let a2706 = {
                let a2704 = v1.clone();
                length_list(a2704)
            };
            nat_add(a2705, a2706)?
        };
        let a2708 = 1u64;
        nat_add(a2707, a2708)?
    };
    Ok(f410({
        let (_, h2709) = {
            let (_, h2710) = {
                let (h2711, _) = f411(Fn25::F412, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h2711
            };
            h2710
        };
        h2709
    }))
}

pub fn f406(v0: Bytes, v1: Bytes) -> bool {
    let a2716 = {
        let a2712 = v0;
        let a2713 = v1;
        compare_bytes(a2712, a2713)
    };
    let a2717 = {
        let a2714 = Bytes::lit(&[]);
        let a2715 = Bytes::lit(&[0x01]);
        compare_bytes(a2714, a2715)
    };
    equal(a2716, a2717)
}

pub fn f407(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2762 = {
            let a2738 = {
                let a2726 = {
                    let a2720 = {
                        let a2718 = v1;
                        let a2719 = 128u8;
                        fixed_u8::bit_and(a2718, a2719)
                    };
                    let a2721 = 128u8;
                    if equal(a2720, a2721) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2727 = {
                    let a2724 = {
                        let a2722 = v1;
                        let a2723 = 64u8;
                        fixed_u8::bit_and(a2722, a2723)
                    };
                    let a2725 = 64u8;
                    if equal(a2724, a2725) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2726, a2727)?
            };
            let a2739 = {
                let a2736 = {
                    let a2730 = {
                        let a2728 = v1;
                        let a2729 = 32u8;
                        fixed_u8::bit_and(a2728, a2729)
                    };
                    let a2731 = 32u8;
                    if equal(a2730, a2731) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2737 = {
                    let a2734 = {
                        let a2732 = v1;
                        let a2733 = 16u8;
                        fixed_u8::bit_and(a2732, a2733)
                    };
                    let a2735 = 16u8;
                    if equal(a2734, a2735) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2736, a2737)?
            };
            nat_add(a2738, a2739)?
        };
        let a2763 = {
            let a2760 = {
                let a2748 = {
                    let a2742 = {
                        let a2740 = v1;
                        let a2741 = 8u8;
                        fixed_u8::bit_and(a2740, a2741)
                    };
                    let a2743 = 8u8;
                    if equal(a2742, a2743) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2749 = {
                    let a2746 = {
                        let a2744 = v1;
                        let a2745 = 4u8;
                        fixed_u8::bit_and(a2744, a2745)
                    };
                    let a2747 = 4u8;
                    if equal(a2746, a2747) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2748, a2749)?
            };
            let a2761 = {
                let a2758 = {
                    let a2752 = {
                        let a2750 = v1;
                        let a2751 = 2u8;
                        fixed_u8::bit_and(a2750, a2751)
                    };
                    let a2753 = 2u8;
                    if equal(a2752, a2753) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2759 = {
                    let a2756 = {
                        let a2754 = v1;
                        let a2755 = 1u8;
                        fixed_u8::bit_and(a2754, a2755)
                    };
                    let a2757 = 1u8;
                    if equal(a2756, a2757) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2758, a2759)?
            };
            nat_add(a2760, a2761)?
        };
        nat_add(a2762, a2763)?
    };
    let a2764 = 48u64;
    let a2765 = v2;
    if if nat_le(a2764, a2765) {
        let a2766 = v2;
        let a2767 = 57u64;
        nat_le(a2766, a2767)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2768 = 97u64;
        let a2769 = v2;
        if nat_le(a2768, a2769) {
            let a2770 = v2;
            let a2771 = 102u64;
            Ok(nat_le(a2770, a2771))
        } else {
            Ok(false)
        }
    }
}

pub fn f408() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f409(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2816 = {
            let a2792 = {
                let a2780 = {
                    let a2774 = {
                        let a2772 = v1;
                        let a2773 = 128u8;
                        fixed_u8::bit_and(a2772, a2773)
                    };
                    let a2775 = 128u8;
                    if equal(a2774, a2775) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2781 = {
                    let a2778 = {
                        let a2776 = v1;
                        let a2777 = 64u8;
                        fixed_u8::bit_and(a2776, a2777)
                    };
                    let a2779 = 64u8;
                    if equal(a2778, a2779) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2780, a2781)?
            };
            let a2793 = {
                let a2790 = {
                    let a2784 = {
                        let a2782 = v1;
                        let a2783 = 32u8;
                        fixed_u8::bit_and(a2782, a2783)
                    };
                    let a2785 = 32u8;
                    if equal(a2784, a2785) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2791 = {
                    let a2788 = {
                        let a2786 = v1;
                        let a2787 = 16u8;
                        fixed_u8::bit_and(a2786, a2787)
                    };
                    let a2789 = 16u8;
                    if equal(a2788, a2789) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2790, a2791)?
            };
            nat_add(a2792, a2793)?
        };
        let a2817 = {
            let a2814 = {
                let a2802 = {
                    let a2796 = {
                        let a2794 = v1;
                        let a2795 = 8u8;
                        fixed_u8::bit_and(a2794, a2795)
                    };
                    let a2797 = 8u8;
                    if equal(a2796, a2797) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2803 = {
                    let a2800 = {
                        let a2798 = v1;
                        let a2799 = 4u8;
                        fixed_u8::bit_and(a2798, a2799)
                    };
                    let a2801 = 4u8;
                    if equal(a2800, a2801) {
                        4u64
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
                        let a2805 = 2u8;
                        fixed_u8::bit_and(a2804, a2805)
                    };
                    let a2807 = 2u8;
                    if equal(a2806, a2807) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2813 = {
                    let a2810 = {
                        let a2808 = v1;
                        let a2809 = 1u8;
                        fixed_u8::bit_and(a2808, a2809)
                    };
                    let a2811 = 1u8;
                    if equal(a2810, a2811) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2812, a2813)?
            };
            nat_add(a2814, a2815)?
        };
        nat_add(a2816, a2817)?
    };
    let a2818 = v2;
    let a2819 = 57u64;
    if nat_le(a2818, a2819) {
        let a2820 = v2;
        let a2821 = 48u64;
        Ok(nat_sub(a2820, a2821))
    } else {
        let a2822 = v2;
        let a2823 = 87u64;
        Ok(nat_sub(a2822, a2823))
    }
}

pub fn f410(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f289(Fn10::F413, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f411(v0: Fn25, p2828: u64, p2829: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m2827 = (p2828, p2829);
    {
        let mut m2827 = m2827;
        loop {
            let step = {
                let (v1, v2) = m2827;
                let m2824 = v1;
                if m2824 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m2824 - 1;
                    let m2826 = {
                        let c2825: Fn25 = v0.clone();
                        c2825.apply(v2.clone())
                    };
                    match m2826 {
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
                Ok(next) => { m2827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f414(v0)
}

pub fn f413(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f414(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h2830, _) = v0.clone();
        h2830
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h2831, _) = {
            let (_, h2832) = v0.clone();
            h2832
        };
        h2831
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h2833) = {
            let (_, h2834) = v0;
            h2834
        };
        h2833
    };
    let m2835 = v1.clone();
    match m2835.uncons() {
        None => {
            let m2836 = v2;
            match m2836.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2837 = v2.clone();
            match m2837.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f406({
                        let (h2838, _) = v8.clone();
                        h2838
                    }, {
                        let (h2839, _) = v6.clone();
                        h2839
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
