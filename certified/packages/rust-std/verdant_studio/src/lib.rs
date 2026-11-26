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
    C0,
    C1(Bytes, std::rc::Rc<Adt17>),
    C2(bool),
    C3(bool, u64),
    C4(bool, u64, u64),
    C5(Bytes),
    C6(std::rc::Rc<Adt17>),
    C7(std::rc::Rc<Adt17>),
    C8,
    C9(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C10(Bytes, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(bool, bool, Bytes, List<Bytes>, List<List<Bytes>>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64, Adt22),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, Option<u64>),
    C1(u64),
    C2(Adt23, List<u64>, u64),
}

#[derive(Clone)]
pub enum Adt23 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt24 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Bytes, Adt17),
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
pub enum Adt30 {
    C0(bool, List<(u64, u64)>, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F64(u8, Bytes),
    F156(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn0::F156(k0) => f156(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F94,
    F198,
    F296,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F94 => f94(p0, p1),
            Fn1::F198 => f198(p0, p1),
            Fn1::F296 => f296(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F170(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F170(k0) => f170(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F353(Bytes),
    F493(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F353(k0) => f353(k0.clone(), p0),
            Fn3::F493(k0, k1) => Ok(f493(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F355,
    F703,
}

impl Fn4 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn4::F355 => f355(p0, p1),
            Fn4::F703 => f703(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F358,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn5::F358 => f358(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F364(Bytes),
    F365(u64, Bytes),
    F373(Bytes, u64, Bytes, u64),
    F486(Bytes),
    F487(Bytes),
    F537(Bytes),
    F575(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn6::F364(k0) => Ok(f364(k0.clone(), p0)),
            Fn6::F365(k0, k1) => f365(*k0, k1.clone(), p0),
            Fn6::F373(k0, k1, k2, k3) => f373(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn6::F486(k0) => f486(k0.clone(), p0),
            Fn6::F487(k0) => f487(k0.clone(), p0),
            Fn6::F537(k0) => f537(k0.clone(), p0),
            Fn6::F575(k0) => f575(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F384,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F384 => f384(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F387,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F387 => f387(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F389,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F389 => f389(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F392,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F392 => f392(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F417(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn11::F417(k0, k1, k2) => f417(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F450,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F450 => f450(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F453,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F453 => f453(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F485(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn14::F485(k0) => f485(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F490(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn15::F490(k0) => f490(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F497,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F497 => f497(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F499,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F499 => f499(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F517,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F517 => f517(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F541,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F541 => f541(p0, p1),
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
    F608,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F608 => f608(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F622,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F622 => f622(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F655,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F655 => f655(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F676(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F676(k0) => f676(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F686,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F686 => f686(p0),
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
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h308, _) => {
            h308
        }
    }, f112())) {
        Ok(Adt11::C1(List::cons(f112(), List::<Bytes>::nil())))
    } else {
        let m309 = f45(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f113(v0.clone())?)));
        match m309.uncons() {
            None => {
                let m310 = f114(v0.clone(), f113(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m310.uncons() {
                    None => {
                        f116(v0)
                    }
                    Some((v3, v4)) => {
                        Ok(Adt11::C4(f7(List::cons(v3, v4))))
                    }
                }
            }
            Some((v1, v2)) => {
                Ok(Adt11::C0(List::cons(v1, v2)))
            }
        }
    }
}

pub fn f70(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h311, _) => {
            h311
        }
    }, f117())) {
        Ok(Adt11::C1(List::cons(f117(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f118(f119(f117(), f120(match v0.clone() {
            Adt1::C0(_, h312, _) => {
                h312
            }
        }, f117())))?;
        let m313 = f45(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m313.uncons() {
            None => {
                let m314 = f121(v0.clone(), f122(f81(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f123(v2, v1.clone()))?;
                match m314 {
                    None => {
                        f124(v0, v1)
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
    let m315 = f97(v0.clone());
    match m315.uncons() {
        None => {
            let m316 = f98(f45(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f125(), List::<Bytes>::nil())))), f99(v0.clone())?);
            match m316.uncons() {
                None => {
                    Ok(Adt11::C3(f126(f127(v0.clone())?, List::cons((f128(), f129()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f130()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f131()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f132()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f133()), List::cons((f8(Str::lit("docs/catalog.md")), f134(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f135(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f136(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f137()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m321 = {
        let a318 = v0;
        let a319 = 0u64;
        let a320 = {
            let a317 = v1.clone();
            length_bytes(a317)
        };
        slice_bytes(a318, a319, a320)
    };
    match m321 {
        None => {
            false
        }
        Some(v2) => {
            let a322 = v2;
            let a323 = v1;
            equal(a322, a323)
        }
    }
}

pub fn f73(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f138(v3.clone(), v1.clone(), f139(v3, v1, false)?)
    } else {
        if f28(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f138(v3.clone(), v1.clone(), f140(v3, v1)?)
        } else {
            if f28(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f138(v3.clone(), v1.clone(), f139(v3, v1, true)?)
            } else {
                if f28(v0.clone(), f8(Str::lit("stage-registries"))) {
                    f138(v3.clone(), v1.clone(), f141(v3, v1)?)
                } else {
                    if f28(v0.clone(), f8(Str::lit("stage-encodings"))) {
                        f138(v3.clone(), v1.clone(), f142(v3, v1)?)
                    } else {
                        if f28(v0.clone(), f8(Str::lit("stage-mappings"))) {
                            f138(v3.clone(), v1.clone(), f143(v3, v1, false)?)
                        } else {
                            if f28(v0, f8(Str::lit("stage-matrix"))) {
                                f138(v3.clone(), v1.clone(), f143(v3, v1, true)?)
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

pub fn f74(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("admit"))) {
        f144(v3, v1)
    } else {
        if f28(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f145(v3, v1)
        } else {
            if f28(v0.clone(), f8(Str::lit("admit-registry"))) {
                f146(v3, v1, v2)
            } else {
                if f28(v0.clone(), f8(Str::lit("ingest-authority"))) {
                    f147(v3, v1)
                } else {
                    if f28(v0.clone(), f8(Str::lit("ingest-index"))) {
                        f148(v3, v1)
                    } else {
                        if f28(v0.clone(), f8(Str::lit("merge-gate"))) {
                            f149(v3, v1)
                        } else {
                            if f28(v0.clone(), f8(Str::lit("propose-encoding"))) {
                                let m324 = v2;
                                match m324.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f66()))
                                    }
                                    Some((v4, v5)) => {
                                        f150(v3, v1, v4, v5)
                                    }
                                }
                            } else {
                                if f28(v0, f8(Str::lit("record-decision"))) {
                                    f151(v3, v1)
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
    f152(v0.clone(), 0u64, {
        let a325 = v0;
        length_bytes(a325)
    })
}

pub fn f76(p330: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m329 = p330;
    {
        let mut m329 = m329;
        loop {
            let step = {
                let v0 = m329;
                let m326 = v0;
                match m326.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h327, _) = v2;
                            h327
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h328) = v2;
                                h328
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Bytes) -> R<Option<Adt13>> {
    let m331 = f153(v0)?;
    match m331 {
        None => {
            Ok(None::<Adt13>)
        }
        Some(v1) => {
            Ok(f154(v1))
        }
    }
}

pub fn f78(v0: Option<Adt13>) -> Option<Bytes> {
    let m332 = v0;
    match m332 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m333 = v1;
            match m333 {
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

pub fn f79(p339: Adt13, v1: Bytes) -> Option<Adt13> {
    let m338 = p339;
    {
        let mut m338 = m338;
        loop {
            let step = {
                let v0 = m338;
                let m334 = v0;
                match m334 {
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
                    Adt13::C4(r335) => {
                        let v5 = (*r335).clone();
                        Ok::<Adt13, Option<Adt13>>(v5)
                    }
                    Adt13::C5 => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C6(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C7(v2, r336, r337) => {
                        let v3 = (*r336).clone();
                        let v4 = (*r337).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt13, Option<Adt13>>(Some(v3))
                        } else {
                            Ok::<Adt13, Option<Adt13>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m340 = v0;
    match m340.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m341 = f155(v2, v1.clone())?;
            match m341 {
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
        let a342 = v0.clone();
        length_bytes(a342)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h343, _) = f63(Fn0::F156(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h343
    };
    let v3: u64 = {
        let (h344, _) = {
            let (_, h345) = v2.clone();
            h345
        };
        h344
    };
    let v4: List<Bytes> = {
        let (_, h346) = {
            let (_, h347) = v2;
            h347
        };
        h346
    };
    let a348 = v3;
    let a349 = v1;
    if nat_lt(a348, a349) {
        Ok(f65(List::cons(f35(v0, v3, {
            let a350 = v1;
            let a351 = v3;
            nat_sub(a350, a351)
        }), v4)))
    } else {
        Ok(f65(v4))
    }
}

pub fn f82(v0: Adt1, v1: Bytes) -> R<List<Adt13>> {
    Ok(f157(f46(v0, v1)?))
}

pub fn f83(v0: List<Adt13>, v1: Bytes) -> List<Adt13> {
    let m352 = v0;
    match m352.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f158(v2, v1.clone()), f83(v3, v1))
        }
    }
}

pub fn f84(p357: List<Bytes>, p358: List<Bytes>) -> bool {
    let m356 = (p357, p358);
    {
        let mut m356 = m356;
        loop {
            let step = {
                let (v0, v1) = m356;
                let m353 = v0;
                match m353.uncons() {
                    None => {
                        let m354 = v1;
                        match m354.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m355 = v1;
                        match m355.uncons() {
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
                Ok(next) => { m356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<Adt13>) -> List<Bytes> {
    let m359 = v0;
    match m359.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m360 = f79(v1, f8(Str::lit("std")));
            match m360 {
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
    let a361 = f159(v0)?;
    let a362 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a361, a362))
}

pub fn f87(v0: List<Adt13>) -> Adt13 {
    let m363 = v0;
    match m363.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f87(v2)))
        }
    }
}

pub fn f88(v0: Adt12) -> Bytes {
    let m364 = v0;
    match m364 {
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
    let a366 = {
        let a365 = v2.clone();
        length_bytes(a365)
    };
    let a367 = 0u64;
    if nat_eq(a366, a367) {
        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m368 = f160(v0.clone(), f49())?;
        match m368 {
            None => {
                if f40(f41(f163(), match v0.clone() {
                    Adt1::C0(_, _, h369) => {
                        h369
                    }
                })) {
                    Ok(Adt11::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f163()))
                } else {
                    let m370 = f45(v0.clone(), List::cons(f164(), f165(f49())?));
                    match m370.uncons() {
                        None => {
                            Ok(Adt11::C3(f166(v0.clone(), f49(), v1.clone(), v2.clone(), f167(f53(v0, f164()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt11::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f43(f161(v3.clone())?, f162(v3, v2)))
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h371, _) = v2.clone();
        h371
    };
    let v4: u64 = {
        let (h372, _) = {
            let (_, h373) = v2.clone();
            h373
        };
        h372
    };
    let v5: List<Bytes> = {
        let (_, h374) = {
            let (_, h375) = v2;
            h375
        };
        h374
    };
    let m376 = f168(v0.clone(), v3);
    match m376 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a377 = v6;
            let a378 = v1;
            if equal(a377, a378) {
                Ok(Some(({
                    let a379 = v3;
                    let a380 = 1u64;
                    nat_add(a379, a380)?
                }, ({
                    let a381 = v3;
                    let a382 = 1u64;
                    nat_add(a381, a382)?
                }, List::cons(f35(v0, v4, {
                    let a383 = v3;
                    let a384 = v4;
                    nat_sub(a383, a384)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a385 = v3;
                    let a386 = 1u64;
                    nat_add(a385, a386)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f93(v0: Fn1, p390: List<Bytes>, p391: List<Bytes>) -> List<Bytes> {
    let m389 = (p390, p391);
    {
        let mut m389 = m389;
        loop {
            let step = {
                let (v1, v2) = m389;
                let m387 = v2;
                match m387.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c388: Fn1 = v0.clone();
                            c388.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m389 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m392 = f169(Fn2::F170(v1), None::<Bytes>, v0);
    match m392 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f96() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::<Bytes>::nil()))))))))))))))))))))))))))
}

pub fn f97(v0: Adt1) -> List<Bytes> {
    if f111(match v0.clone() {
        Adt1::C0(_, h393, _) => {
            h393
        }
    }, f171()) {
        if f111(match v0 {
            Adt1::C0(_, h394, _) => {
                h394
            }
        }, f172()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f172(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f171(), List::<Bytes>::nil())
    }
}

pub fn f98(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m395 = v0;
    match m395.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f98(v3, v1))
        }
    }
}

pub fn f99(v0: Adt1) -> R<List<Bytes>> {
    Ok(f45(v0.clone(), List::cons(f173(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f109(v0)?)))))
}

pub fn f100(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f174(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f115(f82(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f101(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m396 = v0;
    match m396.uncons() {
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
    let m397 = v0;
    match m397.uncons() {
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
    let m398 = v0;
    match m398.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f98(f175(f157(f79(v1, f8(Str::lit("defers"))))), f103(v2))
        }
    }
}

pub fn f104(v0: List<Adt13>) -> List<Bytes> {
    let m399 = v0;
    match m399.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m400 = f79(v1, f8(Str::lit("std")));
            match m400 {
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
    let m401 = v0;
    match m401.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f126(f176(v2.clone()), f177(f178(v0, v1.clone(), v2)?));
    f179(f118(f180(v1))?, v3.clone(), {
        let a403 = {
            let a402 = v3;
            length_list(a402)
        };
        let a404 = 1u64;
        nat_add(a403, a404)?
    })
}

pub fn f107(v0: Adt14, v1: List<Adt13>) -> List<Adt13> {
    let m405 = v1;
    match m405.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            if if f181(v2.clone(), f8(Str::lit("subject"))) {
                f182(v0.clone(), v2.clone())
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
    let m406 = v1;
    match m406.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v2, v3)) => {
            let m407 = f183(f81(f53(v0.clone(), v2.clone()))?, f72(v2, f172()))?;
            match m407 {
                None => {
                    f108(v0, v3)
                }
                Some(v4) => {
                    Ok(f184(v4, f108(v0, v3)?))
                }
            }
        }
    }
}

pub fn f109(v0: Adt1) -> R<List<Bytes>> {
    Ok(f98(f118(f185(f171(), f120(match v0.clone() {
        Adt1::C0(_, h408, _) => {
            h408
        }
    }, f171())))?, f118(f185(f172(), f120(match v0 {
        Adt1::C0(_, h409, _) => {
            h409
        }
    }, f172())))?))
}

pub fn f110(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f111(p413: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
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
                        if f28({
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

pub fn f112() -> Bytes {
    f8(Str::lit("model/registries"))
}

pub fn f113(v0: Adt1) -> R<List<Bytes>> {
    f118(f185(f112(), f120(match v0 {
        Adt1::C0(_, h414, _) => {
            h414
        }
    }, f112())))
}

pub fn f114(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m415 = v1;
    match m415.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f98(if f41(v4.clone(), v3.clone()) {
                f186(v4.clone(), f81(f53(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f187(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
            }, f114(v0, v5, v2, v3)?))
        }
    }
}

pub fn f115(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m416 = v0;
    match m416.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f47(v2, v1.clone()), f115(v3, v1))
        }
    }
}

pub fn f116(v0: Adt1) -> R<Adt11> {
    let m417 = f45(v0.clone(), f188());
    match m417.uncons() {
        None => {
            let m418 = f189(v0.clone(), f188())?;
            match m418.uncons() {
                None => {
                    let m421 = f190(match v0.clone() {
                        Adt1::C0(_, h419, _) => {
                            h419
                        }
                    }, f98(f191(), f192(match v0.clone() {
                        Adt1::C0(_, h420, _) => {
                            h420
                        }
                    }, false)));
                    match m421.uncons() {
                        None => {
                            let m423 = f45(v0.clone(), f192(match v0.clone() {
                                Adt1::C0(_, h422, _) => {
                                    h422
                                }
                            }, true));
                            match m423.uncons() {
                                None => {
                                    let m424 = f45(v0.clone(), f193());
                                    match m424.uncons() {
                                        None => {
                                            let m425 = f194(v0.clone())?;
                                            match m425 {
                                                None => {
                                                    let m427 = f195(v0.clone(), f192(match v0.clone() {
                                                        Adt1::C0(_, h426, _) => {
                                                            h426
                                                        }
                                                    }, true))?;
                                                    match m427 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f196(Adt16::C0(Adt15::C0(f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f174(v0.clone())?, f115(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f52(f53(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f175(f82(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f192(match v0 {
                                                                Adt1::C0(_, h428, _) => {
                                                                    h428
                                                                }
                                                            }, true)))?;
                                                            let (h429, _) = v13;
                                                            if h429 {
                                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h430) = v13;
                                                                    h430
                                                                }))
                                                            } else {
                                                                Ok(Adt11::C4({
                                                                    let (_, h431) = v13;
                                                                    h431
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

pub fn f117() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f118(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f197(f93(Fn1::F198, List::<Bytes>::nil(), f199(v0)?)))
}

pub fn f119(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m432 = v1;
    match m432.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f200(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f201(v0.clone(), v2), f119(v0, v3))
            } else {
                f119(v0, v3)
            }
        }
    }
}

pub fn f120(p437: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m436 = p437;
    {
        let mut m436 = m436;
        loop {
            let step = {
                let v0 = m436;
                let m433 = v0;
                match m433.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h434, _) = v2;
                            h434
                        }, v1.clone()) {
                            let (_, h435) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h435)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m436 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt1, v1: List<List<Bytes>>, p440: List<Bytes>) -> R<Option<Bytes>> {
    let m439 = p440;
    {
        let mut m439 = m439;
        loop {
            let step = {
                let v2 = m439;
                let m438 = v2;
                match m438.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f202(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(p443: List<Bytes>, p444: List<Bytes>, p445: List<List<Bytes>>) -> List<List<Bytes>> {
    let m442 = (p443, (p444, p445));
    {
        let mut m442 = m442;
        loop {
            let step = {
                let (v0, (v1, v2)) = m442;
                let m441 = v0;
                match m441.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f203(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f28(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f203(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m442 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m446 = v0;
    match m446.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f123(v3, v1)
            } else {
                List::cons(v2, f123(v3, v1))
            }
        }
    }
}

pub fn f124(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h447, _) => {
            h447
        }
    }, f204())) {
        Ok(Adt11::C1(List::cons(f204(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f123(f118(f119(f204(), f120(match v0.clone() {
            Adt1::C0(_, h448, _) => {
                h448
            }
        }, f204())))?, v1.clone());
        let m449 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m449.uncons() {
            None => {
                let v5: List<Bytes> = f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m450 = f45(v0.clone(), f205(v0.clone(), v2.clone(), v5.clone())?);
                match m450.uncons() {
                    None => {
                        let m451 = f206(v0.clone(), v2, v5)?;
                        match m451 {
                            None => {
                                f207(v0, v1)
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

pub fn f125() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f126(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m452 = v0;
    match m452.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m453 = f82(v0.clone(), f125())?;
    match m453.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f208(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f128() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f129() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f130() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f209(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f131() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}      merges: ${{ steps.stale.outputs.merges }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          merges=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # Drafting drops a pull request's auto-merge, so the owner's merge method is kept\u{a}              # here and restored once the regenerated pull request is ready again.\u{a}              method=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge.merge_method // \"\"')\u{a}              if [ -n \"$method\" ]; then merges=\"$merges $pr:$method\"; fi\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"merges=$merges\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}          MERGES: ${{ needs.find.outputs.merges }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f209(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            # The owner's auto-merge comes back with the same method, and only where it was set.\u{a}            for entry in $MERGES; do\u{a}              if [ \"${entry/:*/}\" = \"$pr\" ] && ! gh pr merge \"$pr\" --auto \"--${entry#*:}\"; then\u{a}                echo \"::error::#${pr}: its auto-merge (${entry#*:}) could not be restored\"\u{a}                failed=\"$failed $pr\"\u{a}              fi\u{a}            done\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f132() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, and readers when it exists.\u{a}pids=\"\"\u{a}for project in studio readers; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.$project\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}"))
}

pub fn f133() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f134(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f100(v0.clone())?;
    let v2: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt13> = f107(v1.clone(), f108(v0.clone(), f109(v0.clone())?)?);
    let v4: List<List<Bytes>> = f210(f211(f53(v0, f173()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f212(v4.clone(), f213(v4), f178(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f106(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f135(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f214(f100(v0.clone())?, f108(v0.clone(), f109(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f136(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f215(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f216(f82(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f137() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f138(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m454 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m454.uncons() {
        None => {
            if f217(f81(f53(v0, v1))?)? {
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

pub fn f139(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m455 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m455.uncons() {
        None => {
            let v5: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
            if f40(f200(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f218(v5)?
                } else {
                    f219(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f201(f220(), f221(v1.clone())?), f53(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f201(f220(), f221(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f222()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f140(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m456 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m456.uncons() {
        None => {
            let m457 = f223(f81(f53(v0.clone(), v1.clone()))?)?;
            match m457 {
                None => {
                    Ok(Adt11::C4(f224()))
                }
                Some(v4) => {
                    let m458 = f225(f81(f53(v0, v1))?)?;
                    match m458 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f95(f38(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f226(v5)))
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

pub fn f141(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let v2: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
    if f40(f200(v1.clone(), f8(Str::lit(".jsonl")))) {
        Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
    } else {
        if f227(f228(f229(v2.clone())?), f40({
            let a460 = {
                let a459 = f230(f229(v2)?);
                length_list(a459)
            };
            let a461 = 0u64;
            nat_eq(a460, a461)
        })) {
            Ok(Adt11::C3(List::cons((f201(f220(), f221(v1.clone())?), f53(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f201(f220(), f221(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt11::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f142(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m462 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m462.uncons() {
        None => {
            let m463 = f231(f81(f53(v0, v1))?)?;
            match m463 {
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

pub fn f143(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m464 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m464.uncons() {
        None => {
            let v5: Bytes = f201(if v2 {
                f172()
            } else {
                f171()
            }, f221(v1.clone())?);
            if f40(f200(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m465 = f183(f81(f53(v0.clone(), v1.clone()))?, v2)?;
                match m465 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a467 = {
                            let a466 = v6;
                            length_list(a466)
                        };
                        let a468 = 0u64;
                        if nat_eq(a467, a468) {
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

pub fn f144(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m469 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m469.uncons() {
        None => {
            let v4: List<Bytes> = f81(f53(v0.clone(), v1))?;
            let m470 = f232(v4.clone(), f8(Str::lit("requirement")))?;
            match m470 {
                None => {
                    Ok(Adt11::C4(f222()))
                }
                Some(v5) => {
                    let m471 = f232(v4, f8(Str::lit("admission")))?;
                    match m471 {
                        None => {
                            Ok(Adt11::C4(f222()))
                        }
                        Some(v6) => {
                            let v7: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f98(f115(v5.clone(), f8(Str::lit("id"))), f115(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f118(f98(f115(v7.clone(), f8(Str::lit("id"))), f115(v8.clone(), f8(Str::lit("construct")))))?;
                            let m472 = f233(f104(v6.clone()), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m472 {
                                None => {
                                    let a474 = {
                                        let a473 = v9.clone();
                                        length_list(a473)
                                    };
                                    let a475 = 0u64;
                                    if nat_eq(a474, a475) {
                                        Ok(Adt11::C4(f222()))
                                    } else {
                                        if f40({
                                            let a478 = {
                                                let a476 = f118(f98(v9.clone(), v10.clone()))?;
                                                length_list(a476)
                                            };
                                            let a479 = {
                                                let a477 = f98(v9.clone(), v10);
                                                length_list(a477)
                                            };
                                            nat_eq(a478, a479)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f235(f8(Str::lit("model/requirements.json")), f184(v7, v5))?, List::cons(f235(f8(Str::lit("model/admissions.json")), f184(v8, v6))?, List::cons(f236(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f95(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f234(v11))
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

pub fn f145(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m480 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m480.uncons() {
        None => {
            let m481 = f237(v0.clone(), v1.clone())?;
            match m481 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m482 = f238(v4.clone(), f8(Str::lit("schema")));
                    match m482 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f239(v0, f240(v5.clone())?, v5, f241(v4.clone(), f8(Str::lit("name"))), f242(f243(f238(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f146(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt11> {
    let m483 = v2;
    match m483.uncons() {
        None => {
            Ok(Adt11::C4(f66()))
        }
        Some((v3, v4)) => {
            let m484 = v4;
            match m484.uncons() {
                None => {
                    Ok(Adt11::C4(f66()))
                }
                Some((v5, v6)) => {
                    let m485 = v6.clone();
                    match m485.uncons() {
                        None => {
                            Ok(Adt11::C4(f66()))
                        }
                        Some((v7, v8)) => {
                            let m486 = v8;
                            match m486.uncons() {
                                None => {
                                    Ok(Adt11::C4(f66()))
                                }
                                Some((v9, _)) => {
                                    if f40(f72(v7, f8(Str::lit("read-")))) {
                                        Ok(Adt11::C4(f8(Str::lit("name the verdant_readers command that reads the artifact: admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT\u{a}"))))
                                    } else {
                                        let m487 = f45(v0.clone(), List::cons(f244(v1.clone()), List::cons(v9.clone(), List::cons(f8(Str::lit("model/admissions.json")), if f28(v5.clone(), f8(Str::lit("none"))) {
                                            List::<Bytes>::nil()
                                        } else {
                                            List::cons(f7(List::cons(f8(Str::lit("encodings/")), List::cons(v5.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), List::<Bytes>::nil())
                                        }))));
                                        match m487.uncons() {
                                            None => {
                                                if f40(f56(v0.clone(), f244(v1.clone()))) {
                                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v1.clone(), List::cons(f8(Str::lit(" is not a bound authority, because ")), List::cons(f244(v1), List::cons(f8(Str::lit(" is absent, and every admitted value cites a bound standard (REQ-ADMISSION)\u{a}")), List::<Bytes>::nil()))))))))
                                                } else {
                                                    let m488 = f46(v0.clone(), f244(v1.clone()))?;
                                                    match m488 {
                                                        None => {
                                                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(f244(v1), List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                        Some(v13) => {
                                                            let v15: Bytes = {
                                                                let m489 = f79(v13, f8(Str::lit("source")));
                                                                match m489 {
                                                                    None => {
                                                                        Bytes::lit(&[])
                                                                    }
                                                                    Some(v14) => {
                                                                        f47(v14, f8(Str::lit("sha256")))
                                                                    }
                                                                }
                                                            };
                                                            let v16: Bytes = f245(f53(v0.clone(), v9.clone()))?;
                                                            if f40(f28(v16.clone(), v15.clone())) {
                                                                Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v9, List::cons(f8(Str::lit(" is not the artifact ")), List::cons(v1, List::cons(f8(Str::lit(" binds: its SHA-256 is ")), List::cons(v16, List::cons(f8(Str::lit(" and the binding's source.sha256 is ")), List::cons(v15, List::cons(f8(Str::lit(" (FIPS 180-4 section 6.2)\u{a}")), List::<Bytes>::nil()))))))))))))
                                                            } else {
                                                                f246(v0, v1, v3, v5, v6, v9)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Some((v11, v12)) => {
                                                Ok(Adt11::C0(List::cons(v11, v12)))
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

pub fn f147(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m490 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m490.uncons() {
        None => {
            let m491 = f46(v0.clone(), v1.clone())?;
            match m491 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f40(f28(f47(v4.clone(), f8(Str::lit("schema"))), f247())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt13 = {
                            let m492 = f79(v4.clone(), f8(Str::lit("source")));
                            match m492 {
                                None => {
                                    Adt13::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f78(f79(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f201(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f201(v9.clone(), f8(Str::lit("binding.json"))), f53(v0.clone(), v1));
                        let m493 = v8;
                        match m493 {
                            None => {
                                f248(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f40(f28(f47(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f248(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f201(v9, f221(f47(v7.clone(), f8(Str::lit("url"))))?);
                                    if f40(f41(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h494) => {
                                            h494
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f47(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m495 = f45(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m495.uncons() {
                                            None => {
                                                if f28(f245(f53(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f248(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f148(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m496 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m496.uncons() {
        None => {
            let v4: Bytes = f53(v0.clone(), v1);
            let v5: List<List<Bytes>> = f211(v4.clone())?;
            if f40(f84(f249(v5.clone()), f250())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f251(Str::lit("id"), v6.clone())?, List::cons(f251(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f251(Str::lit("artifact"), f173())?, List::cons(f251(Str::lit("checksum"), f245(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f173(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f252(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f236(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f95(f213(f210(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f149(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m497 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m497.uncons() {
        None => {
            let m498 = f46(v0, v1)?;
            match m498 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f253(f157(Some(v4))) {
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

pub fn f150(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m499 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m499.uncons() {
        None => {
            let m500 = f237(v0.clone(), v1.clone())?;
            match m500 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f239(v0.clone(), f53(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f151(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m501 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f125(), List::<Bytes>::nil())));
    match m501.uncons() {
        None => {
            let m502 = f254(f81(f53(v0.clone(), v1))?)?;
            match m502 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt13> = f82(v0, f125())?;
                    let v6: List<Bytes> = f115(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f118(f115(v5.clone(), f8(Str::lit("at"))))?;
                    let a504 = {
                        let a503 = v6.clone();
                        length_list(a503)
                    };
                    let a505 = 0u64;
                    if if nat_eq(a504, a505) {
                        true
                    } else {
                        f40({
                            let a508 = {
                                let a506 = f118(f98(v6.clone(), v7.clone()))?;
                                length_list(a506)
                            };
                            let a509 = {
                                let a507 = f98(v6.clone(), v7);
                                length_list(a507)
                            };
                            nat_eq(a508, a509)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f235(f125(), f184(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f95(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f152(v0: Bytes, p563: u64, p564: u64) -> R<bool> {
    let m562 = (p563, p564);
    {
        let mut m562 = m562;
        loop {
            let step = {
                let (v1, v2) = m562;
                let m510 = v2;
                if m510 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m510 - 1;
                    let m513 = {
                        let a511 = v0.clone();
                        let a512 = v1;
                        index_bytes(a511, a512)
                    };
                    match m513 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f255({
                                let v5: u8 = v4;
                                let a558 = {
                                    let a534 = {
                                        let a522 = {
                                            let a516 = {
                                                let a514 = v5;
                                                let a515 = 128u8;
                                                fixed_u8::bit_and(a514, a515)
                                            };
                                            let a517 = 128u8;
                                            if equal(a516, a517) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a523 = {
                                            let a520 = {
                                                let a518 = v5;
                                                let a519 = 64u8;
                                                fixed_u8::bit_and(a518, a519)
                                            };
                                            let a521 = 64u8;
                                            if equal(a520, a521) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a522, a523)?
                                    };
                                    let a535 = {
                                        let a532 = {
                                            let a526 = {
                                                let a524 = v5;
                                                let a525 = 32u8;
                                                fixed_u8::bit_and(a524, a525)
                                            };
                                            let a527 = 32u8;
                                            if equal(a526, a527) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a533 = {
                                            let a530 = {
                                                let a528 = v5;
                                                let a529 = 16u8;
                                                fixed_u8::bit_and(a528, a529)
                                            };
                                            let a531 = 16u8;
                                            if equal(a530, a531) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a532, a533)?
                                    };
                                    nat_add(a534, a535)?
                                };
                                let a559 = {
                                    let a556 = {
                                        let a544 = {
                                            let a538 = {
                                                let a536 = v5;
                                                let a537 = 8u8;
                                                fixed_u8::bit_and(a536, a537)
                                            };
                                            let a539 = 8u8;
                                            if equal(a538, a539) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a545 = {
                                            let a542 = {
                                                let a540 = v5;
                                                let a541 = 4u8;
                                                fixed_u8::bit_and(a540, a541)
                                            };
                                            let a543 = 4u8;
                                            if equal(a542, a543) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a544, a545)?
                                    };
                                    let a557 = {
                                        let a554 = {
                                            let a548 = {
                                                let a546 = v5;
                                                let a547 = 2u8;
                                                fixed_u8::bit_and(a546, a547)
                                            };
                                            let a549 = 2u8;
                                            if equal(a548, a549) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a555 = {
                                            let a552 = {
                                                let a550 = v5;
                                                let a551 = 1u8;
                                                fixed_u8::bit_and(a550, a551)
                                            };
                                            let a553 = 1u8;
                                            if equal(a552, a553) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a554, a555)?
                                    };
                                    nat_add(a556, a557)?
                                };
                                nat_add(a558, a559)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a560 = v1;
                                    let a561 = 1u64;
                                    nat_add(a560, a561)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
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

pub fn f153(v0: Bytes) -> R<Option<Adt17>> {
    let v1: Bytes = f256(v0)?;
    let m568 = f257(v1.clone(), 0u64, {
        let a566 = {
            let a565 = v1.clone();
            length_bytes(a565)
        };
        let a567 = 2u64;
        nat_add(a566, a567)?
    }, 0u64)?;
    match m568 {
        None => {
            Ok(None::<Adt17>)
        }
        Some(v2) => {
            let a571 = {
                let (_, h569) = v2;
                h569
            };
            let a572 = {
                let a570 = v1;
                length_bytes(a570)
            };
            if nat_eq(a571, a572) {
                Ok(Some({
                    let (h573, _) = v2;
                    h573
                }))
            } else {
                Ok(None::<Adt17>)
            }
        }
    }
}

pub fn f154(v0: Adt17) -> Option<Adt13> {
    let m574 = v0;
    match m574 {
        Adt17::C0 => {
            Some(Adt13::C3(std::rc::Rc::new(Adt13::C5)))
        }
        Adt17::C1(_, _) => {
            None::<Adt13>
        }
        Adt17::C2(v3) => {
            Some(Adt13::C0(v3))
        }
        Adt17::C3(v4, v5) => {
            Some(Adt13::C1(v4, v5))
        }
        Adt17::C4(_, _, _) => {
            None::<Adt13>
        }
        Adt17::C5(v9) => {
            Some(Adt13::C2(v9))
        }
        Adt17::C6(r581) => {
            let v10 = (*r581).clone();
            let m575 = f154(v10);
            match m575 {
                None => {
                    None::<Adt13>
                }
                Some(v11) => {
                    Some(Adt13::C3(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt17::C7(r582) => {
            let v12 = (*r582).clone();
            let m576 = f154(v12);
            match m576 {
                None => {
                    None::<Adt13>
                }
                Some(v13) => {
                    Some(Adt13::C4(std::rc::Rc::new(v13)))
                }
            }
        }
        Adt17::C8 => {
            Some(Adt13::C5)
        }
        Adt17::C9(r583, r584) => {
            let v14 = (*r583).clone();
            let v15 = (*r584).clone();
            let m577 = f154(v14);
            match m577 {
                None => {
                    None::<Adt13>
                }
                Some(v16) => {
                    let m578 = f154(v15);
                    match m578 {
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
        Adt17::C10(v18, r585, r586) => {
            let v19 = (*r585).clone();
            let v20 = (*r586).clone();
            let m579 = f154(v19);
            match m579 {
                None => {
                    None::<Adt13>
                }
                Some(v21) => {
                    let m580 = f154(v20);
                    match m580 {
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

pub fn f155(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), v1) {
        let m587 = f33(v0, 34u8)?;
        match m587.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m588 = v3;
                match m588.uncons() {
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

pub fn f156(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f258(v0, v1)
}

pub fn f157(v0: Option<Adt13>) -> List<Adt13> {
    let m589 = v0;
    match m589 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            f259(v1)
        }
    }
}

pub fn f158(v0: Adt13, v1: Bytes) -> Adt13 {
    let m590 = v0;
    match m590 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r591) => {
            let v10 = (*r591).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r592) => {
            let v5 = (*r592).clone();
            Adt13::C4(std::rc::Rc::new(f158(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r593, r594) => {
            let v11 = (*r593).clone();
            let v12 = (*r594).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r595, r596) => {
            let v3 = (*r595).clone();
            let v4 = (*r596).clone();
            if f28(v2.clone(), f8(Str::lit("std"))) {
                Adt13::C7(v2, std::rc::Rc::new(f260(v3, v1.clone())), std::rc::Rc::new(f158(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f158(v4, v1)))
            }
        }
    }
}

pub fn f159(v0: Adt13) -> R<Bytes> {
    let m597 = v0;
    match m597 {
        Adt13::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt13::C1(v2, v3) => {
            f261(v2, v3)
        }
        Adt13::C2(v4) => {
            f262(v4)
        }
        Adt13::C3(r598) => {
            let v5 = (*r598).clone();
            Ok(f263(f264(v5)?))
        }
        Adt13::C4(r599) => {
            let v6 = (*r599).clone();
            f265(f266(v6)?)
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

pub fn f160(v0: Adt1, p603: List<Adt12>) -> R<Option<Adt12>> {
    let m602 = p603;
    {
        let mut m602 = m602;
        loop {
            let step = {
                let v1 = m602;
                let m600 = v1;
                match m600.uncons() {
                    None => {
                        Err::<List<Adt12>, R<Option<Adt12>>>(Ok(None::<Adt12>))
                    }
                    Some((v2, v3)) => {
                        if f41(f161(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h601) => {
                                h601
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
                Ok(next) => { m602 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: Adt12) -> R<Bytes> {
    Ok(f201(f267(), f221(f268(v0))?))
}

pub fn f162(v0: Adt12, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f268(v0), List::<Bytes>::nil())))))
}

pub fn f163() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f164() -> Bytes {
    f269()
}

pub fn f165(v0: List<Adt12>) -> R<List<Bytes>> {
    let m604 = v0;
    match m604.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f161(v1)?, f165(v2)?))
        }
    }
}

pub fn f166(v0: Adt1, v1: List<Adt12>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m605 = v1;
    match m605.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f57(f58(v5.clone(), f50(v2.clone()))), f270(v5.clone(), v2.clone(), v3.clone(), f245(f53(v0.clone(), f161(v5)?))?, v4.clone())?), f166(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f167(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f271(v0.clone())?;
    let v2: u64 = f272(v0.clone(), v1)?;
    Ok(f35(v0, v2, {
        let a606 = v1;
        let a607 = v2;
        nat_sub(a606, a607)
    }))
}

pub fn f168(v0: Bytes, v1: u64) -> Option<u8> {
    let a608 = v0;
    let a609 = v1;
    index_bytes(a608, a609)
}

pub fn f169(v0: Fn2, p613: Option<Bytes>, p614: List<Bytes>) -> Option<Bytes> {
    let m612 = (p613, p614);
    {
        let mut m612 = m612;
        loop {
            let step = {
                let (v1, v2) = m612;
                let m610 = v2;
                match m610.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c611: Fn2 = v0.clone();
                            c611.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m612 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m615 = v1;
    match m615 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a618 = {
                    let a616 = v3;
                    let a617 = v0;
                    append_bytes(a616, a617)
                };
                let a619 = v2;
                append_bytes(a618, a619)
            })
        }
    }
}

pub fn f171() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f172() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f173() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f174(v0: Adt1) -> R<List<Bytes>> {
    let m620 = f211(f53(v0, f173()))?;
    match m620.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f213(v2))
        }
    }
}

pub fn f175(v0: List<Adt13>) -> List<Bytes> {
    let m621 = v0;
    match m621.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m622 = f78(Some(v1));
            match m622 {
                None => {
                    f175(v2)
                }
                Some(v3) => {
                    List::cons(v3, f175(v2))
                }
            }
        }
    }
}

pub fn f176(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m623 = v0;
    match m623.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f47(v1.clone(), f8(Str::lit("subject"))), f47(v1, f8(Str::lit("object")))), f176(v2))
        }
    }
}

pub fn f177(v0: List<Adt18>) -> List<(Bytes, Bytes)> {
    let m624 = v0;
    match m624.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt18::C0(h625, _, _) => {
                    h625
                }
            }, match v1 {
                Adt18::C0(_, _, h626) => {
                    h626
                }
            }), f177(v2))
        }
    }
}

pub fn f178(v0: Adt14, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Adt18>> {
    let v3: List<(Bytes, Bytes)> = f273(v1);
    let v4: List<(Bytes, Bytes)> = f274(v0, v2);
    f275(v3.clone(), f126(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f179(p633: List<Bytes>, v1: List<(Bytes, Bytes)>, p634: u64) -> R<List<Bytes>> {
    let m632 = (p633, p634);
    {
        let mut m632 = m632;
        loop {
            let step = {
                let (v0, v2) = m632;
                let m627 = v2;
                if m627 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m627 - 1;
                    let v4: List<Bytes> = f118(f98(v0.clone(), f276(v0.clone(), v1.clone())))?;
                    let a630 = {
                        let a628 = v4.clone();
                        length_list(a628)
                    };
                    let a631 = {
                        let a629 = v0.clone();
                        length_list(a629)
                    };
                    if nat_eq(a630, a631) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m632 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: List<Adt13>) -> List<Bytes> {
    let m635 = v0;
    match m635.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m636 = f79(v1, f8(Str::lit("std")));
            match m636 {
                None => {
                    f180(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3.clone(), f8(Str::lit("authority"))), if f181(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f47(v3, f8(Str::lit("row"))), f180(v2))
                    } else {
                        f180(v2)
                    })
                }
            }
        }
    }
}

pub fn f181(v0: Adt13, v1: Bytes) -> bool {
    f40({
        let a638 = {
            let a637 = f47(v0, v1);
            length_bytes(a637)
        };
        let a639 = 0u64;
        nat_eq(a638, a639)
    })
}

pub fn f182(v0: Adt14, v1: Adt13) -> bool {
    if f277(v0.clone(), v1.clone(), f47(v1.clone(), f8(Str::lit("subject")))) {
        f277(v0, v1.clone(), f47(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f183(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt13>>> {
    let m640 = v0;
    match m640.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f167(v2)?;
            let a642 = {
                let a641 = v4.clone();
                length_bytes(a641)
            };
            let a643 = 0u64;
            if nat_eq(a642, a643) {
                f183(v3, v1)
            } else {
                let m644 = f77(v4)?;
                match m644 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f278(v5.clone())
                        } else {
                            f279(v5.clone())
                        } {
                            let m645 = f183(v3, v1)?;
                            match m645 {
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

pub fn f184(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m646 = v0;
    match m646.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f184(v3, v1))
        }
    }
}

pub fn f185(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m647 = v1;
    match m647.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f200(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f201(v0.clone(), v2), f185(v0, v3))
            } else {
                f185(v0, v3)
            }
        }
    }
}

pub fn f186(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m648 = v1;
    match m648.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f98({
                let a650 = {
                    let a649 = f167(v4.clone())?;
                    length_bytes(a649)
                };
                let a651 = 0u64;
                if nat_eq(a650, a651) {
                    List::<Bytes>::nil()
                } else {
                    f280(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f186(v0, v5, v2, v3)?))
        }
    }
}

pub fn f187(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f188() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f173(), List::<Bytes>::nil())))))))
}

pub fn f189(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m652 = v1;
    match m652.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f56(v0.clone(), v2.clone()) {
                f200(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m653 = f46(v0.clone(), v2.clone())?;
                match m653 {
                    None => {
                        Ok(List::cons(v2, f189(v0, v3)?))
                    }
                    Some(_) => {
                        f189(v0, v3)
                    }
                }
            } else {
                f189(v0, v3)
            }
        }
    }
}

pub fn f190(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m654 = v1;
    match m654.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f111(v0.clone(), v2.clone()) {
                f190(v0, v3)
            } else {
                List::cons(v2, f190(v0, v3))
            }
        }
    }
}

pub fn f191() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f192(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m655 = v0;
    match m655.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f98(f281({
                let (h656, _) = v2;
                h656
            }, {
                let (_, h657) = v2;
                h657
            }, v1), f192(v3, v1))
        }
    }
}

pub fn f193() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f194(v0: Adt1) -> R<Option<Bytes>> {
    f282(f283(v0, f193())?)
}

pub fn f195(v0: Adt1, p661: List<Bytes>) -> R<Option<Bytes>> {
    let m660 = p661;
    {
        let mut m660 = m660;
        loop {
            let step = {
                let v1 = m660;
                let m658 = v1;
                match m658.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m659 = f284(f53(v0.clone(), v2.clone()), f285())?;
                        match m659 {
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
                Ok(next) => { m660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h662, _, _, _, _) => {
            h662
        }
    };
    let v2: Adt19 = f286(v1.clone())?;
    let v3: List<Bytes> = f287(v1.clone());
    let v4: List<Bytes> = f115(match v0.clone() {
        Adt16::C0(_, _, h663, _, _) => {
            h663
        }
    }, f8(Str::lit("path")));
    let m665 = f288(v2, List::<u64>::nil(), f289(v1.clone(), match v0.clone() {
        Adt16::C0(_, h664, _, _, _) => {
            h664
        }
    }, 0u64)?);
    match m665 {
        None => {
            if f291(f292(match v0.clone() {
                Adt16::C0(_, _, _, _, h667) => {
                    h667
                }
            }, f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h668) => {
                    h668
                }
            }, v4.clone()))?, f292(v4.clone(), f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h669) => {
                    h669
                }
            }, v4.clone()))?, f293(match v0.clone() {
                Adt16::C0(_, _, h670, _, _) => {
                    h670
                }
            })) {
                let m674 = f294(f292(match v0.clone() {
                    Adt16::C0(_, _, _, h671, _) => {
                        h671
                    }
                }, f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h672, _) => {
                        h672
                    }
                }))?, f292(v3.clone(), f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h673, _) => {
                        h673
                    }
                }))?);
                match m674 {
                    None => {
                        let m677 = f295(f115(match v0 {
                            Adt16::C0(_, h676, _, _, _) => {
                                h676
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m677 {
                            None => {
                                let m678 = f295(v4, List::<Bytes>::nil());
                                match m678 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f290(f98(v3, match v0 {
                            Adt16::C0(_, _, _, h675, _) => {
                                h675
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f290(match v1 {
                Adt15::C0(_, _, _, _, h666) => {
                    h666
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f197(v0: List<Bytes>) -> List<Bytes> {
    f93(Fn1::F296, List::<Bytes>::nil(), v0)
}

pub fn f198(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f297(v0, v1)
}

pub fn f199(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f298(f299(f300(v0))?))
}

pub fn f200(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a679 = v0.clone();
        length_bytes(a679)
    };
    let v3: u64 = {
        let a680 = v1.clone();
        length_bytes(a680)
    };
    let a681 = v3;
    let a682 = v2;
    if nat_le(a681, a682) {
        let m688 = {
            let a685 = v0;
            let a686 = {
                let a683 = v2;
                let a684 = v3;
                nat_sub(a683, a684)
            };
            let a687 = v3;
            slice_bytes(a685, a686, a687)
        };
        match m688 {
            None => {
                false
            }
            Some(v4) => {
                let a689 = v4;
                let a690 = v1;
                equal(a689, a690)
            }
        }
    } else {
        false
    }
}

pub fn f201(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f202(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m691 = f46(v0, v2)?;
    match m691 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f301(v1, f47(v3.clone(), f8(Str::lit("id"))))?;
            let m692 = v4.clone();
            match m692.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m694 = f78(f79({
                        let m693 = f79(v3.clone(), f8(Str::lit("source")));
                        match m693 {
                            None => {
                                Adt13::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m694 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f40(f41(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f302(f303(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f203(p697: List<Bytes>, p698: List<Bytes>) -> List<Bytes> {
    let m696 = (p697, p698);
    {
        let mut m696 = m696;
        loop {
            let step = {
                let (v0, v1) = m696;
                let m695 = v0;
                match m695.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f204() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f205(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m699 = v1;
    match m699.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt17 = f304(v0.clone(), v3)?;
            if f41(f305(f241(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f306(v5)?, f205(v0, v4, v2)?))
            } else {
                f205(v0, v4, v2)
            }
        }
    }
}

pub fn f206(v0: Adt1, p703: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m702 = p703;
    {
        let mut m702 = m702;
        loop {
            let step = {
                let v1 = m702;
                let m700 = v1;
                match m700.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt17 = f304(v0.clone(), v3.clone())?;
                        if if f41(f305(f241(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f28(f240({
                                let m701 = f238(v5.clone(), f8(Str::lit("schema")));
                                match m701 {
                                    None => {
                                        Adt17::C8
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f53(v0.clone(), f306(v5)?))
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
                Ok(next) => { m702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h704, _) => {
            h704
        }
    }, f220())) {
        Ok(Adt11::C1(List::cons(f220(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f123(f118(f307(f120(match v0.clone() {
            Adt1::C0(_, h705, _) => {
                h705
            }
        }, f220())))?, v1.clone());
        let m706 = f45(v0.clone(), List::cons(f125(), List::cons(f173(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m706.uncons() {
            None => {
                let m707 = f45(v0.clone(), f308(v0.clone(), v2.clone())?);
                match m707.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f309(v0.clone(), v2, v1)?;
                        let (h708, _) = v7;
                        if h708 {
                            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                let (_, h709) = v7;
                                h709
                            }))
                        } else {
                            let m710 = f97(v0.clone());
                            match m710.uncons() {
                                None => {
                                    let m711 = f99(v0.clone())?;
                                    match m711.uncons() {
                                        None => {
                                            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                                let (_, h712) = v7;
                                                h712
                                            }, List::cons(f7(f118(f310(f100(v0.clone())?, f108(v0.clone(), f109(v0.clone())?)?))?), List::cons(f311(v0)?, List::<Bytes>::nil()))))))
                                        }
                                        Some((v10, v11)) => {
                                            Ok(Adt11::C0(List::cons(v10, v11)))
                                        }
                                    }
                                }
                                Some((v8, v9)) => {
                                    Ok(Adt11::C1(List::cons(v8, v9)))
                                }
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        Ok(Adt11::C0(List::cons(v5, v6)))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt11::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f208(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f312(f82(v0, f125())?)?)))
}

pub fn f209() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f210(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m713 = v0;
    match m713.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f211(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt20 = f313(v0.clone(), 0u64, {
        let a714 = v0;
        length_bytes(a714)
    }, Adt20::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt20 = {
        let a717 = {
            let a716 = match v1.clone() {
                Adt20::C0(_, _, h715, _, _) => {
                    h715
                }
            };
            length_bytes(a716)
        };
        let a718 = 0u64;
        if if nat_eq(a717, a718) {
            let m720 = match v1.clone() {
                Adt20::C0(_, _, _, h719, _) => {
                    h719
                }
            };
            match m720.uncons() {
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
            f314(v1)
        }
    };
    Ok(f315(match v4 {
        Adt20::C0(_, _, _, _, h721) => {
            h721
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f212(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt18>, v3: List<Adt13>, v4: List<Bytes>) -> List<Bytes> {
    let m722 = v1;
    match m722.uncons() {
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
                let a724 = {
                    let a723 = v7.clone();
                    length_list(a723)
                };
                let a725 = 0u64;
                if nat_eq(a724, a725) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f95(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f212(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f213(v0: List<List<Bytes>>) -> List<Bytes> {
    let m726 = v0;
    match m726.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m727 = v1;
            match m727.uncons() {
                None => {
                    f213(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f213(v2))
                }
            }
        }
    }
}

pub fn f214(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m728 = v1;
    match m728.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f181(v2.clone(), f8(Str::lit("domain"))) {
                f319(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f47(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m729 = f79(v2.clone(), f8(Str::lit("role")));
                    match m729 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f47(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f47(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f320(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m730 = f79(v2.clone(), f8(Str::lit("origin")));
                    match m730 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f47(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m731 = f79(v2, f8(Str::lit("evidence")));
                    match m731 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f47(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f214(v0, v3))
            } else {
                f214(v0, v3)
            }
        }
    }
}

pub fn f215(v0: List<Adt13>) -> List<Bytes> {
    let m732 = v0;
    match m732.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f47(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f47(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f47(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f215(v2))
        }
    }
}

pub fn f216(v0: List<Adt13>) -> List<Bytes> {
    let m733 = v0;
    match m733.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f47(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f321(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f216(v2))
        }
    }
}

pub fn f217(p739: List<Bytes>) -> R<bool> {
    let m738 = p739;
    {
        let mut m738 = m738;
        loop {
            let step = {
                let v0 = m738;
                let m734 = v0;
                match m734.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a736 = {
                            let a735 = f167(v1)?;
                            length_bytes(a735)
                        };
                        let a737 = 0u64;
                        if nat_eq(a736, a737) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
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

pub fn f218(v0: List<Bytes>) -> R<bool> {
    if f228(f254(v0.clone())?) {
        Ok(f40({
            let a741 = {
                let a740 = f230(f254(v0)?);
                length_list(a740)
            };
            let a742 = 0u64;
            nat_eq(a741, a742)
        }))
    } else {
        Ok(false)
    }
}

pub fn f219(v0: List<Bytes>) -> R<bool> {
    if f228(f232(v0.clone(), f8(Str::lit("requirement")))?) {
        if f228(f232(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f40({
                let a744 = {
                    let a743 = f184(f230(f232(v0.clone(), f8(Str::lit("requirement")))?), f230(f232(v0, f8(Str::lit("admission")))?));
                    length_list(a743)
                };
                let a745 = 0u64;
                nat_eq(a744, a745)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f220() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f221(v0: Bytes) -> R<Bytes> {
    Ok(f34(f33(v0, 47u8)?))
}

pub fn f222() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f223(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m746 = v0;
    match m746.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f167(v1)?;
            let a748 = {
                let a747 = v3.clone();
                length_bytes(a747)
            };
            let a749 = 0u64;
            if nat_eq(a748, a749) {
                f223(v2)
            } else {
                let m750 = f77(v3.clone())?;
                match m750 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        if f322(f47(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m751 = f223(v2)?;
                            match m751 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f323(v5), {
                                        let a752 = v3;
                                        let a753 = f8(Str::lit("\u{a}"));
                                        append_bytes(a752, a753)
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

pub fn f224() -> Bytes {
    let a754 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a754)
}

pub fn f225(p759: List<Bytes>) -> R<Option<Bytes>> {
    let m758 = p759;
    {
        let mut m758 = m758;
        loop {
            let step = {
                let v0 = m758;
                let m755 = v0;
                match m755.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m756 = f77(f167(v1)?)?;
                        match m756 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m757 = f79(v3.clone(), f8(Str::lit("source")));
                                match m757 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f324(f47(v4.clone(), f8(Str::lit("redistribution"))), f47(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: Bytes) -> Bytes {
    let a764 = {
        let a760 = Str::lit("binding ");
        utf8_encode(a760)
    };
    let a765 = {
        let a762 = v0;
        let a763 = {
            let a761 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a761)
        };
        append_bytes(a762, a763)
    };
    append_bytes(a764, a765)
}

pub fn f227(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f228(v0: Option<List<Adt13>>) -> bool {
    let m766 = v0;
    match m766 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f229(v0: List<Bytes>) -> R<Option<List<Adt13>>> {
    let m767 = v0;
    match m767.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f167(v1)?;
            let a769 = {
                let a768 = v3.clone();
                length_bytes(a768)
            };
            let a770 = 0u64;
            if nat_eq(a769, a770) {
                f229(v2)
            } else {
                let m771 = f77(v3)?;
                match m771 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v4) => {
                        if f227(f227(f181(v4.clone(), f8(Str::lit("authority"))), f181(v4.clone(), f8(Str::lit("row")))), f227(f227(f181(v4.clone(), f8(Str::lit("construct"))), f181(v4.clone(), f8(Str::lit("artifact")))), f72(f47(v4.clone(), f8(Str::lit("reader"))), f8(Str::lit("read-"))))) {
                            let m772 = f229(v2)?;
                            match m772 {
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

pub fn f230(v0: Option<List<Adt13>>) -> List<Adt13> {
    let m773 = v0;
    match m773 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f231(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m774 = v0;
    match m774.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f167(v1)?;
            let a776 = {
                let a775 = v3.clone();
                length_bytes(a775)
            };
            let a777 = 0u64;
            if nat_eq(a776, a777) {
                f231(v2)
            } else {
                let m778 = f153(v3.clone())?;
                match m778 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f325(v4.clone()) {
                            let m779 = f231(v2)?;
                            match m779 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f204(), List::cons(f8(Str::lit("/")), List::cons(f326(f241(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a780 = v3;
                                        let a781 = f8(Str::lit("\u{a}"));
                                        append_bytes(a780, a781)
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

pub fn f232(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt13>>> {
    let m782 = v0;
    match m782.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f167(v2)?;
            let a784 = {
                let a783 = v4.clone();
                length_bytes(a783)
            };
            let a785 = 0u64;
            if nat_eq(a784, a785) {
                f232(v3, v1)
            } else {
                let m786 = f77(v4)?;
                match m786 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f327(v5.clone());
                        let a788 = {
                            let a787 = v6.clone();
                            length_bytes(a787)
                        };
                        let a789 = 0u64;
                        if nat_eq(a788, a789) {
                            Ok(None::<List<Adt13>>)
                        } else {
                            let m790 = f232(v3, v1.clone())?;
                            match m790 {
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

pub fn f233(p793: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m792 = p793;
    {
        let mut m792 = m792;
        loop {
            let step = {
                let v0 = m792;
                let m791 = v0;
                match m791.uncons() {
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
                Ok(next) => { m792 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f235(v0: Bytes, v1: List<Adt13>) -> R<(Bytes, Bytes)> {
    Ok((v0, f86(Adt13::C3(std::rc::Rc::new(f87(v1))))?))
}

pub fn f236(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f86(Adt13::C3(std::rc::Rc::new(f328(f118(f98(v1, f175(f82(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f237(v0: Adt1, v1: Bytes) -> R<Option<Adt17>> {
    f153(f53(v0, v1))
}

pub fn f238(p799: Adt17, v1: Bytes) -> Option<Adt17> {
    let m798 = p799;
    {
        let mut m798 = m798;
        loop {
            let step = {
                let v0 = m798;
                let m794 = v0;
                match m794 {
                    Adt17::C0 => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C1(_, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C3(_, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C4(_, _, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C5(_) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C6(_) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C7(r795) => {
                        let v5 = (*r795).clone();
                        Ok::<Adt17, Option<Adt17>>(v5)
                    }
                    Adt17::C8 => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C9(_, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C10(v2, r796, r797) => {
                        let v3 = (*r796).clone();
                        let v4 = (*r797).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt17, Option<Adt17>>(Some(v3))
                        } else {
                            Ok::<Adt17, Option<Adt17>>(v4)
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

pub fn f239(v0: Adt1, v1: Bytes, v2: Adt17, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m800 = f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m800.uncons() {
        None => {
            let m802 = f233(f104(List::cons(Adt13::C4(std::rc::Rc::new({
                let m801 = f329(v4.clone());
                match m801 {
                    None => {
                        Adt13::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt13>::nil())), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m802 {
                None => {
                    f330(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f234(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f240(v0: Adt17) -> R<Bytes> {
    let a803 = f331(v0)?;
    let a804 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a803, a804))
}

pub fn f241(v0: Adt17, v1: Bytes) -> Bytes {
    let m805 = f332(f238(v0, v1));
    match m805 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f242(v0: List<Adt17>) -> List<Bytes> {
    let m806 = v0;
    match m806.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m807 = f332(Some(v1));
            match m807 {
                None => {
                    f242(v2)
                }
                Some(v3) => {
                    List::cons(v3, f242(v2))
                }
            }
        }
    }
}

pub fn f243(v0: Option<Adt17>) -> List<Adt17> {
    let m808 = v0;
    match m808 {
        None => {
            List::<Adt17>::nil()
        }
        Some(v1) => {
            f333(v1)
        }
    }
}

pub fn f244(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f245(v0: Bytes) -> R<Bytes> {
    Ok(f334(f335(v0)?))
}

pub fn f246(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: List<Bytes>, v5: Bytes) -> R<Adt11> {
    let v6: Bytes = {
        let a809 = f8(Str::lit("registry-read "));
        let a810 = v5;
        append_bytes(a809, a810)
    };
    if f40(f41(v6.clone(), match v0.clone() {
        Adt1::C0(_, _, h811) => {
            h811
        }
    })) {
        Ok(Adt11::C2(f336(), v4, v6))
    } else {
        let m812 = f45(v0.clone(), List::cons(f269(), List::<Bytes>::nil()));
        match m812.uncons() {
            None => {
                f337(v0, v1, f269(), v2, v3, v4)
            }
            Some((v7, v8)) => {
                Ok(Adt11::C0(List::cons(v7, v8)))
            }
        }
    }
}

pub fn f247() -> Bytes {
    f338()
}

pub fn f248(v0: Adt1, v1: Adt13, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f252(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f339(v1, v4)?)?), List::cons(f236(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f249(v0: List<List<Bytes>>) -> List<Bytes> {
    let m813 = v0;
    match m813.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f250() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f251(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f302(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f252(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f340(f122(f81(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f341(f342(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f253(p816: List<Adt13>) -> bool {
    let m815 = p816;
    {
        let mut m815 = m815;
        loop {
            let step = {
                let v0 = m815;
                let m814 = v0;
                match m814.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f28(f47(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f41(f8(Str::lit("studio")), f115(f343(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f254(v0: List<Bytes>) -> R<Option<List<Adt13>>> {
    let m817 = v0;
    match m817.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f167(v1)?;
            let a819 = {
                let a818 = v3.clone();
                length_bytes(a818)
            };
            let a820 = 0u64;
            if nat_eq(a819, a820) {
                f254(v2)
            } else {
                let m821 = f77(v3)?;
                match m821 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v4) => {
                        if if f181(v4.clone(), f8(Str::lit("agent"))) {
                            if f181(v4.clone(), f8(Str::lit("at"))) {
                                if f181(v4.clone(), f8(Str::lit("quote"))) {
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
                            let m822 = f254(v2)?;
                            match m822 {
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

pub fn f255(v0: u64) -> bool {
    let a823 = v0;
    let a824 = 127u64;
    nat_le(a823, a824)
}

pub fn f256(v0: Bytes) -> R<Bytes> {
    f345(v0.clone(), 0u64, {
        let a825 = v0;
        length_bytes(a825)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f257(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt17, u64)>> {
    let m826 = v2;
    if m826 == 0 {
        Ok(None::<(Adt17, u64)>)
    } else {
        let v4 = m826 - 1;
        let v5: u64 = f346(v0.clone(), v1)?;
        let a827 = v3;
        let a828 = 1u64;
        if nat_eq(a827, a828) {
            let m829 = f257(v0.clone(), v1, v4, 0u64)?;
            match m829 {
                None => {
                    Ok(None::<(Adt17, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f346(v0.clone(), {
                        let (_, h830) = v6.clone();
                        h830
                    })?;
                    let a831 = v7;
                    let a832 = 44u64;
                    if nat_eq(a831, a832) {
                        let m836 = f257(v0, {
                            let a834 = {
                                let (_, h833) = v6;
                                h833
                            };
                            let a835 = 1u64;
                            nat_add(a834, a835)?
                        }, v4, 1u64)?;
                        match m836 {
                            None => {
                                Ok(None::<(Adt17, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt17::C9(std::rc::Rc::new({
                                    let (h837, _) = v6;
                                    h837
                                }), std::rc::Rc::new({
                                    let (h838, _) = v8;
                                    h838
                                })), {
                                    let (_, h839) = v8;
                                    h839
                                })))
                            }
                        }
                    } else {
                        let a840 = v7;
                        let a841 = 93u64;
                        if nat_eq(a840, a841) {
                            Ok(Some((Adt17::C9(std::rc::Rc::new({
                                let (h842, _) = v6;
                                h842
                            }), std::rc::Rc::new(Adt17::C8)), {
                                let a844 = {
                                    let (_, h843) = v6;
                                    h843
                                };
                                let a845 = 1u64;
                                nat_add(a844, a845)?
                            })))
                        } else {
                            Ok(None::<(Adt17, u64)>)
                        }
                    }
                }
            }
        } else {
            let a846 = v3;
            let a847 = 2u64;
            if nat_eq(a846, a847) {
                let m850 = f347(v0.clone(), {
                    let a848 = v1;
                    let a849 = 1u64;
                    nat_add(a848, a849)?
                })?;
                match m850 {
                    None => {
                        Ok(None::<(Adt17, u64)>)
                    }
                    Some(v9) => {
                        if f227({
                            let a851 = v5;
                            let a852 = 34u64;
                            nat_eq(a851, a852)
                        }, {
                            let a854 = f346(v0.clone(), {
                                let (h853, _) = v9.clone();
                                h853
                            })?;
                            let a855 = 58u64;
                            nat_eq(a854, a855)
                        }) {
                            let m859 = f257(v0.clone(), {
                                let a857 = {
                                    let (h856, _) = v9;
                                    h856
                                };
                                let a858 = 1u64;
                                nat_add(a857, a858)?
                            }, v4, 0u64)?;
                            match m859 {
                                None => {
                                    Ok(None::<(Adt17, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f346(v0.clone(), {
                                        let (_, h860) = v10.clone();
                                        h860
                                    })?;
                                    let a861 = v11;
                                    let a862 = 44u64;
                                    if nat_eq(a861, a862) {
                                        let m866 = f257(v0, {
                                            let a864 = {
                                                let (_, h863) = v10;
                                                h863
                                            };
                                            let a865 = 1u64;
                                            nat_add(a864, a865)?
                                        }, v4, 2u64)?;
                                        match m866 {
                                            None => {
                                                Ok(None::<(Adt17, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt17::C10({
                                                    let (_, h867) = v9;
                                                    h867
                                                }, std::rc::Rc::new({
                                                    let (h868, _) = v10;
                                                    h868
                                                }), std::rc::Rc::new({
                                                    let (h869, _) = v12;
                                                    h869
                                                })), {
                                                    let (_, h870) = v12;
                                                    h870
                                                })))
                                            }
                                        }
                                    } else {
                                        let a871 = v11;
                                        let a872 = 125u64;
                                        if nat_eq(a871, a872) {
                                            Ok(Some((Adt17::C10({
                                                let (_, h873) = v9;
                                                h873
                                            }, std::rc::Rc::new({
                                                let (h874, _) = v10;
                                                h874
                                            }), std::rc::Rc::new(Adt17::C8)), {
                                                let a876 = {
                                                    let (_, h875) = v10;
                                                    h875
                                                };
                                                let a877 = 1u64;
                                                nat_add(a876, a877)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt17, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt17, u64)>)
                        }
                    }
                }
            } else {
                let a878 = v5;
                let a879 = 34u64;
                if nat_eq(a878, a879) {
                    let m882 = f347(v0, {
                        let a880 = v1;
                        let a881 = 1u64;
                        nat_add(a880, a881)?
                    })?;
                    match m882 {
                        None => {
                            Ok(None::<(Adt17, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt17::C5({
                                let (_, h883) = v13;
                                h883
                            }), {
                                let (h884, _) = v13;
                                h884
                            })))
                        }
                    }
                } else {
                    if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt17::C0, {
                            let a885 = v1;
                            let a886 = 4u64;
                            nat_add(a885, a886)?
                        })))
                    } else {
                        if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt17::C2(true), {
                                let a887 = v1;
                                let a888 = 4u64;
                                nat_add(a887, a888)?
                            })))
                        } else {
                            if f28(f35(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt17::C2(false), {
                                    let a889 = v1;
                                    let a890 = 5u64;
                                    nat_add(a889, a890)?
                                })))
                            } else {
                                let a891 = v5;
                                let a892 = 91u64;
                                if nat_eq(a891, a892) {
                                    let a895 = f346(v0.clone(), {
                                        let a893 = v1;
                                        let a894 = 1u64;
                                        nat_add(a893, a894)?
                                    })?;
                                    let a896 = 93u64;
                                    if nat_eq(a895, a896) {
                                        Ok(Some((Adt17::C6(std::rc::Rc::new(Adt17::C8)), {
                                            let a897 = v1;
                                            let a898 = 2u64;
                                            nat_add(a897, a898)?
                                        })))
                                    } else {
                                        let m901 = f257(v0, {
                                            let a899 = v1;
                                            let a900 = 1u64;
                                            nat_add(a899, a900)?
                                        }, v4, 1u64)?;
                                        match m901 {
                                            None => {
                                                Ok(None::<(Adt17, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt17::C6(std::rc::Rc::new({
                                                    let (h902, _) = v14;
                                                    h902
                                                })), {
                                                    let (_, h903) = v14;
                                                    h903
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a904 = v5;
                                    let a905 = 123u64;
                                    if nat_eq(a904, a905) {
                                        let a908 = f346(v0.clone(), {
                                            let a906 = v1;
                                            let a907 = 1u64;
                                            nat_add(a906, a907)?
                                        })?;
                                        let a909 = 125u64;
                                        if nat_eq(a908, a909) {
                                            Ok(Some((Adt17::C7(std::rc::Rc::new(Adt17::C8)), {
                                                let a910 = v1;
                                                let a911 = 2u64;
                                                nat_add(a910, a911)?
                                            })))
                                        } else {
                                            let m914 = f257(v0, {
                                                let a912 = v1;
                                                let a913 = 1u64;
                                                nat_add(a912, a913)?
                                            }, v4, 2u64)?;
                                            match m914 {
                                                None => {
                                                    Ok(None::<(Adt17, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt17::C7(std::rc::Rc::new({
                                                        let (h915, _) = v15;
                                                        h915
                                                    })), {
                                                        let (_, h916) = v15;
                                                        h916
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m917 = f348(v0, v1)?;
                                        match m917 {
                                            None => {
                                                Ok(None::<(Adt17, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h918) = v16;
                                                    h918
                                                }, {
                                                    let (h919, _) = v16;
                                                    h919
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

pub fn f258(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h920, _) = v1.clone();
        h920
    };
    let v3: u64 = {
        let (h921, _) = {
            let (_, h922) = v1.clone();
            h922
        };
        h921
    };
    let v4: List<Bytes> = {
        let (_, h923) = {
            let (_, h924) = v1;
            h924
        };
        h923
    };
    let m925 = f168(v0.clone(), v2);
    match m925 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a926 = v5;
            let a927 = 10u8;
            if equal(a926, a927) {
                Ok(Some(({
                    let a928 = v2;
                    let a929 = 1u64;
                    nat_add(a928, a929)?
                }, ({
                    let a930 = v2;
                    let a931 = 1u64;
                    nat_add(a930, a931)?
                }, List::cons(f349(f35(v0, v3, {
                    let a932 = v2;
                    let a933 = v3;
                    nat_sub(a932, a933)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a934 = v2;
                    let a935 = 1u64;
                    nat_add(a934, a935)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f259(v0: Adt13) -> List<Adt13> {
    let m936 = v0;
    match m936 {
        Adt13::C0(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C1(_, _) => {
            List::<Adt13>::nil()
        }
        Adt13::C2(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C3(r937) => {
            let v3 = (*r937).clone();
            f259(v3)
        }
        Adt13::C4(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C5 => {
            List::<Adt13>::nil()
        }
        Adt13::C6(r938, r939) => {
            let v1 = (*r938).clone();
            let v2 = (*r939).clone();
            List::cons(v1, f259(v2))
        }
        Adt13::C7(_, _, _) => {
            List::<Adt13>::nil()
        }
    }
}

pub fn f260(v0: Adt13, v1: Bytes) -> Adt13 {
    let m940 = v0;
    match m940 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r941) => {
            let v10 = (*r941).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r942) => {
            let v5 = (*r942).clone();
            Adt13::C4(std::rc::Rc::new(f260(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r943, r944) => {
            let v11 = (*r943).clone();
            let v12 = (*r944).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r945, r946) => {
            let v3 = (*r945).clone();
            let v4 = (*r946).clone();
            if f28(v2.clone(), f8(Str::lit("authority"))) {
                Adt13::C7(v2, std::rc::Rc::new(f350(v3, v1.clone())), std::rc::Rc::new(f260(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f260(v4, v1)))
            }
        }
    }
}

pub fn f261(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a947 = 0u64;
        let a948 = v1;
        nat_lt(a947, a948)
    } else {
        false
    } {
        let a949 = Bytes::lit(&[0x2d]);
        let a950 = f351(v1)?;
        Ok(append_bytes(a949, a950))
    } else {
        f351(v1)
    }
}

pub fn f262(v0: Bytes) -> R<Bytes> {
    let a956 = {
        let a954 = Bytes::lit(&[0x22]);
        let a955 = {
            let (_, h951) = {
                let (h952, _) = f352(Fn3::F353(v0.clone()), {
                    let a953 = v0;
                    length_bytes(a953)
                }, (0u64, Bytes::lit(&[])))?;
                h952
            };
            h951
        };
        append_bytes(a954, a955)
    };
    let a957 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a956, a957))
}

pub fn f263(v0: List<Bytes>) -> Bytes {
    let a960 = {
        let a958 = Bytes::lit(&[0x5b]);
        let a959 = f95(v0, Bytes::lit(&[0x2c]));
        append_bytes(a958, a959)
    };
    let a961 = Bytes::lit(&[0x5d]);
    append_bytes(a960, a961)
}

pub fn f264(v0: Adt13) -> R<List<Bytes>> {
    let m962 = v0;
    match m962 {
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
        Adt13::C6(r963, r964) => {
            let v7 = (*r963).clone();
            let v8 = (*r964).clone();
            Ok(List::cons(f159(v7)?, f264(v8)?))
        }
        Adt13::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f265(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f354(Fn4::F355, List::<(Bytes, Bytes)>::nil(), f356(v0)?);
    let v2: List<Bytes> = f357(Fn5::F358, List::<Bytes>::nil(), v1)?;
    let a967 = {
        let a965 = Bytes::lit(&[0x7b]);
        let a966 = f95(v2, Bytes::lit(&[0x2c]));
        append_bytes(a965, a966)
    };
    let a968 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a967, a968))
}

pub fn f266(v0: Adt13) -> R<List<(Bytes, Bytes)>> {
    let m969 = v0;
    match m969 {
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
        Adt13::C7(v9, r970, r971) => {
            let v10 = (*r970).clone();
            let v11 = (*r971).clone();
            Ok(List::cons((v9, f159(v10)?), f266(v11)?))
        }
    }
}

pub fn f267() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f268(v0: Adt12) -> Bytes {
    let m972 = v0;
    match m972 {
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

pub fn f269() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f270(v0: Adt12, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f359(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f359(f8(Str::lit("id")), f58(v0.clone(), f50(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f359(f8(Str::lit("name")), f360(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f359(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f359(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f359(f8(Str::lit("statement")), f361(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f359(f8(Str::lit("media_type")), f362(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f359(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f359(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f359(f8(Str::lit("url")), f162(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f271(v0: Bytes) -> R<u64> {
    let (h973, _) = f363(Fn6::F364(v0.clone()), {
        let a974 = v0.clone();
        length_bytes(a974)
    }, {
        let a975 = v0;
        length_bytes(a975)
    })?;
    Ok(h973)
}

pub fn f272(v0: Bytes, v1: u64) -> R<u64> {
    let (h976, _) = f363(Fn6::F365(v1, v0), v1, 0u64)?;
    Ok(h976)
}

pub fn f273(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m977 = v0;
    match m977.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m978 = f79(v1, f8(Str::lit("std")));
            match m978 {
                None => {
                    f273(v2)
                }
                Some(v3) => {
                    if f181(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f47(v3.clone(), f8(Str::lit("row"))), f47(v3, f8(Str::lit("authority")))), f273(v2))
                    } else {
                        f273(v2)
                    }
                }
            }
        }
    }
}

pub fn f274(v0: Adt14, v1: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m979 = v1;
    match m979.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f47(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f47(v2, f8(Str::lit("object")));
            if if f41(v4.clone(), match v0.clone() {
                Adt14::C0(h980, _, _) => {
                    h980
                }
            }) {
                f41(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h981, _) => {
                        h981
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f274(v0, v3))
            } else {
                if if f41(v5.clone(), match v0.clone() {
                    Adt14::C0(h982, _, _) => {
                        h982
                    }
                }) {
                    f41(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h983, _) => {
                            h983
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f274(v0, v3))
                } else {
                    f274(v0, v3)
                }
            }
        }
    }
}

pub fn f275(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt18>> {
    let m984 = v0;
    match m984.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v4, v5)) => {
            if if f366(v4.clone(), v2.clone()) {
                true
            } else {
                f366(v4.clone(), v3.clone())
            } {
                f275(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt18::C0({
                    let (h985, _) = v4.clone();
                    h985
                }, {
                    let a988 = {
                        let a987 = f118(f367({
                            let (h986, _) = v4.clone();
                            h986
                        }, v1.clone()))?;
                        length_list(a987)
                    };
                    let a989 = 1u64;
                    if nat_eq(a988, a989) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h990) = v4.clone();
                    h990
                }), f275(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f276(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m991 = v1;
    match m991.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41({
                let (h992, _) = v2.clone();
                h992
            }, v0.clone()) {
                List::cons({
                    let (_, h993) = v2;
                    h993
                }, f276(v0, v3))
            } else {
                if f41({
                    let (_, h994) = v2;
                    h994
                }, v0.clone()) {
                    List::cons({
                        let (h995, _) = v2;
                        h995
                    }, f276(v0, v3))
                } else {
                    f276(v0, v3)
                }
            }
        }
    }
}

pub fn f277(v0: Adt14, v1: Adt13, v2: Bytes) -> bool {
    if f41(v2.clone(), match v0.clone() {
        Adt14::C0(h996, _, _) => {
            h996
        }
    }) {
        true
    } else {
        if f41(v2.clone(), match v0.clone() {
            Adt14::C0(_, h997, _) => {
                h997
            }
        }) {
            true
        } else {
            if f41(v2, match v0.clone() {
                Adt14::C0(_, _, h998) => {
                    h998
                }
            }) {
                true
            } else {
                if f181(v1.clone(), f8(Str::lit("scheme"))) {
                    f41(f47(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h999, _) => {
                            h999
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f278(v0: Adt13) -> bool {
    if f181(v0.clone(), f8(Str::lit("domain"))) {
        if f181(v0.clone(), f8(Str::lit("row"))) {
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

pub fn f279(v0: Adt13) -> bool {
    if f181(v0.clone(), f8(Str::lit("subject"))) {
        if f181(v0.clone(), f8(Str::lit("object"))) {
            if f181(v0.clone(), f8(Str::lit("evidence"))) {
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

pub fn f280(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1000 = f153(v1)?;
    match m1000 {
        None => {
            Ok(f187(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f242(f243(f238(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1001 = f332(f238(v4.clone(), f8(Str::lit("construct"))));
                match m1001 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1002 = v5;
            match m1002.uncons() {
                None => {
                    Ok(f187(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1003 = v9;
                    match m1003.uncons() {
                        None => {
                            Ok(f187(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f40(f28(v8, f8(Str::lit("std")))) {
                                Ok(f187(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f40(f41(v10.clone(), v2)) {
                                    Ok(f187(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f227(f40(f28(v7.clone(), f8(Str::lit("none")))), f40(f41(v7.clone(), v3))) {
                                        Ok(f187(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1004 = f332(f238(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1004 {
                                            None => {
                                                Ok(f187(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1005 = f238(v4, f8(Str::lit("value")));
                                                match m1005 {
                                                    None => {
                                                        Ok(f187(v0, f8(Str::lit("a row has no value"))))
                                                    }
                                                    Some(_) => {
                                                        Ok(List::<Bytes>::nil())
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

pub fn f281(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1006 = v1;
    match m1006.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f371(v3.clone())
            } else {
                f40(f371(v3.clone()))
            } {
                List::cons(f201(v0.clone(), v3), f281(v0, v4, v2))
            } else {
                f281(v0, v4, v2)
            }
        }
    }
}

pub fn f282(p1009: List<Bytes>) -> R<Option<Bytes>> {
    let m1008 = p1009;
    {
        let mut m1008 = m1008;
        loop {
            let step = {
                let v0 = m1008;
                let m1007 = v0;
                match m1007.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f372(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f167(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1008 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1010 = v1;
    match m1010.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f98(f81(f53(v0.clone(), v2))?, f283(v0, v3)?))
        }
    }
}

pub fn f284(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1011 = v0.clone();
        length_bytes(a1011)
    };
    let v3: u64 = {
        let a1012 = v1.clone();
        length_bytes(a1012)
    };
    let v4: u64 = {
        let (h1013, _) = f363(Fn6::F373(v1.clone(), v3, v0.clone(), v2), {
            let a1014 = v2;
            let a1015 = 1u64;
            nat_add(a1014, a1015)?
        }, 0u64)?;
        h1013
    };
    if f374(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f285() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f286(v0: Adt15) -> R<Adt19> {
    Ok(Adt19::C0(f375(match v0.clone() {
        Adt15::C0(h1016, _, _, _, _) => {
            h1016
        }
    }, 0u64)?, f375(match v0.clone() {
        Adt15::C0(_, h1017, _, _, _) => {
            h1017
        }
    }, 0u64)?, f375(match v0.clone() {
        Adt15::C0(_, _, h1018, _, _) => {
            h1018
        }
    }, 0u64)?, f375(match v0 {
        Adt15::C0(_, _, _, h1019, _) => {
            h1019
        }
    }, 0u64)?))
}

pub fn f287(v0: Adt15) -> List<Bytes> {
    f98(match v0.clone() {
        Adt15::C0(h1020, _, _, _, _) => {
            h1020
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, h1021, _, _) => {
            h1021
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, _, h1022, _) => {
            h1022
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1023) => {
            h1023
        }
    })))
}

pub fn f288(v0: Adt19, p1029: List<u64>, p1030: List<Adt21>) -> Option<u64> {
    let m1028 = (p1029, p1030);
    {
        let mut m1028 = m1028;
        loop {
            let step = {
                let (v1, v2) = m1028;
                let m1024 = v2;
                match m1024.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt21>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f376(v0.clone(), v1.clone(), match v3.clone() {
                            Adt21::C0(_, h1025) => {
                                h1025
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt21>), Option<u64>>((List::cons(match v3 {
                                Adt21::C0(h1026, _) => {
                                    h1026
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt21>), Option<u64>>(Some(match v3 {
                                Adt21::C0(h1027, _) => {
                                    h1027
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1028 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f289(v0: Adt15, v1: List<Adt13>, v2: u64) -> R<List<Adt21>> {
    let m1031 = v1;
    match m1031.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt21::C0(v2, f377(v0.clone(), v3)?), f289(v0, v4, {
                let a1032 = v2;
                let a1033 = 1u64;
                nat_add(a1032, a1033)?
            })?))
        }
    }
}

pub fn f290(p1037: List<Bytes>, p1038: u64) -> Bytes {
    let m1036 = (p1037, p1038);
    {
        let mut m1036 = m1036;
        loop {
            let step = {
                let (v0, v1) = m1036;
                let m1034 = v0;
                match m1034.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1035 = v1;
                        if m1035 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1035 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1036 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f378(v0.clone(), v1.clone()) {
        if f378(v1, v0) {
            f379(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f292(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1039 = v0;
    match m1039.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f380(v2, v1.clone())?, f292(v3, v1)?))
        }
    }
}

pub fn f293(v0: List<Adt13>) -> List<u64> {
    let m1040 = v0;
    match m1040.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f381(v1), f293(v2))
        }
    }
}

pub fn f294(p1043: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1042 = p1043;
    {
        let mut m1042 = m1042;
        loop {
            let step = {
                let v0 = m1042;
                let m1041 = v0;
                match m1041.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f382(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1042 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(p1046: List<Bytes>, p1047: List<Bytes>) -> Option<Bytes> {
    let m1045 = (p1046, p1047);
    {
        let mut m1045 = m1045;
        loop {
            let step = {
                let (v0, v1) = m1045;
                let m1044 = v0;
                match m1044.uncons() {
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
                Ok(next) => { m1045 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f297(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1048 = v0.clone();
    match m1048.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1049 = v2;
            let a1050 = v1.clone();
            if equal(a1049, a1050) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f298(v0: List<(Bytes, ())>) -> List<Bytes> {
    f197(f383(Fn7::F384, List::<Bytes>::nil(), v0))
}

pub fn f299(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f385(f386(Fn8::F387, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1055 = {
        let (h1051, _) = f388(Fn9::F389, {
            let a1053 = {
                let a1052 = v0;
                length_list(a1052)
            };
            let a1054 = 1u64;
            nat_add(a1053, a1054)?
        }, v1)?;
        h1051
    };
    match m1055.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f300(v0: List<Bytes>) -> List<(Bytes, ())> {
    f390(f391(Fn10::F392, List::<(Bytes, ())>::nil(), v0))
}

pub fn f301(v0: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1056 = f33(v1, 34u8)?;
    match m1056.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1057 = v3;
            match m1057.uncons() {
                None => {
                    f393(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f302(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f394(v0.clone(), 0u64, {
        let a1058 = v0;
        length_bytes(a1058)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f303(v0: Adt13, v1: Bytes) -> Bytes {
    if f28(f395(f79(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f304(v0: Adt1, v1: Bytes) -> R<Adt17> {
    let m1059 = f237(v0, v1)?;
    match m1059 {
        None => {
            Ok(Adt17::C8)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f305(v0: Bytes) -> R<Bytes> {
    let a1060 = f8(Str::lit("Catalog."));
    let a1061 = f326(v0)?;
    Ok(append_bytes(a1060, a1061))
}

pub fn f306(v0: Adt17) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f326(f241(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f307(v0: List<Bytes>) -> List<Bytes> {
    let m1062 = v0;
    match m1062.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f200(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f201(f220(), v1), f307(v2))
            } else {
                f307(v2)
            }
        }
    }
}

pub fn f308(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1063 = v1;
    match m1063.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f98(f396(f397(f81(f53(v0.clone(), v2))?)?), f308(v0, v3)?))
        }
    }
}

pub fn f309(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1064 = v1;
    match m1064.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f398(v0.clone(), v3, v2.clone())?;
            let (h1065, _) = v5.clone();
            if h1065 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f309(v0, v4, v2)?;
                let (h1066, _) = v6.clone();
                if h1066 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1069 = {
                            let (_, h1067) = v5;
                            h1067
                        };
                        let a1070 = {
                            let (_, h1068) = v6;
                            h1068
                        };
                        append_bytes(a1069, a1070)
                    }))
                }
            }
        }
    }
}

pub fn f310(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m1071 = v1;
    match m1071.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f181(v2.clone(), f8(Str::lit("subject"))) {
                if f182(v0.clone(), v2.clone()) {
                    f310(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f47(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f310(v0, v3))
                }
            } else {
                if f319(v0.clone(), v2.clone()) {
                    f310(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f320(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f310(v0, v3))
                }
            }
        }
    }
}

pub fn f311(v0: Adt1) -> R<Bytes> {
    let a1073 = {
        let a1072 = f101(f102(f213(f210(f211(f53(v0.clone(), f173()))?)), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(f82(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1072)
    };
    let a1074 = 0u64;
    if nat_eq(a1073, a1074) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f312(v0: List<Adt13>) -> R<List<Bytes>> {
    let m1075 = v0;
    match m1075.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f95(f175(f157(f79(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f47(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f47(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f399(f81(f47(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f312(v2)?))
        }
    }
}

pub fn f313(v0: Bytes, p1133: u64, p1134: u64, p1135: Adt20) -> R<Adt20> {
    let m1132 = (p1133, (p1134, p1135));
    {
        let mut m1132 = m1132;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1132;
                let m1076 = v2;
                if m1076 == 0 {
                    Err::<(u64, (u64, Adt20)), R<Adt20>>(Ok(v3))
                } else {
                    let v4 = m1076 - 1;
                    let v5: u64 = f400(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt20::C0(_, h1077, _, _, _) => {
                            h1077
                        }
                    } {
                        Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                            let a1078 = v1;
                            let a1079 = 1u64;
                            nat_add(a1078, a1079)?
                        }, (v4, Adt20::C0(true, false, match v3.clone() {
                            Adt20::C0(_, _, h1080, _, _) => {
                                h1080
                            }
                        }, match v3.clone() {
                            Adt20::C0(_, _, _, h1081, _) => {
                                h1081
                            }
                        }, match v3 {
                            Adt20::C0(_, _, _, _, h1082) => {
                                h1082
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt20::C0(h1083, _, _, _, _) => {
                                h1083
                            }
                        } {
                            let a1084 = v5;
                            let a1085 = 34u64;
                            if nat_eq(a1084, a1085) {
                                let a1088 = f400(v0.clone(), {
                                    let a1086 = v1;
                                    let a1087 = 1u64;
                                    nat_add(a1086, a1087)?
                                })?;
                                let a1089 = 34u64;
                                if nat_eq(a1088, a1089) {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1090 = v1;
                                        let a1091 = 1u64;
                                        nat_add(a1090, a1091)?
                                    }, (v4, Adt20::C0(true, true, f401(match v3.clone() {
                                        Adt20::C0(_, _, h1092, _, _) => {
                                            h1092
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt20::C0(_, _, _, h1093, _) => {
                                            h1093
                                        }
                                    }, match v3 {
                                        Adt20::C0(_, _, _, _, h1094) => {
                                            h1094
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1095 = v1;
                                        let a1096 = 1u64;
                                        nat_add(a1095, a1096)?
                                    }, (v4, Adt20::C0(false, false, match v3.clone() {
                                        Adt20::C0(_, _, h1097, _, _) => {
                                            h1097
                                        }
                                    }, match v3.clone() {
                                        Adt20::C0(_, _, _, h1098, _) => {
                                            h1098
                                        }
                                    }, match v3 {
                                        Adt20::C0(_, _, _, _, h1099) => {
                                            h1099
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                    let a1100 = v1;
                                    let a1101 = 1u64;
                                    nat_add(a1100, a1101)?
                                }, (v4, Adt20::C0(true, false, f401(match v3.clone() {
                                    Adt20::C0(_, _, h1102, _, _) => {
                                        h1102
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt20::C0(_, _, _, h1103, _) => {
                                        h1103
                                    }
                                }, match v3 {
                                    Adt20::C0(_, _, _, _, h1104) => {
                                        h1104
                                    }
                                }))))
                            }
                        } else {
                            let a1105 = v5;
                            let a1106 = 34u64;
                            if nat_eq(a1105, a1106) {
                                Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                    let a1107 = v1;
                                    let a1108 = 1u64;
                                    nat_add(a1107, a1108)?
                                }, (v4, Adt20::C0(true, false, match v3.clone() {
                                    Adt20::C0(_, _, h1109, _, _) => {
                                        h1109
                                    }
                                }, match v3.clone() {
                                    Adt20::C0(_, _, _, h1110, _) => {
                                        h1110
                                    }
                                }, match v3 {
                                    Adt20::C0(_, _, _, _, h1111) => {
                                        h1111
                                    }
                                }))))
                            } else {
                                let a1112 = v5;
                                let a1113 = 44u64;
                                if nat_eq(a1112, a1113) {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1114 = v1;
                                        let a1115 = 1u64;
                                        nat_add(a1114, a1115)?
                                    }, (v4, Adt20::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt20::C0(_, _, h1116, _, _) => {
                                            h1116
                                        }
                                    }, match v3.clone() {
                                        Adt20::C0(_, _, _, h1117, _) => {
                                            h1117
                                        }
                                    }), match v3 {
                                        Adt20::C0(_, _, _, _, h1118) => {
                                            h1118
                                        }
                                    }))))
                                } else {
                                    let a1119 = v5;
                                    let a1120 = 13u64;
                                    if nat_eq(a1119, a1120) {
                                        Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                            let a1121 = v1;
                                            let a1122 = 1u64;
                                            nat_add(a1121, a1122)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1123 = v5;
                                        let a1124 = 10u64;
                                        if nat_eq(a1123, a1124) {
                                            Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                                let a1125 = v1;
                                                let a1126 = 1u64;
                                                nat_add(a1125, a1126)?
                                            }, (v4, f314(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                                let a1127 = v1;
                                                let a1128 = 1u64;
                                                nat_add(a1127, a1128)?
                                            }, (v4, Adt20::C0(false, false, f401(match v3.clone() {
                                                Adt20::C0(_, _, h1129, _, _) => {
                                                    h1129
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt20::C0(_, _, _, h1130, _) => {
                                                    h1130
                                                }
                                            }, match v3 {
                                                Adt20::C0(_, _, _, _, h1131) => {
                                                    h1131
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
                Ok(next) => { m1132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f314(v0: Adt20) -> Adt20 {
    Adt20::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f402(List::cons(match v0.clone() {
        Adt20::C0(_, _, h1136, _, _) => {
            h1136
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1137, _) => {
            h1137
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt20::C0(_, _, _, _, h1138) => {
            h1138
        }
    }))
}

pub fn f315(p1141: List<List<Bytes>>, p1142: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1140 = (p1141, p1142);
    {
        let mut m1140 = m1140;
        loop {
            let step = {
                let (v0, v1) = m1140;
                let m1139 = v0;
                match m1139.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1140 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(v0: Bytes, v1: List<Adt18>) -> List<Bytes> {
    let m1143 = v1;
    match m1143.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(match v2.clone() {
                Adt18::C0(h1144, _, _) => {
                    h1144
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt18::C0(_, h1145, _) => {
                        h1145
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt18::C0(_, _, h1146) => {
                        h1146
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f316(v0, v3))
            } else {
                f316(v0, v3)
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: List<Adt13>) -> List<Bytes> {
    let m1147 = v1;
    match m1147.uncons() {
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

pub fn f318(p1152: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1151 = p1152;
    {
        let mut m1151 = m1151;
        loop {
            let step = {
                let v0 = m1151;
                let m1148 = v0;
                match m1148.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1149 = v2;
                        match m1149.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f28(v4, v1.clone()) {
                                    let m1150 = v5;
                                    match m1150.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f403(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1151 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: Adt14, v1: Adt13) -> bool {
    if f41(f47(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1153, _, _) => {
            h1153
        }
    }) {
        f41(f320(v1), match v0 {
            Adt14::C0(_, h1154, _) => {
                h1154
            }
        })
    } else {
        false
    }
}

pub fn f320(v0: Adt13) -> Bytes {
    let m1155 = f79(v0, f8(Str::lit("evidence")));
    match m1155 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f47(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f321(v0: Adt13) -> Bytes {
    if f181(v0.clone(), f8(Str::lit("req"))) {
        let a1156 = f8(Str::lit("req "));
        let a1157 = f47(v0, f8(Str::lit("req")));
        append_bytes(a1156, a1157)
    } else {
        let m1158 = f79(v0.clone(), f8(Str::lit("std")));
        match m1158 {
            None => {
                let m1161 = f79(v0, f8(Str::lit("der")));
                match m1161 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f47(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f95(f175(f157(f79(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f47(v1.clone(), f8(Str::lit("authority"))), List::cons(if f181(v1.clone(), f8(Str::lit("row"))) {
                    let a1159 = f8(Str::lit(" row "));
                    let a1160 = f47(v1, f8(Str::lit("row")));
                    append_bytes(a1159, a1160)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f322(v0: Bytes, v1: Bytes) -> bool {
    let a1162 = v0;
    let a1163 = f338();
    if equal(a1162, a1163) {
        let a1167 = {
            let a1165 = {
                let a1164 = v1;
                length_bytes(a1164)
            };
            let a1166 = 0u64;
            nat_eq(a1165, a1166)
        };
        bool_not(a1167)
    } else {
        false
    }
}

pub fn f323(v0: Bytes) -> Bytes {
    f7(List::cons(f117(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f324(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1169 = v0;
    let a1170 = {
        let a1168 = Str::lit("redistributable");
        utf8_encode(a1168)
    };
    if equal(a1169, a1170) {
        f404(v1)
    } else {
        Ok(false)
    }
}

pub fn f325(v0: Adt17) -> bool {
    if f405(v0.clone(), f8(Str::lit("name"))) {
        if f406(v0.clone(), f8(Str::lit("schema"))) {
            let m1171 = f329(f242(f243(f238(v0, f8(Str::lit("citation"))))));
            match m1171 {
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

pub fn f326(v0: Bytes) -> R<Bytes> {
    f407(v0.clone(), 0u64, {
        let a1172 = v0;
        length_bytes(a1172)
    }, true, Bytes::lit(&[]))
}

pub fn f327(v0: Adt13) -> Bytes {
    if f181(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f181(v0.clone(), f8(Str::lit("id"))) {
            if f181(v0.clone(), f8(Str::lit("statement"))) {
                if f181(v0.clone(), f8(Str::lit("stakeholder"))) {
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

pub fn f328(v0: List<Bytes>) -> Adt13 {
    let m1173 = v0;
    match m1173.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(f328(v2)))
        }
    }
}

pub fn f329(v0: List<Bytes>) -> Option<Adt13> {
    let m1174 = v0;
    match m1174.uncons() {
        None => {
            None::<Adt13>
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("req"))) {
                let m1175 = v2;
                match m1175.uncons() {
                    None => {
                        None::<Adt13>
                    }
                    Some((v3, _)) => {
                        Some(Adt13::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5)))
                    }
                }
            } else {
                if f28(v1.clone(), f8(Str::lit("std"))) {
                    let m1176 = v2;
                    match m1176.uncons() {
                        None => {
                            None::<Adt13>
                        }
                        Some((v5, v6)) => {
                            Some(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v5)), std::rc::Rc::new({
                                let m1177 = v6;
                                match m1177.uncons() {
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
                        let m1178 = v2;
                        match m1178.uncons() {
                            None => {
                                None::<Adt13>
                            }
                            Some((v9, v10)) => {
                                let m1179 = v10;
                                match m1179.uncons() {
                                    None => {
                                        None::<Adt13>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt13::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt13::C2(v11)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f328(v12)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt13::C2(v9)), std::rc::Rc::new(Adt13::C5))))))))), std::rc::Rc::new(Adt13::C5)))
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

pub fn f330(v0: Adt1, v1: Bytes, v2: Adt17, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1180 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1180.uncons() {
        None => {
            let m1181 = f329(v4);
            match m1181 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1182 = f8(Str::lit("Catalog."));
                        let a1183 = f326(v3.clone())?;
                        append_bytes(a1182, a1183)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f326(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1184 = f408(v2, v9.clone(), f326(v3.clone())?)?;
                    match m1184 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f326(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f409(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f409(f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt13::C2(v10.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt13::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt13::C1(false, 0u64)), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1188 = f410(f53(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f35(v10.clone(), 4u64, {
                                    let a1186 = {
                                        let a1185 = v10;
                                        length_bytes(a1185)
                                    };
                                    let a1187 = 4u64;
                                    nat_sub(a1186, a1187)
                                }))?;
                                let a1189 = f8(Str::lit("\u{a}"));
                                append_bytes(a1188, a1189)
                            }), List::cons(f236(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f331(v0: Adt17) -> R<Bytes> {
    let m1190 = v0;
    match m1190 {
        Adt17::C0 => {
            Ok(Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
        }
        Adt17::C1(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C2(v3) => {
            if v3 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt17::C3(v4, v5) => {
            f261(v4, v5)
        }
        Adt17::C4(v6, v7, v8) => {
            f411(v6, v7, v8)
        }
        Adt17::C5(v9) => {
            f412(v9)
        }
        Adt17::C6(r1191) => {
            let v10 = (*r1191).clone();
            Ok(f263(f413(v10)?))
        }
        Adt17::C7(r1192) => {
            let v11 = (*r1192).clone();
            f265(f414(v11)?)
        }
        Adt17::C8 => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C9(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C10(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f332(v0: Option<Adt17>) -> Option<Bytes> {
    let m1193 = v0;
    match m1193 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1194 = v1;
            match m1194 {
                Adt17::C0 => {
                    None::<Bytes>
                }
                Adt17::C1(_, _) => {
                    None::<Bytes>
                }
                Adt17::C2(_) => {
                    None::<Bytes>
                }
                Adt17::C3(_, _) => {
                    None::<Bytes>
                }
                Adt17::C4(_, _, _) => {
                    None::<Bytes>
                }
                Adt17::C5(v2) => {
                    Some(v2)
                }
                Adt17::C6(_) => {
                    None::<Bytes>
                }
                Adt17::C7(_) => {
                    None::<Bytes>
                }
                Adt17::C8 => {
                    None::<Bytes>
                }
                Adt17::C9(_, _) => {
                    None::<Bytes>
                }
                Adt17::C10(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f333(v0: Adt17) -> List<Adt17> {
    let m1195 = v0;
    match m1195 {
        Adt17::C0 => {
            List::<Adt17>::nil()
        }
        Adt17::C1(_, _) => {
            List::<Adt17>::nil()
        }
        Adt17::C2(_) => {
            List::<Adt17>::nil()
        }
        Adt17::C3(_, _) => {
            List::<Adt17>::nil()
        }
        Adt17::C4(_, _, _) => {
            List::<Adt17>::nil()
        }
        Adt17::C5(_) => {
            List::<Adt17>::nil()
        }
        Adt17::C6(r1196) => {
            let v3 = (*r1196).clone();
            f333(v3)
        }
        Adt17::C7(_) => {
            List::<Adt17>::nil()
        }
        Adt17::C8 => {
            List::<Adt17>::nil()
        }
        Adt17::C9(r1197, r1198) => {
            let v1 = (*r1197).clone();
            let v2 = (*r1198).clone();
            List::cons(v1, f333(v2))
        }
        Adt17::C10(_, _, _) => {
            List::<Adt17>::nil()
        }
    }
}

pub fn f334(v0: Adt24) -> Bytes {
    let a1219 = f415(match v0.clone() {
        Adt24::C0(h1199, _, _, _, _, _, _, _) => {
            h1199
        }
    });
    let a1220 = {
        let a1217 = f415(match v0.clone() {
            Adt24::C0(_, h1200, _, _, _, _, _, _) => {
                h1200
            }
        });
        let a1218 = {
            let a1215 = f415(match v0.clone() {
                Adt24::C0(_, _, h1201, _, _, _, _, _) => {
                    h1201
                }
            });
            let a1216 = {
                let a1213 = f415(match v0.clone() {
                    Adt24::C0(_, _, _, h1202, _, _, _, _) => {
                        h1202
                    }
                });
                let a1214 = {
                    let a1211 = f415(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1203, _, _, _) => {
                            h1203
                        }
                    });
                    let a1212 = {
                        let a1209 = f415(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1204, _, _) => {
                                h1204
                            }
                        });
                        let a1210 = {
                            let a1207 = f415(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1205, _) => {
                                    h1205
                                }
                            });
                            let a1208 = f415(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1206) => {
                                    h1206
                                }
                            });
                            append_bytes(a1207, a1208)
                        };
                        append_bytes(a1209, a1210)
                    };
                    append_bytes(a1211, a1212)
                };
                append_bytes(a1213, a1214)
            };
            append_bytes(a1215, a1216)
        };
        append_bytes(a1217, a1218)
    };
    append_bytes(a1219, a1220)
}

pub fn f335(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1221 = v0.clone();
        length_bytes(a1221)
    };
    let v2: u64 = {
        let a1227 = {
            let a1224 = {
                let a1222 = v1;
                let a1223 = 72u64;
                nat_add(a1222, a1223)?
            };
            let a1225 = 64u64;
            let a1226 = 0u64;
            nat_quot(a1224, a1225, a1226)
        };
        let a1228 = 64u64;
        nat_mul(a1227, a1228)?
    };
    let (_, h1229) = f416(Fn11::F417(v0, v1, v2), {
        let a1230 = v2;
        let a1231 = 64u64;
        let a1232 = 0u64;
        nat_quot(a1230, a1231, a1232)
    }, (0u64, f418()))?;
    Ok(h1229)
}

pub fn f336() -> Bytes {
    f8(Str::lit("target/release/verdant_readers"))
}

pub fn f337(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>) -> R<Adt11> {
    let m1233 = f237(v0.clone(), v2.clone())?;
    match m1233 {
        None => {
            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
        }
        Some(v6) => {
            if f28(v4.clone(), f8(Str::lit("none"))) {
                f419(v0, v1, v2, v3, v4, v5, v6)
            } else {
                let v7: Bytes = {
                    let a1234 = f8(Str::lit("Catalog."));
                    let a1235 = v4.clone();
                    append_bytes(a1234, a1235)
                };
                if f40(f420(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, v7.clone(), v3.clone())) {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v3, List::cons(f8(Str::lit(" in model/admissions.json, so it cannot check the entries\u{a}")), List::<Bytes>::nil()))))))))
                } else {
                    let m1236 = f237(v0.clone(), f7(List::cons(f8(Str::lit("encodings/")), List::cons(v4.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))?;
                    match m1236 {
                        None => {
                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: encodings/")), List::cons(v4, List::cons(f8(Str::lit(".schema.json is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v8) => {
                            let m1237 = f421(v8, v6.clone())?;
                            match m1237 {
                                None => {
                                    f419(v0, v1, v2, v3, {
                                        let a1238 = f8(Str::lit("Catalog."));
                                        let a1239 = v4;
                                        append_bytes(a1238, a1239)
                                    }, v5, v6)
                                }
                                Some(v9) => {
                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" does not conform to ")), List::cons(v7, List::cons(f8(Str::lit(": ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f338() -> Bytes {
    let a1240 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1240)
}

pub fn f339(v0: Adt13, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt13> = f79(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f251(Str::lit("id"), f47(v0.clone(), f8(Str::lit("id"))))?, List::cons(f251(Str::lit("name"), f47(v0.clone(), f8(Str::lit("name"))))?, List::cons(f251(Str::lit("issuer"), f47(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f251(Str::lit("edition"), f47(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f251(Str::lit("canonical_id"), f47(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f251(Str::lit("source_role"), f47(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f251(Str::lit("retrieval_date"), f47(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f251(Str::lit("url"), f395(v2.clone(), f8(Str::lit("url"))))?, List::cons(f251(Str::lit("media_type"), f395(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f251(Str::lit("license"), f395(v2.clone(), f8(Str::lit("license"))))?, List::cons(f251(Str::lit("redistribution"), f395(v2, f8(Str::lit("redistribution"))))?, List::cons(f251(Str::lit("statement"), f47(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1241 = v1;
        match m1241 {
            None => {
                f7(List::cons(f251(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f251(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f251(Str::lit("artifact"), {
                    let (h1242, _) = v3;
                    h1242
                })?, List::cons(f251(Str::lit("checksum"), {
                    let (_, h1243) = v3;
                    h1243
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f422(f175(f157(f79(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f340(p1246: List<List<Bytes>>, v1: Bytes, p1247: List<Bytes>) -> R<List<Bytes>> {
    let m1245 = (p1246, p1247);
    {
        let mut m1245 = m1245;
        loop {
            let step = {
                let (v0, v2) = m1245;
                let m1244 = v0;
                match m1244.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f423(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f342(f424(v3), f8(Str::lit("\u{a}"))), v2)))
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

pub fn f341(v0: Bytes) -> R<Bytes> {
    Ok(f35(v0.clone(), 0u64, f271(v0)?))
}

pub fn f342(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1248 = v0;
    match m1248.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1249 = v3.clone();
            match m1249.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1252 = v2;
                    let a1253 = {
                        let a1250 = v1.clone();
                        let a1251 = f342(v3, v1);
                        append_bytes(a1250, a1251)
                    };
                    append_bytes(a1252, a1253)
                }
            }
        }
    }
}

pub fn f343(v0: Adt13, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1254 = f79(v0, v1);
    match m1254 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v3) => {
            f157(f79(v3, v2))
        }
    }
}

pub fn f344(v0: Adt13, v1: Bytes) -> bool {
    let m1255 = f79(v0, v1);
    match m1255 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f345(v0: Bytes, p1283: u64, p1284: u64, p1285: bool, p1286: bool, p1287: u64, p1288: Bytes) -> R<Bytes> {
    let m1282 = (p1283, (p1284, (p1285, (p1286, (p1287, p1288)))));
    {
        let mut m1282 = m1282;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m1282;
                let m1256 = v2;
                if m1256 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f425(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m1256 - 1;
                    let m1259 = {
                        let a1257 = v0.clone();
                        let a1258 = v1;
                        index_bytes(a1257, a1258)
                    };
                    match m1259 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f425(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f62(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a1260 = v1;
                                    let a1261 = 1u64;
                                    nat_add(a1260, a1261)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a1262 = v9;
                                    let a1263 = 34u64;
                                    !nat_eq(a1262, a1263)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1264 = v9;
                                    let a1265 = 92u64;
                                    nat_eq(a1264, a1265)
                                }, (v5, v6))))))
                            } else {
                                let a1266 = v9;
                                let a1267 = 32u64;
                                if if if if nat_eq(a1266, a1267) {
                                    true
                                } else {
                                    let a1268 = v9;
                                    let a1269 = 9u64;
                                    nat_eq(a1268, a1269)
                                } {
                                    true
                                } else {
                                    let a1270 = v9;
                                    let a1271 = 10u64;
                                    nat_eq(a1270, a1271)
                                } {
                                    true
                                } else {
                                    let a1272 = v9;
                                    let a1273 = 13u64;
                                    nat_eq(a1272, a1273)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1274 = v1;
                                        let a1275 = 1u64;
                                        nat_add(a1274, a1275)?
                                    }, (v7, (false, (false, ({
                                        let a1276 = v1;
                                        let a1277 = 1u64;
                                        nat_add(a1276, a1277)?
                                    }, f425(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1278 = v1;
                                        let a1279 = 1u64;
                                        nat_add(a1278, a1279)?
                                    }, (v7, ({
                                        let a1280 = v9;
                                        let a1281 = 34u64;
                                        nat_eq(a1280, a1281)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f346(v0: Bytes, v1: u64) -> R<u64> {
    let m1291 = {
        let a1289 = v0;
        let a1290 = v1;
        index_bytes(a1289, a1290)
    };
    match m1291 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1336 = {
                let a1312 = {
                    let a1300 = {
                        let a1294 = {
                            let a1292 = v3;
                            let a1293 = 128u8;
                            fixed_u8::bit_and(a1292, a1293)
                        };
                        let a1295 = 128u8;
                        if equal(a1294, a1295) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1301 = {
                        let a1298 = {
                            let a1296 = v3;
                            let a1297 = 64u8;
                            fixed_u8::bit_and(a1296, a1297)
                        };
                        let a1299 = 64u8;
                        if equal(a1298, a1299) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1300, a1301)?
                };
                let a1313 = {
                    let a1310 = {
                        let a1304 = {
                            let a1302 = v3;
                            let a1303 = 32u8;
                            fixed_u8::bit_and(a1302, a1303)
                        };
                        let a1305 = 32u8;
                        if equal(a1304, a1305) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1311 = {
                        let a1308 = {
                            let a1306 = v3;
                            let a1307 = 16u8;
                            fixed_u8::bit_and(a1306, a1307)
                        };
                        let a1309 = 16u8;
                        if equal(a1308, a1309) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1310, a1311)?
                };
                nat_add(a1312, a1313)?
            };
            let a1337 = {
                let a1334 = {
                    let a1322 = {
                        let a1316 = {
                            let a1314 = v3;
                            let a1315 = 8u8;
                            fixed_u8::bit_and(a1314, a1315)
                        };
                        let a1317 = 8u8;
                        if equal(a1316, a1317) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1323 = {
                        let a1320 = {
                            let a1318 = v3;
                            let a1319 = 4u8;
                            fixed_u8::bit_and(a1318, a1319)
                        };
                        let a1321 = 4u8;
                        if equal(a1320, a1321) {
                            4u64
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
                            let a1325 = 2u8;
                            fixed_u8::bit_and(a1324, a1325)
                        };
                        let a1327 = 2u8;
                        if equal(a1326, a1327) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1333 = {
                        let a1330 = {
                            let a1328 = v3;
                            let a1329 = 1u8;
                            fixed_u8::bit_and(a1328, a1329)
                        };
                        let a1331 = 1u8;
                        if equal(a1330, a1331) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1332, a1333)?
                };
                nat_add(a1334, a1335)?
            };
            nat_add(a1336, a1337)
        }
    }
}

pub fn f347(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f426(v0.clone(), v1, {
        let a1339 = {
            let a1338 = v0.clone();
            length_bytes(a1338)
        };
        let a1340 = 1u64;
        nat_add(a1339, a1340)?
    })?;
    let a1341 = f346(v0.clone(), v2)?;
    let a1342 = 34u64;
    if nat_eq(a1341, a1342) {
        Ok(Some(({
            let a1343 = v2;
            let a1344 = 1u64;
            nat_add(a1343, a1344)?
        }, f35(v0, v1, {
            let a1345 = v2;
            let a1346 = v1;
            nat_sub(a1345, a1346)
        }))))
    } else {
        f427(v0, v1)
    }
}

pub fn f348(v0: Bytes, v1: u64) -> R<Option<(u64, Adt17)>> {
    let v2: bool = {
        let a1347 = f346(v0.clone(), v1)?;
        let a1348 = 45u64;
        nat_eq(a1347, a1348)
    };
    let v3: u64 = if v2 {
        let a1349 = v1;
        let a1350 = 1u64;
        nat_add(a1349, a1350)?
    } else {
        v1
    };
    let v4: u64 = f428(v0.clone(), v3)?;
    let v5: u64 = {
        let a1351 = v4;
        let a1352 = v3;
        nat_sub(a1351, a1352)
    };
    let v6: bool = {
        let a1353 = f346(v0.clone(), v4)?;
        let a1354 = 46u64;
        nat_eq(a1353, a1354)
    };
    let v7: u64 = if v6 {
        let a1355 = v4;
        let a1356 = 1u64;
        nat_add(a1355, a1356)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f428(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f346(v0.clone(), v8)?;
    let v10: bool = {
        let a1357 = v9;
        let a1358 = 101u64;
        if nat_eq(a1357, a1358) {
            true
        } else {
            let a1359 = v9;
            let a1360 = 69u64;
            nat_eq(a1359, a1360)
        }
    };
    let v11: u64 = f346(v0.clone(), {
        let a1361 = v8;
        let a1362 = 1u64;
        nat_add(a1361, a1362)?
    })?;
    let v12: bool = f227(v10, f429({
        let a1363 = v11;
        let a1364 = 43u64;
        nat_eq(a1363, a1364)
    }, {
        let a1365 = v11;
        let a1366 = 45u64;
        nat_eq(a1365, a1366)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a1367 = v8;
            let a1368 = 2u64;
            nat_add(a1367, a1368)?
        } else {
            let a1369 = v8;
            let a1370 = 1u64;
            nat_add(a1369, a1370)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f428(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f35(v0.clone(), v13, {
        let a1371 = v14;
        let a1372 = v13;
        nat_sub(a1371, a1372)
    });
    let v16: u64 = f430(v15.clone())?;
    let v17: Bytes = f35(v15.clone(), v16, {
        let a1374 = {
            let a1373 = v15;
            length_bytes(a1373)
        };
        let a1375 = v16;
        nat_sub(a1374, a1375)
    });
    let a1376 = v5;
    let a1377 = 0u64;
    if if nat_eq(a1376, a1377) {
        true
    } else {
        if f227({
            let a1378 = 1u64;
            let a1379 = v5;
            nat_lt(a1378, a1379)
        }, {
            let a1380 = f346(v0.clone(), v3)?;
            let a1381 = 48u64;
            nat_eq(a1380, a1381)
        }) {
            true
        } else {
            if f227(v6, {
                let a1382 = v8;
                let a1383 = v7;
                nat_eq(a1382, a1383)
            }) {
                true
            } else {
                f429(if v10 {
                    let a1384 = v14;
                    let a1385 = v13;
                    nat_eq(a1384, a1385)
                } else {
                    false
                }, {
                    let a1387 = 18u64;
                    let a1388 = {
                        let a1386 = v17.clone();
                        length_bytes(a1386)
                    };
                    nat_lt(a1387, a1388)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt17)>)
    } else {
        let m1392 = {
            let a1390 = {
                let a1389 = v17.clone();
                length_bytes(a1389)
            };
            let a1391 = 0u64;
            if nat_eq(a1390, a1391) {
                Some(0u64)
            } else {
                f431(v17)?
            }
        };
        match m1392 {
            None => {
                Ok(None::<(u64, Adt17)>)
            }
            Some(v18) => {
                let m1397 = f432(f35(v0.clone(), v3, v5), f35(v0, v7, {
                    let a1393 = v8;
                    let a1394 = v7;
                    nat_sub(a1393, a1394)
                }), f227(v12, {
                    let a1395 = v11;
                    let a1396 = 45u64;
                    nat_eq(a1395, a1396)
                }), v18)?;
                match m1397 {
                    None => {
                        Ok(None::<(u64, Adt17)>)
                    }
                    Some(v19) => {
                        let a1400 = 19u64;
                        let a1401 = {
                            let a1399 = {
                                let (h1398, _) = v19.clone();
                                h1398
                            };
                            length_bytes(a1399)
                        };
                        if nat_lt(a1400, a1401) {
                            Ok(None::<(u64, Adt17)>)
                        } else {
                            let m1403 = f431({
                                let (h1402, _) = v19.clone();
                                h1402
                            })?;
                            match m1403 {
                                None => {
                                    Ok(None::<(u64, Adt17)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f227(v2, {
                                        let a1404 = 0u64;
                                        let a1405 = v20;
                                        nat_lt(a1404, a1405)
                                    });
                                    if f433(v21, v20) {
                                        Ok(Some((v14, {
                                            let a1407 = {
                                                let (_, h1406) = v19.clone();
                                                h1406
                                            };
                                            let a1408 = 0u64;
                                            if nat_eq(a1407, a1408) {
                                                Adt17::C3(v21, v20)
                                            } else {
                                                Adt17::C4(v21, v20, {
                                                    let (_, h1409) = v19;
                                                    h1409
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt17)>)
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

pub fn f349(v0: Bytes) -> Bytes {
    if f200(v0.clone(), Bytes::lit(&[0x0d])) {
        f35(v0.clone(), 0u64, {
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

pub fn f350(v0: Adt13, v1: Bytes) -> Adt13 {
    let m1413 = v0;
    match m1413 {
        Adt13::C0(v3) => {
            Adt13::C0(v3)
        }
        Adt13::C1(v4, v5) => {
            Adt13::C1(v4, v5)
        }
        Adt13::C2(v2) => {
            Adt13::C2(f434(f49(), v2, v1))
        }
        Adt13::C3(r1414) => {
            let v6 = (*r1414).clone();
            Adt13::C3(std::rc::Rc::new(v6))
        }
        Adt13::C4(r1415) => {
            let v7 = (*r1415).clone();
            Adt13::C4(std::rc::Rc::new(v7))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r1416, r1417) => {
            let v8 = (*r1416).clone();
            let v9 = (*r1417).clone();
            Adt13::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt13::C7(v10, r1418, r1419) => {
            let v11 = (*r1418).clone();
            let v12 = (*r1419).clone();
            Adt13::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f351(v0: u64) -> R<Bytes> {
    f435(f436(), 10u64, v0)
}

pub fn f352(v0: Fn3, p1424: u64, p1425: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1423 = (p1424, p1425);
    {
        let mut m1423 = m1423;
        loop {
            let step = {
                let (v1, v2) = m1423;
                let m1420 = v1;
                if m1420 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1420 - 1;
                    let m1422 = {
                        let c1421: Fn3 = v0.clone();
                        c1421.apply(v2.clone())?
                    };
                    match m1422 {
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
                Ok(next) => { m1423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f437(v0, v1)
}

pub fn f354(v0: Fn4, p1429: List<(Bytes, Bytes)>, p1430: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1428 = (p1429, p1430);
    {
        let mut m1428 = m1428;
        loop {
            let step = {
                let (v1, v2) = m1428;
                let m1426 = v2;
                match m1426.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1427: Fn4 = v0.clone();
                            c1427.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1428 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f438(v0, v1)
}

pub fn f356(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f439(v0)
}

pub fn f357(v0: Fn5, p1434: List<Bytes>, p1435: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1433 = (p1434, p1435);
    {
        let mut m1433 = m1433;
        loop {
            let step = {
                let (v1, v2) = m1433;
                let m1431 = v2;
                match m1431.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1432: Fn5 = v0.clone();
                            c1432.apply(v1, v3)?
                        }, v4))
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

pub fn f358(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f440(v1)?, v0))
}

pub fn f359(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f262(v0)?, List::cons(f8(Str::lit(":")), List::cons(f262(v1)?, List::<Bytes>::nil())))))
}

pub fn f360(v0: Adt12) -> Bytes {
    let m1436 = v0;
    match m1436 {
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

pub fn f361(v0: Adt12) -> Bytes {
    let m1437 = v0;
    match m1437 {
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

pub fn f362(v0: Adt12) -> Bytes {
    let m1438 = v0;
    match m1438 {
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

pub fn f363(v0: Fn6, p1443: u64, p1444: u64) -> R<(u64, bool)> {
    let m1442 = (p1443, p1444);
    {
        let mut m1442 = m1442;
        loop {
            let step = {
                let (v1, v2) = m1442;
                let m1439 = v1;
                if m1439 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1439 - 1;
                    let m1441 = {
                        let c1440: Fn6 = v0.clone();
                        c1440.apply(v2)?
                    };
                    match m1441 {
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
                Ok(next) => { m1442 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: Bytes, v1: u64) -> Option<u64> {
    let a1445 = 0u64;
    let a1446 = v1;
    if nat_lt(a1445, a1446) {
        if f441(v0, {
            let a1447 = v1;
            let a1448 = 1u64;
            nat_sub(a1447, a1448)
        }) {
            Some({
                let a1449 = v1;
                let a1450 = 1u64;
                nat_sub(a1449, a1450)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f365(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1451 = v2;
    let a1452 = v0;
    if nat_lt(a1451, a1452) {
        if f441(v1, v2) {
            Ok(Some({
                let a1453 = v2;
                let a1454 = 1u64;
                nat_add(a1453, a1454)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f366(v0: (Bytes, Bytes), p1457: List<(Bytes, Bytes)>) -> bool {
    let m1456 = p1457;
    {
        let mut m1456 = m1456;
        loop {
            let step = {
                let v1 = m1456;
                let m1455 = v1;
                match m1455.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f442(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
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

pub fn f367(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1458 = v1;
    match m1458.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h1459, _) = v2;
                h1459
            }, v0.clone()) {
                List::cons({
                    let (_, h1460) = v2;
                    h1460
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
    let m1461 = f79(v0, v1);
    match m1461 {
        None => {
            false
        }
        Some(v3) => {
            f181(v3, v2)
        }
    }
}

pub fn f370() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f371(v0: Bytes) -> bool {
    f200(v0, f8(Str::lit(".lex.tex")))
}

pub fn f372(v0: Bytes) -> R<bool> {
    let v1: Bytes = f167(v0)?;
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
        let a1463 = {
            let a1462 = v1;
            length_bytes(a1462)
        };
        let a1464 = v2;
        nat_sub(a1463, a1464)
    });
    let v5: Bytes = if f72(v3.clone(), f8(Str::lit("("))) {
        let m1465 = f284(v3.clone(), f8(Str::lit(")")))?;
        match m1465 {
            None => {
                v3
            }
            Some(v4) => {
                f35(v3.clone(), {
                    let a1466 = v4;
                    let a1467 = 1u64;
                    nat_add(a1466, a1467)?
                }, {
                    let a1471 = {
                        let a1468 = v3;
                        length_bytes(a1468)
                    };
                    let a1472 = {
                        let a1469 = v4;
                        let a1470 = 1u64;
                        nat_add(a1469, a1470)?
                    };
                    nat_sub(a1471, a1472)
                })
            }
        }
    } else {
        v3
    };
    let a1473 = 0u64;
    let a1474 = v2;
    if nat_lt(a1473, a1474) {
        f443(v5)
    } else {
        Ok(false)
    }
}

pub fn f373(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f374(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1477 = {
            let a1475 = v4;
            let a1476 = v1;
            nat_add(a1475, a1476)?
        };
        let a1478 = v3;
        if nat_lt(a1477, a1478) {
            Ok(Some({
                let a1479 = v4;
                let a1480 = 1u64;
                nat_add(a1479, a1480)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f374(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1485 = {
        let a1482 = v0;
        let a1483 = v2;
        let a1484 = {
            let a1481 = v1.clone();
            length_bytes(a1481)
        };
        slice_bytes(a1482, a1483, a1484)
    };
    match m1485 {
        None => {
            false
        }
        Some(v3) => {
            let a1486 = v3;
            let a1487 = v1;
            equal(a1486, a1487)
        }
    }
}

pub fn f375(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1488 = v0;
    match m1488.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f375(v3, {
                let a1489 = v1;
                let a1490 = 1u64;
                nat_add(a1489, a1490)?
            })?))
        }
    }
}

pub fn f376(v0: Adt19, v1: List<u64>, v2: Adt22) -> bool {
    let m1491 = v2;
    match m1491 {
        Adt22::C0(v3, v4) => {
            if f382(v3, match v0.clone() {
                Adt19::C0(h1492, _, _, _) => {
                    h1492
                }
            }) {
                f444(v0, v4)
            } else {
                false
            }
        }
        Adt22::C1(v5) => {
            f382(v5, match v0 {
                Adt19::C0(_, _, h1493, _) => {
                    h1493
                }
            })
        }
        Adt22::C2(_, v7, v8) => {
            if f378(v7, v1) {
                f382(v8, match v0 {
                    Adt19::C0(_, _, _, h1494) => {
                        h1494
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f377(v0: Adt15, v1: Adt13) -> R<Adt22> {
    let m1495 = f79(v1.clone(), f8(Str::lit("std")));
    match m1495 {
        None => {
            let m1500 = f78(f79(v1.clone(), f8(Str::lit("req"))));
            match m1500 {
                None => {
                    let m1502 = f79(v1, f8(Str::lit("der")));
                    match m1502 {
                        None => {
                            Ok(f445(v0))
                        }
                        Some(v5) => {
                            let m1503 = f446(f47(v5.clone(), f8(Str::lit("rule"))));
                            match m1503 {
                                None => {
                                    Ok(f445(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt22::C2(v6, f292(f175(f157(f79(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1504) => {
                                            h1504
                                        }
                                    })?, f380(f47(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1505, _) => {
                                            h1505
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt22::C1(f380(v4, match v0 {
                        Adt15::C0(_, _, h1501, _, _) => {
                            h1501
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1496 = f78(f79(v2.clone(), f8(Str::lit("row"))));
            match m1496 {
                None => {
                    Ok(Adt22::C0(f380(f47(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1497, _, _, _, _) => {
                            h1497
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt22::C0(f380(f47(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1498, _, _, _, _) => {
                            h1498
                        }
                    })?, Some(f380(v3, match v0 {
                        Adt15::C0(_, h1499, _, _, _) => {
                            h1499
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f378(p1508: List<u64>, v1: List<u64>) -> bool {
    let m1507 = p1508;
    {
        let mut m1507 = m1507;
        loop {
            let step = {
                let v0 = m1507;
                let m1506 = v0;
                match m1506.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f382(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(p1513: List<u64>) -> bool {
    let m1512 = p1513;
    {
        let mut m1512 = m1512;
        loop {
            let step = {
                let v0 = m1512;
                let m1509 = v0;
                match m1509.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1510 = v1;
                        let a1511 = 7u64;
                        if nat_le(a1510, a1511) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1514 = f447(v0, v1.clone(), 0u64)?;
    match m1514 {
        None => {
            let a1515 = v1;
            Ok(length_list(a1515))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f381(v0: Adt13) -> u64 {
    let m1516 = f448(f79(v0, f8(Str::lit("stratum"))));
    match m1516 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f382(v0: u64, p1521: List<u64>) -> bool {
    let m1520 = p1521;
    {
        let mut m1520 = m1520;
        loop {
            let step = {
                let v1 = m1520;
                let m1517 = v1;
                match m1517.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1518 = v2;
                        let a1519 = v0;
                        if nat_eq(a1518, a1519) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
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

pub fn f383(v0: Fn7, p1525: List<Bytes>, p1526: List<(Bytes, ())>) -> List<Bytes> {
    let m1524 = (p1525, p1526);
    {
        let mut m1524 = m1524;
        loop {
            let step = {
                let (v1, v2) = m1524;
                let m1522 = v2;
                match m1522.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1523: Fn7 = v0.clone();
                            c1523.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1527, _) = v1;
        h1527
    }, v0)
}

pub fn f385(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f449(Fn12::F450, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f386(v0: Fn8, p1531: List<List<(Bytes, ())>>, p1532: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1530 = (p1531, p1532);
    {
        let mut m1530 = m1530;
        loop {
            let step = {
                let (v1, v2) = m1530;
                let m1528 = v2;
                match m1528.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1529: Fn8 = v0.clone();
                            c1529.apply(v1, v3)
                        }, v4))
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

pub fn f387(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f388(v0: Fn9, p1537: u64, p1538: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1536 = (p1537, p1538);
    {
        let mut m1536 = m1536;
        loop {
            let step = {
                let (v1, v2) = m1536;
                let m1533 = v1;
                if m1533 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1533 - 1;
                    let m1535 = {
                        let c1534: Fn9 = v0.clone();
                        c1534.apply(v2.clone())?
                    };
                    match m1535 {
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
                Ok(next) => { m1536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f389(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f451(v0)
}

pub fn f390(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f452(Fn13::F453, List::<(Bytes, ())>::nil(), v0)
}

pub fn f391(v0: Fn10, p1542: List<(Bytes, ())>, p1543: List<Bytes>) -> List<(Bytes, ())> {
    let m1541 = (p1542, p1543);
    {
        let mut m1541 = m1541;
        loop {
            let step = {
                let (v1, v2) = m1541;
                let m1539 = v2;
                match m1539.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1540: Fn10 = v0.clone();
                            c1540.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1541 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f393(p1546: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1545 = p1546;
    {
        let mut m1545 = m1545;
        loop {
            let step = {
                let v0 = m1545;
                let m1544 = v0;
                match m1544.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f454(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: Bytes, p1572: u64, p1573: u64, p1574: Bytes) -> R<Bytes> {
    let m1571 = (p1572, (p1573, p1574));
    {
        let mut m1571 = m1571;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1571;
                let m1547 = v2;
                if m1547 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1547 - 1;
                    let m1550 = {
                        let a1548 = v0.clone();
                        let a1549 = v1;
                        index_bytes(a1548, a1549)
                    };
                    match m1550 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            if f429({
                                let a1551 = v6;
                                let a1552 = 34u64;
                                nat_eq(a1551, a1552)
                            }, {
                                let a1553 = v6;
                                let a1554 = 92u64;
                                nat_eq(a1553, a1554)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1555 = v1;
                                    let a1556 = 1u64;
                                    nat_add(a1555, a1556)?
                                }, (v4, {
                                    let a1559 = v3;
                                    let a1560 = {
                                        let a1557 = f8(Str::lit("\\"));
                                        let a1558 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1557, a1558)
                                    };
                                    append_bytes(a1559, a1560)
                                })))
                            } else {
                                let a1561 = v6;
                                let a1562 = 10u64;
                                if nat_eq(a1561, a1562) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1563 = v1;
                                        let a1564 = 1u64;
                                        nat_add(a1563, a1564)?
                                    }, (v4, {
                                        let a1565 = v3;
                                        let a1566 = f8(Str::lit("\\n"));
                                        append_bytes(a1565, a1566)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1567 = v1;
                                        let a1568 = 1u64;
                                        nat_add(a1567, a1568)?
                                    }, (v4, {
                                        let a1569 = v3;
                                        let a1570 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1569, a1570)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: Option<Adt13>, v1: Bytes) -> Bytes {
    let m1575 = v0;
    match m1575 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f47(v2, v1)
        }
    }
}

pub fn f396(v0: List<Adt13>) -> List<Bytes> {
    let m1576 = v0;
    match m1576.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f455(f47(v1.clone(), f8(Str::lit("authority")))), List::cons(f244(f47(v1, f8(Str::lit("authority")))), f396(v2)))
        }
    }
}

pub fn f397(v0: List<Bytes>) -> R<List<Adt13>> {
    let m1577 = f254(v0.clone())?;
    match m1577 {
        None => {
            Ok(f230(f229(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt13>::nil())
        }
    }
}

pub fn f398(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
    let m1578 = f254(v3.clone())?;
    match m1578 {
        None => {
            let m1579 = f229(v3.clone())?;
            match m1579 {
                None => {
                    f458(v0, v1, v3)
                }
                Some(v5) => {
                    f457(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f456(v0, v1, v4)
        }
    }
}

pub fn f399(v0: List<Bytes>) -> List<Bytes> {
    let m1580 = v0;
    match m1580.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1582 = {
                    let a1581 = v1.clone();
                    length_bytes(a1581)
                };
                let a1583 = 0u64;
                if nat_eq(a1582, a1583) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f399(v2))
        }
    }
}

pub fn f400(v0: Bytes, v1: u64) -> R<u64> {
    let m1586 = {
        let a1584 = v0;
        let a1585 = v1;
        index_bytes(a1584, a1585)
    };
    match m1586 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f62(v2)
        }
    }
}

pub fn f401(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1590 = {
        let a1587 = v1;
        let a1588 = v2;
        let a1589 = 1u64;
        slice_bytes(a1587, a1588, a1589)
    };
    match m1590 {
        None => {
            v0
        }
        Some(v3) => {
            let a1591 = v0;
            let a1592 = v3;
            append_bytes(a1591, a1592)
        }
    }
}

pub fn f402(p1595: List<Bytes>, p1596: List<Bytes>) -> List<Bytes> {
    let m1594 = (p1595, p1596);
    {
        let mut m1594 = m1594;
        loop {
            let step = {
                let (v0, v1) = m1594;
                let m1593 = v0;
                match m1593.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1594 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403(v0: List<Bytes>) -> Bytes {
    let m1597 = v0;
    match m1597.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f404(v0: Bytes) -> R<bool> {
    let m1599 = {
        let a1598 = v0.clone();
        length_bytes(a1598)
    };
    if m1599 == 0 {
        Ok(true)
    } else {
        let v1 = m1599 - 1;
        let m1602 = {
            let a1600 = v0;
            let a1601 = v1;
            index_bytes(a1600, a1601)
        };
        match m1602 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f459({
                    let v3: u8 = v2;
                    let a1647 = {
                        let a1623 = {
                            let a1611 = {
                                let a1605 = {
                                    let a1603 = v3;
                                    let a1604 = 128u8;
                                    fixed_u8::bit_and(a1603, a1604)
                                };
                                let a1606 = 128u8;
                                if equal(a1605, a1606) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1612 = {
                                let a1609 = {
                                    let a1607 = v3;
                                    let a1608 = 64u8;
                                    fixed_u8::bit_and(a1607, a1608)
                                };
                                let a1610 = 64u8;
                                if equal(a1609, a1610) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1611, a1612)?
                        };
                        let a1624 = {
                            let a1621 = {
                                let a1615 = {
                                    let a1613 = v3;
                                    let a1614 = 32u8;
                                    fixed_u8::bit_and(a1613, a1614)
                                };
                                let a1616 = 32u8;
                                if equal(a1615, a1616) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1622 = {
                                let a1619 = {
                                    let a1617 = v3;
                                    let a1618 = 16u8;
                                    fixed_u8::bit_and(a1617, a1618)
                                };
                                let a1620 = 16u8;
                                if equal(a1619, a1620) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1621, a1622)?
                        };
                        nat_add(a1623, a1624)?
                    };
                    let a1648 = {
                        let a1645 = {
                            let a1633 = {
                                let a1627 = {
                                    let a1625 = v3;
                                    let a1626 = 8u8;
                                    fixed_u8::bit_and(a1625, a1626)
                                };
                                let a1628 = 8u8;
                                if equal(a1627, a1628) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1634 = {
                                let a1631 = {
                                    let a1629 = v3;
                                    let a1630 = 4u8;
                                    fixed_u8::bit_and(a1629, a1630)
                                };
                                let a1632 = 4u8;
                                if equal(a1631, a1632) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1633, a1634)?
                        };
                        let a1646 = {
                            let a1643 = {
                                let a1637 = {
                                    let a1635 = v3;
                                    let a1636 = 2u8;
                                    fixed_u8::bit_and(a1635, a1636)
                                };
                                let a1638 = 2u8;
                                if equal(a1637, a1638) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1644 = {
                                let a1641 = {
                                    let a1639 = v3;
                                    let a1640 = 1u8;
                                    fixed_u8::bit_and(a1639, a1640)
                                };
                                let a1642 = 1u8;
                                if equal(a1641, a1642) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1643, a1644)?
                        };
                        nat_add(a1645, a1646)?
                    };
                    nat_add(a1647, a1648)?
                }))
            }
        }
    }
}

pub fn f405(v0: Adt17, v1: Bytes) -> bool {
    let a1650 = {
        let a1649 = f241(v0, v1);
        length_bytes(a1649)
    };
    let a1651 = 0u64;
    !nat_eq(a1650, a1651)
}

pub fn f406(v0: Adt17, v1: Bytes) -> bool {
    let m1652 = f238(v0, v1);
    match m1652 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f407(v0: Bytes, p1673: u64, p1674: u64, p1675: bool, p1676: Bytes) -> R<Bytes> {
    let m1672 = (p1673, (p1674, (p1675, p1676)));
    {
        let mut m1672 = m1672;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1672;
                let m1653 = v2;
                if m1653 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1653 - 1;
                    let m1656 = {
                        let a1654 = v0.clone();
                        let a1655 = v1;
                        index_bytes(a1654, a1655)
                    };
                    match m1656 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f62(v6)?;
                            if f460(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1657 = v1;
                                    let a1658 = 1u64;
                                    nat_add(a1657, a1658)?
                                }, (v5, (true, v4))))
                            } else {
                                if f227(f461(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1659 = v1;
                                        let a1660 = 1u64;
                                        nat_add(a1659, a1660)?
                                    }, (v5, (false, {
                                        let a1664 = v4;
                                        let a1665 = f35({
                                            let a1661 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1661)
                                        }, {
                                            let a1662 = v7;
                                            let a1663 = 97u64;
                                            nat_sub(a1662, a1663)
                                        }, 1u64);
                                        append_bytes(a1664, a1665)
                                    }))))
                                } else {
                                    if f429(f461(v7), f429(f462(v7), f463(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1666 = v1;
                                            let a1667 = 1u64;
                                            nat_add(a1666, a1667)?
                                        }, (v5, (false, {
                                            let a1668 = v4;
                                            let a1669 = f35(v0.clone(), v1, 1u64);
                                            append_bytes(a1668, a1669)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1670 = v1;
                                            let a1671 = 1u64;
                                            nat_add(a1670, a1671)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1672 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: Adt17, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt17)> = f464(f465(f238(v0.clone(), f8(Str::lit("$defs")))), f465(f238(v0.clone(), f8(Str::lit("definitions")))));
    let m1677 = f466(4096u64, v3.clone(), List::<Bytes>::nil(), f467(v3)?)?;
    match m1677 {
        Ok(v5) => {
            let m1678 = f468(4096u64, f469(f470(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1678 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f342(f203(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f409(v0: List<Adt13>, v1: Bytes, v2: Bytes, v3: Adt13) -> List<Adt13> {
    let m1679 = v0;
    match m1679.uncons() {
        None => {
            List::cons(v3, List::<Adt13>::nil())
        }
        Some((v4, v5)) => {
            if f28(f47(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f471(v5, v1, v2))
            } else {
                List::cons(v4, f409(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f410(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f342(f472(f81(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f411(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f351(v1)?;
    let v4: u64 = {
        let a1680 = v3.clone();
        length_bytes(a1680)
    };
    let v5: Bytes = {
        let a1681 = v2;
        let a1682 = v4;
        if nat_lt(a1681, a1682) {
            f7(List::cons(f35(v3.clone(), 0u64, {
                let a1683 = v4;
                let a1684 = v2;
                nat_sub(a1683, a1684)
            }), List::cons(f8(Str::lit(".")), List::cons(f35(v3, {
                let a1685 = v4;
                let a1686 = v2;
                nat_sub(a1685, a1686)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f473({
                let a1687 = v2;
                let a1688 = v4;
                nat_sub(a1687, a1688)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1689 = f8(Str::lit("-"));
        let a1690 = v5;
        Ok(append_bytes(a1689, a1690))
    } else {
        Ok(v5)
    }
}

pub fn f412(v0: Bytes) -> R<Bytes> {
    let a1695 = f426(v0.clone(), 0u64, {
        let a1692 = {
            let a1691 = v0.clone();
            length_bytes(a1691)
        };
        let a1693 = 1u64;
        nat_add(a1692, a1693)?
    })?;
    let a1696 = {
        let a1694 = v0.clone();
        length_bytes(a1694)
    };
    if nat_eq(a1695, a1696) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f262(v0)
    }
}

pub fn f413(v0: Adt17) -> R<List<Bytes>> {
    let m1697 = v0;
    match m1697 {
        Adt17::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C3(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C4(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C8 => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C9(r1698, r1699) => {
            let v12 = (*r1698).clone();
            let v13 = (*r1699).clone();
            Ok(List::cons(f331(v12)?, f413(v13)?))
        }
        Adt17::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f414(v0: Adt17) -> R<List<(Bytes, Bytes)>> {
    let m1700 = v0;
    match m1700 {
        Adt17::C0 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C3(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C4(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C8 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C9(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C10(v14, r1701, r1702) => {
            let v15 = (*r1701).clone();
            let v16 = (*r1702).clone();
            Ok(List::cons((v14, f331(v15)?), f414(v16)?))
        }
    }
}

pub fn f415(v0: u32) -> Bytes {
    let a1731 = f474({
        let a1703 = f475(v0, 28u32);
        let a1704 = 15u32;
        fixed_u32::bit_and(a1703, a1704)
    });
    let a1732 = {
        let a1729 = f474({
            let a1705 = f475(v0, 24u32);
            let a1706 = 15u32;
            fixed_u32::bit_and(a1705, a1706)
        });
        let a1730 = {
            let a1727 = f474({
                let a1707 = f475(v0, 20u32);
                let a1708 = 15u32;
                fixed_u32::bit_and(a1707, a1708)
            });
            let a1728 = {
                let a1725 = f474({
                    let a1709 = f475(v0, 16u32);
                    let a1710 = 15u32;
                    fixed_u32::bit_and(a1709, a1710)
                });
                let a1726 = {
                    let a1723 = f474({
                        let a1711 = f475(v0, 12u32);
                        let a1712 = 15u32;
                        fixed_u32::bit_and(a1711, a1712)
                    });
                    let a1724 = {
                        let a1721 = f474({
                            let a1713 = f475(v0, 8u32);
                            let a1714 = 15u32;
                            fixed_u32::bit_and(a1713, a1714)
                        });
                        let a1722 = {
                            let a1719 = f474({
                                let a1715 = f475(v0, 4u32);
                                let a1716 = 15u32;
                                fixed_u32::bit_and(a1715, a1716)
                            });
                            let a1720 = f474({
                                let a1717 = v0;
                                let a1718 = 15u32;
                                fixed_u32::bit_and(a1717, a1718)
                            });
                            append_bytes(a1719, a1720)
                        };
                        append_bytes(a1721, a1722)
                    };
                    append_bytes(a1723, a1724)
                };
                append_bytes(a1725, a1726)
            };
            append_bytes(a1727, a1728)
        };
        append_bytes(a1729, a1730)
    };
    append_bytes(a1731, a1732)
}

pub fn f416(v0: Fn11, p1736: u64, p1737: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1735 = (p1736, p1737);
    {
        let mut m1735 = m1735;
        loop {
            let step = {
                let (v1, v2) = m1735;
                let m1733 = v1;
                if m1733 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1733 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1734: Fn11 = v0.clone();
                        c1734.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1739 = {
            let (h1738, _) = v3.clone();
            h1738
        };
        let a1740 = 64u64;
        nat_add(a1739, a1740)?
    }, f476({
        let (_, h1741) = v3;
        h1741
    }, f477(v0, v1, v2, {
        let (h1742, _) = v3;
        h1742
    })?)?))
}

pub fn f418() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f419(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Adt17) -> R<Adt11> {
    let m1743 = f478(f479(v1.clone(), v3.clone(), v4, v6))?;
    match m1743 {
        Ok(v8) => {
            let v9: Bytes = f455(v1.clone());
            Ok(Adt11::C3(List::cons((v9.clone(), v8), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f409(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v9.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("row")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5))))))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("translation")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("command")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f328(v5)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt13::C2(f245(f53(v0, v2))?)), std::rc::Rc::new(Adt13::C5))))))), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::<(Bytes, Bytes)>::nil())), f7(List::cons(f8(Str::lit("admitted ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        }
        Err(v7) => {
            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(": ")), List::cons(v7, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
        }
    }
}

pub fn f420(p1747: List<Adt13>, v1: Bytes, v2: Bytes) -> bool {
    let m1746 = p1747;
    {
        let mut m1746 = m1746;
        loop {
            let step = {
                let v0 = m1746;
                let m1744 = v0;
                match m1744.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f227(f28(f47(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m1745 = f79(v3, f8(Str::lit("std")));
                            match m1745 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    f28(f47(v5, f8(Str::lit("row"))), v2.clone())
                                }
                            }
                        }) {
                            Err::<List<Adt13>, bool>(true)
                        } else {
                            Ok::<List<Adt13>, bool>(v4)
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

pub fn f421(v0: Adt17, v1: Adt17) -> R<Option<Bytes>> {
    f480({
        let a1754 = {
            let a1749 = {
                let a1748 = f481(v0.clone())?;
                length_bytes(a1748)
            };
            let a1750 = 2u64;
            nat_add(a1749, a1750)?
        };
        let a1755 = {
            let a1752 = {
                let a1751 = f481(v1.clone())?;
                length_bytes(a1751)
            };
            let a1753 = 2u64;
            nat_add(a1752, a1753)?
        };
        nat_mul(a1754, a1755)?
    }, v0.clone(), v0, v1, Bytes::lit(&[]))
}

pub fn f422(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f342(f482(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f423(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1756 = f33(v1, 34u8)?;
    match m1756.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1757 = v3;
            match m1757.uncons() {
                None => {
                    Ok(f483(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f424(v0: List<Bytes>) -> List<Bytes> {
    let m1758 = v0;
    match m1758.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("authority = []"))) {
                f424(v2)
            } else {
                List::cons(v1, f424(v2))
            }
        }
    }
}

pub fn f425(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1761 = v3;
    let a1762 = f35(v0, v1, {
        let a1759 = v2;
        let a1760 = v1;
        nat_sub(a1759, a1760)
    });
    append_bytes(a1761, a1762)
}

pub fn f426(v0: Bytes, p1775: u64, p1776: u64) -> R<u64> {
    let m1774 = (p1775, p1776);
    {
        let mut m1774 = m1774;
        loop {
            let step = {
                let (v1, v2) = m1774;
                let m1763 = v2;
                if m1763 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1763 - 1;
                    let v4: u64 = f346(v0.clone(), v1)?;
                    let a1764 = v4;
                    let a1765 = 34u64;
                    if nat_eq(a1764, a1765) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1766 = v4;
                        let a1767 = 92u64;
                        if nat_eq(a1766, a1767) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1768 = v4;
                            let a1769 = 32u64;
                            if nat_lt(a1768, a1769) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1770 = v4;
                                let a1771 = 256u64;
                                if nat_eq(a1770, a1771) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1772 = v1;
                                        let a1773 = 1u64;
                                        nat_add(a1772, a1773)?
                                    }, v3))
                                }
                            }
                        }
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

pub fn f427(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f484(Fn14::F485(v0.clone()), {
        let a1778 = {
            let a1777 = v0;
            length_bytes(a1777)
        };
        let a1779 = 1u64;
        nat_add(a1778, a1779)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1783 = {
        let (_, h1780) = {
            let (_, h1781) = {
                let (h1782, _) = v2.clone();
                h1782
            };
            h1781
        };
        h1780
    };
    let a1784 = 1u64;
    if nat_eq(a1783, a1784) {
        Ok(Some(({
            let (h1785, _) = {
                let (h1786, _) = v2.clone();
                h1786
            };
            h1785
        }, {
            let (h1787, _) = {
                let (_, h1788) = {
                    let (h1789, _) = v2;
                    h1789
                };
                h1788
            };
            h1787
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f428(v0: Bytes, v1: u64) -> R<u64> {
    let (h1790, _) = f363(Fn6::F486(v0.clone()), {
        let a1792 = {
            let a1791 = v0;
            length_bytes(a1791)
        };
        let a1793 = 1u64;
        nat_add(a1792, a1793)?
    }, v1)?;
    Ok(h1790)
}

pub fn f429(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f430(v0: Bytes) -> R<u64> {
    let (h1794, _) = f363(Fn6::F487(v0.clone()), {
        let a1796 = {
            let a1795 = v0;
            length_bytes(a1795)
        };
        let a1797 = 1u64;
        nat_add(a1796, a1797)?
    }, 0u64)?;
    Ok(h1794)
}

pub fn f431(v0: Bytes) -> R<Option<u64>> {
    if f488(v0.clone())? {
        Ok(Some({
            let (_, h1798) = {
                let (h1799, _) = f489(Fn15::F490(v0.clone()), {
                    let a1800 = v0;
                    length_bytes(a1800)
                }, (0u64, 0u64))?;
                h1799
            };
            h1798
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f432(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f35(v1.clone(), 0u64, {
        let a1802 = {
            let a1801 = v1.clone();
            length_bytes(a1801)
        };
        let a1803 = f491(v1)?;
        nat_sub(a1802, a1803)
    });
    let v5: Bytes = {
        let a1804 = v0;
        let a1805 = v4.clone();
        append_bytes(a1804, a1805)
    };
    let v6: u64 = f430(v5.clone())?;
    let v7: Bytes = f35(v5.clone(), v6, {
        let a1807 = {
            let a1806 = v5;
            length_bytes(a1806)
        };
        let a1808 = v6;
        nat_sub(a1807, a1808)
    });
    let v8: u64 = {
        let a1809 = v4;
        length_bytes(a1809)
    };
    let v9: u64 = {
        let a1810 = v7.clone();
        length_bytes(a1810)
    };
    let a1811 = v9;
    let a1812 = 0u64;
    if nat_eq(a1811, a1812) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f429(v2, {
            let a1813 = v3;
            let a1814 = v8;
            nat_lt(a1813, a1814)
        }) {
            let v10: u64 = if v2 {
                let a1815 = v3;
                let a1816 = v8;
                nat_add(a1815, a1816)?
            } else {
                let a1817 = v8;
                let a1818 = v3;
                nat_sub(a1817, a1818)
            };
            let v11: u64 = f491(v7.clone())?;
            let a1819 = v10;
            let a1820 = v11;
            if nat_le(a1819, a1820) {
                Ok(Some((f35(v7, 0u64, {
                    let a1821 = v9;
                    let a1822 = v10;
                    nat_sub(a1821, a1822)
                }), 0u64)))
            } else {
                let a1825 = {
                    let a1823 = v10;
                    let a1824 = v11;
                    nat_sub(a1823, a1824)
                };
                let a1826 = 19u64;
                if nat_le(a1825, a1826) {
                    Ok(Some((f35(v7, 0u64, {
                        let a1827 = v9;
                        let a1828 = v11;
                        nat_sub(a1827, a1828)
                    }), {
                        let a1829 = v10;
                        let a1830 = v11;
                        nat_sub(a1829, a1830)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1831 = v3;
                let a1832 = v8;
                nat_sub(a1831, a1832)
            };
            let a1835 = {
                let a1833 = v9;
                let a1834 = v12;
                nat_add(a1833, a1834)?
            };
            let a1836 = 19u64;
            if nat_le(a1835, a1836) {
                Ok(Some(({
                    let a1837 = v7;
                    let a1838 = f473(v12);
                    append_bytes(a1837, a1838)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f433(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1839 = 0u64;
        let a1840 = v1;
        if nat_lt(a1839, a1840) {
            let a1841 = v1;
            let a1842 = 9223372036854775808u64;
            nat_le(a1841, a1842)
        } else {
            false
        }
    } else {
        let a1843 = v1;
        let a1844 = 9223372036854775807u64;
        nat_le(a1843, a1844)
    }
}

pub fn f434(p1847: List<Adt12>, v1: Bytes, v2: Bytes) -> Bytes {
    let m1846 = p1847;
    {
        let mut m1846 = m1846;
        loop {
            let step = {
                let v0 = m1846;
                let m1845 = v0;
                match m1845.uncons() {
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
                Ok(next) => { m1846 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f435(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1848 = v2;
    let a1849 = 0u64;
    if nat_eq(a1848, a1849) {
        Ok(f492(v0, 0u64))
    } else {
        let (_, h1850) = {
            let (h1851, _) = f352(Fn3::F493(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1851
        };
        Ok(h1850)
    }
}

pub fn f436() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f437(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1855 = {
        let a1853 = v0.clone();
        let a1854 = {
            let (h1852, _) = v1.clone();
            h1852
        };
        index_bytes(a1853, a1854)
    };
    match m1855 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1857 = {
                    let (h1856, _) = v1.clone();
                    h1856
                };
                let a1858 = 1u64;
                nat_add(a1857, a1858)?
            }, {
                let a1861 = {
                    let (_, h1859) = v1;
                    h1859
                };
                let a1862 = f494(v0, {
                    let (h1860, _) = v1;
                    h1860
                }, v2)?;
                append_bytes(a1861, a1862)
            })))
        }
    }
}

pub fn f438(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1863 = v0.clone();
    match m1863.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1866 = {
                let (h1864, _) = v2;
                h1864
            };
            let a1867 = {
                let (h1865, _) = v1.clone();
                h1865
            };
            if equal(a1866, a1867) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f439(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f495(f496(Fn16::F497, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1872 = {
        let (h1868, _) = f498(Fn17::F499, {
            let a1870 = {
                let a1869 = v0;
                length_list(a1869)
            };
            let a1871 = 1u64;
            nat_add(a1870, a1871)?
        }, v1)?;
        h1868
    };
    match m1872.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f440(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1877 = {
        let a1874 = f262({
            let (h1873, _) = v0;
            h1873
        })?;
        let a1875 = Bytes::lit(&[0x3a]);
        append_bytes(a1874, a1875)
    };
    let a1878 = {
        let (_, h1876) = v0;
        h1876
    };
    Ok(append_bytes(a1877, a1878))
}

pub fn f441(v0: Bytes, v1: u64) -> bool {
    let m1879 = f168(v0, v1);
    match m1879 {
        None => {
            false
        }
        Some(v2) => {
            f500(v2)
        }
    }
}

pub fn f442(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f227(f28({
        let (h1880, _) = v0;
        h1880
    }, {
        let (h1881, _) = v1;
        h1881
    }), f28({
        let (_, h1882) = v0;
        h1882
    }, {
        let (_, h1883) = v1;
        h1883
    }))
}

pub fn f443(v0: Bytes) -> R<bool> {
    let m1884 = f284(v0.clone(), f8(Str::lit("&&")))?;
    if match m1884 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1885 = f284(v0, f8(Str::lit("||")))?;
        match m1885 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f444(v0: Adt19, v1: Option<u64>) -> bool {
    let m1886 = v1;
    match m1886 {
        None => {
            true
        }
        Some(v2) => {
            f382(v2, match v0 {
                Adt19::C0(_, h1887, _, _) => {
                    h1887
                }
            })
        }
    }
}

pub fn f445(v0: Adt15) -> Adt22 {
    Adt22::C1({
        let a1889 = match v0 {
            Adt15::C0(_, _, h1888, _, _) => {
                h1888
            }
        };
        length_list(a1889)
    })
}

pub fn f446(v0: Bytes) -> Option<Adt23> {
    if f28(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt23::C0)
    } else {
        if f28(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt23::C1)
        } else {
            if f28(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt23::C2)
            } else {
                None::<Adt23>
            }
        }
    }
}

pub fn f447(v0: Bytes, p1894: List<Bytes>, p1895: u64) -> R<Option<u64>> {
    let m1893 = (p1894, p1895);
    {
        let mut m1893 = m1893;
        loop {
            let step = {
                let (v1, v2) = m1893;
                let m1890 = v1;
                match m1890.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f28(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1891 = v2;
                                let a1892 = 1u64;
                                nat_add(a1891, a1892)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f448(v0: Option<Adt13>) -> Option<u64> {
    let m1896 = v0;
    match m1896 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1897 = v1;
            match m1897 {
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

pub fn f449(v0: Fn12, p1901: List<List<(Bytes, ())>>, p1902: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1900 = (p1901, p1902);
    {
        let mut m1900 = m1900;
        loop {
            let step = {
                let (v1, v2) = m1900;
                let m1898 = v2;
                match m1898.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1899: Fn12 = v0.clone();
                            c1899.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1900 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f451(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1904 = {
        let a1903 = v0.clone();
        length_list(a1903)
    };
    let a1905 = 1u64;
    if nat_le(a1904, a1905) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f501(v0)?))
    }
}

pub fn f452(v0: Fn13, p1909: List<(Bytes, ())>, p1910: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1908 = (p1909, p1910);
    {
        let mut m1908 = m1908;
        loop {
            let step = {
                let (v1, v2) = m1908;
                let m1906 = v2;
                match m1906.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1907: Fn13 = v0.clone();
                            c1907.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f454(p1914: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1913 = p1914;
    {
        let mut m1913 = m1913;
        loop {
            let step = {
                let v0 = m1913;
                let m1911 = v0;
                match m1911.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f502(v2, v1.clone(), 0u64, {
                            let a1912 = v1.clone();
                            length_bytes(a1912)
                        })? {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v3)
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

pub fn f455(v0: Bytes) -> Bytes {
    f7(List::cons(f112(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f456(v0: Adt1, v1: Bytes, v2: List<Adt13>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f115(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f51(v3.clone(), f115(f82(v0, f125())?, f8(Str::lit("at"))));
    let a1916 = {
        let a1915 = v4.clone();
        length_list(a1915)
    };
    let a1917 = 0u64;
    if nat_eq(a1916, a1917) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1920 = {
            let a1918 = v4;
            length_list(a1918)
        };
        let a1921 = {
            let a1919 = v3;
            length_list(a1919)
        };
        if nat_eq(a1920, a1921) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f503(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f457(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt13>) -> R<(bool, Bytes)> {
    let m1922 = v3;
    match m1922.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f47(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f47(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f47(v4.clone(), f8(Str::lit("construct")));
            if f429(f56(v0.clone(), f455(v6.clone())), f41(v6.clone(), v2.clone())) {
                f457(v0, v1, v2, v5)
            } else {
                if f40(f56(v0.clone(), f244(v6.clone()))) {
                    let v9: (bool, Bytes) = f457(v0, v1.clone(), v2, v5)?;
                    let (h1923, _) = v9.clone();
                    if h1923 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a1925 = f503(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a1926 = {
                                let (_, h1924) = v9;
                                h1924
                            };
                            append_bytes(a1925, a1926)
                        }))
                    }
                } else {
                    if f227(f40(f28(v8.clone(), f8(Str::lit("none")))), f40(f420(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a1927 = f8(Str::lit("Catalog."));
                        let a1928 = v8.clone();
                        append_bytes(a1927, a1928)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f457(v0, v1.clone(), v2, v5)?;
                        let (h1929, _) = v10.clone();
                        if h1929 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a1931 = f503(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a1932 = {
                                    let (_, h1930) = v10;
                                    h1930
                                };
                                append_bytes(a1931, a1932)
                            }))
                        }
                    } else {
                        Ok((true, f7(List::cons(f8(Str::lit("admit-registry ")), List::cons(v6, List::cons(f8(Str::lit(" ")), List::cons(v7, List::cons(f8(Str::lit(" ")), List::cons(v8, List::cons(f8(Str::lit(" ")), List::cons(f47(v4.clone(), f8(Str::lit("reader"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v4, f8(Str::lit("artifact"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))))))
                    }
                }
            }
        }
    }
}

pub fn f458(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt13> = f230(f232(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt13> = f230(f232(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f98(f115(v3.clone(), f8(Str::lit("id"))), f115(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f115(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f51(v5.clone(), f98(v6.clone(), f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1934 = {
        let a1933 = v7.clone();
        length_list(a1933)
    };
    let a1935 = 0u64;
    if nat_eq(a1934, a1935) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f40({
            let a1938 = {
                let a1936 = v7;
                length_list(a1936)
            };
            let a1939 = {
                let a1937 = v5;
                length_list(a1937)
            };
            nat_eq(a1938, a1939)
        }) {
            Ok((false, f503(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f118(f98(f51(f504(v4.clone()), f52(f53(v0, f8(Str::lit("model/authorities.toml"))))?), f51(f505(v4), f98(v6, f115(v3, f8(Str::lit("id")))))))?;
            let a1941 = {
                let a1940 = v8.clone();
                length_list(a1940)
            };
            let a1942 = 0u64;
            if nat_eq(a1941, a1942) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f503(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f95(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f459(v0: u64) -> bool {
    let a1943 = v0;
    let a1944 = 47u64;
    nat_eq(a1943, a1944)
}

pub fn f460(v0: u64) -> bool {
    f429({
        let a1945 = v0;
        let a1946 = 95u64;
        nat_eq(a1945, a1946)
    }, f429({
        let a1947 = v0;
        let a1948 = 45u64;
        nat_eq(a1947, a1948)
    }, f429({
        let a1949 = v0;
        let a1950 = 32u64;
        nat_eq(a1949, a1950)
    }, f429({
        let a1951 = v0;
        let a1952 = 46u64;
        nat_eq(a1951, a1952)
    }, {
        let a1953 = v0;
        let a1954 = 47u64;
        nat_eq(a1953, a1954)
    }))))
}

pub fn f461(v0: u64) -> bool {
    f227({
        let a1955 = 97u64;
        let a1956 = v0;
        nat_le(a1955, a1956)
    }, {
        let a1957 = v0;
        let a1958 = 122u64;
        nat_le(a1957, a1958)
    })
}

pub fn f462(v0: u64) -> bool {
    f227({
        let a1959 = 65u64;
        let a1960 = v0;
        nat_le(a1959, a1960)
    }, {
        let a1961 = v0;
        let a1962 = 90u64;
        nat_le(a1961, a1962)
    })
}

pub fn f463(v0: u64) -> bool {
    f227({
        let a1963 = 48u64;
        let a1964 = v0;
        nat_le(a1963, a1964)
    }, {
        let a1965 = v0;
        let a1966 = 57u64;
        nat_le(a1965, a1966)
    })
}

pub fn f464(v0: List<(Bytes, Adt17)>, v1: List<(Bytes, Adt17)>) -> List<(Bytes, Adt17)> {
    let m1967 = v0;
    match m1967.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f464(v3, v1))
        }
    }
}

pub fn f465(v0: Option<Adt17>) -> List<(Bytes, Adt17)> {
    let m1968 = v0;
    match m1968 {
        None => {
            List::<(Bytes, Adt17)>::nil()
        }
        Some(v1) => {
            f506(v1)
        }
    }
}

pub fn f466(v0: u64, v1: List<(Bytes, Adt17)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt17)>, Bytes>> {
    let m1969 = v0;
    if m1969 == 0 {
        let m1970 = v1;
        match m1970.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt17)>, Bytes>(List::<(Bytes, Adt17)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt17)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m1969 - 1;
        let m1971 = v1.clone();
        match m1971.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt17)>, Bytes>(List::<(Bytes, Adt17)>::nil()))
            }
            Some((_, _)) => {
                let m1972 = f507(v1.clone(), v2.clone(), v3.clone())?;
                match m1972 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt17)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f342(f467(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m1975 = f466(v6, f508(v1, {
                            let (h1973, _) = v9.clone();
                            h1973
                        }), List::cons(f509({
                            let (h1974, _) = v9.clone();
                            h1974
                        })?, v2), v3)?;
                        match m1975 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt17)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt17)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f467(v0: List<(Bytes, Adt17)>) -> R<List<Bytes>> {
    let m1976 = v0;
    match m1976.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f509({
                let (h1977, _) = v1;
                h1977
            })?, f467(v2)?))
        }
    }
}

pub fn f468(p1983: u64, p1984: List<Adt25>, p1985: List<Bytes>, p1986: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1982 = (p1983, (p1984, (p1985, p1986)));
    {
        let mut m1982 = m1982;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1982;
                let m1978 = v0;
                if m1978 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1978 - 1;
                    let m1979 = v1;
                    match m1979.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1980 = v5;
                            match m1980 {
                                Adt25::C0(v8, v9) => {
                                    if f28(v8.clone(), f510()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f41(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1981 = f512(v8.clone(), v9)?;
                                            match m1981 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f469(f513(v11.clone()), List::cons(Adt25::C1(f514(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f41(f510(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f510(), v2), List::cons(f511(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1982 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m1987 = v0;
    match m1987.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f469(v3, v1))
        }
    }
}

pub fn f470(v0: List<(Bytes, Adt17)>) -> R<List<Adt25>> {
    let m1988 = v0;
    match m1988.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f509({
                let (h1989, _) = v1;
                h1989
            })?, {
                let (_, h1990) = v1;
                h1990
            }), f470(v2)?))
        }
    }
}

pub fn f471(v0: List<Adt13>, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1991 = v0;
    match m1991.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            if f28(f47(v3.clone(), v1.clone()), v2.clone()) {
                f471(v4, v1, v2)
            } else {
                List::cons(v3, f471(v4, v1, v2))
            }
        }
    }
}

pub fn f472(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1992 = v0;
    match m1992.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f72(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f342(f482(f118(List::cons(v1, f515(f33(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f472(v3, v1)?))
            }
        }
    }
}

pub fn f473(v0: u64) -> Bytes {
    let m1993 = v0;
    if m1993 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1993 - 1;
        let a1994 = f8(Str::lit("0"));
        let a1995 = f473(v1);
        append_bytes(a1994, a1995)
    }
}

pub fn f474(v0: u32) -> Bytes {
    let a1998 = {
        let a1996 = v0;
        let a1997 = 8u32;
        fixed_u32::bit_and(a1996, a1997)
    };
    let a1999 = 0u32;
    if equal(a1998, a1999) {
        let a2002 = {
            let a2000 = v0;
            let a2001 = 4u32;
            fixed_u32::bit_and(a2000, a2001)
        };
        let a2003 = 0u32;
        if equal(a2002, a2003) {
            let a2006 = {
                let a2004 = v0;
                let a2005 = 2u32;
                fixed_u32::bit_and(a2004, a2005)
            };
            let a2007 = 0u32;
            if equal(a2006, a2007) {
                let a2010 = {
                    let a2008 = v0;
                    let a2009 = 1u32;
                    fixed_u32::bit_and(a2008, a2009)
                };
                let a2011 = 0u32;
                if equal(a2010, a2011) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2014 = {
                    let a2012 = v0;
                    let a2013 = 1u32;
                    fixed_u32::bit_and(a2012, a2013)
                };
                let a2015 = 0u32;
                if equal(a2014, a2015) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2018 = {
                let a2016 = v0;
                let a2017 = 2u32;
                fixed_u32::bit_and(a2016, a2017)
            };
            let a2019 = 0u32;
            if equal(a2018, a2019) {
                let a2022 = {
                    let a2020 = v0;
                    let a2021 = 1u32;
                    fixed_u32::bit_and(a2020, a2021)
                };
                let a2023 = 0u32;
                if equal(a2022, a2023) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2026 = {
                    let a2024 = v0;
                    let a2025 = 1u32;
                    fixed_u32::bit_and(a2024, a2025)
                };
                let a2027 = 0u32;
                if equal(a2026, a2027) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2030 = {
            let a2028 = v0;
            let a2029 = 4u32;
            fixed_u32::bit_and(a2028, a2029)
        };
        let a2031 = 0u32;
        if equal(a2030, a2031) {
            let a2034 = {
                let a2032 = v0;
                let a2033 = 2u32;
                fixed_u32::bit_and(a2032, a2033)
            };
            let a2035 = 0u32;
            if equal(a2034, a2035) {
                let a2038 = {
                    let a2036 = v0;
                    let a2037 = 1u32;
                    fixed_u32::bit_and(a2036, a2037)
                };
                let a2039 = 0u32;
                if equal(a2038, a2039) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2042 = {
                    let a2040 = v0;
                    let a2041 = 1u32;
                    fixed_u32::bit_and(a2040, a2041)
                };
                let a2043 = 0u32;
                if equal(a2042, a2043) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2046 = {
                let a2044 = v0;
                let a2045 = 2u32;
                fixed_u32::bit_and(a2044, a2045)
            };
            let a2047 = 0u32;
            if equal(a2046, a2047) {
                let a2050 = {
                    let a2048 = v0;
                    let a2049 = 1u32;
                    fixed_u32::bit_and(a2048, a2049)
                };
                let a2051 = 0u32;
                if equal(a2050, a2051) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2054 = {
                    let a2052 = v0;
                    let a2053 = 1u32;
                    fixed_u32::bit_and(a2052, a2053)
                };
                let a2055 = 0u32;
                if equal(a2054, a2055) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f475(v0: u32, v1: u32) -> u32 {
    let m2058 = {
        let a2056 = v0;
        let a2057 = v1;
        fixed_u32::shift_right(a2056, a2057)
    };
    match m2058 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f476(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f516(Fn18::F517, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2059, _, _, _, _, _, _, _) => {
            h2059
        }
    }, match v0.clone() {
        Adt24::C0(_, h2060, _, _, _, _, _, _) => {
            h2060
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2061, _, _, _, _, _) => {
            h2061
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2062, _, _, _, _) => {
            h2062
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2063, _, _, _) => {
            h2063
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2064, _, _) => {
            h2064
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2065, _) => {
            h2065
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2066) => {
            h2066
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f518(match v0.clone() {
        Adt24::C0(h2067, _, _, _, _, _, _, _) => {
            h2067
        }
    }, match v2.clone() {
        Adt27::C0(h2068, _, _, _, _, _, _, _, _, _) => {
            h2068
        }
    }), f518(match v0.clone() {
        Adt24::C0(_, h2069, _, _, _, _, _, _) => {
            h2069
        }
    }, match v2.clone() {
        Adt27::C0(_, h2070, _, _, _, _, _, _, _, _) => {
            h2070
        }
    }), f518(match v0.clone() {
        Adt24::C0(_, _, h2071, _, _, _, _, _) => {
            h2071
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2072, _, _, _, _, _, _, _) => {
            h2072
        }
    }), f518(match v0.clone() {
        Adt24::C0(_, _, _, h2073, _, _, _, _) => {
            h2073
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2074, _, _, _, _, _, _) => {
            h2074
        }
    }), f518(match v0.clone() {
        Adt24::C0(_, _, _, _, h2075, _, _, _) => {
            h2075
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2076, _, _, _, _, _) => {
            h2076
        }
    }), f518(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2077, _, _) => {
            h2077
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2078, _, _, _, _) => {
            h2078
        }
    }), f518(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2079, _) => {
            h2079
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2080, _, _, _) => {
            h2080
        }
    }), f518(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2081) => {
            h2081
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2082, _, _) => {
            h2082
        }
    })))
}

pub fn f477(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f519(v0.clone(), v1, v2, v3)?, f519(v0.clone(), v1, v2, {
        let a2083 = v3;
        let a2084 = 4u64;
        nat_add(a2083, a2084)?
    })?, f519(v0.clone(), v1, v2, {
        let a2085 = v3;
        let a2086 = 8u64;
        nat_add(a2085, a2086)?
    })?, f519(v0.clone(), v1, v2, {
        let a2087 = v3;
        let a2088 = 12u64;
        nat_add(a2087, a2088)?
    })?, f519(v0.clone(), v1, v2, {
        let a2089 = v3;
        let a2090 = 16u64;
        nat_add(a2089, a2090)?
    })?, f519(v0.clone(), v1, v2, {
        let a2091 = v3;
        let a2092 = 20u64;
        nat_add(a2091, a2092)?
    })?, f519(v0.clone(), v1, v2, {
        let a2093 = v3;
        let a2094 = 24u64;
        nat_add(a2093, a2094)?
    })?, f519(v0.clone(), v1, v2, {
        let a2095 = v3;
        let a2096 = 28u64;
        nat_add(a2095, a2096)?
    })?, f519(v0.clone(), v1, v2, {
        let a2097 = v3;
        let a2098 = 32u64;
        nat_add(a2097, a2098)?
    })?, f519(v0.clone(), v1, v2, {
        let a2099 = v3;
        let a2100 = 36u64;
        nat_add(a2099, a2100)?
    })?, f519(v0.clone(), v1, v2, {
        let a2101 = v3;
        let a2102 = 40u64;
        nat_add(a2101, a2102)?
    })?, f519(v0.clone(), v1, v2, {
        let a2103 = v3;
        let a2104 = 44u64;
        nat_add(a2103, a2104)?
    })?, f519(v0.clone(), v1, v2, {
        let a2105 = v3;
        let a2106 = 48u64;
        nat_add(a2105, a2106)?
    })?, f519(v0.clone(), v1, v2, {
        let a2107 = v3;
        let a2108 = 52u64;
        nat_add(a2107, a2108)?
    })?, f519(v0.clone(), v1, v2, {
        let a2109 = v3;
        let a2110 = 56u64;
        nat_add(a2109, a2110)?
    })?, f519(v0, v1, v2, {
        let a2111 = v3;
        let a2112 = 60u64;
        nat_add(a2111, a2112)?
    })?))
}

pub fn f478(v0: Adt17) -> R<Result<Bytes, Bytes>> {
    let m2113 = f520(v0.clone());
    match m2113 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f331(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f479(v0: Bytes, v1: Bytes, v2: Bytes, v3: Adt17) -> Adt17 {
    Adt17::C7(std::rc::Rc::new(Adt17::C10(f8(Str::lit("citation")), std::rc::Rc::new(Adt17::C6(std::rc::Rc::new(f521(List::cons(f8(Str::lit("std")), List::cons(v0, List::cons(v1, List::<Bytes>::nil()))))))), std::rc::Rc::new(Adt17::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt17::C5(v2)), std::rc::Rc::new(Adt17::C10(f8(Str::lit("pointer")), std::rc::Rc::new(Adt17::C5(Bytes::lit(&[]))), std::rc::Rc::new(Adt17::C10(f8(Str::lit("value")), std::rc::Rc::new(v3), std::rc::Rc::new(Adt17::C8))))))))))
}

pub fn f480(v0: u64, v1: Adt17, v2: Adt17, v3: Adt17, v4: Bytes) -> R<Option<Bytes>> {
    let m2114 = v0;
    if m2114 == 0 {
        Ok(Some(f7(List::cons(f8(Str::lit("the schema's references do not end before the instance at \"")), List::cons(v4, List::cons(f8(Str::lit("\" is checked (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
    } else {
        let v5 = m2114 - 1;
        let m2115 = v2.clone();
        match m2115 {
            Adt17::C0 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C1(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C2(v6) => {
                if v6 {
                    Ok(None::<Bytes>)
                } else {
                    Ok(f522(v4, f8(Str::lit("is refused by the false schema (JSON Schema 2020-12 Core section 4.3.2)"))))
                }
            }
            Adt17::C3(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C4(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C5(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C6(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C7(r2129) => {
                let v7 = (*r2129).clone();
                let m2116 = f523(f506(v7));
                match m2116 {
                    None => {
                        Ok(f524(f525(v2.clone(), v3.clone(), v4.clone())?, f524({
                            let m2117 = f332(f238(v2.clone(), f8(Str::lit("$ref"))));
                            match m2117 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v9) => {
                                    let m2118 = f526(v1.clone(), v9)?;
                                    match m2118 {
                                        Ok(v11) => {
                                            f480(v5, v1.clone(), v11, v3.clone(), v4.clone())?
                                        }
                                        Err(v10) => {
                                            Some(v10)
                                        }
                                    }
                                }
                            }
                        }, f524(f527(v5, v1.clone(), f243(f238(v2.clone(), f8(Str::lit("allOf")))), v3.clone(), v4.clone())?, f524({
                            let m2119 = f238(v2.clone(), f8(Str::lit("anyOf")));
                            match m2119 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v12) => {
                                    let a2120 = 0u64;
                                    let a2121 = f528(v5, v1.clone(), f243(Some(v12)), v3.clone(), v4.clone())?;
                                    if nat_lt(a2120, a2121) {
                                        None::<Bytes>
                                    } else {
                                        f522(v4.clone(), f8(Str::lit("matches no anyOf branch (JSON Schema 2020-12 Core section 10.2.1.2)")))
                                    }
                                }
                            }
                        }, f524({
                            let m2122 = f238(v2.clone(), f8(Str::lit("oneOf")));
                            match m2122 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v13) => {
                                    let a2123 = f528(v5, v1.clone(), f243(Some(v13)), v3.clone(), v4.clone())?;
                                    let a2124 = 1u64;
                                    if nat_eq(a2123, a2124) {
                                        None::<Bytes>
                                    } else {
                                        f522(v4.clone(), f8(Str::lit("does not match exactly one oneOf branch (JSON Schema 2020-12 Core section 10.2.1.3)")))
                                    }
                                }
                            }
                        }, {
                            let m2125 = v3.clone();
                            match m2125 {
                                Adt17::C0 => {
                                    None::<Bytes>
                                }
                                Adt17::C1(_, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C2(_) => {
                                    None::<Bytes>
                                }
                                Adt17::C3(_, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C4(_, _, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C5(_) => {
                                    None::<Bytes>
                                }
                                Adt17::C6(r2127) => {
                                    let v15 = (*r2127).clone();
                                    let m2126 = f238(v2, f8(Str::lit("items")));
                                    match m2126 {
                                        None => {
                                            None::<Bytes>
                                        }
                                        Some(v16) => {
                                            f531(v5, v1, v16, f333(v15), v4, 0u64)?
                                        }
                                    }
                                }
                                Adt17::C7(r2128) => {
                                    let v14 = (*r2128).clone();
                                    f524(f529(v5, v1.clone(), f465(f238(v2.clone(), f8(Str::lit("properties")))), v3, v4.clone())?, f530(v5, v1, v2, f506(v14), v4)?)
                                }
                                Adt17::C8 => {
                                    None::<Bytes>
                                }
                                Adt17::C9(_, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C10(_, _, _) => {
                                    None::<Bytes>
                                }
                            }
                        }))))))
                    }
                    Some(v8) => {
                        Ok(Some(f7(List::cons(f8(Str::lit("the keyword ")), List::cons(v8, List::cons(f8(Str::lit(" is not checked by conforms, so the instance is not admitted (JSON Schema 2020-12 Core section 4.3.1)")), List::<Bytes>::nil()))))))
                    }
                }
            }
            Adt17::C8 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C9(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C10(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
        }
    }
}

pub fn f481(v0: Adt17) -> R<Bytes> {
    f331(v0)
}

pub fn f482(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2130 = v0;
    match m2130.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f302(v1)?, f482(v2)?))
        }
    }
}

pub fn f483(p2133: List<Bytes>, v1: Bytes) -> bool {
    let m2132 = p2133;
    {
        let mut m2132 = m2132;
        loop {
            let step = {
                let v0 = m2132;
                let m2131 = v0;
                match m2131.uncons() {
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
                Ok(next) => { m2132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: Fn14, p2138: u64, p2139: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2137 = (p2138, p2139);
    {
        let mut m2137 = m2137;
        loop {
            let step = {
                let (v1, v2) = m2137;
                let m2134 = v1;
                if m2134 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2134 - 1;
                    let m2136 = {
                        let c2135: Fn14 = v0.clone();
                        c2135.apply(v2.clone())?
                    };
                    match m2136 {
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
                Ok(next) => { m2137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f532(v0, v1)
}

pub fn f486(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f533(v0, v1)
}

pub fn f487(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f534(v0, v1)
}

pub fn f488(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2140 = v0.clone();
        length_bytes(a2140)
    };
    let a2141 = 0u64;
    let a2142 = v1;
    if nat_lt(a2141, a2142) {
        let a2143 = f535(v0.clone())?;
        let a2144 = v1;
        if nat_eq(a2143, a2144) {
            let a2145 = 1u64;
            let a2146 = v1;
            if nat_lt(a2145, a2146) {
                let m2149 = {
                    let a2147 = v0;
                    let a2148 = 0u64;
                    index_bytes(a2147, a2148)
                };
                match m2149 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2150 = v2;
                        let a2151 = 48u8;
                        Ok(!equal(a2150, a2151))
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

pub fn f489(v0: Fn15, p2156: u64, p2157: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2155 = (p2156, p2157);
    {
        let mut m2155 = m2155;
        loop {
            let step = {
                let (v1, v2) = m2155;
                let m2152 = v1;
                if m2152 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2152 - 1;
                    let m2154 = {
                        let c2153: Fn15 = v0.clone();
                        c2153.apply(v2)?
                    };
                    match m2154 {
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
                Ok(next) => { m2155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f536(v0, v1)
}

pub fn f491(v0: Bytes) -> R<u64> {
    let (h2158, _) = f363(Fn6::F537(v0.clone()), {
        let a2160 = {
            let a2159 = v0;
            length_bytes(a2159)
        };
        let a2161 = 1u64;
        nat_add(a2160, a2161)?
    }, 0u64)?;
    Ok(h2158)
}

pub fn f492(v0: Bytes, v1: u64) -> Bytes {
    let m2165 = {
        let a2162 = v0;
        let a2163 = v1;
        let a2164 = 1u64;
        slice_bytes(a2162, a2163, a2164)
    };
    match m2165 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f493(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f538(v1, v0, v2)
}

pub fn f494(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2210 = {
            let a2186 = {
                let a2174 = {
                    let a2168 = {
                        let a2166 = v3;
                        let a2167 = 128u8;
                        fixed_u8::bit_and(a2166, a2167)
                    };
                    let a2169 = 128u8;
                    if equal(a2168, a2169) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2175 = {
                    let a2172 = {
                        let a2170 = v3;
                        let a2171 = 64u8;
                        fixed_u8::bit_and(a2170, a2171)
                    };
                    let a2173 = 64u8;
                    if equal(a2172, a2173) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2174, a2175)?
            };
            let a2187 = {
                let a2184 = {
                    let a2178 = {
                        let a2176 = v3;
                        let a2177 = 32u8;
                        fixed_u8::bit_and(a2176, a2177)
                    };
                    let a2179 = 32u8;
                    if equal(a2178, a2179) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2185 = {
                    let a2182 = {
                        let a2180 = v3;
                        let a2181 = 16u8;
                        fixed_u8::bit_and(a2180, a2181)
                    };
                    let a2183 = 16u8;
                    if equal(a2182, a2183) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2184, a2185)?
            };
            nat_add(a2186, a2187)?
        };
        let a2211 = {
            let a2208 = {
                let a2196 = {
                    let a2190 = {
                        let a2188 = v3;
                        let a2189 = 8u8;
                        fixed_u8::bit_and(a2188, a2189)
                    };
                    let a2191 = 8u8;
                    if equal(a2190, a2191) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2197 = {
                    let a2194 = {
                        let a2192 = v3;
                        let a2193 = 4u8;
                        fixed_u8::bit_and(a2192, a2193)
                    };
                    let a2195 = 4u8;
                    if equal(a2194, a2195) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2196, a2197)?
            };
            let a2209 = {
                let a2206 = {
                    let a2200 = {
                        let a2198 = v3;
                        let a2199 = 2u8;
                        fixed_u8::bit_and(a2198, a2199)
                    };
                    let a2201 = 2u8;
                    if equal(a2200, a2201) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2207 = {
                    let a2204 = {
                        let a2202 = v3;
                        let a2203 = 1u8;
                        fixed_u8::bit_and(a2202, a2203)
                    };
                    let a2205 = 1u8;
                    if equal(a2204, a2205) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2206, a2207)?
            };
            nat_add(a2208, a2209)?
        };
        nat_add(a2210, a2211)?
    };
    let a2212 = v4;
    let a2213 = 34u64;
    if nat_eq(a2212, a2213) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2214 = v4;
        let a2215 = 92u64;
        if nat_eq(a2214, a2215) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2216 = v4;
            let a2217 = 8u64;
            if nat_eq(a2216, a2217) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2218 = v4;
                let a2219 = 12u64;
                if nat_eq(a2218, a2219) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2220 = v4;
                    let a2221 = 10u64;
                    if nat_eq(a2220, a2221) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2222 = v4;
                        let a2223 = 13u64;
                        if nat_eq(a2222, a2223) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2224 = v4;
                            let a2225 = 9u64;
                            if nat_eq(a2224, a2225) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2226 = v4;
                                let a2227 = 32u64;
                                if nat_lt(a2226, a2227) {
                                    let a2228 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2229 = f539(v2)?;
                                    Ok(append_bytes(a2228, a2229))
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

pub fn f495(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f540(Fn19::F541, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f496(v0: Fn16, p2233: List<List<(Bytes, Bytes)>>, p2234: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2232 = (p2233, p2234);
    {
        let mut m2232 = m2232;
        loop {
            let step = {
                let (v1, v2) = m2232;
                let m2230 = v2;
                match m2230.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2231: Fn16 = v0.clone();
                            c2231.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f498(v0: Fn17, p2239: u64, p2240: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2238 = (p2239, p2240);
    {
        let mut m2238 = m2238;
        loop {
            let step = {
                let (v1, v2) = m2238;
                let m2235 = v1;
                if m2235 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2235 - 1;
                    let m2237 = {
                        let c2236: Fn17 = v0.clone();
                        c2236.apply(v2.clone())?
                    };
                    match m2237 {
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
                Ok(next) => { m2238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f499(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f542(v0)
}

pub fn f500(v0: u8) -> bool {
    let a2241 = v0;
    let a2242 = 32u8;
    if if if equal(a2241, a2242) {
        true
    } else {
        let a2243 = v0;
        let a2244 = 9u8;
        equal(a2243, a2244)
    } {
        true
    } else {
        let a2245 = v0;
        let a2246 = 10u8;
        equal(a2245, a2246)
    } {
        true
    } else {
        let a2247 = v0;
        let a2248 = 12u8;
        if equal(a2247, a2248) {
            true
        } else {
            let a2249 = v0;
            let a2250 = 13u8;
            equal(a2249, a2250)
        }
    }
}

pub fn f501(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f385({
        let (_, h2251) = {
            let (h2252, _) = f543(Fn20::F544, {
                let a2254 = {
                    let a2253 = v0.clone();
                    length_list(a2253)
                };
                let a2255 = 1u64;
                nat_add(a2254, a2255)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2252
        };
        h2251
    }))
}

pub fn f502(v0: Bytes, v1: Bytes, p2268: u64, p2269: u64) -> R<bool> {
    let m2267 = (p2268, p2269);
    {
        let mut m2267 = m2267;
        loop {
            let step = {
                let (v2, v3) = m2267;
                let m2256 = v3;
                if m2256 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m2256 - 1;
                    let m2259 = {
                        let a2257 = v1.clone();
                        let a2258 = v2;
                        index_bytes(a2257, a2258)
                    };
                    match m2259 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m2262 = {
                                let a2260 = v0.clone();
                                let a2261 = v2;
                                index_bytes(a2260, a2261)
                            };
                            match m2262 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a2263 = v5;
                                    let a2264 = v6;
                                    if equal(a2263, a2264) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a2265 = v2;
                                            let a2266 = 1u64;
                                            nat_add(a2265, a2266)?
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
                Ok(next) => { m2267 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f504(v0: List<Adt13>) -> List<Bytes> {
    let m2270 = v0;
    match m2270.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2271 = f79(v1, f8(Str::lit("std")));
            match m2271 {
                None => {
                    f504(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f504(v2))
                }
            }
        }
    }
}

pub fn f505(v0: List<Adt13>) -> List<Bytes> {
    let m2272 = v0;
    match m2272.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2273 = f78(f79(v1, f8(Str::lit("req"))));
            match m2273 {
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

pub fn f506(v0: Adt17) -> List<(Bytes, Adt17)> {
    let m2274 = v0;
    match m2274 {
        Adt17::C0 => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C1(_, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C2(_) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C3(_, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C4(_, _, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C5(_) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C6(_) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C7(r2275) => {
            let v4 = (*r2275).clone();
            f506(v4)
        }
        Adt17::C8 => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C9(_, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C10(v1, r2276, r2277) => {
            let v2 = (*r2276).clone();
            let v3 = (*r2277).clone();
            List::cons((v1, v2), f506(v3))
        }
    }
}

pub fn f507(p2282: List<(Bytes, Adt17)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt17)>> {
    let m2281 = p2282;
    {
        let mut m2281 = m2281;
        loop {
            let step = {
                let v0 = m2281;
                let m2278 = v0;
                match m2278.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, R<Option<(Bytes, Adt17)>>>(Ok(None::<(Bytes, Adt17)>))
                    }
                    Some((v3, v4)) => {
                        if f545(f546({
                            let (_, h2279) = v3.clone();
                            h2279
                        }, List::<Bytes>::nil())?, f509({
                            let (h2280, _) = v3.clone();
                            h2280
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt17)>, R<Option<(Bytes, Adt17)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt17)>, R<Option<(Bytes, Adt17)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f508(v0: List<(Bytes, Adt17)>, v1: Bytes) -> List<(Bytes, Adt17)> {
    let m2283 = v0;
    match m2283.uncons() {
        None => {
            List::<(Bytes, Adt17)>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h2284, _) = v2.clone();
                h2284
            }, v1.clone()) {
                f508(v3, v1)
            } else {
                List::cons(v2, f508(v3, v1))
            }
        }
    }
}

pub fn f509(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f326(v0)?;
    if f41(v1.clone(), f547()) {
        f548(v1)
    } else {
        Ok(v1)
    }
}

pub fn f510() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f511() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f512(v0: Bytes, v1: Adt17) -> R<Result<Adt28, Bytes>> {
    f549(v0, f550(v1))
}

pub fn f513(v0: Adt28) -> List<Adt25> {
    let m2285 = v0;
    match m2285 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f514(v0: Adt28) -> Bytes {
    let m2286 = v0;
    match m2286 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f515(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2287 = v0;
    match m2287.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f515(v3, false))
            } else {
                f515(v3, true)
            }
        }
    }
}

pub fn f516(v0: Fn18, p2291: u64, p2292: Adt27) -> R<Adt27> {
    let m2290 = (p2291, p2292);
    {
        let mut m2290 = m2290;
        loop {
            let step = {
                let (v1, v2) = m2290;
                let m2288 = v1;
                if m2288 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2288 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2289: Fn18 = v0.clone();
                        c2289.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2290 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: Adt27) -> R<Adt27> {
    f551(v0)
}

pub fn f518(v0: u32, v1: u32) -> u32 {
    let m2295 = {
        let a2293 = f552(v0);
        let a2294 = f552(v1);
        fixed_u64::checked_add(a2293, a2294)
    };
    match m2295 {
        None => {
            0u32
        }
        Some(v2) => {
            f553(v2)
        }
    }
}

pub fn f519(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2298 = {
        let a2296 = v3;
        let a2297 = 8u64;
        nat_add(a2296, a2297)?
    };
    let a2299 = v2;
    if nat_eq(a2298, a2299) {
        Ok(f554({
            let a2302 = {
                let a2300 = v1;
                let a2301 = 8u64;
                nat_mul(a2300, a2301)?
            };
            let a2303 = 4294967296u64;
            let a2304 = 0u64;
            nat_quot(a2302, a2303, a2304)
        }))
    } else {
        let a2307 = {
            let a2305 = v3;
            let a2306 = 4u64;
            nat_add(a2305, a2306)?
        };
        let a2308 = v2;
        if nat_eq(a2307, a2308) {
            Ok(f554({
                let a2311 = {
                    let a2309 = v1;
                    let a2310 = 8u64;
                    nat_mul(a2309, a2310)?
                };
                let a2312 = 4294967296u64;
                let a2313 = 0u64;
                nat_rem(a2311, a2312, a2313)
            }))
        } else {
            Ok(f555(f556(v0.clone(), v1, v3), f556(v0.clone(), v1, {
                let a2314 = v3;
                let a2315 = 1u64;
                nat_add(a2314, a2315)?
            }), f556(v0.clone(), v1, {
                let a2316 = v3;
                let a2317 = 2u64;
                nat_add(a2316, a2317)?
            }), f556(v0, v1, {
                let a2318 = v3;
                let a2319 = 3u64;
                nat_add(a2318, a2319)?
            })))
        }
    }
}

pub fn f520(v0: Adt17) -> Option<Bytes> {
    let m2320 = v0;
    match m2320 {
        Adt17::C0 => {
            None::<Bytes>
        }
        Adt17::C1(v1, _) => {
            Some(v1)
        }
        Adt17::C2(_) => {
            None::<Bytes>
        }
        Adt17::C3(_, _) => {
            None::<Bytes>
        }
        Adt17::C4(_, _, _) => {
            None::<Bytes>
        }
        Adt17::C5(_) => {
            None::<Bytes>
        }
        Adt17::C6(r2323) => {
            let v3 = (*r2323).clone();
            f520(v3)
        }
        Adt17::C7(r2324) => {
            let v4 = (*r2324).clone();
            f520(v4)
        }
        Adt17::C8 => {
            None::<Bytes>
        }
        Adt17::C9(r2325, r2326) => {
            let v5 = (*r2325).clone();
            let v6 = (*r2326).clone();
            let m2321 = f520(v5);
            match m2321 {
                None => {
                    f520(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt17::C10(_, r2327, r2328) => {
            let v9 = (*r2327).clone();
            let v10 = (*r2328).clone();
            let m2322 = f520(v9);
            match m2322 {
                None => {
                    f520(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f521(v0: List<Bytes>) -> Adt17 {
    let m2329 = v0;
    match m2329.uncons() {
        None => {
            Adt17::C8
        }
        Some((v1, v2)) => {
            Adt17::C9(std::rc::Rc::new(Adt17::C5(v1)), std::rc::Rc::new(f521(v2)))
        }
    }
}

pub fn f522(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    Some(f7(List::cons(f8(Str::lit("the instance at \"")), List::cons(v0, List::cons(f8(Str::lit("\" ")), List::cons(v1, List::<Bytes>::nil()))))))
}

pub fn f523(p2334: List<(Bytes, Adt17)>) -> Option<Bytes> {
    let m2333 = p2334;
    {
        let mut m2333 = m2333;
        loop {
            let step = {
                let v0 = m2333;
                let m2330 = v0;
                match m2330.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h2331, _) = v1.clone();
                            h2331
                        }, f557()) {
                            Ok::<List<(Bytes, Adt17)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt17)>, Option<Bytes>>(Some({
                                let (h2332, _) = v1;
                                h2332
                            }))
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

pub fn f524(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2335 = v0;
    match m2335 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f525(v0: Adt17, v1: Adt17, v2: Bytes) -> R<Option<Bytes>> {
    let v3: Bytes = f558(v1.clone());
    let v7: Option<Bytes> = {
        let m2336 = f238(v0.clone(), f8(Str::lit("type")));
        match m2336 {
            None => {
                None::<Bytes>
            }
            Some(v4) => {
                let v6: List<Bytes> = {
                    let m2337 = f332(Some(v4.clone()));
                    match m2337 {
                        None => {
                            f242(f243(Some(v4)))
                        }
                        Some(v5) => {
                            List::cons(v5, List::<Bytes>::nil())
                        }
                    }
                };
                if f559(v6.clone(), v1.clone()) {
                    None::<Bytes>
                } else {
                    f522(v2.clone(), f7(List::cons(f8(Str::lit("is ")), List::cons(v3, List::cons(f8(Str::lit(", not of the type ")), List::cons(f342(v6, f8(Str::lit(" or "))), List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.1.1)")), List::<Bytes>::nil())))))))
                }
            }
        }
    };
    let v9: Option<Bytes> = {
        let m2338 = f238(v0.clone(), f8(Str::lit("enum")));
        match m2338 {
            None => {
                None::<Bytes>
            }
            Some(v8) => {
                if f560(f243(Some(v8)), v1.clone())? {
                    None::<Bytes>
                } else {
                    f522(v2.clone(), f8(Str::lit("is none of the enum values (JSON Schema 2020-12 Validation section 6.1.2)")))
                }
            }
        }
    };
    let v11: Option<Bytes> = {
        let m2339 = f238(v0.clone(), f8(Str::lit("const")));
        match m2339 {
            None => {
                None::<Bytes>
            }
            Some(v10) => {
                if f561(false, v10, v1.clone())? {
                    None::<Bytes>
                } else {
                    f522(v2.clone(), f8(Str::lit("is not the const value (JSON Schema 2020-12 Validation section 6.1.3)")))
                }
            }
        }
    };
    let v41: Option<Bytes> = {
        let m2340 = v1.clone();
        match m2340 {
            Adt17::C0 => {
                None::<Bytes>
            }
            Adt17::C1(v28, _) => {
                f522(v2.clone(), f7(List::cons(f8(Str::lit("is tagged ")), List::cons(v28, List::cons(f8(Str::lit(" and has no JSON type (RFC 8259 section 3)")), List::<Bytes>::nil())))))
            }
            Adt17::C2(_) => {
                None::<Bytes>
            }
            Adt17::C3(_, _) => {
                None::<Bytes>
            }
            Adt17::C4(_, _, _) => {
                None::<Bytes>
            }
            Adt17::C5(v12) => {
                let m2341 = f562(v12.clone())?;
                match m2341 {
                    None => {
                        f522(v2.clone(), f8(Str::lit("is not well-formed UTF-8 (RFC 3629 section 4)")))
                    }
                    Some(v13) => {
                        let v14: u64 = f563(v13)?;
                        f524({
                            let m2342 = f564(f238(v0.clone(), f8(Str::lit("minLength"))));
                            match m2342 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v15) => {
                                    let a2343 = v14;
                                    let a2344 = v15;
                                    if nat_lt(a2343, a2344) {
                                        f522(v2.clone(), f8(Str::lit("is shorter than minLength (JSON Schema 2020-12 Validation section 6.3.2)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, f524({
                            let m2345 = f564(f238(v0.clone(), f8(Str::lit("maxLength"))));
                            match m2345 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v16) => {
                                    let a2346 = v16;
                                    let a2347 = v14;
                                    if nat_lt(a2346, a2347) {
                                        f522(v2.clone(), f8(Str::lit("is longer than maxLength (JSON Schema 2020-12 Validation section 6.3.1)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, {
                            let m2348 = f332(f238(v0.clone(), f8(Str::lit("pattern"))));
                            match m2348 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v17) => {
                                    let m2349 = f565(v17.clone(), v12)?;
                                    match m2349 {
                                        Ok(v19) => {
                                            if v19 {
                                                None::<Bytes>
                                            } else {
                                                f522(v2.clone(), f7(List::cons(f8(Str::lit("does not match the pattern ")), List::cons(v17, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.3.3)")), List::<Bytes>::nil())))))
                                            }
                                        }
                                        Err(v18) => {
                                            Some(v18)
                                        }
                                    }
                                }
                            }
                        }))
                    }
                }
            }
            Adt17::C6(r2357) => {
                let v20 = (*r2357).clone();
                let v21: u64 = f566(v20)?;
                f524({
                    let m2350 = f564(f238(v0.clone(), f8(Str::lit("minItems"))));
                    match m2350 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v22) => {
                            let a2351 = v21;
                            let a2352 = v22;
                            if nat_lt(a2351, a2352) {
                                f522(v2.clone(), f8(Str::lit("has fewer items than minItems (JSON Schema 2020-12 Validation section 6.4.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2353 = f564(f238(v0.clone(), f8(Str::lit("maxItems"))));
                    match m2353 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v23) => {
                            let a2354 = v23;
                            let a2355 = v21;
                            if nat_lt(a2354, a2355) {
                                f522(v2.clone(), f8(Str::lit("has more items than maxItems (JSON Schema 2020-12 Validation section 6.4.1)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
            Adt17::C7(r2358) => {
                let v24 = (*r2358).clone();
                let v25: List<Bytes> = f567(f242(f243(f238(v0.clone(), f8(Str::lit("required"))))), f568(f506(v24)));
                let m2356 = v25;
                match m2356.uncons() {
                    None => {
                        None::<Bytes>
                    }
                    Some((v26, _)) => {
                        f522(v2.clone(), f7(List::cons(f8(Str::lit("lacks the required member ")), List::cons(v26, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.5.3)")), List::<Bytes>::nil())))))
                    }
                }
            }
            Adt17::C8 => {
                None::<Bytes>
            }
            Adt17::C9(_, _) => {
                None::<Bytes>
            }
            Adt17::C10(_, _, _) => {
                None::<Bytes>
            }
        }
    };
    let v47: Option<Bytes> = {
        let m2359 = f569(v1);
        match m2359 {
            None => {
                None::<Bytes>
            }
            Some(v42) => {
                f524({
                    let m2361 = {
                        let m2360 = f238(v0.clone(), f8(Str::lit("minimum")));
                        match m2360 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v43) => {
                                f569(v43)
                            }
                        }
                    };
                    match m2361 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v44) => {
                            if f570(v42, v44)? {
                                f522(v2.clone(), f8(Str::lit("is less than minimum (JSON Schema 2020-12 Validation section 6.2.4)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2363 = {
                        let m2362 = f238(v0, f8(Str::lit("maximum")));
                        match m2362 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v45) => {
                                f569(v45)
                            }
                        }
                    };
                    match m2363 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v46) => {
                            if f570(v46, v42)? {
                                f522(v2, f8(Str::lit("is greater than maximum (JSON Schema 2020-12 Validation section 6.2.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
        }
    };
    Ok(f524(v7, f524(v9, f524(v11, f524(v41, v47)))))
}

pub fn f526(v0: Adt17, v1: Bytes) -> R<Result<Adt17, Bytes>> {
    let v2: u64 = {
        let a2364 = v1.clone();
        length_bytes(a2364)
    };
    if f28(v1.clone(), f8(Str::lit("#"))) {
        Ok(Ok::<Adt17, Bytes>(v0))
    } else {
        let m2369 = if f72(v1.clone(), f8(Str::lit("#/$defs/"))) {
            Some((f8(Str::lit("$defs")), f35(v1.clone(), 8u64, {
                let a2365 = v2;
                let a2366 = 8u64;
                nat_sub(a2365, a2366)
            })))
        } else {
            if f72(v1.clone(), f8(Str::lit("#/definitions/"))) {
                Some((f8(Str::lit("definitions")), f35(v1.clone(), 14u64, {
                    let a2367 = v2;
                    let a2368 = 14u64;
                    nat_sub(a2367, a2368)
                })))
            } else {
                None::<(Bytes, Bytes)>
            }
        };
        match m2369 {
            None => {
                Ok(Err::<Adt17, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" is not resolved: only #, #/$defs/NAME and #/definitions/NAME in the same document are (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
            }
            Some(v3) => {
                let v4: Bytes = f571({
                    let a2372 = {
                        let a2371 = {
                            let (_, h2370) = v3.clone();
                            h2370
                        };
                        length_bytes(a2371)
                    };
                    let a2373 = 1u64;
                    nat_add(a2372, a2373)?
                }, {
                    let (_, h2374) = v3;
                    h2374
                }, 0u64, Bytes::lit(&[]))?;
                let m2377 = {
                    let m2376 = f238(v0, {
                        let (h2375, _) = v3;
                        h2375
                    });
                    match m2376 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v5) => {
                            f238(v5, v4)
                        }
                    }
                };
                match m2377 {
                    None => {
                        Ok(Err::<Adt17, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" names no definition of this schema (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok(Ok::<Adt17, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f527(v0: u64, v1: Adt17, v2: List<Adt17>, v3: Adt17, v4: Bytes) -> R<Option<Bytes>> {
    let m2378 = v0;
    if m2378 == 0 {
        Ok(Some(f8(Str::lit("the allOf branches do not end (JSON Schema 2020-12 Core section 10.2.1.1)"))))
    } else {
        let v5 = m2378 - 1;
        let m2379 = v2;
        match m2379.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f524(f480(v5, v1.clone(), v6, v3.clone(), v4.clone())?, f527(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f528(v0: u64, v1: Adt17, v2: List<Adt17>, v3: Adt17, v4: Bytes) -> R<u64> {
    let m2380 = v0;
    if m2380 == 0 {
        Ok(0u64)
    } else {
        let v5 = m2380 - 1;
        let m2381 = v2;
        match m2381.uncons() {
            None => {
                Ok(0u64)
            }
            Some((v6, v7)) => {
                let a2383 = {
                    let m2382 = f480(v5, v1.clone(), v6, v3.clone(), v4.clone())?;
                    match m2382 {
                        None => {
                            1u64
                        }
                        Some(_) => {
                            0u64
                        }
                    }
                };
                let a2384 = f528(v5, v1, v7, v3, v4)?;
                nat_add(a2383, a2384)
            }
        }
    }
}

pub fn f529(v0: u64, v1: Adt17, v2: List<(Bytes, Adt17)>, v3: Adt17, v4: Bytes) -> R<Option<Bytes>> {
    let m2385 = v0;
    if m2385 == 0 {
        Ok(Some(f8(Str::lit("the properties do not end (JSON Schema 2020-12 Core section 10.3.2.1)"))))
    } else {
        let v5 = m2385 - 1;
        let m2386 = v2;
        match m2386.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f524({
                    let m2388 = f238(v3.clone(), {
                        let (h2387, _) = v6.clone();
                        h2387
                    });
                    match m2388 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v8) => {
                            f480(v5, v1.clone(), {
                                let (_, h2389) = v6;
                                h2389
                            }, v8, f572(v4.clone(), {
                                let (h2390, _) = v6;
                                h2390
                            })?)?
                        }
                    }
                }, f529(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f530(v0: u64, v1: Adt17, v2: Adt17, v3: List<(Bytes, Adt17)>, v4: Bytes) -> R<Option<Bytes>> {
    let m2391 = v0;
    if m2391 == 0 {
        Ok(Some(f8(Str::lit("the members do not end (JSON Schema 2020-12 Core section 10.3.2.2)"))))
    } else {
        let v5 = m2391 - 1;
        let m2392 = v3;
        match m2392.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                let v8: Bytes = f572(v4.clone(), {
                    let (h2393, _) = v6.clone();
                    h2393
                })?;
                let m2395 = f573(f465(f238(v2.clone(), f8(Str::lit("patternProperties")))), {
                    let (h2394, _) = v6.clone();
                    h2394
                })?;
                match m2395 {
                    Ok(v10) => {
                        Ok(f524(f527(v5, v1.clone(), v10.clone(), {
                            let (_, h2396) = v6.clone();
                            h2396
                        }, v8.clone())?, f524({
                            let m2397 = v10;
                            match m2397.uncons() {
                                None => {
                                    if f41({
                                        let (h2398, _) = v6;
                                        h2398
                                    }, f568(f506({
                                        let m2399 = f238(v2.clone(), f8(Str::lit("properties")));
                                        match m2399 {
                                            None => {
                                                Adt17::C7(std::rc::Rc::new(Adt17::C8))
                                            }
                                            Some(v13) => {
                                                v13
                                            }
                                        }
                                    }))) {
                                        None::<Bytes>
                                    } else {
                                        let m2400 = f238(v2.clone(), f8(Str::lit("additionalProperties")));
                                        match m2400 {
                                            None => {
                                                None::<Bytes>
                                            }
                                            Some(v14) => {
                                                f480(v5, v1.clone(), v14, {
                                                    let (_, h2401) = v6;
                                                    h2401
                                                }, v8)?
                                            }
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    None::<Bytes>
                                }
                            }
                        }, f530(v5, v1, v2, v7, v4)?)))
                    }
                    Err(v9) => {
                        Ok(Some(v9))
                    }
                }
            }
        }
    }
}

pub fn f531(v0: u64, v1: Adt17, v2: Adt17, v3: List<Adt17>, v4: Bytes, v5: u64) -> R<Option<Bytes>> {
    let m2402 = v0;
    if m2402 == 0 {
        Ok(Some(f8(Str::lit("the items do not end (JSON Schema 2020-12 Core section 10.3.1.2)"))))
    } else {
        let v6 = m2402 - 1;
        let m2403 = v3;
        match m2403.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v7, v8)) => {
                Ok(f524(f480(v6, v1.clone(), v2.clone(), v7, f572(v4.clone(), f351(v5)?)?)?, f531(v6, v1, v2, v8, v4, {
                    let a2404 = v5;
                    let a2405 = 1u64;
                    nat_add(a2404, a2405)?
                })?))
            }
        }
    }
}

pub fn f532(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2406, _) = v1.clone();
        h2406
    };
    let v3: Bytes = {
        let (h2407, _) = {
            let (_, h2408) = v1.clone();
            h2408
        };
        h2407
    };
    let a2411 = {
        let (_, h2409) = {
            let (_, h2410) = v1;
            h2410
        };
        h2409
    };
    let a2412 = 0u64;
    if nat_eq(a2411, a2412) {
        let v4: u64 = f346(v0.clone(), v2)?;
        let a2413 = v4;
        let a2414 = 34u64;
        if nat_eq(a2413, a2414) {
            Ok(Some(({
                let a2415 = v2;
                let a2416 = 1u64;
                nat_add(a2415, a2416)?
            }, (v3, 1u64))))
        } else {
            let a2417 = v4;
            let a2418 = 92u64;
            if nat_eq(a2417, a2418) {
                f574(v0, v2, v3)
            } else {
                let a2419 = v4;
                let a2420 = 32u64;
                if if nat_lt(a2419, a2420) {
                    true
                } else {
                    let a2421 = v4;
                    let a2422 = 256u64;
                    nat_eq(a2421, a2422)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2423 = v2;
                        let a2424 = 1u64;
                        nat_add(a2423, a2424)?
                    }, ({
                        let a2425 = v3;
                        let a2426 = f35(v0, v2, 1u64);
                        append_bytes(a2425, a2426)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f533(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f346(v0, v1)?;
    let a2427 = 47u64;
    let a2428 = v2;
    if if nat_lt(a2427, a2428) {
        let a2429 = v2;
        let a2430 = 58u64;
        nat_lt(a2429, a2430)
    } else {
        false
    } {
        Ok(Some({
            let a2431 = v1;
            let a2432 = 1u64;
            nat_add(a2431, a2432)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f534(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2433 = f346(v0, v1)?;
    let a2434 = 48u64;
    if nat_eq(a2433, a2434) {
        Ok(Some({
            let a2435 = v1;
            let a2436 = 1u64;
            nat_add(a2435, a2436)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f535(v0: Bytes) -> R<u64> {
    let (h2437, _) = f363(Fn6::F575(v0.clone()), {
        let a2438 = v0;
        length_bytes(a2438)
    }, 0u64)?;
    Ok(h2437)
}

pub fn f536(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2442 = {
        let a2440 = v0;
        let a2441 = {
            let (h2439, _) = v1;
            h2439
        };
        index_bytes(a2440, a2441)
    };
    match m2442 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2444 = {
                    let (h2443, _) = v1;
                    h2443
                };
                let a2445 = 1u64;
                nat_add(a2444, a2445)?
            }, {
                let a2449 = {
                    let a2447 = {
                        let (_, h2446) = v1;
                        h2446
                    };
                    let a2448 = 10u64;
                    nat_mul(a2447, a2448)?
                };
                let a2450 = f576(v2)?;
                nat_add(a2449, a2450)?
            })))
        }
    }
}

pub fn f537(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f577(v0, v1)
}

pub fn f538(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2451, _) = v2;
        h2451
    };
    let a2452 = v3;
    let a2453 = 0u64;
    if nat_eq(a2452, a2453) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2454 = v3;
            let a2455 = v1;
            let a2456 = 0u64;
            nat_quot(a2454, a2455, a2456)
        }, {
            let a2461 = f492(v0, {
                let a2457 = v3;
                let a2458 = v1;
                let a2459 = 0u64;
                nat_rem(a2457, a2458, a2459)
            });
            let a2462 = {
                let (_, h2460) = v2;
                h2460
            };
            append_bytes(a2461, a2462)
        }))
    }
}

pub fn f539(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2507 = {
            let a2483 = {
                let a2471 = {
                    let a2465 = {
                        let a2463 = v1;
                        let a2464 = 128u8;
                        fixed_u8::bit_and(a2463, a2464)
                    };
                    let a2466 = 128u8;
                    if equal(a2465, a2466) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2472 = {
                    let a2469 = {
                        let a2467 = v1;
                        let a2468 = 64u8;
                        fixed_u8::bit_and(a2467, a2468)
                    };
                    let a2470 = 64u8;
                    if equal(a2469, a2470) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2471, a2472)?
            };
            let a2484 = {
                let a2481 = {
                    let a2475 = {
                        let a2473 = v1;
                        let a2474 = 32u8;
                        fixed_u8::bit_and(a2473, a2474)
                    };
                    let a2476 = 32u8;
                    if equal(a2475, a2476) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2482 = {
                    let a2479 = {
                        let a2477 = v1;
                        let a2478 = 16u8;
                        fixed_u8::bit_and(a2477, a2478)
                    };
                    let a2480 = 16u8;
                    if equal(a2479, a2480) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2481, a2482)?
            };
            nat_add(a2483, a2484)?
        };
        let a2508 = {
            let a2505 = {
                let a2493 = {
                    let a2487 = {
                        let a2485 = v1;
                        let a2486 = 8u8;
                        fixed_u8::bit_and(a2485, a2486)
                    };
                    let a2488 = 8u8;
                    if equal(a2487, a2488) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2494 = {
                    let a2491 = {
                        let a2489 = v1;
                        let a2490 = 4u8;
                        fixed_u8::bit_and(a2489, a2490)
                    };
                    let a2492 = 4u8;
                    if equal(a2491, a2492) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2493, a2494)?
            };
            let a2506 = {
                let a2503 = {
                    let a2497 = {
                        let a2495 = v1;
                        let a2496 = 2u8;
                        fixed_u8::bit_and(a2495, a2496)
                    };
                    let a2498 = 2u8;
                    if equal(a2497, a2498) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2504 = {
                    let a2501 = {
                        let a2499 = v1;
                        let a2500 = 1u8;
                        fixed_u8::bit_and(a2499, a2500)
                    };
                    let a2502 = 1u8;
                    if equal(a2501, a2502) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2503, a2504)?
            };
            nat_add(a2505, a2506)?
        };
        nat_add(a2507, a2508)?
    };
    let a2515 = f492(f578(), {
        let a2509 = v2;
        let a2510 = 16u64;
        let a2511 = 0u64;
        nat_quot(a2509, a2510, a2511)
    });
    let a2516 = f492(f578(), {
        let a2512 = v2;
        let a2513 = 16u64;
        let a2514 = 0u64;
        nat_rem(a2512, a2513, a2514)
    });
    Ok(append_bytes(a2515, a2516))
}

pub fn f540(v0: Fn19, p2520: List<List<(Bytes, Bytes)>>, p2521: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2519 = (p2520, p2521);
    {
        let mut m2519 = m2519;
        loop {
            let step = {
                let (v1, v2) = m2519;
                let m2517 = v2;
                match m2517.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2518: Fn19 = v0.clone();
                            c2518.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f541(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f542(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2523 = {
        let a2522 = v0.clone();
        length_list(a2522)
    };
    let a2524 = 1u64;
    if nat_le(a2523, a2524) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f579(v0)?))
    }
}

pub fn f543(v0: Fn20, p2529: u64, p2530: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2528 = (p2529, p2530);
    {
        let mut m2528 = m2528;
        loop {
            let step = {
                let (v1, v2) = m2528;
                let m2525 = v1;
                if m2525 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2525 - 1;
                    let m2527 = {
                        let c2526: Fn20 = v0.clone();
                        c2526.apply(v2.clone())?
                    };
                    match m2527 {
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
                Ok(next) => { m2528 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f544(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f580(v0)
}

pub fn f545(p2533: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2532 = p2533;
    {
        let mut m2532 = m2532;
        loop {
            let step = {
                let v0 = m2532;
                let m2531 = v0;
                match m2531.uncons() {
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
                Ok(next) => { m2532 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f546(v0: Adt17, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2534 = v0;
    match m2534 {
        Adt17::C0 => {
            Ok(v1)
        }
        Adt17::C1(_, _) => {
            Ok(v1)
        }
        Adt17::C2(_) => {
            Ok(v1)
        }
        Adt17::C3(_, _) => {
            Ok(v1)
        }
        Adt17::C4(_, _, _) => {
            Ok(v1)
        }
        Adt17::C5(_) => {
            Ok(v1)
        }
        Adt17::C6(r2537) => {
            let v2 = (*r2537).clone();
            f546(v2, v1)
        }
        Adt17::C7(r2538) => {
            let v3 = (*r2538).clone();
            f546(v3, v1)
        }
        Adt17::C8 => {
            Ok(v1)
        }
        Adt17::C9(r2539, r2540) => {
            let v4 = (*r2539).clone();
            let v5 = (*r2540).clone();
            f546(v4, f546(v5, v1)?)
        }
        Adt17::C10(v6, r2541, r2542) => {
            let v7 = (*r2541).clone();
            let v8 = (*r2542).clone();
            let v9: List<Bytes> = f546(v7.clone(), f546(v8, v1)?)?;
            if f28(v6, f8(Str::lit("$ref"))) {
                let m2535 = f332(Some(v7));
                match m2535 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2536 = f581(v10)?;
                        match m2536 {
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
    let m2545 = {
        let a2543 = v0.clone();
        let a2544 = 0u64;
        index_bytes(a2543, a2544)
    };
    match m2545 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2551 = {
                let a2546 = f8(Str::lit("u"));
                let a2547 = f582(8u64, f62(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2546, a2547)
            };
            let a2552 = f35(v0.clone(), 1u64, {
                let a2549 = {
                    let a2548 = v0;
                    length_bytes(a2548)
                };
                let a2550 = 1u64;
                nat_sub(a2549, a2550)
            });
            Ok(append_bytes(a2551, a2552))
        }
    }
}

pub fn f549(v0: Bytes, v1: Adt17) -> R<Result<Adt28, Bytes>> {
    let m2553 = f238(v1.clone(), f8(Str::lit("enum")));
    match m2553 {
        None => {
            let m2555 = f238(v1.clone(), f8(Str::lit("oneOf")));
            match m2555 {
                None => {
                    if f587(v1.clone(), v0.clone())? {
                        let m2556 = f588(v0.clone(), f465(f238(v1.clone(), f8(Str::lit("properties")))), f242(f243(f238(v1, f8(Str::lit("required"))))), true)?;
                        match m2556 {
                            Ok(v7) => {
                                Ok(f585(f589(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f342(f590(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f591(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2557 = f465(f238(v1.clone(), f8(Str::lit("properties"))));
                        match m2557.uncons() {
                            None => {
                                if f592(v1.clone()) {
                                    Ok(f585(f593(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2560 = f594(v1, {
                                        let a2558 = v0.clone();
                                        let a2559 = f8(Str::lit("Value"));
                                        append_bytes(a2558, a2559)
                                    })?;
                                    match m2560 {
                                        Ok(v9) => {
                                            Ok(f585(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f514(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f513(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2561 = f588(v0.clone(), f465(f238(v1.clone(), f8(Str::lit("properties")))), f242(f243(f238(v1, f8(Str::lit("required"))))), false)?;
                                match m2561 {
                                    Ok(v13) => {
                                        Ok(f585(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2562 = f590(v13.clone());
                                            match m2562.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f342(f590(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f591(v13)))
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
                    f586(v0, f333(v5))
                }
            }
        }
        Some(v2) => {
            let m2554 = f583(f333(v2), List::<Bytes>::nil())?;
            match m2554 {
                Ok(v4) => {
                    Ok(f585(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f342(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f584(v3, v0))
                }
            }
        }
    }
}

pub fn f550(v0: Adt17) -> Adt17 {
    let v2: Adt17 = {
        let m2563 = f238(v0.clone(), f8(Str::lit("anyOf")));
        match m2563 {
            None => {
                v0
            }
            Some(v1) => {
                if f595(f333(v1.clone())) {
                    f596(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f597(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f598(f333(v1.clone()))
                    } {
                        v0
                    } else {
                        f599(f596(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt17 = {
        let m2564 = f238(v2.clone(), f8(Str::lit("oneOf")));
        match m2564 {
            None => {
                v2
            }
            Some(v3) => {
                if f595(f333(v3)) {
                    f596(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2565 = f238(v4.clone(), f8(Str::lit("allOf")));
    match m2565 {
        None => {
            v4
        }
        Some(v5) => {
            if f600(f333(v5)) {
                f596(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f551(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f601(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2566) => {
            h2566
        }
    });
    let v2: u32 = f518(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2567, _, _) => {
            h2567
        }
    }, f518(f602(match v0.clone() {
        Adt27::C0(_, _, _, _, h2568, _, _, _, _, _) => {
            h2568
        }
    }), f518(f603(match v0.clone() {
        Adt27::C0(_, _, _, _, h2569, _, _, _, _, _) => {
            h2569
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2570, _, _, _, _) => {
            h2570
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2571, _, _, _) => {
            h2571
        }
    }), f518(v1, {
        let m2574 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2573, _) => {
                h2573
            }
        };
        match m2574 {
            Adt26::C0(h2572, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2572
            }
        }
    }))));
    let v3: u32 = f518(f604(match v0.clone() {
        Adt27::C0(h2575, _, _, _, _, _, _, _, _, _) => {
            h2575
        }
    }), f605(match v0.clone() {
        Adt27::C0(h2576, _, _, _, _, _, _, _, _, _) => {
            h2576
        }
    }, match v0.clone() {
        Adt27::C0(_, h2577, _, _, _, _, _, _, _, _) => {
            h2577
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2578, _, _, _, _, _, _, _) => {
            h2578
        }
    }));
    Ok(Adt27::C0(f518(v2, v3), match v0.clone() {
        Adt27::C0(h2579, _, _, _, _, _, _, _, _, _) => {
            h2579
        }
    }, match v0.clone() {
        Adt27::C0(_, h2580, _, _, _, _, _, _, _, _) => {
            h2580
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2581, _, _, _, _, _, _, _) => {
            h2581
        }
    }, f518(match v0.clone() {
        Adt27::C0(_, _, _, h2582, _, _, _, _, _, _) => {
            h2582
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2583, _, _, _, _, _) => {
            h2583
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2584, _, _, _, _) => {
            h2584
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2585, _, _, _) => {
            h2585
        }
    }, f606(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2586, _) => {
            h2586
        }
    }), {
        let a2588 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2587) => {
                h2587
            }
        };
        let a2589 = 1u64;
        nat_add(a2588, a2589)?
    }))
}

pub fn f552(v0: u32) -> u64 {
    let m2591 = {
        let a2590 = v0;
        fixed_u64::convert(i128::from(a2590))
    };
    match m2591 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f553(v0: u64) -> u32 {
    let m2595 = {
        let a2594 = {
            let a2592 = v0;
            let a2593 = 4294967295u64;
            fixed_u64::bit_and(a2592, a2593)
        };
        fixed_u32::convert(i128::from(a2594))
    };
    match m2595 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f554(v0: u64) -> u32 {
    let (_, h2596) = {
        let (_, h2597) = f607(Fn21::F608, 32u64, (v0, (1u32, 0u32)));
        h2597
    };
    h2596
}

pub fn f555(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2602 = f609(v0, 24u32);
    let a2603 = {
        let a2600 = f609(v1, 16u32);
        let a2601 = {
            let a2598 = f609(v2, 8u32);
            let a2599 = v3;
            fixed_u32::bit_or(a2598, a2599)
        };
        fixed_u32::bit_or(a2600, a2601)
    };
    fixed_u32::bit_or(a2602, a2603)
}

pub fn f556(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2604 = v2;
    let a2605 = v1;
    if nat_lt(a2604, a2605) {
        let m2608 = {
            let a2606 = v0;
            let a2607 = v2;
            index_bytes(a2606, a2607)
        };
        match m2608 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2610 = {
                    let a2609 = v3;
                    fixed_u32::convert(i128::from(a2609))
                };
                match m2610 {
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
        let a2611 = v2;
        let a2612 = v1;
        if nat_eq(a2611, a2612) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f557() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("patternProperties")), List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("definitions")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("format")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))
}

pub fn f558(v0: Adt17) -> Bytes {
    let m2613 = v0;
    match m2613 {
        Adt17::C0 => {
            f8(Str::lit("null"))
        }
        Adt17::C1(_, _) => {
            f8(Str::lit("tagged"))
        }
        Adt17::C2(_) => {
            f8(Str::lit("boolean"))
        }
        Adt17::C3(_, _) => {
            f8(Str::lit("integer"))
        }
        Adt17::C4(_, _, _) => {
            f8(Str::lit("number"))
        }
        Adt17::C5(_) => {
            f8(Str::lit("string"))
        }
        Adt17::C6(_) => {
            f8(Str::lit("array"))
        }
        Adt17::C7(_) => {
            f8(Str::lit("object"))
        }
        Adt17::C8 => {
            f8(Str::lit("spine"))
        }
        Adt17::C9(_, _) => {
            f8(Str::lit("spine"))
        }
        Adt17::C10(_, _, _) => {
            f8(Str::lit("spine"))
        }
    }
}

pub fn f559(p2616: List<Bytes>, v1: Adt17) -> bool {
    let m2615 = p2616;
    {
        let mut m2615 = m2615;
        loop {
            let step = {
                let v0 = m2615;
                let m2614 = v0;
                match m2614.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f610(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
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

pub fn f560(p2619: List<Adt17>, v1: Adt17) -> R<bool> {
    let m2618 = p2619;
    {
        let mut m2618 = m2618;
        loop {
            let step = {
                let v0 = m2618;
                let m2617 = v0;
                match m2617.uncons() {
                    None => {
                        Err::<List<Adt17>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f561(false, v2, v1.clone())? {
                            Err::<List<Adt17>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt17>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2618 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(v0: bool, v1: Adt17, v2: Adt17) -> R<bool> {
    let m2620 = v1;
    match m2620 {
        Adt17::C0 => {
            Ok(f28(f481(v2)?, f8(Str::lit("null"))))
        }
        Adt17::C1(_, _) => {
            Ok(false)
        }
        Adt17::C2(v77) => {
            Ok(f28(f481(v2)?, f481(Adt17::C2(v77))?))
        }
        Adt17::C3(v78, v79) => {
            Ok(f28(f481(v2)?, f481(Adt17::C3(v78, v79))?))
        }
        Adt17::C4(v80, v81, v82) => {
            Ok(f28(f481(v2)?, f481(Adt17::C4(v80, v81, v82))?))
        }
        Adt17::C5(v83) => {
            let m2632 = v2;
            match m2632 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(v84) => {
                    Ok(f28(v83, v84))
                }
                Adt17::C6(_) => {
                    Ok(false)
                }
                Adt17::C7(_) => {
                    Ok(false)
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(_, _) => {
                    Ok(false)
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C6(r2633) => {
            let v58 = (*r2633).clone();
            let m2630 = v2;
            match m2630 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(_) => {
                    Ok(false)
                }
                Adt17::C6(r2631) => {
                    let v59 = (*r2631).clone();
                    f561(false, v58, v59)
                }
                Adt17::C7(_) => {
                    Ok(false)
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(_, _) => {
                    Ok(false)
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C7(r2634) => {
            let v41 = (*r2634).clone();
            let m2626 = v2;
            match m2626 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(_) => {
                    Ok(false)
                }
                Adt17::C6(_) => {
                    Ok(false)
                }
                Adt17::C7(r2629) => {
                    let v42 = (*r2629).clone();
                    if f227({
                        let a2627 = f566(v41.clone())?;
                        let a2628 = f566(v42.clone())?;
                        nat_eq(a2627, a2628)
                    }, true) {
                        f561(true, v41, v42)
                    } else {
                        Ok(false)
                    }
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(_, _) => {
                    Ok(false)
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C8 => {
            if v0 {
                Ok(true)
            } else {
                let m2622 = v2;
                match m2622 {
                    Adt17::C0 => {
                        Ok(false)
                    }
                    Adt17::C1(_, _) => {
                        Ok(false)
                    }
                    Adt17::C2(_) => {
                        Ok(false)
                    }
                    Adt17::C3(_, _) => {
                        Ok(false)
                    }
                    Adt17::C4(_, _, _) => {
                        Ok(false)
                    }
                    Adt17::C5(_) => {
                        Ok(false)
                    }
                    Adt17::C6(_) => {
                        Ok(false)
                    }
                    Adt17::C7(_) => {
                        Ok(false)
                    }
                    Adt17::C8 => {
                        Ok(true)
                    }
                    Adt17::C9(_, _) => {
                        Ok(false)
                    }
                    Adt17::C10(_, _, _) => {
                        Ok(false)
                    }
                }
            }
        }
        Adt17::C9(r2635, r2636) => {
            let v23 = (*r2635).clone();
            let v24 = (*r2636).clone();
            let m2623 = v2;
            match m2623 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(_) => {
                    Ok(false)
                }
                Adt17::C6(_) => {
                    Ok(false)
                }
                Adt17::C7(_) => {
                    Ok(false)
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(r2624, r2625) => {
                    let v25 = (*r2624).clone();
                    let v26 = (*r2625).clone();
                    if f561(false, v23, v25)? {
                        f561(false, v24, v26)
                    } else {
                        Ok(false)
                    }
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C10(v3, r2637, r2638) => {
            let v4 = (*r2637).clone();
            let v5 = (*r2638).clone();
            let m2621 = f238(v2.clone(), v3);
            if match m2621 {
                None => {
                    false
                }
                Some(v6) => {
                    f561(false, v4, v6)?
                }
            } {
                f561(true, v5, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f562(v0: Bytes) -> R<Option<List<u64>>> {
    f611({
        let a2640 = {
            let a2639 = v0.clone();
            length_bytes(a2639)
        };
        let a2641 = 1u64;
        nat_add(a2640, a2641)?
    }, v0, 0u64)
}

pub fn f563(v0: List<u64>) -> R<u64> {
    let m2642 = v0;
    match m2642.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a2643 = f563(v2)?;
            let a2644 = 1u64;
            nat_add(a2643, a2644)
        }
    }
}

pub fn f564(v0: Option<Adt17>) -> Option<u64> {
    let m2645 = v0;
    match m2645 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2646 = v1;
            match m2646 {
                Adt17::C0 => {
                    None::<u64>
                }
                Adt17::C1(_, _) => {
                    None::<u64>
                }
                Adt17::C2(_) => {
                    None::<u64>
                }
                Adt17::C3(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt17::C4(_, _, _) => {
                    None::<u64>
                }
                Adt17::C5(_) => {
                    None::<u64>
                }
                Adt17::C6(_) => {
                    None::<u64>
                }
                Adt17::C7(_) => {
                    None::<u64>
                }
                Adt17::C8 => {
                    None::<u64>
                }
                Adt17::C9(_, _) => {
                    None::<u64>
                }
                Adt17::C10(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f565(v0: Bytes, v1: Bytes) -> R<Result<bool, Bytes>> {
    let m2647 = f612(v0)?;
    match m2647 {
        Ok(v3) => {
            let m2648 = f562(v1)?;
            match m2648 {
                None => {
                    Ok(Err::<bool, Bytes>(f8(Str::lit("a string is not well-formed UTF-8 (RFC 3629 section 4)"))))
                }
                Some(v4) => {
                    Ok(Ok::<bool, Bytes>({
                        let (h2649, _) = {
                            let (h2650, _) = v3.clone();
                            h2650
                        };
                        if h2649 {
                            f613({
                                let a2654 = {
                                    let a2652 = f614({
                                        let (_, h2651) = v3.clone();
                                        h2651
                                    })?;
                                    let a2653 = f563(v4.clone())?;
                                    nat_add(a2652, a2653)?
                                };
                                let a2655 = 2u64;
                                nat_add(a2654, a2655)?
                            }, {
                                let (_, h2656) = v3;
                                h2656
                            }, v4, {
                                let (_, h2657) = {
                                    let (h2658, _) = v3;
                                    h2658
                                };
                                h2657
                            })?
                        } else {
                            f615({
                                let (_, h2659) = v3;
                                h2659
                            }, v4, {
                                let (_, h2660) = {
                                    let (h2661, _) = v3;
                                    h2661
                                };
                                h2660
                            })?
                        }
                    }))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<bool, Bytes>(v2))
        }
    }
}

pub fn f566(v0: Adt17) -> R<u64> {
    let m2662 = v0;
    match m2662 {
        Adt17::C0 => {
            Ok(0u64)
        }
        Adt17::C1(_, _) => {
            Ok(0u64)
        }
        Adt17::C2(_) => {
            Ok(0u64)
        }
        Adt17::C3(_, _) => {
            Ok(0u64)
        }
        Adt17::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C5(_) => {
            Ok(0u64)
        }
        Adt17::C6(_) => {
            Ok(0u64)
        }
        Adt17::C7(_) => {
            Ok(0u64)
        }
        Adt17::C8 => {
            Ok(0u64)
        }
        Adt17::C9(_, r2667) => {
            let v5 = (*r2667).clone();
            let a2665 = f566(v5)?;
            let a2666 = 1u64;
            nat_add(a2665, a2666)
        }
        Adt17::C10(_, _, r2668) => {
            let v3 = (*r2668).clone();
            let a2663 = f566(v3)?;
            let a2664 = 1u64;
            nat_add(a2663, a2664)
        }
    }
}

pub fn f567(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2669 = v0;
    match m2669.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f567(v3, v1)
            } else {
                List::cons(v2, f567(v3, v1))
            }
        }
    }
}

pub fn f568(v0: List<(Bytes, Adt17)>) -> List<Bytes> {
    let m2670 = v0;
    match m2670.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2671, _) = v1;
                h2671
            }, f568(v2))
        }
    }
}

pub fn f569(v0: Adt17) -> Option<(bool, (u64, u64))> {
    let m2672 = v0;
    match m2672 {
        Adt17::C0 => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C1(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C2(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C3(v1, v2) => {
            Some((v1, (v2, 0u64)))
        }
        Adt17::C4(v3, v4, v5) => {
            Some((v3, (v4, v5)))
        }
        Adt17::C5(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C6(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C7(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C8 => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C9(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C10(_, _, _) => {
            None::<(bool, (u64, u64))>
        }
    }
}

pub fn f570(v0: (bool, (u64, u64)), v1: (bool, (u64, u64))) -> R<bool> {
    let v2: u64 = {
        let a2677 = {
            let (h2673, _) = {
                let (_, h2674) = v0;
                h2674
            };
            h2673
        };
        let a2678 = f616({
            let (_, h2675) = {
                let (_, h2676) = v1;
                h2676
            };
            h2675
        })?;
        nat_mul(a2677, a2678)?
    };
    let v3: u64 = {
        let a2683 = {
            let (h2679, _) = {
                let (_, h2680) = v1;
                h2680
            };
            h2679
        };
        let a2684 = f616({
            let (_, h2681) = {
                let (_, h2682) = v0;
                h2682
            };
            h2681
        })?;
        nat_mul(a2683, a2684)?
    };
    let (h2685, _) = v0;
    if h2685 {
        let (h2686, _) = v1;
        if h2686 {
            let a2687 = v3;
            let a2688 = v2;
            Ok(nat_lt(a2687, a2688))
        } else {
            Ok(true)
        }
    } else {
        let (h2689, _) = v1;
        if h2689 {
            Ok(false)
        } else {
            let a2690 = v2;
            let a2691 = v3;
            Ok(nat_lt(a2690, a2691))
        }
    }
}

pub fn f571(p2721: u64, v1: Bytes, p2722: u64, p2723: Bytes) -> R<Bytes> {
    let m2720 = (p2721, (p2722, p2723));
    {
        let mut m2720 = m2720;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2720;
                let m2692 = v0;
                if m2692 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2692 - 1;
                    let a2694 = {
                        let a2693 = v1.clone();
                        length_bytes(a2693)
                    };
                    let a2695 = v2;
                    if nat_le(a2694, a2695) {
                        Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                    } else {
                        if f227({
                            let a2696 = f346(v1.clone(), v2)?;
                            let a2697 = 126u64;
                            nat_eq(a2696, a2697)
                        }, {
                            let a2700 = f346(v1.clone(), {
                                let a2698 = v2;
                                let a2699 = 1u64;
                                nat_add(a2698, a2699)?
                            })?;
                            let a2701 = 48u64;
                            nat_eq(a2700, a2701)
                        }) {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a2702 = v2;
                                let a2703 = 2u64;
                                nat_add(a2702, a2703)?
                            }, {
                                let a2704 = v3;
                                let a2705 = f8(Str::lit("~"));
                                append_bytes(a2704, a2705)
                            })))
                        } else {
                            if f227({
                                let a2706 = f346(v1.clone(), v2)?;
                                let a2707 = 126u64;
                                nat_eq(a2706, a2707)
                            }, {
                                let a2710 = f346(v1.clone(), {
                                    let a2708 = v2;
                                    let a2709 = 1u64;
                                    nat_add(a2708, a2709)?
                                })?;
                                let a2711 = 49u64;
                                nat_eq(a2710, a2711)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a2712 = v2;
                                    let a2713 = 2u64;
                                    nat_add(a2712, a2713)?
                                }, {
                                    let a2714 = v3;
                                    let a2715 = f8(Str::lit("/"));
                                    append_bytes(a2714, a2715)
                                })))
                            } else {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a2716 = v2;
                                    let a2717 = 1u64;
                                    nat_add(a2716, a2717)?
                                }, {
                                    let a2718 = v3;
                                    let a2719 = f35(v1.clone(), v2, 1u64);
                                    append_bytes(a2718, a2719)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2720 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(f617({
        let a2725 = {
            let a2724 = v1.clone();
            length_bytes(a2724)
        };
        let a2726 = 1u64;
        nat_add(a2725, a2726)?
    }, v1, 0u64, Bytes::lit(&[]))?, List::<Bytes>::nil())))))
}

pub fn f573(v0: List<(Bytes, Adt17)>, v1: Bytes) -> R<Result<List<Adt17>, Bytes>> {
    let m2727 = v0;
    match m2727.uncons() {
        None => {
            Ok(Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil()))
        }
        Some((v2, v3)) => {
            let m2729 = f565({
                let (h2728, _) = v2;
                h2728
            }, v1.clone())?;
            match m2729 {
                Ok(v5) => {
                    let m2730 = f573(v3, v1)?;
                    match m2730 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt17>, Bytes>(if v5 {
                                List::cons({
                                    let (_, h2731) = v2;
                                    h2731
                                }, v7)
                            } else {
                                v7
                            }))
                        }
                        Err(v6) => {
                            Ok(Err::<List<Adt17>, Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt17>, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f574(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f346(v0.clone(), {
        let a2732 = v1;
        let a2733 = 1u64;
        nat_add(a2732, a2733)?
    })?;
    let v4: Bytes = {
        let a2734 = v3;
        let a2735 = 117u64;
        if nat_eq(a2734, a2735) {
            f618(v0, v1)?
        } else {
            f619(v3)
        }
    };
    let a2736 = v4.clone();
    let a2737 = Bytes::lit(&[]);
    if equal(a2736, a2737) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2738 = v3;
            let a2739 = 117u64;
            if nat_eq(a2738, a2739) {
                let a2740 = v1;
                let a2741 = 6u64;
                nat_add(a2740, a2741)?
            } else {
                let a2742 = v1;
                let a2743 = 2u64;
                nat_add(a2742, a2743)?
            }
        }, ({
            let a2744 = v2;
            let a2745 = v4;
            append_bytes(a2744, a2745)
        }, 0u64))))
    }
}

pub fn f575(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2748 = {
        let a2746 = v0;
        let a2747 = v1;
        index_bytes(a2746, a2747)
    };
    match m2748 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f620(v2)? {
                Ok(Some({
                    let a2749 = v1;
                    let a2750 = 1u64;
                    nat_add(a2749, a2750)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f576(v0: u8) -> R<u64> {
    let a2797 = {
        let v1: u8 = v0;
        let a2795 = {
            let a2771 = {
                let a2759 = {
                    let a2753 = {
                        let a2751 = v1;
                        let a2752 = 128u8;
                        fixed_u8::bit_and(a2751, a2752)
                    };
                    let a2754 = 128u8;
                    if equal(a2753, a2754) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2760 = {
                    let a2757 = {
                        let a2755 = v1;
                        let a2756 = 64u8;
                        fixed_u8::bit_and(a2755, a2756)
                    };
                    let a2758 = 64u8;
                    if equal(a2757, a2758) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2759, a2760)?
            };
            let a2772 = {
                let a2769 = {
                    let a2763 = {
                        let a2761 = v1;
                        let a2762 = 32u8;
                        fixed_u8::bit_and(a2761, a2762)
                    };
                    let a2764 = 32u8;
                    if equal(a2763, a2764) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2770 = {
                    let a2767 = {
                        let a2765 = v1;
                        let a2766 = 16u8;
                        fixed_u8::bit_and(a2765, a2766)
                    };
                    let a2768 = 16u8;
                    if equal(a2767, a2768) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2769, a2770)?
            };
            nat_add(a2771, a2772)?
        };
        let a2796 = {
            let a2793 = {
                let a2781 = {
                    let a2775 = {
                        let a2773 = v1;
                        let a2774 = 8u8;
                        fixed_u8::bit_and(a2773, a2774)
                    };
                    let a2776 = 8u8;
                    if equal(a2775, a2776) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2782 = {
                    let a2779 = {
                        let a2777 = v1;
                        let a2778 = 4u8;
                        fixed_u8::bit_and(a2777, a2778)
                    };
                    let a2780 = 4u8;
                    if equal(a2779, a2780) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2781, a2782)?
            };
            let a2794 = {
                let a2791 = {
                    let a2785 = {
                        let a2783 = v1;
                        let a2784 = 2u8;
                        fixed_u8::bit_and(a2783, a2784)
                    };
                    let a2786 = 2u8;
                    if equal(a2785, a2786) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2792 = {
                    let a2789 = {
                        let a2787 = v1;
                        let a2788 = 1u8;
                        fixed_u8::bit_and(a2787, a2788)
                    };
                    let a2790 = 1u8;
                    if equal(a2789, a2790) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2791, a2792)?
            };
            nat_add(a2793, a2794)?
        };
        nat_add(a2795, a2796)?
    };
    let a2798 = 48u64;
    Ok(nat_sub(a2797, a2798))
}

pub fn f577(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2799 = v0.clone();
        length_bytes(a2799)
    };
    let a2800 = v1;
    let a2801 = v2;
    if if nat_lt(a2800, a2801) {
        let a2806 = f346(v0, {
            let a2804 = v2;
            let a2805 = {
                let a2802 = v1;
                let a2803 = 1u64;
                nat_add(a2802, a2803)?
            };
            nat_sub(a2804, a2805)
        })?;
        let a2807 = 48u64;
        nat_eq(a2806, a2807)
    } else {
        false
    } {
        Ok(Some({
            let a2808 = v1;
            let a2809 = 1u64;
            nat_add(a2808, a2809)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f578() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f579(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f495({
        let (_, h2810) = {
            let (h2811, _) = f621(Fn22::F622, {
                let a2813 = {
                    let a2812 = v0.clone();
                    length_list(a2812)
                };
                let a2814 = 1u64;
                nat_add(a2813, a2814)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2811
        };
        h2810
    }))
}

pub fn f580(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2815, _) = v0;
        h2815
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2816) = v0;
        h2816
    };
    let m2817 = v1;
    match m2817.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2818 = v4;
            match m2818.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f623(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f581(v0: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f509(f35(v0.clone(), 8u64, {
            let a2820 = {
                let a2819 = v0;
                length_bytes(a2819)
            };
            let a2821 = 8u64;
            nat_sub(a2820, a2821)
        }))?))
    } else {
        if f72(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f509(f35(v0.clone(), 14u64, {
                let a2823 = {
                    let a2822 = v0;
                    length_bytes(a2822)
                };
                let a2824 = 14u64;
                nat_sub(a2823, a2824)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f582(p2841: u64, p2842: u64, p2843: u64, p2844: Bytes) -> Bytes {
    let m2840 = (p2841, (p2842, (p2843, p2844)));
    {
        let mut m2840 = m2840;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2840;
                let m2825 = v0;
                if m2825 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2825 - 1;
                    if f227({
                        let a2826 = v1;
                        let a2827 = 0u64;
                        nat_eq(a2826, a2827)
                    }, {
                        let a2828 = v2;
                        let a2829 = 0u64;
                        nat_eq(a2828, a2829)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2830 = v1;
                            let a2831 = 16u64;
                            let a2832 = 0u64;
                            nat_quot(a2830, a2831, a2832)
                        }, ({
                            let a2833 = v2;
                            let a2834 = 1u64;
                            nat_sub(a2833, a2834)
                        }, {
                            let a2838 = f624({
                                let a2835 = v1;
                                let a2836 = 16u64;
                                let a2837 = 0u64;
                                nat_rem(a2835, a2836, a2837)
                            });
                            let a2839 = v3;
                            append_bytes(a2838, a2839)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2840 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: List<Adt17>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f625(f626(f242(v0.clone()))?, v0, v1)
}

pub fn f584(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f585(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f586(v0: Bytes, v1: List<Adt17>) -> R<Result<Adt28, Bytes>> {
    let m2845 = v1.clone();
    match m2845.uncons() {
        None => {
            Ok(f584(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2846 = f627(f465(f238(v2, f8(Str::lit("properties")))), v1.clone());
            match m2846.uncons() {
                None => {
                    let m2847 = f628(f626(f629(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2847 {
                        Ok(v5) => {
                            Ok(f585(f589(v0, f590(v5.clone())), f591(v5)))
                        }
                        Err(v4) => {
                            Ok(f584(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2848 = f630(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2848 {
                        Ok(v9) => {
                            Ok(f585(f589(v0, f590(v9.clone())), f591(v9)))
                        }
                        Err(v8) => {
                            Ok(f584(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f587(v0: Adt17, v1: Bytes) -> R<bool> {
    let m2849 = v0;
    match m2849 {
        Adt17::C0 => {
            Ok(false)
        }
        Adt17::C1(_, _) => {
            Ok(false)
        }
        Adt17::C2(_) => {
            Ok(false)
        }
        Adt17::C3(_, _) => {
            Ok(false)
        }
        Adt17::C4(_, _, _) => {
            Ok(false)
        }
        Adt17::C5(_) => {
            Ok(false)
        }
        Adt17::C6(r2850) => {
            let v2 = (*r2850).clone();
            f587(v2, v1)
        }
        Adt17::C7(r2851) => {
            let v3 = (*r2851).clone();
            f587(v3, v1)
        }
        Adt17::C8 => {
            Ok(false)
        }
        Adt17::C9(r2852, r2853) => {
            let v4 = (*r2852).clone();
            let v5 = (*r2853).clone();
            if f587(v4, v1.clone())? {
                Ok(true)
            } else {
                f587(v5, v1)
            }
        }
        Adt17::C10(v6, r2854, r2855) => {
            let v7 = (*r2854).clone();
            let v8 = (*r2855).clone();
            if if f28(v6, f8(Str::lit("$ref"))) {
                f631(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f587(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f587(v8, v1)
                }
            }
        }
    }
}

pub fn f588(v0: Bytes, v1: List<(Bytes, Adt17)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f632(f626(f568(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f589(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f342(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f590(v0: Adt29) -> List<Bytes> {
    let m2856 = v0;
    match m2856 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f591(v0: Adt29) -> List<Adt25> {
    let m2857 = v0;
    match m2857 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f592(v0: Adt17) -> bool {
    let m2858 = f465(f238(v0.clone(), f8(Str::lit("properties"))));
    match m2858.uncons() {
        None => {
            let m2859 = f633(v0);
            match m2859 {
                None => {
                    false
                }
                Some(v3) => {
                    f634(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f593(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f594(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2860 = f635(v0.clone());
    match m2860 {
        None => {
            if f28(f241(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2861 = f238(v0, f8(Str::lit("items")));
                match m2861 {
                    None => {
                        Ok(f636(f8(Str::lit("List")), f638()))
                    }
                    Some(v3) => {
                        Ok(f636(f8(Str::lit("List")), f637(v3, {
                            let a2862 = v1;
                            let a2863 = f8(Str::lit("Item"));
                            append_bytes(a2862, a2863)
                        })?))
                    }
                }
            } else {
                f637(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f636(f8(Str::lit("Option")), f637(v2, v1)?))
        }
    }
}

pub fn f595(p2866: List<Adt17>) -> bool {
    let m2865 = p2866;
    {
        let mut m2865 = m2865;
        loop {
            let step = {
                let v0 = m2865;
                let m2864 = v0;
                match m2864.uncons() {
                    None => {
                        Err::<List<Adt17>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f639(f506(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt17>, bool>(v2)
                        } else {
                            Err::<List<Adt17>, bool>(false)
                        }
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

pub fn f596(v0: Adt17, v1: Bytes) -> Adt17 {
    let m2867 = v0;
    match m2867 {
        Adt17::C0 => {
            Adt17::C0
        }
        Adt17::C1(v6, r2868) => {
            let v7 = (*r2868).clone();
            Adt17::C1(v6, std::rc::Rc::new(v7))
        }
        Adt17::C2(v8) => {
            Adt17::C2(v8)
        }
        Adt17::C3(v12, v13) => {
            Adt17::C3(v12, v13)
        }
        Adt17::C4(v9, v10, v11) => {
            Adt17::C4(v9, v10, v11)
        }
        Adt17::C5(v14) => {
            Adt17::C5(v14)
        }
        Adt17::C6(r2869) => {
            let v15 = (*r2869).clone();
            Adt17::C6(std::rc::Rc::new(v15))
        }
        Adt17::C7(r2870) => {
            let v5 = (*r2870).clone();
            Adt17::C7(std::rc::Rc::new(f596(v5, v1)))
        }
        Adt17::C8 => {
            Adt17::C8
        }
        Adt17::C9(r2871, r2872) => {
            let v16 = (*r2871).clone();
            let v17 = (*r2872).clone();
            Adt17::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt17::C10(v2, r2873, r2874) => {
            let v3 = (*r2873).clone();
            let v4 = (*r2874).clone();
            if f28(v2.clone(), v1.clone()) {
                f596(v4, v1)
            } else {
                Adt17::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f596(v4, v1)))
            }
        }
    }
}

pub fn f597(v0: Adt17, v1: Bytes) -> bool {
    let m2875 = f238(v0, v1);
    match m2875 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f598(v0: List<Adt17>) -> bool {
    let m2876 = v0.clone();
    match m2876.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2877 = f627(f465(f238(v1, f8(Str::lit("properties")))), v0.clone());
            match m2877.uncons() {
                None => {
                    f641(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f640(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f599(v0: Adt17, v1: Bytes, v2: Adt17) -> Adt17 {
    let m2878 = v0;
    match m2878 {
        Adt17::C0 => {
            Adt17::C0
        }
        Adt17::C1(v7, r2879) => {
            let v8 = (*r2879).clone();
            Adt17::C1(v7, std::rc::Rc::new(v8))
        }
        Adt17::C2(v9) => {
            Adt17::C2(v9)
        }
        Adt17::C3(v13, v14) => {
            Adt17::C3(v13, v14)
        }
        Adt17::C4(v10, v11, v12) => {
            Adt17::C4(v10, v11, v12)
        }
        Adt17::C5(v15) => {
            Adt17::C5(v15)
        }
        Adt17::C6(r2880) => {
            let v16 = (*r2880).clone();
            Adt17::C6(std::rc::Rc::new(v16))
        }
        Adt17::C7(r2881) => {
            let v3 = (*r2881).clone();
            Adt17::C7(std::rc::Rc::new(Adt17::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt17::C8 => {
            Adt17::C8
        }
        Adt17::C9(r2882, r2883) => {
            let v17 = (*r2882).clone();
            let v18 = (*r2883).clone();
            Adt17::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt17::C10(v4, r2884, r2885) => {
            let v5 = (*r2884).clone();
            let v6 = (*r2885).clone();
            Adt17::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f600(p2888: List<Adt17>) -> bool {
    let m2887 = p2888;
    {
        let mut m2887 = m2887;
        loop {
            let step = {
                let v0 = m2887;
                let m2886 = v0;
                match m2886.uncons() {
                    None => {
                        Err::<List<Adt17>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f642(v1) {
                            Ok::<List<Adt17>, bool>(v2)
                        } else {
                            Err::<List<Adt17>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f601(v0: u64) -> u32 {
    let a2889 = v0;
    let a2890 = 32u64;
    if nat_lt(a2889, a2890) {
        let a2891 = v0;
        let a2892 = 16u64;
        if nat_lt(a2891, a2892) {
            let a2893 = v0;
            let a2894 = 8u64;
            if nat_lt(a2893, a2894) {
                let a2895 = v0;
                let a2896 = 4u64;
                if nat_lt(a2895, a2896) {
                    let a2897 = v0;
                    let a2898 = 2u64;
                    if nat_lt(a2897, a2898) {
                        let a2899 = v0;
                        let a2900 = 1u64;
                        if nat_lt(a2899, a2900) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2901 = v0;
                        let a2902 = 3u64;
                        if nat_lt(a2901, a2902) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2903 = v0;
                    let a2904 = 6u64;
                    if nat_lt(a2903, a2904) {
                        let a2905 = v0;
                        let a2906 = 5u64;
                        if nat_lt(a2905, a2906) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2907 = v0;
                        let a2908 = 7u64;
                        if nat_lt(a2907, a2908) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2909 = v0;
                let a2910 = 12u64;
                if nat_lt(a2909, a2910) {
                    let a2911 = v0;
                    let a2912 = 10u64;
                    if nat_lt(a2911, a2912) {
                        let a2913 = v0;
                        let a2914 = 9u64;
                        if nat_lt(a2913, a2914) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2915 = v0;
                        let a2916 = 11u64;
                        if nat_lt(a2915, a2916) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2917 = v0;
                    let a2918 = 14u64;
                    if nat_lt(a2917, a2918) {
                        let a2919 = v0;
                        let a2920 = 13u64;
                        if nat_lt(a2919, a2920) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2921 = v0;
                        let a2922 = 15u64;
                        if nat_lt(a2921, a2922) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2923 = v0;
            let a2924 = 24u64;
            if nat_lt(a2923, a2924) {
                let a2925 = v0;
                let a2926 = 20u64;
                if nat_lt(a2925, a2926) {
                    let a2927 = v0;
                    let a2928 = 18u64;
                    if nat_lt(a2927, a2928) {
                        let a2929 = v0;
                        let a2930 = 17u64;
                        if nat_lt(a2929, a2930) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2931 = v0;
                        let a2932 = 19u64;
                        if nat_lt(a2931, a2932) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2933 = v0;
                    let a2934 = 22u64;
                    if nat_lt(a2933, a2934) {
                        let a2935 = v0;
                        let a2936 = 21u64;
                        if nat_lt(a2935, a2936) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2937 = v0;
                        let a2938 = 23u64;
                        if nat_lt(a2937, a2938) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2939 = v0;
                let a2940 = 28u64;
                if nat_lt(a2939, a2940) {
                    let a2941 = v0;
                    let a2942 = 26u64;
                    if nat_lt(a2941, a2942) {
                        let a2943 = v0;
                        let a2944 = 25u64;
                        if nat_lt(a2943, a2944) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2945 = v0;
                        let a2946 = 27u64;
                        if nat_lt(a2945, a2946) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2947 = v0;
                    let a2948 = 30u64;
                    if nat_lt(a2947, a2948) {
                        let a2949 = v0;
                        let a2950 = 29u64;
                        if nat_lt(a2949, a2950) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2951 = v0;
                        let a2952 = 31u64;
                        if nat_lt(a2951, a2952) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2953 = v0;
        let a2954 = 48u64;
        if nat_lt(a2953, a2954) {
            let a2955 = v0;
            let a2956 = 40u64;
            if nat_lt(a2955, a2956) {
                let a2957 = v0;
                let a2958 = 36u64;
                if nat_lt(a2957, a2958) {
                    let a2959 = v0;
                    let a2960 = 34u64;
                    if nat_lt(a2959, a2960) {
                        let a2961 = v0;
                        let a2962 = 33u64;
                        if nat_lt(a2961, a2962) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2963 = v0;
                        let a2964 = 35u64;
                        if nat_lt(a2963, a2964) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2965 = v0;
                    let a2966 = 38u64;
                    if nat_lt(a2965, a2966) {
                        let a2967 = v0;
                        let a2968 = 37u64;
                        if nat_lt(a2967, a2968) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2969 = v0;
                        let a2970 = 39u64;
                        if nat_lt(a2969, a2970) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2971 = v0;
                let a2972 = 44u64;
                if nat_lt(a2971, a2972) {
                    let a2973 = v0;
                    let a2974 = 42u64;
                    if nat_lt(a2973, a2974) {
                        let a2975 = v0;
                        let a2976 = 41u64;
                        if nat_lt(a2975, a2976) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2977 = v0;
                        let a2978 = 43u64;
                        if nat_lt(a2977, a2978) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2979 = v0;
                    let a2980 = 46u64;
                    if nat_lt(a2979, a2980) {
                        let a2981 = v0;
                        let a2982 = 45u64;
                        if nat_lt(a2981, a2982) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2983 = v0;
                        let a2984 = 47u64;
                        if nat_lt(a2983, a2984) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2985 = v0;
            let a2986 = 56u64;
            if nat_lt(a2985, a2986) {
                let a2987 = v0;
                let a2988 = 52u64;
                if nat_lt(a2987, a2988) {
                    let a2989 = v0;
                    let a2990 = 50u64;
                    if nat_lt(a2989, a2990) {
                        let a2991 = v0;
                        let a2992 = 49u64;
                        if nat_lt(a2991, a2992) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2993 = v0;
                        let a2994 = 51u64;
                        if nat_lt(a2993, a2994) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2995 = v0;
                    let a2996 = 54u64;
                    if nat_lt(a2995, a2996) {
                        let a2997 = v0;
                        let a2998 = 53u64;
                        if nat_lt(a2997, a2998) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2999 = v0;
                        let a3000 = 55u64;
                        if nat_lt(a2999, a3000) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3001 = v0;
                let a3002 = 60u64;
                if nat_lt(a3001, a3002) {
                    let a3003 = v0;
                    let a3004 = 58u64;
                    if nat_lt(a3003, a3004) {
                        let a3005 = v0;
                        let a3006 = 57u64;
                        if nat_lt(a3005, a3006) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3007 = v0;
                        let a3008 = 59u64;
                        if nat_lt(a3007, a3008) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3009 = v0;
                    let a3010 = 62u64;
                    if nat_lt(a3009, a3010) {
                        let a3011 = v0;
                        let a3012 = 61u64;
                        if nat_lt(a3011, a3012) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3013 = v0;
                        let a3014 = 63u64;
                        if nat_lt(a3013, a3014) {
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

pub fn f602(v0: u32) -> u32 {
    let a3017 = {
        let a3015 = f643(v0, 6u32, 26u32);
        let a3016 = f643(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3015, a3016)
    };
    let a3018 = f643(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3017, a3018)
}

pub fn f603(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3024 = {
        let a3019 = v0;
        let a3020 = v1;
        fixed_u32::bit_and(a3019, a3020)
    };
    let a3025 = {
        let a3022 = {
            let a3021 = v0;
            fixed_u32::bit_not(a3021)
        };
        let a3023 = v2;
        fixed_u32::bit_and(a3022, a3023)
    };
    fixed_u32::bit_xor(a3024, a3025)
}

pub fn f604(v0: u32) -> u32 {
    let a3028 = {
        let a3026 = f643(v0, 2u32, 30u32);
        let a3027 = f643(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3026, a3027)
    };
    let a3029 = f643(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3028, a3029)
}

pub fn f605(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3038 = {
        let a3034 = {
            let a3030 = v0;
            let a3031 = v1;
            fixed_u32::bit_and(a3030, a3031)
        };
        let a3035 = {
            let a3032 = v0;
            let a3033 = v2;
            fixed_u32::bit_and(a3032, a3033)
        };
        fixed_u32::bit_xor(a3034, a3035)
    };
    let a3039 = {
        let a3036 = v1;
        let a3037 = v2;
        fixed_u32::bit_and(a3036, a3037)
    };
    fixed_u32::bit_xor(a3038, a3039)
}

pub fn f606(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3040, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3040
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3041, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3041
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3042, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3042
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3043, _, _, _, _, _, _, _, _, _, _, _) => {
            h3043
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3044, _, _, _, _, _, _, _, _, _, _) => {
            h3044
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3045, _, _, _, _, _, _, _, _, _) => {
            h3045
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3046, _, _, _, _, _, _, _, _) => {
            h3046
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3047, _, _, _, _, _, _, _) => {
            h3047
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3048, _, _, _, _, _, _) => {
            h3048
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3049, _, _, _, _, _) => {
            h3049
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3050, _, _, _, _) => {
            h3050
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3051, _, _, _) => {
            h3051
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3052, _, _) => {
            h3052
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3053, _) => {
            h3053
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3054) => {
            h3054
        }
    }, f518(f644(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3055, _) => {
            h3055
        }
    }), f518(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3056, _, _, _, _, _, _) => {
            h3056
        }
    }, f518(f645(match v0.clone() {
        Adt26::C0(_, h3057, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3057
        }
    }), match v0 {
        Adt26::C0(h3058, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3058
        }
    }))))
}

pub fn f607(v0: Fn21, p3062: u64, p3063: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3061 = (p3062, p3063);
    {
        let mut m3061 = m3061;
        loop {
            let step = {
                let (v1, v2) = m3061;
                let m3059 = v1;
                if m3059 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3059 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3060: Fn21 = v0.clone();
                        c3060.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3061 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f608(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3064, _) = v0;
        h3064
    };
    let v2: u32 = {
        let (h3065, _) = {
            let (_, h3066) = v0;
            h3066
        };
        h3065
    };
    let v3: u32 = {
        let (_, h3067) = {
            let (_, h3068) = v0;
            h3068
        };
        h3067
    };
    ({
        let a3069 = v1;
        let a3070 = 2u64;
        let a3071 = 0u64;
        nat_quot(a3069, a3070, a3071)
    }, (f609(v2, 1u32), {
        let a3075 = {
            let a3072 = v1;
            let a3073 = 2u64;
            let a3074 = 0u64;
            nat_rem(a3072, a3073, a3074)
        };
        let a3076 = 1u64;
        if nat_eq(a3075, a3076) {
            let a3077 = v3;
            let a3078 = v2;
            fixed_u32::bit_or(a3077, a3078)
        } else {
            v3
        }
    }))
}

pub fn f609(v0: u32, v1: u32) -> u32 {
    let m3081 = {
        let a3079 = v0;
        let a3080 = v1;
        fixed_u32::shift_left(a3079, a3080)
    };
    match m3081 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f610(v0: Bytes, v1: Adt17) -> bool {
    let v2: Bytes = f558(v1);
    if f28(v0.clone(), f8(Str::lit("number"))) {
        if f28(v2.clone(), f8(Str::lit("integer"))) {
            true
        } else {
            f28(v2, f8(Str::lit("number")))
        }
    } else {
        f28(v0, v2)
    }
}

pub fn f611(v0: u64, v1: Bytes, v2: u64) -> R<Option<List<u64>>> {
    let m3082 = v0;
    if m3082 == 0 {
        Ok(Some(List::<u64>::nil()))
    } else {
        let v3 = m3082 - 1;
        let a3084 = {
            let a3083 = v1.clone();
            length_bytes(a3083)
        };
        let a3085 = v2;
        if nat_le(a3084, a3085) {
            Ok(Some(List::<u64>::nil()))
        } else {
            let m3086 = f646(v1.clone(), v2)?;
            match m3086 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v4) => {
                    let m3090 = f611(v3, v1, {
                        let a3088 = v2;
                        let a3089 = {
                            let (_, h3087) = v4;
                            h3087
                        };
                        nat_add(a3088, a3089)?
                    })?;
                    match m3090 {
                        None => {
                            Ok(None::<List<u64>>)
                        }
                        Some(v5) => {
                            Ok(Some(List::cons({
                                let (h3091, _) = v4;
                                h3091
                            }, v5)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f612(v0: Bytes) -> R<Result<((bool, bool), List<Adt30>), Bytes>> {
    let v1: u64 = {
        let a3092 = v0.clone();
        length_bytes(a3092)
    };
    let v2: bool = {
        let a3093 = 0u64;
        let a3094 = v1;
        if nat_lt(a3093, a3094) {
            let a3095 = f346(v0.clone(), 0u64)?;
            let a3096 = 94u64;
            nat_eq(a3095, a3096)
        } else {
            false
        }
    };
    let v3: bool = {
        let a3097 = 0u64;
        let a3098 = v1;
        if nat_lt(a3097, a3098) {
            f227({
                let a3101 = f346(v0.clone(), {
                    let a3099 = v1;
                    let a3100 = 1u64;
                    nat_sub(a3099, a3100)
                })?;
                let a3102 = 36u64;
                nat_eq(a3101, a3102)
            }, f647({
                let a3105 = f346(v0.clone(), {
                    let a3103 = v1;
                    let a3104 = 2u64;
                    nat_sub(a3103, a3104)
                })?;
                let a3106 = 92u64;
                nat_eq(a3105, a3106)
            }))
        } else {
            false
        }
    };
    let m3111 = f648({
        let a3107 = v1;
        let a3108 = 1u64;
        nat_add(a3107, a3108)?
    }, v0, if v2 {
        1u64
    } else {
        0u64
    }, if v3 {
        let a3109 = v1;
        let a3110 = 1u64;
        nat_sub(a3109, a3110)
    } else {
        v1
    })?;
    match m3111 {
        Ok(v5) => {
            Ok(Ok::<((bool, bool), List<Adt30>), Bytes>(((v2, v3), v5)))
        }
        Err(v4) => {
            Ok(Err::<((bool, bool), List<Adt30>), Bytes>(v4))
        }
    }
}

pub fn f613(v0: u64, v1: List<Adt30>, v2: List<u64>, v3: bool) -> R<bool> {
    let m3112 = v0;
    if m3112 == 0 {
        Ok(false)
    } else {
        let v4 = m3112 - 1;
        let m3113 = v1.clone();
        match m3113.uncons() {
            None => {
                if v3 {
                    let a3114 = f563(v2)?;
                    let a3115 = 0u64;
                    Ok(nat_eq(a3114, a3115))
                } else {
                    Ok(true)
                }
            }
            Some((v5, v6)) => {
                let m3116 = v5.clone();
                match m3116 {
                    Adt30::C0(v7, v8, v9) => {
                        let a3117 = v9;
                        let a3118 = 1u64;
                        if nat_eq(a3117, a3118) {
                            if f613(v4, v6, v2.clone(), v3)? {
                                Ok(true)
                            } else {
                                let m3119 = v2;
                                match m3119.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v10, v11)) => {
                                        if f649(v5, v10) {
                                            f613(v4, v1, v11, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a3120 = v9;
                            let a3121 = 2u64;
                            if nat_eq(a3120, a3121) {
                                let m3122 = v2;
                                match m3122.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v12, v13)) => {
                                        if f649(v5, v12) {
                                            f613(v4, List::cons(Adt30::C0(v7, v8, 1u64), v6), v13, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            } else {
                                let a3123 = v9;
                                let a3124 = 3u64;
                                if nat_eq(a3123, a3124) {
                                    if f613(v4, v6.clone(), v2.clone(), v3)? {
                                        Ok(true)
                                    } else {
                                        let m3125 = v2;
                                        match m3125.uncons() {
                                            None => {
                                                Ok(false)
                                            }
                                            Some((v14, v15)) => {
                                                if f649(v5, v14) {
                                                    f613(v4, v6, v15, v3)
                                                } else {
                                                    Ok(false)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let m3126 = v2;
                                    match m3126.uncons() {
                                        None => {
                                            Ok(false)
                                        }
                                        Some((v16, v17)) => {
                                            if f649(v5, v16) {
                                                f613(v4, v6, v17, v3)
                                            } else {
                                                Ok(false)
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

pub fn f614(v0: List<Adt30>) -> R<u64> {
    let m3127 = v0;
    match m3127.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a3128 = f614(v2)?;
            let a3129 = 1u64;
            nat_add(a3128, a3129)
        }
    }
}

pub fn f615(v0: List<Adt30>, v1: List<u64>, v2: bool) -> R<bool> {
    if f613({
        let a3132 = {
            let a3130 = f614(v0.clone())?;
            let a3131 = f563(v1.clone())?;
            nat_add(a3130, a3131)?
        };
        let a3133 = 2u64;
        nat_add(a3132, a3133)?
    }, v0.clone(), v1.clone(), v2)? {
        Ok(true)
    } else {
        f650(v0, v1, v2)
    }
}

pub fn f616(v0: u64) -> R<u64> {
    let m3134 = v0;
    if m3134 == 0 {
        Ok(1u64)
    } else {
        let v1 = m3134 - 1;
        let a3135 = f616(v1)?;
        let a3136 = 10u64;
        nat_mul(a3135, a3136)
    }
}

pub fn f617(p3150: u64, v1: Bytes, p3151: u64, p3152: Bytes) -> R<Bytes> {
    let m3149 = (p3150, (p3151, p3152));
    {
        let mut m3149 = m3149;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3149;
                let m3137 = v0;
                if m3137 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3137 - 1;
                    let m3140 = {
                        let a3138 = v1.clone();
                        let a3139 = v2;
                        index_bytes(a3138, a3139)
                    };
                    match m3140 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3141 = v2;
                                let a3142 = 1u64;
                                nat_add(a3141, a3142)?
                            }, {
                                let a3147 = v3;
                                let a3148 = {
                                    let a3143 = v6;
                                    let a3144 = 126u64;
                                    if nat_eq(a3143, a3144) {
                                        f8(Str::lit("~0"))
                                    } else {
                                        let a3145 = v6;
                                        let a3146 = 47u64;
                                        if nat_eq(a3145, a3146) {
                                            f8(Str::lit("~1"))
                                        } else {
                                            f35(v1.clone(), v2, 1u64)
                                        }
                                    }
                                };
                                append_bytes(a3147, a3148)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: Bytes, v1: u64) -> R<Bytes> {
    let a3155 = f35(v0.clone(), {
        let a3153 = v1;
        let a3154 = 2u64;
        nat_add(a3153, a3154)?
    }, 2u64);
    let a3156 = Bytes::lit(&[0x30, 0x30]);
    if equal(a3155, a3156) {
        let m3159 = f651(f35(v0, {
            let a3157 = v1;
            let a3158 = 4u64;
            nat_add(a3157, a3158)?
        }, 2u64))?;
        match m3159 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f346(v2.clone(), 0u64)?;
                let a3160 = v3;
                let a3161 = 32u64;
                if nat_lt(a3160, a3161) {
                    if f652(v3) {
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

pub fn f619(v0: u64) -> Bytes {
    let a3162 = v0;
    let a3163 = 34u64;
    if nat_eq(a3162, a3163) {
        Bytes::lit(&[0x22])
    } else {
        let a3164 = v0;
        let a3165 = 92u64;
        if nat_eq(a3164, a3165) {
            Bytes::lit(&[0x5c])
        } else {
            let a3166 = v0;
            let a3167 = 98u64;
            if nat_eq(a3166, a3167) {
                Bytes::lit(&[0x08])
            } else {
                let a3168 = v0;
                let a3169 = 102u64;
                if nat_eq(a3168, a3169) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a3170 = v0;
                    let a3171 = 110u64;
                    if nat_eq(a3170, a3171) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a3172 = v0;
                        let a3173 = 114u64;
                        if nat_eq(a3172, a3173) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a3174 = v0;
                            let a3175 = 116u64;
                            if nat_eq(a3174, a3175) {
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

pub fn f620(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3220 = {
            let a3196 = {
                let a3184 = {
                    let a3178 = {
                        let a3176 = v1;
                        let a3177 = 128u8;
                        fixed_u8::bit_and(a3176, a3177)
                    };
                    let a3179 = 128u8;
                    if equal(a3178, a3179) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3185 = {
                    let a3182 = {
                        let a3180 = v1;
                        let a3181 = 64u8;
                        fixed_u8::bit_and(a3180, a3181)
                    };
                    let a3183 = 64u8;
                    if equal(a3182, a3183) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3184, a3185)?
            };
            let a3197 = {
                let a3194 = {
                    let a3188 = {
                        let a3186 = v1;
                        let a3187 = 32u8;
                        fixed_u8::bit_and(a3186, a3187)
                    };
                    let a3189 = 32u8;
                    if equal(a3188, a3189) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3195 = {
                    let a3192 = {
                        let a3190 = v1;
                        let a3191 = 16u8;
                        fixed_u8::bit_and(a3190, a3191)
                    };
                    let a3193 = 16u8;
                    if equal(a3192, a3193) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3194, a3195)?
            };
            nat_add(a3196, a3197)?
        };
        let a3221 = {
            let a3218 = {
                let a3206 = {
                    let a3200 = {
                        let a3198 = v1;
                        let a3199 = 8u8;
                        fixed_u8::bit_and(a3198, a3199)
                    };
                    let a3201 = 8u8;
                    if equal(a3200, a3201) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3207 = {
                    let a3204 = {
                        let a3202 = v1;
                        let a3203 = 4u8;
                        fixed_u8::bit_and(a3202, a3203)
                    };
                    let a3205 = 4u8;
                    if equal(a3204, a3205) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3206, a3207)?
            };
            let a3219 = {
                let a3216 = {
                    let a3210 = {
                        let a3208 = v1;
                        let a3209 = 2u8;
                        fixed_u8::bit_and(a3208, a3209)
                    };
                    let a3211 = 2u8;
                    if equal(a3210, a3211) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3217 = {
                    let a3214 = {
                        let a3212 = v1;
                        let a3213 = 1u8;
                        fixed_u8::bit_and(a3212, a3213)
                    };
                    let a3215 = 1u8;
                    if equal(a3214, a3215) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3216, a3217)?
            };
            nat_add(a3218, a3219)?
        };
        nat_add(a3220, a3221)?
    };
    let a3222 = 48u64;
    let a3223 = v2;
    if nat_le(a3222, a3223) {
        let a3224 = v2;
        let a3225 = 57u64;
        Ok(nat_le(a3224, a3225))
    } else {
        Ok(false)
    }
}

pub fn f621(v0: Fn22, p3230: u64, p3231: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3229 = (p3230, p3231);
    {
        let mut m3229 = m3229;
        loop {
            let step = {
                let (v1, v2) = m3229;
                let m3226 = v1;
                if m3226 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3226 - 1;
                    let m3228 = {
                        let c3227: Fn22 = v0.clone();
                        c3227.apply(v2.clone())?
                    };
                    match m3228 {
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
                Ok(next) => { m3229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f653(v0)
}

pub fn f623(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3236 = {
            let a3234 = {
                let a3232 = v0.clone();
                length_list(a3232)
            };
            let a3235 = {
                let a3233 = v1.clone();
                length_list(a3233)
            };
            nat_add(a3234, a3235)?
        };
        let a3237 = 1u64;
        nat_add(a3236, a3237)?
    };
    Ok(f390({
        let (_, h3238) = {
            let (_, h3239) = {
                let (h3240, _) = f654(Fn23::F655, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3240
            };
            h3239
        };
        h3238
    }))
}

pub fn f624(v0: u64) -> Bytes {
    f35({
        let a3241 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3241)
    }, v0, 1u64)
}

pub fn f625(v0: bool, v1: List<Adt17>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3242 = v1;
    match m3242.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3243 = f656(v0, v3.clone())?;
            match m3243 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f481(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f41(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3244 = f625(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3244 {
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

pub fn f626(p3248: List<Bytes>) -> R<bool> {
    let m3247 = p3248;
    {
        let mut m3247 = m3247;
        loop {
            let step = {
                let v0 = m3247;
                let m3245 = v0;
                match m3245.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3246 = f657(v1)?;
                        match m3246 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f41(v3, f658(false, v2.clone())?) {
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
                Ok(next) => { m3247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f627(v0: List<(Bytes, Adt17)>, v1: List<Adt17>) -> List<Bytes> {
    let m3249 = v0;
    match m3249.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f659(v1.clone(), {
                let (h3250, _) = v2.clone();
                h3250
            }) {
                List::cons({
                    let (h3251, _) = v2;
                    h3251
                }, f627(v3, v1))
            } else {
                f627(v3, v1)
            }
        }
    }
}

pub fn f628(v0: bool, v1: Bytes, v2: List<Adt17>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3252 = v2;
    match m3252.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3253 = f660(v4.clone())?;
            match m3253 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3254 = f661(v0, v6.clone())?;
                    match m3254 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f41(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3257 = f662(v4, {
                                    let a3255 = v1.clone();
                                    let a3256 = f326(v6)?;
                                    append_bytes(a3255, a3256)
                                })?;
                                match m3257 {
                                    Ok(v9) => {
                                        let m3258 = f628(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3258 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f514(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f590(v11.clone())), f469(f513(v9), f591(v11)))))
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

pub fn f629(v0: List<Adt17>) -> R<List<Bytes>> {
    let m3259 = v0;
    match m3259.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3260 = f660(v1)?;
            match m3260 {
                None => {
                    f629(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f629(v2)?))
                }
            }
        }
    }
}

pub fn f630(v0: Bytes, v1: List<Adt17>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f663(f626(f664(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f631(v0: Adt17, v1: Bytes) -> R<bool> {
    let m3261 = f332(Some(v0));
    match m3261 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3262 = f581(v2)?;
            match m3262 {
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

pub fn f632(v0: bool, v1: Bytes, v2: List<(Bytes, Adt17)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m3263 = v2;
    match m3263.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3265 = f665(v0, {
                let (h3264, _) = v5.clone();
                h3264
            })?;
            match m3265 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3266, _) = v5;
                        h3266
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f41(v7.clone(), f666(v0, f568(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3271 = f594({
                            let (_, h3267) = v5.clone();
                            h3267
                        }, {
                            let a3269 = v1.clone();
                            let a3270 = f326({
                                let (h3268, _) = v5.clone();
                                h3268
                            })?;
                            append_bytes(a3269, a3270)
                        })?;
                        match m3271 {
                            Ok(v9) => {
                                let v10: Bytes = if f41({
                                    let (h3272, _) = v5;
                                    h3272
                                }, v3.clone()) {
                                    f514(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f514(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3273 = f632(v0, v1, v6, v3, v4)?;
                                match m3273 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f590(v12.clone())), f469(f513(v9), f591(v12)))))
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

pub fn f633(v0: Adt17) -> Option<Adt17> {
    let m3274 = f238(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3274 {
        None => {
            f667(v0)
        }
        Some(v1) => {
            if f634(v1.clone()) {
                let m3275 = f667(v0);
                match m3275 {
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

pub fn f634(v0: Adt17) -> bool {
    let m3276 = v0;
    match m3276 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_, _) => {
            false
        }
        Adt17::C2(v3) => {
            !v3
        }
        Adt17::C3(_, _) => {
            false
        }
        Adt17::C4(_, _, _) => {
            false
        }
        Adt17::C5(_) => {
            false
        }
        Adt17::C6(_) => {
            false
        }
        Adt17::C7(_) => {
            false
        }
        Adt17::C8 => {
            false
        }
        Adt17::C9(_, _) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
    }
}

pub fn f635(v0: Adt17) -> Option<Adt17> {
    let m3277 = f243(f238(v0, f8(Str::lit("oneOf"))));
    match m3277.uncons() {
        None => {
            None::<Adt17>
        }
        Some((v1, v2)) => {
            let m3278 = v2;
            match m3278.uncons() {
                None => {
                    None::<Adt17>
                }
                Some((v3, v4)) => {
                    let m3279 = v4;
                    match m3279.uncons() {
                        None => {
                            if f668(v3.clone()) {
                                Some(v1)
                            } else {
                                if f668(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt17>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt17>
                        }
                    }
                }
            }
        }
    }
}

pub fn f636(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3280 = v1;
    match m3280 {
        Ok(v3) => {
            f585(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f514(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f513(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f637(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3281 = f635(v0.clone());
    match m3281 {
        None => {
            if f28(f241(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3282 = f238(v0, f8(Str::lit("items")));
                match m3282 {
                    None => {
                        Ok(f636(f8(Str::lit("List")), f638()))
                    }
                    Some(v3) => {
                        Ok(f636(f8(Str::lit("List")), f662(v3, {
                            let a3283 = v1;
                            let a3284 = f8(Str::lit("Item"));
                            append_bytes(a3283, a3284)
                        })?))
                    }
                }
            } else {
                f662(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f636(f8(Str::lit("Option")), f662(v2, v1)?))
        }
    }
}

pub fn f638() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f510(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f639(p3288: List<(Bytes, Adt17)>, v1: List<Bytes>) -> bool {
    let m3287 = p3288;
    {
        let mut m3287 = m3287;
        loop {
            let step = {
                let v0 = m3287;
                let m3285 = v0;
                match m3285.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h3286, _) = v2;
                            h3286
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt17)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt17)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3287 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(p3292: List<Adt17>, v1: Bytes, p3293: List<Bytes>) -> bool {
    let m3291 = (p3292, p3293);
    {
        let mut m3291 = m3291;
        loop {
            let step = {
                let (v0, v2) = m3291;
                let m3289 = v0;
                match m3289.uncons() {
                    None => {
                        Err::<(List<Adt17>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3290 = f669(v3, v1.clone());
                        match m3290 {
                            None => {
                                Err::<(List<Adt17>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f41(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt17>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt17>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f641(p3296: List<Adt17>, p3297: List<Bytes>) -> bool {
    let m3295 = (p3296, p3297);
    {
        let mut m3295 = m3295;
        loop {
            let step = {
                let (v0, v1) = m3295;
                let m3294 = v0;
                match m3294.uncons() {
                    None => {
                        Err::<(List<Adt17>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f241(v2, f8(Str::lit("type")));
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
                                    Ok::<(List<Adt17>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt17>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt17>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt17>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f642(v0: Adt17) -> bool {
    let v2: Adt17 = {
        let m3298 = f238(v0.clone(), f8(Str::lit("anyOf")));
        match m3298 {
            None => {
                Adt17::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f639(f506(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f595(f333(v2))
    } else {
        false
    }
}

pub fn f643(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3299 = f475(v0, v1);
    let a3300 = f609(v0, v2);
    fixed_u32::bit_or(a3299, a3300)
}

pub fn f644(v0: u32) -> u32 {
    let a3303 = {
        let a3301 = f643(v0, 17u32, 15u32);
        let a3302 = f643(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3301, a3302)
    };
    let a3304 = f475(v0, 10u32);
    fixed_u32::bit_xor(a3303, a3304)
}

pub fn f645(v0: u32) -> u32 {
    let a3307 = {
        let a3305 = f643(v0, 7u32, 25u32);
        let a3306 = f643(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3305, a3306)
    };
    let a3308 = f475(v0, 3u32);
    fixed_u32::bit_xor(a3307, a3308)
}

pub fn f646(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3311 = {
        let a3309 = v0.clone();
        let a3310 = v1;
        index_bytes(a3309, a3310)
    };
    match m3311 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            let a3312 = v3;
            let a3313 = 128u64;
            if nat_lt(a3312, a3313) {
                Ok(Some((v3, 1u64)))
            } else {
                if f227({
                    let a3314 = 194u64;
                    let a3315 = v3;
                    nat_le(a3314, a3315)
                }, {
                    let a3316 = v3;
                    let a3317 = 223u64;
                    nat_le(a3316, a3317)
                }) {
                    let m3320 = f670(v0, {
                        let a3318 = v1;
                        let a3319 = 1u64;
                        nat_add(a3318, a3319)?
                    })?;
                    match m3320 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f671({
                                let a3325 = {
                                    let a3323 = {
                                        let a3321 = v3;
                                        let a3322 = 192u64;
                                        nat_sub(a3321, a3322)
                                    };
                                    let a3324 = 64u64;
                                    nat_mul(a3323, a3324)?
                                };
                                let a3326 = v4;
                                nat_add(a3325, a3326)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f227({
                        let a3327 = 224u64;
                        let a3328 = v3;
                        nat_le(a3327, a3328)
                    }, {
                        let a3329 = v3;
                        let a3330 = 239u64;
                        nat_le(a3329, a3330)
                    }) {
                        let m3333 = f670(v0.clone(), {
                            let a3331 = v1;
                            let a3332 = 1u64;
                            nat_add(a3331, a3332)?
                        })?;
                        match m3333 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3336 = f670(v0, {
                                    let a3334 = v1;
                                    let a3335 = 2u64;
                                    nat_add(a3334, a3335)?
                                })?;
                                match m3336 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f671({
                                            let a3345 = {
                                                let a3339 = {
                                                    let a3337 = v3;
                                                    let a3338 = 224u64;
                                                    nat_sub(a3337, a3338)
                                                };
                                                let a3340 = 4096u64;
                                                nat_mul(a3339, a3340)?
                                            };
                                            let a3346 = {
                                                let a3343 = {
                                                    let a3341 = v5;
                                                    let a3342 = 64u64;
                                                    nat_mul(a3341, a3342)?
                                                };
                                                let a3344 = v6;
                                                nat_add(a3343, a3344)?
                                            };
                                            nat_add(a3345, a3346)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f227({
                            let a3347 = 240u64;
                            let a3348 = v3;
                            nat_le(a3347, a3348)
                        }, {
                            let a3349 = v3;
                            let a3350 = 244u64;
                            nat_le(a3349, a3350)
                        }) {
                            let m3353 = f670(v0.clone(), {
                                let a3351 = v1;
                                let a3352 = 1u64;
                                nat_add(a3351, a3352)?
                            })?;
                            match m3353 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3356 = f670(v0.clone(), {
                                        let a3354 = v1;
                                        let a3355 = 2u64;
                                        nat_add(a3354, a3355)?
                                    })?;
                                    match m3356 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3359 = f670(v0, {
                                                let a3357 = v1;
                                                let a3358 = 3u64;
                                                nat_add(a3357, a3358)?
                                            })?;
                                            match m3359 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f671({
                                                        let a3372 = {
                                                            let a3362 = {
                                                                let a3360 = v3;
                                                                let a3361 = 240u64;
                                                                nat_sub(a3360, a3361)
                                                            };
                                                            let a3363 = 262144u64;
                                                            nat_mul(a3362, a3363)?
                                                        };
                                                        let a3373 = {
                                                            let a3370 = {
                                                                let a3364 = v7;
                                                                let a3365 = 4096u64;
                                                                nat_mul(a3364, a3365)?
                                                            };
                                                            let a3371 = {
                                                                let a3368 = {
                                                                    let a3366 = v8;
                                                                    let a3367 = 64u64;
                                                                    nat_mul(a3366, a3367)?
                                                                };
                                                                let a3369 = v9;
                                                                nat_add(a3368, a3369)?
                                                            };
                                                            nat_add(a3370, a3371)?
                                                        };
                                                        nat_add(a3372, a3373)?
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

pub fn f647(v0: bool) -> bool {
    !v0
}

pub fn f648(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Result<List<Adt30>, Bytes>> {
    let m3374 = v0;
    if m3374 == 0 {
        Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
    } else {
        let v4 = m3374 - 1;
        let a3375 = v3;
        let a3376 = v2;
        if nat_le(a3375, a3376) {
            Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
        } else {
            let v5: u64 = f346(v1.clone(), v2)?;
            let m3425 = {
                let a3377 = v5;
                let a3378 = 91u64;
                if nat_eq(a3377, a3378) {
                    let a3381 = f346(v1.clone(), {
                        let a3379 = v2;
                        let a3380 = 1u64;
                        nat_add(a3379, a3380)?
                    })?;
                    let a3382 = 94u64;
                    if nat_eq(a3381, a3382) {
                        let m3386 = f672({
                            let a3383 = v1.clone();
                            length_bytes(a3383)
                        }, v1.clone(), {
                            let a3384 = v2;
                            let a3385 = 2u64;
                            nat_add(a3384, a3385)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3386 {
                            Ok(v7) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, {
                                    let (h3387, _) = v7;
                                    h3387
                                }), {
                                    let (_, h3388) = v7;
                                    h3388
                                }))
                            }
                            Err(v6) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v6)
                            }
                        }
                    } else {
                        let m3392 = f672({
                            let a3389 = v1.clone();
                            length_bytes(a3389)
                        }, v1.clone(), {
                            let a3390 = v2;
                            let a3391 = 1u64;
                            nat_add(a3390, a3391)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3392 {
                            Ok(v9) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, {
                                    let (h3393, _) = v9;
                                    h3393
                                }), {
                                    let (_, h3394) = v9;
                                    h3394
                                }))
                            }
                            Err(v8) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v8)
                            }
                        }
                    }
                } else {
                    let a3395 = v5;
                    let a3396 = 46u64;
                    if nat_eq(a3395, a3396) {
                        Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, List::cons((10u64, 10u64), List::cons((13u64, 13u64), List::cons((8232u64, 8233u64), List::<(u64, u64)>::nil())))), {
                            let a3397 = v2;
                            let a3398 = 1u64;
                            nat_add(a3397, a3398)?
                        }))
                    } else {
                        let a3399 = v5;
                        let a3400 = 40u64;
                        if if if if if if nat_eq(a3399, a3400) {
                            true
                        } else {
                            let a3401 = v5;
                            let a3402 = 41u64;
                            nat_eq(a3401, a3402)
                        } {
                            true
                        } else {
                            let a3403 = v5;
                            let a3404 = 124u64;
                            if nat_eq(a3403, a3404) {
                                true
                            } else {
                                let a3405 = v5;
                                let a3406 = 123u64;
                                nat_eq(a3405, a3406)
                            }
                        } {
                            true
                        } else {
                            let a3407 = v5;
                            let a3408 = 125u64;
                            if if nat_eq(a3407, a3408) {
                                true
                            } else {
                                let a3409 = v5;
                                let a3410 = 92u64;
                                nat_eq(a3409, a3410)
                            } {
                                true
                            } else {
                                let a3411 = v5;
                                let a3412 = 94u64;
                                if nat_eq(a3411, a3412) {
                                    true
                                } else {
                                    let a3413 = v5;
                                    let a3414 = 36u64;
                                    nat_eq(a3413, a3414)
                                }
                            }
                        } {
                            true
                        } else {
                            let a3415 = v5;
                            let a3416 = 42u64;
                            if if nat_eq(a3415, a3416) {
                                true
                            } else {
                                let a3417 = v5;
                                let a3418 = 43u64;
                                nat_eq(a3417, a3418)
                            } {
                                true
                            } else {
                                let a3419 = v5;
                                let a3420 = 63u64;
                                nat_eq(a3419, a3420)
                            }
                        } {
                            true
                        } else {
                            let a3421 = 127u64;
                            let a3422 = v5;
                            nat_lt(a3421, a3422)
                        } {
                            Err::<((bool, List<(u64, u64)>), u64), Bytes>(f7(List::cons(f8(Str::lit("the pattern construct ")), List::cons(f35(v1.clone(), v2, 1u64), List::cons(f8(Str::lit(" is not evaluated: only ^, $, ., character classes, literal ASCII characters and the quantifiers *, + and ? are (ECMA-262 section 22.2.1)")), List::<Bytes>::nil())))))
                        } else {
                            Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, List::cons((v5, v5), List::<(u64, u64)>::nil())), {
                                let a3423 = v2;
                                let a3424 = 1u64;
                                nat_add(a3423, a3424)?
                            }))
                        }
                    }
                }
            };
            match m3425 {
                Ok(v11) => {
                    let v12: u64 = {
                        let (_, h3426) = v11.clone();
                        h3426
                    };
                    let v13: u64 = {
                        let a3427 = v12;
                        let a3428 = v3;
                        if nat_lt(a3427, a3428) {
                            f673(f346(v1.clone(), v12)?)
                        } else {
                            0u64
                        }
                    };
                    let a3429 = v12;
                    let a3430 = v3;
                    if if nat_lt(a3429, a3430) {
                        let a3431 = f346(v1.clone(), v12)?;
                        let a3432 = 123u64;
                        nat_eq(a3431, a3432)
                    } else {
                        false
                    } {
                        Ok(Err::<List<Adt30>, Bytes>(f8(Str::lit("a {n,m} quantifier in a pattern is not evaluated (ECMA-262 section 22.2.1)"))))
                    } else {
                        let m3437 = f648(v4, v1, {
                            let a3433 = 0u64;
                            let a3434 = v13;
                            if nat_lt(a3433, a3434) {
                                let a3435 = v12;
                                let a3436 = 1u64;
                                nat_add(a3435, a3436)?
                            } else {
                                v12
                            }
                        }, v3)?;
                        match m3437 {
                            Ok(v15) => {
                                Ok(Ok::<List<Adt30>, Bytes>(List::cons(Adt30::C0({
                                    let (h3438, _) = {
                                        let (h3439, _) = v11.clone();
                                        h3439
                                    };
                                    h3438
                                }, {
                                    let (_, h3440) = {
                                        let (h3441, _) = v11;
                                        h3441
                                    };
                                    h3440
                                }, v13), v15)))
                            }
                            Err(v14) => {
                                Ok(Err::<List<Adt30>, Bytes>(v14))
                            }
                        }
                    }
                }
                Err(v10) => {
                    Ok(Err::<List<Adt30>, Bytes>(v10))
                }
            }
        }
    }
}

pub fn f649(v0: Adt30, v1: u64) -> bool {
    let m3442 = v0;
    match m3442 {
        Adt30::C0(v2, v3, _) => {
            if v2 {
                f647(f674(v3, v1))
            } else {
                f674(v3, v1)
            }
        }
    }
}

pub fn f650(v0: List<Adt30>, p3449: List<u64>, v2: bool) -> R<bool> {
    let m3448 = p3449;
    {
        let mut m3448 = m3448;
        loop {
            let step = {
                let v1 = m3448;
                let m3443 = v1;
                match m3443.uncons() {
                    None => {
                        Err::<List<u64>, R<bool>>(Ok(false))
                    }
                    Some((_, v4)) => {
                        if f613({
                            let a3446 = {
                                let a3444 = f614(v0.clone())?;
                                let a3445 = f563(v4.clone())?;
                                nat_add(a3444, a3445)?
                            };
                            let a3447 = 2u64;
                            nat_add(a3446, a3447)?
                        }, v0.clone(), v4.clone(), v2)? {
                            Err::<List<u64>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<u64>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f651(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3450 = v0.clone();
        length_bytes(a3450)
    };
    let a3454 = {
        let a3451 = v1;
        let a3452 = 2u64;
        let a3453 = 0u64;
        nat_rem(a3451, a3452, a3453)
    };
    let a3455 = 0u64;
    if nat_eq(a3454, a3455) {
        let (_, h3456) = {
            let (h3457, _) = f675(Fn24::F676(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3457
        };
        Ok(h3456)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f652(v0: u64) -> bool {
    let a3458 = v0;
    let a3459 = 8u64;
    if nat_eq(a3458, a3459) {
        true
    } else {
        let a3460 = v0;
        let a3461 = 9u64;
        if nat_eq(a3460, a3461) {
            true
        } else {
            let a3462 = v0;
            let a3463 = 10u64;
            if nat_eq(a3462, a3463) {
                true
            } else {
                let a3464 = v0;
                let a3465 = 12u64;
                if nat_eq(a3464, a3465) {
                    true
                } else {
                    let a3466 = v0;
                    let a3467 = 13u64;
                    nat_eq(a3466, a3467)
                }
            }
        }
    }
}

pub fn f653(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3468, _) = v0;
        h3468
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3469) = v0;
        h3469
    };
    let m3470 = v1;
    match m3470.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3471 = v4;
            match m3471.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f677(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f654(v0: Fn23, p3476: u64, p3477: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3475 = (p3476, p3477);
    {
        let mut m3475 = m3475;
        loop {
            let step = {
                let (v1, v2) = m3475;
                let m3472 = v1;
                if m3472 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3472 - 1;
                    let m3474 = {
                        let c3473: Fn23 = v0.clone();
                        c3473.apply(v2.clone())
                    };
                    match m3474 {
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
                Ok(next) => { m3475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f655(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f678(v0)
}

pub fn f656(v0: bool, v1: Adt17) -> R<Option<Bytes>> {
    let m3478 = v1;
    match m3478 {
        Adt17::C0 => {
            Ok(None::<Bytes>)
        }
        Adt17::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt17::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C3(v7, v8) => {
            Ok(Some({
                let a3489 = if f227(v7, {
                    let a3487 = 0u64;
                    let a3488 = v8;
                    nat_lt(a3487, a3488)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3490 = f261(false, v8)?;
                append_bytes(a3489, a3490)
            }))
        }
        Adt17::C4(v3, v4, v5) => {
            let m3481 = f679({
                let a3479 = f8(Str::lit("n"));
                let a3480 = f411(false, v4, v5)?;
                append_bytes(a3479, a3480)
            })?;
            match m3481 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3485 = f8(Str::lit("nMinus"));
                        let a3486 = f35(v6.clone(), 1u64, {
                            let a3483 = {
                                let a3482 = v6;
                                length_bytes(a3482)
                            };
                            let a3484 = 1u64;
                            nat_sub(a3483, a3484)
                        });
                        append_bytes(a3485, a3486)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt17::C5(v2) => {
            f661(v0, v2)
        }
        Adt17::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C8 => {
            Ok(None::<Bytes>)
        }
        Adt17::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt17::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f657(v0: Bytes) -> R<Option<Bytes>> {
    let m3491 = f680(v0)?;
    match m3491 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f681(v1)?))
        }
    }
}

pub fn f658(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3492 = v1;
    match m3492.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3493 = f661(v0, v2)?;
            match m3493 {
                None => {
                    f658(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f658(v0, v3)?))
                }
            }
        }
    }
}

pub fn f659(p3497: List<Adt17>, v1: Bytes) -> bool {
    let m3496 = p3497;
    {
        let mut m3496 = m3496;
        loop {
            let step = {
                let v0 = m3496;
                let m3494 = v0;
                match m3494.uncons() {
                    None => {
                        Err::<List<Adt17>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3495 = f669(v2, v1.clone());
                        if match m3495 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt17>, bool>(v3)
                        } else {
                            Err::<List<Adt17>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3496 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f660(v0: Adt17) -> R<Option<Bytes>> {
    let m3498 = f332(f238(v0.clone(), f8(Str::lit("$ref"))));
    match m3498 {
        None => {
            Ok(f332(f238(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f581(v1)
        }
    }
}

pub fn f661(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f679(v1)
    } else {
        f657(v1)
    }
}

pub fn f662(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f682(f550(v0), v1)
}

pub fn f663(v0: bool, v1: Bytes, v2: List<Adt17>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3499 = v2;
    match m3499.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3500 = f669(v5.clone(), v3.clone());
            match m3500 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3501 = f661(v0, v7.clone())?;
                    match m3501 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f41(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3504 = f588({
                                    let a3502 = v1.clone();
                                    let a3503 = f326(v7)?;
                                    append_bytes(a3502, a3503)
                                }, f508(f465(f238(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f242(f243(f238(v5, f8(Str::lit("required"))))), true)?;
                                match m3504 {
                                    Ok(v10) => {
                                        let m3505 = f663(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3505 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f342(f590(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f590(v12.clone())), f469(f591(v10), f591(v12)))))
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

pub fn f664(v0: List<Adt17>, v1: Bytes) -> List<Bytes> {
    let m3506 = v0;
    match m3506.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3507 = f669(v2, v1.clone());
            match m3507 {
                None => {
                    f664(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f664(v3, v1))
                }
            }
        }
    }
}

pub fn f665(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3508 = f661(v0, v1)?;
    match m3508 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f28(v2.clone(), f8(Str::lit("mk"))) {
                f548(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f666(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3509 = v1;
    match m3509.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3510 = f665(v0, v2)?;
            match m3510 {
                None => {
                    f666(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f666(v0, v3)?))
                }
            }
        }
    }
}

pub fn f667(v0: Adt17) -> Option<Adt17> {
    let m3511 = f465(f238(v0, f8(Str::lit("patternProperties"))));
    match m3511.uncons() {
        None => {
            None::<Adt17>
        }
        Some((v1, v2)) => {
            let m3512 = v2;
            match m3512.uncons() {
                None => {
                    Some({
                        let (_, h3513) = v1;
                        h3513
                    })
                }
                Some((_, _)) => {
                    None::<Adt17>
                }
            }
        }
    }
}

pub fn f668(v0: Adt17) -> bool {
    f28(f241(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f669(v0: Adt17, v1: Bytes) -> Option<Bytes> {
    if f41(v1.clone(), f242(f243(f238(v0.clone(), f8(Str::lit("required")))))) {
        let m3514 = f238(v0, f8(Str::lit("properties")));
        match m3514 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3515 = f238(v2, v1);
                match m3515 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f332(f238(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f670(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3518 = {
        let a3516 = v0;
        let a3517 = v1;
        index_bytes(a3516, a3517)
    };
    match m3518 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f227({
                let a3519 = 128u64;
                let a3520 = v3;
                nat_le(a3519, a3520)
            }, {
                let a3521 = v3;
                let a3522 = 191u64;
                nat_le(a3521, a3522)
            }) {
                Ok(Some({
                    let a3523 = v3;
                    let a3524 = 128u64;
                    nat_sub(a3523, a3524)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f671(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f227({
        let a3525 = v1;
        let a3526 = v0;
        nat_le(a3525, a3526)
    }, f227({
        let a3527 = v0;
        let a3528 = v2;
        nat_le(a3527, a3528)
    }, f429({
        let a3529 = v0;
        let a3530 = 55296u64;
        nat_lt(a3529, a3530)
    }, {
        let a3531 = 57343u64;
        let a3532 = v0;
        nat_lt(a3531, a3532)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f672(p3569: u64, v1: Bytes, p3570: u64, p3571: List<(u64, u64)>) -> R<Result<(List<(u64, u64)>, u64), Bytes>> {
    let m3568 = (p3569, (p3570, p3571));
    {
        let mut m3568 = m3568;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3568;
                let m3533 = v0;
                if m3533 == 0 {
                    Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                } else {
                    let v4 = m3533 - 1;
                    let v5: u64 = f346(v1.clone(), v2)?;
                    let a3535 = {
                        let a3534 = v1.clone();
                        length_bytes(a3534)
                    };
                    let a3536 = v2;
                    if nat_le(a3535, a3536) {
                        Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                    } else {
                        let a3537 = v5;
                        let a3538 = 93u64;
                        if nat_eq(a3537, a3538) {
                            Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Ok::<(List<(u64, u64)>, u64), Bytes>((v3, {
                                let a3539 = v2;
                                let a3540 = 1u64;
                                nat_add(a3539, a3540)?
                            }))))
                        } else {
                            let a3541 = v5;
                            let a3542 = 92u64;
                            if if nat_eq(a3541, a3542) {
                                true
                            } else {
                                let a3543 = 127u64;
                                let a3544 = v5;
                                nat_lt(a3543, a3544)
                            } {
                                Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("an escape or a non-ASCII character in a pattern's character class is not evaluated (ECMA-262 section 22.2.1)")))))
                            } else {
                                let a3547 = f346(v1.clone(), {
                                    let a3545 = v2;
                                    let a3546 = 1u64;
                                    nat_add(a3545, a3546)?
                                })?;
                                let a3548 = 45u64;
                                if if nat_eq(a3547, a3548) {
                                    f227({
                                        let a3552 = {
                                            let a3549 = v2;
                                            let a3550 = 2u64;
                                            nat_add(a3549, a3550)?
                                        };
                                        let a3553 = {
                                            let a3551 = v1.clone();
                                            length_bytes(a3551)
                                        };
                                        nat_lt(a3552, a3553)
                                    }, f227(f647({
                                        let a3556 = f346(v1.clone(), {
                                            let a3554 = v2;
                                            let a3555 = 2u64;
                                            nat_add(a3554, a3555)?
                                        })?;
                                        let a3557 = 93u64;
                                        nat_eq(a3556, a3557)
                                    }), f647({
                                        let a3560 = f346(v1.clone(), {
                                            let a3558 = v2;
                                            let a3559 = 2u64;
                                            nat_add(a3558, a3559)?
                                        })?;
                                        let a3561 = 92u64;
                                        nat_eq(a3560, a3561)
                                    })))
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3562 = v2;
                                        let a3563 = 3u64;
                                        nat_add(a3562, a3563)?
                                    }, List::cons((v5, f346(v1.clone(), {
                                        let a3564 = v2;
                                        let a3565 = 2u64;
                                        nat_add(a3564, a3565)?
                                    })?), v3))))
                                } else {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3566 = v2;
                                        let a3567 = 1u64;
                                        nat_add(a3566, a3567)?
                                    }, List::cons((v5, v5), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f673(v0: u64) -> u64 {
    let a3572 = v0;
    let a3573 = 42u64;
    if nat_eq(a3572, a3573) {
        1u64
    } else {
        let a3574 = v0;
        let a3575 = 43u64;
        if nat_eq(a3574, a3575) {
            2u64
        } else {
            let a3576 = v0;
            let a3577 = 63u64;
            if nat_eq(a3576, a3577) {
                3u64
            } else {
                0u64
            }
        }
    }
}

pub fn f674(p3586: List<(u64, u64)>, v1: u64) -> bool {
    let m3585 = p3586;
    {
        let mut m3585 = m3585;
        loop {
            let step = {
                let v0 = m3585;
                let m3578 = v0;
                match m3578.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3580 = {
                            let (h3579, _) = v2;
                            h3579
                        };
                        let a3581 = v1;
                        if if nat_le(a3580, a3581) {
                            let a3583 = v1;
                            let a3584 = {
                                let (_, h3582) = v2;
                                h3582
                            };
                            nat_le(a3583, a3584)
                        } else {
                            false
                        } {
                            Err::<List<(u64, u64)>, bool>(true)
                        } else {
                            Ok::<List<(u64, u64)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3585 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f675(v0: Fn24, p3591: u64, p3592: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3590 = (p3591, p3592);
    {
        let mut m3590 = m3590;
        loop {
            let step = {
                let (v1, v2) = m3590;
                let m3587 = v1;
                if m3587 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3587 - 1;
                    let m3589 = {
                        let c3588: Fn24 = v0.clone();
                        c3588.apply(v2.clone())?
                    };
                    match m3589 {
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
                Ok(next) => { m3590 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f676(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f683(v0, v1)
}

pub fn f677(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3597 = {
            let a3595 = {
                let a3593 = v0.clone();
                length_list(a3593)
            };
            let a3596 = {
                let a3594 = v1.clone();
                length_list(a3594)
            };
            nat_add(a3595, a3596)?
        };
        let a3598 = 1u64;
        nat_add(a3597, a3598)?
    };
    Ok(f684({
        let (_, h3599) = {
            let (_, h3600) = {
                let (h3601, _) = f685(Fn25::F686, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3601
            };
            h3600
        };
        h3599
    }))
}

pub fn f678(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3602, _) = v0.clone();
        h3602
    };
    let v2: List<(Bytes, ())> = {
        let (h3603, _) = {
            let (_, h3604) = v0.clone();
            h3604
        };
        h3603
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3605) = {
            let (_, h3606) = v0;
            h3606
        };
        h3605
    };
    let m3607 = v1.clone();
    match m3607.uncons() {
        None => {
            let m3608 = v2;
            match m3608.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3609 = v2.clone();
            match m3609.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f687({
                        let (h3610, _) = v8.clone();
                        h3610
                    }, {
                        let (h3611, _) = v6.clone();
                        h3611
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

pub fn f679(v0: Bytes) -> R<Option<Bytes>> {
    let m3613 = f688({
        let a3612 = v0.clone();
        length_bytes(a3612)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3613 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3615 = {
                let a3614 = v1.clone();
                length_bytes(a3614)
            };
            let a3616 = 0u64;
            if nat_eq(a3615, a3616) {
                let a3618 = {
                    let a3617 = v0;
                    length_bytes(a3617)
                };
                let a3619 = 0u64;
                if nat_eq(a3618, a3619) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f681(v1)?))
            }
        }
    }
}

pub fn f680(v0: Bytes) -> R<Option<Bytes>> {
    if f689({
        let a3620 = v0.clone();
        length_bytes(a3620)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f690(v0)?;
        if f691(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3623 = {
                let a3621 = v1.clone();
                let a3622 = 0u64;
                index_bytes(a3621, a3622)
            };
            match m3623 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f463(f62(v2)?) {
                        Ok(Some({
                            let a3624 = f8(Str::lit("n"));
                            let a3625 = v1;
                            append_bytes(a3624, a3625)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f692({
            let a3626 = v0.clone();
            length_bytes(a3626)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f681(v0: Bytes) -> R<Bytes> {
    if f41(v0.clone(), f693()) {
        f548(v0)
    } else {
        Ok(v0)
    }
}

pub fn f682(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3627 = v0.clone();
    match m3627 {
        Adt17::C0 => {
            Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C1(_, _) => {
            Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C2(v4) => {
            if v4 {
                Ok(f638())
            } else {
                Ok(f584(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt17::C3(_, _) => {
            Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C4(_, _, _) => {
            Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C5(_) => {
            Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C6(_) => {
            Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C7(_) => {
            let m3628 = f694(f506(v0.clone()));
            match m3628 {
                None => {
                    let m3629 = f332(f238(v0.clone(), f8(Str::lit("$ref"))));
                    match m3629 {
                        None => {
                            let m3631 = f238(v0.clone(), f8(Str::lit("allOf")));
                            match m3631 {
                                None => {
                                    let m3636 = f238(v0.clone(), f8(Str::lit("enum")));
                                    match m3636 {
                                        None => {
                                            let m3637 = f238(v0.clone(), f8(Str::lit("const")));
                                            match m3637 {
                                                None => {
                                                    let m3639 = f238(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3639 {
                                                        None => {
                                                            let v32: Bytes = f241(v0.clone(), f8(Str::lit("type")));
                                                            if if f28(v32.clone(), Bytes::lit(&[])) {
                                                                !f695(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f638())
                                                            } else {
                                                                if f28(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f585(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f28(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f585(f696(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f28(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f585(f697(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f28(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f585(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f28(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3640 = f465(f238(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3640.uncons() {
                                                                                        None => {
                                                                                            if f592(v0.clone()) {
                                                                                                Ok(f585(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3641 = f633(v0);
                                                                                                match m3641 {
                                                                                                    None => {
                                                                                                        Ok(f636(f8(Str::lit("List")), f698(f638())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f636(f8(Str::lit("List")), f698(f699(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f585(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f584(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f585(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3638 = f332(Some(v29));
                                                    match m3638 {
                                                        None => {
                                                            Ok(f584(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f585(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f585(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3632 = f333(v21);
                                    match m3632.uncons() {
                                        None => {
                                            Ok(f584(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3633 = v23;
                                            match m3633.uncons() {
                                                None => {
                                                    let m3634 = f332(f238(v22, f8(Str::lit("$ref"))));
                                                    match m3634 {
                                                        None => {
                                                            Ok(f584(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3635 = f581(v26)?;
                                                            match m3635 {
                                                                None => {
                                                                    Ok(f584(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f585(v27, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f584(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3630 = f581(v19)?;
                            match m3630 {
                                None => {
                                    Ok(f584(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f585(v20, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f584(if f28(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt17::C8 => {
            Ok(f638())
        }
        Adt17::C9(_, _) => {
            Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C10(_, _, _) => {
            Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f683(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3642, _) = v1;
        h3642
    };
    let m3644 = {
        let (_, h3643) = v1;
        h3643
    };
    match m3644 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3647 = {
                let a3645 = v0.clone();
                let a3646 = v2;
                index_bytes(a3645, a3646)
            };
            match m3647 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3652 = {
                        let a3650 = v0;
                        let a3651 = {
                            let a3648 = v2;
                            let a3649 = 1u64;
                            nat_add(a3648, a3649)?
                        };
                        index_bytes(a3650, a3651)
                    };
                    match m3652 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f700(v4)? {
                                f700(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3653 = v2;
                                    let a3654 = 2u64;
                                    nat_add(a3653, a3654)?
                                }, Some({
                                    let a3659 = v3;
                                    let a3660 = f492(f701(), {
                                        let a3657 = {
                                            let a3655 = f702(v4)?;
                                            let a3656 = 16u64;
                                            nat_mul(a3655, a3656)?
                                        };
                                        let a3658 = f702(v5)?;
                                        nat_add(a3657, a3658)?
                                    });
                                    append_bytes(a3659, a3660)
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

pub fn f684(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f354(Fn4::F703, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f685(v0: Fn25, p3665: u64, p3666: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3664 = (p3665, p3666);
    {
        let mut m3664 = m3664;
        loop {
            let step = {
                let (v1, v2) = m3664;
                let m3661 = v1;
                if m3661 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3661 - 1;
                    let m3663 = {
                        let c3662: Fn25 = v0.clone();
                        c3662.apply(v2.clone())
                    };
                    match m3663 {
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
                Ok(next) => { m3664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f704(v0)
}

pub fn f687(v0: Bytes, v1: Bytes) -> bool {
    let a3671 = {
        let a3667 = v0;
        let a3668 = v1;
        compare_bytes(a3667, a3668)
    };
    let a3672 = {
        let a3669 = Bytes::lit(&[]);
        let a3670 = Bytes::lit(&[0x01]);
        compare_bytes(a3669, a3670)
    };
    equal(a3671, a3672)
}

pub fn f688(p3685: u64, v1: Bytes, p3686: u64, p3687: Bytes) -> R<Option<Bytes>> {
    let m3684 = (p3685, (p3686, p3687));
    {
        let mut m3684 = m3684;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3684;
                let m3673 = v0;
                if m3673 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3673 - 1;
                    let a3675 = {
                        let a3674 = v1.clone();
                        length_bytes(a3674)
                    };
                    let a3676 = v2;
                    if nat_le(a3675, a3676) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3677 = f646(v1.clone(), v2)?;
                        match m3677 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3679 = v2;
                                    let a3680 = {
                                        let (_, h3678) = v5;
                                        h3678
                                    };
                                    nat_add(a3679, a3680)?
                                }, {
                                    let a3682 = v3;
                                    let a3683 = f705(v1.clone(), v2, {
                                        let (h3681, _) = v5;
                                        h3681
                                    });
                                    append_bytes(a3682, a3683)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3684 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f689(p3697: u64, v1: Bytes, p3698: u64) -> R<bool> {
    let m3696 = (p3697, p3698);
    {
        let mut m3696 = m3696;
        loop {
            let step = {
                let (v0, v2) = m3696;
                let m3688 = v0;
                if m3688 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3688 - 1;
                    let m3691 = {
                        let a3689 = v1.clone();
                        let a3690 = v2;
                        index_bytes(a3689, a3690)
                    };
                    match m3691 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3692 = f62(v4)?;
                            let a3693 = 128u64;
                            if nat_lt(a3692, a3693) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3694 = v2;
                                    let a3695 = 1u64;
                                    nat_add(a3694, a3695)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3696 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f690(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f407(v0.clone(), 0u64, {
        let a3699 = v0;
        length_bytes(a3699)
    }, false, Bytes::lit(&[]))?;
    let m3702 = {
        let a3700 = v1.clone();
        let a3701 = 0u64;
        index_bytes(a3700, a3701)
    };
    match m3702 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f462(v3) {
                let a3709 = f35({
                    let a3703 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3703)
                }, {
                    let a3704 = v3;
                    let a3705 = 65u64;
                    nat_sub(a3704, a3705)
                }, 1u64);
                let a3710 = f35(v1.clone(), 1u64, {
                    let a3707 = {
                        let a3706 = v1;
                        length_bytes(a3706)
                    };
                    let a3708 = 1u64;
                    nat_sub(a3707, a3708)
                });
                Ok(append_bytes(a3709, a3710))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f691(v0: Bytes) -> R<bool> {
    let m3713 = {
        let a3711 = v0;
        let a3712 = 0u64;
        index_bytes(a3711, a3712)
    };
    match m3713 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f429(f461(f62(v1)?), f462(f62(v1)?)))
        }
    }
}

pub fn f692(p3726: u64, v1: Bytes, p3727: u64, p3728: Bytes) -> R<Option<Bytes>> {
    let m3725 = (p3726, (p3727, p3728));
    {
        let mut m3725 = m3725;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3725;
                let m3714 = v0;
                if m3714 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3714 - 1;
                    let a3716 = {
                        let a3715 = v1.clone();
                        length_bytes(a3715)
                    };
                    let a3717 = v2;
                    if nat_le(a3716, a3717) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3718 = f646(v1.clone(), v2)?;
                        match m3718 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3720 = v2;
                                    let a3721 = {
                                        let (_, h3719) = v5;
                                        h3719
                                    };
                                    nat_add(a3720, a3721)?
                                }, {
                                    let a3723 = v3;
                                    let a3724 = f706({
                                        let (h3722, _) = v5;
                                        h3722
                                    });
                                    append_bytes(a3723, a3724)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f693() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f694(p3733: List<(Bytes, Adt17)>) -> Option<Bytes> {
    let m3732 = p3733;
    {
        let mut m3732 = m3732;
        loop {
            let step = {
                let v0 = m3732;
                let m3729 = v0;
                match m3729.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h3730, _) = v1.clone();
                            h3730
                        }, f707()) {
                            Err::<List<(Bytes, Adt17)>, Option<Bytes>>(Some({
                                let (h3731, _) = v1;
                                h3731
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt17)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f695(v0: Adt17) -> bool {
    if f597(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f597(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f597(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f597(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f597(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f597(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f696(v0: Adt17) -> Bytes {
    let m3734 = f564(f238(v0, f8(Str::lit("minimum"))));
    match m3734 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f697() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f698(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3735 = v0;
    match m3735 {
        Ok(v2) => {
            f585(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f514(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f513(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f699(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f708(16u64, v0, v1)
}

pub fn f700(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3780 = {
            let a3756 = {
                let a3744 = {
                    let a3738 = {
                        let a3736 = v1;
                        let a3737 = 128u8;
                        fixed_u8::bit_and(a3736, a3737)
                    };
                    let a3739 = 128u8;
                    if equal(a3738, a3739) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3745 = {
                    let a3742 = {
                        let a3740 = v1;
                        let a3741 = 64u8;
                        fixed_u8::bit_and(a3740, a3741)
                    };
                    let a3743 = 64u8;
                    if equal(a3742, a3743) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3744, a3745)?
            };
            let a3757 = {
                let a3754 = {
                    let a3748 = {
                        let a3746 = v1;
                        let a3747 = 32u8;
                        fixed_u8::bit_and(a3746, a3747)
                    };
                    let a3749 = 32u8;
                    if equal(a3748, a3749) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3755 = {
                    let a3752 = {
                        let a3750 = v1;
                        let a3751 = 16u8;
                        fixed_u8::bit_and(a3750, a3751)
                    };
                    let a3753 = 16u8;
                    if equal(a3752, a3753) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3754, a3755)?
            };
            nat_add(a3756, a3757)?
        };
        let a3781 = {
            let a3778 = {
                let a3766 = {
                    let a3760 = {
                        let a3758 = v1;
                        let a3759 = 8u8;
                        fixed_u8::bit_and(a3758, a3759)
                    };
                    let a3761 = 8u8;
                    if equal(a3760, a3761) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3767 = {
                    let a3764 = {
                        let a3762 = v1;
                        let a3763 = 4u8;
                        fixed_u8::bit_and(a3762, a3763)
                    };
                    let a3765 = 4u8;
                    if equal(a3764, a3765) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3766, a3767)?
            };
            let a3779 = {
                let a3776 = {
                    let a3770 = {
                        let a3768 = v1;
                        let a3769 = 2u8;
                        fixed_u8::bit_and(a3768, a3769)
                    };
                    let a3771 = 2u8;
                    if equal(a3770, a3771) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3777 = {
                    let a3774 = {
                        let a3772 = v1;
                        let a3773 = 1u8;
                        fixed_u8::bit_and(a3772, a3773)
                    };
                    let a3775 = 1u8;
                    if equal(a3774, a3775) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3776, a3777)?
            };
            nat_add(a3778, a3779)?
        };
        nat_add(a3780, a3781)?
    };
    let a3782 = 48u64;
    let a3783 = v2;
    if if nat_le(a3782, a3783) {
        let a3784 = v2;
        let a3785 = 57u64;
        nat_le(a3784, a3785)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3786 = 97u64;
        let a3787 = v2;
        if nat_le(a3786, a3787) {
            let a3788 = v2;
            let a3789 = 102u64;
            Ok(nat_le(a3788, a3789))
        } else {
            Ok(false)
        }
    }
}

pub fn f701() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f702(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3834 = {
            let a3810 = {
                let a3798 = {
                    let a3792 = {
                        let a3790 = v1;
                        let a3791 = 128u8;
                        fixed_u8::bit_and(a3790, a3791)
                    };
                    let a3793 = 128u8;
                    if equal(a3792, a3793) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3799 = {
                    let a3796 = {
                        let a3794 = v1;
                        let a3795 = 64u8;
                        fixed_u8::bit_and(a3794, a3795)
                    };
                    let a3797 = 64u8;
                    if equal(a3796, a3797) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3798, a3799)?
            };
            let a3811 = {
                let a3808 = {
                    let a3802 = {
                        let a3800 = v1;
                        let a3801 = 32u8;
                        fixed_u8::bit_and(a3800, a3801)
                    };
                    let a3803 = 32u8;
                    if equal(a3802, a3803) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3809 = {
                    let a3806 = {
                        let a3804 = v1;
                        let a3805 = 16u8;
                        fixed_u8::bit_and(a3804, a3805)
                    };
                    let a3807 = 16u8;
                    if equal(a3806, a3807) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3808, a3809)?
            };
            nat_add(a3810, a3811)?
        };
        let a3835 = {
            let a3832 = {
                let a3820 = {
                    let a3814 = {
                        let a3812 = v1;
                        let a3813 = 8u8;
                        fixed_u8::bit_and(a3812, a3813)
                    };
                    let a3815 = 8u8;
                    if equal(a3814, a3815) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3821 = {
                    let a3818 = {
                        let a3816 = v1;
                        let a3817 = 4u8;
                        fixed_u8::bit_and(a3816, a3817)
                    };
                    let a3819 = 4u8;
                    if equal(a3818, a3819) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3820, a3821)?
            };
            let a3833 = {
                let a3830 = {
                    let a3824 = {
                        let a3822 = v1;
                        let a3823 = 2u8;
                        fixed_u8::bit_and(a3822, a3823)
                    };
                    let a3825 = 2u8;
                    if equal(a3824, a3825) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3831 = {
                    let a3828 = {
                        let a3826 = v1;
                        let a3827 = 1u8;
                        fixed_u8::bit_and(a3826, a3827)
                    };
                    let a3829 = 1u8;
                    if equal(a3828, a3829) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3830, a3831)?
            };
            nat_add(a3832, a3833)?
        };
        nat_add(a3834, a3835)?
    };
    let a3836 = v2;
    let a3837 = 57u64;
    if nat_le(a3836, a3837) {
        let a3838 = v2;
        let a3839 = 48u64;
        Ok(nat_sub(a3838, a3839))
    } else {
        let a3840 = v2;
        let a3841 = 87u64;
        Ok(nat_sub(a3840, a3841))
    }
}

pub fn f703(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f704(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3842, _) = v0.clone();
        h3842
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3843, _) = {
            let (_, h3844) = v0.clone();
            h3844
        };
        h3843
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3845) = {
            let (_, h3846) = v0;
            h3846
        };
        h3845
    };
    let m3847 = v1.clone();
    match m3847.uncons() {
        None => {
            let m3848 = v2;
            match m3848.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3849 = v2.clone();
            match m3849.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f687({
                        let (h3850, _) = v8.clone();
                        h3850
                    }, {
                        let (h3851, _) = v6.clone();
                        h3851
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

pub fn f705(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f429(f227(f429(f461(v2), f462(v2)), f227({
        let a3852 = v2;
        let a3853 = 117u64;
        !nat_eq(a3852, a3853)
    }, {
        let a3854 = v2;
        let a3855 = 85u64;
        !nat_eq(a3854, a3855)
    })), f227(f463(v2), {
        let a3856 = 0u64;
        let a3857 = v1;
        nat_lt(a3856, a3857)
    })) {
        f35(v0, v1, 1u64)
    } else {
        let a3858 = v2;
        let a3859 = 65536u64;
        if nat_lt(a3858, a3859) {
            let a3860 = f8(Str::lit("u"));
            let a3861 = f582(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3860, a3861)
        } else {
            let a3862 = f8(Str::lit("U"));
            let a3863 = f582(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3862, a3863)
        }
    }
}

pub fn f706(v0: u64) -> Bytes {
    let a3864 = f8(Str::lit("u"));
    let a3865 = f582(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3864, a3865)
}

pub fn f707() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f708(v0: u64, v1: Adt17, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3866 = v0;
    if m3866 == 0 {
        Ok(f584(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3866 - 1;
        let m3867 = v1.clone();
        match m3867 {
            Adt17::C0 => {
                Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C1(_, _) => {
                Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C2(v6) => {
                if v6 {
                    Ok(f638())
                } else {
                    Ok(f584(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt17::C3(_, _) => {
                Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C4(_, _, _) => {
                Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C5(_) => {
                Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C6(_) => {
                Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C7(_) => {
                let m3868 = f332(f238(v1.clone(), f8(Str::lit("$ref"))));
                match m3868 {
                    None => {
                        if if f597(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f597(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f597(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f585(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v22: Bytes = f241(v1.clone(), f8(Str::lit("type")));
                            if if f28(v22.clone(), Bytes::lit(&[])) {
                                !f695(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f638())
                            } else {
                                if f28(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f585(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f28(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f585(f696(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f28(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f585(f697(), List::<Adt25>::nil()))
                                        } else {
                                            if f28(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f585(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f28(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3870 = f238(v1, f8(Str::lit("items")));
                                                    match m3870 {
                                                        None => {
                                                            Ok(f636(f8(Str::lit("List")), f638()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f636(f8(Str::lit("List")), f708(v3, v23, {
                                                                let a3871 = v2;
                                                                let a3872 = f8(Str::lit("Item"));
                                                                append_bytes(a3871, a3872)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f28(v22.clone(), f8(Str::lit("object"))) {
                                                        if f592(v1.clone()) {
                                                            Ok(f585(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3873 = f633(v1);
                                                            match m3873 {
                                                                None => {
                                                                    Ok(f636(f8(Str::lit("List")), f698(f638())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f636(f8(Str::lit("List")), f698(f708(v3, v24, {
                                                                        let a3874 = v2;
                                                                        let a3875 = f8(Str::lit("Value"));
                                                                        append_bytes(a3874, a3875)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f584(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
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
                        let m3869 = f581(v20)?;
                        match m3869 {
                            None => {
                                Ok(f584(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f585(v21, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt17::C8 => {
                Ok(f638())
            }
            Adt17::C9(_, _) => {
                Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C10(_, _, _) => {
                Ok(f584(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
