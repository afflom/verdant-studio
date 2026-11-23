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
    F95(u8, Bytes),
    F133(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F95(k0, k1) => f95(*k0, k1.clone(), p0),
            Fn0::F133(k0) => f133(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F114,
    F144,
    F156,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F114 => f114(p0, p1),
            Fn1::F144 => f144(p0, p1),
            Fn1::F156 => f156(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F129(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F129(k0) => f129(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F168(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt21)) -> R<(u64, Adt21)> {
        match self {
            Fn3::F168(k0, k1, k2) => f168(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F201,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn4::F201 => f201(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F204,
}

impl Fn5 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn5::F204 => f204(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F206,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn6::F206 => f206(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F209,
}

impl Fn7 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn7::F209 => f209(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F215(Bytes),
    F216(u64, Bytes),
    F267(Bytes),
    F310(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn8::F215(k0) => Ok(f215(k0.clone(), p0)),
            Fn8::F216(k0, k1) => f216(*k0, k1.clone(), p0),
            Fn8::F267(k0) => f267(k0.clone(), p0),
            Fn8::F310(k0) => f310(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F228(Bytes),
    F275(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F228(k0) => f228(k0.clone(), p0),
            Fn9::F275(k0, k1) => Ok(f275(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F230,
    F340,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F230 => f230(p0, p1),
            Fn10::F340 => f340(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F233,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F233 => f233(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F235,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F235 => f235(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F238,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F238 => f238(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F240,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt23) -> R<Adt23> {
        match self {
            Fn14::F240 => f240(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F245(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F245(k0) => f245(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F270(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F270(k0) => f270(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F279,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F279 => f279(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F281,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F281 => f281(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F283,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F283 => f283(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F291,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F291 => f291(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F302,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F302 => f302(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F324,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F324 => f324(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F326,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F326 => f326(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F328(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F328(k0) => f328(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F339,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F339 => f339(p0),
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
                    f28(v1)
                } else {
                    if f26(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f29(v1))
                    } else {
                        if f26(v2.clone(), f8(Str::lit("publish"))) {
                            Ok(Adt11::C3(List::cons((f30(), f31()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f32()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f33()), List::<(Bytes, Bytes)>::nil()))), f8(Str::lit("published .github/workflows/pages.yml, .github/workflows/operate.yml and .github/CODEOWNERS\u{a}"))))
                        } else {
                            let m128 = v3;
                            match m128.uncons() {
                                None => {
                                    Ok(Adt11::C4(f25()))
                                }
                                Some((v4, v5)) => {
                                    if f26(v2.clone(), f8(Str::lit("ingest-index"))) {
                                        f34(v1, v4)
                                    } else {
                                        if f26(v2.clone(), f8(Str::lit("stage-bindings"))) {
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
    f8(Str::lit("usage: verdant_studio check | next | stage-bindings JSONL | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
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

pub fn f28(v0: Adt1) -> R<Adt11> {
    if f55(f56(match v0.clone() {
        Adt1::C0(_, h163, _) => {
            h163
        }
    }, f57())) {
        Ok(Adt11::C1(List::cons(f57(), List::<Bytes>::nil())))
    } else {
        let v1: List<Bytes> = f58(f59(f57(), f60(match v0.clone() {
            Adt1::C0(_, h164, _) => {
                h164
            }
        }, f57())))?;
        let m165 = f41(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v1.clone()));
        match m165.uncons() {
            None => {
                let m166 = f61(v0, v1)?;
                match m166 {
                    None => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[])))
                    }
                    Some(v4) => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("ingest-authority ")), List::cons(v4, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            }
            Some((v2, v3)) => {
                Ok(Adt11::C0(List::cons(v2, v3)))
            }
        }
    }
}

pub fn f29(v0: Adt1) -> Adt11 {
    if f55(f62(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h167) => {
            h167
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f63(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f55(f62(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h168) => {
                h168
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f63(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f30() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f31() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - name: accept only pull requests made through the API\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error::$AUTHOR opened this pull request. Only $app, through the operate workflow, changes this repository.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          git diff --exit-code\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f32() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: operate\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check and build through the API\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}          if [ -n \"$INPUT\" ]; then mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"; fi\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            while op=$(target/release/verdant_studio next) && [ -n \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              target/release/verdant_studio $op | tee -a target/operation.txt\u{a}              last=$op\u{a}            done\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio check\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          git push origin \"HEAD:refs/heads/${branch}\"\u{a}          gh pr create --head \"$branch\" --base main --title \"verdant_studio $ARGS\" --body-file target/operation.txt\u{a}          if gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" --jq '[.[] | select(.type == \"required_status_checks\") | .parameters.required_status_checks[].context] | index(\"studio\")' | grep -qv null; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f33() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f34(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m169 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m169.uncons() {
        None => {
            let v4: Bytes = f50(v0.clone(), v1);
            let v5: List<List<Bytes>> = f64(v4.clone())?;
            if f55(f65(f66(v5.clone()), f67())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f68(Str::lit("id"), v6.clone())?, List::cons(f68(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f68(Str::lit("artifact"), f69())?, List::cons(f68(Str::lit("checksum"), f70(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f69(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f71(f50(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f72(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f73(f74(f75(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f35(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m170 = f41(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m170.uncons() {
        None => {
            let m171 = f76(f77(f50(v0, v1))?)?;
            match m171 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f73(f78(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f36(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m172 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m172.uncons() {
        None => {
            let m173 = f79(v0.clone(), v1.clone())?;
            match m173 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f55(f26(f80(v4.clone(), f8(Str::lit("schema"))), f81())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f80(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m174 = f82(v4.clone(), f8(Str::lit("source")));
                            match m174 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f83(f82(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f84(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f84(v9.clone(), f8(Str::lit("binding.json"))), f50(v0.clone(), v1));
                        let m175 = v8;
                        match m175 {
                            None => {
                                f85(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f55(f26(f80(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f85(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f84(v9, f86(f80(v7.clone(), f8(Str::lit("url"))))?);
                                    if f55(f62(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h176) => {
                                            h176
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f80(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m177 = f41(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m177.uncons() {
                                            None => {
                                                if f26(f70(f50(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f85(v0, v4, v5, v10, Some((v12, v11)))
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
    let m178 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil()))))));
    match m178.uncons() {
        None => {
            let m179 = f87(v3);
            match m179 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v6) => {
                    let m180 = f79(v0.clone(), v1.clone())?;
                    match m180 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                        }
                        Some(v7) => {
                            let v8: Bytes = {
                                let a181 = f8(Str::lit("Catalog."));
                                let a182 = f88(v2.clone())?;
                                append_bytes(a181, a182)
                            };
                            let v9: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f88(v2.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                            let m183 = f89(v7, v8.clone(), f88(v2.clone())?)?;
                            match m183 {
                                Ok(v11) => {
                                    Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f88(v2)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), f50(v0.clone(), v1)), List::cons((v9.clone(), v11), List::cons((f8(Str::lit("model/admissions.json")), f90(Adt14::C3(std::rc::Rc::new(f91(f92(f53(v0.clone(), f8(Str::lit("model/admissions.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v8.clone())), std::rc::Rc::new(v6)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f90(Adt14::C3(std::rc::Rc::new(f91(f92(f53(v0.clone(), f8(Str::lit("model/strata.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                        let a187 = f93(f50(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f40(v9.clone(), 4u64, {
                                            let a185 = {
                                                let a184 = v9;
                                                length_bytes(a184)
                                            };
                                            let a186 = 4u64;
                                            nat_sub(a185, a186)
                                        }))?;
                                        let a188 = Bytes::lit(&[0x0a]);
                                        append_bytes(a187, a188)
                                    }), List::cons(f72(v0, List::cons(v8.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v8, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
        let a189 = v0.clone();
        length_bytes(a189)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h190, _) = f94(Fn0::F95(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h190
    };
    let v4: u64 = {
        let (h191, _) = {
            let (_, h192) = v3.clone();
            h192
        };
        h191
    };
    Ok(f96(List::cons(f40(v0, v4, {
        let a193 = v2;
        let a194 = v4;
        nat_sub(a193, a194)
    }), {
        let (_, h195) = {
            let (_, h196) = v3;
            h196
        };
        h195
    })))
}

pub fn f39(p200: List<Bytes>) -> Bytes {
    let m199 = p200;
    {
        let mut m199 = m199;
        loop {
            let step = {
                let v0 = m199;
                let m197 = v0;
                match m197.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m198 = v2.clone();
                        match m198.uncons() {
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
                Ok(next) => { m199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m204 = {
        let a201 = v0;
        let a202 = v1;
        let a203 = v2;
        slice_bytes(a201, a202, a203)
    };
    match m204 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f41(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m205 = v1;
    match m205.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m207 = f97(match v0.clone() {
                Adt1::C0(h206, _, _) => {
                    h206
                }
            }, v2.clone());
            match m207 {
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
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f69(), List::<Bytes>::nil())))))))
}

pub fn f43(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m208 = v1;
    match m208.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f98(v0.clone(), v2.clone()) {
                f99(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m209 = f79(v0.clone(), v2.clone())?;
                match m209 {
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
    let m210 = v1;
    match m210.uncons() {
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
    let m211 = v0;
    match m211.uncons() {
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
    let m212 = v0;
    match m212.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f45(f100({
                let (h213, _) = v2;
                h213
            }, {
                let (_, h214) = v2;
                h214
            }, v1), f47(v3, v1))
        }
    }
}

pub fn f48(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h215, _, _, _, _) => {
            h215
        }
    };
    let v2: Adt15 = f101(v1.clone())?;
    let v3: List<Bytes> = f102(v1.clone());
    let v4: List<Bytes> = f52(match v0.clone() {
        Adt13::C0(_, _, h216, _, _) => {
            h216
        }
    }, f8(Str::lit("path")));
    let m218 = f103(v2, List::<u64>::nil(), f104(v1.clone(), match v0.clone() {
        Adt13::C0(_, h217, _, _, _) => {
            h217
        }
    }, 0u64)?);
    match m218 {
        None => {
            if f106(f107(match v0.clone() {
                Adt13::C0(_, _, _, _, h220) => {
                    h220
                }
            }, f45(match v0.clone() {
                Adt13::C0(_, _, _, _, h221) => {
                    h221
                }
            }, v4.clone()))?, f107(v4.clone(), f45(match v0.clone() {
                Adt13::C0(_, _, _, _, h222) => {
                    h222
                }
            }, v4))?, f108(match v0.clone() {
                Adt13::C0(_, _, h223, _, _) => {
                    h223
                }
            })) {
                let m227 = f109(f107(match v0.clone() {
                    Adt13::C0(_, _, _, h224, _) => {
                        h224
                    }
                }, f45(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h225, _) => {
                        h225
                    }
                }))?, f107(v3.clone(), f45(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h226, _) => {
                        h226
                    }
                }))?);
                match m227 {
                    None => {
                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f73(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f105(f45(v3, match v0 {
                            Adt13::C0(_, _, _, h228, _) => {
                                h228
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f105(match v1 {
                Adt12::C0(_, _, _, _, h219) => {
                    h219
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f49(v0: Bytes) -> R<List<Bytes>> {
    f110(f77(v0)?, f8(Str::lit("id = \"")))
}

pub fn f50(v0: Adt1, v1: Bytes) -> Bytes {
    let m230 = f97(match v0 {
        Adt1::C0(h229, _, _) => {
            h229
        }
    }, v1);
    match m230 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m231 = v2;
            match m231 {
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
    let m232 = f64(f50(v0, f69()))?;
    match m232.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f74(v2))
        }
    }
}

pub fn f52(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m233 = v0;
    match m233.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f80(v2, v1.clone()), f52(v3, v1))
        }
    }
}

pub fn f53(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f111(f79(v0, v1)?))
}

pub fn f54(v0: List<Adt14>) -> List<Bytes> {
    let m234 = v0;
    match m234.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m235 = f83(Some(v1));
            match m235 {
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

pub fn f56(p239: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m238 = p239;
    {
        let mut m238 = m238;
        loop {
            let step = {
                let v0 = m238;
                let m236 = v0;
                match m236.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h237, _) = v2;
                            h237
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f58(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f112(f113(Fn1::F114, List::<Bytes>::nil(), f115(v0)?)))
}

pub fn f59(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m240 = v1;
    match m240.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f99(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f84(v0.clone(), v2), f59(v0, v3))
            } else {
                f59(v0, v3)
            }
        }
    }
}

pub fn f60(p245: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m244 = p245;
    {
        let mut m244 = m244;
        loop {
            let step = {
                let v0 = m244;
                let m241 = v0;
                match m241.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h242, _) = v2;
                            h242
                        }, v1.clone()) {
                            let (_, h243) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h243)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt1, p248: List<Bytes>) -> R<Option<Bytes>> {
    let m247 = p248;
    {
        let mut m247 = m247;
        loop {
            let step = {
                let v1 = m247;
                let m246 = v1;
                match m246.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        if f116(v0.clone(), v2.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v2)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
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

pub fn f62(v0: Bytes, p251: List<Bytes>) -> bool {
    let m250 = p251;
    {
        let mut m250 = m250;
        loop {
            let step = {
                let v1 = m250;
                let m249 = v1;
                match m249.uncons() {
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
                Ok(next) => { m250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f64(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt16 = f117(v0.clone(), 0u64, {
        let a252 = v0;
        length_bytes(a252)
    }, Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt16 = {
        let a255 = {
            let a254 = match v1.clone() {
                Adt16::C0(_, _, h253, _, _) => {
                    h253
                }
            };
            length_bytes(a254)
        };
        let a256 = 0u64;
        if if nat_eq(a255, a256) {
            let m258 = match v1.clone() {
                Adt16::C0(_, _, _, h257, _) => {
                    h257
                }
            };
            match m258.uncons() {
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
            f118(v1)
        }
    };
    Ok(f119(match v4 {
        Adt16::C0(_, _, _, _, h259) => {
            h259
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f65(p264: List<Bytes>, p265: List<Bytes>) -> bool {
    let m263 = (p264, p265);
    {
        let mut m263 = m263;
        loop {
            let step = {
                let (v0, v1) = m263;
                let m260 = v0;
                match m260.uncons() {
                    None => {
                        let m261 = v1;
                        match m261.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m262 = v1;
                        match m262.uncons() {
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
                Ok(next) => { m263 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<List<Bytes>>) -> List<Bytes> {
    let m266 = v0;
    match m266.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f67() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f68(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f120(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f69() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f70(v0: Bytes) -> R<Bytes> {
    Ok(f121(f122(v0)?))
}

pub fn f71(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f123(f124(f77(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f125(f126(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f72(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f90(Adt14::C3(std::rc::Rc::new(f127(f58(f45(v1, f54(f53(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f73(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m267 = f128(Fn2::F129(v1), None::<Bytes>, v0);
    match m267 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f74(v0: List<List<Bytes>>) -> List<Bytes> {
    let m268 = v0;
    match m268.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m269 = v1;
            match m269.uncons() {
                None => {
                    f74(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f74(v2))
                }
            }
        }
    }
}

pub fn f75(v0: List<List<Bytes>>) -> List<List<Bytes>> {
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

pub fn f76(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m271 = v0;
    match m271.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f130(v1)?;
            let a273 = {
                let a272 = v3.clone();
                length_bytes(a272)
            };
            let a274 = 0u64;
            if nat_eq(a273, a274) {
                f76(v2)
            } else {
                let m275 = f131(v3.clone())?;
                match m275 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f80(v4.clone(), f8(Str::lit("id")));
                        if if f26(f80(v4, f8(Str::lit("schema"))), f81()) {
                            f55({
                                let a277 = {
                                    let a276 = v5.clone();
                                    length_bytes(a276)
                                };
                                let a278 = 0u64;
                                nat_eq(a277, a278)
                            })
                        } else {
                            false
                        } {
                            let m279 = f76(v2)?;
                            match m279 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f132(v5), {
                                        let a280 = v3;
                                        let a281 = Bytes::lit(&[0x0a]);
                                        append_bytes(a280, a281)
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

pub fn f77(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a282 = v0.clone();
        length_bytes(a282)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h283, _) = f94(Fn0::F133(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h283
    };
    let v3: u64 = {
        let (h284, _) = {
            let (_, h285) = v2.clone();
            h285
        };
        h284
    };
    let v4: List<Bytes> = {
        let (_, h286) = {
            let (_, h287) = v2;
            h287
        };
        h286
    };
    let a288 = v3;
    let a289 = v1;
    if nat_lt(a288, a289) {
        Ok(f96(List::cons(f40(v0, v3, {
            let a290 = v1;
            let a291 = v3;
            nat_sub(a290, a291)
        }), v4)))
    } else {
        Ok(f96(v4))
    }
}

pub fn f78(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m292 = v0;
    match m292.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h293, _) = v1;
                h293
            }, f78(v2))
        }
    }
}

pub fn f79(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f131(f50(v0, v1))
}

pub fn f80(v0: Adt14, v1: Bytes) -> Bytes {
    let m294 = f83(f82(v0, v1));
    match m294 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f81() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f82(p300: Adt14, v1: Bytes) -> Option<Adt14> {
    let m299 = p300;
    {
        let mut m299 = m299;
        loop {
            let step = {
                let v0 = m299;
                let m295 = v0;
                match m295 {
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
                    Adt14::C4(r296) => {
                        let v5 = (*r296).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r297, r298) => {
                        let v3 = (*r297).clone();
                        let v4 = (*r298).clone();
                        if f26(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
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

pub fn f83(v0: Option<Adt14>) -> Option<Bytes> {
    let m301 = v0;
    match m301 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m302 = v1;
            match m302 {
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

pub fn f84(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f85(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f71(f50(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f134(v1, v4)?)?), List::cons(f72(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f86(v0: Bytes) -> R<Bytes> {
    Ok(f39(f38(v0, 47u8)?))
}

pub fn f87(v0: List<Bytes>) -> Option<Adt14> {
    let m303 = v0;
    match m303.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f26(v1.clone(), f8(Str::lit("req"))) {
                let m304 = v2;
                match m304.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f26(v1.clone(), f8(Str::lit("std"))) {
                    let m305 = v2;
                    match m305.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m306 = v6;
                                match m306.uncons() {
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
                        let m307 = v2;
                        match m307.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m308 = v10;
                                match m308.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f127(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f88(v0: Bytes) -> R<Bytes> {
    f135(v0.clone(), 0u64, {
        let a309 = v0;
        length_bytes(a309)
    }, true, Bytes::lit(&[]))
}

pub fn f89(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m310 = f136(4096u64, f137(f138(f139(f82(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt17::C0(v2, v0), List::<Adt17>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m310 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f126(f140(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f90(v0: Adt14) -> R<Bytes> {
    let a311 = f141(v0)?;
    let a312 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a311, a312))
}

pub fn f91(v0: List<Adt14>) -> Adt14 {
    let m313 = v0;
    match m313.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f91(v2)))
        }
    }
}

pub fn f92(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    let m314 = v0;
    match m314.uncons() {
        None => {
            List::cons(v1, List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            List::cons(v2, f92(v3, v1))
        }
    }
}

pub fn f93(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f126(f142(f77(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f94(v0: Fn0, p319: u64, p320: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m318 = (p319, p320);
    {
        let mut m318 = m318;
        loop {
            let step = {
                let (v1, v2) = m318;
                let m315 = v1;
                if m315 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m315 - 1;
                    let m317 = {
                        let c316: Fn0 = v0.clone();
                        c316.apply(v2.clone())?
                    };
                    match m317 {
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
                Ok(next) => { m318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f143(v1, v0, v2)
}

pub fn f96(v0: List<Bytes>) -> List<Bytes> {
    f113(Fn1::F144, List::<Bytes>::nil(), v0)
}

pub fn f97(p325: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m324 = p325;
    {
        let mut m324 = m324;
        loop {
            let step = {
                let v0 = m324;
                let m321 = v0;
                match m321.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h322, _) = v2;
                            h322
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h323) = v2;
                                h323
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: Adt1, v1: Bytes) -> bool {
    let m327 = f97(match v0 {
        Adt1::C0(h326, _, _) => {
            h326
        }
    }, v1);
    match m327 {
        None => {
            false
        }
        Some(v2) => {
            let m328 = v2;
            match m328 {
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

pub fn f99(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a329 = v0.clone();
        length_bytes(a329)
    };
    let v3: u64 = {
        let a330 = v1.clone();
        length_bytes(a330)
    };
    let a331 = v3;
    let a332 = v2;
    if nat_le(a331, a332) {
        let m338 = {
            let a335 = v0;
            let a336 = {
                let a333 = v2;
                let a334 = v3;
                nat_sub(a333, a334)
            };
            let a337 = v3;
            slice_bytes(a335, a336, a337)
        };
        match m338 {
            None => {
                false
            }
            Some(v4) => {
                let a339 = v4;
                let a340 = v1;
                equal(a339, a340)
            }
        }
    } else {
        false
    }
}

pub fn f100(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m341 = v1;
    match m341.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f145(v3.clone())
            } else {
                f55(f145(v3.clone()))
            } {
                List::cons(f84(v0.clone(), v3), f100(v0, v4, v2))
            } else {
                f100(v0, v4, v2)
            }
        }
    }
}

pub fn f101(v0: Adt12) -> R<Adt15> {
    Ok(Adt15::C0(f146(match v0.clone() {
        Adt12::C0(h342, _, _, _, _) => {
            h342
        }
    }, 0u64)?, f146(match v0.clone() {
        Adt12::C0(_, h343, _, _, _) => {
            h343
        }
    }, 0u64)?, f146(match v0.clone() {
        Adt12::C0(_, _, h344, _, _) => {
            h344
        }
    }, 0u64)?, f146(match v0 {
        Adt12::C0(_, _, _, h345, _) => {
            h345
        }
    }, 0u64)?))
}

pub fn f102(v0: Adt12) -> List<Bytes> {
    f45(match v0.clone() {
        Adt12::C0(h346, _, _, _, _) => {
            h346
        }
    }, f45(match v0.clone() {
        Adt12::C0(_, _, h347, _, _) => {
            h347
        }
    }, f45(match v0.clone() {
        Adt12::C0(_, _, _, h348, _) => {
            h348
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h349) => {
            h349
        }
    })))
}

pub fn f103(v0: Adt15, p355: List<u64>, p356: List<Adt18>) -> Option<u64> {
    let m354 = (p355, p356);
    {
        let mut m354 = m354;
        loop {
            let step = {
                let (v1, v2) = m354;
                let m350 = v2;
                match m350.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f147(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h351) => {
                                h351
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h352, _) => {
                                    h352
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h353, _) => {
                                    h353
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m357 = v1;
    match m357.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f148(v0.clone(), v3)?), f104(v0, v4, {
                let a358 = v2;
                let a359 = 1u64;
                nat_add(a358, a359)?
            })?))
        }
    }
}

pub fn f105(p363: List<Bytes>, p364: u64) -> Bytes {
    let m362 = (p363, p364);
    {
        let mut m362 = m362;
        loop {
            let step = {
                let (v0, v1) = m362;
                let m360 = v0;
                match m360.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m361 = v1;
                        if m361 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m361 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m362 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f149(v0.clone(), v1.clone()) {
        if f149(v1, v0) {
            f150(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f107(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m365 = v0;
    match m365.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f151(v2, v1.clone())?, f107(v3, v1)?))
        }
    }
}

pub fn f108(v0: List<Adt14>) -> List<u64> {
    let m366 = v0;
    match m366.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f152(v1), f108(v2))
        }
    }
}

pub fn f109(p369: List<u64>, v1: List<u64>) -> Option<u64> {
    let m368 = p369;
    {
        let mut m368 = m368;
        loop {
            let step = {
                let v0 = m368;
                let m367 = v0;
                match m367.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f153(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f110(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m370 = v0;
    match m370.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m371 = f154(v2, v1.clone())?;
            match m371 {
                None => {
                    f110(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f110(v3, v1)?))
                }
            }
        }
    }
}

pub fn f111(v0: Option<Adt14>) -> List<Adt14> {
    let m372 = v0;
    match m372 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f155(v1)
        }
    }
}

pub fn f112(v0: List<Bytes>) -> List<Bytes> {
    f113(Fn1::F156, List::<Bytes>::nil(), v0)
}

pub fn f113(v0: Fn1, p376: List<Bytes>, p377: List<Bytes>) -> List<Bytes> {
    let m375 = (p376, p377);
    {
        let mut m375 = m375;
        loop {
            let step = {
                let (v1, v2) = m375;
                let m373 = v2;
                match m373.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c374: Fn1 = v0.clone();
                            c374.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m375 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f157(v0, v1)
}

pub fn f115(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f158(f159(f160(v0))?))
}

pub fn f116(v0: Adt1, v1: Bytes) -> R<bool> {
    let m378 = f79(v0.clone(), v1)?;
    match m378 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let v3: List<Bytes> = f161(f124(f77(f50(v0, f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f80(v2.clone(), f8(Str::lit("id"))))?;
            let m379 = v3.clone();
            match m379.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m381 = f83(f82({
                        let m380 = f82(v2, f8(Str::lit("source")));
                        match m380 {
                            None => {
                                Adt14::C5
                            }
                            Some(v6) => {
                                v6
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m381 {
                        None => {
                            Ok(false)
                        }
                        Some(v7) => {
                            Ok(f55(f62(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f120(v7)?, List::<Bytes>::nil()))), v3)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f117(v0: Bytes, p439: u64, p440: u64, p441: Adt16) -> R<Adt16> {
    let m438 = (p439, (p440, p441));
    {
        let mut m438 = m438;
        loop {
            let step = {
                let (v1, (v2, v3)) = m438;
                let m382 = v2;
                if m382 == 0 {
                    Err::<(u64, (u64, Adt16)), R<Adt16>>(Ok(v3))
                } else {
                    let v4 = m382 - 1;
                    let v5: u64 = f162(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt16::C0(_, h383, _, _, _) => {
                            h383
                        }
                    } {
                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                            let a384 = v1;
                            let a385 = 1u64;
                            nat_add(a384, a385)?
                        }, (v4, Adt16::C0(true, false, match v3.clone() {
                            Adt16::C0(_, _, h386, _, _) => {
                                h386
                            }
                        }, match v3.clone() {
                            Adt16::C0(_, _, _, h387, _) => {
                                h387
                            }
                        }, match v3 {
                            Adt16::C0(_, _, _, _, h388) => {
                                h388
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt16::C0(h389, _, _, _, _) => {
                                h389
                            }
                        } {
                            let a390 = v5;
                            let a391 = 34u64;
                            if nat_eq(a390, a391) {
                                let a394 = f162(v0.clone(), {
                                    let a392 = v1;
                                    let a393 = 1u64;
                                    nat_add(a392, a393)?
                                })?;
                                let a395 = 34u64;
                                if nat_eq(a394, a395) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a396 = v1;
                                        let a397 = 1u64;
                                        nat_add(a396, a397)?
                                    }, (v4, Adt16::C0(true, true, f163(match v3.clone() {
                                        Adt16::C0(_, _, h398, _, _) => {
                                            h398
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt16::C0(_, _, _, h399, _) => {
                                            h399
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h400) => {
                                            h400
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a401 = v1;
                                        let a402 = 1u64;
                                        nat_add(a401, a402)?
                                    }, (v4, Adt16::C0(false, false, match v3.clone() {
                                        Adt16::C0(_, _, h403, _, _) => {
                                            h403
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h404, _) => {
                                            h404
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h405) => {
                                            h405
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a406 = v1;
                                    let a407 = 1u64;
                                    nat_add(a406, a407)?
                                }, (v4, Adt16::C0(true, false, f163(match v3.clone() {
                                    Adt16::C0(_, _, h408, _, _) => {
                                        h408
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt16::C0(_, _, _, h409, _) => {
                                        h409
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h410) => {
                                        h410
                                    }
                                }))))
                            }
                        } else {
                            let a411 = v5;
                            let a412 = 34u64;
                            if nat_eq(a411, a412) {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a413 = v1;
                                    let a414 = 1u64;
                                    nat_add(a413, a414)?
                                }, (v4, Adt16::C0(true, false, match v3.clone() {
                                    Adt16::C0(_, _, h415, _, _) => {
                                        h415
                                    }
                                }, match v3.clone() {
                                    Adt16::C0(_, _, _, h416, _) => {
                                        h416
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h417) => {
                                        h417
                                    }
                                }))))
                            } else {
                                let a418 = v5;
                                let a419 = 44u64;
                                if nat_eq(a418, a419) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a420 = v1;
                                        let a421 = 1u64;
                                        nat_add(a420, a421)?
                                    }, (v4, Adt16::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt16::C0(_, _, h422, _, _) => {
                                            h422
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h423, _) => {
                                            h423
                                        }
                                    }), match v3 {
                                        Adt16::C0(_, _, _, _, h424) => {
                                            h424
                                        }
                                    }))))
                                } else {
                                    let a425 = v5;
                                    let a426 = 13u64;
                                    if nat_eq(a425, a426) {
                                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                            let a427 = v1;
                                            let a428 = 1u64;
                                            nat_add(a427, a428)?
                                        }, (v4, v3)))
                                    } else {
                                        let a429 = v5;
                                        let a430 = 10u64;
                                        if nat_eq(a429, a430) {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a431 = v1;
                                                let a432 = 1u64;
                                                nat_add(a431, a432)?
                                            }, (v4, f118(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a433 = v1;
                                                let a434 = 1u64;
                                                nat_add(a433, a434)?
                                            }, (v4, Adt16::C0(false, false, f163(match v3.clone() {
                                                Adt16::C0(_, _, h435, _, _) => {
                                                    h435
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt16::C0(_, _, _, h436, _) => {
                                                    h436
                                                }
                                            }, match v3 {
                                                Adt16::C0(_, _, _, _, h437) => {
                                                    h437
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
                Ok(next) => { m438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: Adt16) -> Adt16 {
    Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f164(List::cons(match v0.clone() {
        Adt16::C0(_, _, h442, _, _) => {
            h442
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h443, _) => {
            h443
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt16::C0(_, _, _, _, h444) => {
            h444
        }
    }))
}

pub fn f119(p447: List<List<Bytes>>, p448: List<List<Bytes>>) -> List<List<Bytes>> {
    let m446 = (p447, p448);
    {
        let mut m446 = m446;
        loop {
            let step = {
                let (v0, v1) = m446;
                let m445 = v0;
                match m445.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f165(v0.clone(), 0u64, {
        let a449 = v0;
        length_bytes(a449)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f121(v0: Adt21) -> Bytes {
    let a470 = f166(match v0.clone() {
        Adt21::C0(h450, _, _, _, _, _, _, _) => {
            h450
        }
    });
    let a471 = {
        let a468 = f166(match v0.clone() {
            Adt21::C0(_, h451, _, _, _, _, _, _) => {
                h451
            }
        });
        let a469 = {
            let a466 = f166(match v0.clone() {
                Adt21::C0(_, _, h452, _, _, _, _, _) => {
                    h452
                }
            });
            let a467 = {
                let a464 = f166(match v0.clone() {
                    Adt21::C0(_, _, _, h453, _, _, _, _) => {
                        h453
                    }
                });
                let a465 = {
                    let a462 = f166(match v0.clone() {
                        Adt21::C0(_, _, _, _, h454, _, _, _) => {
                            h454
                        }
                    });
                    let a463 = {
                        let a460 = f166(match v0.clone() {
                            Adt21::C0(_, _, _, _, _, h455, _, _) => {
                                h455
                            }
                        });
                        let a461 = {
                            let a458 = f166(match v0.clone() {
                                Adt21::C0(_, _, _, _, _, _, h456, _) => {
                                    h456
                                }
                            });
                            let a459 = f166(match v0 {
                                Adt21::C0(_, _, _, _, _, _, _, h457) => {
                                    h457
                                }
                            });
                            append_bytes(a458, a459)
                        };
                        append_bytes(a460, a461)
                    };
                    append_bytes(a462, a463)
                };
                append_bytes(a464, a465)
            };
            append_bytes(a466, a467)
        };
        append_bytes(a468, a469)
    };
    append_bytes(a470, a471)
}

pub fn f122(v0: Bytes) -> R<Adt21> {
    let v1: u64 = {
        let a472 = v0.clone();
        length_bytes(a472)
    };
    let v2: u64 = {
        let a478 = {
            let a475 = {
                let a473 = v1;
                let a474 = 72u64;
                nat_add(a473, a474)?
            };
            let a476 = 64u64;
            let a477 = 0u64;
            nat_quot(a475, a476, a477)
        };
        let a479 = 64u64;
        nat_mul(a478, a479)?
    };
    let (_, h480) = f167(Fn3::F168(v0, v1, v2), {
        let a481 = v2;
        let a482 = 64u64;
        let a483 = 0u64;
        nat_quot(a481, a482, a483)
    }, (0u64, f169()))?;
    Ok(h480)
}

pub fn f123(p486: List<List<Bytes>>, v1: Bytes, p487: List<Bytes>) -> R<List<Bytes>> {
    let m485 = (p486, p487);
    {
        let mut m485 = m485;
        loop {
            let step = {
                let (v0, v2) = m485;
                let m484 = v0;
                match m484.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f170(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f126(f171(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(p490: List<Bytes>, p491: List<Bytes>, p492: List<List<Bytes>>) -> List<List<Bytes>> {
    let m489 = (p490, (p491, p492));
    {
        let mut m489 = m489;
        loop {
            let step = {
                let (v0, (v1, v2)) = m489;
                let m488 = v0;
                match m488.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f140(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f26(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f140(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
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

pub fn f125(v0: Bytes) -> R<Bytes> {
    Ok(f40(v0.clone(), 0u64, f172(v0)?))
}

pub fn f126(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m493 = v0;
    match m493.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m494 = v3.clone();
            match m494.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a497 = v2;
                    let a498 = {
                        let a495 = v1.clone();
                        let a496 = f126(v3, v1);
                        append_bytes(a495, a496)
                    };
                    append_bytes(a497, a498)
                }
            }
        }
    }
}

pub fn f127(v0: List<Bytes>) -> Adt14 {
    let m499 = v0;
    match m499.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f127(v2)))
        }
    }
}

pub fn f128(v0: Fn2, p503: Option<Bytes>, p504: List<Bytes>) -> Option<Bytes> {
    let m502 = (p503, p504);
    {
        let mut m502 = m502;
        loop {
            let step = {
                let (v1, v2) = m502;
                let m500 = v2;
                match m500.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c501: Fn2 = v0.clone();
                            c501.apply(v1, v3)
                        }, v4))
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

pub fn f129(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m505 = v1;
    match m505 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a508 = {
                    let a506 = v3;
                    let a507 = v0;
                    append_bytes(a506, a507)
                };
                let a509 = v2;
                append_bytes(a508, a509)
            })
        }
    }
}

pub fn f130(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f172(v0.clone())?;
    let v2: u64 = f173(v0.clone(), v1)?;
    Ok(f40(v0, v2, {
        let a510 = v1;
        let a511 = v2;
        nat_sub(a510, a511)
    }))
}

pub fn f131(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f174(v0)?;
    let m515 = f175(v1.clone(), 0u64, {
        let a513 = {
            let a512 = v1.clone();
            length_bytes(a512)
        };
        let a514 = 2u64;
        nat_add(a513, a514)?
    }, 0u64)?;
    match m515 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a518 = {
                let (_, h516) = v2;
                h516
            };
            let a519 = {
                let a517 = v1;
                length_bytes(a517)
            };
            if nat_eq(a518, a519) {
                Ok(Some({
                    let (h520, _) = v2;
                    h520
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f132(v0: Bytes) -> Bytes {
    f7(List::cons(f57(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f133(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f176(v0, v1)
}

pub fn f134(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f82(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f68(Str::lit("id"), f80(v0.clone(), f8(Str::lit("id"))))?, List::cons(f68(Str::lit("name"), f80(v0.clone(), f8(Str::lit("name"))))?, List::cons(f68(Str::lit("issuer"), f80(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f68(Str::lit("edition"), f80(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f68(Str::lit("canonical_id"), f80(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f68(Str::lit("source_role"), f80(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f68(Str::lit("retrieval_date"), f80(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f68(Str::lit("url"), f177(v2.clone(), f8(Str::lit("url"))))?, List::cons(f68(Str::lit("media_type"), f177(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f68(Str::lit("license"), f177(v2.clone(), f8(Str::lit("license"))))?, List::cons(f68(Str::lit("redistribution"), f177(v2, f8(Str::lit("redistribution"))))?, List::cons(f68(Str::lit("statement"), f80(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m521 = v1;
        match m521 {
            None => {
                f7(List::cons(f68(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f68(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f68(Str::lit("artifact"), {
                    let (h522, _) = v3;
                    h522
                })?, List::cons(f68(Str::lit("checksum"), {
                    let (_, h523) = v3;
                    h523
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f178(f54(f111(f82(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f135(v0: Bytes, p590: u64, p591: u64, p592: bool, p593: Bytes) -> R<Bytes> {
    let m589 = (p590, (p591, (p592, p593)));
    {
        let mut m589 = m589;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m589;
                let m524 = v2;
                if m524 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m524 - 1;
                    let m527 = {
                        let a525 = v0.clone();
                        let a526 = v1;
                        index_bytes(a525, a526)
                    };
                    match m527 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a572 = {
                                    let a548 = {
                                        let a536 = {
                                            let a530 = {
                                                let a528 = v7;
                                                let a529 = 128u8;
                                                fixed_u8::bit_and(a528, a529)
                                            };
                                            let a531 = 128u8;
                                            if equal(a530, a531) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a537 = {
                                            let a534 = {
                                                let a532 = v7;
                                                let a533 = 64u8;
                                                fixed_u8::bit_and(a532, a533)
                                            };
                                            let a535 = 64u8;
                                            if equal(a534, a535) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a536, a537)?
                                    };
                                    let a549 = {
                                        let a546 = {
                                            let a540 = {
                                                let a538 = v7;
                                                let a539 = 32u8;
                                                fixed_u8::bit_and(a538, a539)
                                            };
                                            let a541 = 32u8;
                                            if equal(a540, a541) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a547 = {
                                            let a544 = {
                                                let a542 = v7;
                                                let a543 = 16u8;
                                                fixed_u8::bit_and(a542, a543)
                                            };
                                            let a545 = 16u8;
                                            if equal(a544, a545) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a546, a547)?
                                    };
                                    nat_add(a548, a549)?
                                };
                                let a573 = {
                                    let a570 = {
                                        let a558 = {
                                            let a552 = {
                                                let a550 = v7;
                                                let a551 = 8u8;
                                                fixed_u8::bit_and(a550, a551)
                                            };
                                            let a553 = 8u8;
                                            if equal(a552, a553) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a559 = {
                                            let a556 = {
                                                let a554 = v7;
                                                let a555 = 4u8;
                                                fixed_u8::bit_and(a554, a555)
                                            };
                                            let a557 = 4u8;
                                            if equal(a556, a557) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a558, a559)?
                                    };
                                    let a571 = {
                                        let a568 = {
                                            let a562 = {
                                                let a560 = v7;
                                                let a561 = 2u8;
                                                fixed_u8::bit_and(a560, a561)
                                            };
                                            let a563 = 2u8;
                                            if equal(a562, a563) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a569 = {
                                            let a566 = {
                                                let a564 = v7;
                                                let a565 = 1u8;
                                                fixed_u8::bit_and(a564, a565)
                                            };
                                            let a567 = 1u8;
                                            if equal(a566, a567) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a568, a569)?
                                    };
                                    nat_add(a570, a571)?
                                };
                                nat_add(a572, a573)?
                            };
                            if f179(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a574 = v1;
                                    let a575 = 1u64;
                                    nat_add(a574, a575)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f180(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a576 = v1;
                                        let a577 = 1u64;
                                        nat_add(a576, a577)?
                                    }, (v5, (false, {
                                        let a581 = v4;
                                        let a582 = f40({
                                            let a578 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a578)
                                        }, {
                                            let a579 = v8;
                                            let a580 = 97u64;
                                            nat_sub(a579, a580)
                                        }, 1u64);
                                        append_bytes(a581, a582)
                                    }))))
                                } else {
                                    if if f180(v8) {
                                        true
                                    } else {
                                        if f181(v8) {
                                            true
                                        } else {
                                            f182(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a583 = v1;
                                            let a584 = 1u64;
                                            nat_add(a583, a584)?
                                        }, (v5, (false, {
                                            let a585 = v4;
                                            let a586 = f40(v0.clone(), v1, 1u64);
                                            append_bytes(a585, a586)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a587 = v1;
                                            let a588 = 1u64;
                                            nat_add(a587, a588)?
                                        }, (v5, (v3, v4))))
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

pub fn f136(p601: u64, p602: List<Adt17>, p603: List<Bytes>, p604: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m600 = (p601, (p602, (p603, p604)));
    {
        let mut m600 = m600;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m600;
                let m594 = v0;
                if m594 == 0 {
                    Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m594 - 1;
                    let m595 = v1;
                    match m595.uncons() {
                        None => {
                            Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m596 = v5;
                            match m596 {
                                Adt17::C0(v8, v9) => {
                                    if f62(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m597 = f183(v8.clone(), v9)?;
                                        match m597 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f137({
                                                    let (_, h598) = v11;
                                                    h598
                                                }, List::cons(Adt17::C1({
                                                    let (h599, _) = v11;
                                                    h599
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
                Ok(next) => { m600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: List<Adt17>, v1: List<Adt17>) -> List<Adt17> {
    let m605 = v0;
    match m605.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f137(v3, v1))
        }
    }
}

pub fn f138(v0: List<(Bytes, Adt14)>) -> R<List<Adt17>> {
    let m606 = v0;
    match m606.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt17::C0(f88({
                let (h607, _) = v1;
                h607
            })?, {
                let (_, h608) = v1;
                h608
            }), f138(v2)?))
        }
    }
}

pub fn f139(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m609 = v0;
    match m609 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f184(v1)
        }
    }
}

pub fn f140(p612: List<Bytes>, p613: List<Bytes>) -> List<Bytes> {
    let m611 = (p612, p613);
    {
        let mut m611 = m611;
        loop {
            let step = {
                let (v0, v1) = m611;
                let m610 = v0;
                match m610.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m611 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Adt14) -> R<Bytes> {
    let m614 = v0;
    match m614 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f185(v2, v3)
        }
        Adt14::C2(v4) => {
            f186(v4)
        }
        Adt14::C3(r615) => {
            let v5 = (*r615).clone();
            Ok(f187(f188(v5)?))
        }
        Adt14::C4(r616) => {
            let v6 = (*r616).clone();
            f189(f190(v6)?)
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

pub fn f142(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m617 = v0;
    match m617.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f191(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f126(f192(f58(List::cons(v1, f193(f38(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f142(v3, v1)?))
            }
        }
    }
}

pub fn f143(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h618, _) = v2.clone();
        h618
    };
    let v4: u64 = {
        let (h619, _) = {
            let (_, h620) = v2.clone();
            h620
        };
        h619
    };
    let v5: List<Bytes> = {
        let (_, h621) = {
            let (_, h622) = v2;
            h622
        };
        h621
    };
    let m623 = f194(v0.clone(), v3);
    match m623 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a624 = v6;
            let a625 = v1;
            if equal(a624, a625) {
                Ok(Some(({
                    let a626 = v3;
                    let a627 = 1u64;
                    nat_add(a626, a627)?
                }, ({
                    let a628 = v3;
                    let a629 = 1u64;
                    nat_add(a628, a629)?
                }, List::cons(f40(v0, v4, {
                    let a630 = v3;
                    let a631 = v4;
                    nat_sub(a630, a631)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a632 = v3;
                    let a633 = 1u64;
                    nat_add(a632, a633)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f144(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f145(v0: Bytes) -> bool {
    f99(v0, f8(Str::lit(".lex.tex")))
}

pub fn f146(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m634 = v0;
    match m634.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f146(v3, {
                let a635 = v1;
                let a636 = 1u64;
                nat_add(a635, a636)?
            })?))
        }
    }
}

pub fn f147(v0: Adt15, v1: List<u64>, v2: Adt19) -> bool {
    let m637 = v2;
    match m637 {
        Adt19::C0(v3, v4) => {
            if f153(v3, match v0.clone() {
                Adt15::C0(h638, _, _, _) => {
                    h638
                }
            }) {
                f195(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f153(v5, match v0 {
                Adt15::C0(_, _, h639, _) => {
                    h639
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f149(v7, v1) {
                f153(v8, match v0 {
                    Adt15::C0(_, _, _, h640) => {
                        h640
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f148(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m641 = f82(v1.clone(), f8(Str::lit("std")));
    match m641 {
        None => {
            let m646 = f83(f82(v1.clone(), f8(Str::lit("req"))));
            match m646 {
                None => {
                    let m648 = f82(v1, f8(Str::lit("der")));
                    match m648 {
                        None => {
                            Ok(f196(v0))
                        }
                        Some(v5) => {
                            let m649 = f197(f80(v5.clone(), f8(Str::lit("rule"))));
                            match m649 {
                                None => {
                                    Ok(f196(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f107(f54(f111(f82(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h650) => {
                                            h650
                                        }
                                    })?, f151(f80(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h651, _) => {
                                            h651
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f151(v4, match v0 {
                        Adt12::C0(_, _, h647, _, _) => {
                            h647
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m642 = f83(f82(v2.clone(), f8(Str::lit("row"))));
            match m642 {
                None => {
                    Ok(Adt19::C0(f151(f80(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h643, _, _, _, _) => {
                            h643
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f151(f80(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h644, _, _, _, _) => {
                            h644
                        }
                    })?, Some(f151(v3, match v0 {
                        Adt12::C0(_, h645, _, _, _) => {
                            h645
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f149(p654: List<u64>, v1: List<u64>) -> bool {
    let m653 = p654;
    {
        let mut m653 = m653;
        loop {
            let step = {
                let v0 = m653;
                let m652 = v0;
                match m652.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f153(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m653 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(p659: List<u64>) -> bool {
    let m658 = p659;
    {
        let mut m658 = m658;
        loop {
            let step = {
                let v0 = m658;
                let m655 = v0;
                match m655.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a656 = v1;
                        let a657 = 7u64;
                        if nat_le(a656, a657) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f151(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m660 = f198(v0, v1.clone(), 0u64)?;
    match m660 {
        None => {
            let a661 = v1;
            Ok(length_list(a661))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f152(v0: Adt14) -> u64 {
    let m662 = f199(f82(v0, f8(Str::lit("stratum"))));
    match m662 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f153(v0: u64, p667: List<u64>) -> bool {
    let m666 = p667;
    {
        let mut m666 = m666;
        loop {
            let step = {
                let v1 = m666;
                let m663 = v1;
                match m663.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a664 = v2;
                        let a665 = v0;
                        if nat_eq(a664, a665) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m666 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f191(v0.clone(), v1) {
        let m668 = f38(v0, 34u8)?;
        match m668.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m669 = v3;
                match m669.uncons() {
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

pub fn f155(v0: Adt14) -> List<Adt14> {
    let m670 = v0;
    match m670 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r671) => {
            let v3 = (*r671).clone();
            f155(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r672, r673) => {
            let v1 = (*r672).clone();
            let v2 = (*r673).clone();
            List::cons(v1, f155(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f156(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f157(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m674 = v0.clone();
    match m674.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a675 = v2;
            let a676 = v1.clone();
            if equal(a675, a676) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f158(v0: List<(Bytes, ())>) -> List<Bytes> {
    f112(f200(Fn4::F201, List::<Bytes>::nil(), v0))
}

pub fn f159(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f202(f203(Fn5::F204, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m681 = {
        let (h677, _) = f205(Fn6::F206, {
            let a679 = {
                let a678 = v0;
                length_list(a678)
            };
            let a680 = 1u64;
            nat_add(a679, a680)?
        }, v1)?;
        h677
    };
    match m681.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f160(v0: List<Bytes>) -> List<(Bytes, ())> {
    f207(f208(Fn7::F209, List::<(Bytes, ())>::nil(), v0))
}

pub fn f161(p684: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m683 = p684;
    {
        let mut m683 = m683;
        loop {
            let step = {
                let v0 = m683;
                let m682 = v0;
                match m682.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f170(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Bytes, v1: u64) -> R<u64> {
    let m687 = {
        let a685 = v0;
        let a686 = v1;
        index_bytes(a685, a686)
    };
    match m687 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a732 = {
                let a708 = {
                    let a696 = {
                        let a690 = {
                            let a688 = v3;
                            let a689 = 128u8;
                            fixed_u8::bit_and(a688, a689)
                        };
                        let a691 = 128u8;
                        if equal(a690, a691) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a697 = {
                        let a694 = {
                            let a692 = v3;
                            let a693 = 64u8;
                            fixed_u8::bit_and(a692, a693)
                        };
                        let a695 = 64u8;
                        if equal(a694, a695) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a696, a697)?
                };
                let a709 = {
                    let a706 = {
                        let a700 = {
                            let a698 = v3;
                            let a699 = 32u8;
                            fixed_u8::bit_and(a698, a699)
                        };
                        let a701 = 32u8;
                        if equal(a700, a701) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a707 = {
                        let a704 = {
                            let a702 = v3;
                            let a703 = 16u8;
                            fixed_u8::bit_and(a702, a703)
                        };
                        let a705 = 16u8;
                        if equal(a704, a705) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a706, a707)?
                };
                nat_add(a708, a709)?
            };
            let a733 = {
                let a730 = {
                    let a718 = {
                        let a712 = {
                            let a710 = v3;
                            let a711 = 8u8;
                            fixed_u8::bit_and(a710, a711)
                        };
                        let a713 = 8u8;
                        if equal(a712, a713) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a719 = {
                        let a716 = {
                            let a714 = v3;
                            let a715 = 4u8;
                            fixed_u8::bit_and(a714, a715)
                        };
                        let a717 = 4u8;
                        if equal(a716, a717) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a718, a719)?
                };
                let a731 = {
                    let a728 = {
                        let a722 = {
                            let a720 = v3;
                            let a721 = 2u8;
                            fixed_u8::bit_and(a720, a721)
                        };
                        let a723 = 2u8;
                        if equal(a722, a723) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a729 = {
                        let a726 = {
                            let a724 = v3;
                            let a725 = 1u8;
                            fixed_u8::bit_and(a724, a725)
                        };
                        let a727 = 1u8;
                        if equal(a726, a727) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a728, a729)?
                };
                nat_add(a730, a731)?
            };
            nat_add(a732, a733)
        }
    }
}

pub fn f163(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m737 = {
        let a734 = v1;
        let a735 = v2;
        let a736 = 1u64;
        slice_bytes(a734, a735, a736)
    };
    match m737 {
        None => {
            v0
        }
        Some(v3) => {
            let a738 = v0;
            let a739 = v3;
            append_bytes(a738, a739)
        }
    }
}

pub fn f164(p742: List<Bytes>, p743: List<Bytes>) -> List<Bytes> {
    let m741 = (p742, p743);
    {
        let mut m741 = m741;
        loop {
            let step = {
                let (v0, v1) = m741;
                let m740 = v0;
                match m740.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: Bytes, p815: u64, p816: u64, p817: Bytes) -> R<Bytes> {
    let m814 = (p815, (p816, p817));
    {
        let mut m814 = m814;
        loop {
            let step = {
                let (v1, (v2, v3)) = m814;
                let m744 = v2;
                if m744 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m744 - 1;
                    let m747 = {
                        let a745 = v0.clone();
                        let a746 = v1;
                        index_bytes(a745, a746)
                    };
                    match m747 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a792 = {
                                    let a768 = {
                                        let a756 = {
                                            let a750 = {
                                                let a748 = v6;
                                                let a749 = 128u8;
                                                fixed_u8::bit_and(a748, a749)
                                            };
                                            let a751 = 128u8;
                                            if equal(a750, a751) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a757 = {
                                            let a754 = {
                                                let a752 = v6;
                                                let a753 = 64u8;
                                                fixed_u8::bit_and(a752, a753)
                                            };
                                            let a755 = 64u8;
                                            if equal(a754, a755) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a756, a757)?
                                    };
                                    let a769 = {
                                        let a766 = {
                                            let a760 = {
                                                let a758 = v6;
                                                let a759 = 32u8;
                                                fixed_u8::bit_and(a758, a759)
                                            };
                                            let a761 = 32u8;
                                            if equal(a760, a761) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a767 = {
                                            let a764 = {
                                                let a762 = v6;
                                                let a763 = 16u8;
                                                fixed_u8::bit_and(a762, a763)
                                            };
                                            let a765 = 16u8;
                                            if equal(a764, a765) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a766, a767)?
                                    };
                                    nat_add(a768, a769)?
                                };
                                let a793 = {
                                    let a790 = {
                                        let a778 = {
                                            let a772 = {
                                                let a770 = v6;
                                                let a771 = 8u8;
                                                fixed_u8::bit_and(a770, a771)
                                            };
                                            let a773 = 8u8;
                                            if equal(a772, a773) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a779 = {
                                            let a776 = {
                                                let a774 = v6;
                                                let a775 = 4u8;
                                                fixed_u8::bit_and(a774, a775)
                                            };
                                            let a777 = 4u8;
                                            if equal(a776, a777) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a778, a779)?
                                    };
                                    let a791 = {
                                        let a788 = {
                                            let a782 = {
                                                let a780 = v6;
                                                let a781 = 2u8;
                                                fixed_u8::bit_and(a780, a781)
                                            };
                                            let a783 = 2u8;
                                            if equal(a782, a783) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a789 = {
                                            let a786 = {
                                                let a784 = v6;
                                                let a785 = 1u8;
                                                fixed_u8::bit_and(a784, a785)
                                            };
                                            let a787 = 1u8;
                                            if equal(a786, a787) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a788, a789)?
                                    };
                                    nat_add(a790, a791)?
                                };
                                nat_add(a792, a793)?
                            };
                            let a794 = v7;
                            let a795 = 34u64;
                            if if nat_eq(a794, a795) {
                                true
                            } else {
                                let a796 = v7;
                                let a797 = 92u64;
                                nat_eq(a796, a797)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a798 = v1;
                                    let a799 = 1u64;
                                    nat_add(a798, a799)?
                                }, (v4, {
                                    let a802 = v3;
                                    let a803 = {
                                        let a800 = Bytes::lit(&[0x5c]);
                                        let a801 = f40(v0.clone(), v1, 1u64);
                                        append_bytes(a800, a801)
                                    };
                                    append_bytes(a802, a803)
                                })))
                            } else {
                                let a804 = v7;
                                let a805 = 10u64;
                                if nat_eq(a804, a805) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a806 = v1;
                                        let a807 = 1u64;
                                        nat_add(a806, a807)?
                                    }, (v4, {
                                        let a808 = v3;
                                        let a809 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a808, a809)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a810 = v1;
                                        let a811 = 1u64;
                                        nat_add(a810, a811)?
                                    }, (v4, {
                                        let a812 = v3;
                                        let a813 = f40(v0.clone(), v1, 1u64);
                                        append_bytes(a812, a813)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: u32) -> Bytes {
    let a846 = f210({
        let a818 = f211(v0, 28u32);
        let a819 = 15u32;
        fixed_u32::bit_and(a818, a819)
    });
    let a847 = {
        let a844 = f210({
            let a820 = f211(v0, 24u32);
            let a821 = 15u32;
            fixed_u32::bit_and(a820, a821)
        });
        let a845 = {
            let a842 = f210({
                let a822 = f211(v0, 20u32);
                let a823 = 15u32;
                fixed_u32::bit_and(a822, a823)
            });
            let a843 = {
                let a840 = f210({
                    let a824 = f211(v0, 16u32);
                    let a825 = 15u32;
                    fixed_u32::bit_and(a824, a825)
                });
                let a841 = {
                    let a838 = f210({
                        let a826 = f211(v0, 12u32);
                        let a827 = 15u32;
                        fixed_u32::bit_and(a826, a827)
                    });
                    let a839 = {
                        let a836 = f210({
                            let a828 = f211(v0, 8u32);
                            let a829 = 15u32;
                            fixed_u32::bit_and(a828, a829)
                        });
                        let a837 = {
                            let a834 = f210({
                                let a830 = f211(v0, 4u32);
                                let a831 = 15u32;
                                fixed_u32::bit_and(a830, a831)
                            });
                            let a835 = f210({
                                let a832 = v0;
                                let a833 = 15u32;
                                fixed_u32::bit_and(a832, a833)
                            });
                            append_bytes(a834, a835)
                        };
                        append_bytes(a836, a837)
                    };
                    append_bytes(a838, a839)
                };
                append_bytes(a840, a841)
            };
            append_bytes(a842, a843)
        };
        append_bytes(a844, a845)
    };
    append_bytes(a846, a847)
}

pub fn f167(v0: Fn3, p851: u64, p852: (u64, Adt21)) -> R<(u64, Adt21)> {
    let m850 = (p851, p852);
    {
        let mut m850 = m850;
        loop {
            let step = {
                let (v1, v2) = m850;
                let m848 = v1;
                if m848 == 0 {
                    Err::<(u64, (u64, Adt21)), R<(u64, Adt21)>>(Ok(v2))
                } else {
                    let v3 = m848 - 1;
                    Ok::<(u64, (u64, Adt21)), R<(u64, Adt21)>>((v3, {
                        let c849: Fn3 = v0.clone();
                        c849.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m850 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt21)) -> R<(u64, Adt21)> {
    Ok(({
        let a854 = {
            let (h853, _) = v3.clone();
            h853
        };
        let a855 = 64u64;
        nat_add(a854, a855)?
    }, f212({
        let (_, h856) = v3;
        h856
    }, f213(v0, v1, v2, {
        let (h857, _) = v3;
        h857
    })?)?))
}

pub fn f169() -> Adt21 {
    Adt21::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f170(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f62(v1, f49(f126(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f171(v0: List<Bytes>) -> List<Bytes> {
    let m858 = v0;
    match m858.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f26(v1.clone(), f8(Str::lit("authority = []"))) {
                f171(v2)
            } else {
                List::cons(v1, f171(v2))
            }
        }
    }
}

pub fn f172(v0: Bytes) -> R<u64> {
    let (h859, _) = f214(Fn8::F215(v0.clone()), {
        let a860 = v0.clone();
        length_bytes(a860)
    }, {
        let a861 = v0;
        length_bytes(a861)
    })?;
    Ok(h859)
}

pub fn f173(v0: Bytes, v1: u64) -> R<u64> {
    let (h862, _) = f214(Fn8::F216(v1, v0), v1, 0u64)?;
    Ok(h862)
}

pub fn f174(v0: Bytes) -> R<Bytes> {
    f217(v0.clone(), 0u64, {
        let a863 = v0;
        length_bytes(a863)
    }, false, false, Bytes::lit(&[]))
}

pub fn f175(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m864 = v2;
    if m864 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m864 - 1;
        let v5: u64 = f218(v0.clone(), v1)?;
        let a865 = v3;
        let a866 = 1u64;
        if nat_eq(a865, a866) {
            let m867 = f175(v0.clone(), v1, v4, 0u64)?;
            match m867 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f218(v0.clone(), {
                        let (_, h868) = v6.clone();
                        h868
                    })?;
                    let a869 = v7;
                    let a870 = 44u64;
                    if nat_eq(a869, a870) {
                        let m874 = f175(v0, {
                            let a872 = {
                                let (_, h871) = v6;
                                h871
                            };
                            let a873 = 1u64;
                            nat_add(a872, a873)?
                        }, v4, 1u64)?;
                        match m874 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h875, _) = v6;
                                    h875
                                }), std::rc::Rc::new({
                                    let (h876, _) = v8;
                                    h876
                                })), {
                                    let (_, h877) = v8;
                                    h877
                                })))
                            }
                        }
                    } else {
                        let a878 = v7;
                        let a879 = 93u64;
                        if nat_eq(a878, a879) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h880, _) = v6;
                                h880
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a882 = {
                                    let (_, h881) = v6;
                                    h881
                                };
                                let a883 = 1u64;
                                nat_add(a882, a883)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a884 = v3;
            let a885 = 2u64;
            if nat_eq(a884, a885) {
                let m888 = f219(v0.clone(), {
                    let a886 = v1;
                    let a887 = 1u64;
                    nat_add(a886, a887)?
                })?;
                match m888 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a889 = v5;
                        let a890 = 34u64;
                        if if nat_eq(a889, a890) {
                            let a892 = f218(v0.clone(), {
                                let (h891, _) = v9.clone();
                                h891
                            })?;
                            let a893 = 58u64;
                            nat_eq(a892, a893)
                        } else {
                            false
                        } {
                            let m897 = f175(v0.clone(), {
                                let a895 = {
                                    let (h894, _) = v9;
                                    h894
                                };
                                let a896 = 1u64;
                                nat_add(a895, a896)?
                            }, v4, 0u64)?;
                            match m897 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f218(v0.clone(), {
                                        let (_, h898) = v10.clone();
                                        h898
                                    })?;
                                    let a899 = v11;
                                    let a900 = 44u64;
                                    if nat_eq(a899, a900) {
                                        let m904 = f175(v0, {
                                            let a902 = {
                                                let (_, h901) = v10;
                                                h901
                                            };
                                            let a903 = 1u64;
                                            nat_add(a902, a903)?
                                        }, v4, 2u64)?;
                                        match m904 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h905) = v9;
                                                    h905
                                                }, std::rc::Rc::new({
                                                    let (h906, _) = v10;
                                                    h906
                                                }), std::rc::Rc::new({
                                                    let (h907, _) = v12;
                                                    h907
                                                })), {
                                                    let (_, h908) = v12;
                                                    h908
                                                })))
                                            }
                                        }
                                    } else {
                                        let a909 = v11;
                                        let a910 = 125u64;
                                        if nat_eq(a909, a910) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h911) = v9;
                                                h911
                                            }, std::rc::Rc::new({
                                                let (h912, _) = v10;
                                                h912
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a914 = {
                                                    let (_, h913) = v10;
                                                    h913
                                                };
                                                let a915 = 1u64;
                                                nat_add(a914, a915)?
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
                let a916 = v5;
                let a917 = 34u64;
                if nat_eq(a916, a917) {
                    let m920 = f219(v0, {
                        let a918 = v1;
                        let a919 = 1u64;
                        nat_add(a918, a919)?
                    })?;
                    match m920 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h921) = v13;
                                h921
                            }), {
                                let (h922, _) = v13;
                                h922
                            })))
                        }
                    }
                } else {
                    if f26(f40(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a923 = v1;
                            let a924 = 4u64;
                            nat_add(a923, a924)?
                        })))
                    } else {
                        if f26(f40(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a925 = v1;
                                let a926 = 5u64;
                                nat_add(a925, a926)?
                            })))
                        } else {
                            let a927 = v5;
                            let a928 = 91u64;
                            if nat_eq(a927, a928) {
                                let a931 = f218(v0.clone(), {
                                    let a929 = v1;
                                    let a930 = 1u64;
                                    nat_add(a929, a930)?
                                })?;
                                let a932 = 93u64;
                                if nat_eq(a931, a932) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a933 = v1;
                                        let a934 = 2u64;
                                        nat_add(a933, a934)?
                                    })))
                                } else {
                                    let m937 = f175(v0, {
                                        let a935 = v1;
                                        let a936 = 1u64;
                                        nat_add(a935, a936)?
                                    }, v4, 1u64)?;
                                    match m937 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h938, _) = v14;
                                                h938
                                            })), {
                                                let (_, h939) = v14;
                                                h939
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a940 = v5;
                                let a941 = 123u64;
                                if nat_eq(a940, a941) {
                                    let a944 = f218(v0.clone(), {
                                        let a942 = v1;
                                        let a943 = 1u64;
                                        nat_add(a942, a943)?
                                    })?;
                                    let a945 = 125u64;
                                    if nat_eq(a944, a945) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a946 = v1;
                                            let a947 = 2u64;
                                            nat_add(a946, a947)?
                                        })))
                                    } else {
                                        let m950 = f175(v0, {
                                            let a948 = v1;
                                            let a949 = 1u64;
                                            nat_add(a948, a949)?
                                        }, v4, 2u64)?;
                                        match m950 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h951, _) = v15;
                                                    h951
                                                })), {
                                                    let (_, h952) = v15;
                                                    h952
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m953 = f220(v0, v1)?;
                                    match m953 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h954) = v16;
                                                h954
                                            }, {
                                                let (h955, _) = v16;
                                                h955
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

pub fn f176(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h956, _) = v1.clone();
        h956
    };
    let v3: u64 = {
        let (h957, _) = {
            let (_, h958) = v1.clone();
            h958
        };
        h957
    };
    let v4: List<Bytes> = {
        let (_, h959) = {
            let (_, h960) = v1;
            h960
        };
        h959
    };
    let m961 = f194(v0.clone(), v2);
    match m961 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a962 = v5;
            let a963 = 10u8;
            if equal(a962, a963) {
                Ok(Some(({
                    let a964 = v2;
                    let a965 = 1u64;
                    nat_add(a964, a965)?
                }, ({
                    let a966 = v2;
                    let a967 = 1u64;
                    nat_add(a966, a967)?
                }, List::cons(f221(f40(v0, v3, {
                    let a968 = v2;
                    let a969 = v3;
                    nat_sub(a968, a969)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a970 = v2;
                    let a971 = 1u64;
                    nat_add(a970, a971)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f177(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m972 = v0;
    match m972 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f80(v2, v1)
        }
    }
}

pub fn f178(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f126(f192(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f179(v0: u64) -> bool {
    let a973 = v0;
    let a974 = 95u64;
    if nat_eq(a973, a974) {
        true
    } else {
        let a975 = v0;
        let a976 = 45u64;
        if nat_eq(a975, a976) {
            true
        } else {
            let a977 = v0;
            let a978 = 32u64;
            if nat_eq(a977, a978) {
                true
            } else {
                let a979 = v0;
                let a980 = 46u64;
                if nat_eq(a979, a980) {
                    true
                } else {
                    let a981 = v0;
                    let a982 = 47u64;
                    nat_eq(a981, a982)
                }
            }
        }
    }
}

pub fn f180(v0: u64) -> bool {
    let a983 = 97u64;
    let a984 = v0;
    if nat_le(a983, a984) {
        let a985 = v0;
        let a986 = 122u64;
        nat_le(a985, a986)
    } else {
        false
    }
}

pub fn f181(v0: u64) -> bool {
    let a987 = 65u64;
    let a988 = v0;
    if nat_le(a987, a988) {
        let a989 = v0;
        let a990 = 90u64;
        nat_le(a989, a990)
    } else {
        false
    }
}

pub fn f182(v0: u64) -> bool {
    let a991 = 48u64;
    let a992 = v0;
    if nat_le(a991, a992) {
        let a993 = v0;
        let a994 = 57u64;
        nat_le(a993, a994)
    } else {
        false
    }
}

pub fn f183(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m995 = f82(v1.clone(), f8(Str::lit("enum")));
    match m995 {
        None => {
            let m997 = f225(v0.clone(), f139(f82(v1.clone(), f8(Str::lit("properties")))), f54(f111(f82(v1, f8(Str::lit("required"))))))?;
            match m997 {
                Ok(v6) => {
                    Ok(f224(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m999 = {
                            let (h998, _) = v6.clone();
                            h998
                        };
                        match m999.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f126({
                                    let (h1000, _) = v6;
                                    h1000
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1001) = v6;
                        h1001
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt17>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m996 = f222(f54(f155(v2)))?;
            match m996 {
                Ok(v4) => {
                    Ok(f224(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f126(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt17>::nil()))
                }
                Err(v3) => {
                    Ok(f223(v3, v0))
                }
            }
        }
    }
}

pub fn f184(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1002 = v0;
    match m1002 {
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
        Adt14::C4(r1003) => {
            let v4 = (*r1003).clone();
            f184(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1004, r1005) => {
            let v2 = (*r1004).clone();
            let v3 = (*r1005).clone();
            List::cons((v1, v2), f184(v3))
        }
    }
}

pub fn f185(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1006 = 0u64;
        let a1007 = v1;
        nat_lt(a1006, a1007)
    } else {
        false
    } {
        let a1008 = Bytes::lit(&[0x2d]);
        let a1009 = f226(v1)?;
        Ok(append_bytes(a1008, a1009))
    } else {
        f226(v1)
    }
}

pub fn f186(v0: Bytes) -> R<Bytes> {
    let a1015 = {
        let a1013 = Bytes::lit(&[0x22]);
        let a1014 = {
            let (_, h1010) = {
                let (h1011, _) = f227(Fn9::F228(v0.clone()), {
                    let a1012 = v0;
                    length_bytes(a1012)
                }, (0u64, Bytes::lit(&[])))?;
                h1011
            };
            h1010
        };
        append_bytes(a1013, a1014)
    };
    let a1016 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1015, a1016))
}

pub fn f187(v0: List<Bytes>) -> Bytes {
    let a1019 = {
        let a1017 = Bytes::lit(&[0x5b]);
        let a1018 = f73(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1017, a1018)
    };
    let a1020 = Bytes::lit(&[0x5d]);
    append_bytes(a1019, a1020)
}

pub fn f188(v0: Adt14) -> R<List<Bytes>> {
    let m1021 = v0;
    match m1021 {
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
        Adt14::C6(r1022, r1023) => {
            let v7 = (*r1022).clone();
            let v8 = (*r1023).clone();
            Ok(List::cons(f141(v7)?, f188(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f189(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f229(Fn10::F230, List::<(Bytes, Bytes)>::nil(), f231(v0)?);
    let v2: List<Bytes> = f232(Fn11::F233, List::<Bytes>::nil(), v1)?;
    let a1026 = {
        let a1024 = Bytes::lit(&[0x7b]);
        let a1025 = f73(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1024, a1025)
    };
    let a1027 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1026, a1027))
}

pub fn f190(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1028 = v0;
    match m1028 {
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
        Adt14::C7(v9, r1029, r1030) => {
            let v10 = (*r1029).clone();
            let v11 = (*r1030).clone();
            Ok(List::cons((v9, f141(v10)?), f190(v11)?))
        }
    }
}

pub fn f191(v0: Bytes, v1: Bytes) -> bool {
    let m1035 = {
        let a1032 = v0;
        let a1033 = 0u64;
        let a1034 = {
            let a1031 = v1.clone();
            length_bytes(a1031)
        };
        slice_bytes(a1032, a1033, a1034)
    };
    match m1035 {
        None => {
            false
        }
        Some(v2) => {
            let a1036 = v2;
            let a1037 = v1;
            equal(a1036, a1037)
        }
    }
}

pub fn f192(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1038 = v0;
    match m1038.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f120(v1)?, f192(v2)?))
        }
    }
}

pub fn f193(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1039 = v0;
    match m1039.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f193(v3, false))
            } else {
                f193(v3, true)
            }
        }
    }
}

pub fn f194(v0: Bytes, v1: u64) -> Option<u8> {
    let a1040 = v0;
    let a1041 = v1;
    index_bytes(a1040, a1041)
}

pub fn f195(v0: Adt15, v1: Option<u64>) -> bool {
    let m1042 = v1;
    match m1042 {
        None => {
            true
        }
        Some(v2) => {
            f153(v2, match v0 {
                Adt15::C0(_, h1043, _, _) => {
                    h1043
                }
            })
        }
    }
}

pub fn f196(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a1045 = match v0 {
            Adt12::C0(_, _, h1044, _, _) => {
                h1044
            }
        };
        length_list(a1045)
    })
}

pub fn f197(v0: Bytes) -> Option<Adt20> {
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

pub fn f198(v0: Bytes, p1050: List<Bytes>, p1051: u64) -> R<Option<u64>> {
    let m1049 = (p1050, p1051);
    {
        let mut m1049 = m1049;
        loop {
            let step = {
                let (v1, v2) = m1049;
                let m1046 = v1;
                match m1046.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f26(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1047 = v2;
                                let a1048 = 1u64;
                                nat_add(a1047, a1048)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1049 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: Option<Adt14>) -> Option<u64> {
    let m1052 = v0;
    match m1052 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1053 = v1;
            match m1053 {
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

pub fn f200(v0: Fn4, p1057: List<Bytes>, p1058: List<(Bytes, ())>) -> List<Bytes> {
    let m1056 = (p1057, p1058);
    {
        let mut m1056 = m1056;
        loop {
            let step = {
                let (v1, v2) = m1056;
                let m1054 = v2;
                match m1054.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1055: Fn4 = v0.clone();
                            c1055.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1056 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1059, _) = v1;
        h1059
    }, v0)
}

pub fn f202(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f234(Fn12::F235, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f203(v0: Fn5, p1063: List<List<(Bytes, ())>>, p1064: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1062 = (p1063, p1064);
    {
        let mut m1062 = m1062;
        loop {
            let step = {
                let (v1, v2) = m1062;
                let m1060 = v2;
                match m1060.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1061: Fn5 = v0.clone();
                            c1061.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1062 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f205(v0: Fn6, p1069: u64, p1070: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1068 = (p1069, p1070);
    {
        let mut m1068 = m1068;
        loop {
            let step = {
                let (v1, v2) = m1068;
                let m1065 = v1;
                if m1065 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1065 - 1;
                    let m1067 = {
                        let c1066: Fn6 = v0.clone();
                        c1066.apply(v2.clone())?
                    };
                    match m1067 {
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
                Ok(next) => { m1068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f236(v0)
}

pub fn f207(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f237(Fn13::F238, List::<(Bytes, ())>::nil(), v0)
}

pub fn f208(v0: Fn7, p1074: List<(Bytes, ())>, p1075: List<Bytes>) -> List<(Bytes, ())> {
    let m1073 = (p1074, p1075);
    {
        let mut m1073 = m1073;
        loop {
            let step = {
                let (v1, v2) = m1073;
                let m1071 = v2;
                match m1071.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1072: Fn7 = v0.clone();
                            c1072.apply(v1, v3)
                        }, v4))
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

pub fn f209(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f210(v0: u32) -> Bytes {
    let a1078 = {
        let a1076 = v0;
        let a1077 = 8u32;
        fixed_u32::bit_and(a1076, a1077)
    };
    let a1079 = 0u32;
    if equal(a1078, a1079) {
        let a1082 = {
            let a1080 = v0;
            let a1081 = 4u32;
            fixed_u32::bit_and(a1080, a1081)
        };
        let a1083 = 0u32;
        if equal(a1082, a1083) {
            let a1086 = {
                let a1084 = v0;
                let a1085 = 2u32;
                fixed_u32::bit_and(a1084, a1085)
            };
            let a1087 = 0u32;
            if equal(a1086, a1087) {
                let a1090 = {
                    let a1088 = v0;
                    let a1089 = 1u32;
                    fixed_u32::bit_and(a1088, a1089)
                };
                let a1091 = 0u32;
                if equal(a1090, a1091) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1094 = {
                    let a1092 = v0;
                    let a1093 = 1u32;
                    fixed_u32::bit_and(a1092, a1093)
                };
                let a1095 = 0u32;
                if equal(a1094, a1095) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1098 = {
                let a1096 = v0;
                let a1097 = 2u32;
                fixed_u32::bit_and(a1096, a1097)
            };
            let a1099 = 0u32;
            if equal(a1098, a1099) {
                let a1102 = {
                    let a1100 = v0;
                    let a1101 = 1u32;
                    fixed_u32::bit_and(a1100, a1101)
                };
                let a1103 = 0u32;
                if equal(a1102, a1103) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1106 = {
                    let a1104 = v0;
                    let a1105 = 1u32;
                    fixed_u32::bit_and(a1104, a1105)
                };
                let a1107 = 0u32;
                if equal(a1106, a1107) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1110 = {
            let a1108 = v0;
            let a1109 = 4u32;
            fixed_u32::bit_and(a1108, a1109)
        };
        let a1111 = 0u32;
        if equal(a1110, a1111) {
            let a1114 = {
                let a1112 = v0;
                let a1113 = 2u32;
                fixed_u32::bit_and(a1112, a1113)
            };
            let a1115 = 0u32;
            if equal(a1114, a1115) {
                let a1118 = {
                    let a1116 = v0;
                    let a1117 = 1u32;
                    fixed_u32::bit_and(a1116, a1117)
                };
                let a1119 = 0u32;
                if equal(a1118, a1119) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1122 = {
                    let a1120 = v0;
                    let a1121 = 1u32;
                    fixed_u32::bit_and(a1120, a1121)
                };
                let a1123 = 0u32;
                if equal(a1122, a1123) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1126 = {
                let a1124 = v0;
                let a1125 = 2u32;
                fixed_u32::bit_and(a1124, a1125)
            };
            let a1127 = 0u32;
            if equal(a1126, a1127) {
                let a1130 = {
                    let a1128 = v0;
                    let a1129 = 1u32;
                    fixed_u32::bit_and(a1128, a1129)
                };
                let a1131 = 0u32;
                if equal(a1130, a1131) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1134 = {
                    let a1132 = v0;
                    let a1133 = 1u32;
                    fixed_u32::bit_and(a1132, a1133)
                };
                let a1135 = 0u32;
                if equal(a1134, a1135) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f211(v0: u32, v1: u32) -> u32 {
    let m1138 = {
        let a1136 = v0;
        let a1137 = v1;
        fixed_u32::shift_right(a1136, a1137)
    };
    match m1138 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f212(v0: Adt21, v1: Adt22) -> R<Adt21> {
    let v2: Adt23 = f239(Fn14::F240, 64u64, Adt23::C0(match v0.clone() {
        Adt21::C0(h1139, _, _, _, _, _, _, _) => {
            h1139
        }
    }, match v0.clone() {
        Adt21::C0(_, h1140, _, _, _, _, _, _) => {
            h1140
        }
    }, match v0.clone() {
        Adt21::C0(_, _, h1141, _, _, _, _, _) => {
            h1141
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, h1142, _, _, _, _) => {
            h1142
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, h1143, _, _, _) => {
            h1143
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1144, _, _) => {
            h1144
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1145, _) => {
            h1145
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, h1146) => {
            h1146
        }
    }, v1, 0u64))?;
    Ok(Adt21::C0(f241(match v0.clone() {
        Adt21::C0(h1147, _, _, _, _, _, _, _) => {
            h1147
        }
    }, match v2.clone() {
        Adt23::C0(h1148, _, _, _, _, _, _, _, _, _) => {
            h1148
        }
    }), f241(match v0.clone() {
        Adt21::C0(_, h1149, _, _, _, _, _, _) => {
            h1149
        }
    }, match v2.clone() {
        Adt23::C0(_, h1150, _, _, _, _, _, _, _, _) => {
            h1150
        }
    }), f241(match v0.clone() {
        Adt21::C0(_, _, h1151, _, _, _, _, _) => {
            h1151
        }
    }, match v2.clone() {
        Adt23::C0(_, _, h1152, _, _, _, _, _, _, _) => {
            h1152
        }
    }), f241(match v0.clone() {
        Adt21::C0(_, _, _, h1153, _, _, _, _) => {
            h1153
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, h1154, _, _, _, _, _, _) => {
            h1154
        }
    }), f241(match v0.clone() {
        Adt21::C0(_, _, _, _, h1155, _, _, _) => {
            h1155
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, h1156, _, _, _, _, _) => {
            h1156
        }
    }), f241(match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1157, _, _) => {
            h1157
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, h1158, _, _, _, _) => {
            h1158
        }
    }), f241(match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1159, _) => {
            h1159
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, _, h1160, _, _, _) => {
            h1160
        }
    }), f241(match v0 {
        Adt21::C0(_, _, _, _, _, _, _, h1161) => {
            h1161
        }
    }, match v2 {
        Adt23::C0(_, _, _, _, _, _, _, h1162, _, _) => {
            h1162
        }
    })))
}

pub fn f213(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    Ok(Adt22::C0(f242(v0.clone(), v1, v2, v3)?, f242(v0.clone(), v1, v2, {
        let a1163 = v3;
        let a1164 = 4u64;
        nat_add(a1163, a1164)?
    })?, f242(v0.clone(), v1, v2, {
        let a1165 = v3;
        let a1166 = 8u64;
        nat_add(a1165, a1166)?
    })?, f242(v0.clone(), v1, v2, {
        let a1167 = v3;
        let a1168 = 12u64;
        nat_add(a1167, a1168)?
    })?, f242(v0.clone(), v1, v2, {
        let a1169 = v3;
        let a1170 = 16u64;
        nat_add(a1169, a1170)?
    })?, f242(v0.clone(), v1, v2, {
        let a1171 = v3;
        let a1172 = 20u64;
        nat_add(a1171, a1172)?
    })?, f242(v0.clone(), v1, v2, {
        let a1173 = v3;
        let a1174 = 24u64;
        nat_add(a1173, a1174)?
    })?, f242(v0.clone(), v1, v2, {
        let a1175 = v3;
        let a1176 = 28u64;
        nat_add(a1175, a1176)?
    })?, f242(v0.clone(), v1, v2, {
        let a1177 = v3;
        let a1178 = 32u64;
        nat_add(a1177, a1178)?
    })?, f242(v0.clone(), v1, v2, {
        let a1179 = v3;
        let a1180 = 36u64;
        nat_add(a1179, a1180)?
    })?, f242(v0.clone(), v1, v2, {
        let a1181 = v3;
        let a1182 = 40u64;
        nat_add(a1181, a1182)?
    })?, f242(v0.clone(), v1, v2, {
        let a1183 = v3;
        let a1184 = 44u64;
        nat_add(a1183, a1184)?
    })?, f242(v0.clone(), v1, v2, {
        let a1185 = v3;
        let a1186 = 48u64;
        nat_add(a1185, a1186)?
    })?, f242(v0.clone(), v1, v2, {
        let a1187 = v3;
        let a1188 = 52u64;
        nat_add(a1187, a1188)?
    })?, f242(v0.clone(), v1, v2, {
        let a1189 = v3;
        let a1190 = 56u64;
        nat_add(a1189, a1190)?
    })?, f242(v0, v1, v2, {
        let a1191 = v3;
        let a1192 = 60u64;
        nat_add(a1191, a1192)?
    })?))
}

pub fn f214(v0: Fn8, p1197: u64, p1198: u64) -> R<(u64, bool)> {
    let m1196 = (p1197, p1198);
    {
        let mut m1196 = m1196;
        loop {
            let step = {
                let (v1, v2) = m1196;
                let m1193 = v1;
                if m1193 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1193 - 1;
                    let m1195 = {
                        let c1194: Fn8 = v0.clone();
                        c1194.apply(v2)?
                    };
                    match m1195 {
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
                Ok(next) => { m1196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: Bytes, v1: u64) -> Option<u64> {
    let a1199 = 0u64;
    let a1200 = v1;
    if nat_lt(a1199, a1200) {
        if f243(v0, {
            let a1201 = v1;
            let a1202 = 1u64;
            nat_sub(a1201, a1202)
        }) {
            Some({
                let a1203 = v1;
                let a1204 = 1u64;
                nat_sub(a1203, a1204)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f216(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1205 = v2;
    let a1206 = v0;
    if nat_lt(a1205, a1206) {
        if f243(v1, v2) {
            Ok(Some({
                let a1207 = v2;
                let a1208 = 1u64;
                nat_add(a1207, a1208)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f217(v0: Bytes, p1288: u64, p1289: u64, p1290: bool, p1291: bool, p1292: Bytes) -> R<Bytes> {
    let m1287 = (p1288, (p1289, (p1290, (p1291, p1292))));
    {
        let mut m1287 = m1287;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1287;
                let m1209 = v2;
                if m1209 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1209 - 1;
                    let m1212 = {
                        let a1210 = v0.clone();
                        let a1211 = v1;
                        index_bytes(a1210, a1211)
                    };
                    match m1212 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1257 = {
                                    let a1233 = {
                                        let a1221 = {
                                            let a1215 = {
                                                let a1213 = v8;
                                                let a1214 = 128u8;
                                                fixed_u8::bit_and(a1213, a1214)
                                            };
                                            let a1216 = 128u8;
                                            if equal(a1215, a1216) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1222 = {
                                            let a1219 = {
                                                let a1217 = v8;
                                                let a1218 = 64u8;
                                                fixed_u8::bit_and(a1217, a1218)
                                            };
                                            let a1220 = 64u8;
                                            if equal(a1219, a1220) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1221, a1222)?
                                    };
                                    let a1234 = {
                                        let a1231 = {
                                            let a1225 = {
                                                let a1223 = v8;
                                                let a1224 = 32u8;
                                                fixed_u8::bit_and(a1223, a1224)
                                            };
                                            let a1226 = 32u8;
                                            if equal(a1225, a1226) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1232 = {
                                            let a1229 = {
                                                let a1227 = v8;
                                                let a1228 = 16u8;
                                                fixed_u8::bit_and(a1227, a1228)
                                            };
                                            let a1230 = 16u8;
                                            if equal(a1229, a1230) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1231, a1232)?
                                    };
                                    nat_add(a1233, a1234)?
                                };
                                let a1258 = {
                                    let a1255 = {
                                        let a1243 = {
                                            let a1237 = {
                                                let a1235 = v8;
                                                let a1236 = 8u8;
                                                fixed_u8::bit_and(a1235, a1236)
                                            };
                                            let a1238 = 8u8;
                                            if equal(a1237, a1238) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1244 = {
                                            let a1241 = {
                                                let a1239 = v8;
                                                let a1240 = 4u8;
                                                fixed_u8::bit_and(a1239, a1240)
                                            };
                                            let a1242 = 4u8;
                                            if equal(a1241, a1242) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1243, a1244)?
                                    };
                                    let a1256 = {
                                        let a1253 = {
                                            let a1247 = {
                                                let a1245 = v8;
                                                let a1246 = 2u8;
                                                fixed_u8::bit_and(a1245, a1246)
                                            };
                                            let a1248 = 2u8;
                                            if equal(a1247, a1248) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1254 = {
                                            let a1251 = {
                                                let a1249 = v8;
                                                let a1250 = 1u8;
                                                fixed_u8::bit_and(a1249, a1250)
                                            };
                                            let a1252 = 1u8;
                                            if equal(a1251, a1252) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1253, a1254)?
                                    };
                                    nat_add(a1255, a1256)?
                                };
                                nat_add(a1257, a1258)?
                            };
                            let v10: Bytes = f40(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1259 = v1;
                                    let a1260 = 1u64;
                                    nat_add(a1259, a1260)?
                                }, (v6, (if v4 {
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
                                }, {
                                    let a1265 = v5;
                                    let a1266 = v10;
                                    append_bytes(a1265, a1266)
                                })))))
                            } else {
                                let a1267 = v9;
                                let a1268 = 32u64;
                                if if if if nat_eq(a1267, a1268) {
                                    true
                                } else {
                                    let a1269 = v9;
                                    let a1270 = 9u64;
                                    nat_eq(a1269, a1270)
                                } {
                                    true
                                } else {
                                    let a1271 = v9;
                                    let a1272 = 10u64;
                                    nat_eq(a1271, a1272)
                                } {
                                    true
                                } else {
                                    let a1273 = v9;
                                    let a1274 = 13u64;
                                    nat_eq(a1273, a1274)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1275 = v1;
                                        let a1276 = 1u64;
                                        nat_add(a1275, a1276)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f26(f40(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1277 = v1;
                                            let a1278 = 4u64;
                                            nat_add(a1277, a1278)?
                                        }, (v6, (false, (false, {
                                            let a1279 = v5;
                                            let a1280 = f8(Str::lit("[]"));
                                            append_bytes(a1279, a1280)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1281 = v1;
                                            let a1282 = 1u64;
                                            nat_add(a1281, a1282)?
                                        }, (v6, ({
                                            let a1283 = v9;
                                            let a1284 = 34u64;
                                            nat_eq(a1283, a1284)
                                        }, (false, {
                                            let a1285 = v5;
                                            let a1286 = v10;
                                            append_bytes(a1285, a1286)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1287 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f218(v0: Bytes, v1: u64) -> R<u64> {
    let m1295 = {
        let a1293 = v0;
        let a1294 = v1;
        index_bytes(a1293, a1294)
    };
    match m1295 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1340 = {
                let a1316 = {
                    let a1304 = {
                        let a1298 = {
                            let a1296 = v3;
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
                            let a1300 = v3;
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
                            let a1306 = v3;
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
                            let a1310 = v3;
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
                            let a1318 = v3;
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
                            let a1322 = v3;
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
                            let a1328 = v3;
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
                            let a1332 = v3;
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
            nat_add(a1340, a1341)
        }
    }
}

pub fn f219(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f244(Fn15::F245(v0.clone()), {
        let a1343 = {
            let a1342 = v0;
            length_bytes(a1342)
        };
        let a1344 = 1u64;
        nat_add(a1343, a1344)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1348 = {
        let (_, h1345) = {
            let (_, h1346) = {
                let (h1347, _) = v2.clone();
                h1347
            };
            h1346
        };
        h1345
    };
    let a1349 = 1u64;
    if nat_eq(a1348, a1349) {
        Ok(Some(({
            let (h1350, _) = {
                let (h1351, _) = v2.clone();
                h1351
            };
            h1350
        }, {
            let (h1352, _) = {
                let (_, h1353) = {
                    let (h1354, _) = v2;
                    h1354
                };
                h1353
            };
            h1352
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f220(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1355 = f218(v0.clone(), v1)?;
        let a1356 = 45u64;
        nat_eq(a1355, a1356)
    };
    let v3: u64 = if v2 {
        let a1357 = v1;
        let a1358 = 1u64;
        nat_add(a1357, a1358)?
    } else {
        v1
    };
    let v4: u64 = f246(v0.clone(), v3)?;
    let a1361 = 19u64;
    let a1362 = {
        let a1359 = v4;
        let a1360 = v3;
        nat_sub(a1359, a1360)
    };
    if nat_lt(a1361, a1362) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1365 = f247(f40(v0, v3, {
            let a1363 = v4;
            let a1364 = v3;
            nat_sub(a1363, a1364)
        }))?;
        match m1365 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f248(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f221(v0: Bytes) -> Bytes {
    if f99(v0.clone(), Bytes::lit(&[0x0d])) {
        f40(v0.clone(), 0u64, {
            let a1367 = {
                let a1366 = v0;
                length_bytes(a1366)
            };
            let a1368 = 1u64;
            nat_sub(a1367, a1368)
        })
    } else {
        v0
    }
}

pub fn f222(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1369 = v0;
    match m1369.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f249(v1.clone())?;
            if f250(v3.clone())? {
                let m1370 = f222(v2)?;
                match m1370 {
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

pub fn f223(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt17>), Bytes> {
    Err::<(Bytes, List<Adt17>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f224(v0: Bytes, v1: List<Adt17>) -> Result<(Bytes, List<Adt17>), Bytes> {
    Ok::<(Bytes, List<Adt17>), Bytes>((v0, v1))
}

pub fn f225(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt17>), Bytes>> {
    let m1371 = v1;
    match m1371.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt17>), Bytes>((List::<Bytes>::nil(), List::<Adt17>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f249({
                let (h1372, _) = v3.clone();
                h1372
            })?;
            if f250(v5.clone())? {
                let m1377 = f251({
                    let (_, h1373) = v3.clone();
                    h1373
                }, {
                    let a1375 = v0.clone();
                    let a1376 = f88({
                        let (h1374, _) = v3.clone();
                        h1374
                    })?;
                    append_bytes(a1375, a1376)
                })?;
                match m1377 {
                    Ok(v7) => {
                        let v8: Bytes = if f62({
                            let (h1378, _) = v3;
                            h1378
                        }, v2.clone()) {
                            let (h1379, _) = v7;
                            h1379
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1380, _) = v7;
                                h1380
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1381 = f225(v0, v4, v2)?;
                        match m1381 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt17>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1382, _) = v10;
                                    h1382
                                }), f137({
                                    let (_, h1383) = v7;
                                    h1383
                                }, {
                                    let (_, h1384) = v10;
                                    h1384
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
                    let (h1385, _) = v3;
                    h1385
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f226(v0: u64) -> R<Bytes> {
    f252(f253(), 10u64, v0)
}

pub fn f227(v0: Fn9, p1390: u64, p1391: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1389 = (p1390, p1391);
    {
        let mut m1389 = m1389;
        loop {
            let step = {
                let (v1, v2) = m1389;
                let m1386 = v1;
                if m1386 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1386 - 1;
                    let m1388 = {
                        let c1387: Fn9 = v0.clone();
                        c1387.apply(v2.clone())?
                    };
                    match m1388 {
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
                Ok(next) => { m1389 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f228(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f254(v0, v1)
}

pub fn f229(v0: Fn10, p1395: List<(Bytes, Bytes)>, p1396: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1394 = (p1395, p1396);
    {
        let mut m1394 = m1394;
        loop {
            let step = {
                let (v1, v2) = m1394;
                let m1392 = v2;
                match m1392.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1393: Fn10 = v0.clone();
                            c1393.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f255(v0, v1)
}

pub fn f231(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f256(v0)
}

pub fn f232(v0: Fn11, p1400: List<Bytes>, p1401: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1399 = (p1400, p1401);
    {
        let mut m1399 = m1399;
        loop {
            let step = {
                let (v1, v2) = m1399;
                let m1397 = v2;
                match m1397.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1398: Fn11 = v0.clone();
                            c1398.apply(v1, v3)?
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

pub fn f233(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f257(v1)?, v0))
}

pub fn f234(v0: Fn12, p1405: List<List<(Bytes, ())>>, p1406: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1404 = (p1405, p1406);
    {
        let mut m1404 = m1404;
        loop {
            let step = {
                let (v1, v2) = m1404;
                let m1402 = v2;
                match m1402.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1403: Fn12 = v0.clone();
                            c1403.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f236(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1408 = {
        let a1407 = v0.clone();
        length_list(a1407)
    };
    let a1409 = 1u64;
    if nat_le(a1408, a1409) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f258(v0)?))
    }
}

pub fn f237(v0: Fn13, p1413: List<(Bytes, ())>, p1414: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1412 = (p1413, p1414);
    {
        let mut m1412 = m1412;
        loop {
            let step = {
                let (v1, v2) = m1412;
                let m1410 = v2;
                match m1410.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1411: Fn13 = v0.clone();
                            c1411.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1412 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f239(v0: Fn14, p1418: u64, p1419: Adt23) -> R<Adt23> {
    let m1417 = (p1418, p1419);
    {
        let mut m1417 = m1417;
        loop {
            let step = {
                let (v1, v2) = m1417;
                let m1415 = v1;
                if m1415 == 0 {
                    Err::<(u64, Adt23), R<Adt23>>(Ok(v2))
                } else {
                    let v3 = m1415 - 1;
                    Ok::<(u64, Adt23), R<Adt23>>((v3, {
                        let c1416: Fn14 = v0.clone();
                        c1416.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: Adt23) -> R<Adt23> {
    f259(v0)
}

pub fn f241(v0: u32, v1: u32) -> u32 {
    let m1422 = {
        let a1420 = f260(v0);
        let a1421 = f260(v1);
        fixed_u64::checked_add(a1420, a1421)
    };
    match m1422 {
        None => {
            0u32
        }
        Some(v2) => {
            f261(v2)
        }
    }
}

pub fn f242(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1425 = {
        let a1423 = v3;
        let a1424 = 8u64;
        nat_add(a1423, a1424)?
    };
    let a1426 = v2;
    if nat_eq(a1425, a1426) {
        Ok(f262({
            let a1429 = {
                let a1427 = v1;
                let a1428 = 8u64;
                nat_mul(a1427, a1428)?
            };
            let a1430 = 4294967296u64;
            let a1431 = 0u64;
            nat_quot(a1429, a1430, a1431)
        }))
    } else {
        let a1434 = {
            let a1432 = v3;
            let a1433 = 4u64;
            nat_add(a1432, a1433)?
        };
        let a1435 = v2;
        if nat_eq(a1434, a1435) {
            Ok(f262({
                let a1438 = {
                    let a1436 = v1;
                    let a1437 = 8u64;
                    nat_mul(a1436, a1437)?
                };
                let a1439 = 4294967296u64;
                let a1440 = 0u64;
                nat_rem(a1438, a1439, a1440)
            }))
        } else {
            Ok(f263(f264(v0.clone(), v1, v3), f264(v0.clone(), v1, {
                let a1441 = v3;
                let a1442 = 1u64;
                nat_add(a1441, a1442)?
            }), f264(v0.clone(), v1, {
                let a1443 = v3;
                let a1444 = 2u64;
                nat_add(a1443, a1444)?
            }), f264(v0, v1, {
                let a1445 = v3;
                let a1446 = 3u64;
                nat_add(a1445, a1446)?
            })))
        }
    }
}

pub fn f243(v0: Bytes, v1: u64) -> bool {
    let m1447 = f194(v0, v1);
    match m1447 {
        None => {
            false
        }
        Some(v2) => {
            f265(v2)
        }
    }
}

pub fn f244(v0: Fn15, p1452: u64, p1453: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1451 = (p1452, p1453);
    {
        let mut m1451 = m1451;
        loop {
            let step = {
                let (v1, v2) = m1451;
                let m1448 = v1;
                if m1448 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1448 - 1;
                    let m1450 = {
                        let c1449: Fn15 = v0.clone();
                        c1449.apply(v2.clone())?
                    };
                    match m1450 {
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
                Ok(next) => { m1451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f266(v0, v1)
}

pub fn f246(v0: Bytes, v1: u64) -> R<u64> {
    let (h1454, _) = f214(Fn8::F267(v0.clone()), {
        let a1456 = {
            let a1455 = v0;
            length_bytes(a1455)
        };
        let a1457 = 1u64;
        nat_add(a1456, a1457)?
    }, v1)?;
    Ok(h1454)
}

pub fn f247(v0: Bytes) -> R<Option<u64>> {
    if f268(v0.clone())? {
        Ok(Some({
            let (_, h1458) = {
                let (h1459, _) = f269(Fn16::F270(v0.clone()), {
                    let a1460 = v0;
                    length_bytes(a1460)
                }, (0u64, 0u64))?;
                h1459
            };
            h1458
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f248(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1461 = 0u64;
        let a1462 = v1;
        if nat_lt(a1461, a1462) {
            let a1463 = v1;
            let a1464 = 9223372036854775808u64;
            nat_le(a1463, a1464)
        } else {
            false
        }
    } else {
        let a1465 = v1;
        let a1466 = 9223372036854775807u64;
        nat_le(a1465, a1466)
    }
}

pub fn f249(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f135(v0.clone(), 0u64, {
        let a1467 = v0;
        length_bytes(a1467)
    }, false, Bytes::lit(&[]))?;
    let m1470 = {
        let a1468 = v1.clone();
        let a1469 = 0u64;
        index_bytes(a1468, a1469)
    };
    match m1470 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1515 = {
                    let a1491 = {
                        let a1479 = {
                            let a1473 = {
                                let a1471 = v3;
                                let a1472 = 128u8;
                                fixed_u8::bit_and(a1471, a1472)
                            };
                            let a1474 = 128u8;
                            if equal(a1473, a1474) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1480 = {
                            let a1477 = {
                                let a1475 = v3;
                                let a1476 = 64u8;
                                fixed_u8::bit_and(a1475, a1476)
                            };
                            let a1478 = 64u8;
                            if equal(a1477, a1478) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1479, a1480)?
                    };
                    let a1492 = {
                        let a1489 = {
                            let a1483 = {
                                let a1481 = v3;
                                let a1482 = 32u8;
                                fixed_u8::bit_and(a1481, a1482)
                            };
                            let a1484 = 32u8;
                            if equal(a1483, a1484) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1490 = {
                            let a1487 = {
                                let a1485 = v3;
                                let a1486 = 16u8;
                                fixed_u8::bit_and(a1485, a1486)
                            };
                            let a1488 = 16u8;
                            if equal(a1487, a1488) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1489, a1490)?
                    };
                    nat_add(a1491, a1492)?
                };
                let a1516 = {
                    let a1513 = {
                        let a1501 = {
                            let a1495 = {
                                let a1493 = v3;
                                let a1494 = 8u8;
                                fixed_u8::bit_and(a1493, a1494)
                            };
                            let a1496 = 8u8;
                            if equal(a1495, a1496) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1502 = {
                            let a1499 = {
                                let a1497 = v3;
                                let a1498 = 4u8;
                                fixed_u8::bit_and(a1497, a1498)
                            };
                            let a1500 = 4u8;
                            if equal(a1499, a1500) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1501, a1502)?
                    };
                    let a1514 = {
                        let a1511 = {
                            let a1505 = {
                                let a1503 = v3;
                                let a1504 = 2u8;
                                fixed_u8::bit_and(a1503, a1504)
                            };
                            let a1506 = 2u8;
                            if equal(a1505, a1506) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1512 = {
                            let a1509 = {
                                let a1507 = v3;
                                let a1508 = 1u8;
                                fixed_u8::bit_and(a1507, a1508)
                            };
                            let a1510 = 1u8;
                            if equal(a1509, a1510) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1511, a1512)?
                    };
                    nat_add(a1513, a1514)?
                };
                nat_add(a1515, a1516)?
            };
            if f181(v4) {
                let a1523 = f40({
                    let a1517 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a1517)
                }, {
                    let a1518 = v4;
                    let a1519 = 65u64;
                    nat_sub(a1518, a1519)
                }, 1u64);
                let a1524 = f40(v1.clone(), 1u64, {
                    let a1521 = {
                        let a1520 = v1;
                        length_bytes(a1520)
                    };
                    let a1522 = 1u64;
                    nat_sub(a1521, a1522)
                });
                Ok(append_bytes(a1523, a1524))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f250(v0: Bytes) -> R<bool> {
    let m1527 = {
        let a1525 = v0;
        let a1526 = 0u64;
        index_bytes(a1525, a1526)
    };
    match m1527 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f180({
                let v2: u8 = v1;
                let a1572 = {
                    let a1548 = {
                        let a1536 = {
                            let a1530 = {
                                let a1528 = v2;
                                let a1529 = 128u8;
                                fixed_u8::bit_and(a1528, a1529)
                            };
                            let a1531 = 128u8;
                            if equal(a1530, a1531) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1537 = {
                            let a1534 = {
                                let a1532 = v2;
                                let a1533 = 64u8;
                                fixed_u8::bit_and(a1532, a1533)
                            };
                            let a1535 = 64u8;
                            if equal(a1534, a1535) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1536, a1537)?
                    };
                    let a1549 = {
                        let a1546 = {
                            let a1540 = {
                                let a1538 = v2;
                                let a1539 = 32u8;
                                fixed_u8::bit_and(a1538, a1539)
                            };
                            let a1541 = 32u8;
                            if equal(a1540, a1541) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1547 = {
                            let a1544 = {
                                let a1542 = v2;
                                let a1543 = 16u8;
                                fixed_u8::bit_and(a1542, a1543)
                            };
                            let a1545 = 16u8;
                            if equal(a1544, a1545) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1546, a1547)?
                    };
                    nat_add(a1548, a1549)?
                };
                let a1573 = {
                    let a1570 = {
                        let a1558 = {
                            let a1552 = {
                                let a1550 = v2;
                                let a1551 = 8u8;
                                fixed_u8::bit_and(a1550, a1551)
                            };
                            let a1553 = 8u8;
                            if equal(a1552, a1553) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1559 = {
                            let a1556 = {
                                let a1554 = v2;
                                let a1555 = 4u8;
                                fixed_u8::bit_and(a1554, a1555)
                            };
                            let a1557 = 4u8;
                            if equal(a1556, a1557) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1558, a1559)?
                    };
                    let a1571 = {
                        let a1568 = {
                            let a1562 = {
                                let a1560 = v2;
                                let a1561 = 2u8;
                                fixed_u8::bit_and(a1560, a1561)
                            };
                            let a1563 = 2u8;
                            if equal(a1562, a1563) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1569 = {
                            let a1566 = {
                                let a1564 = v2;
                                let a1565 = 1u8;
                                fixed_u8::bit_and(a1564, a1565)
                            };
                            let a1567 = 1u8;
                            if equal(a1566, a1567) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1568, a1569)?
                    };
                    nat_add(a1570, a1571)?
                };
                nat_add(a1572, a1573)?
            }) {
                Ok(true)
            } else {
                Ok(f181({
                    let v3: u8 = v1;
                    let a1618 = {
                        let a1594 = {
                            let a1582 = {
                                let a1576 = {
                                    let a1574 = v3;
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
                                    let a1578 = v3;
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
                                    let a1584 = v3;
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
                                    let a1588 = v3;
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
                                    let a1596 = v3;
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
                                    let a1600 = v3;
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
                                    let a1606 = v3;
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
                                    let a1610 = v3;
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
                }))
            }
        }
    }
}

pub fn f251(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1620 = f271(v0.clone());
    match m1620 {
        None => {
            if f26(f80(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1621 = f82(v0, f8(Str::lit("items")));
                match m1621 {
                    None => {
                        Ok(f223(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f272(f8(Str::lit("List")), f273(v3, {
                            let a1622 = v1;
                            let a1623 = f8(Str::lit("Item"));
                            append_bytes(a1622, a1623)
                        })?))
                    }
                }
            } else {
                f273(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f272(f8(Str::lit("Option")), f273(v2, v1)?))
        }
    }
}

pub fn f252(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1624 = v2;
    let a1625 = 0u64;
    if nat_eq(a1624, a1625) {
        Ok(f274(v0, 0u64))
    } else {
        let (_, h1626) = {
            let (h1627, _) = f227(Fn9::F275(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1627
        };
        Ok(h1626)
    }
}

pub fn f253() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f254(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1631 = {
        let a1629 = v0.clone();
        let a1630 = {
            let (h1628, _) = v1.clone();
            h1628
        };
        index_bytes(a1629, a1630)
    };
    match m1631 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1633 = {
                    let (h1632, _) = v1.clone();
                    h1632
                };
                let a1634 = 1u64;
                nat_add(a1633, a1634)?
            }, {
                let a1637 = {
                    let (_, h1635) = v1;
                    h1635
                };
                let a1638 = f276(v0, {
                    let (h1636, _) = v1;
                    h1636
                }, v2)?;
                append_bytes(a1637, a1638)
            })))
        }
    }
}

pub fn f255(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1639 = v0.clone();
    match m1639.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1642 = {
                let (h1640, _) = v2;
                h1640
            };
            let a1643 = {
                let (h1641, _) = v1.clone();
                h1641
            };
            if equal(a1642, a1643) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f256(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f277(f278(Fn17::F279, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1648 = {
        let (h1644, _) = f280(Fn18::F281, {
            let a1646 = {
                let a1645 = v0;
                length_list(a1645)
            };
            let a1647 = 1u64;
            nat_add(a1646, a1647)?
        }, v1)?;
        h1644
    };
    match m1648.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f257(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1653 = {
        let a1650 = f186({
            let (h1649, _) = v0;
            h1649
        })?;
        let a1651 = Bytes::lit(&[0x3a]);
        append_bytes(a1650, a1651)
    };
    let a1654 = {
        let (_, h1652) = v0;
        h1652
    };
    Ok(append_bytes(a1653, a1654))
}

pub fn f258(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f202({
        let (_, h1655) = {
            let (h1656, _) = f282(Fn19::F283, {
                let a1658 = {
                    let a1657 = v0.clone();
                    length_list(a1657)
                };
                let a1659 = 1u64;
                nat_add(a1658, a1659)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1656
        };
        h1655
    }))
}

pub fn f259(v0: Adt23) -> R<Adt23> {
    let v1: u32 = f284(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, _, h1660) => {
            h1660
        }
    });
    let v2: u32 = f241(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h1661, _, _) => {
            h1661
        }
    }, f241(f285(match v0.clone() {
        Adt23::C0(_, _, _, _, h1662, _, _, _, _, _) => {
            h1662
        }
    }), f241(f286(match v0.clone() {
        Adt23::C0(_, _, _, _, h1663, _, _, _, _, _) => {
            h1663
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1664, _, _, _, _) => {
            h1664
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1665, _, _, _) => {
            h1665
        }
    }), f241(v1, {
        let m1668 = match v0.clone() {
            Adt23::C0(_, _, _, _, _, _, _, _, h1667, _) => {
                h1667
            }
        };
        match m1668 {
            Adt22::C0(h1666, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1666
            }
        }
    }))));
    let v3: u32 = f241(f287(match v0.clone() {
        Adt23::C0(h1669, _, _, _, _, _, _, _, _, _) => {
            h1669
        }
    }), f288(match v0.clone() {
        Adt23::C0(h1670, _, _, _, _, _, _, _, _, _) => {
            h1670
        }
    }, match v0.clone() {
        Adt23::C0(_, h1671, _, _, _, _, _, _, _, _) => {
            h1671
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1672, _, _, _, _, _, _, _) => {
            h1672
        }
    }));
    Ok(Adt23::C0(f241(v2, v3), match v0.clone() {
        Adt23::C0(h1673, _, _, _, _, _, _, _, _, _) => {
            h1673
        }
    }, match v0.clone() {
        Adt23::C0(_, h1674, _, _, _, _, _, _, _, _) => {
            h1674
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1675, _, _, _, _, _, _, _) => {
            h1675
        }
    }, f241(match v0.clone() {
        Adt23::C0(_, _, _, h1676, _, _, _, _, _, _) => {
            h1676
        }
    }, v2), match v0.clone() {
        Adt23::C0(_, _, _, _, h1677, _, _, _, _, _) => {
            h1677
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1678, _, _, _, _) => {
            h1678
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1679, _, _, _) => {
            h1679
        }
    }, f289(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, h1680, _) => {
            h1680
        }
    }), {
        let a1682 = match v0 {
            Adt23::C0(_, _, _, _, _, _, _, _, _, h1681) => {
                h1681
            }
        };
        let a1683 = 1u64;
        nat_add(a1682, a1683)?
    }))
}

pub fn f260(v0: u32) -> u64 {
    let m1685 = {
        let a1684 = v0;
        fixed_u64::convert(i128::from(a1684))
    };
    match m1685 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f261(v0: u64) -> u32 {
    let m1689 = {
        let a1688 = {
            let a1686 = v0;
            let a1687 = 4294967295u64;
            fixed_u64::bit_and(a1686, a1687)
        };
        fixed_u32::convert(i128::from(a1688))
    };
    match m1689 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f262(v0: u64) -> u32 {
    let (_, h1690) = {
        let (_, h1691) = f290(Fn20::F291, 32u64, (v0, (1u32, 0u32)));
        h1691
    };
    h1690
}

pub fn f263(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1696 = f292(v0, 24u32);
    let a1697 = {
        let a1694 = f292(v1, 16u32);
        let a1695 = {
            let a1692 = f292(v2, 8u32);
            let a1693 = v3;
            fixed_u32::bit_or(a1692, a1693)
        };
        fixed_u32::bit_or(a1694, a1695)
    };
    fixed_u32::bit_or(a1696, a1697)
}

pub fn f264(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1698 = v2;
    let a1699 = v1;
    if nat_lt(a1698, a1699) {
        let m1702 = {
            let a1700 = v0;
            let a1701 = v2;
            index_bytes(a1700, a1701)
        };
        match m1702 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1704 = {
                    let a1703 = v3;
                    fixed_u32::convert(i128::from(a1703))
                };
                match m1704 {
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
        let a1705 = v2;
        let a1706 = v1;
        if nat_eq(a1705, a1706) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f265(v0: u8) -> bool {
    let a1707 = v0;
    let a1708 = 32u8;
    if if if equal(a1707, a1708) {
        true
    } else {
        let a1709 = v0;
        let a1710 = 9u8;
        equal(a1709, a1710)
    } {
        true
    } else {
        let a1711 = v0;
        let a1712 = 10u8;
        equal(a1711, a1712)
    } {
        true
    } else {
        let a1713 = v0;
        let a1714 = 12u8;
        if equal(a1713, a1714) {
            true
        } else {
            let a1715 = v0;
            let a1716 = 13u8;
            equal(a1715, a1716)
        }
    }
}

pub fn f266(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1717, _) = v1.clone();
        h1717
    };
    let v3: Bytes = {
        let (h1718, _) = {
            let (_, h1719) = v1.clone();
            h1719
        };
        h1718
    };
    let a1722 = {
        let (_, h1720) = {
            let (_, h1721) = v1;
            h1721
        };
        h1720
    };
    let a1723 = 0u64;
    if nat_eq(a1722, a1723) {
        let v4: u64 = f218(v0.clone(), v2)?;
        let a1724 = v4;
        let a1725 = 34u64;
        if nat_eq(a1724, a1725) {
            Ok(Some(({
                let a1726 = v2;
                let a1727 = 1u64;
                nat_add(a1726, a1727)?
            }, (v3, 1u64))))
        } else {
            let a1728 = v4;
            let a1729 = 92u64;
            if nat_eq(a1728, a1729) {
                f293(v0, v2, v3)
            } else {
                let a1730 = v4;
                let a1731 = 32u64;
                if if nat_lt(a1730, a1731) {
                    true
                } else {
                    let a1732 = v4;
                    let a1733 = 256u64;
                    nat_eq(a1732, a1733)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1734 = v2;
                        let a1735 = 1u64;
                        nat_add(a1734, a1735)?
                    }, ({
                        let a1736 = v3;
                        let a1737 = f40(v0, v2, 1u64);
                        append_bytes(a1736, a1737)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f267(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f294(v0, v1)
}

pub fn f268(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1738 = v0.clone();
        length_bytes(a1738)
    };
    let a1739 = 0u64;
    let a1740 = v1;
    if nat_lt(a1739, a1740) {
        let a1741 = f295(v0.clone())?;
        let a1742 = v1;
        if nat_eq(a1741, a1742) {
            let a1743 = 1u64;
            let a1744 = v1;
            if nat_lt(a1743, a1744) {
                let m1747 = {
                    let a1745 = v0;
                    let a1746 = 0u64;
                    index_bytes(a1745, a1746)
                };
                match m1747 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1748 = v2;
                        let a1749 = 48u8;
                        Ok(!equal(a1748, a1749))
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

pub fn f269(v0: Fn16, p1754: u64, p1755: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1753 = (p1754, p1755);
    {
        let mut m1753 = m1753;
        loop {
            let step = {
                let (v1, v2) = m1753;
                let m1750 = v1;
                if m1750 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1750 - 1;
                    let m1752 = {
                        let c1751: Fn16 = v0.clone();
                        c1751.apply(v2)?
                    };
                    match m1752 {
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
                Ok(next) => { m1753 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f296(v0, v1)
}

pub fn f271(v0: Adt14) -> Option<Adt14> {
    let m1756 = f111(f82(v0, f8(Str::lit("oneOf"))));
    match m1756.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m1757 = v2;
            match m1757.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m1758 = v4;
                    match m1758.uncons() {
                        None => {
                            if f297(v3.clone()) {
                                Some(v1)
                            } else {
                                if f297(v1) {
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

pub fn f272(v0: Bytes, v1: Result<(Bytes, List<Adt17>), Bytes>) -> Result<(Bytes, List<Adt17>), Bytes> {
    let m1759 = v1;
    match m1759 {
        Ok(v3) => {
            f224(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h1760, _) = v3;
                h1760
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h1761) = v3;
                h1761
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt17>), Bytes>(v2)
        }
    }
}

pub fn f273(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1762 = f271(v0.clone());
    match m1762 {
        None => {
            if f26(f80(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1763 = f82(v0, f8(Str::lit("items")));
                match m1763 {
                    None => {
                        Ok(f223(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f272(f8(Str::lit("List")), f298(v3, {
                            let a1764 = v1;
                            let a1765 = f8(Str::lit("Item"));
                            append_bytes(a1764, a1765)
                        })?))
                    }
                }
            } else {
                f298(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f272(f8(Str::lit("Option")), f298(v2, v1)?))
        }
    }
}

pub fn f274(v0: Bytes, v1: u64) -> Bytes {
    let m1769 = {
        let a1766 = v0;
        let a1767 = v1;
        let a1768 = 1u64;
        slice_bytes(a1766, a1767, a1768)
    };
    match m1769 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f275(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f299(v1, v0, v2)
}

pub fn f276(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a1814 = {
            let a1790 = {
                let a1778 = {
                    let a1772 = {
                        let a1770 = v3;
                        let a1771 = 128u8;
                        fixed_u8::bit_and(a1770, a1771)
                    };
                    let a1773 = 128u8;
                    if equal(a1772, a1773) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1779 = {
                    let a1776 = {
                        let a1774 = v3;
                        let a1775 = 64u8;
                        fixed_u8::bit_and(a1774, a1775)
                    };
                    let a1777 = 64u8;
                    if equal(a1776, a1777) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1778, a1779)?
            };
            let a1791 = {
                let a1788 = {
                    let a1782 = {
                        let a1780 = v3;
                        let a1781 = 32u8;
                        fixed_u8::bit_and(a1780, a1781)
                    };
                    let a1783 = 32u8;
                    if equal(a1782, a1783) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1789 = {
                    let a1786 = {
                        let a1784 = v3;
                        let a1785 = 16u8;
                        fixed_u8::bit_and(a1784, a1785)
                    };
                    let a1787 = 16u8;
                    if equal(a1786, a1787) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1788, a1789)?
            };
            nat_add(a1790, a1791)?
        };
        let a1815 = {
            let a1812 = {
                let a1800 = {
                    let a1794 = {
                        let a1792 = v3;
                        let a1793 = 8u8;
                        fixed_u8::bit_and(a1792, a1793)
                    };
                    let a1795 = 8u8;
                    if equal(a1794, a1795) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1801 = {
                    let a1798 = {
                        let a1796 = v3;
                        let a1797 = 4u8;
                        fixed_u8::bit_and(a1796, a1797)
                    };
                    let a1799 = 4u8;
                    if equal(a1798, a1799) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1800, a1801)?
            };
            let a1813 = {
                let a1810 = {
                    let a1804 = {
                        let a1802 = v3;
                        let a1803 = 2u8;
                        fixed_u8::bit_and(a1802, a1803)
                    };
                    let a1805 = 2u8;
                    if equal(a1804, a1805) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1811 = {
                    let a1808 = {
                        let a1806 = v3;
                        let a1807 = 1u8;
                        fixed_u8::bit_and(a1806, a1807)
                    };
                    let a1809 = 1u8;
                    if equal(a1808, a1809) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1810, a1811)?
            };
            nat_add(a1812, a1813)?
        };
        nat_add(a1814, a1815)?
    };
    let a1816 = v4;
    let a1817 = 34u64;
    if nat_eq(a1816, a1817) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a1818 = v4;
        let a1819 = 92u64;
        if nat_eq(a1818, a1819) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a1820 = v4;
            let a1821 = 8u64;
            if nat_eq(a1820, a1821) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a1822 = v4;
                let a1823 = 12u64;
                if nat_eq(a1822, a1823) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a1824 = v4;
                    let a1825 = 10u64;
                    if nat_eq(a1824, a1825) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a1826 = v4;
                        let a1827 = 13u64;
                        if nat_eq(a1826, a1827) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a1828 = v4;
                            let a1829 = 9u64;
                            if nat_eq(a1828, a1829) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a1830 = v4;
                                let a1831 = 32u64;
                                if nat_lt(a1830, a1831) {
                                    let a1832 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a1833 = f300(v2)?;
                                    Ok(append_bytes(a1832, a1833))
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

pub fn f277(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f301(Fn21::F302, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f278(v0: Fn17, p1837: List<List<(Bytes, Bytes)>>, p1838: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m1836 = (p1837, p1838);
    {
        let mut m1836 = m1836;
        loop {
            let step = {
                let (v1, v2) = m1836;
                let m1834 = v2;
                match m1834.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c1835: Fn17 = v0.clone();
                            c1835.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1836 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f280(v0: Fn18, p1843: u64, p1844: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m1842 = (p1843, p1844);
    {
        let mut m1842 = m1842;
        loop {
            let step = {
                let (v1, v2) = m1842;
                let m1839 = v1;
                if m1839 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1839 - 1;
                    let m1841 = {
                        let c1840: Fn18 = v0.clone();
                        c1840.apply(v2.clone())?
                    };
                    match m1841 {
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
                Ok(next) => { m1842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f303(v0)
}

pub fn f282(v0: Fn19, p1849: u64, p1850: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m1848 = (p1849, p1850);
    {
        let mut m1848 = m1848;
        loop {
            let step = {
                let (v1, v2) = m1848;
                let m1845 = v1;
                if m1845 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1845 - 1;
                    let m1847 = {
                        let c1846: Fn19 = v0.clone();
                        c1846.apply(v2.clone())?
                    };
                    match m1847 {
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
                Ok(next) => { m1848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f304(v0)
}

pub fn f284(v0: u64) -> u32 {
    let a1851 = v0;
    let a1852 = 32u64;
    if nat_lt(a1851, a1852) {
        let a1853 = v0;
        let a1854 = 16u64;
        if nat_lt(a1853, a1854) {
            let a1855 = v0;
            let a1856 = 8u64;
            if nat_lt(a1855, a1856) {
                let a1857 = v0;
                let a1858 = 4u64;
                if nat_lt(a1857, a1858) {
                    let a1859 = v0;
                    let a1860 = 2u64;
                    if nat_lt(a1859, a1860) {
                        let a1861 = v0;
                        let a1862 = 1u64;
                        if nat_lt(a1861, a1862) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1863 = v0;
                        let a1864 = 3u64;
                        if nat_lt(a1863, a1864) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1865 = v0;
                    let a1866 = 6u64;
                    if nat_lt(a1865, a1866) {
                        let a1867 = v0;
                        let a1868 = 5u64;
                        if nat_lt(a1867, a1868) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1869 = v0;
                        let a1870 = 7u64;
                        if nat_lt(a1869, a1870) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1871 = v0;
                let a1872 = 12u64;
                if nat_lt(a1871, a1872) {
                    let a1873 = v0;
                    let a1874 = 10u64;
                    if nat_lt(a1873, a1874) {
                        let a1875 = v0;
                        let a1876 = 9u64;
                        if nat_lt(a1875, a1876) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1877 = v0;
                        let a1878 = 11u64;
                        if nat_lt(a1877, a1878) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1879 = v0;
                    let a1880 = 14u64;
                    if nat_lt(a1879, a1880) {
                        let a1881 = v0;
                        let a1882 = 13u64;
                        if nat_lt(a1881, a1882) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1883 = v0;
                        let a1884 = 15u64;
                        if nat_lt(a1883, a1884) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1885 = v0;
            let a1886 = 24u64;
            if nat_lt(a1885, a1886) {
                let a1887 = v0;
                let a1888 = 20u64;
                if nat_lt(a1887, a1888) {
                    let a1889 = v0;
                    let a1890 = 18u64;
                    if nat_lt(a1889, a1890) {
                        let a1891 = v0;
                        let a1892 = 17u64;
                        if nat_lt(a1891, a1892) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1893 = v0;
                        let a1894 = 19u64;
                        if nat_lt(a1893, a1894) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1895 = v0;
                    let a1896 = 22u64;
                    if nat_lt(a1895, a1896) {
                        let a1897 = v0;
                        let a1898 = 21u64;
                        if nat_lt(a1897, a1898) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1899 = v0;
                        let a1900 = 23u64;
                        if nat_lt(a1899, a1900) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1901 = v0;
                let a1902 = 28u64;
                if nat_lt(a1901, a1902) {
                    let a1903 = v0;
                    let a1904 = 26u64;
                    if nat_lt(a1903, a1904) {
                        let a1905 = v0;
                        let a1906 = 25u64;
                        if nat_lt(a1905, a1906) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1907 = v0;
                        let a1908 = 27u64;
                        if nat_lt(a1907, a1908) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1909 = v0;
                    let a1910 = 30u64;
                    if nat_lt(a1909, a1910) {
                        let a1911 = v0;
                        let a1912 = 29u64;
                        if nat_lt(a1911, a1912) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1913 = v0;
                        let a1914 = 31u64;
                        if nat_lt(a1913, a1914) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1915 = v0;
        let a1916 = 48u64;
        if nat_lt(a1915, a1916) {
            let a1917 = v0;
            let a1918 = 40u64;
            if nat_lt(a1917, a1918) {
                let a1919 = v0;
                let a1920 = 36u64;
                if nat_lt(a1919, a1920) {
                    let a1921 = v0;
                    let a1922 = 34u64;
                    if nat_lt(a1921, a1922) {
                        let a1923 = v0;
                        let a1924 = 33u64;
                        if nat_lt(a1923, a1924) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1925 = v0;
                        let a1926 = 35u64;
                        if nat_lt(a1925, a1926) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1927 = v0;
                    let a1928 = 38u64;
                    if nat_lt(a1927, a1928) {
                        let a1929 = v0;
                        let a1930 = 37u64;
                        if nat_lt(a1929, a1930) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1931 = v0;
                        let a1932 = 39u64;
                        if nat_lt(a1931, a1932) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1933 = v0;
                let a1934 = 44u64;
                if nat_lt(a1933, a1934) {
                    let a1935 = v0;
                    let a1936 = 42u64;
                    if nat_lt(a1935, a1936) {
                        let a1937 = v0;
                        let a1938 = 41u64;
                        if nat_lt(a1937, a1938) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1939 = v0;
                        let a1940 = 43u64;
                        if nat_lt(a1939, a1940) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1941 = v0;
                    let a1942 = 46u64;
                    if nat_lt(a1941, a1942) {
                        let a1943 = v0;
                        let a1944 = 45u64;
                        if nat_lt(a1943, a1944) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1945 = v0;
                        let a1946 = 47u64;
                        if nat_lt(a1945, a1946) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1947 = v0;
            let a1948 = 56u64;
            if nat_lt(a1947, a1948) {
                let a1949 = v0;
                let a1950 = 52u64;
                if nat_lt(a1949, a1950) {
                    let a1951 = v0;
                    let a1952 = 50u64;
                    if nat_lt(a1951, a1952) {
                        let a1953 = v0;
                        let a1954 = 49u64;
                        if nat_lt(a1953, a1954) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1955 = v0;
                        let a1956 = 51u64;
                        if nat_lt(a1955, a1956) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1957 = v0;
                    let a1958 = 54u64;
                    if nat_lt(a1957, a1958) {
                        let a1959 = v0;
                        let a1960 = 53u64;
                        if nat_lt(a1959, a1960) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1961 = v0;
                        let a1962 = 55u64;
                        if nat_lt(a1961, a1962) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1963 = v0;
                let a1964 = 60u64;
                if nat_lt(a1963, a1964) {
                    let a1965 = v0;
                    let a1966 = 58u64;
                    if nat_lt(a1965, a1966) {
                        let a1967 = v0;
                        let a1968 = 57u64;
                        if nat_lt(a1967, a1968) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1969 = v0;
                        let a1970 = 59u64;
                        if nat_lt(a1969, a1970) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1971 = v0;
                    let a1972 = 62u64;
                    if nat_lt(a1971, a1972) {
                        let a1973 = v0;
                        let a1974 = 61u64;
                        if nat_lt(a1973, a1974) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1975 = v0;
                        let a1976 = 63u64;
                        if nat_lt(a1975, a1976) {
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

pub fn f285(v0: u32) -> u32 {
    let a1979 = {
        let a1977 = f305(v0, 6u32, 26u32);
        let a1978 = f305(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1977, a1978)
    };
    let a1980 = f305(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1979, a1980)
}

pub fn f286(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1986 = {
        let a1981 = v0;
        let a1982 = v1;
        fixed_u32::bit_and(a1981, a1982)
    };
    let a1987 = {
        let a1984 = {
            let a1983 = v0;
            fixed_u32::bit_not(a1983)
        };
        let a1985 = v2;
        fixed_u32::bit_and(a1984, a1985)
    };
    fixed_u32::bit_xor(a1986, a1987)
}

pub fn f287(v0: u32) -> u32 {
    let a1990 = {
        let a1988 = f305(v0, 2u32, 30u32);
        let a1989 = f305(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1988, a1989)
    };
    let a1991 = f305(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1990, a1991)
}

pub fn f288(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2000 = {
        let a1996 = {
            let a1992 = v0;
            let a1993 = v1;
            fixed_u32::bit_and(a1992, a1993)
        };
        let a1997 = {
            let a1994 = v0;
            let a1995 = v2;
            fixed_u32::bit_and(a1994, a1995)
        };
        fixed_u32::bit_xor(a1996, a1997)
    };
    let a2001 = {
        let a1998 = v1;
        let a1999 = v2;
        fixed_u32::bit_and(a1998, a1999)
    };
    fixed_u32::bit_xor(a2000, a2001)
}

pub fn f289(v0: Adt22) -> Adt22 {
    Adt22::C0(match v0.clone() {
        Adt22::C0(_, h2002, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2002
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2003, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2003
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h2004, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2004
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h2005, _, _, _, _, _, _, _, _, _, _, _) => {
            h2005
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2006, _, _, _, _, _, _, _, _, _, _) => {
            h2006
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2007, _, _, _, _, _, _, _, _, _) => {
            h2007
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2008, _, _, _, _, _, _, _, _) => {
            h2008
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2009, _, _, _, _, _, _, _) => {
            h2009
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2010, _, _, _, _, _, _) => {
            h2010
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, h2011, _, _, _, _, _) => {
            h2011
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, h2012, _, _, _, _) => {
            h2012
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2013, _, _, _) => {
            h2013
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2014, _, _) => {
            h2014
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2015, _) => {
            h2015
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2016) => {
            h2016
        }
    }, f241(f306(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2017, _) => {
            h2017
        }
    }), f241(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2018, _, _, _, _, _, _) => {
            h2018
        }
    }, f241(f307(match v0.clone() {
        Adt22::C0(_, h2019, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2019
        }
    }), match v0 {
        Adt22::C0(h2020, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2020
        }
    }))))
}

pub fn f290(v0: Fn20, p2024: u64, p2025: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2023 = (p2024, p2025);
    {
        let mut m2023 = m2023;
        loop {
            let step = {
                let (v1, v2) = m2023;
                let m2021 = v1;
                if m2021 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2021 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2022: Fn20 = v0.clone();
                        c2022.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2023 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2026, _) = v0;
        h2026
    };
    let v2: u32 = {
        let (h2027, _) = {
            let (_, h2028) = v0;
            h2028
        };
        h2027
    };
    let v3: u32 = {
        let (_, h2029) = {
            let (_, h2030) = v0;
            h2030
        };
        h2029
    };
    ({
        let a2031 = v1;
        let a2032 = 2u64;
        let a2033 = 0u64;
        nat_quot(a2031, a2032, a2033)
    }, (f292(v2, 1u32), {
        let a2037 = {
            let a2034 = v1;
            let a2035 = 2u64;
            let a2036 = 0u64;
            nat_rem(a2034, a2035, a2036)
        };
        let a2038 = 1u64;
        if nat_eq(a2037, a2038) {
            let a2039 = v3;
            let a2040 = v2;
            fixed_u32::bit_or(a2039, a2040)
        } else {
            v3
        }
    }))
}

pub fn f292(v0: u32, v1: u32) -> u32 {
    let m2043 = {
        let a2041 = v0;
        let a2042 = v1;
        fixed_u32::shift_left(a2041, a2042)
    };
    match m2043 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f293(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f218(v0.clone(), {
        let a2044 = v1;
        let a2045 = 1u64;
        nat_add(a2044, a2045)?
    })?;
    let v4: Bytes = {
        let a2046 = v3;
        let a2047 = 117u64;
        if nat_eq(a2046, a2047) {
            f308(v0, v1)?
        } else {
            f309(v3)
        }
    };
    let a2048 = v4.clone();
    let a2049 = Bytes::lit(&[]);
    if equal(a2048, a2049) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2050 = v3;
            let a2051 = 117u64;
            if nat_eq(a2050, a2051) {
                let a2052 = v1;
                let a2053 = 6u64;
                nat_add(a2052, a2053)?
            } else {
                let a2054 = v1;
                let a2055 = 2u64;
                nat_add(a2054, a2055)?
            }
        }, ({
            let a2056 = v2;
            let a2057 = v4;
            append_bytes(a2056, a2057)
        }, 0u64))))
    }
}

pub fn f294(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f218(v0, v1)?;
    let a2058 = 47u64;
    let a2059 = v2;
    if if nat_lt(a2058, a2059) {
        let a2060 = v2;
        let a2061 = 58u64;
        nat_lt(a2060, a2061)
    } else {
        false
    } {
        Ok(Some({
            let a2062 = v1;
            let a2063 = 1u64;
            nat_add(a2062, a2063)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f295(v0: Bytes) -> R<u64> {
    let (h2064, _) = f214(Fn8::F310(v0.clone()), {
        let a2065 = v0;
        length_bytes(a2065)
    }, 0u64)?;
    Ok(h2064)
}

pub fn f296(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2069 = {
        let a2067 = v0;
        let a2068 = {
            let (h2066, _) = v1;
            h2066
        };
        index_bytes(a2067, a2068)
    };
    match m2069 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2071 = {
                    let (h2070, _) = v1;
                    h2070
                };
                let a2072 = 1u64;
                nat_add(a2071, a2072)?
            }, {
                let a2076 = {
                    let a2074 = {
                        let (_, h2073) = v1;
                        h2073
                    };
                    let a2075 = 10u64;
                    nat_mul(a2074, a2075)?
                };
                let a2077 = f311(v2)?;
                nat_add(a2076, a2077)?
            })))
        }
    }
}

pub fn f297(v0: Adt14) -> bool {
    f26(f80(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f298(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m2078 = f312(f184(v0.clone()));
    match m2078 {
        None => {
            let m2079 = f83(f82(v0.clone(), f8(Str::lit("$ref"))));
            match m2079 {
                None => {
                    let m2081 = f82(v0.clone(), f8(Str::lit("enum")));
                    match m2081 {
                        None => {
                            let m2082 = f82(v0.clone(), f8(Str::lit("const")));
                            match m2082 {
                                None => {
                                    let v8: Bytes = f80(v0.clone(), f8(Str::lit("type")));
                                    if f26(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f224(f8(Str::lit("Bytes")), List::<Adt17>::nil()))
                                    } else {
                                        if f26(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f224(f314(v0), List::<Adt17>::nil()))
                                        } else {
                                            if f26(v8.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f224(f8(Str::lit("Bool")), List::<Adt17>::nil()))
                                            } else {
                                                if f26(v8.clone(), f8(Str::lit("object"))) {
                                                    Ok(f224(v1.clone(), List::cons(Adt17::C0(v1, v0), List::<Adt17>::nil())))
                                                } else {
                                                    Ok(f223(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2083 = f83(Some(v6));
                                    match m2083 {
                                        None => {
                                            Ok(f223(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f224(f8(Str::lit("Bytes")), List::<Adt17>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f224(v1.clone(), List::cons(Adt17::C0(v1, v0), List::<Adt17>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2080 = f313(v3)?;
                    match m2080 {
                        None => {
                            Ok(f223(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f224(v4, List::<Adt17>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f223(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f299(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2084, _) = v2;
        h2084
    };
    let a2085 = v3;
    let a2086 = 0u64;
    if nat_eq(a2085, a2086) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2087 = v3;
            let a2088 = v1;
            let a2089 = 0u64;
            nat_quot(a2087, a2088, a2089)
        }, {
            let a2094 = f274(v0, {
                let a2090 = v3;
                let a2091 = v1;
                let a2092 = 0u64;
                nat_rem(a2090, a2091, a2092)
            });
            let a2095 = {
                let (_, h2093) = v2;
                h2093
            };
            append_bytes(a2094, a2095)
        }))
    }
}

pub fn f300(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2140 = {
            let a2116 = {
                let a2104 = {
                    let a2098 = {
                        let a2096 = v1;
                        let a2097 = 128u8;
                        fixed_u8::bit_and(a2096, a2097)
                    };
                    let a2099 = 128u8;
                    if equal(a2098, a2099) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2105 = {
                    let a2102 = {
                        let a2100 = v1;
                        let a2101 = 64u8;
                        fixed_u8::bit_and(a2100, a2101)
                    };
                    let a2103 = 64u8;
                    if equal(a2102, a2103) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2104, a2105)?
            };
            let a2117 = {
                let a2114 = {
                    let a2108 = {
                        let a2106 = v1;
                        let a2107 = 32u8;
                        fixed_u8::bit_and(a2106, a2107)
                    };
                    let a2109 = 32u8;
                    if equal(a2108, a2109) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2115 = {
                    let a2112 = {
                        let a2110 = v1;
                        let a2111 = 16u8;
                        fixed_u8::bit_and(a2110, a2111)
                    };
                    let a2113 = 16u8;
                    if equal(a2112, a2113) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2114, a2115)?
            };
            nat_add(a2116, a2117)?
        };
        let a2141 = {
            let a2138 = {
                let a2126 = {
                    let a2120 = {
                        let a2118 = v1;
                        let a2119 = 8u8;
                        fixed_u8::bit_and(a2118, a2119)
                    };
                    let a2121 = 8u8;
                    if equal(a2120, a2121) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2127 = {
                    let a2124 = {
                        let a2122 = v1;
                        let a2123 = 4u8;
                        fixed_u8::bit_and(a2122, a2123)
                    };
                    let a2125 = 4u8;
                    if equal(a2124, a2125) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2126, a2127)?
            };
            let a2139 = {
                let a2136 = {
                    let a2130 = {
                        let a2128 = v1;
                        let a2129 = 2u8;
                        fixed_u8::bit_and(a2128, a2129)
                    };
                    let a2131 = 2u8;
                    if equal(a2130, a2131) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2137 = {
                    let a2134 = {
                        let a2132 = v1;
                        let a2133 = 1u8;
                        fixed_u8::bit_and(a2132, a2133)
                    };
                    let a2135 = 1u8;
                    if equal(a2134, a2135) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2136, a2137)?
            };
            nat_add(a2138, a2139)?
        };
        nat_add(a2140, a2141)?
    };
    let a2148 = f274(f315(), {
        let a2142 = v2;
        let a2143 = 16u64;
        let a2144 = 0u64;
        nat_quot(a2142, a2143, a2144)
    });
    let a2149 = f274(f315(), {
        let a2145 = v2;
        let a2146 = 16u64;
        let a2147 = 0u64;
        nat_rem(a2145, a2146, a2147)
    });
    Ok(append_bytes(a2148, a2149))
}

pub fn f301(v0: Fn21, p2153: List<List<(Bytes, Bytes)>>, p2154: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2152 = (p2153, p2154);
    {
        let mut m2152 = m2152;
        loop {
            let step = {
                let (v1, v2) = m2152;
                let m2150 = v2;
                match m2150.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2151: Fn21 = v0.clone();
                            c2151.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f303(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2156 = {
        let a2155 = v0.clone();
        length_list(a2155)
    };
    let a2157 = 1u64;
    if nat_le(a2156, a2157) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f316(v0)?))
    }
}

pub fn f304(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2158, _) = v0;
        h2158
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2159) = v0;
        h2159
    };
    let m2160 = v1;
    match m2160.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2161 = v4;
            match m2161.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f317(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f305(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2162 = f211(v0, v1);
    let a2163 = f292(v0, v2);
    fixed_u32::bit_or(a2162, a2163)
}

pub fn f306(v0: u32) -> u32 {
    let a2166 = {
        let a2164 = f305(v0, 17u32, 15u32);
        let a2165 = f305(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2164, a2165)
    };
    let a2167 = f211(v0, 10u32);
    fixed_u32::bit_xor(a2166, a2167)
}

pub fn f307(v0: u32) -> u32 {
    let a2170 = {
        let a2168 = f305(v0, 7u32, 25u32);
        let a2169 = f305(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2168, a2169)
    };
    let a2171 = f211(v0, 3u32);
    fixed_u32::bit_xor(a2170, a2171)
}

pub fn f308(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2174 = f40(v0.clone(), {
        let a2172 = v1;
        let a2173 = 2u64;
        nat_add(a2172, a2173)?
    }, 2u64);
    let a2175 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2174, a2175) {
        let m2178 = f318(f40(v0, {
            let a2176 = v1;
            let a2177 = 4u64;
            nat_add(a2176, a2177)?
        }, 2u64))?;
        match m2178 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f218(v2.clone(), 0u64)?;
                let a2179 = v3;
                let a2180 = 32u64;
                if nat_lt(a2179, a2180) {
                    if f319(v3) {
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

pub fn f309(v0: u64) -> Bytes {
    let a2181 = v0;
    let a2182 = 34u64;
    if nat_eq(a2181, a2182) {
        Bytes::lit(&[0x22])
    } else {
        let a2183 = v0;
        let a2184 = 92u64;
        if nat_eq(a2183, a2184) {
            Bytes::lit(&[0x5c])
        } else {
            let a2185 = v0;
            let a2186 = 98u64;
            if nat_eq(a2185, a2186) {
                Bytes::lit(&[0x08])
            } else {
                let a2187 = v0;
                let a2188 = 102u64;
                if nat_eq(a2187, a2188) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2189 = v0;
                    let a2190 = 110u64;
                    if nat_eq(a2189, a2190) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2191 = v0;
                        let a2192 = 114u64;
                        if nat_eq(a2191, a2192) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2193 = v0;
                            let a2194 = 116u64;
                            if nat_eq(a2193, a2194) {
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

pub fn f310(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2197 = {
        let a2195 = v0;
        let a2196 = v1;
        index_bytes(a2195, a2196)
    };
    match m2197 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f320(v2)? {
                Ok(Some({
                    let a2198 = v1;
                    let a2199 = 1u64;
                    nat_add(a2198, a2199)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f311(v0: u8) -> R<u64> {
    let a2246 = {
        let v1: u8 = v0;
        let a2244 = {
            let a2220 = {
                let a2208 = {
                    let a2202 = {
                        let a2200 = v1;
                        let a2201 = 128u8;
                        fixed_u8::bit_and(a2200, a2201)
                    };
                    let a2203 = 128u8;
                    if equal(a2202, a2203) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2209 = {
                    let a2206 = {
                        let a2204 = v1;
                        let a2205 = 64u8;
                        fixed_u8::bit_and(a2204, a2205)
                    };
                    let a2207 = 64u8;
                    if equal(a2206, a2207) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2208, a2209)?
            };
            let a2221 = {
                let a2218 = {
                    let a2212 = {
                        let a2210 = v1;
                        let a2211 = 32u8;
                        fixed_u8::bit_and(a2210, a2211)
                    };
                    let a2213 = 32u8;
                    if equal(a2212, a2213) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2219 = {
                    let a2216 = {
                        let a2214 = v1;
                        let a2215 = 16u8;
                        fixed_u8::bit_and(a2214, a2215)
                    };
                    let a2217 = 16u8;
                    if equal(a2216, a2217) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2218, a2219)?
            };
            nat_add(a2220, a2221)?
        };
        let a2245 = {
            let a2242 = {
                let a2230 = {
                    let a2224 = {
                        let a2222 = v1;
                        let a2223 = 8u8;
                        fixed_u8::bit_and(a2222, a2223)
                    };
                    let a2225 = 8u8;
                    if equal(a2224, a2225) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2231 = {
                    let a2228 = {
                        let a2226 = v1;
                        let a2227 = 4u8;
                        fixed_u8::bit_and(a2226, a2227)
                    };
                    let a2229 = 4u8;
                    if equal(a2228, a2229) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2230, a2231)?
            };
            let a2243 = {
                let a2240 = {
                    let a2234 = {
                        let a2232 = v1;
                        let a2233 = 2u8;
                        fixed_u8::bit_and(a2232, a2233)
                    };
                    let a2235 = 2u8;
                    if equal(a2234, a2235) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2241 = {
                    let a2238 = {
                        let a2236 = v1;
                        let a2237 = 1u8;
                        fixed_u8::bit_and(a2236, a2237)
                    };
                    let a2239 = 1u8;
                    if equal(a2238, a2239) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2240, a2241)?
            };
            nat_add(a2242, a2243)?
        };
        nat_add(a2244, a2245)?
    };
    let a2247 = 48u64;
    Ok(nat_sub(a2246, a2247))
}

pub fn f312(p2253: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2252 = p2253;
    {
        let mut m2252 = m2252;
        loop {
            let step = {
                let v0 = m2252;
                let m2248 = v0;
                match m2248.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f62({
                            let (h2249, _) = v1.clone();
                            h2249
                        }, f321()) {
                            true
                        } else {
                            f62({
                                let (h2250, _) = v1.clone();
                                h2250
                            }, f322())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2251, _) = v1;
                                h2251
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: Bytes) -> R<Option<Bytes>> {
    if f191(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f88(f40(v0.clone(), 8u64, {
            let a2255 = {
                let a2254 = v0;
                length_bytes(a2254)
            };
            let a2256 = 8u64;
            nat_sub(a2255, a2256)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f314(v0: Adt14) -> Bytes {
    let m2257 = f199(f82(v0, f8(Str::lit("minimum"))));
    match m2257 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f315() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f316(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f277({
        let (_, h2258) = {
            let (h2259, _) = f323(Fn22::F324, {
                let a2261 = {
                    let a2260 = v0.clone();
                    length_list(a2260)
                };
                let a2262 = 1u64;
                nat_add(a2261, a2262)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2259
        };
        h2258
    }))
}

pub fn f317(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2267 = {
            let a2265 = {
                let a2263 = v0.clone();
                length_list(a2263)
            };
            let a2266 = {
                let a2264 = v1.clone();
                length_list(a2264)
            };
            nat_add(a2265, a2266)?
        };
        let a2268 = 1u64;
        nat_add(a2267, a2268)?
    };
    Ok(f207({
        let (_, h2269) = {
            let (_, h2270) = {
                let (h2271, _) = f325(Fn23::F326, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2271
            };
            h2270
        };
        h2269
    }))
}

pub fn f318(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2272 = v0.clone();
        length_bytes(a2272)
    };
    let a2276 = {
        let a2273 = v1;
        let a2274 = 2u64;
        let a2275 = 0u64;
        nat_rem(a2273, a2274, a2275)
    };
    let a2277 = 0u64;
    if nat_eq(a2276, a2277) {
        let (_, h2278) = {
            let (h2279, _) = f327(Fn24::F328(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2279
        };
        Ok(h2278)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f319(v0: u64) -> bool {
    let a2280 = v0;
    let a2281 = 8u64;
    if nat_eq(a2280, a2281) {
        true
    } else {
        let a2282 = v0;
        let a2283 = 9u64;
        if nat_eq(a2282, a2283) {
            true
        } else {
            let a2284 = v0;
            let a2285 = 10u64;
            if nat_eq(a2284, a2285) {
                true
            } else {
                let a2286 = v0;
                let a2287 = 12u64;
                if nat_eq(a2286, a2287) {
                    true
                } else {
                    let a2288 = v0;
                    let a2289 = 13u64;
                    nat_eq(a2288, a2289)
                }
            }
        }
    }
}

pub fn f320(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2334 = {
            let a2310 = {
                let a2298 = {
                    let a2292 = {
                        let a2290 = v1;
                        let a2291 = 128u8;
                        fixed_u8::bit_and(a2290, a2291)
                    };
                    let a2293 = 128u8;
                    if equal(a2292, a2293) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2299 = {
                    let a2296 = {
                        let a2294 = v1;
                        let a2295 = 64u8;
                        fixed_u8::bit_and(a2294, a2295)
                    };
                    let a2297 = 64u8;
                    if equal(a2296, a2297) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2298, a2299)?
            };
            let a2311 = {
                let a2308 = {
                    let a2302 = {
                        let a2300 = v1;
                        let a2301 = 32u8;
                        fixed_u8::bit_and(a2300, a2301)
                    };
                    let a2303 = 32u8;
                    if equal(a2302, a2303) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2309 = {
                    let a2306 = {
                        let a2304 = v1;
                        let a2305 = 16u8;
                        fixed_u8::bit_and(a2304, a2305)
                    };
                    let a2307 = 16u8;
                    if equal(a2306, a2307) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2308, a2309)?
            };
            nat_add(a2310, a2311)?
        };
        let a2335 = {
            let a2332 = {
                let a2320 = {
                    let a2314 = {
                        let a2312 = v1;
                        let a2313 = 8u8;
                        fixed_u8::bit_and(a2312, a2313)
                    };
                    let a2315 = 8u8;
                    if equal(a2314, a2315) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2321 = {
                    let a2318 = {
                        let a2316 = v1;
                        let a2317 = 4u8;
                        fixed_u8::bit_and(a2316, a2317)
                    };
                    let a2319 = 4u8;
                    if equal(a2318, a2319) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2320, a2321)?
            };
            let a2333 = {
                let a2330 = {
                    let a2324 = {
                        let a2322 = v1;
                        let a2323 = 2u8;
                        fixed_u8::bit_and(a2322, a2323)
                    };
                    let a2325 = 2u8;
                    if equal(a2324, a2325) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2331 = {
                    let a2328 = {
                        let a2326 = v1;
                        let a2327 = 1u8;
                        fixed_u8::bit_and(a2326, a2327)
                    };
                    let a2329 = 1u8;
                    if equal(a2328, a2329) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2330, a2331)?
            };
            nat_add(a2332, a2333)?
        };
        nat_add(a2334, a2335)?
    };
    let a2336 = 48u64;
    let a2337 = v2;
    if nat_le(a2336, a2337) {
        let a2338 = v2;
        let a2339 = 57u64;
        Ok(nat_le(a2338, a2339))
    } else {
        Ok(false)
    }
}

pub fn f321() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f322() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f323(v0: Fn22, p2344: u64, p2345: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2343 = (p2344, p2345);
    {
        let mut m2343 = m2343;
        loop {
            let step = {
                let (v1, v2) = m2343;
                let m2340 = v1;
                if m2340 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2340 - 1;
                    let m2342 = {
                        let c2341: Fn22 = v0.clone();
                        c2341.apply(v2.clone())?
                    };
                    match m2342 {
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
                Ok(next) => { m2343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f329(v0)
}

pub fn f325(v0: Fn23, p2350: u64, p2351: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2349 = (p2350, p2351);
    {
        let mut m2349 = m2349;
        loop {
            let step = {
                let (v1, v2) = m2349;
                let m2346 = v1;
                if m2346 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2346 - 1;
                    let m2348 = {
                        let c2347: Fn23 = v0.clone();
                        c2347.apply(v2.clone())
                    };
                    match m2348 {
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
                Ok(next) => { m2349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f330(v0)
}

pub fn f327(v0: Fn24, p2356: u64, p2357: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2355 = (p2356, p2357);
    {
        let mut m2355 = m2355;
        loop {
            let step = {
                let (v1, v2) = m2355;
                let m2352 = v1;
                if m2352 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2352 - 1;
                    let m2354 = {
                        let c2353: Fn24 = v0.clone();
                        c2353.apply(v2.clone())?
                    };
                    match m2354 {
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
                Ok(next) => { m2355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f331(v0, v1)
}

pub fn f329(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2358, _) = v0;
        h2358
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2359) = v0;
        h2359
    };
    let m2360 = v1;
    match m2360.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2361 = v4;
            match m2361.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f332(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f330(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2362, _) = v0.clone();
        h2362
    };
    let v2: List<(Bytes, ())> = {
        let (h2363, _) = {
            let (_, h2364) = v0.clone();
            h2364
        };
        h2363
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2365) = {
            let (_, h2366) = v0;
            h2366
        };
        h2365
    };
    let m2367 = v1.clone();
    match m2367.uncons() {
        None => {
            let m2368 = v2;
            match m2368.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2369 = v2.clone();
            match m2369.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f333({
                        let (h2370, _) = v8.clone();
                        h2370
                    }, {
                        let (h2371, _) = v6.clone();
                        h2371
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

pub fn f331(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2372, _) = v1;
        h2372
    };
    let m2374 = {
        let (_, h2373) = v1;
        h2373
    };
    match m2374 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2377 = {
                let a2375 = v0.clone();
                let a2376 = v2;
                index_bytes(a2375, a2376)
            };
            match m2377 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2382 = {
                        let a2380 = v0;
                        let a2381 = {
                            let a2378 = v2;
                            let a2379 = 1u64;
                            nat_add(a2378, a2379)?
                        };
                        index_bytes(a2380, a2381)
                    };
                    match m2382 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f334(v4)? {
                                f334(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2383 = v2;
                                    let a2384 = 2u64;
                                    nat_add(a2383, a2384)?
                                }, Some({
                                    let a2389 = v3;
                                    let a2390 = f274(f335(), {
                                        let a2387 = {
                                            let a2385 = f336(v4)?;
                                            let a2386 = 16u64;
                                            nat_mul(a2385, a2386)?
                                        };
                                        let a2388 = f336(v5)?;
                                        nat_add(a2387, a2388)?
                                    });
                                    append_bytes(a2389, a2390)
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

pub fn f332(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a2395 = {
            let a2393 = {
                let a2391 = v0.clone();
                length_list(a2391)
            };
            let a2394 = {
                let a2392 = v1.clone();
                length_list(a2392)
            };
            nat_add(a2393, a2394)?
        };
        let a2396 = 1u64;
        nat_add(a2395, a2396)?
    };
    Ok(f337({
        let (_, h2397) = {
            let (_, h2398) = {
                let (h2399, _) = f338(Fn25::F339, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h2399
            };
            h2398
        };
        h2397
    }))
}

pub fn f333(v0: Bytes, v1: Bytes) -> bool {
    let a2404 = {
        let a2400 = v0;
        let a2401 = v1;
        compare_bytes(a2400, a2401)
    };
    let a2405 = {
        let a2402 = Bytes::lit(&[]);
        let a2403 = Bytes::lit(&[0x01]);
        compare_bytes(a2402, a2403)
    };
    equal(a2404, a2405)
}

pub fn f334(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2450 = {
            let a2426 = {
                let a2414 = {
                    let a2408 = {
                        let a2406 = v1;
                        let a2407 = 128u8;
                        fixed_u8::bit_and(a2406, a2407)
                    };
                    let a2409 = 128u8;
                    if equal(a2408, a2409) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2415 = {
                    let a2412 = {
                        let a2410 = v1;
                        let a2411 = 64u8;
                        fixed_u8::bit_and(a2410, a2411)
                    };
                    let a2413 = 64u8;
                    if equal(a2412, a2413) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2414, a2415)?
            };
            let a2427 = {
                let a2424 = {
                    let a2418 = {
                        let a2416 = v1;
                        let a2417 = 32u8;
                        fixed_u8::bit_and(a2416, a2417)
                    };
                    let a2419 = 32u8;
                    if equal(a2418, a2419) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2425 = {
                    let a2422 = {
                        let a2420 = v1;
                        let a2421 = 16u8;
                        fixed_u8::bit_and(a2420, a2421)
                    };
                    let a2423 = 16u8;
                    if equal(a2422, a2423) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2424, a2425)?
            };
            nat_add(a2426, a2427)?
        };
        let a2451 = {
            let a2448 = {
                let a2436 = {
                    let a2430 = {
                        let a2428 = v1;
                        let a2429 = 8u8;
                        fixed_u8::bit_and(a2428, a2429)
                    };
                    let a2431 = 8u8;
                    if equal(a2430, a2431) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2437 = {
                    let a2434 = {
                        let a2432 = v1;
                        let a2433 = 4u8;
                        fixed_u8::bit_and(a2432, a2433)
                    };
                    let a2435 = 4u8;
                    if equal(a2434, a2435) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2436, a2437)?
            };
            let a2449 = {
                let a2446 = {
                    let a2440 = {
                        let a2438 = v1;
                        let a2439 = 2u8;
                        fixed_u8::bit_and(a2438, a2439)
                    };
                    let a2441 = 2u8;
                    if equal(a2440, a2441) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2447 = {
                    let a2444 = {
                        let a2442 = v1;
                        let a2443 = 1u8;
                        fixed_u8::bit_and(a2442, a2443)
                    };
                    let a2445 = 1u8;
                    if equal(a2444, a2445) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2446, a2447)?
            };
            nat_add(a2448, a2449)?
        };
        nat_add(a2450, a2451)?
    };
    let a2452 = 48u64;
    let a2453 = v2;
    if if nat_le(a2452, a2453) {
        let a2454 = v2;
        let a2455 = 57u64;
        nat_le(a2454, a2455)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2456 = 97u64;
        let a2457 = v2;
        if nat_le(a2456, a2457) {
            let a2458 = v2;
            let a2459 = 102u64;
            Ok(nat_le(a2458, a2459))
        } else {
            Ok(false)
        }
    }
}

pub fn f335() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f336(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2504 = {
            let a2480 = {
                let a2468 = {
                    let a2462 = {
                        let a2460 = v1;
                        let a2461 = 128u8;
                        fixed_u8::bit_and(a2460, a2461)
                    };
                    let a2463 = 128u8;
                    if equal(a2462, a2463) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2469 = {
                    let a2466 = {
                        let a2464 = v1;
                        let a2465 = 64u8;
                        fixed_u8::bit_and(a2464, a2465)
                    };
                    let a2467 = 64u8;
                    if equal(a2466, a2467) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2468, a2469)?
            };
            let a2481 = {
                let a2478 = {
                    let a2472 = {
                        let a2470 = v1;
                        let a2471 = 32u8;
                        fixed_u8::bit_and(a2470, a2471)
                    };
                    let a2473 = 32u8;
                    if equal(a2472, a2473) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2479 = {
                    let a2476 = {
                        let a2474 = v1;
                        let a2475 = 16u8;
                        fixed_u8::bit_and(a2474, a2475)
                    };
                    let a2477 = 16u8;
                    if equal(a2476, a2477) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2478, a2479)?
            };
            nat_add(a2480, a2481)?
        };
        let a2505 = {
            let a2502 = {
                let a2490 = {
                    let a2484 = {
                        let a2482 = v1;
                        let a2483 = 8u8;
                        fixed_u8::bit_and(a2482, a2483)
                    };
                    let a2485 = 8u8;
                    if equal(a2484, a2485) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2491 = {
                    let a2488 = {
                        let a2486 = v1;
                        let a2487 = 4u8;
                        fixed_u8::bit_and(a2486, a2487)
                    };
                    let a2489 = 4u8;
                    if equal(a2488, a2489) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2490, a2491)?
            };
            let a2503 = {
                let a2500 = {
                    let a2494 = {
                        let a2492 = v1;
                        let a2493 = 2u8;
                        fixed_u8::bit_and(a2492, a2493)
                    };
                    let a2495 = 2u8;
                    if equal(a2494, a2495) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2501 = {
                    let a2498 = {
                        let a2496 = v1;
                        let a2497 = 1u8;
                        fixed_u8::bit_and(a2496, a2497)
                    };
                    let a2499 = 1u8;
                    if equal(a2498, a2499) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2500, a2501)?
            };
            nat_add(a2502, a2503)?
        };
        nat_add(a2504, a2505)?
    };
    let a2506 = v2;
    let a2507 = 57u64;
    if nat_le(a2506, a2507) {
        let a2508 = v2;
        let a2509 = 48u64;
        Ok(nat_sub(a2508, a2509))
    } else {
        let a2510 = v2;
        let a2511 = 87u64;
        Ok(nat_sub(a2510, a2511))
    }
}

pub fn f337(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f229(Fn10::F340, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f338(v0: Fn25, p2516: u64, p2517: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m2515 = (p2516, p2517);
    {
        let mut m2515 = m2515;
        loop {
            let step = {
                let (v1, v2) = m2515;
                let m2512 = v1;
                if m2512 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m2512 - 1;
                    let m2514 = {
                        let c2513: Fn25 = v0.clone();
                        c2513.apply(v2.clone())
                    };
                    match m2514 {
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
                Ok(next) => { m2515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f341(v0)
}

pub fn f340(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f341(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h2518, _) = v0.clone();
        h2518
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h2519, _) = {
            let (_, h2520) = v0.clone();
            h2520
        };
        h2519
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h2521) = {
            let (_, h2522) = v0;
            h2522
        };
        h2521
    };
    let m2523 = v1.clone();
    match m2523.uncons() {
        None => {
            let m2524 = v2;
            match m2524.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2525 = v2.clone();
            match m2525.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f333({
                        let (h2526, _) = v8.clone();
                        h2526
                    }, {
                        let (h2527, _) = v6.clone();
                        h2527
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
