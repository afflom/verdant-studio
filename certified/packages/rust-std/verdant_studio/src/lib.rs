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
    F240,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F154 => f154(p0, p1),
            Fn1::F172 => f172(p0, p1),
            Fn1::F240 => f240(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F207(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F207(k0) => f207(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F229(Bytes, u64, Bytes, u64),
    F322(Bytes),
    F323(u64, Bytes),
    F394(Bytes),
    F451(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F229(k0, k1, k2, k3) => f229(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F322(k0) => Ok(f322(k0.clone(), p0)),
            Fn3::F323(k0, k1) => f323(*k0, k1.clone(), p0),
            Fn3::F394(k0) => f394(k0.clone(), p0),
            Fn3::F451(k0) => f451(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F273(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt22)) -> R<(u64, Adt22)> {
        match self {
            Fn4::F273(k0, k1, k2) => f273(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F302,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn5::F302 => f302(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F305,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn6::F305 => f305(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F307,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn7::F307 => f307(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F310,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn8::F310 => f310(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F343(Bytes),
    F403(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F343(k0) => f343(k0.clone(), p0),
            Fn9::F403(k0, k1) => Ok(f403(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F345,
    F527,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F345 => f345(p0, p1),
            Fn10::F527 => f527(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F348,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F348 => f348(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F351,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F351 => f351(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F354,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F354 => f354(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F359,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn14::F359 => f359(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F364(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F364(k0) => f364(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F397(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F397(k0) => f397(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F407,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F407 => f407(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F409,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F409 => f409(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F411,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F411 => f411(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F419,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F419 => f419(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F443,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F443 => f443(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F488,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F488 => f488(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F490,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F490 => f490(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F492(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F492(k0) => f492(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F525,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F525 => f525(p0),
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
                let v5: List<Bytes> = f71(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m456 = f59(v0.clone(), f178(v0.clone(), v2.clone(), v5.clone())?);
                match m456.uncons() {
                    None => {
                        let m457 = f179(v0.clone(), v2, v5)?;
                        match m457 {
                            None => {
                                f180(v0, v1)
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

pub fn f85(v0: Bytes, p460: List<Bytes>) -> bool {
    let m459 = p460;
    {
        let mut m459 = m459;
        loop {
            let step = {
                let v1 = m459;
                let m458 = v1;
                match m458.uncons() {
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
                Ok(next) => { m459 = next; }
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
        Adt1::C0(_, h461, _) => {
            h461
        }
    }, f127()) {
        if f75(match v0 {
            Adt1::C0(_, h462, _) => {
                h462
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
    let m463 = v0;
    match m463.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f90(v3, v1))
        }
    }
}

pub fn f91(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m464 = f72(v0.clone(), f88())?;
    match m464.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f181(v0)?), List::<(Bytes, Bytes)>::nil()))
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
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^features/suites/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, features/suites and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f182(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml features/suites README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f95() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, features/suites and\u{a}# README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught when it opens, too.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  pull_request:\u{a}    types: [opened]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  refresh:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply each change to the new main, export it again, and push it to its own pull request\u{a}        if: steps.stale.outputs.stale != ''\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ steps.stale.outputs.stale }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again either way.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml features/suites README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f182(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml features/suites README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f96() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f97(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f101(v0.clone())?;
    let v2: List<Adt14> = f72(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f108(v1.clone(), f109(v0.clone(), f110(v0.clone())?)?);
    let v4: List<List<Bytes>> = f122(f111(f69(v0, f116()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f183(v4.clone(), f121(v4), f184(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f107(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f98(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f185(f101(v0.clone())?, f109(v0.clone(), f110(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f99(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f186(f72(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f187(f72(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f100() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f101(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f70(v0.clone())?, f68(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f71(f72(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f102(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m465 = v0;
    match m465.uncons() {
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
    let m466 = v0;
    match m466.uncons() {
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
    let m467 = v0;
    match m467.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f63(f73(f146(f142(v1, f8(Str::lit("defers"))))), f104(v2))
        }
    }
}

pub fn f105(v0: List<Adt14>) -> List<Bytes> {
    let m468 = v0;
    match m468.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m469 = f142(v1, f8(Str::lit("std")));
            match m469 {
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
    let m470 = v0;
    match m470.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f90(f188(v2.clone()), f189(f184(v0, v1.clone(), v2)?));
    f190(f77(f191(v1))?, v3.clone(), {
        let a472 = {
            let a471 = v3;
            length_list(a471)
        };
        let a473 = 1u64;
        nat_add(a472, a473)?
    })
}

pub fn f108(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m474 = v1;
    match m474.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f192(v2.clone(), f8(Str::lit("subject"))) {
                f193(v0.clone(), v2.clone())
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
    let m475 = v1;
    match m475.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m476 = f130(f82(f69(v0.clone(), v2.clone()))?, f194(v2, f126()))?;
            match m476 {
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
    Ok(f63(f77(f195(f127(), f79(match v0.clone() {
        Adt1::C0(_, h477, _) => {
            h477
        }
    }, f127())))?, f77(f195(f126(), f79(match v0 {
        Adt1::C0(_, h478, _) => {
            h478
        }
    }, f126())))?))
}

pub fn f111(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f196(v0.clone(), 0u64, {
        let a479 = v0;
        length_bytes(a479)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a482 = {
            let a481 = match v1.clone() {
                Adt17::C0(_, _, h480, _, _) => {
                    h480
                }
            };
            length_bytes(a481)
        };
        let a483 = 0u64;
        if if nat_eq(a482, a483) {
            let m485 = match v1.clone() {
                Adt17::C0(_, _, _, h484, _) => {
                    h484
                }
            };
            match m485.uncons() {
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
            f197(v1)
        }
    };
    Ok(f198(match v4 {
        Adt17::C0(_, _, _, _, h486) => {
            h486
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f112(p491: List<Bytes>, p492: List<Bytes>) -> bool {
    let m490 = (p491, p492);
    {
        let mut m490 = m490;
        loop {
            let step = {
                let (v0, v1) = m490;
                let m487 = v0;
                match m487.uncons() {
                    None => {
                        let m488 = v1;
                        match m488.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m489 = v1;
                        match m489.uncons() {
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
                Ok(next) => { m490 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: List<List<Bytes>>) -> List<Bytes> {
    let m493 = v0;
    match m493.uncons() {
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
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f199(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f116() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f117(v0: Bytes) -> R<Bytes> {
    Ok(f200(f201(v0)?))
}

pub fn f118(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f202(f81(f82(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f203(f204(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f119(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f144(Adt14::C3(std::rc::Rc::new(f205(f77(f63(v1, f73(f72(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f120(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m494 = f206(Fn2::F207(v1), None::<Bytes>, v0);
    match m494 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f121(v0: List<List<Bytes>>) -> List<Bytes> {
    let m495 = v0;
    match m495.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m496 = v1;
            match m496.uncons() {
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
    let m497 = v0;
    match m497.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f123(p503: List<Bytes>) -> R<bool> {
    let m502 = p503;
    {
        let mut m502 = m502;
        loop {
            let step = {
                let v0 = m502;
                let m498 = v0;
                match m498.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a500 = {
                            let a499 = f208(v1)?;
                            length_bytes(a499)
                        };
                        let a501 = 0u64;
                        if nat_eq(a500, a501) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m504 = v0;
    match m504.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f208(v1)?;
            let a506 = {
                let a505 = v3.clone();
                length_bytes(a505)
            };
            let a507 = 0u64;
            if nat_eq(a506, a507) {
                f124(v2)
            } else {
                let m508 = f209(v3.clone())?;
                match m508 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f145(v4.clone(), f8(Str::lit("id")));
                        if if f36(f145(v4, f8(Str::lit("schema"))), f149()) {
                            f74({
                                let a510 = {
                                    let a509 = v5.clone();
                                    length_bytes(a509)
                                };
                                let a511 = 0u64;
                                nat_eq(a510, a511)
                            })
                        } else {
                            false
                        } {
                            let m512 = f124(v2)?;
                            match m512 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f210(v5), {
                                        let a513 = v3;
                                        let a514 = Bytes::lit(&[0x0a]);
                                        append_bytes(a513, a514)
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
        let a515 = v0.clone();
        length_bytes(a515)
    };
    let v3: u64 = {
        let a516 = v1.clone();
        length_bytes(a516)
    };
    let a517 = v3;
    let a518 = v2;
    if nat_le(a517, a518) {
        let m524 = {
            let a521 = v0;
            let a522 = {
                let a519 = v2;
                let a520 = v3;
                nat_sub(a519, a520)
            };
            let a523 = v3;
            slice_bytes(a521, a522, a523)
        };
        match m524 {
            None => {
                false
            }
            Some(v4) => {
                let a525 = v4;
                let a526 = v1;
                equal(a525, a526)
            }
        }
    } else {
        false
    }
}

pub fn f130(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m527 = v0;
    match m527.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f208(v2)?;
            let a529 = {
                let a528 = v4.clone();
                length_bytes(a528)
            };
            let a530 = 0u64;
            if nat_eq(a529, a530) {
                f130(v3, v1)
            } else {
                let m531 = f209(v4)?;
                match m531 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f211(v5.clone())
                        } else {
                            f212(v5.clone())
                        } {
                            let m532 = f130(v3, v1)?;
                            match m532 {
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
    if f213(f147(v0.clone())?) {
        Ok(f74({
            let a534 = {
                let a533 = f214(f147(v0)?);
                length_list(a533)
            };
            let a535 = 0u64;
            nat_eq(a534, a535)
        }))
    } else {
        Ok(false)
    }
}

pub fn f132(v0: List<Bytes>) -> R<bool> {
    if f213(f135(v0.clone(), f8(Str::lit("requirement")))?) {
        if f213(f135(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f74({
                let a537 = {
                    let a536 = f139(f214(f135(v0.clone(), f8(Str::lit("requirement")))?), f214(f135(v0, f8(Str::lit("admission")))?));
                    length_list(a536)
                };
                let a538 = 0u64;
                nat_eq(a537, a538)
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
    let m539 = v0;
    match m539.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f208(v2)?;
            let a541 = {
                let a540 = v4.clone();
                length_bytes(a540)
            };
            let a542 = 0u64;
            if nat_eq(a541, a542) {
                f135(v3, v1)
            } else {
                let m543 = f209(v4)?;
                match m543 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f215(v5.clone());
                        let a545 = {
                            let a544 = v6.clone();
                            length_bytes(a544)
                        };
                        let a546 = 0u64;
                        if nat_eq(a545, a546) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m547 = f135(v3, v1.clone())?;
                            match m547 {
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

pub fn f136(p550: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m549 = p550;
    {
        let mut m549 = m549;
        loop {
            let step = {
                let v0 = m549;
                let m548 = v0;
                match m548.uncons() {
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
                Ok(next) => { m549 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f138(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f144(Adt14::C3(std::rc::Rc::new(f216(v1))))?))
}

pub fn f139(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m551 = v0;
    match m551.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f139(v3, v1))
        }
    }
}

pub fn f140(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m552 = v0;
    match m552.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f208(v1)?;
            let a554 = {
                let a553 = v3.clone();
                length_bytes(a553)
            };
            let a555 = 0u64;
            if nat_eq(a554, a555) {
                f140(v2)
            } else {
                let m556 = f209(v3.clone())?;
                match m556 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f217(v4.clone()) {
                            let m557 = f140(v2)?;
                            match m557 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f177(), List::cons(f8(Str::lit("/")), List::cons(f218(f145(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a558 = v3;
                                        let a559 = Bytes::lit(&[0x0a]);
                                        append_bytes(a558, a559)
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
    f209(f69(v0, v1))
}

pub fn f142(p565: Adt14, v1: Bytes) -> Option<Adt14> {
    let m564 = p565;
    {
        let mut m564 = m564;
        loop {
            let step = {
                let v0 = m564;
                let m560 = v0;
                match m560 {
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
                    Adt14::C4(r561) => {
                        let v5 = (*r561).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r562, r563) => {
                        let v3 = (*r562).clone();
                        let v4 = (*r563).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m564 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m566 = f59(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m566.uncons() {
        None => {
            let m568 = f136(f105(List::cons(Adt14::C4(std::rc::Rc::new({
                let m567 = f219(v4.clone());
                match m567 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m568 {
                None => {
                    f220(v0, v1, v2, v3, v4, v5)
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
    let a569 = f221(v0)?;
    let a570 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a569, a570))
}

pub fn f145(v0: Adt14, v1: Bytes) -> Bytes {
    let m571 = f150(f142(v0, v1));
    match m571 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f146(v0: Option<Adt14>) -> List<Adt14> {
    let m572 = v0;
    match m572 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f222(v1)
        }
    }
}

pub fn f147(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m573 = v0;
    match m573.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f208(v1)?;
            let a575 = {
                let a574 = v3.clone();
                length_bytes(a574)
            };
            let a576 = 0u64;
            if nat_eq(a575, a576) {
                f147(v2)
            } else {
                let m577 = f209(v3)?;
                match m577 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f192(v4.clone(), f8(Str::lit("agent"))) {
                            if f192(v4.clone(), f8(Str::lit("at"))) {
                                if f192(v4.clone(), f8(Str::lit("quote"))) {
                                    f223(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m578 = f147(v2)?;
                            match m578 {
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

pub fn f148(p581: List<Adt14>) -> bool {
    let m580 = p581;
    {
        let mut m580 = m580;
        loop {
            let step = {
                let v0 = m580;
                let m579 = v0;
                match m579.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f145(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f85(f8(Str::lit("studio")), f71(f224(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m580 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f150(v0: Option<Adt14>) -> Option<Bytes> {
    let m582 = v0;
    match m582 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m583 = v1;
            match m583 {
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
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f118(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f225(v1, v4)?)?), List::cons(f119(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f152(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h584, _) = v2.clone();
        h584
    };
    let v4: u64 = {
        let (h585, _) = {
            let (_, h586) = v2.clone();
            h586
        };
        h585
    };
    let v5: List<Bytes> = {
        let (_, h587) = {
            let (_, h588) = v2;
            h588
        };
        h587
    };
    let m589 = f226(v0.clone(), v3);
    match m589 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a590 = v6;
            let a591 = v1;
            if equal(a590, a591) {
                Ok(Some(({
                    let a592 = v3;
                    let a593 = 1u64;
                    nat_add(a592, a593)?
                }, ({
                    let a594 = v3;
                    let a595 = 1u64;
                    nat_add(a594, a595)?
                }, List::cons(f32(v0, v4, {
                    let a596 = v3;
                    let a597 = v4;
                    nat_sub(a596, a597)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a598 = v3;
                    let a599 = 1u64;
                    nat_add(a598, a599)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f153(v0: Fn1, p603: List<Bytes>, p604: List<Bytes>) -> List<Bytes> {
    let m602 = (p603, p604);
    {
        let mut m602 = m602;
        loop {
            let step = {
                let (v1, v2) = m602;
                let m600 = v2;
                match m600.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c601: Fn1 = v0.clone();
                            c601.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m602 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f155(p609: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m608 = p609;
    {
        let mut m608 = m608;
        loop {
            let step = {
                let v0 = m608;
                let m605 = v0;
                match m605.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h606, _) = v2;
                            h606
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h607) = v2;
                                h607
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m608 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Adt1, v1: Bytes) -> bool {
    let m611 = f155(match v0 {
        Adt1::C0(h610, _, _) => {
            h610
        }
    }, v1);
    match m611 {
        None => {
            false
        }
        Some(v2) => {
            let m612 = v2;
            match m612 {
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
    let m613 = v1;
    match m613.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f227(v3.clone())
            } else {
                f74(f227(v3.clone()))
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
        let a614 = v0.clone();
        length_bytes(a614)
    };
    let v3: u64 = {
        let a615 = v1.clone();
        length_bytes(a615)
    };
    let v4: u64 = {
        let (h616, _) = f228(Fn3::F229(v1.clone(), v3, v0.clone(), v2), {
            let a617 = v2;
            let a618 = 1u64;
            nat_add(a617, a618)?
        }, 0u64)?;
        h616
    };
    if f230(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f159() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f160(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f231(match v0.clone() {
        Adt12::C0(h619, _, _, _, _) => {
            h619
        }
    }, 0u64)?, f231(match v0.clone() {
        Adt12::C0(_, h620, _, _, _) => {
            h620
        }
    }, 0u64)?, f231(match v0.clone() {
        Adt12::C0(_, _, h621, _, _) => {
            h621
        }
    }, 0u64)?, f231(match v0 {
        Adt12::C0(_, _, _, h622, _) => {
            h622
        }
    }, 0u64)?))
}

pub fn f161(v0: Adt12) -> List<Bytes> {
    f63(match v0.clone() {
        Adt12::C0(h623, _, _, _, _) => {
            h623
        }
    }, f63(match v0.clone() {
        Adt12::C0(_, _, h624, _, _) => {
            h624
        }
    }, f63(match v0.clone() {
        Adt12::C0(_, _, _, h625, _) => {
            h625
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h626) => {
            h626
        }
    })))
}

pub fn f162(v0: Adt16, p632: List<u64>, p633: List<Adt18>) -> Option<u64> {
    let m631 = (p632, p633);
    {
        let mut m631 = m631;
        loop {
            let step = {
                let (v1, v2) = m631;
                let m627 = v2;
                match m627.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f232(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h628) => {
                                h628
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h629, _) => {
                                    h629
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h630, _) => {
                                    h630
                                }
                            }))
                        }
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

pub fn f163(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m634 = v1;
    match m634.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f233(v0.clone(), v3)?), f163(v0, v4, {
                let a635 = v2;
                let a636 = 1u64;
                nat_add(a635, a636)?
            })?))
        }
    }
}

pub fn f164(p640: List<Bytes>, p641: u64) -> Bytes {
    let m639 = (p640, p641);
    {
        let mut m639 = m639;
        loop {
            let step = {
                let (v0, v1) = m639;
                let m637 = v0;
                match m637.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m638 = v1;
                        if m638 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m638 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m639 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f234(v0.clone(), v1.clone()) {
        if f234(v1, v0) {
            f235(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f166(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m642 = v0;
    match m642.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f236(v2, v1.clone())?, f166(v3, v1)?))
        }
    }
}

pub fn f167(v0: List<Adt14>) -> List<u64> {
    let m643 = v0;
    match m643.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f237(v1), f167(v2))
        }
    }
}

pub fn f168(p646: List<u64>, v1: List<u64>) -> Option<u64> {
    let m645 = p646;
    {
        let mut m645 = m645;
        loop {
            let step = {
                let v0 = m645;
                let m644 = v0;
                match m644.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f238(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m645 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(p649: List<Bytes>, p650: List<Bytes>) -> Option<Bytes> {
    let m648 = (p649, p650);
    {
        let mut m648 = m648;
        loop {
            let step = {
                let (v0, v1) = m648;
                let m647 = v0;
                match m647.uncons() {
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
                Ok(next) => { m648 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m651 = v0;
    match m651.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m652 = f239(v2, v1.clone())?;
            match m652 {
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
    f153(Fn1::F240, List::<Bytes>::nil(), v0)
}

pub fn f172(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f241(v0, v1)
}

pub fn f173(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f242(f243(f244(v0))?))
}

pub fn f174(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m653 = f141(v0, v2)?;
    match m653 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f245(v1, f145(v3.clone(), f8(Str::lit("id"))))?;
            let m654 = v4.clone();
            match m654.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m656 = f150(f142({
                        let m655 = f142(v3.clone(), f8(Str::lit("source")));
                        match m655 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m656 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f74(f85(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f199(f246(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f175(p659: List<Bytes>, p660: List<Bytes>) -> List<Bytes> {
    let m658 = (p659, p660);
    {
        let mut m658 = m658;
        loop {
            let step = {
                let (v0, v1) = m658;
                let m657 = v0;
                match m657.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f176(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f247(v0, v1)
}

pub fn f177() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f178(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m661 = v1;
    match m661.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt14 = f248(v0.clone(), v3)?;
            if f85(f249(f145(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f250(v5)?, f178(v0, v4, v2)?))
            } else {
                f178(v0, v4, v2)
            }
        }
    }
}

pub fn f179(v0: Adt1, p665: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m664 = p665;
    {
        let mut m664 = m664;
        loop {
            let step = {
                let v1 = m664;
                let m662 = v1;
                match m662.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt14 = f248(v0.clone(), v3.clone())?;
                        if if f85(f249(f145(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f36(f144({
                                let m663 = f142(v5.clone(), f8(Str::lit("schema")));
                                match m663 {
                                    None => {
                                        Adt14::C5
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f69(v0.clone(), f250(v5)?))
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
                Ok(next) => { m664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f74(f75(match v0.clone() {
        Adt1::C0(_, h666, _) => {
            h666
        }
    }, f133())) {
        Ok(Adt11::C1(List::cons(f133(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f83(f77(f251(f79(match v0.clone() {
            Adt1::C0(_, h667, _) => {
                h667
            }
        }, f133())))?, v1);
        let m668 = f59(v0.clone(), List::cons(f88(), List::cons(f116(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m668.uncons() {
            None => {
                let v5: (bool, Bytes) = f252(v0.clone(), v2)?;
                let (h669, _) = v5;
                if h669 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h670) = v5;
                        h670
                    }))
                } else {
                    let m671 = f87(v0.clone());
                    match m671.uncons() {
                        None => {
                            let m672 = f89(v0.clone())?;
                            match m672.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h673) = v5;
                                        h673
                                    }, List::cons(f7(f77(f253(f101(v0.clone())?, f109(v0.clone(), f110(v0.clone())?)?))?), List::cons(f254(v0)?, List::<Bytes>::nil()))))))
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

pub fn f181(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f255(f72(v0, f88())?)?)))
}

pub fn f182() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f183(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt21>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m674 = v1;
    match m674.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f63(f256(v5.clone(), v2.clone()), f257(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f258(v0.clone(), v5.clone()), List::cons(if f85(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a676 = {
                    let a675 = v7.clone();
                    length_list(a675)
                };
                let a677 = 0u64;
                if nat_eq(a676, a677) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f120(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f183(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f184(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt21>> {
    let v3: List<(Bytes, Bytes)> = f259(v1);
    let v4: List<(Bytes, Bytes)> = f260(v0, v2);
    f261(v3.clone(), f90(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f185(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m678 = v1;
    match m678.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f192(v2.clone(), f8(Str::lit("domain"))) {
                f262(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f145(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f145(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m679 = f142(v2.clone(), f8(Str::lit("role")));
                    match m679 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f145(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f145(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f263(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m680 = f142(v2.clone(), f8(Str::lit("origin")));
                    match m680 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f145(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m681 = f142(v2, f8(Str::lit("evidence")));
                    match m681 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f145(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f185(v0, v3))
            } else {
                f185(v0, v3)
            }
        }
    }
}

pub fn f186(v0: List<Adt14>) -> List<Bytes> {
    let m682 = v0;
    match m682.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f145(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f145(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f145(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f186(v2))
        }
    }
}

pub fn f187(v0: List<Adt14>) -> List<Bytes> {
    let m683 = v0;
    match m683.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f145(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f264(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f187(v2))
        }
    }
}

pub fn f188(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m684 = v0;
    match m684.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f145(v1.clone(), f8(Str::lit("subject"))), f145(v1, f8(Str::lit("object")))), f188(v2))
        }
    }
}

pub fn f189(v0: List<Adt21>) -> List<(Bytes, Bytes)> {
    let m685 = v0;
    match m685.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt21::C0(h686, _, _) => {
                    h686
                }
            }, match v1 {
                Adt21::C0(_, _, h687) => {
                    h687
                }
            }), f189(v2))
        }
    }
}

pub fn f190(p694: List<Bytes>, v1: List<(Bytes, Bytes)>, p695: u64) -> R<List<Bytes>> {
    let m693 = (p694, p695);
    {
        let mut m693 = m693;
        loop {
            let step = {
                let (v0, v2) = m693;
                let m688 = v2;
                if m688 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m688 - 1;
                    let v4: List<Bytes> = f77(f63(v0.clone(), f265(v0.clone(), v1.clone())))?;
                    let a691 = {
                        let a689 = v4.clone();
                        length_list(a689)
                    };
                    let a692 = {
                        let a690 = v0.clone();
                        length_list(a690)
                    };
                    if nat_eq(a691, a692) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
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

pub fn f191(v0: List<Adt14>) -> List<Bytes> {
    let m696 = v0;
    match m696.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m697 = f142(v1, f8(Str::lit("std")));
            match m697 {
                None => {
                    f191(v2)
                }
                Some(v3) => {
                    List::cons(f145(v3.clone(), f8(Str::lit("authority"))), if f192(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f145(v3, f8(Str::lit("row"))), f191(v2))
                    } else {
                        f191(v2)
                    })
                }
            }
        }
    }
}

pub fn f192(v0: Adt14, v1: Bytes) -> bool {
    f74({
        let a699 = {
            let a698 = f145(v0, v1);
            length_bytes(a698)
        };
        let a700 = 0u64;
        nat_eq(a699, a700)
    })
}

pub fn f193(v0: Adt15, v1: Adt14) -> bool {
    if f266(v0.clone(), v1.clone(), f145(v1.clone(), f8(Str::lit("subject")))) {
        f266(v0, v1.clone(), f145(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f194(v0: Bytes, v1: Bytes) -> bool {
    let m705 = {
        let a702 = v0;
        let a703 = 0u64;
        let a704 = {
            let a701 = v1.clone();
            length_bytes(a701)
        };
        slice_bytes(a702, a703, a704)
    };
    match m705 {
        None => {
            false
        }
        Some(v2) => {
            let a706 = v2;
            let a707 = v1;
            equal(a706, a707)
        }
    }
}

pub fn f195(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m708 = v1;
    match m708.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f129(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f125(v0.clone(), v2), f195(v0, v3))
            } else {
                f195(v0, v3)
            }
        }
    }
}

pub fn f196(v0: Bytes, p766: u64, p767: u64, p768: Adt17) -> R<Adt17> {
    let m765 = (p766, (p767, p768));
    {
        let mut m765 = m765;
        loop {
            let step = {
                let (v1, (v2, v3)) = m765;
                let m709 = v2;
                if m709 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m709 - 1;
                    let v5: u64 = f267(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h710, _, _, _) => {
                            h710
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a711 = v1;
                            let a712 = 1u64;
                            nat_add(a711, a712)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h713, _, _) => {
                                h713
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h714, _) => {
                                h714
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h715) => {
                                h715
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h716, _, _, _, _) => {
                                h716
                            }
                        } {
                            let a717 = v5;
                            let a718 = 34u64;
                            if nat_eq(a717, a718) {
                                let a721 = f267(v0.clone(), {
                                    let a719 = v1;
                                    let a720 = 1u64;
                                    nat_add(a719, a720)?
                                })?;
                                let a722 = 34u64;
                                if nat_eq(a721, a722) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a723 = v1;
                                        let a724 = 1u64;
                                        nat_add(a723, a724)?
                                    }, (v4, Adt17::C0(true, true, f268(match v3.clone() {
                                        Adt17::C0(_, _, h725, _, _) => {
                                            h725
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h726, _) => {
                                            h726
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h727) => {
                                            h727
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a728 = v1;
                                        let a729 = 1u64;
                                        nat_add(a728, a729)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h730, _, _) => {
                                            h730
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h731, _) => {
                                            h731
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h732) => {
                                            h732
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a733 = v1;
                                    let a734 = 1u64;
                                    nat_add(a733, a734)?
                                }, (v4, Adt17::C0(true, false, f268(match v3.clone() {
                                    Adt17::C0(_, _, h735, _, _) => {
                                        h735
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h736, _) => {
                                        h736
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h737) => {
                                        h737
                                    }
                                }))))
                            }
                        } else {
                            let a738 = v5;
                            let a739 = 34u64;
                            if nat_eq(a738, a739) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a740 = v1;
                                    let a741 = 1u64;
                                    nat_add(a740, a741)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h742, _, _) => {
                                        h742
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h743, _) => {
                                        h743
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h744) => {
                                        h744
                                    }
                                }))))
                            } else {
                                let a745 = v5;
                                let a746 = 44u64;
                                if nat_eq(a745, a746) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a747 = v1;
                                        let a748 = 1u64;
                                        nat_add(a747, a748)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h749, _, _) => {
                                            h749
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h750, _) => {
                                            h750
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h751) => {
                                            h751
                                        }
                                    }))))
                                } else {
                                    let a752 = v5;
                                    let a753 = 13u64;
                                    if nat_eq(a752, a753) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a754 = v1;
                                            let a755 = 1u64;
                                            nat_add(a754, a755)?
                                        }, (v4, v3)))
                                    } else {
                                        let a756 = v5;
                                        let a757 = 10u64;
                                        if nat_eq(a756, a757) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a758 = v1;
                                                let a759 = 1u64;
                                                nat_add(a758, a759)?
                                            }, (v4, f197(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a760 = v1;
                                                let a761 = 1u64;
                                                nat_add(a760, a761)?
                                            }, (v4, Adt17::C0(false, false, f268(match v3.clone() {
                                                Adt17::C0(_, _, h762, _, _) => {
                                                    h762
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h763, _) => {
                                                    h763
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h764) => {
                                                    h764
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
                Ok(next) => { m765 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f269(List::cons(match v0.clone() {
        Adt17::C0(_, _, h769, _, _) => {
            h769
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h770, _) => {
            h770
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h771) => {
            h771
        }
    }))
}

pub fn f198(p774: List<List<Bytes>>, p775: List<List<Bytes>>) -> List<List<Bytes>> {
    let m773 = (p774, p775);
    {
        let mut m773 = m773;
        loop {
            let step = {
                let (v0, v1) = m773;
                let m772 = v0;
                match m772.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
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

pub fn f199(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f270(v0.clone(), 0u64, {
        let a776 = v0;
        length_bytes(a776)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f200(v0: Adt22) -> Bytes {
    let a797 = f271(match v0.clone() {
        Adt22::C0(h777, _, _, _, _, _, _, _) => {
            h777
        }
    });
    let a798 = {
        let a795 = f271(match v0.clone() {
            Adt22::C0(_, h778, _, _, _, _, _, _) => {
                h778
            }
        });
        let a796 = {
            let a793 = f271(match v0.clone() {
                Adt22::C0(_, _, h779, _, _, _, _, _) => {
                    h779
                }
            });
            let a794 = {
                let a791 = f271(match v0.clone() {
                    Adt22::C0(_, _, _, h780, _, _, _, _) => {
                        h780
                    }
                });
                let a792 = {
                    let a789 = f271(match v0.clone() {
                        Adt22::C0(_, _, _, _, h781, _, _, _) => {
                            h781
                        }
                    });
                    let a790 = {
                        let a787 = f271(match v0.clone() {
                            Adt22::C0(_, _, _, _, _, h782, _, _) => {
                                h782
                            }
                        });
                        let a788 = {
                            let a785 = f271(match v0.clone() {
                                Adt22::C0(_, _, _, _, _, _, h783, _) => {
                                    h783
                                }
                            });
                            let a786 = f271(match v0 {
                                Adt22::C0(_, _, _, _, _, _, _, h784) => {
                                    h784
                                }
                            });
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
    };
    append_bytes(a797, a798)
}

pub fn f201(v0: Bytes) -> R<Adt22> {
    let v1: u64 = {
        let a799 = v0.clone();
        length_bytes(a799)
    };
    let v2: u64 = {
        let a805 = {
            let a802 = {
                let a800 = v1;
                let a801 = 72u64;
                nat_add(a800, a801)?
            };
            let a803 = 64u64;
            let a804 = 0u64;
            nat_quot(a802, a803, a804)
        };
        let a806 = 64u64;
        nat_mul(a805, a806)?
    };
    let (_, h807) = f272(Fn4::F273(v0, v1, v2), {
        let a808 = v2;
        let a809 = 64u64;
        let a810 = 0u64;
        nat_quot(a808, a809, a810)
    }, (0u64, f274()))?;
    Ok(h807)
}

pub fn f202(p813: List<List<Bytes>>, v1: Bytes, p814: List<Bytes>) -> R<List<Bytes>> {
    let m812 = (p813, p814);
    {
        let mut m812 = m812;
        loop {
            let step = {
                let (v0, v2) = m812;
                let m811 = v0;
                match m811.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f275(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f204(f276(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f277(v0)?))
}

pub fn f204(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m815 = v0;
    match m815.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m816 = v3.clone();
            match m816.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a819 = v2;
                    let a820 = {
                        let a817 = v1.clone();
                        let a818 = f204(v3, v1);
                        append_bytes(a817, a818)
                    };
                    append_bytes(a819, a820)
                }
            }
        }
    }
}

pub fn f205(v0: List<Bytes>) -> Adt14 {
    let m821 = v0;
    match m821.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f205(v2)))
        }
    }
}

pub fn f206(v0: Fn2, p825: Option<Bytes>, p826: List<Bytes>) -> Option<Bytes> {
    let m824 = (p825, p826);
    {
        let mut m824 = m824;
        loop {
            let step = {
                let (v1, v2) = m824;
                let m822 = v2;
                match m822.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c823: Fn2 = v0.clone();
                            c823.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m827 = v1;
    match m827 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a830 = {
                    let a828 = v3;
                    let a829 = v0;
                    append_bytes(a828, a829)
                };
                let a831 = v2;
                append_bytes(a830, a831)
            })
        }
    }
}

pub fn f208(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f277(v0.clone())?;
    let v2: u64 = f278(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a832 = v1;
        let a833 = v2;
        nat_sub(a832, a833)
    }))
}

pub fn f209(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f279(v0)?;
    let m837 = f280(v1.clone(), 0u64, {
        let a835 = {
            let a834 = v1.clone();
            length_bytes(a834)
        };
        let a836 = 2u64;
        nat_add(a835, a836)?
    }, 0u64)?;
    match m837 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a840 = {
                let (_, h838) = v2;
                h838
            };
            let a841 = {
                let a839 = v1;
                length_bytes(a839)
            };
            if nat_eq(a840, a841) {
                Ok(Some({
                    let (h842, _) = v2;
                    h842
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f210(v0: Bytes) -> Bytes {
    f7(List::cons(f76(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f211(v0: Adt14) -> bool {
    if f192(v0.clone(), f8(Str::lit("domain"))) {
        if f192(v0.clone(), f8(Str::lit("row"))) {
            if f85(f145(v0.clone(), f8(Str::lit("relation"))), f281()) {
                if f282(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f282(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f282(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f282(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f212(v0: Adt14) -> bool {
    if f192(v0.clone(), f8(Str::lit("subject"))) {
        if f192(v0.clone(), f8(Str::lit("object"))) {
            if f192(v0.clone(), f8(Str::lit("evidence"))) {
                f85(f145(v0, f8(Str::lit("relation"))), f283())
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

pub fn f213(v0: Option<List<Adt14>>) -> bool {
    let m843 = v0;
    match m843 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f214(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m844 = v0;
    match m844 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f215(v0: Adt14) -> Bytes {
    if f192(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f192(v0.clone(), f8(Str::lit("id"))) {
            if f192(v0.clone(), f8(Str::lit("statement"))) {
                if f192(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f223(v0, f8(Str::lit("evidence")))
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

pub fn f216(v0: List<Adt14>) -> Adt14 {
    let m845 = v0;
    match m845.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f216(v2)))
        }
    }
}

pub fn f217(v0: Adt14) -> bool {
    if f192(v0.clone(), f8(Str::lit("name"))) {
        if f223(v0.clone(), f8(Str::lit("schema"))) {
            let m846 = f219(f73(f146(f142(v0, f8(Str::lit("citation"))))));
            match m846 {
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

pub fn f218(v0: Bytes) -> R<Bytes> {
    f284(v0.clone(), 0u64, {
        let a847 = v0;
        length_bytes(a847)
    }, true, Bytes::lit(&[]))
}

pub fn f219(v0: List<Bytes>) -> Option<Adt14> {
    let m848 = v0;
    match m848.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m849 = v2;
                match m849.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m850 = v2;
                    match m850.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m851 = v6;
                                match m851.uncons() {
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
                        let m852 = v2;
                        match m852.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m853 = v10;
                                match m853.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f205(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f220(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m854 = f59(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m854.uncons() {
        None => {
            let m855 = f219(v4);
            match m855 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a856 = f8(Str::lit("Catalog."));
                        let a857 = f218(v3.clone())?;
                        append_bytes(a856, a857)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f218(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m858 = f285(v2, v9.clone(), f218(v3.clone())?)?;
                    match m858 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f218(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f144(Adt14::C3(std::rc::Rc::new(f216(f286(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f144(Adt14::C3(std::rc::Rc::new(f216(f286(f72(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a862 = f287(f69(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a860 = {
                                        let a859 = v10;
                                        length_bytes(a859)
                                    };
                                    let a861 = 4u64;
                                    nat_sub(a860, a861)
                                }))?;
                                let a863 = Bytes::lit(&[0x0a]);
                                append_bytes(a862, a863)
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

pub fn f221(v0: Adt14) -> R<Bytes> {
    let m864 = v0;
    match m864 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f288(v2, v3)
        }
        Adt14::C2(v4) => {
            f289(v4)
        }
        Adt14::C3(r865) => {
            let v5 = (*r865).clone();
            Ok(f290(f291(v5)?))
        }
        Adt14::C4(r866) => {
            let v6 = (*r866).clone();
            f292(f293(v6)?)
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

pub fn f222(v0: Adt14) -> List<Adt14> {
    let m867 = v0;
    match m867 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r868) => {
            let v3 = (*r868).clone();
            f222(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r869, r870) => {
            let v1 = (*r869).clone();
            let v2 = (*r870).clone();
            List::cons(v1, f222(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f223(v0: Adt14, v1: Bytes) -> bool {
    let m871 = f142(v0, v1);
    match m871 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f224(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m872 = f142(v0, v1);
    match m872 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f146(f142(v3, v2))
        }
    }
}

pub fn f225(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f142(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f115(Str::lit("id"), f145(v0.clone(), f8(Str::lit("id"))))?, List::cons(f115(Str::lit("name"), f145(v0.clone(), f8(Str::lit("name"))))?, List::cons(f115(Str::lit("issuer"), f145(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f115(Str::lit("edition"), f145(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f115(Str::lit("canonical_id"), f145(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f115(Str::lit("source_role"), f145(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f115(Str::lit("retrieval_date"), f145(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f115(Str::lit("url"), f294(v2.clone(), f8(Str::lit("url"))))?, List::cons(f115(Str::lit("media_type"), f294(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f115(Str::lit("license"), f294(v2.clone(), f8(Str::lit("license"))))?, List::cons(f115(Str::lit("redistribution"), f294(v2, f8(Str::lit("redistribution"))))?, List::cons(f115(Str::lit("statement"), f145(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m873 = v1;
        match m873 {
            None => {
                f7(List::cons(f115(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f115(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f115(Str::lit("artifact"), {
                    let (h874, _) = v3;
                    h874
                })?, List::cons(f115(Str::lit("checksum"), {
                    let (_, h875) = v3;
                    h875
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f295(f73(f146(f142(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f226(v0: Bytes, v1: u64) -> Option<u8> {
    let a876 = v0;
    let a877 = v1;
    index_bytes(a876, a877)
}

pub fn f227(v0: Bytes) -> bool {
    f129(v0, f8(Str::lit(".lex.tex")))
}

pub fn f228(v0: Fn3, p882: u64, p883: u64) -> R<(u64, bool)> {
    let m881 = (p882, p883);
    {
        let mut m881 = m881;
        loop {
            let step = {
                let (v1, v2) = m881;
                let m878 = v1;
                if m878 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m878 - 1;
                    let m880 = {
                        let c879: Fn3 = v0.clone();
                        c879.apply(v2)?
                    };
                    match m880 {
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
                Ok(next) => { m881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f230(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a886 = {
            let a884 = v4;
            let a885 = v1;
            nat_add(a884, a885)?
        };
        let a887 = v3;
        if nat_lt(a886, a887) {
            Ok(Some({
                let a888 = v4;
                let a889 = 1u64;
                nat_add(a888, a889)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f230(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m894 = {
        let a891 = v0;
        let a892 = v2;
        let a893 = {
            let a890 = v1.clone();
            length_bytes(a890)
        };
        slice_bytes(a891, a892, a893)
    };
    match m894 {
        None => {
            false
        }
        Some(v3) => {
            let a895 = v3;
            let a896 = v1;
            equal(a895, a896)
        }
    }
}

pub fn f231(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m897 = v0;
    match m897.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f231(v3, {
                let a898 = v1;
                let a899 = 1u64;
                nat_add(a898, a899)?
            })?))
        }
    }
}

pub fn f232(v0: Adt16, v1: List<u64>, v2: Adt19) -> bool {
    let m900 = v2;
    match m900 {
        Adt19::C0(v3, v4) => {
            if f238(v3, match v0.clone() {
                Adt16::C0(h901, _, _, _) => {
                    h901
                }
            }) {
                f296(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f238(v5, match v0 {
                Adt16::C0(_, _, h902, _) => {
                    h902
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f234(v7, v1) {
                f238(v8, match v0 {
                    Adt16::C0(_, _, _, h903) => {
                        h903
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f233(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m904 = f142(v1.clone(), f8(Str::lit("std")));
    match m904 {
        None => {
            let m909 = f150(f142(v1.clone(), f8(Str::lit("req"))));
            match m909 {
                None => {
                    let m911 = f142(v1, f8(Str::lit("der")));
                    match m911 {
                        None => {
                            Ok(f297(v0))
                        }
                        Some(v5) => {
                            let m912 = f298(f145(v5.clone(), f8(Str::lit("rule"))));
                            match m912 {
                                None => {
                                    Ok(f297(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f166(f73(f146(f142(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h913) => {
                                            h913
                                        }
                                    })?, f236(f145(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h914, _) => {
                                            h914
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f236(v4, match v0 {
                        Adt12::C0(_, _, h910, _, _) => {
                            h910
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m905 = f150(f142(v2.clone(), f8(Str::lit("row"))));
            match m905 {
                None => {
                    Ok(Adt19::C0(f236(f145(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h906, _, _, _, _) => {
                            h906
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f236(f145(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h907, _, _, _, _) => {
                            h907
                        }
                    })?, Some(f236(v3, match v0 {
                        Adt12::C0(_, h908, _, _, _) => {
                            h908
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f234(p917: List<u64>, v1: List<u64>) -> bool {
    let m916 = p917;
    {
        let mut m916 = m916;
        loop {
            let step = {
                let v0 = m916;
                let m915 = v0;
                match m915.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f238(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(p922: List<u64>) -> bool {
    let m921 = p922;
    {
        let mut m921 = m921;
        loop {
            let step = {
                let v0 = m921;
                let m918 = v0;
                match m918.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a919 = v1;
                        let a920 = 7u64;
                        if nat_le(a919, a920) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f236(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m923 = f299(v0, v1.clone(), 0u64)?;
    match m923 {
        None => {
            let a924 = v1;
            Ok(length_list(a924))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f237(v0: Adt14) -> u64 {
    let m925 = f300(f142(v0, f8(Str::lit("stratum"))));
    match m925 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f238(v0: u64, p930: List<u64>) -> bool {
    let m929 = p930;
    {
        let mut m929 = m929;
        loop {
            let step = {
                let v1 = m929;
                let m926 = v1;
                match m926.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a927 = v2;
                        let a928 = v0;
                        if nat_eq(a927, a928) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m929 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f194(v0.clone(), v1) {
        let m931 = f30(v0, 34u8)?;
        match m931.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m932 = v3;
                match m932.uncons() {
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

pub fn f240(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f241(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m933 = v0.clone();
    match m933.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a934 = v2;
            let a935 = v1.clone();
            if equal(a934, a935) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f242(v0: List<(Bytes, ())>) -> List<Bytes> {
    f171(f301(Fn5::F302, List::<Bytes>::nil(), v0))
}

pub fn f243(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f303(f304(Fn6::F305, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m940 = {
        let (h936, _) = f306(Fn7::F307, {
            let a938 = {
                let a937 = v0;
                length_list(a937)
            };
            let a939 = 1u64;
            nat_add(a938, a939)?
        }, v1)?;
        h936
    };
    match m940.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f244(v0: List<Bytes>) -> List<(Bytes, ())> {
    f308(f309(Fn8::F310, List::<(Bytes, ())>::nil(), v0))
}

pub fn f245(p943: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m942 = p943;
    {
        let mut m942 = m942;
        loop {
            let step = {
                let v0 = m942;
                let m941 = v0;
                match m941.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f275(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
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

pub fn f246(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f294(f142(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f247(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h944, _) = v1.clone();
        h944
    };
    let v3: u64 = {
        let (h945, _) = {
            let (_, h946) = v1.clone();
            h946
        };
        h945
    };
    let v4: List<Bytes> = {
        let (_, h947) = {
            let (_, h948) = v1;
            h948
        };
        h947
    };
    let m949 = f226(v0.clone(), v2);
    match m949 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a950 = v5;
            let a951 = 10u8;
            if equal(a950, a951) {
                Ok(Some(({
                    let a952 = v2;
                    let a953 = 1u64;
                    nat_add(a952, a953)?
                }, ({
                    let a954 = v2;
                    let a955 = 1u64;
                    nat_add(a954, a955)?
                }, List::cons(f311(f32(v0, v3, {
                    let a956 = v2;
                    let a957 = v3;
                    nat_sub(a956, a957)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a958 = v2;
                    let a959 = 1u64;
                    nat_add(a958, a959)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f248(v0: Adt1, v1: Bytes) -> R<Adt14> {
    let m960 = f141(v0, v1)?;
    match m960 {
        None => {
            Ok(Adt14::C5)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f249(v0: Bytes) -> R<Bytes> {
    let a961 = f8(Str::lit("Catalog."));
    let a962 = f218(v0)?;
    Ok(append_bytes(a961, a962))
}

pub fn f250(v0: Adt14) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f218(f145(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f251(v0: List<Bytes>) -> List<Bytes> {
    let m963 = v0;
    match m963.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f129(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f125(f133(), v1), f251(v2))
            } else {
                f251(v2)
            }
        }
    }
}

pub fn f252(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m964 = v1;
    match m964.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f312(v0.clone(), v2)?;
            let (h965, _) = v4.clone();
            if h965 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f252(v0, v3)?;
                let (h966, _) = v5.clone();
                if h966 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a969 = {
                            let (_, h967) = v4;
                            h967
                        };
                        let a970 = {
                            let (_, h968) = v5;
                            h968
                        };
                        append_bytes(a969, a970)
                    }))
                }
            }
        }
    }
}

pub fn f253(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m971 = v1;
    match m971.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f192(v2.clone(), f8(Str::lit("subject"))) {
                if f193(v0.clone(), v2.clone()) {
                    f253(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f145(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f145(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f253(v0, v3))
                }
            } else {
                if f262(v0.clone(), v2.clone()) {
                    f253(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f145(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f263(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f253(v0, v3))
                }
            }
        }
    }
}

pub fn f254(v0: Adt1) -> R<Bytes> {
    let a973 = {
        let a972 = f102(f103(f121(f122(f111(f69(v0.clone(), f116()))?)), f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f105(f72(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a972)
    };
    let a974 = 0u64;
    if nat_eq(a973, a974) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f255(v0: List<Adt14>) -> R<List<Bytes>> {
    let m975 = v0;
    match m975.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f120(f73(f146(f142(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f145(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f145(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f313(f82(f145(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f255(v2)?))
        }
    }
}

pub fn f256(v0: Bytes, v1: List<Adt21>) -> List<Bytes> {
    let m976 = v1;
    match m976.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(match v2.clone() {
                Adt21::C0(h977, _, _) => {
                    h977
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt21::C0(_, h978, _) => {
                        h978
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt21::C0(_, _, h979) => {
                        h979
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f256(v0, v3))
            } else {
                f256(v0, v3)
            }
        }
    }
}

pub fn f257(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m980 = v1;
    match m980.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(f145(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f145(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f257(v0, v3))
            } else {
                if f36(f145(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f145(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f257(v0, v3))
                } else {
                    f257(v0, v3)
                }
            }
        }
    }
}

pub fn f258(p985: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m984 = p985;
    {
        let mut m984 = m984;
        loop {
            let step = {
                let v0 = m984;
                let m981 = v0;
                match m981.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m982 = v2;
                        match m982.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f36(v4, v1.clone()) {
                                    let m983 = v5;
                                    match m983.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f314(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m984 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m986 = v0;
    match m986.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m987 = f142(v1, f8(Str::lit("std")));
            match m987 {
                None => {
                    f259(v2)
                }
                Some(v3) => {
                    if f192(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f145(v3.clone(), f8(Str::lit("row"))), f145(v3, f8(Str::lit("authority")))), f259(v2))
                    } else {
                        f259(v2)
                    }
                }
            }
        }
    }
}

pub fn f260(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m988 = v1;
    match m988.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f145(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f145(v2, f8(Str::lit("object")));
            if if f85(v4.clone(), match v0.clone() {
                Adt15::C0(h989, _, _) => {
                    h989
                }
            }) {
                f85(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h990, _) => {
                        h990
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f260(v0, v3))
            } else {
                if if f85(v5.clone(), match v0.clone() {
                    Adt15::C0(h991, _, _) => {
                        h991
                    }
                }) {
                    f85(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h992, _) => {
                            h992
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f260(v0, v3))
                } else {
                    f260(v0, v3)
                }
            }
        }
    }
}

pub fn f261(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt21>> {
    let m993 = v0;
    match m993.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v4, v5)) => {
            if if f315(v4.clone(), v2.clone()) {
                true
            } else {
                f315(v4.clone(), v3.clone())
            } {
                f261(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt21::C0({
                    let (h994, _) = v4.clone();
                    h994
                }, {
                    let a997 = {
                        let a996 = f77(f316({
                            let (h995, _) = v4.clone();
                            h995
                        }, v1.clone()))?;
                        length_list(a996)
                    };
                    let a998 = 1u64;
                    if nat_eq(a997, a998) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h999) = v4.clone();
                    h999
                }), f261(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f262(v0: Adt15, v1: Adt14) -> bool {
    if f85(f145(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1000, _, _) => {
            h1000
        }
    }) {
        f85(f263(v1), match v0 {
            Adt15::C0(_, h1001, _) => {
                h1001
            }
        })
    } else {
        false
    }
}

pub fn f263(v0: Adt14) -> Bytes {
    let m1002 = f142(v0, f8(Str::lit("evidence")));
    match m1002 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f145(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f264(v0: Adt14) -> Bytes {
    if f192(v0.clone(), f8(Str::lit("req"))) {
        let a1003 = f8(Str::lit("req "));
        let a1004 = f145(v0, f8(Str::lit("req")));
        append_bytes(a1003, a1004)
    } else {
        let m1005 = f142(v0.clone(), f8(Str::lit("std")));
        match m1005 {
            None => {
                let m1008 = f142(v0, f8(Str::lit("der")));
                match m1008 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f145(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f120(f73(f146(f142(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f145(v1.clone(), f8(Str::lit("authority"))), List::cons(if f192(v1.clone(), f8(Str::lit("row"))) {
                    let a1006 = f8(Str::lit(" row "));
                    let a1007 = f145(v1, f8(Str::lit("row")));
                    append_bytes(a1006, a1007)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f265(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1009 = v1;
    match m1009.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85({
                let (h1010, _) = v2.clone();
                h1010
            }, v0.clone()) {
                List::cons({
                    let (_, h1011) = v2;
                    h1011
                }, f265(v0, v3))
            } else {
                if f85({
                    let (_, h1012) = v2;
                    h1012
                }, v0.clone()) {
                    List::cons({
                        let (h1013, _) = v2;
                        h1013
                    }, f265(v0, v3))
                } else {
                    f265(v0, v3)
                }
            }
        }
    }
}

pub fn f266(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f85(v2.clone(), match v0.clone() {
        Adt15::C0(h1014, _, _) => {
            h1014
        }
    }) {
        true
    } else {
        if f85(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1015, _) => {
                h1015
            }
        }) {
            true
        } else {
            if f85(v2, match v0.clone() {
                Adt15::C0(_, _, h1016) => {
                    h1016
                }
            }) {
                true
            } else {
                if f192(v1.clone(), f8(Str::lit("scheme"))) {
                    f85(f145(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1017, _) => {
                            h1017
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f267(v0: Bytes, v1: u64) -> R<u64> {
    let m1020 = {
        let a1018 = v0;
        let a1019 = v1;
        index_bytes(a1018, a1019)
    };
    match m1020 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1065 = {
                let a1041 = {
                    let a1029 = {
                        let a1023 = {
                            let a1021 = v3;
                            let a1022 = 128u8;
                            fixed_u8::bit_and(a1021, a1022)
                        };
                        let a1024 = 128u8;
                        if equal(a1023, a1024) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1030 = {
                        let a1027 = {
                            let a1025 = v3;
                            let a1026 = 64u8;
                            fixed_u8::bit_and(a1025, a1026)
                        };
                        let a1028 = 64u8;
                        if equal(a1027, a1028) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1029, a1030)?
                };
                let a1042 = {
                    let a1039 = {
                        let a1033 = {
                            let a1031 = v3;
                            let a1032 = 32u8;
                            fixed_u8::bit_and(a1031, a1032)
                        };
                        let a1034 = 32u8;
                        if equal(a1033, a1034) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1040 = {
                        let a1037 = {
                            let a1035 = v3;
                            let a1036 = 16u8;
                            fixed_u8::bit_and(a1035, a1036)
                        };
                        let a1038 = 16u8;
                        if equal(a1037, a1038) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1039, a1040)?
                };
                nat_add(a1041, a1042)?
            };
            let a1066 = {
                let a1063 = {
                    let a1051 = {
                        let a1045 = {
                            let a1043 = v3;
                            let a1044 = 8u8;
                            fixed_u8::bit_and(a1043, a1044)
                        };
                        let a1046 = 8u8;
                        if equal(a1045, a1046) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1052 = {
                        let a1049 = {
                            let a1047 = v3;
                            let a1048 = 4u8;
                            fixed_u8::bit_and(a1047, a1048)
                        };
                        let a1050 = 4u8;
                        if equal(a1049, a1050) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1051, a1052)?
                };
                let a1064 = {
                    let a1061 = {
                        let a1055 = {
                            let a1053 = v3;
                            let a1054 = 2u8;
                            fixed_u8::bit_and(a1053, a1054)
                        };
                        let a1056 = 2u8;
                        if equal(a1055, a1056) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1062 = {
                        let a1059 = {
                            let a1057 = v3;
                            let a1058 = 1u8;
                            fixed_u8::bit_and(a1057, a1058)
                        };
                        let a1060 = 1u8;
                        if equal(a1059, a1060) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1061, a1062)?
                };
                nat_add(a1063, a1064)?
            };
            nat_add(a1065, a1066)
        }
    }
}

pub fn f268(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1070 = {
        let a1067 = v1;
        let a1068 = v2;
        let a1069 = 1u64;
        slice_bytes(a1067, a1068, a1069)
    };
    match m1070 {
        None => {
            v0
        }
        Some(v3) => {
            let a1071 = v0;
            let a1072 = v3;
            append_bytes(a1071, a1072)
        }
    }
}

pub fn f269(p1075: List<Bytes>, p1076: List<Bytes>) -> List<Bytes> {
    let m1074 = (p1075, p1076);
    {
        let mut m1074 = m1074;
        loop {
            let step = {
                let (v0, v1) = m1074;
                let m1073 = v0;
                match m1073.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1074 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Bytes, p1148: u64, p1149: u64, p1150: Bytes) -> R<Bytes> {
    let m1147 = (p1148, (p1149, p1150));
    {
        let mut m1147 = m1147;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1147;
                let m1077 = v2;
                if m1077 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1077 - 1;
                    let m1080 = {
                        let a1078 = v0.clone();
                        let a1079 = v1;
                        index_bytes(a1078, a1079)
                    };
                    match m1080 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1125 = {
                                    let a1101 = {
                                        let a1089 = {
                                            let a1083 = {
                                                let a1081 = v6;
                                                let a1082 = 128u8;
                                                fixed_u8::bit_and(a1081, a1082)
                                            };
                                            let a1084 = 128u8;
                                            if equal(a1083, a1084) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1090 = {
                                            let a1087 = {
                                                let a1085 = v6;
                                                let a1086 = 64u8;
                                                fixed_u8::bit_and(a1085, a1086)
                                            };
                                            let a1088 = 64u8;
                                            if equal(a1087, a1088) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1089, a1090)?
                                    };
                                    let a1102 = {
                                        let a1099 = {
                                            let a1093 = {
                                                let a1091 = v6;
                                                let a1092 = 32u8;
                                                fixed_u8::bit_and(a1091, a1092)
                                            };
                                            let a1094 = 32u8;
                                            if equal(a1093, a1094) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1100 = {
                                            let a1097 = {
                                                let a1095 = v6;
                                                let a1096 = 16u8;
                                                fixed_u8::bit_and(a1095, a1096)
                                            };
                                            let a1098 = 16u8;
                                            if equal(a1097, a1098) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1099, a1100)?
                                    };
                                    nat_add(a1101, a1102)?
                                };
                                let a1126 = {
                                    let a1123 = {
                                        let a1111 = {
                                            let a1105 = {
                                                let a1103 = v6;
                                                let a1104 = 8u8;
                                                fixed_u8::bit_and(a1103, a1104)
                                            };
                                            let a1106 = 8u8;
                                            if equal(a1105, a1106) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1112 = {
                                            let a1109 = {
                                                let a1107 = v6;
                                                let a1108 = 4u8;
                                                fixed_u8::bit_and(a1107, a1108)
                                            };
                                            let a1110 = 4u8;
                                            if equal(a1109, a1110) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1111, a1112)?
                                    };
                                    let a1124 = {
                                        let a1121 = {
                                            let a1115 = {
                                                let a1113 = v6;
                                                let a1114 = 2u8;
                                                fixed_u8::bit_and(a1113, a1114)
                                            };
                                            let a1116 = 2u8;
                                            if equal(a1115, a1116) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1122 = {
                                            let a1119 = {
                                                let a1117 = v6;
                                                let a1118 = 1u8;
                                                fixed_u8::bit_and(a1117, a1118)
                                            };
                                            let a1120 = 1u8;
                                            if equal(a1119, a1120) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1121, a1122)?
                                    };
                                    nat_add(a1123, a1124)?
                                };
                                nat_add(a1125, a1126)?
                            };
                            let a1127 = v7;
                            let a1128 = 34u64;
                            if if nat_eq(a1127, a1128) {
                                true
                            } else {
                                let a1129 = v7;
                                let a1130 = 92u64;
                                nat_eq(a1129, a1130)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1131 = v1;
                                    let a1132 = 1u64;
                                    nat_add(a1131, a1132)?
                                }, (v4, {
                                    let a1135 = v3;
                                    let a1136 = {
                                        let a1133 = Bytes::lit(&[0x5c]);
                                        let a1134 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1133, a1134)
                                    };
                                    append_bytes(a1135, a1136)
                                })))
                            } else {
                                let a1137 = v7;
                                let a1138 = 10u64;
                                if nat_eq(a1137, a1138) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1139 = v1;
                                        let a1140 = 1u64;
                                        nat_add(a1139, a1140)?
                                    }, (v4, {
                                        let a1141 = v3;
                                        let a1142 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1141, a1142)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1143 = v1;
                                        let a1144 = 1u64;
                                        nat_add(a1143, a1144)?
                                    }, (v4, {
                                        let a1145 = v3;
                                        let a1146 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1145, a1146)
                                    })))
                                }
                            }
                        }
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

pub fn f271(v0: u32) -> Bytes {
    let a1179 = f317({
        let a1151 = f318(v0, 28u32);
        let a1152 = 15u32;
        fixed_u32::bit_and(a1151, a1152)
    });
    let a1180 = {
        let a1177 = f317({
            let a1153 = f318(v0, 24u32);
            let a1154 = 15u32;
            fixed_u32::bit_and(a1153, a1154)
        });
        let a1178 = {
            let a1175 = f317({
                let a1155 = f318(v0, 20u32);
                let a1156 = 15u32;
                fixed_u32::bit_and(a1155, a1156)
            });
            let a1176 = {
                let a1173 = f317({
                    let a1157 = f318(v0, 16u32);
                    let a1158 = 15u32;
                    fixed_u32::bit_and(a1157, a1158)
                });
                let a1174 = {
                    let a1171 = f317({
                        let a1159 = f318(v0, 12u32);
                        let a1160 = 15u32;
                        fixed_u32::bit_and(a1159, a1160)
                    });
                    let a1172 = {
                        let a1169 = f317({
                            let a1161 = f318(v0, 8u32);
                            let a1162 = 15u32;
                            fixed_u32::bit_and(a1161, a1162)
                        });
                        let a1170 = {
                            let a1167 = f317({
                                let a1163 = f318(v0, 4u32);
                                let a1164 = 15u32;
                                fixed_u32::bit_and(a1163, a1164)
                            });
                            let a1168 = f317({
                                let a1165 = v0;
                                let a1166 = 15u32;
                                fixed_u32::bit_and(a1165, a1166)
                            });
                            append_bytes(a1167, a1168)
                        };
                        append_bytes(a1169, a1170)
                    };
                    append_bytes(a1171, a1172)
                };
                append_bytes(a1173, a1174)
            };
            append_bytes(a1175, a1176)
        };
        append_bytes(a1177, a1178)
    };
    append_bytes(a1179, a1180)
}

pub fn f272(v0: Fn4, p1184: u64, p1185: (u64, Adt22)) -> R<(u64, Adt22)> {
    let m1183 = (p1184, p1185);
    {
        let mut m1183 = m1183;
        loop {
            let step = {
                let (v1, v2) = m1183;
                let m1181 = v1;
                if m1181 == 0 {
                    Err::<(u64, (u64, Adt22)), R<(u64, Adt22)>>(Ok(v2))
                } else {
                    let v3 = m1181 - 1;
                    Ok::<(u64, (u64, Adt22)), R<(u64, Adt22)>>((v3, {
                        let c1182: Fn4 = v0.clone();
                        c1182.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1183 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt22)) -> R<(u64, Adt22)> {
    Ok(({
        let a1187 = {
            let (h1186, _) = v3.clone();
            h1186
        };
        let a1188 = 64u64;
        nat_add(a1187, a1188)?
    }, f319({
        let (_, h1189) = v3;
        h1189
    }, f320(v0, v1, v2, {
        let (h1190, _) = v3;
        h1190
    })?)?))
}

pub fn f274() -> Adt22 {
    Adt22::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f275(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1191 = f30(v1, 34u8)?;
    match m1191.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1192 = v3;
            match m1192.uncons() {
                None => {
                    Ok(f321(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f276(v0: List<Bytes>) -> List<Bytes> {
    let m1193 = v0;
    match m1193.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f276(v2)
            } else {
                List::cons(v1, f276(v2))
            }
        }
    }
}

pub fn f277(v0: Bytes) -> R<u64> {
    let (h1194, _) = f228(Fn3::F322(v0.clone()), {
        let a1195 = v0.clone();
        length_bytes(a1195)
    }, {
        let a1196 = v0;
        length_bytes(a1196)
    })?;
    Ok(h1194)
}

pub fn f278(v0: Bytes, v1: u64) -> R<u64> {
    let (h1197, _) = f228(Fn3::F323(v1, v0), v1, 0u64)?;
    Ok(h1197)
}

pub fn f279(v0: Bytes) -> R<Bytes> {
    f324(v0.clone(), 0u64, {
        let a1198 = v0;
        length_bytes(a1198)
    }, false, false, Bytes::lit(&[]))
}

pub fn f280(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1199 = v2;
    if m1199 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1199 - 1;
        let v5: u64 = f325(v0.clone(), v1)?;
        let a1200 = v3;
        let a1201 = 1u64;
        if nat_eq(a1200, a1201) {
            let m1202 = f280(v0.clone(), v1, v4, 0u64)?;
            match m1202 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f325(v0.clone(), {
                        let (_, h1203) = v6.clone();
                        h1203
                    })?;
                    let a1204 = v7;
                    let a1205 = 44u64;
                    if nat_eq(a1204, a1205) {
                        let m1209 = f280(v0, {
                            let a1207 = {
                                let (_, h1206) = v6;
                                h1206
                            };
                            let a1208 = 1u64;
                            nat_add(a1207, a1208)?
                        }, v4, 1u64)?;
                        match m1209 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1210, _) = v6;
                                    h1210
                                }), std::rc::Rc::new({
                                    let (h1211, _) = v8;
                                    h1211
                                })), {
                                    let (_, h1212) = v8;
                                    h1212
                                })))
                            }
                        }
                    } else {
                        let a1213 = v7;
                        let a1214 = 93u64;
                        if nat_eq(a1213, a1214) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1215, _) = v6;
                                h1215
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1217 = {
                                    let (_, h1216) = v6;
                                    h1216
                                };
                                let a1218 = 1u64;
                                nat_add(a1217, a1218)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1219 = v3;
            let a1220 = 2u64;
            if nat_eq(a1219, a1220) {
                let m1223 = f326(v0.clone(), {
                    let a1221 = v1;
                    let a1222 = 1u64;
                    nat_add(a1221, a1222)?
                })?;
                match m1223 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1224 = v5;
                        let a1225 = 34u64;
                        if if nat_eq(a1224, a1225) {
                            let a1227 = f325(v0.clone(), {
                                let (h1226, _) = v9.clone();
                                h1226
                            })?;
                            let a1228 = 58u64;
                            nat_eq(a1227, a1228)
                        } else {
                            false
                        } {
                            let m1232 = f280(v0.clone(), {
                                let a1230 = {
                                    let (h1229, _) = v9;
                                    h1229
                                };
                                let a1231 = 1u64;
                                nat_add(a1230, a1231)?
                            }, v4, 0u64)?;
                            match m1232 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f325(v0.clone(), {
                                        let (_, h1233) = v10.clone();
                                        h1233
                                    })?;
                                    let a1234 = v11;
                                    let a1235 = 44u64;
                                    if nat_eq(a1234, a1235) {
                                        let m1239 = f280(v0, {
                                            let a1237 = {
                                                let (_, h1236) = v10;
                                                h1236
                                            };
                                            let a1238 = 1u64;
                                            nat_add(a1237, a1238)?
                                        }, v4, 2u64)?;
                                        match m1239 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1240) = v9;
                                                    h1240
                                                }, std::rc::Rc::new({
                                                    let (h1241, _) = v10;
                                                    h1241
                                                }), std::rc::Rc::new({
                                                    let (h1242, _) = v12;
                                                    h1242
                                                })), {
                                                    let (_, h1243) = v12;
                                                    h1243
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1244 = v11;
                                        let a1245 = 125u64;
                                        if nat_eq(a1244, a1245) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1246) = v9;
                                                h1246
                                            }, std::rc::Rc::new({
                                                let (h1247, _) = v10;
                                                h1247
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1249 = {
                                                    let (_, h1248) = v10;
                                                    h1248
                                                };
                                                let a1250 = 1u64;
                                                nat_add(a1249, a1250)?
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
                let a1251 = v5;
                let a1252 = 34u64;
                if nat_eq(a1251, a1252) {
                    let m1255 = f326(v0, {
                        let a1253 = v1;
                        let a1254 = 1u64;
                        nat_add(a1253, a1254)?
                    })?;
                    match m1255 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1256) = v13;
                                h1256
                            }), {
                                let (h1257, _) = v13;
                                h1257
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1258 = v1;
                            let a1259 = 4u64;
                            nat_add(a1258, a1259)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1260 = v1;
                                let a1261 = 5u64;
                                nat_add(a1260, a1261)?
                            })))
                        } else {
                            let a1262 = v5;
                            let a1263 = 91u64;
                            if nat_eq(a1262, a1263) {
                                let a1266 = f325(v0.clone(), {
                                    let a1264 = v1;
                                    let a1265 = 1u64;
                                    nat_add(a1264, a1265)?
                                })?;
                                let a1267 = 93u64;
                                if nat_eq(a1266, a1267) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1268 = v1;
                                        let a1269 = 2u64;
                                        nat_add(a1268, a1269)?
                                    })))
                                } else {
                                    let m1272 = f280(v0, {
                                        let a1270 = v1;
                                        let a1271 = 1u64;
                                        nat_add(a1270, a1271)?
                                    }, v4, 1u64)?;
                                    match m1272 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1273, _) = v14;
                                                h1273
                                            })), {
                                                let (_, h1274) = v14;
                                                h1274
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1275 = v5;
                                let a1276 = 123u64;
                                if nat_eq(a1275, a1276) {
                                    let a1279 = f325(v0.clone(), {
                                        let a1277 = v1;
                                        let a1278 = 1u64;
                                        nat_add(a1277, a1278)?
                                    })?;
                                    let a1280 = 125u64;
                                    if nat_eq(a1279, a1280) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1281 = v1;
                                            let a1282 = 2u64;
                                            nat_add(a1281, a1282)?
                                        })))
                                    } else {
                                        let m1285 = f280(v0, {
                                            let a1283 = v1;
                                            let a1284 = 1u64;
                                            nat_add(a1283, a1284)?
                                        }, v4, 2u64)?;
                                        match m1285 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1286, _) = v15;
                                                    h1286
                                                })), {
                                                    let (_, h1287) = v15;
                                                    h1287
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1288 = f327(v0, v1)?;
                                    match m1288 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1289) = v16;
                                                h1289
                                            }, {
                                                let (h1290, _) = v16;
                                                h1290
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

pub fn f281() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f282(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1291 = f142(v0, v1);
    match m1291 {
        None => {
            false
        }
        Some(v3) => {
            f192(v3, v2)
        }
    }
}

pub fn f283() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f284(v0: Bytes, p1358: u64, p1359: u64, p1360: bool, p1361: Bytes) -> R<Bytes> {
    let m1357 = (p1358, (p1359, (p1360, p1361)));
    {
        let mut m1357 = m1357;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1357;
                let m1292 = v2;
                if m1292 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1292 - 1;
                    let m1295 = {
                        let a1293 = v0.clone();
                        let a1294 = v1;
                        index_bytes(a1293, a1294)
                    };
                    match m1295 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1340 = {
                                    let a1316 = {
                                        let a1304 = {
                                            let a1298 = {
                                                let a1296 = v7;
                                                let a1297 = 128u8;
                                                fixed_u8::bit_and(a1296, a1297)
                                            };
                                            let a1299 = 128u8;
                                            if equal(a1298, a1299) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1305 = {
                                            let a1302 = {
                                                let a1300 = v7;
                                                let a1301 = 64u8;
                                                fixed_u8::bit_and(a1300, a1301)
                                            };
                                            let a1303 = 64u8;
                                            if equal(a1302, a1303) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1304, a1305)?
                                    };
                                    let a1317 = {
                                        let a1314 = {
                                            let a1308 = {
                                                let a1306 = v7;
                                                let a1307 = 32u8;
                                                fixed_u8::bit_and(a1306, a1307)
                                            };
                                            let a1309 = 32u8;
                                            if equal(a1308, a1309) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1315 = {
                                            let a1312 = {
                                                let a1310 = v7;
                                                let a1311 = 16u8;
                                                fixed_u8::bit_and(a1310, a1311)
                                            };
                                            let a1313 = 16u8;
                                            if equal(a1312, a1313) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1314, a1315)?
                                    };
                                    nat_add(a1316, a1317)?
                                };
                                let a1341 = {
                                    let a1338 = {
                                        let a1326 = {
                                            let a1320 = {
                                                let a1318 = v7;
                                                let a1319 = 8u8;
                                                fixed_u8::bit_and(a1318, a1319)
                                            };
                                            let a1321 = 8u8;
                                            if equal(a1320, a1321) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1327 = {
                                            let a1324 = {
                                                let a1322 = v7;
                                                let a1323 = 4u8;
                                                fixed_u8::bit_and(a1322, a1323)
                                            };
                                            let a1325 = 4u8;
                                            if equal(a1324, a1325) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1326, a1327)?
                                    };
                                    let a1339 = {
                                        let a1336 = {
                                            let a1330 = {
                                                let a1328 = v7;
                                                let a1329 = 2u8;
                                                fixed_u8::bit_and(a1328, a1329)
                                            };
                                            let a1331 = 2u8;
                                            if equal(a1330, a1331) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1337 = {
                                            let a1334 = {
                                                let a1332 = v7;
                                                let a1333 = 1u8;
                                                fixed_u8::bit_and(a1332, a1333)
                                            };
                                            let a1335 = 1u8;
                                            if equal(a1334, a1335) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1336, a1337)?
                                    };
                                    nat_add(a1338, a1339)?
                                };
                                nat_add(a1340, a1341)?
                            };
                            if f328(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1342 = v1;
                                    let a1343 = 1u64;
                                    nat_add(a1342, a1343)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f329(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1344 = v1;
                                        let a1345 = 1u64;
                                        nat_add(a1344, a1345)?
                                    }, (v5, (false, {
                                        let a1349 = v4;
                                        let a1350 = f32({
                                            let a1346 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1346)
                                        }, {
                                            let a1347 = v8;
                                            let a1348 = 97u64;
                                            nat_sub(a1347, a1348)
                                        }, 1u64);
                                        append_bytes(a1349, a1350)
                                    }))))
                                } else {
                                    if if f329(v8) {
                                        true
                                    } else {
                                        if f330(v8) {
                                            true
                                        } else {
                                            f331(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1351 = v1;
                                            let a1352 = 1u64;
                                            nat_add(a1351, a1352)?
                                        }, (v5, (false, {
                                            let a1353 = v4;
                                            let a1354 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1353, a1354)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1355 = v1;
                                            let a1356 = 1u64;
                                            nat_add(a1355, a1356)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt14)> = f332(f333(f142(v0.clone(), f8(Str::lit("$defs")))), f333(f142(v0.clone(), f8(Str::lit("definitions")))));
    let m1362 = f334(4096u64, v3.clone(), List::<Bytes>::nil(), f335(v3)?)?;
    match m1362 {
        Ok(v5) => {
            let m1363 = f336(4096u64, f337(f338(v5)?, List::cons(Adt23::C0(v2, v0), List::<Adt23>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1363 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f204(f175(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f286(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1364 = v0;
    match m1364.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f36(f145(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f339(v5, v1, v2))
            } else {
                List::cons(v4, f286(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f287(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f204(f340(f82(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f288(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1365 = 0u64;
        let a1366 = v1;
        nat_lt(a1365, a1366)
    } else {
        false
    } {
        let a1367 = Bytes::lit(&[0x2d]);
        let a1368 = f341(v1)?;
        Ok(append_bytes(a1367, a1368))
    } else {
        f341(v1)
    }
}

pub fn f289(v0: Bytes) -> R<Bytes> {
    let a1374 = {
        let a1372 = Bytes::lit(&[0x22]);
        let a1373 = {
            let (_, h1369) = {
                let (h1370, _) = f342(Fn9::F343(v0.clone()), {
                    let a1371 = v0;
                    length_bytes(a1371)
                }, (0u64, Bytes::lit(&[])))?;
                h1370
            };
            h1369
        };
        append_bytes(a1372, a1373)
    };
    let a1375 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1374, a1375))
}

pub fn f290(v0: List<Bytes>) -> Bytes {
    let a1378 = {
        let a1376 = Bytes::lit(&[0x5b]);
        let a1377 = f120(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1376, a1377)
    };
    let a1379 = Bytes::lit(&[0x5d]);
    append_bytes(a1378, a1379)
}

pub fn f291(v0: Adt14) -> R<List<Bytes>> {
    let m1380 = v0;
    match m1380 {
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
        Adt14::C6(r1381, r1382) => {
            let v7 = (*r1381).clone();
            let v8 = (*r1382).clone();
            Ok(List::cons(f221(v7)?, f291(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f292(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f344(Fn10::F345, List::<(Bytes, Bytes)>::nil(), f346(v0)?);
    let v2: List<Bytes> = f347(Fn11::F348, List::<Bytes>::nil(), v1)?;
    let a1385 = {
        let a1383 = Bytes::lit(&[0x7b]);
        let a1384 = f120(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1383, a1384)
    };
    let a1386 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1385, a1386))
}

pub fn f293(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1387 = v0;
    match m1387 {
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
        Adt14::C7(v9, r1388, r1389) => {
            let v10 = (*r1388).clone();
            let v11 = (*r1389).clone();
            Ok(List::cons((v9, f221(v10)?), f293(v11)?))
        }
    }
}

pub fn f294(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1390 = v0;
    match m1390 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f145(v2, v1)
        }
    }
}

pub fn f295(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f204(f349(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f296(v0: Adt16, v1: Option<u64>) -> bool {
    let m1391 = v1;
    match m1391 {
        None => {
            true
        }
        Some(v2) => {
            f238(v2, match v0 {
                Adt16::C0(_, h1392, _, _) => {
                    h1392
                }
            })
        }
    }
}

pub fn f297(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a1394 = match v0 {
            Adt12::C0(_, _, h1393, _, _) => {
                h1393
            }
        };
        length_list(a1394)
    })
}

pub fn f298(v0: Bytes) -> Option<Adt20> {
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

pub fn f299(v0: Bytes, p1399: List<Bytes>, p1400: u64) -> R<Option<u64>> {
    let m1398 = (p1399, p1400);
    {
        let mut m1398 = m1398;
        loop {
            let step = {
                let (v1, v2) = m1398;
                let m1395 = v1;
                match m1395.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1396 = v2;
                                let a1397 = 1u64;
                                nat_add(a1396, a1397)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: Option<Adt14>) -> Option<u64> {
    let m1401 = v0;
    match m1401 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1402 = v1;
            match m1402 {
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

pub fn f301(v0: Fn5, p1406: List<Bytes>, p1407: List<(Bytes, ())>) -> List<Bytes> {
    let m1405 = (p1406, p1407);
    {
        let mut m1405 = m1405;
        loop {
            let step = {
                let (v1, v2) = m1405;
                let m1403 = v2;
                match m1403.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1404: Fn5 = v0.clone();
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

pub fn f302(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1408, _) = v1;
        h1408
    }, v0)
}

pub fn f303(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f350(Fn12::F351, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f304(v0: Fn6, p1412: List<List<(Bytes, ())>>, p1413: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1411 = (p1412, p1413);
    {
        let mut m1411 = m1411;
        loop {
            let step = {
                let (v1, v2) = m1411;
                let m1409 = v2;
                match m1409.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1410: Fn6 = v0.clone();
                            c1410.apply(v1, v3)
                        }, v4))
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

pub fn f305(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f306(v0: Fn7, p1418: u64, p1419: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1417 = (p1418, p1419);
    {
        let mut m1417 = m1417;
        loop {
            let step = {
                let (v1, v2) = m1417;
                let m1414 = v1;
                if m1414 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1414 - 1;
                    let m1416 = {
                        let c1415: Fn7 = v0.clone();
                        c1415.apply(v2.clone())?
                    };
                    match m1416 {
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
                Ok(next) => { m1417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f307(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f352(v0)
}

pub fn f308(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f353(Fn13::F354, List::<(Bytes, ())>::nil(), v0)
}

pub fn f309(v0: Fn8, p1423: List<(Bytes, ())>, p1424: List<Bytes>) -> List<(Bytes, ())> {
    let m1422 = (p1423, p1424);
    {
        let mut m1422 = m1422;
        loop {
            let step = {
                let (v1, v2) = m1422;
                let m1420 = v2;
                match m1420.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1421: Fn8 = v0.clone();
                            c1421.apply(v1, v3)
                        }, v4))
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

pub fn f310(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f311(v0: Bytes) -> Bytes {
    if f129(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1426 = {
                let a1425 = v0;
                length_bytes(a1425)
            };
            let a1427 = 1u64;
            nat_sub(a1426, a1427)
        })
    } else {
        v0
    }
}

pub fn f312(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f82(f69(v0.clone(), v1.clone()))?;
    let m1428 = f147(v2.clone())?;
    match m1428 {
        None => {
            f356(v0, v1, v2)
        }
        Some(v3) => {
            f355(v0, v1, v3)
        }
    }
}

pub fn f313(v0: List<Bytes>) -> List<Bytes> {
    let m1429 = v0;
    match m1429.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1431 = {
                    let a1430 = v1.clone();
                    length_bytes(a1430)
                };
                let a1432 = 0u64;
                if nat_eq(a1431, a1432) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f313(v2))
        }
    }
}

pub fn f314(v0: List<Bytes>) -> Bytes {
    let m1433 = v0;
    match m1433.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f315(v0: (Bytes, Bytes), p1436: List<(Bytes, Bytes)>) -> bool {
    let m1435 = p1436;
    {
        let mut m1435 = m1435;
        loop {
            let step = {
                let v1 = m1435;
                let m1434 = v1;
                match m1434.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f357(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1437 = v1;
    match m1437.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1438, _) = v2;
                h1438
            }, v0.clone()) {
                List::cons({
                    let (_, h1439) = v2;
                    h1439
                }, f316(v0, v3))
            } else {
                f316(v0, v3)
            }
        }
    }
}

pub fn f317(v0: u32) -> Bytes {
    let a1442 = {
        let a1440 = v0;
        let a1441 = 8u32;
        fixed_u32::bit_and(a1440, a1441)
    };
    let a1443 = 0u32;
    if equal(a1442, a1443) {
        let a1446 = {
            let a1444 = v0;
            let a1445 = 4u32;
            fixed_u32::bit_and(a1444, a1445)
        };
        let a1447 = 0u32;
        if equal(a1446, a1447) {
            let a1450 = {
                let a1448 = v0;
                let a1449 = 2u32;
                fixed_u32::bit_and(a1448, a1449)
            };
            let a1451 = 0u32;
            if equal(a1450, a1451) {
                let a1454 = {
                    let a1452 = v0;
                    let a1453 = 1u32;
                    fixed_u32::bit_and(a1452, a1453)
                };
                let a1455 = 0u32;
                if equal(a1454, a1455) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1458 = {
                    let a1456 = v0;
                    let a1457 = 1u32;
                    fixed_u32::bit_and(a1456, a1457)
                };
                let a1459 = 0u32;
                if equal(a1458, a1459) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1462 = {
                let a1460 = v0;
                let a1461 = 2u32;
                fixed_u32::bit_and(a1460, a1461)
            };
            let a1463 = 0u32;
            if equal(a1462, a1463) {
                let a1466 = {
                    let a1464 = v0;
                    let a1465 = 1u32;
                    fixed_u32::bit_and(a1464, a1465)
                };
                let a1467 = 0u32;
                if equal(a1466, a1467) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1470 = {
                    let a1468 = v0;
                    let a1469 = 1u32;
                    fixed_u32::bit_and(a1468, a1469)
                };
                let a1471 = 0u32;
                if equal(a1470, a1471) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1474 = {
            let a1472 = v0;
            let a1473 = 4u32;
            fixed_u32::bit_and(a1472, a1473)
        };
        let a1475 = 0u32;
        if equal(a1474, a1475) {
            let a1478 = {
                let a1476 = v0;
                let a1477 = 2u32;
                fixed_u32::bit_and(a1476, a1477)
            };
            let a1479 = 0u32;
            if equal(a1478, a1479) {
                let a1482 = {
                    let a1480 = v0;
                    let a1481 = 1u32;
                    fixed_u32::bit_and(a1480, a1481)
                };
                let a1483 = 0u32;
                if equal(a1482, a1483) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1486 = {
                    let a1484 = v0;
                    let a1485 = 1u32;
                    fixed_u32::bit_and(a1484, a1485)
                };
                let a1487 = 0u32;
                if equal(a1486, a1487) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1490 = {
                let a1488 = v0;
                let a1489 = 2u32;
                fixed_u32::bit_and(a1488, a1489)
            };
            let a1491 = 0u32;
            if equal(a1490, a1491) {
                let a1494 = {
                    let a1492 = v0;
                    let a1493 = 1u32;
                    fixed_u32::bit_and(a1492, a1493)
                };
                let a1495 = 0u32;
                if equal(a1494, a1495) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1498 = {
                    let a1496 = v0;
                    let a1497 = 1u32;
                    fixed_u32::bit_and(a1496, a1497)
                };
                let a1499 = 0u32;
                if equal(a1498, a1499) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f318(v0: u32, v1: u32) -> u32 {
    let m1502 = {
        let a1500 = v0;
        let a1501 = v1;
        fixed_u32::shift_right(a1500, a1501)
    };
    match m1502 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f319(v0: Adt22, v1: Adt24) -> R<Adt22> {
    let v2: Adt25 = f358(Fn14::F359, 64u64, Adt25::C0(match v0.clone() {
        Adt22::C0(h1503, _, _, _, _, _, _, _) => {
            h1503
        }
    }, match v0.clone() {
        Adt22::C0(_, h1504, _, _, _, _, _, _) => {
            h1504
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h1505, _, _, _, _, _) => {
            h1505
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h1506, _, _, _, _) => {
            h1506
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h1507, _, _, _) => {
            h1507
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1508, _, _) => {
            h1508
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1509, _) => {
            h1509
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h1510) => {
            h1510
        }
    }, v1, 0u64))?;
    Ok(Adt22::C0(f360(match v0.clone() {
        Adt22::C0(h1511, _, _, _, _, _, _, _) => {
            h1511
        }
    }, match v2.clone() {
        Adt25::C0(h1512, _, _, _, _, _, _, _, _, _) => {
            h1512
        }
    }), f360(match v0.clone() {
        Adt22::C0(_, h1513, _, _, _, _, _, _) => {
            h1513
        }
    }, match v2.clone() {
        Adt25::C0(_, h1514, _, _, _, _, _, _, _, _) => {
            h1514
        }
    }), f360(match v0.clone() {
        Adt22::C0(_, _, h1515, _, _, _, _, _) => {
            h1515
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h1516, _, _, _, _, _, _, _) => {
            h1516
        }
    }), f360(match v0.clone() {
        Adt22::C0(_, _, _, h1517, _, _, _, _) => {
            h1517
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h1518, _, _, _, _, _, _) => {
            h1518
        }
    }), f360(match v0.clone() {
        Adt22::C0(_, _, _, _, h1519, _, _, _) => {
            h1519
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h1520, _, _, _, _, _) => {
            h1520
        }
    }), f360(match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1521, _, _) => {
            h1521
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h1522, _, _, _, _) => {
            h1522
        }
    }), f360(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1523, _) => {
            h1523
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h1524, _, _, _) => {
            h1524
        }
    }), f360(match v0 {
        Adt22::C0(_, _, _, _, _, _, _, h1525) => {
            h1525
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h1526, _, _) => {
            h1526
        }
    })))
}

pub fn f320(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f361(v0.clone(), v1, v2, v3)?, f361(v0.clone(), v1, v2, {
        let a1527 = v3;
        let a1528 = 4u64;
        nat_add(a1527, a1528)?
    })?, f361(v0.clone(), v1, v2, {
        let a1529 = v3;
        let a1530 = 8u64;
        nat_add(a1529, a1530)?
    })?, f361(v0.clone(), v1, v2, {
        let a1531 = v3;
        let a1532 = 12u64;
        nat_add(a1531, a1532)?
    })?, f361(v0.clone(), v1, v2, {
        let a1533 = v3;
        let a1534 = 16u64;
        nat_add(a1533, a1534)?
    })?, f361(v0.clone(), v1, v2, {
        let a1535 = v3;
        let a1536 = 20u64;
        nat_add(a1535, a1536)?
    })?, f361(v0.clone(), v1, v2, {
        let a1537 = v3;
        let a1538 = 24u64;
        nat_add(a1537, a1538)?
    })?, f361(v0.clone(), v1, v2, {
        let a1539 = v3;
        let a1540 = 28u64;
        nat_add(a1539, a1540)?
    })?, f361(v0.clone(), v1, v2, {
        let a1541 = v3;
        let a1542 = 32u64;
        nat_add(a1541, a1542)?
    })?, f361(v0.clone(), v1, v2, {
        let a1543 = v3;
        let a1544 = 36u64;
        nat_add(a1543, a1544)?
    })?, f361(v0.clone(), v1, v2, {
        let a1545 = v3;
        let a1546 = 40u64;
        nat_add(a1545, a1546)?
    })?, f361(v0.clone(), v1, v2, {
        let a1547 = v3;
        let a1548 = 44u64;
        nat_add(a1547, a1548)?
    })?, f361(v0.clone(), v1, v2, {
        let a1549 = v3;
        let a1550 = 48u64;
        nat_add(a1549, a1550)?
    })?, f361(v0.clone(), v1, v2, {
        let a1551 = v3;
        let a1552 = 52u64;
        nat_add(a1551, a1552)?
    })?, f361(v0.clone(), v1, v2, {
        let a1553 = v3;
        let a1554 = 56u64;
        nat_add(a1553, a1554)?
    })?, f361(v0, v1, v2, {
        let a1555 = v3;
        let a1556 = 60u64;
        nat_add(a1555, a1556)?
    })?))
}

pub fn f321(p1559: List<Bytes>, v1: Bytes) -> bool {
    let m1558 = p1559;
    {
        let mut m1558 = m1558;
        loop {
            let step = {
                let v0 = m1558;
                let m1557 = v0;
                match m1557.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f194(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
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

pub fn f322(v0: Bytes, v1: u64) -> Option<u64> {
    let a1560 = 0u64;
    let a1561 = v1;
    if nat_lt(a1560, a1561) {
        if f362(v0, {
            let a1562 = v1;
            let a1563 = 1u64;
            nat_sub(a1562, a1563)
        }) {
            Some({
                let a1564 = v1;
                let a1565 = 1u64;
                nat_sub(a1564, a1565)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f323(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1566 = v2;
    let a1567 = v0;
    if nat_lt(a1566, a1567) {
        if f362(v1, v2) {
            Ok(Some({
                let a1568 = v2;
                let a1569 = 1u64;
                nat_add(a1568, a1569)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f324(v0: Bytes, p1649: u64, p1650: u64, p1651: bool, p1652: bool, p1653: Bytes) -> R<Bytes> {
    let m1648 = (p1649, (p1650, (p1651, (p1652, p1653))));
    {
        let mut m1648 = m1648;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1648;
                let m1570 = v2;
                if m1570 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1570 - 1;
                    let m1573 = {
                        let a1571 = v0.clone();
                        let a1572 = v1;
                        index_bytes(a1571, a1572)
                    };
                    match m1573 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1618 = {
                                    let a1594 = {
                                        let a1582 = {
                                            let a1576 = {
                                                let a1574 = v8;
                                                let a1575 = 128u8;
                                                fixed_u8::bit_and(a1574, a1575)
                                            };
                                            let a1577 = 128u8;
                                            if equal(a1576, a1577) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1583 = {
                                            let a1580 = {
                                                let a1578 = v8;
                                                let a1579 = 64u8;
                                                fixed_u8::bit_and(a1578, a1579)
                                            };
                                            let a1581 = 64u8;
                                            if equal(a1580, a1581) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1582, a1583)?
                                    };
                                    let a1595 = {
                                        let a1592 = {
                                            let a1586 = {
                                                let a1584 = v8;
                                                let a1585 = 32u8;
                                                fixed_u8::bit_and(a1584, a1585)
                                            };
                                            let a1587 = 32u8;
                                            if equal(a1586, a1587) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1593 = {
                                            let a1590 = {
                                                let a1588 = v8;
                                                let a1589 = 16u8;
                                                fixed_u8::bit_and(a1588, a1589)
                                            };
                                            let a1591 = 16u8;
                                            if equal(a1590, a1591) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1592, a1593)?
                                    };
                                    nat_add(a1594, a1595)?
                                };
                                let a1619 = {
                                    let a1616 = {
                                        let a1604 = {
                                            let a1598 = {
                                                let a1596 = v8;
                                                let a1597 = 8u8;
                                                fixed_u8::bit_and(a1596, a1597)
                                            };
                                            let a1599 = 8u8;
                                            if equal(a1598, a1599) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1605 = {
                                            let a1602 = {
                                                let a1600 = v8;
                                                let a1601 = 4u8;
                                                fixed_u8::bit_and(a1600, a1601)
                                            };
                                            let a1603 = 4u8;
                                            if equal(a1602, a1603) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1604, a1605)?
                                    };
                                    let a1617 = {
                                        let a1614 = {
                                            let a1608 = {
                                                let a1606 = v8;
                                                let a1607 = 2u8;
                                                fixed_u8::bit_and(a1606, a1607)
                                            };
                                            let a1609 = 2u8;
                                            if equal(a1608, a1609) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1615 = {
                                            let a1612 = {
                                                let a1610 = v8;
                                                let a1611 = 1u8;
                                                fixed_u8::bit_and(a1610, a1611)
                                            };
                                            let a1613 = 1u8;
                                            if equal(a1612, a1613) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1614, a1615)?
                                    };
                                    nat_add(a1616, a1617)?
                                };
                                nat_add(a1618, a1619)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1620 = v1;
                                    let a1621 = 1u64;
                                    nat_add(a1620, a1621)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1622 = v9;
                                    let a1623 = 34u64;
                                    !nat_eq(a1622, a1623)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1624 = v9;
                                    let a1625 = 92u64;
                                    nat_eq(a1624, a1625)
                                }, {
                                    let a1626 = v5;
                                    let a1627 = v10;
                                    append_bytes(a1626, a1627)
                                })))))
                            } else {
                                let a1628 = v9;
                                let a1629 = 32u64;
                                if if if if nat_eq(a1628, a1629) {
                                    true
                                } else {
                                    let a1630 = v9;
                                    let a1631 = 9u64;
                                    nat_eq(a1630, a1631)
                                } {
                                    true
                                } else {
                                    let a1632 = v9;
                                    let a1633 = 10u64;
                                    nat_eq(a1632, a1633)
                                } {
                                    true
                                } else {
                                    let a1634 = v9;
                                    let a1635 = 13u64;
                                    nat_eq(a1634, a1635)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1636 = v1;
                                        let a1637 = 1u64;
                                        nat_add(a1636, a1637)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1638 = v1;
                                            let a1639 = 4u64;
                                            nat_add(a1638, a1639)?
                                        }, (v6, (false, (false, {
                                            let a1640 = v5;
                                            let a1641 = f8(Str::lit("[]"));
                                            append_bytes(a1640, a1641)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1642 = v1;
                                            let a1643 = 1u64;
                                            nat_add(a1642, a1643)?
                                        }, (v6, ({
                                            let a1644 = v9;
                                            let a1645 = 34u64;
                                            nat_eq(a1644, a1645)
                                        }, (false, {
                                            let a1646 = v5;
                                            let a1647 = v10;
                                            append_bytes(a1646, a1647)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1648 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f325(v0: Bytes, v1: u64) -> R<u64> {
    let m1656 = {
        let a1654 = v0;
        let a1655 = v1;
        index_bytes(a1654, a1655)
    };
    match m1656 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1701 = {
                let a1677 = {
                    let a1665 = {
                        let a1659 = {
                            let a1657 = v3;
                            let a1658 = 128u8;
                            fixed_u8::bit_and(a1657, a1658)
                        };
                        let a1660 = 128u8;
                        if equal(a1659, a1660) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1666 = {
                        let a1663 = {
                            let a1661 = v3;
                            let a1662 = 64u8;
                            fixed_u8::bit_and(a1661, a1662)
                        };
                        let a1664 = 64u8;
                        if equal(a1663, a1664) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1665, a1666)?
                };
                let a1678 = {
                    let a1675 = {
                        let a1669 = {
                            let a1667 = v3;
                            let a1668 = 32u8;
                            fixed_u8::bit_and(a1667, a1668)
                        };
                        let a1670 = 32u8;
                        if equal(a1669, a1670) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1676 = {
                        let a1673 = {
                            let a1671 = v3;
                            let a1672 = 16u8;
                            fixed_u8::bit_and(a1671, a1672)
                        };
                        let a1674 = 16u8;
                        if equal(a1673, a1674) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1675, a1676)?
                };
                nat_add(a1677, a1678)?
            };
            let a1702 = {
                let a1699 = {
                    let a1687 = {
                        let a1681 = {
                            let a1679 = v3;
                            let a1680 = 8u8;
                            fixed_u8::bit_and(a1679, a1680)
                        };
                        let a1682 = 8u8;
                        if equal(a1681, a1682) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1688 = {
                        let a1685 = {
                            let a1683 = v3;
                            let a1684 = 4u8;
                            fixed_u8::bit_and(a1683, a1684)
                        };
                        let a1686 = 4u8;
                        if equal(a1685, a1686) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1687, a1688)?
                };
                let a1700 = {
                    let a1697 = {
                        let a1691 = {
                            let a1689 = v3;
                            let a1690 = 2u8;
                            fixed_u8::bit_and(a1689, a1690)
                        };
                        let a1692 = 2u8;
                        if equal(a1691, a1692) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1698 = {
                        let a1695 = {
                            let a1693 = v3;
                            let a1694 = 1u8;
                            fixed_u8::bit_and(a1693, a1694)
                        };
                        let a1696 = 1u8;
                        if equal(a1695, a1696) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1697, a1698)?
                };
                nat_add(a1699, a1700)?
            };
            nat_add(a1701, a1702)
        }
    }
}

pub fn f326(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f363(Fn15::F364(v0.clone()), {
        let a1704 = {
            let a1703 = v0;
            length_bytes(a1703)
        };
        let a1705 = 1u64;
        nat_add(a1704, a1705)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1709 = {
        let (_, h1706) = {
            let (_, h1707) = {
                let (h1708, _) = v2.clone();
                h1708
            };
            h1707
        };
        h1706
    };
    let a1710 = 1u64;
    if nat_eq(a1709, a1710) {
        Ok(Some(({
            let (h1711, _) = {
                let (h1712, _) = v2.clone();
                h1712
            };
            h1711
        }, {
            let (h1713, _) = {
                let (_, h1714) = {
                    let (h1715, _) = v2;
                    h1715
                };
                h1714
            };
            h1713
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f327(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1716 = f325(v0.clone(), v1)?;
        let a1717 = 45u64;
        nat_eq(a1716, a1717)
    };
    let v3: u64 = if v2 {
        let a1718 = v1;
        let a1719 = 1u64;
        nat_add(a1718, a1719)?
    } else {
        v1
    };
    let v4: u64 = f365(v0.clone(), v3)?;
    let a1722 = 19u64;
    let a1723 = {
        let a1720 = v4;
        let a1721 = v3;
        nat_sub(a1720, a1721)
    };
    if nat_lt(a1722, a1723) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1726 = f366(f32(v0, v3, {
            let a1724 = v4;
            let a1725 = v3;
            nat_sub(a1724, a1725)
        }))?;
        match m1726 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f367(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f328(v0: u64) -> bool {
    let a1727 = v0;
    let a1728 = 95u64;
    if nat_eq(a1727, a1728) {
        true
    } else {
        let a1729 = v0;
        let a1730 = 45u64;
        if nat_eq(a1729, a1730) {
            true
        } else {
            let a1731 = v0;
            let a1732 = 32u64;
            if nat_eq(a1731, a1732) {
                true
            } else {
                let a1733 = v0;
                let a1734 = 46u64;
                if nat_eq(a1733, a1734) {
                    true
                } else {
                    let a1735 = v0;
                    let a1736 = 47u64;
                    nat_eq(a1735, a1736)
                }
            }
        }
    }
}

pub fn f329(v0: u64) -> bool {
    let a1737 = 97u64;
    let a1738 = v0;
    if nat_le(a1737, a1738) {
        let a1739 = v0;
        let a1740 = 122u64;
        nat_le(a1739, a1740)
    } else {
        false
    }
}

pub fn f330(v0: u64) -> bool {
    let a1741 = 65u64;
    let a1742 = v0;
    if nat_le(a1741, a1742) {
        let a1743 = v0;
        let a1744 = 90u64;
        nat_le(a1743, a1744)
    } else {
        false
    }
}

pub fn f331(v0: u64) -> bool {
    let a1745 = 48u64;
    let a1746 = v0;
    if nat_le(a1745, a1746) {
        let a1747 = v0;
        let a1748 = 57u64;
        nat_le(a1747, a1748)
    } else {
        false
    }
}

pub fn f332(v0: List<(Bytes, Adt14)>, v1: List<(Bytes, Adt14)>) -> List<(Bytes, Adt14)> {
    let m1749 = v0;
    match m1749.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f332(v3, v1))
        }
    }
}

pub fn f333(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1750 = v0;
    match m1750 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f368(v1)
        }
    }
}

pub fn f334(v0: u64, v1: List<(Bytes, Adt14)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt14)>, Bytes>> {
    let m1751 = v0;
    if m1751 == 0 {
        let m1752 = v1;
        match m1752.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt14)>, Bytes>(List::<(Bytes, Adt14)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt14)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m1751 - 1;
        let m1753 = v1.clone();
        match m1753.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt14)>, Bytes>(List::<(Bytes, Adt14)>::nil()))
            }
            Some((_, _)) => {
                let m1754 = f369(v1.clone(), v2.clone(), v3.clone())?;
                match m1754 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt14)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f204(f335(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m1757 = f334(v6, f370(v1, {
                            let (h1755, _) = v9.clone();
                            h1755
                        }), List::cons(f218({
                            let (h1756, _) = v9.clone();
                            h1756
                        })?, v2), v3)?;
                        match m1757 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt14)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt14)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f335(v0: List<(Bytes, Adt14)>) -> R<List<Bytes>> {
    let m1758 = v0;
    match m1758.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f218({
                let (h1759, _) = v1;
                h1759
            })?, f335(v2)?))
        }
    }
}

pub fn f336(p1767: u64, p1768: List<Adt23>, p1769: List<Bytes>, p1770: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1766 = (p1767, (p1768, (p1769, p1770)));
    {
        let mut m1766 = m1766;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1766;
                let m1760 = v0;
                if m1760 == 0 {
                    Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1760 - 1;
                    let m1761 = v1;
                    match m1761.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1762 = v5;
                            match m1762 {
                                Adt23::C0(v8, v9) => {
                                    if f36(v8.clone(), f371()) {
                                        Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f85(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1763 = f373(v8.clone(), v9)?;
                                            match m1763 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f337({
                                                        let (_, h1764) = v11;
                                                        h1764
                                                    }, List::cons(Adt23::C1({
                                                        let (h1765, _) = v11;
                                                        h1765
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
                                    if f85(f371(), v2.clone()) {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f371(), v2), List::cons(f372(), v3)))))
                                    }
                                }
                            }
                        }
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

pub fn f337(v0: List<Adt23>, v1: List<Adt23>) -> List<Adt23> {
    let m1771 = v0;
    match m1771.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f337(v3, v1))
        }
    }
}

pub fn f338(v0: List<(Bytes, Adt14)>) -> R<List<Adt23>> {
    let m1772 = v0;
    match m1772.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt23::C0(f218({
                let (h1773, _) = v1;
                h1773
            })?, {
                let (_, h1774) = v1;
                h1774
            }), f338(v2)?))
        }
    }
}

pub fn f339(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1775 = v0;
    match m1775.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f36(f145(v3.clone(), v1.clone()), v2.clone()) {
                f339(v4, v1, v2)
            } else {
                List::cons(v3, f339(v4, v1, v2))
            }
        }
    }
}

pub fn f340(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1776 = v0;
    match m1776.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f194(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f204(f349(f77(List::cons(v1, f374(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f340(v3, v1)?))
            }
        }
    }
}

pub fn f341(v0: u64) -> R<Bytes> {
    f375(f376(), 10u64, v0)
}

pub fn f342(v0: Fn9, p1781: u64, p1782: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1780 = (p1781, p1782);
    {
        let mut m1780 = m1780;
        loop {
            let step = {
                let (v1, v2) = m1780;
                let m1777 = v1;
                if m1777 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1777 - 1;
                    let m1779 = {
                        let c1778: Fn9 = v0.clone();
                        c1778.apply(v2.clone())?
                    };
                    match m1779 {
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
                Ok(next) => { m1780 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f377(v0, v1)
}

pub fn f344(v0: Fn10, p1786: List<(Bytes, Bytes)>, p1787: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1785 = (p1786, p1787);
    {
        let mut m1785 = m1785;
        loop {
            let step = {
                let (v1, v2) = m1785;
                let m1783 = v2;
                match m1783.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1784: Fn10 = v0.clone();
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

pub fn f345(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f378(v0, v1)
}

pub fn f346(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f379(v0)
}

pub fn f347(v0: Fn11, p1791: List<Bytes>, p1792: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1790 = (p1791, p1792);
    {
        let mut m1790 = m1790;
        loop {
            let step = {
                let (v1, v2) = m1790;
                let m1788 = v2;
                match m1788.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1789: Fn11 = v0.clone();
                            c1789.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1790 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f348(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f380(v1)?, v0))
}

pub fn f349(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1793 = v0;
    match m1793.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f199(v1)?, f349(v2)?))
        }
    }
}

pub fn f350(v0: Fn12, p1797: List<List<(Bytes, ())>>, p1798: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1796 = (p1797, p1798);
    {
        let mut m1796 = m1796;
        loop {
            let step = {
                let (v1, v2) = m1796;
                let m1794 = v2;
                match m1794.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1795: Fn12 = v0.clone();
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

pub fn f351(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f352(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1800 = {
        let a1799 = v0.clone();
        length_list(a1799)
    };
    let a1801 = 1u64;
    if nat_le(a1800, a1801) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f381(v0)?))
    }
}

pub fn f353(v0: Fn13, p1805: List<(Bytes, ())>, p1806: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1804 = (p1805, p1806);
    {
        let mut m1804 = m1804;
        loop {
            let step = {
                let (v1, v2) = m1804;
                let m1802 = v2;
                match m1802.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1803: Fn13 = v0.clone();
                            c1803.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1804 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f355(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f71(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f382(v3.clone(), f71(f72(v0, f88())?, f8(Str::lit("at"))));
    let a1808 = {
        let a1807 = v4.clone();
        length_list(a1807)
    };
    let a1809 = 0u64;
    if nat_eq(a1808, a1809) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1812 = {
            let a1810 = v4;
            length_list(a1810)
        };
        let a1813 = {
            let a1811 = v3;
            length_list(a1811)
        };
        if nat_eq(a1812, a1813) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f383(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f356(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f214(f135(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f214(f135(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f63(f71(v3.clone(), f8(Str::lit("id"))), f71(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f71(f72(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f382(v5.clone(), f63(v6.clone(), f71(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1815 = {
        let a1814 = v7.clone();
        length_list(a1814)
    };
    let a1816 = 0u64;
    if nat_eq(a1815, a1816) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f74({
            let a1819 = {
                let a1817 = v7;
                length_list(a1817)
            };
            let a1820 = {
                let a1818 = v5;
                length_list(a1818)
            };
            nat_eq(a1819, a1820)
        }) {
            Ok((false, f383(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f77(f63(f382(f384(v4.clone()), f68(f69(v0, f8(Str::lit("model/authorities.toml"))))?), f382(f385(v4), f63(v6, f71(v3, f8(Str::lit("id")))))))?;
            let a1822 = {
                let a1821 = v8.clone();
                length_list(a1821)
            };
            let a1823 = 0u64;
            if nat_eq(a1822, a1823) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f383(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f120(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f357(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h1824, _) = v0;
        h1824
    }, {
        let (h1825, _) = v1;
        h1825
    }) {
        f36({
            let (_, h1826) = v0;
            h1826
        }, {
            let (_, h1827) = v1;
            h1827
        })
    } else {
        false
    }
}

pub fn f358(v0: Fn14, p1831: u64, p1832: Adt25) -> R<Adt25> {
    let m1830 = (p1831, p1832);
    {
        let mut m1830 = m1830;
        loop {
            let step = {
                let (v1, v2) = m1830;
                let m1828 = v1;
                if m1828 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m1828 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c1829: Fn14 = v0.clone();
                        c1829.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1830 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: Adt25) -> R<Adt25> {
    f386(v0)
}

pub fn f360(v0: u32, v1: u32) -> u32 {
    let m1835 = {
        let a1833 = f387(v0);
        let a1834 = f387(v1);
        fixed_u64::checked_add(a1833, a1834)
    };
    match m1835 {
        None => {
            0u32
        }
        Some(v2) => {
            f388(v2)
        }
    }
}

pub fn f361(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1838 = {
        let a1836 = v3;
        let a1837 = 8u64;
        nat_add(a1836, a1837)?
    };
    let a1839 = v2;
    if nat_eq(a1838, a1839) {
        Ok(f389({
            let a1842 = {
                let a1840 = v1;
                let a1841 = 8u64;
                nat_mul(a1840, a1841)?
            };
            let a1843 = 4294967296u64;
            let a1844 = 0u64;
            nat_quot(a1842, a1843, a1844)
        }))
    } else {
        let a1847 = {
            let a1845 = v3;
            let a1846 = 4u64;
            nat_add(a1845, a1846)?
        };
        let a1848 = v2;
        if nat_eq(a1847, a1848) {
            Ok(f389({
                let a1851 = {
                    let a1849 = v1;
                    let a1850 = 8u64;
                    nat_mul(a1849, a1850)?
                };
                let a1852 = 4294967296u64;
                let a1853 = 0u64;
                nat_rem(a1851, a1852, a1853)
            }))
        } else {
            Ok(f390(f391(v0.clone(), v1, v3), f391(v0.clone(), v1, {
                let a1854 = v3;
                let a1855 = 1u64;
                nat_add(a1854, a1855)?
            }), f391(v0.clone(), v1, {
                let a1856 = v3;
                let a1857 = 2u64;
                nat_add(a1856, a1857)?
            }), f391(v0, v1, {
                let a1858 = v3;
                let a1859 = 3u64;
                nat_add(a1858, a1859)?
            })))
        }
    }
}

pub fn f362(v0: Bytes, v1: u64) -> bool {
    let m1860 = f226(v0, v1);
    match m1860 {
        None => {
            false
        }
        Some(v2) => {
            f392(v2)
        }
    }
}

pub fn f363(v0: Fn15, p1865: u64, p1866: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1864 = (p1865, p1866);
    {
        let mut m1864 = m1864;
        loop {
            let step = {
                let (v1, v2) = m1864;
                let m1861 = v1;
                if m1861 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1861 - 1;
                    let m1863 = {
                        let c1862: Fn15 = v0.clone();
                        c1862.apply(v2.clone())?
                    };
                    match m1863 {
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
                Ok(next) => { m1864 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f393(v0, v1)
}

pub fn f365(v0: Bytes, v1: u64) -> R<u64> {
    let (h1867, _) = f228(Fn3::F394(v0.clone()), {
        let a1869 = {
            let a1868 = v0;
            length_bytes(a1868)
        };
        let a1870 = 1u64;
        nat_add(a1869, a1870)?
    }, v1)?;
    Ok(h1867)
}

pub fn f366(v0: Bytes) -> R<Option<u64>> {
    if f395(v0.clone())? {
        Ok(Some({
            let (_, h1871) = {
                let (h1872, _) = f396(Fn16::F397(v0.clone()), {
                    let a1873 = v0;
                    length_bytes(a1873)
                }, (0u64, 0u64))?;
                h1872
            };
            h1871
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f367(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1874 = 0u64;
        let a1875 = v1;
        if nat_lt(a1874, a1875) {
            let a1876 = v1;
            let a1877 = 9223372036854775808u64;
            nat_le(a1876, a1877)
        } else {
            false
        }
    } else {
        let a1878 = v1;
        let a1879 = 9223372036854775807u64;
        nat_le(a1878, a1879)
    }
}

pub fn f368(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1880 = v0;
    match m1880 {
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
        Adt14::C4(r1881) => {
            let v4 = (*r1881).clone();
            f368(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1882, r1883) => {
            let v2 = (*r1882).clone();
            let v3 = (*r1883).clone();
            List::cons((v1, v2), f368(v3))
        }
    }
}

pub fn f369(p1888: List<(Bytes, Adt14)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt14)>> {
    let m1887 = p1888;
    {
        let mut m1887 = m1887;
        loop {
            let step = {
                let v0 = m1887;
                let m1884 = v0;
                match m1884.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, R<Option<(Bytes, Adt14)>>>(Ok(None::<(Bytes, Adt14)>))
                    }
                    Some((v3, v4)) => {
                        if f398(f399({
                            let (_, h1885) = v3.clone();
                            h1885
                        }, List::<Bytes>::nil())?, f218({
                            let (h1886, _) = v3.clone();
                            h1886
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt14)>, R<Option<(Bytes, Adt14)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, R<Option<(Bytes, Adt14)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f370(v0: List<(Bytes, Adt14)>, v1: Bytes) -> List<(Bytes, Adt14)> {
    let m1889 = v0;
    match m1889.uncons() {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1890, _) = v2.clone();
                h1890
            }, v1.clone()) {
                f370(v3, v1)
            } else {
                List::cons(v2, f370(v3, v1))
            }
        }
    }
}

pub fn f371() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f372() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f373(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    f400(v0, f401(v1))
}

pub fn f374(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1891 = v0;
    match m1891.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f374(v3, false))
            } else {
                f374(v3, true)
            }
        }
    }
}

pub fn f375(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1892 = v2;
    let a1893 = 0u64;
    if nat_eq(a1892, a1893) {
        Ok(f402(v0, 0u64))
    } else {
        let (_, h1894) = {
            let (h1895, _) = f342(Fn9::F403(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1895
        };
        Ok(h1894)
    }
}

pub fn f376() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f377(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1899 = {
        let a1897 = v0.clone();
        let a1898 = {
            let (h1896, _) = v1.clone();
            h1896
        };
        index_bytes(a1897, a1898)
    };
    match m1899 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1901 = {
                    let (h1900, _) = v1.clone();
                    h1900
                };
                let a1902 = 1u64;
                nat_add(a1901, a1902)?
            }, {
                let a1905 = {
                    let (_, h1903) = v1;
                    h1903
                };
                let a1906 = f404(v0, {
                    let (h1904, _) = v1;
                    h1904
                }, v2)?;
                append_bytes(a1905, a1906)
            })))
        }
    }
}

pub fn f378(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1907 = v0.clone();
    match m1907.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1910 = {
                let (h1908, _) = v2;
                h1908
            };
            let a1911 = {
                let (h1909, _) = v1.clone();
                h1909
            };
            if equal(a1910, a1911) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f379(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f405(f406(Fn17::F407, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1916 = {
        let (h1912, _) = f408(Fn18::F409, {
            let a1914 = {
                let a1913 = v0;
                length_list(a1913)
            };
            let a1915 = 1u64;
            nat_add(a1914, a1915)?
        }, v1)?;
        h1912
    };
    match m1916.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f380(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1921 = {
        let a1918 = f289({
            let (h1917, _) = v0;
            h1917
        })?;
        let a1919 = Bytes::lit(&[0x3a]);
        append_bytes(a1918, a1919)
    };
    let a1922 = {
        let (_, h1920) = v0;
        h1920
    };
    Ok(append_bytes(a1921, a1922))
}

pub fn f381(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f303({
        let (_, h1923) = {
            let (h1924, _) = f410(Fn19::F411, {
                let a1926 = {
                    let a1925 = v0.clone();
                    length_list(a1925)
                };
                let a1927 = 1u64;
                nat_add(a1926, a1927)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1924
        };
        h1923
    }))
}

pub fn f382(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1928 = v0;
    match m1928.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f382(v3, v1)
            } else {
                List::cons(v2, f382(v3, v1))
            }
        }
    }
}

pub fn f383(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f384(v0: List<Adt14>) -> List<Bytes> {
    let m1929 = v0;
    match m1929.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1930 = f142(v1, f8(Str::lit("std")));
            match m1930 {
                None => {
                    f384(v2)
                }
                Some(v3) => {
                    List::cons(f145(v3, f8(Str::lit("authority"))), f384(v2))
                }
            }
        }
    }
}

pub fn f385(v0: List<Adt14>) -> List<Bytes> {
    let m1931 = v0;
    match m1931.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1932 = f150(f142(v1, f8(Str::lit("req"))));
            match m1932 {
                None => {
                    f385(v2)
                }
                Some(v3) => {
                    List::cons(v3, f385(v2))
                }
            }
        }
    }
}

pub fn f386(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f412(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h1933) => {
            h1933
        }
    });
    let v2: u32 = f360(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h1934, _, _) => {
            h1934
        }
    }, f360(f413(match v0.clone() {
        Adt25::C0(_, _, _, _, h1935, _, _, _, _, _) => {
            h1935
        }
    }), f360(f414(match v0.clone() {
        Adt25::C0(_, _, _, _, h1936, _, _, _, _, _) => {
            h1936
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1937, _, _, _, _) => {
            h1937
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1938, _, _, _) => {
            h1938
        }
    }), f360(v1, {
        let m1941 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h1940, _) => {
                h1940
            }
        };
        match m1941 {
            Adt24::C0(h1939, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1939
            }
        }
    }))));
    let v3: u32 = f360(f415(match v0.clone() {
        Adt25::C0(h1942, _, _, _, _, _, _, _, _, _) => {
            h1942
        }
    }), f416(match v0.clone() {
        Adt25::C0(h1943, _, _, _, _, _, _, _, _, _) => {
            h1943
        }
    }, match v0.clone() {
        Adt25::C0(_, h1944, _, _, _, _, _, _, _, _) => {
            h1944
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1945, _, _, _, _, _, _, _) => {
            h1945
        }
    }));
    Ok(Adt25::C0(f360(v2, v3), match v0.clone() {
        Adt25::C0(h1946, _, _, _, _, _, _, _, _, _) => {
            h1946
        }
    }, match v0.clone() {
        Adt25::C0(_, h1947, _, _, _, _, _, _, _, _) => {
            h1947
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1948, _, _, _, _, _, _, _) => {
            h1948
        }
    }, f360(match v0.clone() {
        Adt25::C0(_, _, _, h1949, _, _, _, _, _, _) => {
            h1949
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h1950, _, _, _, _, _) => {
            h1950
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1951, _, _, _, _) => {
            h1951
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1952, _, _, _) => {
            h1952
        }
    }, f417(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h1953, _) => {
            h1953
        }
    }), {
        let a1955 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h1954) => {
                h1954
            }
        };
        let a1956 = 1u64;
        nat_add(a1955, a1956)?
    }))
}

pub fn f387(v0: u32) -> u64 {
    let m1958 = {
        let a1957 = v0;
        fixed_u64::convert(i128::from(a1957))
    };
    match m1958 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f388(v0: u64) -> u32 {
    let m1962 = {
        let a1961 = {
            let a1959 = v0;
            let a1960 = 4294967295u64;
            fixed_u64::bit_and(a1959, a1960)
        };
        fixed_u32::convert(i128::from(a1961))
    };
    match m1962 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f389(v0: u64) -> u32 {
    let (_, h1963) = {
        let (_, h1964) = f418(Fn20::F419, 32u64, (v0, (1u32, 0u32)));
        h1964
    };
    h1963
}

pub fn f390(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1969 = f420(v0, 24u32);
    let a1970 = {
        let a1967 = f420(v1, 16u32);
        let a1968 = {
            let a1965 = f420(v2, 8u32);
            let a1966 = v3;
            fixed_u32::bit_or(a1965, a1966)
        };
        fixed_u32::bit_or(a1967, a1968)
    };
    fixed_u32::bit_or(a1969, a1970)
}

pub fn f391(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1971 = v2;
    let a1972 = v1;
    if nat_lt(a1971, a1972) {
        let m1975 = {
            let a1973 = v0;
            let a1974 = v2;
            index_bytes(a1973, a1974)
        };
        match m1975 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1977 = {
                    let a1976 = v3;
                    fixed_u32::convert(i128::from(a1976))
                };
                match m1977 {
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
        let a1978 = v2;
        let a1979 = v1;
        if nat_eq(a1978, a1979) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f392(v0: u8) -> bool {
    let a1980 = v0;
    let a1981 = 32u8;
    if if if equal(a1980, a1981) {
        true
    } else {
        let a1982 = v0;
        let a1983 = 9u8;
        equal(a1982, a1983)
    } {
        true
    } else {
        let a1984 = v0;
        let a1985 = 10u8;
        equal(a1984, a1985)
    } {
        true
    } else {
        let a1986 = v0;
        let a1987 = 12u8;
        if equal(a1986, a1987) {
            true
        } else {
            let a1988 = v0;
            let a1989 = 13u8;
            equal(a1988, a1989)
        }
    }
}

pub fn f393(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1990, _) = v1.clone();
        h1990
    };
    let v3: Bytes = {
        let (h1991, _) = {
            let (_, h1992) = v1.clone();
            h1992
        };
        h1991
    };
    let a1995 = {
        let (_, h1993) = {
            let (_, h1994) = v1;
            h1994
        };
        h1993
    };
    let a1996 = 0u64;
    if nat_eq(a1995, a1996) {
        let v4: u64 = f325(v0.clone(), v2)?;
        let a1997 = v4;
        let a1998 = 34u64;
        if nat_eq(a1997, a1998) {
            Ok(Some(({
                let a1999 = v2;
                let a2000 = 1u64;
                nat_add(a1999, a2000)?
            }, (v3, 1u64))))
        } else {
            let a2001 = v4;
            let a2002 = 92u64;
            if nat_eq(a2001, a2002) {
                f421(v0, v2, v3)
            } else {
                let a2003 = v4;
                let a2004 = 32u64;
                if if nat_lt(a2003, a2004) {
                    true
                } else {
                    let a2005 = v4;
                    let a2006 = 256u64;
                    nat_eq(a2005, a2006)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2007 = v2;
                        let a2008 = 1u64;
                        nat_add(a2007, a2008)?
                    }, ({
                        let a2009 = v3;
                        let a2010 = f32(v0, v2, 1u64);
                        append_bytes(a2009, a2010)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f394(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f422(v0, v1)
}

pub fn f395(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2011 = v0.clone();
        length_bytes(a2011)
    };
    let a2012 = 0u64;
    let a2013 = v1;
    if nat_lt(a2012, a2013) {
        let a2014 = f423(v0.clone())?;
        let a2015 = v1;
        if nat_eq(a2014, a2015) {
            let a2016 = 1u64;
            let a2017 = v1;
            if nat_lt(a2016, a2017) {
                let m2020 = {
                    let a2018 = v0;
                    let a2019 = 0u64;
                    index_bytes(a2018, a2019)
                };
                match m2020 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2021 = v2;
                        let a2022 = 48u8;
                        Ok(!equal(a2021, a2022))
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

pub fn f396(v0: Fn16, p2027: u64, p2028: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2026 = (p2027, p2028);
    {
        let mut m2026 = m2026;
        loop {
            let step = {
                let (v1, v2) = m2026;
                let m2023 = v1;
                if m2023 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2023 - 1;
                    let m2025 = {
                        let c2024: Fn16 = v0.clone();
                        c2024.apply(v2)?
                    };
                    match m2025 {
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
                Ok(next) => { m2026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f424(v0, v1)
}

pub fn f398(p2031: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2030 = p2031;
    {
        let mut m2030 = m2030;
        loop {
            let step = {
                let v0 = m2030;
                let m2029 = v0;
                match m2029.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f36(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f85(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f85(v4, v3.clone())
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
                Ok(next) => { m2030 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(v0: Adt14, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2032 = v0;
    match m2032 {
        Adt14::C0(_) => {
            Ok(v1)
        }
        Adt14::C1(_, _) => {
            Ok(v1)
        }
        Adt14::C2(_) => {
            Ok(v1)
        }
        Adt14::C3(r2035) => {
            let v2 = (*r2035).clone();
            f399(v2, v1)
        }
        Adt14::C4(r2036) => {
            let v3 = (*r2036).clone();
            f399(v3, v1)
        }
        Adt14::C5 => {
            Ok(v1)
        }
        Adt14::C6(r2037, r2038) => {
            let v4 = (*r2037).clone();
            let v5 = (*r2038).clone();
            f399(v4, f399(v5, v1)?)
        }
        Adt14::C7(v6, r2039, r2040) => {
            let v7 = (*r2039).clone();
            let v8 = (*r2040).clone();
            let v9: List<Bytes> = f399(v7.clone(), f399(v8, v1)?)?;
            if f36(v6, f8(Str::lit("$ref"))) {
                let m2033 = f150(Some(v7));
                match m2033 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2034 = f425(v10)?;
                        match m2034 {
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

pub fn f400(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2041 = f142(v1.clone(), f8(Str::lit("enum")));
    match m2041 {
        None => {
            let m2043 = f142(v1.clone(), f8(Str::lit("oneOf")));
            match m2043 {
                None => {
                    if f430(v1.clone(), v0.clone())? {
                        let m2044 = f431(v0.clone(), f333(f142(v1.clone(), f8(Str::lit("properties")))), f73(f146(f142(v1, f8(Str::lit("required"))))), true)?;
                        match m2044 {
                            Ok(v7) => {
                                Ok(f428(f432(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f204({
                                    let (h2045, _) = v7;
                                    h2045
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h2046) = v7;
                                    h2046
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt23>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m2047 = f333(f142(v1.clone(), f8(Str::lit("properties"))));
                        match m2047.uncons() {
                            None => {
                                let m2050 = f433(v1, {
                                    let a2048 = v0.clone();
                                    let a2049 = f8(Str::lit("Value"));
                                    append_bytes(a2048, a2049)
                                })?;
                                match m2050 {
                                    Ok(v9) => {
                                        Ok(f428(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h2051, _) = v9;
                                            h2051
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h2052) = v9;
                                            h2052
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt23>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2053 = f431(v0.clone(), f333(f142(v1.clone(), f8(Str::lit("properties")))), f73(f146(f142(v1, f8(Str::lit("required"))))), false)?;
                                match m2053 {
                                    Ok(v13) => {
                                        Ok(f428(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2055 = {
                                                let (h2054, _) = v13.clone();
                                                h2054
                                            };
                                            match m2055.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f204({
                                                        let (h2056, _) = v13;
                                                        h2056
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h2057) = v13;
                                            h2057
                                        }))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<(Bytes, List<Adt23>), Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f429(v0, f222(v5))
                }
            }
        }
        Some(v2) => {
            let m2042 = f426(f222(v2), List::<Bytes>::nil())?;
            match m2042 {
                Ok(v4) => {
                    Ok(f428(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f204(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt23>::nil()))
                }
                Err(v3) => {
                    Ok(f427(v3, v0))
                }
            }
        }
    }
}

pub fn f401(v0: Adt14) -> Adt14 {
    let v2: Adt14 = {
        let m2058 = f142(v0.clone(), f8(Str::lit("anyOf")));
        match m2058 {
            None => {
                v0
            }
            Some(v1) => {
                if f434(f222(v1.clone())) {
                    f435(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f436(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f437(f222(v1.clone()))
                    } {
                        v0
                    } else {
                        f438(f435(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt14 = {
        let m2059 = f142(v2.clone(), f8(Str::lit("oneOf")));
        match m2059 {
            None => {
                v2
            }
            Some(v3) => {
                if f434(f222(v3)) {
                    f435(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2060 = f142(v4.clone(), f8(Str::lit("allOf")));
    match m2060 {
        None => {
            v4
        }
        Some(v5) => {
            if f439(f222(v5)) {
                f435(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f402(v0: Bytes, v1: u64) -> Bytes {
    let m2064 = {
        let a2061 = v0;
        let a2062 = v1;
        let a2063 = 1u64;
        slice_bytes(a2061, a2062, a2063)
    };
    match m2064 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f403(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f440(v1, v0, v2)
}

pub fn f404(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2109 = {
            let a2085 = {
                let a2073 = {
                    let a2067 = {
                        let a2065 = v3;
                        let a2066 = 128u8;
                        fixed_u8::bit_and(a2065, a2066)
                    };
                    let a2068 = 128u8;
                    if equal(a2067, a2068) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2074 = {
                    let a2071 = {
                        let a2069 = v3;
                        let a2070 = 64u8;
                        fixed_u8::bit_and(a2069, a2070)
                    };
                    let a2072 = 64u8;
                    if equal(a2071, a2072) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2073, a2074)?
            };
            let a2086 = {
                let a2083 = {
                    let a2077 = {
                        let a2075 = v3;
                        let a2076 = 32u8;
                        fixed_u8::bit_and(a2075, a2076)
                    };
                    let a2078 = 32u8;
                    if equal(a2077, a2078) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2084 = {
                    let a2081 = {
                        let a2079 = v3;
                        let a2080 = 16u8;
                        fixed_u8::bit_and(a2079, a2080)
                    };
                    let a2082 = 16u8;
                    if equal(a2081, a2082) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2083, a2084)?
            };
            nat_add(a2085, a2086)?
        };
        let a2110 = {
            let a2107 = {
                let a2095 = {
                    let a2089 = {
                        let a2087 = v3;
                        let a2088 = 8u8;
                        fixed_u8::bit_and(a2087, a2088)
                    };
                    let a2090 = 8u8;
                    if equal(a2089, a2090) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2096 = {
                    let a2093 = {
                        let a2091 = v3;
                        let a2092 = 4u8;
                        fixed_u8::bit_and(a2091, a2092)
                    };
                    let a2094 = 4u8;
                    if equal(a2093, a2094) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2095, a2096)?
            };
            let a2108 = {
                let a2105 = {
                    let a2099 = {
                        let a2097 = v3;
                        let a2098 = 2u8;
                        fixed_u8::bit_and(a2097, a2098)
                    };
                    let a2100 = 2u8;
                    if equal(a2099, a2100) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2106 = {
                    let a2103 = {
                        let a2101 = v3;
                        let a2102 = 1u8;
                        fixed_u8::bit_and(a2101, a2102)
                    };
                    let a2104 = 1u8;
                    if equal(a2103, a2104) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2105, a2106)?
            };
            nat_add(a2107, a2108)?
        };
        nat_add(a2109, a2110)?
    };
    let a2111 = v4;
    let a2112 = 34u64;
    if nat_eq(a2111, a2112) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2113 = v4;
        let a2114 = 92u64;
        if nat_eq(a2113, a2114) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2115 = v4;
            let a2116 = 8u64;
            if nat_eq(a2115, a2116) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2117 = v4;
                let a2118 = 12u64;
                if nat_eq(a2117, a2118) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2119 = v4;
                    let a2120 = 10u64;
                    if nat_eq(a2119, a2120) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2121 = v4;
                        let a2122 = 13u64;
                        if nat_eq(a2121, a2122) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2123 = v4;
                            let a2124 = 9u64;
                            if nat_eq(a2123, a2124) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2125 = v4;
                                let a2126 = 32u64;
                                if nat_lt(a2125, a2126) {
                                    let a2127 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2128 = f441(v2)?;
                                    Ok(append_bytes(a2127, a2128))
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

pub fn f405(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f442(Fn21::F443, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f406(v0: Fn17, p2132: List<List<(Bytes, Bytes)>>, p2133: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2131 = (p2132, p2133);
    {
        let mut m2131 = m2131;
        loop {
            let step = {
                let (v1, v2) = m2131;
                let m2129 = v2;
                match m2129.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2130: Fn17 = v0.clone();
                            c2130.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f408(v0: Fn18, p2138: u64, p2139: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2137 = (p2138, p2139);
    {
        let mut m2137 = m2137;
        loop {
            let step = {
                let (v1, v2) = m2137;
                let m2134 = v1;
                if m2134 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2134 - 1;
                    let m2136 = {
                        let c2135: Fn18 = v0.clone();
                        c2135.apply(v2.clone())?
                    };
                    match m2136 {
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
                Ok(next) => { m2137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f409(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f444(v0)
}

pub fn f410(v0: Fn19, p2144: u64, p2145: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2143 = (p2144, p2145);
    {
        let mut m2143 = m2143;
        loop {
            let step = {
                let (v1, v2) = m2143;
                let m2140 = v1;
                if m2140 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2140 - 1;
                    let m2142 = {
                        let c2141: Fn19 = v0.clone();
                        c2141.apply(v2.clone())?
                    };
                    match m2142 {
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
                Ok(next) => { m2143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f445(v0)
}

pub fn f412(v0: u64) -> u32 {
    let a2146 = v0;
    let a2147 = 32u64;
    if nat_lt(a2146, a2147) {
        let a2148 = v0;
        let a2149 = 16u64;
        if nat_lt(a2148, a2149) {
            let a2150 = v0;
            let a2151 = 8u64;
            if nat_lt(a2150, a2151) {
                let a2152 = v0;
                let a2153 = 4u64;
                if nat_lt(a2152, a2153) {
                    let a2154 = v0;
                    let a2155 = 2u64;
                    if nat_lt(a2154, a2155) {
                        let a2156 = v0;
                        let a2157 = 1u64;
                        if nat_lt(a2156, a2157) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2158 = v0;
                        let a2159 = 3u64;
                        if nat_lt(a2158, a2159) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2160 = v0;
                    let a2161 = 6u64;
                    if nat_lt(a2160, a2161) {
                        let a2162 = v0;
                        let a2163 = 5u64;
                        if nat_lt(a2162, a2163) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2164 = v0;
                        let a2165 = 7u64;
                        if nat_lt(a2164, a2165) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2166 = v0;
                let a2167 = 12u64;
                if nat_lt(a2166, a2167) {
                    let a2168 = v0;
                    let a2169 = 10u64;
                    if nat_lt(a2168, a2169) {
                        let a2170 = v0;
                        let a2171 = 9u64;
                        if nat_lt(a2170, a2171) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2172 = v0;
                        let a2173 = 11u64;
                        if nat_lt(a2172, a2173) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2174 = v0;
                    let a2175 = 14u64;
                    if nat_lt(a2174, a2175) {
                        let a2176 = v0;
                        let a2177 = 13u64;
                        if nat_lt(a2176, a2177) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2178 = v0;
                        let a2179 = 15u64;
                        if nat_lt(a2178, a2179) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2180 = v0;
            let a2181 = 24u64;
            if nat_lt(a2180, a2181) {
                let a2182 = v0;
                let a2183 = 20u64;
                if nat_lt(a2182, a2183) {
                    let a2184 = v0;
                    let a2185 = 18u64;
                    if nat_lt(a2184, a2185) {
                        let a2186 = v0;
                        let a2187 = 17u64;
                        if nat_lt(a2186, a2187) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2188 = v0;
                        let a2189 = 19u64;
                        if nat_lt(a2188, a2189) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2190 = v0;
                    let a2191 = 22u64;
                    if nat_lt(a2190, a2191) {
                        let a2192 = v0;
                        let a2193 = 21u64;
                        if nat_lt(a2192, a2193) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2194 = v0;
                        let a2195 = 23u64;
                        if nat_lt(a2194, a2195) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2196 = v0;
                let a2197 = 28u64;
                if nat_lt(a2196, a2197) {
                    let a2198 = v0;
                    let a2199 = 26u64;
                    if nat_lt(a2198, a2199) {
                        let a2200 = v0;
                        let a2201 = 25u64;
                        if nat_lt(a2200, a2201) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2202 = v0;
                        let a2203 = 27u64;
                        if nat_lt(a2202, a2203) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2204 = v0;
                    let a2205 = 30u64;
                    if nat_lt(a2204, a2205) {
                        let a2206 = v0;
                        let a2207 = 29u64;
                        if nat_lt(a2206, a2207) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2208 = v0;
                        let a2209 = 31u64;
                        if nat_lt(a2208, a2209) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2210 = v0;
        let a2211 = 48u64;
        if nat_lt(a2210, a2211) {
            let a2212 = v0;
            let a2213 = 40u64;
            if nat_lt(a2212, a2213) {
                let a2214 = v0;
                let a2215 = 36u64;
                if nat_lt(a2214, a2215) {
                    let a2216 = v0;
                    let a2217 = 34u64;
                    if nat_lt(a2216, a2217) {
                        let a2218 = v0;
                        let a2219 = 33u64;
                        if nat_lt(a2218, a2219) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2220 = v0;
                        let a2221 = 35u64;
                        if nat_lt(a2220, a2221) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2222 = v0;
                    let a2223 = 38u64;
                    if nat_lt(a2222, a2223) {
                        let a2224 = v0;
                        let a2225 = 37u64;
                        if nat_lt(a2224, a2225) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2226 = v0;
                        let a2227 = 39u64;
                        if nat_lt(a2226, a2227) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2228 = v0;
                let a2229 = 44u64;
                if nat_lt(a2228, a2229) {
                    let a2230 = v0;
                    let a2231 = 42u64;
                    if nat_lt(a2230, a2231) {
                        let a2232 = v0;
                        let a2233 = 41u64;
                        if nat_lt(a2232, a2233) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2234 = v0;
                        let a2235 = 43u64;
                        if nat_lt(a2234, a2235) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2236 = v0;
                    let a2237 = 46u64;
                    if nat_lt(a2236, a2237) {
                        let a2238 = v0;
                        let a2239 = 45u64;
                        if nat_lt(a2238, a2239) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2240 = v0;
                        let a2241 = 47u64;
                        if nat_lt(a2240, a2241) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2242 = v0;
            let a2243 = 56u64;
            if nat_lt(a2242, a2243) {
                let a2244 = v0;
                let a2245 = 52u64;
                if nat_lt(a2244, a2245) {
                    let a2246 = v0;
                    let a2247 = 50u64;
                    if nat_lt(a2246, a2247) {
                        let a2248 = v0;
                        let a2249 = 49u64;
                        if nat_lt(a2248, a2249) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2250 = v0;
                        let a2251 = 51u64;
                        if nat_lt(a2250, a2251) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2252 = v0;
                    let a2253 = 54u64;
                    if nat_lt(a2252, a2253) {
                        let a2254 = v0;
                        let a2255 = 53u64;
                        if nat_lt(a2254, a2255) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2256 = v0;
                        let a2257 = 55u64;
                        if nat_lt(a2256, a2257) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2258 = v0;
                let a2259 = 60u64;
                if nat_lt(a2258, a2259) {
                    let a2260 = v0;
                    let a2261 = 58u64;
                    if nat_lt(a2260, a2261) {
                        let a2262 = v0;
                        let a2263 = 57u64;
                        if nat_lt(a2262, a2263) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2264 = v0;
                        let a2265 = 59u64;
                        if nat_lt(a2264, a2265) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2266 = v0;
                    let a2267 = 62u64;
                    if nat_lt(a2266, a2267) {
                        let a2268 = v0;
                        let a2269 = 61u64;
                        if nat_lt(a2268, a2269) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2270 = v0;
                        let a2271 = 63u64;
                        if nat_lt(a2270, a2271) {
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

pub fn f413(v0: u32) -> u32 {
    let a2274 = {
        let a2272 = f446(v0, 6u32, 26u32);
        let a2273 = f446(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2272, a2273)
    };
    let a2275 = f446(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2274, a2275)
}

pub fn f414(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2281 = {
        let a2276 = v0;
        let a2277 = v1;
        fixed_u32::bit_and(a2276, a2277)
    };
    let a2282 = {
        let a2279 = {
            let a2278 = v0;
            fixed_u32::bit_not(a2278)
        };
        let a2280 = v2;
        fixed_u32::bit_and(a2279, a2280)
    };
    fixed_u32::bit_xor(a2281, a2282)
}

pub fn f415(v0: u32) -> u32 {
    let a2285 = {
        let a2283 = f446(v0, 2u32, 30u32);
        let a2284 = f446(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2283, a2284)
    };
    let a2286 = f446(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2285, a2286)
}

pub fn f416(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2295 = {
        let a2291 = {
            let a2287 = v0;
            let a2288 = v1;
            fixed_u32::bit_and(a2287, a2288)
        };
        let a2292 = {
            let a2289 = v0;
            let a2290 = v2;
            fixed_u32::bit_and(a2289, a2290)
        };
        fixed_u32::bit_xor(a2291, a2292)
    };
    let a2296 = {
        let a2293 = v1;
        let a2294 = v2;
        fixed_u32::bit_and(a2293, a2294)
    };
    fixed_u32::bit_xor(a2295, a2296)
}

pub fn f417(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h2297, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2297
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2298, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2298
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2299, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2299
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2300, _, _, _, _, _, _, _, _, _, _, _) => {
            h2300
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2301, _, _, _, _, _, _, _, _, _, _) => {
            h2301
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2302, _, _, _, _, _, _, _, _, _) => {
            h2302
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2303, _, _, _, _, _, _, _, _) => {
            h2303
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h2304, _, _, _, _, _, _, _) => {
            h2304
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2305, _, _, _, _, _, _) => {
            h2305
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h2306, _, _, _, _, _) => {
            h2306
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h2307, _, _, _, _) => {
            h2307
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2308, _, _, _) => {
            h2308
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2309, _, _) => {
            h2309
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2310, _) => {
            h2310
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2311) => {
            h2311
        }
    }, f360(f447(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2312, _) => {
            h2312
        }
    }), f360(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2313, _, _, _, _, _, _) => {
            h2313
        }
    }, f360(f448(match v0.clone() {
        Adt24::C0(_, h2314, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2314
        }
    }), match v0 {
        Adt24::C0(h2315, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2315
        }
    }))))
}

pub fn f418(v0: Fn20, p2319: u64, p2320: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2318 = (p2319, p2320);
    {
        let mut m2318 = m2318;
        loop {
            let step = {
                let (v1, v2) = m2318;
                let m2316 = v1;
                if m2316 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2316 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2317: Fn20 = v0.clone();
                        c2317.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f419(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2321, _) = v0;
        h2321
    };
    let v2: u32 = {
        let (h2322, _) = {
            let (_, h2323) = v0;
            h2323
        };
        h2322
    };
    let v3: u32 = {
        let (_, h2324) = {
            let (_, h2325) = v0;
            h2325
        };
        h2324
    };
    ({
        let a2326 = v1;
        let a2327 = 2u64;
        let a2328 = 0u64;
        nat_quot(a2326, a2327, a2328)
    }, (f420(v2, 1u32), {
        let a2332 = {
            let a2329 = v1;
            let a2330 = 2u64;
            let a2331 = 0u64;
            nat_rem(a2329, a2330, a2331)
        };
        let a2333 = 1u64;
        if nat_eq(a2332, a2333) {
            let a2334 = v3;
            let a2335 = v2;
            fixed_u32::bit_or(a2334, a2335)
        } else {
            v3
        }
    }))
}

pub fn f420(v0: u32, v1: u32) -> u32 {
    let m2338 = {
        let a2336 = v0;
        let a2337 = v1;
        fixed_u32::shift_left(a2336, a2337)
    };
    match m2338 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f421(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f325(v0.clone(), {
        let a2339 = v1;
        let a2340 = 1u64;
        nat_add(a2339, a2340)?
    })?;
    let v4: Bytes = {
        let a2341 = v3;
        let a2342 = 117u64;
        if nat_eq(a2341, a2342) {
            f449(v0, v1)?
        } else {
            f450(v3)
        }
    };
    let a2343 = v4.clone();
    let a2344 = Bytes::lit(&[]);
    if equal(a2343, a2344) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2345 = v3;
            let a2346 = 117u64;
            if nat_eq(a2345, a2346) {
                let a2347 = v1;
                let a2348 = 6u64;
                nat_add(a2347, a2348)?
            } else {
                let a2349 = v1;
                let a2350 = 2u64;
                nat_add(a2349, a2350)?
            }
        }, ({
            let a2351 = v2;
            let a2352 = v4;
            append_bytes(a2351, a2352)
        }, 0u64))))
    }
}

pub fn f422(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f325(v0, v1)?;
    let a2353 = 47u64;
    let a2354 = v2;
    if if nat_lt(a2353, a2354) {
        let a2355 = v2;
        let a2356 = 58u64;
        nat_lt(a2355, a2356)
    } else {
        false
    } {
        Ok(Some({
            let a2357 = v1;
            let a2358 = 1u64;
            nat_add(a2357, a2358)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f423(v0: Bytes) -> R<u64> {
    let (h2359, _) = f228(Fn3::F451(v0.clone()), {
        let a2360 = v0;
        length_bytes(a2360)
    }, 0u64)?;
    Ok(h2359)
}

pub fn f424(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2364 = {
        let a2362 = v0;
        let a2363 = {
            let (h2361, _) = v1;
            h2361
        };
        index_bytes(a2362, a2363)
    };
    match m2364 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2366 = {
                    let (h2365, _) = v1;
                    h2365
                };
                let a2367 = 1u64;
                nat_add(a2366, a2367)?
            }, {
                let a2371 = {
                    let a2369 = {
                        let (_, h2368) = v1;
                        h2368
                    };
                    let a2370 = 10u64;
                    nat_mul(a2369, a2370)?
                };
                let a2372 = f452(v2)?;
                nat_add(a2371, a2372)?
            })))
        }
    }
}

pub fn f425(v0: Bytes) -> R<Option<Bytes>> {
    if f194(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f218(f32(v0.clone(), 8u64, {
            let a2374 = {
                let a2373 = v0;
                length_bytes(a2373)
            };
            let a2375 = 8u64;
            nat_sub(a2374, a2375)
        }))?))
    } else {
        if f194(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f218(f32(v0.clone(), 14u64, {
                let a2377 = {
                    let a2376 = v0;
                    length_bytes(a2376)
                };
                let a2378 = 14u64;
                nat_sub(a2377, a2378)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f426(v0: List<Adt14>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f453(f454(f73(v0.clone()))?, v0, v1)
}

pub fn f427(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt23>), Bytes> {
    Err::<(Bytes, List<Adt23>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f428(v0: Bytes, v1: List<Adt23>) -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((v0, v1))
}

pub fn f429(v0: Bytes, v1: List<Adt14>) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2379 = v1.clone();
    match m2379.uncons() {
        None => {
            Ok(f427(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2380 = f455(f333(f142(v2, f8(Str::lit("properties")))), v1.clone());
            match m2380.uncons() {
                None => {
                    let m2381 = f456(f454(f457(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2381 {
                        Ok(v5) => {
                            Ok(f428(f432(v0, {
                                let (h2382, _) = v5;
                                h2382
                            }), {
                                let (_, h2383) = v5;
                                h2383
                            }))
                        }
                        Err(v4) => {
                            Ok(f427(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2384 = f458(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2384 {
                        Ok(v9) => {
                            Ok(f428(f432(v0, {
                                let (h2385, _) = v9;
                                h2385
                            }), {
                                let (_, h2386) = v9;
                                h2386
                            }))
                        }
                        Err(v8) => {
                            Ok(f427(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f430(v0: Adt14, v1: Bytes) -> R<bool> {
    let m2387 = v0;
    match m2387 {
        Adt14::C0(_) => {
            Ok(false)
        }
        Adt14::C1(_, _) => {
            Ok(false)
        }
        Adt14::C2(_) => {
            Ok(false)
        }
        Adt14::C3(r2388) => {
            let v2 = (*r2388).clone();
            f430(v2, v1)
        }
        Adt14::C4(r2389) => {
            let v3 = (*r2389).clone();
            f430(v3, v1)
        }
        Adt14::C5 => {
            Ok(false)
        }
        Adt14::C6(r2390, r2391) => {
            let v4 = (*r2390).clone();
            let v5 = (*r2391).clone();
            if f430(v4, v1.clone())? {
                Ok(true)
            } else {
                f430(v5, v1)
            }
        }
        Adt14::C7(v6, r2392, r2393) => {
            let v7 = (*r2392).clone();
            let v8 = (*r2393).clone();
            if if f36(v6, f8(Str::lit("$ref"))) {
                f459(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f430(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f430(v8, v1)
                }
            }
        }
    }
}

pub fn f431(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    f460(f454(f461(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f432(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f204(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f433(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2394 = f462(v0.clone());
    match m2394 {
        None => {
            if f36(f145(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2395 = f142(v0, f8(Str::lit("items")));
                match m2395 {
                    None => {
                        Ok(f463(f8(Str::lit("List")), f465()))
                    }
                    Some(v3) => {
                        Ok(f463(f8(Str::lit("List")), f464(v3, {
                            let a2396 = v1;
                            let a2397 = f8(Str::lit("Item"));
                            append_bytes(a2396, a2397)
                        })?))
                    }
                }
            } else {
                f464(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f463(f8(Str::lit("Option")), f464(v2, v1)?))
        }
    }
}

pub fn f434(p2400: List<Adt14>) -> bool {
    let m2399 = p2400;
    {
        let mut m2399 = m2399;
        loop {
            let step = {
                let v0 = m2399;
                let m2398 = v0;
                match m2398.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f466(f368(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt14>, bool>(v2)
                        } else {
                            Err::<List<Adt14>, bool>(false)
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

pub fn f435(v0: Adt14, v1: Bytes) -> Adt14 {
    let m2401 = v0;
    match m2401 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r2402) => {
            let v10 = (*r2402).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r2403) => {
            let v5 = (*r2403).clone();
            Adt14::C4(std::rc::Rc::new(f435(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r2404, r2405) => {
            let v11 = (*r2404).clone();
            let v12 = (*r2405).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r2406, r2407) => {
            let v3 = (*r2406).clone();
            let v4 = (*r2407).clone();
            if f36(v2.clone(), v1.clone()) {
                f435(v4, v1)
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f435(v4, v1)))
            }
        }
    }
}

pub fn f436(v0: Adt14, v1: Bytes) -> bool {
    let m2408 = f142(v0, v1);
    match m2408 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f437(v0: List<Adt14>) -> bool {
    let m2409 = v0.clone();
    match m2409.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2410 = f455(f333(f142(v1, f8(Str::lit("properties")))), v0.clone());
            match m2410.uncons() {
                None => {
                    f468(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f467(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f438(v0: Adt14, v1: Bytes, v2: Adt14) -> Adt14 {
    let m2411 = v0;
    match m2411 {
        Adt14::C0(v7) => {
            Adt14::C0(v7)
        }
        Adt14::C1(v8, v9) => {
            Adt14::C1(v8, v9)
        }
        Adt14::C2(v10) => {
            Adt14::C2(v10)
        }
        Adt14::C3(r2412) => {
            let v11 = (*r2412).clone();
            Adt14::C3(std::rc::Rc::new(v11))
        }
        Adt14::C4(r2413) => {
            let v3 = (*r2413).clone();
            Adt14::C4(std::rc::Rc::new(Adt14::C7(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r2414, r2415) => {
            let v12 = (*r2414).clone();
            let v13 = (*r2415).clone();
            Adt14::C6(std::rc::Rc::new(v12), std::rc::Rc::new(v13))
        }
        Adt14::C7(v4, r2416, r2417) => {
            let v5 = (*r2416).clone();
            let v6 = (*r2417).clone();
            Adt14::C7(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f439(p2421: List<Adt14>) -> bool {
    let m2420 = p2421;
    {
        let mut m2420 = m2420;
        loop {
            let step = {
                let v0 = m2420;
                let m2418 = v0;
                match m2418.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f466(f368(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f434(f222({
                                let m2419 = f142(v1, f8(Str::lit("anyOf")));
                                match m2419 {
                                    None => {
                                        Adt14::C5
                                    }
                                    Some(v3) => {
                                        v3
                                    }
                                }
                            })) {
                                Ok::<List<Adt14>, bool>(v2)
                            } else {
                                Err::<List<Adt14>, bool>(false)
                            }
                        } else {
                            Err::<List<Adt14>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2420 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2422, _) = v2;
        h2422
    };
    let a2423 = v3;
    let a2424 = 0u64;
    if nat_eq(a2423, a2424) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2425 = v3;
            let a2426 = v1;
            let a2427 = 0u64;
            nat_quot(a2425, a2426, a2427)
        }, {
            let a2432 = f402(v0, {
                let a2428 = v3;
                let a2429 = v1;
                let a2430 = 0u64;
                nat_rem(a2428, a2429, a2430)
            });
            let a2433 = {
                let (_, h2431) = v2;
                h2431
            };
            append_bytes(a2432, a2433)
        }))
    }
}

pub fn f441(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2478 = {
            let a2454 = {
                let a2442 = {
                    let a2436 = {
                        let a2434 = v1;
                        let a2435 = 128u8;
                        fixed_u8::bit_and(a2434, a2435)
                    };
                    let a2437 = 128u8;
                    if equal(a2436, a2437) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2443 = {
                    let a2440 = {
                        let a2438 = v1;
                        let a2439 = 64u8;
                        fixed_u8::bit_and(a2438, a2439)
                    };
                    let a2441 = 64u8;
                    if equal(a2440, a2441) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2442, a2443)?
            };
            let a2455 = {
                let a2452 = {
                    let a2446 = {
                        let a2444 = v1;
                        let a2445 = 32u8;
                        fixed_u8::bit_and(a2444, a2445)
                    };
                    let a2447 = 32u8;
                    if equal(a2446, a2447) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2453 = {
                    let a2450 = {
                        let a2448 = v1;
                        let a2449 = 16u8;
                        fixed_u8::bit_and(a2448, a2449)
                    };
                    let a2451 = 16u8;
                    if equal(a2450, a2451) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2452, a2453)?
            };
            nat_add(a2454, a2455)?
        };
        let a2479 = {
            let a2476 = {
                let a2464 = {
                    let a2458 = {
                        let a2456 = v1;
                        let a2457 = 8u8;
                        fixed_u8::bit_and(a2456, a2457)
                    };
                    let a2459 = 8u8;
                    if equal(a2458, a2459) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2465 = {
                    let a2462 = {
                        let a2460 = v1;
                        let a2461 = 4u8;
                        fixed_u8::bit_and(a2460, a2461)
                    };
                    let a2463 = 4u8;
                    if equal(a2462, a2463) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2464, a2465)?
            };
            let a2477 = {
                let a2474 = {
                    let a2468 = {
                        let a2466 = v1;
                        let a2467 = 2u8;
                        fixed_u8::bit_and(a2466, a2467)
                    };
                    let a2469 = 2u8;
                    if equal(a2468, a2469) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2475 = {
                    let a2472 = {
                        let a2470 = v1;
                        let a2471 = 1u8;
                        fixed_u8::bit_and(a2470, a2471)
                    };
                    let a2473 = 1u8;
                    if equal(a2472, a2473) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2474, a2475)?
            };
            nat_add(a2476, a2477)?
        };
        nat_add(a2478, a2479)?
    };
    let a2486 = f402(f469(), {
        let a2480 = v2;
        let a2481 = 16u64;
        let a2482 = 0u64;
        nat_quot(a2480, a2481, a2482)
    });
    let a2487 = f402(f469(), {
        let a2483 = v2;
        let a2484 = 16u64;
        let a2485 = 0u64;
        nat_rem(a2483, a2484, a2485)
    });
    Ok(append_bytes(a2486, a2487))
}

pub fn f442(v0: Fn21, p2491: List<List<(Bytes, Bytes)>>, p2492: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2490 = (p2491, p2492);
    {
        let mut m2490 = m2490;
        loop {
            let step = {
                let (v1, v2) = m2490;
                let m2488 = v2;
                match m2488.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2489: Fn21 = v0.clone();
                            c2489.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2490 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f444(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2494 = {
        let a2493 = v0.clone();
        length_list(a2493)
    };
    let a2495 = 1u64;
    if nat_le(a2494, a2495) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f470(v0)?))
    }
}

pub fn f445(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2496, _) = v0;
        h2496
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2497) = v0;
        h2497
    };
    let m2498 = v1;
    match m2498.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2499 = v4;
            match m2499.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f471(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f446(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2500 = f318(v0, v1);
    let a2501 = f420(v0, v2);
    fixed_u32::bit_or(a2500, a2501)
}

pub fn f447(v0: u32) -> u32 {
    let a2504 = {
        let a2502 = f446(v0, 17u32, 15u32);
        let a2503 = f446(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2502, a2503)
    };
    let a2505 = f318(v0, 10u32);
    fixed_u32::bit_xor(a2504, a2505)
}

pub fn f448(v0: u32) -> u32 {
    let a2508 = {
        let a2506 = f446(v0, 7u32, 25u32);
        let a2507 = f446(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2506, a2507)
    };
    let a2509 = f318(v0, 3u32);
    fixed_u32::bit_xor(a2508, a2509)
}

pub fn f449(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2512 = f32(v0.clone(), {
        let a2510 = v1;
        let a2511 = 2u64;
        nat_add(a2510, a2511)?
    }, 2u64);
    let a2513 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2512, a2513) {
        let m2516 = f472(f32(v0, {
            let a2514 = v1;
            let a2515 = 4u64;
            nat_add(a2514, a2515)?
        }, 2u64))?;
        match m2516 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f325(v2.clone(), 0u64)?;
                let a2517 = v3;
                let a2518 = 32u64;
                if nat_lt(a2517, a2518) {
                    if f473(v3) {
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

pub fn f450(v0: u64) -> Bytes {
    let a2519 = v0;
    let a2520 = 34u64;
    if nat_eq(a2519, a2520) {
        Bytes::lit(&[0x22])
    } else {
        let a2521 = v0;
        let a2522 = 92u64;
        if nat_eq(a2521, a2522) {
            Bytes::lit(&[0x5c])
        } else {
            let a2523 = v0;
            let a2524 = 98u64;
            if nat_eq(a2523, a2524) {
                Bytes::lit(&[0x08])
            } else {
                let a2525 = v0;
                let a2526 = 102u64;
                if nat_eq(a2525, a2526) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2527 = v0;
                    let a2528 = 110u64;
                    if nat_eq(a2527, a2528) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2529 = v0;
                        let a2530 = 114u64;
                        if nat_eq(a2529, a2530) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2531 = v0;
                            let a2532 = 116u64;
                            if nat_eq(a2531, a2532) {
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

pub fn f451(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2535 = {
        let a2533 = v0;
        let a2534 = v1;
        index_bytes(a2533, a2534)
    };
    match m2535 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f474(v2)? {
                Ok(Some({
                    let a2536 = v1;
                    let a2537 = 1u64;
                    nat_add(a2536, a2537)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f452(v0: u8) -> R<u64> {
    let a2584 = {
        let v1: u8 = v0;
        let a2582 = {
            let a2558 = {
                let a2546 = {
                    let a2540 = {
                        let a2538 = v1;
                        let a2539 = 128u8;
                        fixed_u8::bit_and(a2538, a2539)
                    };
                    let a2541 = 128u8;
                    if equal(a2540, a2541) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2547 = {
                    let a2544 = {
                        let a2542 = v1;
                        let a2543 = 64u8;
                        fixed_u8::bit_and(a2542, a2543)
                    };
                    let a2545 = 64u8;
                    if equal(a2544, a2545) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2546, a2547)?
            };
            let a2559 = {
                let a2556 = {
                    let a2550 = {
                        let a2548 = v1;
                        let a2549 = 32u8;
                        fixed_u8::bit_and(a2548, a2549)
                    };
                    let a2551 = 32u8;
                    if equal(a2550, a2551) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2557 = {
                    let a2554 = {
                        let a2552 = v1;
                        let a2553 = 16u8;
                        fixed_u8::bit_and(a2552, a2553)
                    };
                    let a2555 = 16u8;
                    if equal(a2554, a2555) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2556, a2557)?
            };
            nat_add(a2558, a2559)?
        };
        let a2583 = {
            let a2580 = {
                let a2568 = {
                    let a2562 = {
                        let a2560 = v1;
                        let a2561 = 8u8;
                        fixed_u8::bit_and(a2560, a2561)
                    };
                    let a2563 = 8u8;
                    if equal(a2562, a2563) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2569 = {
                    let a2566 = {
                        let a2564 = v1;
                        let a2565 = 4u8;
                        fixed_u8::bit_and(a2564, a2565)
                    };
                    let a2567 = 4u8;
                    if equal(a2566, a2567) {
                        4u64
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
                        let a2571 = 2u8;
                        fixed_u8::bit_and(a2570, a2571)
                    };
                    let a2573 = 2u8;
                    if equal(a2572, a2573) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2579 = {
                    let a2576 = {
                        let a2574 = v1;
                        let a2575 = 1u8;
                        fixed_u8::bit_and(a2574, a2575)
                    };
                    let a2577 = 1u8;
                    if equal(a2576, a2577) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2578, a2579)?
            };
            nat_add(a2580, a2581)?
        };
        nat_add(a2582, a2583)?
    };
    let a2585 = 48u64;
    Ok(nat_sub(a2584, a2585))
}

pub fn f453(v0: bool, v1: List<Adt14>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2586 = v1;
    match m2586.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2587 = f475(v0, v3.clone())?;
            match m2587 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f476(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f85(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2588 = f453(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2588 {
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

pub fn f454(p2592: List<Bytes>) -> R<bool> {
    let m2591 = p2592;
    {
        let mut m2591 = m2591;
        loop {
            let step = {
                let v0 = m2591;
                let m2589 = v0;
                match m2589.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2590 = f477(v1)?;
                        match m2590 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f85(v3, f478(false, v2.clone())?) {
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
                Ok(next) => { m2591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455(v0: List<(Bytes, Adt14)>, v1: List<Adt14>) -> List<Bytes> {
    let m2593 = v0;
    match m2593.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f479(v1.clone(), {
                let (h2594, _) = v2.clone();
                h2594
            }) {
                List::cons({
                    let (h2595, _) = v2;
                    h2595
                }, f455(v3, v1))
            } else {
                f455(v3, v1)
            }
        }
    }
}

pub fn f456(v0: bool, v1: Bytes, v2: List<Adt14>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2596 = v2;
    match m2596.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v4, v5)) => {
            let m2597 = f480(v4.clone())?;
            match m2597 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m2598 = f481(v0, v6.clone())?;
                    match m2598 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f85(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2601 = f482(v4, {
                                    let a2599 = v1.clone();
                                    let a2600 = f218(v6)?;
                                    append_bytes(a2599, a2600)
                                })?;
                                match m2601 {
                                    Ok(v9) => {
                                        let m2602 = f456(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m2602 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h2603, _) = v9;
                                                    h2603
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h2604, _) = v11;
                                                    h2604
                                                }), f337({
                                                    let (_, h2605) = v9;
                                                    h2605
                                                }, {
                                                    let (_, h2606) = v11;
                                                    h2606
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

pub fn f457(v0: List<Adt14>) -> R<List<Bytes>> {
    let m2607 = v0;
    match m2607.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m2608 = f480(v1)?;
            match m2608 {
                None => {
                    f457(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f457(v2)?))
                }
            }
        }
    }
}

pub fn f458(v0: Bytes, v1: List<Adt14>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    f483(f454(f484(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f459(v0: Adt14, v1: Bytes) -> R<bool> {
    let m2609 = f150(Some(v0));
    match m2609 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2610 = f425(v2)?;
            match m2610 {
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

pub fn f460(v0: bool, v1: Bytes, v2: List<(Bytes, Adt14)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2611 = v2;
    match m2611.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v5, v6)) => {
            let m2613 = f481(v0, {
                let (h2612, _) = v5.clone();
                h2612
            })?;
            match m2613 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h2614, _) = v5;
                        h2614
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f85(v7.clone(), f478(v0, f461(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2619 = f433({
                            let (_, h2615) = v5.clone();
                            h2615
                        }, {
                            let a2617 = v1.clone();
                            let a2618 = f218({
                                let (h2616, _) = v5.clone();
                                h2616
                            })?;
                            append_bytes(a2617, a2618)
                        })?;
                        match m2619 {
                            Ok(v9) => {
                                let v10: Bytes = if f85({
                                    let (h2620, _) = v5;
                                    h2620
                                }, v3.clone()) {
                                    let (h2621, _) = v9;
                                    h2621
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h2622, _) = v9;
                                        h2622
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m2623 = f460(v0, v1, v6, v3, v4)?;
                                match m2623 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h2624, _) = v12;
                                            h2624
                                        }), f337({
                                            let (_, h2625) = v9;
                                            h2625
                                        }, {
                                            let (_, h2626) = v12;
                                            h2626
                                        }))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v11))
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

pub fn f461(v0: List<(Bytes, Adt14)>) -> List<Bytes> {
    let m2627 = v0;
    match m2627.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2628, _) = v1;
                h2628
            }, f461(v2))
        }
    }
}

pub fn f462(v0: Adt14) -> Option<Adt14> {
    let m2629 = f146(f142(v0, f8(Str::lit("oneOf"))));
    match m2629.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2630 = v2;
            match m2630.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2631 = v4;
                    match m2631.uncons() {
                        None => {
                            if f485(v3.clone()) {
                                Some(v1)
                            } else {
                                if f485(v1) {
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

pub fn f463(v0: Bytes, v1: Result<(Bytes, List<Adt23>), Bytes>) -> Result<(Bytes, List<Adt23>), Bytes> {
    let m2632 = v1;
    match m2632 {
        Ok(v3) => {
            f428(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2633, _) = v3;
                h2633
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2634) = v3;
                h2634
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v2)
        }
    }
}

pub fn f464(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2635 = f462(v0.clone());
    match m2635 {
        None => {
            if f36(f145(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2636 = f142(v0, f8(Str::lit("items")));
                match m2636 {
                    None => {
                        Ok(f463(f8(Str::lit("List")), f465()))
                    }
                    Some(v3) => {
                        Ok(f463(f8(Str::lit("List")), f482(v3, {
                            let a2637 = v1;
                            let a2638 = f8(Str::lit("Item"));
                            append_bytes(a2637, a2638)
                        })?))
                    }
                }
            } else {
                f482(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f463(f8(Str::lit("Option")), f482(v2, v1)?))
        }
    }
}

pub fn f465() -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((f371(), List::cons(Adt23::C2, List::<Adt23>::nil())))
}

pub fn f466(p2642: List<(Bytes, Adt14)>, v1: List<Bytes>) -> bool {
    let m2641 = p2642;
    {
        let mut m2641 = m2641;
        loop {
            let step = {
                let v0 = m2641;
                let m2639 = v0;
                match m2639.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f85({
                            let (h2640, _) = v2;
                            h2640
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt14)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt14)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(p2646: List<Adt14>, v1: Bytes, p2647: List<Bytes>) -> bool {
    let m2645 = (p2646, p2647);
    {
        let mut m2645 = m2645;
        loop {
            let step = {
                let (v0, v2) = m2645;
                let m2643 = v0;
                match m2643.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m2644 = f486(v3, v1.clone());
                        match m2644 {
                            None => {
                                Err::<(List<Adt14>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f85(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt14>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt14>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2645 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f468(p2650: List<Adt14>, p2651: List<Bytes>) -> bool {
    let m2649 = (p2650, p2651);
    {
        let mut m2649 = m2649;
        loop {
            let step = {
                let (v0, v1) = m2649;
                let m2648 = v0;
                match m2648.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f145(v2, f8(Str::lit("type")));
                        if !f36(v4.clone(), Bytes::lit(&[])) {
                            if !f85(v4.clone(), v1.clone()) {
                                if !if if f36(v4.clone(), f8(Str::lit("integer"))) {
                                    f85(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f36(v4.clone(), f8(Str::lit("number"))) {
                                        f85(f8(Str::lit("integer")), v1.clone())
                                    } else {
                                        false
                                    }
                                } {
                                    Ok::<(List<Adt14>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt14>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt14>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt14>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f470(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f405({
        let (_, h2652) = {
            let (h2653, _) = f487(Fn22::F488, {
                let a2655 = {
                    let a2654 = v0.clone();
                    length_list(a2654)
                };
                let a2656 = 1u64;
                nat_add(a2655, a2656)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2653
        };
        h2652
    }))
}

pub fn f471(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2661 = {
            let a2659 = {
                let a2657 = v0.clone();
                length_list(a2657)
            };
            let a2660 = {
                let a2658 = v1.clone();
                length_list(a2658)
            };
            nat_add(a2659, a2660)?
        };
        let a2662 = 1u64;
        nat_add(a2661, a2662)?
    };
    Ok(f308({
        let (_, h2663) = {
            let (_, h2664) = {
                let (h2665, _) = f489(Fn23::F490, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2665
            };
            h2664
        };
        h2663
    }))
}

pub fn f472(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2666 = v0.clone();
        length_bytes(a2666)
    };
    let a2670 = {
        let a2667 = v1;
        let a2668 = 2u64;
        let a2669 = 0u64;
        nat_rem(a2667, a2668, a2669)
    };
    let a2671 = 0u64;
    if nat_eq(a2670, a2671) {
        let (_, h2672) = {
            let (h2673, _) = f491(Fn24::F492(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2673
        };
        Ok(h2672)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f473(v0: u64) -> bool {
    let a2674 = v0;
    let a2675 = 8u64;
    if nat_eq(a2674, a2675) {
        true
    } else {
        let a2676 = v0;
        let a2677 = 9u64;
        if nat_eq(a2676, a2677) {
            true
        } else {
            let a2678 = v0;
            let a2679 = 10u64;
            if nat_eq(a2678, a2679) {
                true
            } else {
                let a2680 = v0;
                let a2681 = 12u64;
                if nat_eq(a2680, a2681) {
                    true
                } else {
                    let a2682 = v0;
                    let a2683 = 13u64;
                    nat_eq(a2682, a2683)
                }
            }
        }
    }
}

pub fn f474(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2728 = {
            let a2704 = {
                let a2692 = {
                    let a2686 = {
                        let a2684 = v1;
                        let a2685 = 128u8;
                        fixed_u8::bit_and(a2684, a2685)
                    };
                    let a2687 = 128u8;
                    if equal(a2686, a2687) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2693 = {
                    let a2690 = {
                        let a2688 = v1;
                        let a2689 = 64u8;
                        fixed_u8::bit_and(a2688, a2689)
                    };
                    let a2691 = 64u8;
                    if equal(a2690, a2691) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2692, a2693)?
            };
            let a2705 = {
                let a2702 = {
                    let a2696 = {
                        let a2694 = v1;
                        let a2695 = 32u8;
                        fixed_u8::bit_and(a2694, a2695)
                    };
                    let a2697 = 32u8;
                    if equal(a2696, a2697) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2703 = {
                    let a2700 = {
                        let a2698 = v1;
                        let a2699 = 16u8;
                        fixed_u8::bit_and(a2698, a2699)
                    };
                    let a2701 = 16u8;
                    if equal(a2700, a2701) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2702, a2703)?
            };
            nat_add(a2704, a2705)?
        };
        let a2729 = {
            let a2726 = {
                let a2714 = {
                    let a2708 = {
                        let a2706 = v1;
                        let a2707 = 8u8;
                        fixed_u8::bit_and(a2706, a2707)
                    };
                    let a2709 = 8u8;
                    if equal(a2708, a2709) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2715 = {
                    let a2712 = {
                        let a2710 = v1;
                        let a2711 = 4u8;
                        fixed_u8::bit_and(a2710, a2711)
                    };
                    let a2713 = 4u8;
                    if equal(a2712, a2713) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2714, a2715)?
            };
            let a2727 = {
                let a2724 = {
                    let a2718 = {
                        let a2716 = v1;
                        let a2717 = 2u8;
                        fixed_u8::bit_and(a2716, a2717)
                    };
                    let a2719 = 2u8;
                    if equal(a2718, a2719) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2725 = {
                    let a2722 = {
                        let a2720 = v1;
                        let a2721 = 1u8;
                        fixed_u8::bit_and(a2720, a2721)
                    };
                    let a2723 = 1u8;
                    if equal(a2722, a2723) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2724, a2725)?
            };
            nat_add(a2726, a2727)?
        };
        nat_add(a2728, a2729)?
    };
    let a2730 = 48u64;
    let a2731 = v2;
    if nat_le(a2730, a2731) {
        let a2732 = v2;
        let a2733 = 57u64;
        Ok(nat_le(a2732, a2733))
    } else {
        Ok(false)
    }
}

pub fn f475(v0: bool, v1: Adt14) -> R<Option<Bytes>> {
    let m2734 = v1;
    match m2734 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v3, v4) => {
            Ok(Some({
                let a2737 = if if v3 {
                    let a2735 = 0u64;
                    let a2736 = v4;
                    nat_lt(a2735, a2736)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2738 = f288(false, v4)?;
                append_bytes(a2737, a2738)
            }))
        }
        Adt14::C2(v2) => {
            f481(v0, v2)
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

pub fn f476(v0: Adt14) -> R<Bytes> {
    f221(v0)
}

pub fn f477(v0: Bytes) -> R<Option<Bytes>> {
    if f493({
        let a2739 = v0.clone();
        length_bytes(a2739)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f494(v0)?;
        if f495(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2742 = {
                let a2740 = v1.clone();
                let a2741 = 0u64;
                index_bytes(a2740, a2741)
            };
            match m2742 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f331({
                        let v3: u8 = v2;
                        let a2787 = {
                            let a2763 = {
                                let a2751 = {
                                    let a2745 = {
                                        let a2743 = v3;
                                        let a2744 = 128u8;
                                        fixed_u8::bit_and(a2743, a2744)
                                    };
                                    let a2746 = 128u8;
                                    if equal(a2745, a2746) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2752 = {
                                    let a2749 = {
                                        let a2747 = v3;
                                        let a2748 = 64u8;
                                        fixed_u8::bit_and(a2747, a2748)
                                    };
                                    let a2750 = 64u8;
                                    if equal(a2749, a2750) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2751, a2752)?
                            };
                            let a2764 = {
                                let a2761 = {
                                    let a2755 = {
                                        let a2753 = v3;
                                        let a2754 = 32u8;
                                        fixed_u8::bit_and(a2753, a2754)
                                    };
                                    let a2756 = 32u8;
                                    if equal(a2755, a2756) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2762 = {
                                    let a2759 = {
                                        let a2757 = v3;
                                        let a2758 = 16u8;
                                        fixed_u8::bit_and(a2757, a2758)
                                    };
                                    let a2760 = 16u8;
                                    if equal(a2759, a2760) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2761, a2762)?
                            };
                            nat_add(a2763, a2764)?
                        };
                        let a2788 = {
                            let a2785 = {
                                let a2773 = {
                                    let a2767 = {
                                        let a2765 = v3;
                                        let a2766 = 8u8;
                                        fixed_u8::bit_and(a2765, a2766)
                                    };
                                    let a2768 = 8u8;
                                    if equal(a2767, a2768) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2774 = {
                                    let a2771 = {
                                        let a2769 = v3;
                                        let a2770 = 4u8;
                                        fixed_u8::bit_and(a2769, a2770)
                                    };
                                    let a2772 = 4u8;
                                    if equal(a2771, a2772) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2773, a2774)?
                            };
                            let a2786 = {
                                let a2783 = {
                                    let a2777 = {
                                        let a2775 = v3;
                                        let a2776 = 2u8;
                                        fixed_u8::bit_and(a2775, a2776)
                                    };
                                    let a2778 = 2u8;
                                    if equal(a2777, a2778) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2784 = {
                                    let a2781 = {
                                        let a2779 = v3;
                                        let a2780 = 1u8;
                                        fixed_u8::bit_and(a2779, a2780)
                                    };
                                    let a2782 = 1u8;
                                    if equal(a2781, a2782) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2783, a2784)?
                            };
                            nat_add(a2785, a2786)?
                        };
                        nat_add(a2787, a2788)?
                    }) {
                        Ok(Some({
                            let a2789 = f8(Str::lit("n"));
                            let a2790 = v1;
                            append_bytes(a2789, a2790)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f496({
            let a2791 = v0.clone();
            length_bytes(a2791)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f478(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2792 = v1;
    match m2792.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2793 = f481(v0, v2)?;
            match m2793 {
                None => {
                    f478(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f478(v0, v3)?))
                }
            }
        }
    }
}

pub fn f479(p2797: List<Adt14>, v1: Bytes) -> bool {
    let m2796 = p2797;
    {
        let mut m2796 = m2796;
        loop {
            let step = {
                let v0 = m2796;
                let m2794 = v0;
                match m2794.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m2795 = f486(v2, v1.clone());
                        if match m2795 {
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
                Ok(next) => { m2796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: Adt14) -> R<Option<Bytes>> {
    let m2798 = f150(f142(v0.clone(), f8(Str::lit("$ref"))));
    match m2798 {
        None => {
            Ok(f150(f142(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f425(v1)
        }
    }
}

pub fn f481(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f497(v1)
    } else {
        f477(v1)
    }
}

pub fn f482(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    f498(f401(v0), v1)
}

pub fn f483(v0: bool, v1: Bytes, v2: List<Adt14>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2799 = v2;
    match m2799.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v5, v6)) => {
            let m2800 = f486(v5.clone(), v3.clone());
            match m2800 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m2801 = f481(v0, v7.clone())?;
                    match m2801 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f85(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2804 = f431({
                                    let a2802 = v1.clone();
                                    let a2803 = f218(v7)?;
                                    append_bytes(a2802, a2803)
                                }, f370(f333(f142(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f73(f146(f142(v5, f8(Str::lit("required"))))), true)?;
                                match m2804 {
                                    Ok(v10) => {
                                        let m2805 = f483(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m2805 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f204({
                                                    let (h2806, _) = v10;
                                                    h2806
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h2807, _) = v12;
                                                    h2807
                                                }), f337({
                                                    let (_, h2808) = v10;
                                                    h2808
                                                }, {
                                                    let (_, h2809) = v12;
                                                    h2809
                                                }))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v9))
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

pub fn f484(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m2810 = v0;
    match m2810.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m2811 = f486(v2, v1.clone());
            match m2811 {
                None => {
                    f484(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f484(v3, v1))
                }
            }
        }
    }
}

pub fn f485(v0: Adt14) -> bool {
    f36(f145(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f486(v0: Adt14, v1: Bytes) -> Option<Bytes> {
    if f85(v1.clone(), f73(f146(f142(v0.clone(), f8(Str::lit("required")))))) {
        let m2812 = f142(v0, f8(Str::lit("properties")));
        match m2812 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m2813 = f142(v2, v1);
                match m2813 {
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

pub fn f487(v0: Fn22, p2818: u64, p2819: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2817 = (p2818, p2819);
    {
        let mut m2817 = m2817;
        loop {
            let step = {
                let (v1, v2) = m2817;
                let m2814 = v1;
                if m2814 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2814 - 1;
                    let m2816 = {
                        let c2815: Fn22 = v0.clone();
                        c2815.apply(v2.clone())?
                    };
                    match m2816 {
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
                Ok(next) => { m2817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f499(v0)
}

pub fn f489(v0: Fn23, p2824: u64, p2825: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2823 = (p2824, p2825);
    {
        let mut m2823 = m2823;
        loop {
            let step = {
                let (v1, v2) = m2823;
                let m2820 = v1;
                if m2820 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2820 - 1;
                    let m2822 = {
                        let c2821: Fn23 = v0.clone();
                        c2821.apply(v2.clone())
                    };
                    match m2822 {
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
                Ok(next) => { m2823 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f500(v0)
}

pub fn f491(v0: Fn24, p2830: u64, p2831: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2829 = (p2830, p2831);
    {
        let mut m2829 = m2829;
        loop {
            let step = {
                let (v1, v2) = m2829;
                let m2826 = v1;
                if m2826 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2826 - 1;
                    let m2828 = {
                        let c2827: Fn24 = v0.clone();
                        c2827.apply(v2.clone())?
                    };
                    match m2828 {
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
                Ok(next) => { m2829 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f501(v0, v1)
}

pub fn f493(p2887: u64, v1: Bytes, p2888: u64) -> R<bool> {
    let m2886 = (p2887, p2888);
    {
        let mut m2886 = m2886;
        loop {
            let step = {
                let (v0, v2) = m2886;
                let m2832 = v0;
                if m2832 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2832 - 1;
                    let m2835 = {
                        let a2833 = v1.clone();
                        let a2834 = v2;
                        index_bytes(a2833, a2834)
                    };
                    match m2835 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2882 = {
                                let v5: u8 = v4;
                                let a2880 = {
                                    let a2856 = {
                                        let a2844 = {
                                            let a2838 = {
                                                let a2836 = v5;
                                                let a2837 = 128u8;
                                                fixed_u8::bit_and(a2836, a2837)
                                            };
                                            let a2839 = 128u8;
                                            if equal(a2838, a2839) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2845 = {
                                            let a2842 = {
                                                let a2840 = v5;
                                                let a2841 = 64u8;
                                                fixed_u8::bit_and(a2840, a2841)
                                            };
                                            let a2843 = 64u8;
                                            if equal(a2842, a2843) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2844, a2845)?
                                    };
                                    let a2857 = {
                                        let a2854 = {
                                            let a2848 = {
                                                let a2846 = v5;
                                                let a2847 = 32u8;
                                                fixed_u8::bit_and(a2846, a2847)
                                            };
                                            let a2849 = 32u8;
                                            if equal(a2848, a2849) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2855 = {
                                            let a2852 = {
                                                let a2850 = v5;
                                                let a2851 = 16u8;
                                                fixed_u8::bit_and(a2850, a2851)
                                            };
                                            let a2853 = 16u8;
                                            if equal(a2852, a2853) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2854, a2855)?
                                    };
                                    nat_add(a2856, a2857)?
                                };
                                let a2881 = {
                                    let a2878 = {
                                        let a2866 = {
                                            let a2860 = {
                                                let a2858 = v5;
                                                let a2859 = 8u8;
                                                fixed_u8::bit_and(a2858, a2859)
                                            };
                                            let a2861 = 8u8;
                                            if equal(a2860, a2861) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2867 = {
                                            let a2864 = {
                                                let a2862 = v5;
                                                let a2863 = 4u8;
                                                fixed_u8::bit_and(a2862, a2863)
                                            };
                                            let a2865 = 4u8;
                                            if equal(a2864, a2865) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2866, a2867)?
                                    };
                                    let a2879 = {
                                        let a2876 = {
                                            let a2870 = {
                                                let a2868 = v5;
                                                let a2869 = 2u8;
                                                fixed_u8::bit_and(a2868, a2869)
                                            };
                                            let a2871 = 2u8;
                                            if equal(a2870, a2871) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2877 = {
                                            let a2874 = {
                                                let a2872 = v5;
                                                let a2873 = 1u8;
                                                fixed_u8::bit_and(a2872, a2873)
                                            };
                                            let a2875 = 1u8;
                                            if equal(a2874, a2875) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2876, a2877)?
                                    };
                                    nat_add(a2878, a2879)?
                                };
                                nat_add(a2880, a2881)?
                            };
                            let a2883 = 128u64;
                            if nat_lt(a2882, a2883) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2884 = v2;
                                    let a2885 = 1u64;
                                    nat_add(a2884, a2885)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2886 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f284(v0.clone(), 0u64, {
        let a2889 = v0;
        length_bytes(a2889)
    }, false, Bytes::lit(&[]))?;
    let m2892 = {
        let a2890 = v1.clone();
        let a2891 = 0u64;
        index_bytes(a2890, a2891)
    };
    match m2892 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2937 = {
                    let a2913 = {
                        let a2901 = {
                            let a2895 = {
                                let a2893 = v3;
                                let a2894 = 128u8;
                                fixed_u8::bit_and(a2893, a2894)
                            };
                            let a2896 = 128u8;
                            if equal(a2895, a2896) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2902 = {
                            let a2899 = {
                                let a2897 = v3;
                                let a2898 = 64u8;
                                fixed_u8::bit_and(a2897, a2898)
                            };
                            let a2900 = 64u8;
                            if equal(a2899, a2900) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2901, a2902)?
                    };
                    let a2914 = {
                        let a2911 = {
                            let a2905 = {
                                let a2903 = v3;
                                let a2904 = 32u8;
                                fixed_u8::bit_and(a2903, a2904)
                            };
                            let a2906 = 32u8;
                            if equal(a2905, a2906) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2912 = {
                            let a2909 = {
                                let a2907 = v3;
                                let a2908 = 16u8;
                                fixed_u8::bit_and(a2907, a2908)
                            };
                            let a2910 = 16u8;
                            if equal(a2909, a2910) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2911, a2912)?
                    };
                    nat_add(a2913, a2914)?
                };
                let a2938 = {
                    let a2935 = {
                        let a2923 = {
                            let a2917 = {
                                let a2915 = v3;
                                let a2916 = 8u8;
                                fixed_u8::bit_and(a2915, a2916)
                            };
                            let a2918 = 8u8;
                            if equal(a2917, a2918) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2924 = {
                            let a2921 = {
                                let a2919 = v3;
                                let a2920 = 4u8;
                                fixed_u8::bit_and(a2919, a2920)
                            };
                            let a2922 = 4u8;
                            if equal(a2921, a2922) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2923, a2924)?
                    };
                    let a2936 = {
                        let a2933 = {
                            let a2927 = {
                                let a2925 = v3;
                                let a2926 = 2u8;
                                fixed_u8::bit_and(a2925, a2926)
                            };
                            let a2928 = 2u8;
                            if equal(a2927, a2928) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2934 = {
                            let a2931 = {
                                let a2929 = v3;
                                let a2930 = 1u8;
                                fixed_u8::bit_and(a2929, a2930)
                            };
                            let a2932 = 1u8;
                            if equal(a2931, a2932) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2933, a2934)?
                    };
                    nat_add(a2935, a2936)?
                };
                nat_add(a2937, a2938)?
            };
            if f330(v4) {
                let a2945 = f32({
                    let a2939 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2939)
                }, {
                    let a2940 = v4;
                    let a2941 = 65u64;
                    nat_sub(a2940, a2941)
                }, 1u64);
                let a2946 = f32(v1.clone(), 1u64, {
                    let a2943 = {
                        let a2942 = v1;
                        length_bytes(a2942)
                    };
                    let a2944 = 1u64;
                    nat_sub(a2943, a2944)
                });
                Ok(append_bytes(a2945, a2946))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f495(v0: Bytes) -> R<bool> {
    let m2949 = {
        let a2947 = v0;
        let a2948 = 0u64;
        index_bytes(a2947, a2948)
    };
    match m2949 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f329({
                let v2: u8 = v1;
                let a2994 = {
                    let a2970 = {
                        let a2958 = {
                            let a2952 = {
                                let a2950 = v2;
                                let a2951 = 128u8;
                                fixed_u8::bit_and(a2950, a2951)
                            };
                            let a2953 = 128u8;
                            if equal(a2952, a2953) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2959 = {
                            let a2956 = {
                                let a2954 = v2;
                                let a2955 = 64u8;
                                fixed_u8::bit_and(a2954, a2955)
                            };
                            let a2957 = 64u8;
                            if equal(a2956, a2957) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2958, a2959)?
                    };
                    let a2971 = {
                        let a2968 = {
                            let a2962 = {
                                let a2960 = v2;
                                let a2961 = 32u8;
                                fixed_u8::bit_and(a2960, a2961)
                            };
                            let a2963 = 32u8;
                            if equal(a2962, a2963) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2969 = {
                            let a2966 = {
                                let a2964 = v2;
                                let a2965 = 16u8;
                                fixed_u8::bit_and(a2964, a2965)
                            };
                            let a2967 = 16u8;
                            if equal(a2966, a2967) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2968, a2969)?
                    };
                    nat_add(a2970, a2971)?
                };
                let a2995 = {
                    let a2992 = {
                        let a2980 = {
                            let a2974 = {
                                let a2972 = v2;
                                let a2973 = 8u8;
                                fixed_u8::bit_and(a2972, a2973)
                            };
                            let a2975 = 8u8;
                            if equal(a2974, a2975) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2981 = {
                            let a2978 = {
                                let a2976 = v2;
                                let a2977 = 4u8;
                                fixed_u8::bit_and(a2976, a2977)
                            };
                            let a2979 = 4u8;
                            if equal(a2978, a2979) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2980, a2981)?
                    };
                    let a2993 = {
                        let a2990 = {
                            let a2984 = {
                                let a2982 = v2;
                                let a2983 = 2u8;
                                fixed_u8::bit_and(a2982, a2983)
                            };
                            let a2985 = 2u8;
                            if equal(a2984, a2985) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2991 = {
                            let a2988 = {
                                let a2986 = v2;
                                let a2987 = 1u8;
                                fixed_u8::bit_and(a2986, a2987)
                            };
                            let a2989 = 1u8;
                            if equal(a2988, a2989) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2990, a2991)?
                    };
                    nat_add(a2992, a2993)?
                };
                nat_add(a2994, a2995)?
            }) {
                Ok(true)
            } else {
                Ok(f330({
                    let v3: u8 = v1;
                    let a3040 = {
                        let a3016 = {
                            let a3004 = {
                                let a2998 = {
                                    let a2996 = v3;
                                    let a2997 = 128u8;
                                    fixed_u8::bit_and(a2996, a2997)
                                };
                                let a2999 = 128u8;
                                if equal(a2998, a2999) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a3005 = {
                                let a3002 = {
                                    let a3000 = v3;
                                    let a3001 = 64u8;
                                    fixed_u8::bit_and(a3000, a3001)
                                };
                                let a3003 = 64u8;
                                if equal(a3002, a3003) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3004, a3005)?
                        };
                        let a3017 = {
                            let a3014 = {
                                let a3008 = {
                                    let a3006 = v3;
                                    let a3007 = 32u8;
                                    fixed_u8::bit_and(a3006, a3007)
                                };
                                let a3009 = 32u8;
                                if equal(a3008, a3009) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a3015 = {
                                let a3012 = {
                                    let a3010 = v3;
                                    let a3011 = 16u8;
                                    fixed_u8::bit_and(a3010, a3011)
                                };
                                let a3013 = 16u8;
                                if equal(a3012, a3013) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3014, a3015)?
                        };
                        nat_add(a3016, a3017)?
                    };
                    let a3041 = {
                        let a3038 = {
                            let a3026 = {
                                let a3020 = {
                                    let a3018 = v3;
                                    let a3019 = 8u8;
                                    fixed_u8::bit_and(a3018, a3019)
                                };
                                let a3021 = 8u8;
                                if equal(a3020, a3021) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a3027 = {
                                let a3024 = {
                                    let a3022 = v3;
                                    let a3023 = 4u8;
                                    fixed_u8::bit_and(a3022, a3023)
                                };
                                let a3025 = 4u8;
                                if equal(a3024, a3025) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3026, a3027)?
                        };
                        let a3039 = {
                            let a3036 = {
                                let a3030 = {
                                    let a3028 = v3;
                                    let a3029 = 2u8;
                                    fixed_u8::bit_and(a3028, a3029)
                                };
                                let a3031 = 2u8;
                                if equal(a3030, a3031) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a3037 = {
                                let a3034 = {
                                    let a3032 = v3;
                                    let a3033 = 1u8;
                                    fixed_u8::bit_and(a3032, a3033)
                                };
                                let a3035 = 1u8;
                                if equal(a3034, a3035) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3036, a3037)?
                        };
                        nat_add(a3038, a3039)?
                    };
                    nat_add(a3040, a3041)?
                }))
            }
        }
    }
}

pub fn f496(p3054: u64, v1: Bytes, p3055: u64, p3056: Bytes) -> R<Option<Bytes>> {
    let m3053 = (p3054, (p3055, p3056));
    {
        let mut m3053 = m3053;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3053;
                let m3042 = v0;
                if m3042 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3042 - 1;
                    let a3044 = {
                        let a3043 = v1.clone();
                        length_bytes(a3043)
                    };
                    let a3045 = v2;
                    if nat_le(a3044, a3045) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3046 = f502(v1.clone(), v2)?;
                        match m3046 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3048 = v2;
                                    let a3049 = {
                                        let (_, h3047) = v5;
                                        h3047
                                    };
                                    nat_add(a3048, a3049)?
                                }, {
                                    let a3051 = v3;
                                    let a3052 = f503({
                                        let (h3050, _) = v5;
                                        h3050
                                    });
                                    append_bytes(a3051, a3052)
                                })))
                            }
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

pub fn f497(v0: Bytes) -> R<Option<Bytes>> {
    let m3058 = f504({
        let a3057 = v0.clone();
        length_bytes(a3057)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m3058 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3060 = {
                let a3059 = v1.clone();
                length_bytes(a3059)
            };
            let a3061 = 0u64;
            if nat_eq(a3060, a3061) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f498(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m3062 = v0.clone();
    match m3062 {
        Adt14::C0(v2) => {
            if v2 {
                Ok(f465())
            } else {
                Ok(f427(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt14::C1(_, _) => {
            Ok(f427(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C2(_) => {
            Ok(f427(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C3(_) => {
            Ok(f427(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C4(_) => {
            let m3063 = f505(f368(v0.clone()));
            match m3063 {
                None => {
                    let m3064 = f150(f142(v0.clone(), f8(Str::lit("$ref"))));
                    match m3064 {
                        None => {
                            let m3066 = f142(v0.clone(), f8(Str::lit("allOf")));
                            match m3066 {
                                None => {
                                    let m3071 = f142(v0.clone(), f8(Str::lit("enum")));
                                    match m3071 {
                                        None => {
                                            let m3072 = f142(v0.clone(), f8(Str::lit("const")));
                                            match m3072 {
                                                None => {
                                                    let m3074 = f142(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3074 {
                                                        None => {
                                                            let v27: Bytes = f145(v0.clone(), f8(Str::lit("type")));
                                                            if if f36(v27.clone(), Bytes::lit(&[])) {
                                                                !f506(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f465())
                                                            } else {
                                                                if f36(v27.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f428(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                                                } else {
                                                                    if f36(v27.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f428(f507(v0), List::<Adt23>::nil()))
                                                                    } else {
                                                                        if f36(v27.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f428(f508(), List::<Adt23>::nil()))
                                                                        } else {
                                                                            if f36(v27.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f428(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                                                            } else {
                                                                                if f36(v27.clone(), f8(Str::lit("object"))) {
                                                                                    let m3075 = f333(f142(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3075.uncons() {
                                                                                        None => {
                                                                                            let m3076 = f509(v0);
                                                                                            match m3076 {
                                                                                                None => {
                                                                                                    Ok(f463(f8(Str::lit("List")), f510(f465())))
                                                                                                }
                                                                                                Some(v30) => {
                                                                                                    Ok(f463(f8(Str::lit("List")), f510(f511(v30, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f428(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f427(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v27, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f428(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v24) => {
                                                    let m3073 = f150(Some(v24));
                                                    match m3073 {
                                                        None => {
                                                            Ok(f427(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f428(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f428(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                        }
                                    }
                                }
                                Some(v16) => {
                                    let m3067 = f222(v16);
                                    match m3067.uncons() {
                                        None => {
                                            Ok(f427(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v17, v18)) => {
                                            let m3068 = v18;
                                            match m3068.uncons() {
                                                None => {
                                                    let m3069 = f150(f142(v17, f8(Str::lit("$ref"))));
                                                    match m3069 {
                                                        None => {
                                                            Ok(f427(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v21) => {
                                                            let m3070 = f425(v21)?;
                                                            match m3070 {
                                                                None => {
                                                                    Ok(f427(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v22) => {
                                                                    Ok(f428(v22, List::<Adt23>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f427(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v14) => {
                            let m3065 = f425(v14)?;
                            match m3065 {
                                None => {
                                    Ok(f427(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v15) => {
                                    Ok(f428(v15, List::<Adt23>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v13) => {
                    Ok(f427(if f36(v13.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v13, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt14::C5 => {
            Ok(f465())
        }
        Adt14::C6(_, _) => {
            Ok(f427(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C7(_, _, _) => {
            Ok(f427(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f499(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3077, _) = v0;
        h3077
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3078) = v0;
        h3078
    };
    let m3079 = v1;
    match m3079.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3080 = v4;
            match m3080.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f512(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f500(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3081, _) = v0.clone();
        h3081
    };
    let v2: List<(Bytes, ())> = {
        let (h3082, _) = {
            let (_, h3083) = v0.clone();
            h3083
        };
        h3082
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3084) = {
            let (_, h3085) = v0;
            h3085
        };
        h3084
    };
    let m3086 = v1.clone();
    match m3086.uncons() {
        None => {
            let m3087 = v2;
            match m3087.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3088 = v2.clone();
            match m3088.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f513({
                        let (h3089, _) = v8.clone();
                        h3089
                    }, {
                        let (h3090, _) = v6.clone();
                        h3090
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

pub fn f501(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3091, _) = v1;
        h3091
    };
    let m3093 = {
        let (_, h3092) = v1;
        h3092
    };
    match m3093 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3096 = {
                let a3094 = v0.clone();
                let a3095 = v2;
                index_bytes(a3094, a3095)
            };
            match m3096 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3101 = {
                        let a3099 = v0;
                        let a3100 = {
                            let a3097 = v2;
                            let a3098 = 1u64;
                            nat_add(a3097, a3098)?
                        };
                        index_bytes(a3099, a3100)
                    };
                    match m3101 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f514(v4)? {
                                f514(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3102 = v2;
                                    let a3103 = 2u64;
                                    nat_add(a3102, a3103)?
                                }, Some({
                                    let a3108 = v3;
                                    let a3109 = f402(f515(), {
                                        let a3106 = {
                                            let a3104 = f516(v4)?;
                                            let a3105 = 16u64;
                                            nat_mul(a3104, a3105)?
                                        };
                                        let a3107 = f516(v5)?;
                                        nat_add(a3106, a3107)?
                                    });
                                    append_bytes(a3108, a3109)
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

pub fn f502(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3112 = {
        let a3110 = v0.clone();
        let a3111 = v1;
        index_bytes(a3110, a3111)
    };
    match m3112 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3157 = {
                    let a3133 = {
                        let a3121 = {
                            let a3115 = {
                                let a3113 = v3;
                                let a3114 = 128u8;
                                fixed_u8::bit_and(a3113, a3114)
                            };
                            let a3116 = 128u8;
                            if equal(a3115, a3116) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3122 = {
                            let a3119 = {
                                let a3117 = v3;
                                let a3118 = 64u8;
                                fixed_u8::bit_and(a3117, a3118)
                            };
                            let a3120 = 64u8;
                            if equal(a3119, a3120) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3121, a3122)?
                    };
                    let a3134 = {
                        let a3131 = {
                            let a3125 = {
                                let a3123 = v3;
                                let a3124 = 32u8;
                                fixed_u8::bit_and(a3123, a3124)
                            };
                            let a3126 = 32u8;
                            if equal(a3125, a3126) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3132 = {
                            let a3129 = {
                                let a3127 = v3;
                                let a3128 = 16u8;
                                fixed_u8::bit_and(a3127, a3128)
                            };
                            let a3130 = 16u8;
                            if equal(a3129, a3130) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3131, a3132)?
                    };
                    nat_add(a3133, a3134)?
                };
                let a3158 = {
                    let a3155 = {
                        let a3143 = {
                            let a3137 = {
                                let a3135 = v3;
                                let a3136 = 8u8;
                                fixed_u8::bit_and(a3135, a3136)
                            };
                            let a3138 = 8u8;
                            if equal(a3137, a3138) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3144 = {
                            let a3141 = {
                                let a3139 = v3;
                                let a3140 = 4u8;
                                fixed_u8::bit_and(a3139, a3140)
                            };
                            let a3142 = 4u8;
                            if equal(a3141, a3142) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3143, a3144)?
                    };
                    let a3156 = {
                        let a3153 = {
                            let a3147 = {
                                let a3145 = v3;
                                let a3146 = 2u8;
                                fixed_u8::bit_and(a3145, a3146)
                            };
                            let a3148 = 2u8;
                            if equal(a3147, a3148) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3154 = {
                            let a3151 = {
                                let a3149 = v3;
                                let a3150 = 1u8;
                                fixed_u8::bit_and(a3149, a3150)
                            };
                            let a3152 = 1u8;
                            if equal(a3151, a3152) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3153, a3154)?
                    };
                    nat_add(a3155, a3156)?
                };
                nat_add(a3157, a3158)?
            };
            let a3159 = v4;
            let a3160 = 128u64;
            if nat_lt(a3159, a3160) {
                Ok(Some((v4, 1u64)))
            } else {
                let a3161 = 194u64;
                let a3162 = v4;
                if if nat_le(a3161, a3162) {
                    let a3163 = v4;
                    let a3164 = 223u64;
                    nat_le(a3163, a3164)
                } else {
                    false
                } {
                    let m3167 = f517(v0, {
                        let a3165 = v1;
                        let a3166 = 1u64;
                        nat_add(a3165, a3166)?
                    })?;
                    match m3167 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f518({
                                let a3172 = {
                                    let a3170 = {
                                        let a3168 = v4;
                                        let a3169 = 192u64;
                                        nat_sub(a3168, a3169)
                                    };
                                    let a3171 = 64u64;
                                    nat_mul(a3170, a3171)?
                                };
                                let a3173 = v5;
                                nat_add(a3172, a3173)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a3174 = 224u64;
                    let a3175 = v4;
                    if if nat_le(a3174, a3175) {
                        let a3176 = v4;
                        let a3177 = 239u64;
                        nat_le(a3176, a3177)
                    } else {
                        false
                    } {
                        let m3180 = f517(v0.clone(), {
                            let a3178 = v1;
                            let a3179 = 1u64;
                            nat_add(a3178, a3179)?
                        })?;
                        match m3180 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m3183 = f517(v0, {
                                    let a3181 = v1;
                                    let a3182 = 2u64;
                                    nat_add(a3181, a3182)?
                                })?;
                                match m3183 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f518({
                                            let a3192 = {
                                                let a3186 = {
                                                    let a3184 = v4;
                                                    let a3185 = 224u64;
                                                    nat_sub(a3184, a3185)
                                                };
                                                let a3187 = 4096u64;
                                                nat_mul(a3186, a3187)?
                                            };
                                            let a3193 = {
                                                let a3190 = {
                                                    let a3188 = v6;
                                                    let a3189 = 64u64;
                                                    nat_mul(a3188, a3189)?
                                                };
                                                let a3191 = v7;
                                                nat_add(a3190, a3191)?
                                            };
                                            nat_add(a3192, a3193)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a3194 = 240u64;
                        let a3195 = v4;
                        if if nat_le(a3194, a3195) {
                            let a3196 = v4;
                            let a3197 = 244u64;
                            nat_le(a3196, a3197)
                        } else {
                            false
                        } {
                            let m3200 = f517(v0.clone(), {
                                let a3198 = v1;
                                let a3199 = 1u64;
                                nat_add(a3198, a3199)?
                            })?;
                            match m3200 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m3203 = f517(v0.clone(), {
                                        let a3201 = v1;
                                        let a3202 = 2u64;
                                        nat_add(a3201, a3202)?
                                    })?;
                                    match m3203 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m3206 = f517(v0, {
                                                let a3204 = v1;
                                                let a3205 = 3u64;
                                                nat_add(a3204, a3205)?
                                            })?;
                                            match m3206 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f518({
                                                        let a3219 = {
                                                            let a3209 = {
                                                                let a3207 = v4;
                                                                let a3208 = 240u64;
                                                                nat_sub(a3207, a3208)
                                                            };
                                                            let a3210 = 262144u64;
                                                            nat_mul(a3209, a3210)?
                                                        };
                                                        let a3220 = {
                                                            let a3217 = {
                                                                let a3211 = v8;
                                                                let a3212 = 4096u64;
                                                                nat_mul(a3211, a3212)?
                                                            };
                                                            let a3218 = {
                                                                let a3215 = {
                                                                    let a3213 = v9;
                                                                    let a3214 = 64u64;
                                                                    nat_mul(a3213, a3214)?
                                                                };
                                                                let a3216 = v10;
                                                                nat_add(a3215, a3216)?
                                                            };
                                                            nat_add(a3217, a3218)?
                                                        };
                                                        nat_add(a3219, a3220)?
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

pub fn f503(v0: u64) -> Bytes {
    let a3221 = f8(Str::lit("u"));
    let a3222 = f519(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3221, a3222)
}

pub fn f504(p3235: u64, v1: Bytes, p3236: u64, p3237: Bytes) -> R<Option<Bytes>> {
    let m3234 = (p3235, (p3236, p3237));
    {
        let mut m3234 = m3234;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3234;
                let m3223 = v0;
                if m3223 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3223 - 1;
                    let a3225 = {
                        let a3224 = v1.clone();
                        length_bytes(a3224)
                    };
                    let a3226 = v2;
                    if nat_le(a3225, a3226) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3227 = f502(v1.clone(), v2)?;
                        match m3227 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3229 = v2;
                                    let a3230 = {
                                        let (_, h3228) = v5;
                                        h3228
                                    };
                                    nat_add(a3229, a3230)?
                                }, {
                                    let a3232 = v3;
                                    let a3233 = f520(v1.clone(), v2, {
                                        let (h3231, _) = v5;
                                        h3231
                                    });
                                    append_bytes(a3232, a3233)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f505(p3242: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m3241 = p3242;
    {
        let mut m3241 = m3241;
        loop {
            let step = {
                let v0 = m3241;
                let m3238 = v0;
                match m3238.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f85({
                            let (h3239, _) = v1.clone();
                            h3239
                        }, f521()) {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h3240, _) = v1;
                                h3240
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(v0: Adt14) -> bool {
    if f436(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f436(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f436(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f436(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f436(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f436(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f507(v0: Adt14) -> Bytes {
    let m3243 = f300(f142(v0, f8(Str::lit("minimum"))));
    match m3243 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f508() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f509(v0: Adt14) -> Option<Adt14> {
    let m3244 = f142(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3244 {
        None => {
            let m3245 = f333(f142(v0, f8(Str::lit("patternProperties"))));
            match m3245.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v2, v3)) => {
                    let m3246 = v3;
                    match m3246.uncons() {
                        None => {
                            Some({
                                let (_, h3247) = v2;
                                h3247
                            })
                        }
                        Some((_, _)) => {
                            None::<Adt14>
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

pub fn f510(v0: Result<(Bytes, List<Adt23>), Bytes>) -> Result<(Bytes, List<Adt23>), Bytes> {
    let m3248 = v0;
    match m3248 {
        Ok(v2) => {
            f428(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h3249, _) = v2;
                h3249
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h3250) = v2;
                h3250
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v1)
        }
    }
}

pub fn f511(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    f522(16u64, v0, v1)
}

pub fn f512(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3255 = {
            let a3253 = {
                let a3251 = v0.clone();
                length_list(a3251)
            };
            let a3254 = {
                let a3252 = v1.clone();
                length_list(a3252)
            };
            nat_add(a3253, a3254)?
        };
        let a3256 = 1u64;
        nat_add(a3255, a3256)?
    };
    Ok(f523({
        let (_, h3257) = {
            let (_, h3258) = {
                let (h3259, _) = f524(Fn25::F525, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3259
            };
            h3258
        };
        h3257
    }))
}

pub fn f513(v0: Bytes, v1: Bytes) -> bool {
    let a3264 = {
        let a3260 = v0;
        let a3261 = v1;
        compare_bytes(a3260, a3261)
    };
    let a3265 = {
        let a3262 = Bytes::lit(&[]);
        let a3263 = Bytes::lit(&[0x01]);
        compare_bytes(a3262, a3263)
    };
    equal(a3264, a3265)
}

pub fn f514(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3310 = {
            let a3286 = {
                let a3274 = {
                    let a3268 = {
                        let a3266 = v1;
                        let a3267 = 128u8;
                        fixed_u8::bit_and(a3266, a3267)
                    };
                    let a3269 = 128u8;
                    if equal(a3268, a3269) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3275 = {
                    let a3272 = {
                        let a3270 = v1;
                        let a3271 = 64u8;
                        fixed_u8::bit_and(a3270, a3271)
                    };
                    let a3273 = 64u8;
                    if equal(a3272, a3273) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3274, a3275)?
            };
            let a3287 = {
                let a3284 = {
                    let a3278 = {
                        let a3276 = v1;
                        let a3277 = 32u8;
                        fixed_u8::bit_and(a3276, a3277)
                    };
                    let a3279 = 32u8;
                    if equal(a3278, a3279) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3285 = {
                    let a3282 = {
                        let a3280 = v1;
                        let a3281 = 16u8;
                        fixed_u8::bit_and(a3280, a3281)
                    };
                    let a3283 = 16u8;
                    if equal(a3282, a3283) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3284, a3285)?
            };
            nat_add(a3286, a3287)?
        };
        let a3311 = {
            let a3308 = {
                let a3296 = {
                    let a3290 = {
                        let a3288 = v1;
                        let a3289 = 8u8;
                        fixed_u8::bit_and(a3288, a3289)
                    };
                    let a3291 = 8u8;
                    if equal(a3290, a3291) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3297 = {
                    let a3294 = {
                        let a3292 = v1;
                        let a3293 = 4u8;
                        fixed_u8::bit_and(a3292, a3293)
                    };
                    let a3295 = 4u8;
                    if equal(a3294, a3295) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3296, a3297)?
            };
            let a3309 = {
                let a3306 = {
                    let a3300 = {
                        let a3298 = v1;
                        let a3299 = 2u8;
                        fixed_u8::bit_and(a3298, a3299)
                    };
                    let a3301 = 2u8;
                    if equal(a3300, a3301) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3307 = {
                    let a3304 = {
                        let a3302 = v1;
                        let a3303 = 1u8;
                        fixed_u8::bit_and(a3302, a3303)
                    };
                    let a3305 = 1u8;
                    if equal(a3304, a3305) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3306, a3307)?
            };
            nat_add(a3308, a3309)?
        };
        nat_add(a3310, a3311)?
    };
    let a3312 = 48u64;
    let a3313 = v2;
    if if nat_le(a3312, a3313) {
        let a3314 = v2;
        let a3315 = 57u64;
        nat_le(a3314, a3315)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3316 = 97u64;
        let a3317 = v2;
        if nat_le(a3316, a3317) {
            let a3318 = v2;
            let a3319 = 102u64;
            Ok(nat_le(a3318, a3319))
        } else {
            Ok(false)
        }
    }
}

pub fn f515() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f516(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3364 = {
            let a3340 = {
                let a3328 = {
                    let a3322 = {
                        let a3320 = v1;
                        let a3321 = 128u8;
                        fixed_u8::bit_and(a3320, a3321)
                    };
                    let a3323 = 128u8;
                    if equal(a3322, a3323) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3329 = {
                    let a3326 = {
                        let a3324 = v1;
                        let a3325 = 64u8;
                        fixed_u8::bit_and(a3324, a3325)
                    };
                    let a3327 = 64u8;
                    if equal(a3326, a3327) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3328, a3329)?
            };
            let a3341 = {
                let a3338 = {
                    let a3332 = {
                        let a3330 = v1;
                        let a3331 = 32u8;
                        fixed_u8::bit_and(a3330, a3331)
                    };
                    let a3333 = 32u8;
                    if equal(a3332, a3333) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3339 = {
                    let a3336 = {
                        let a3334 = v1;
                        let a3335 = 16u8;
                        fixed_u8::bit_and(a3334, a3335)
                    };
                    let a3337 = 16u8;
                    if equal(a3336, a3337) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3338, a3339)?
            };
            nat_add(a3340, a3341)?
        };
        let a3365 = {
            let a3362 = {
                let a3350 = {
                    let a3344 = {
                        let a3342 = v1;
                        let a3343 = 8u8;
                        fixed_u8::bit_and(a3342, a3343)
                    };
                    let a3345 = 8u8;
                    if equal(a3344, a3345) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3351 = {
                    let a3348 = {
                        let a3346 = v1;
                        let a3347 = 4u8;
                        fixed_u8::bit_and(a3346, a3347)
                    };
                    let a3349 = 4u8;
                    if equal(a3348, a3349) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3350, a3351)?
            };
            let a3363 = {
                let a3360 = {
                    let a3354 = {
                        let a3352 = v1;
                        let a3353 = 2u8;
                        fixed_u8::bit_and(a3352, a3353)
                    };
                    let a3355 = 2u8;
                    if equal(a3354, a3355) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3361 = {
                    let a3358 = {
                        let a3356 = v1;
                        let a3357 = 1u8;
                        fixed_u8::bit_and(a3356, a3357)
                    };
                    let a3359 = 1u8;
                    if equal(a3358, a3359) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3360, a3361)?
            };
            nat_add(a3362, a3363)?
        };
        nat_add(a3364, a3365)?
    };
    let a3366 = v2;
    let a3367 = 57u64;
    if nat_le(a3366, a3367) {
        let a3368 = v2;
        let a3369 = 48u64;
        Ok(nat_sub(a3368, a3369))
    } else {
        let a3370 = v2;
        let a3371 = 87u64;
        Ok(nat_sub(a3370, a3371))
    }
}

pub fn f517(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3374 = {
        let a3372 = v0;
        let a3373 = v1;
        index_bytes(a3372, a3373)
    };
    match m3374 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3419 = {
                    let a3395 = {
                        let a3383 = {
                            let a3377 = {
                                let a3375 = v3;
                                let a3376 = 128u8;
                                fixed_u8::bit_and(a3375, a3376)
                            };
                            let a3378 = 128u8;
                            if equal(a3377, a3378) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3384 = {
                            let a3381 = {
                                let a3379 = v3;
                                let a3380 = 64u8;
                                fixed_u8::bit_and(a3379, a3380)
                            };
                            let a3382 = 64u8;
                            if equal(a3381, a3382) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3383, a3384)?
                    };
                    let a3396 = {
                        let a3393 = {
                            let a3387 = {
                                let a3385 = v3;
                                let a3386 = 32u8;
                                fixed_u8::bit_and(a3385, a3386)
                            };
                            let a3388 = 32u8;
                            if equal(a3387, a3388) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3394 = {
                            let a3391 = {
                                let a3389 = v3;
                                let a3390 = 16u8;
                                fixed_u8::bit_and(a3389, a3390)
                            };
                            let a3392 = 16u8;
                            if equal(a3391, a3392) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3393, a3394)?
                    };
                    nat_add(a3395, a3396)?
                };
                let a3420 = {
                    let a3417 = {
                        let a3405 = {
                            let a3399 = {
                                let a3397 = v3;
                                let a3398 = 8u8;
                                fixed_u8::bit_and(a3397, a3398)
                            };
                            let a3400 = 8u8;
                            if equal(a3399, a3400) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3406 = {
                            let a3403 = {
                                let a3401 = v3;
                                let a3402 = 4u8;
                                fixed_u8::bit_and(a3401, a3402)
                            };
                            let a3404 = 4u8;
                            if equal(a3403, a3404) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3405, a3406)?
                    };
                    let a3418 = {
                        let a3415 = {
                            let a3409 = {
                                let a3407 = v3;
                                let a3408 = 2u8;
                                fixed_u8::bit_and(a3407, a3408)
                            };
                            let a3410 = 2u8;
                            if equal(a3409, a3410) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3416 = {
                            let a3413 = {
                                let a3411 = v3;
                                let a3412 = 1u8;
                                fixed_u8::bit_and(a3411, a3412)
                            };
                            let a3414 = 1u8;
                            if equal(a3413, a3414) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3415, a3416)?
                    };
                    nat_add(a3417, a3418)?
                };
                nat_add(a3419, a3420)?
            };
            let a3421 = 128u64;
            let a3422 = v4;
            if if nat_le(a3421, a3422) {
                let a3423 = v4;
                let a3424 = 191u64;
                nat_le(a3423, a3424)
            } else {
                false
            } {
                Ok(Some({
                    let a3425 = v4;
                    let a3426 = 128u64;
                    nat_sub(a3425, a3426)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f518(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3427 = v1;
    let a3428 = v0;
    if if nat_le(a3427, a3428) {
        let a3429 = v0;
        let a3430 = v2;
        if nat_le(a3429, a3430) {
            let a3431 = v0;
            let a3432 = 55296u64;
            if nat_lt(a3431, a3432) {
                true
            } else {
                let a3433 = 57343u64;
                let a3434 = v0;
                nat_lt(a3433, a3434)
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

pub fn f519(p3451: u64, p3452: u64, p3453: u64, p3454: Bytes) -> Bytes {
    let m3450 = (p3451, (p3452, (p3453, p3454)));
    {
        let mut m3450 = m3450;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3450;
                let m3435 = v0;
                if m3435 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3435 - 1;
                    let a3436 = v1;
                    let a3437 = 0u64;
                    if if nat_eq(a3436, a3437) {
                        let a3438 = v2;
                        let a3439 = 0u64;
                        nat_eq(a3438, a3439)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3440 = v1;
                            let a3441 = 16u64;
                            let a3442 = 0u64;
                            nat_quot(a3440, a3441, a3442)
                        }, ({
                            let a3443 = v2;
                            let a3444 = 1u64;
                            nat_sub(a3443, a3444)
                        }, {
                            let a3448 = f526({
                                let a3445 = v1;
                                let a3446 = 16u64;
                                let a3447 = 0u64;
                                nat_rem(a3445, a3446, a3447)
                            });
                            let a3449 = v3;
                            append_bytes(a3448, a3449)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f520(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f329(v2) {
        true
    } else {
        f330(v2)
    } {
        let a3455 = v2;
        let a3456 = 117u64;
        if !nat_eq(a3455, a3456) {
            let a3457 = v2;
            let a3458 = 85u64;
            !nat_eq(a3457, a3458)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f331(v2) {
            let a3459 = 0u64;
            let a3460 = v1;
            nat_lt(a3459, a3460)
        } else {
            false
        }
    } {
        f32(v0, v1, 1u64)
    } else {
        let a3461 = v2;
        let a3462 = 65536u64;
        if nat_lt(a3461, a3462) {
            let a3463 = f8(Str::lit("u"));
            let a3464 = f519(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3463, a3464)
        } else {
            let a3465 = f8(Str::lit("U"));
            let a3466 = f519(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3465, a3466)
        }
    }
}

pub fn f521() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f522(v0: u64, v1: Adt14, v2: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m3467 = v0;
    if m3467 == 0 {
        Ok(f427(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3467 - 1;
        let m3468 = v1.clone();
        match m3468 {
            Adt14::C0(v4) => {
                if v4 {
                    Ok(f465())
                } else {
                    Ok(f427(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt14::C1(_, _) => {
                Ok(f427(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C2(_) => {
                Ok(f427(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C3(_) => {
                Ok(f427(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C4(_) => {
                let m3469 = f150(f142(v1.clone(), f8(Str::lit("$ref"))));
                match m3469 {
                    None => {
                        if if f436(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f436(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f436(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f428(v2.clone(), List::cons(Adt23::C0(v2, v1), List::<Adt23>::nil())))
                        } else {
                            let v17: Bytes = f145(v1.clone(), f8(Str::lit("type")));
                            if if f36(v17.clone(), Bytes::lit(&[])) {
                                !f506(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f465())
                            } else {
                                if f36(v17.clone(), f8(Str::lit("string"))) {
                                    Ok(f428(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                } else {
                                    if f36(v17.clone(), f8(Str::lit("integer"))) {
                                        Ok(f428(f507(v1), List::<Adt23>::nil()))
                                    } else {
                                        if f36(v17.clone(), f8(Str::lit("number"))) {
                                            Ok(f428(f508(), List::<Adt23>::nil()))
                                        } else {
                                            if f36(v17.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f428(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                            } else {
                                                if f36(v17.clone(), f8(Str::lit("array"))) {
                                                    let m3471 = f142(v1, f8(Str::lit("items")));
                                                    match m3471 {
                                                        None => {
                                                            Ok(f463(f8(Str::lit("List")), f465()))
                                                        }
                                                        Some(v18) => {
                                                            Ok(f463(f8(Str::lit("List")), f522(v3, v18, {
                                                                let a3472 = v2;
                                                                let a3473 = f8(Str::lit("Item"));
                                                                append_bytes(a3472, a3473)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f36(v17.clone(), f8(Str::lit("object"))) {
                                                        let m3474 = f509(v1);
                                                        match m3474 {
                                                            None => {
                                                                Ok(f463(f8(Str::lit("List")), f510(f465())))
                                                            }
                                                            Some(v19) => {
                                                                Ok(f463(f8(Str::lit("List")), f510(f522(v3, v19, {
                                                                    let a3475 = v2;
                                                                    let a3476 = f8(Str::lit("Value"));
                                                                    append_bytes(a3475, a3476)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f427(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v17, List::<Bytes>::nil()))), v2))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(v15) => {
                        let m3470 = f425(v15)?;
                        match m3470 {
                            None => {
                                Ok(f427(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v16) => {
                                Ok(f428(v16, List::<Adt23>::nil()))
                            }
                        }
                    }
                }
            }
            Adt14::C5 => {
                Ok(f465())
            }
            Adt14::C6(_, _) => {
                Ok(f427(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C7(_, _, _) => {
                Ok(f427(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f523(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f344(Fn10::F527, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f524(v0: Fn25, p3481: u64, p3482: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3480 = (p3481, p3482);
    {
        let mut m3480 = m3480;
        loop {
            let step = {
                let (v1, v2) = m3480;
                let m3477 = v1;
                if m3477 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3477 - 1;
                    let m3479 = {
                        let c3478: Fn25 = v0.clone();
                        c3478.apply(v2.clone())
                    };
                    match m3479 {
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
                Ok(next) => { m3480 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f528(v0)
}

pub fn f526(v0: u64) -> Bytes {
    f32({
        let a3483 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3483)
    }, v0, 1u64)
}

pub fn f527(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f528(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3484, _) = v0.clone();
        h3484
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3485, _) = {
            let (_, h3486) = v0.clone();
            h3486
        };
        h3485
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3487) = {
            let (_, h3488) = v0;
            h3488
        };
        h3487
    };
    let m3489 = v1.clone();
    match m3489.uncons() {
        None => {
            let m3490 = v2;
            match m3490.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3491 = v2.clone();
            match m3491.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f513({
                        let (h3492, _) = v8.clone();
                        h3492
                    }, {
                        let (h3493, _) = v6.clone();
                        h3493
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
