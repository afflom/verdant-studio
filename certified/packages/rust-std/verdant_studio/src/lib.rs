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
    C0(u64, Adt20),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(u64, Option<u64>),
    C1(u64),
    C2(Adt21, List<u64>, u64),
}

#[derive(Clone)]
pub enum Adt21 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Bytes, Adt14),
    C1(Bytes),
    C2,
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt25, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F57(u8, Bytes),
    F181(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F57(k0, k1) => f57(*k0, k1.clone(), p0),
            Fn0::F181(k0) => f181(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F159,
    F177,
    F255,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F159 => f159(p0, p1),
            Fn1::F177 => f177(p0, p1),
            Fn1::F255 => f255(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F212(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F212(k0) => f212(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F245(Bytes, u64, Bytes, u64),
    F295(Bytes),
    F352(Bytes),
    F355(Bytes, Bytes),
    F356(Bytes),
    F357(Bytes, u64),
    F362(u64, Bytes),
    F406(bool, u64, Bytes),
    F446(Bytes),
    F447(Bytes),
    F482(Bytes),
    F524(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F245(k0, k1, k2, k3) => f245(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F295(k0) => f295(k0.clone(), p0),
            Fn3::F352(k0) => Ok(f352(k0.clone(), p0)),
            Fn3::F355(k0, k1) => f355(k0.clone(), k1.clone(), p0),
            Fn3::F356(k0) => f356(k0.clone(), p0),
            Fn3::F357(k0, k1) => f357(k0.clone(), *k1, p0),
            Fn3::F362(k0, k1) => f362(*k0, k1.clone(), p0),
            Fn3::F406(k0, k1, k2) => f406(*k0, *k1, k2.clone(), p0),
            Fn3::F446(k0) => f446(k0.clone(), p0),
            Fn3::F447(k0) => f447(k0.clone(), p0),
            Fn3::F482(k0) => f482(k0.clone(), p0),
            Fn3::F524(k0) => f524(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F288(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt23)) -> R<(u64, Adt23)> {
        match self {
            Fn4::F288(k0, k1, k2) => f288(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F294(Bytes),
    F307(u64, Bytes),
    F379(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F294(k0) => f294(k0.clone(), p0),
            Fn5::F307(k0, k1) => Ok(f307(*k0, k1.clone(), p0)),
            Fn5::F379(k0) => f379(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F332,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn6::F332 => f332(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F335,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn7::F335 => f335(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F337,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn8::F337 => f337(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F340,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn9::F340 => f340(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F381,
    F598,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F381 => f381(p0, p1),
            Fn10::F598 => f598(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F384,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F384 => f384(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F387,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F387 => f387(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F390,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F390 => f390(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F395,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt26) -> R<Adt26> {
        match self {
            Fn14::F395 => f395(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F408(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F408(k0) => f408(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F442(bool, Bytes),
    F449(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F442(k0, k1) => f442(*k0, k1.clone(), p0),
            Fn16::F449(k0) => f449(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F459,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F459 => f459(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F461,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F461 => f461(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F463,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F463 => f463(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F471,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F471 => f471(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F500,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F500 => f500(p0, p1),
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
    F564,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F564 => f564(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F566(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F566(k0) => f566(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F596,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F596 => f596(p0),
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
                                            if f36(v2.clone(), f8(Str::lit("read-xml"))) {
                                                f43(v1, v4)
                                            } else {
                                                if f36(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                    f44(v1.clone(), v4.clone(), f45(v1, v4)?)
                                                } else {
                                                    if f36(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                        f44(v1.clone(), v4.clone(), f46(v1, v4, false)?)
                                                    } else {
                                                        if f36(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                            f44(v1.clone(), v4.clone(), f46(v1, v4, true)?)
                                                        } else {
                                                            if f36(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                                f44(v1.clone(), v4.clone(), f47(v1, v4, false)?)
                                                            } else {
                                                                if f36(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                    f44(v1.clone(), v4.clone(), f47(v1, v4, true)?)
                                                                } else {
                                                                    if f36(v2.clone(), f8(Str::lit("admit"))) {
                                                                        f48(v1, v4)
                                                                    } else {
                                                                        if f36(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                            f44(v1.clone(), v4.clone(), f49(v1, v4)?)
                                                                        } else {
                                                                            if f36(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                                f50(v1, v4)
                                                                            } else {
                                                                                if f36(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                    f51(v1, v4)
                                                                                } else {
                                                                                    if f36(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                        f52(v1, v4)
                                                                                    } else {
                                                                                        if f36(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                            f53(v1, v4)
                                                                                        } else {
                                                                                            if f36(v2, f8(Str::lit("propose-encoding"))) {
                                                                                                let m179 = v5;
                                                                                                match m179.uncons() {
                                                                                                    None => {
                                                                                                        Ok(Adt11::C4(f35()))
                                                                                                    }
                                                                                                    Some((v6, v7)) => {
                                                                                                        f54(v1, v4, v6, v7)
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
}

pub fn f29(v0: u8) -> R<Str> {
    let m228 = {
        let a226 = f55();
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
        let (h230, _) = f56(Fn0::F57(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h230
    };
    let v4: u64 = {
        let (h231, _) = {
            let (_, h232) = v3.clone();
            h232
        };
        h231
    };
    Ok(f58(List::cons(f32(v0, v4, {
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
                        if f59(v1.clone(), 0u64, {
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
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | stage-mappings JSONL | stage-matrix JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | read-xml FILE | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f36(v0: Bytes, v1: Bytes) -> bool {
    let a251 = v0;
    let a252 = v1;
    equal(a251, a252)
}

pub fn f37(v0: Adt1) -> R<Adt11> {
    let m253 = f60(v0.clone(), f61());
    match m253.uncons() {
        None => {
            let m254 = f62(v0.clone(), f61())?;
            match m254.uncons() {
                None => {
                    let m257 = f63(match v0.clone() {
                        Adt1::C0(_, h255, _) => {
                            h255
                        }
                    }, f64(f65(), f66(match v0.clone() {
                        Adt1::C0(_, h256, _) => {
                            h256
                        }
                    }, false)));
                    match m257.uncons() {
                        None => {
                            let m259 = f60(v0.clone(), f66(match v0.clone() {
                                Adt1::C0(_, h258, _) => {
                                    h258
                                }
                            }, true));
                            match m259.uncons() {
                                None => {
                                    let m261 = f67(v0.clone(), f66(match v0.clone() {
                                        Adt1::C0(_, h260, _) => {
                                            h260
                                        }
                                    }, true))?;
                                    match m261 {
                                        None => {
                                            let v10: (bool, Bytes) = f68(Adt13::C0(Adt12::C0(f69(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f71(v0.clone())?, f72(f73(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f69(f70(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f72(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f73(v0.clone(), f8(Str::lit("model/strata.json")))?, f74(f73(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f66(match v0 {
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
    if f75(f76(match v0.clone() {
        Adt1::C0(_, h266, _) => {
            h266
        }
    }, f77())) {
        Ok(Adt11::C1(List::cons(f77(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f78(f79(f77(), f80(match v0.clone() {
            Adt1::C0(_, h267, _) => {
                h267
            }
        }, f77())))?;
        let m268 = f60(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m268.uncons() {
            None => {
                let m269 = f81(v0.clone(), f82(f83(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f84(v2, v1.clone()))?;
                match m269 {
                    None => {
                        f85(v0, v1)
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
    if f75(f86(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h270) => {
            h270
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f87(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f75(f86(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h271) => {
                h271
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f87(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f40(v0: Adt1) -> R<Adt11> {
    let m272 = f88(v0.clone());
    match m272.uncons() {
        None => {
            let m273 = f64(f60(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f89(), List::<Bytes>::nil())))), f90(v0.clone())?);
            match m273.uncons() {
                None => {
                    Ok(Adt11::C3(f91(f92(v0.clone())?, List::cons((f93(), f94()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f95()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f96()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f97()), List::cons((f8(Str::lit("docs/catalog.md")), f98(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f99(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f100(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f101()), List::<(Bytes, Bytes)>::nil()))))))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m274 = f88(v0.clone());
    match m274.uncons() {
        None => {
            let m275 = f64(f60(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f90(v0.clone())?);
            match m275.uncons() {
                None => {
                    let v5: Adt15 = f102(v0.clone())?;
                    let v6: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f64(f103(f104(match v5.clone() {
                        Adt15::C0(h276, _, _) => {
                            h276
                        }
                    }, f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f106(v6.clone())), f107(f104(match v5.clone() {
                        Adt15::C0(h277, _, _) => {
                            h277
                        }
                    }, f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f108(v5.clone(), v6, f109(v5, f110(v0.clone(), f111(v0)?)?))?)))))
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
    let m278 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m278.uncons() {
        None => {
            let v4: Bytes = f70(v0.clone(), v1);
            let v5: List<List<Bytes>> = f112(v4.clone())?;
            if f75(f113(f114(v5.clone()), f115())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f116(Str::lit("id"), v6.clone())?, List::cons(f116(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f116(Str::lit("artifact"), f117())?, List::cons(f116(Str::lit("checksum"), f118(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f117(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f119(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f120(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f121(f122(f123(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f43(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m279 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m279.uncons() {
        None => {
            let m280 = f124(f70(v0, v1))?;
            match m280 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f125(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f126(f127(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f44(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m281 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m281.uncons() {
        None => {
            if f128(f83(f70(v0, v1))?)? {
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

pub fn f45(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m282 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m282.uncons() {
        None => {
            let m283 = f129(f83(f70(v0, v1))?)?;
            match m283 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f121(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f46(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m284 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m284.uncons() {
        None => {
            let v5: Bytes = f130(if v2 {
                f131()
            } else {
                f132()
            }, f133(v1.clone())?);
            if f75(f134(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m285 = f135(f83(f70(v0.clone(), v1.clone()))?, v2)?;
                match m285 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a287 = {
                            let a286 = v6;
                            length_list(a286)
                        };
                        let a288 = 0u64;
                        if nat_eq(a287, a288) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f70(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f47(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m289 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m289.uncons() {
        None => {
            let v5: List<Bytes> = f83(f70(v0.clone(), v1.clone()))?;
            if f75(f134(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f136(v5)?
                } else {
                    f137(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f130(f138(), f133(v1.clone())?), f70(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f130(f138(), f133(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f139()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f48(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m290 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m290.uncons() {
        None => {
            let v4: List<Bytes> = f83(f70(v0.clone(), v1))?;
            let m291 = f140(v4.clone(), f8(Str::lit("requirement")))?;
            match m291 {
                None => {
                    Ok(Adt11::C4(f139()))
                }
                Some(v5) => {
                    let m292 = f140(v4, f8(Str::lit("admission")))?;
                    match m292 {
                        None => {
                            Ok(Adt11::C4(f139()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f64(f72(v5.clone(), f8(Str::lit("id"))), f72(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f78(f64(f72(v7.clone(), f8(Str::lit("id"))), f72(v8.clone(), f8(Str::lit("construct")))))?;
                            let m293 = f141(f106(v6.clone()), f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m293 {
                                None => {
                                    let a295 = {
                                        let a294 = v9.clone();
                                        length_list(a294)
                                    };
                                    let a296 = 0u64;
                                    if nat_eq(a295, a296) {
                                        Ok(Adt11::C4(f139()))
                                    } else {
                                        if f75({
                                            let a299 = {
                                                let a297 = f78(f64(v9.clone(), v10.clone()))?;
                                                length_list(a297)
                                            };
                                            let a300 = {
                                                let a298 = f64(v9.clone(), v10);
                                                length_list(a298)
                                            };
                                            nat_eq(a299, a300)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f143(f8(Str::lit("model/requirements.json")), f144(v7, v5))?, List::cons(f143(f8(Str::lit("model/admissions.json")), f144(v8, v6))?, List::cons(f120(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f121(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f142(v11))
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

pub fn f49(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m301 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m301.uncons() {
        None => {
            let m302 = f145(f83(f70(v0, v1))?)?;
            match m302 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f121(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f50(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m303 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m303.uncons() {
        None => {
            let m304 = f146(v0.clone(), v1.clone())?;
            match m304 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m305 = f147(v4.clone(), f8(Str::lit("schema")));
                    match m305 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f148(v0, f149(v5.clone())?, v5, f150(v4.clone(), f8(Str::lit("name"))), f74(f151(f147(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
    let m306 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f89(), List::<Bytes>::nil())));
    match m306.uncons() {
        None => {
            let m307 = f152(f83(f70(v0.clone(), v1))?)?;
            match m307 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f73(v0, f89())?;
                    let v6: List<Bytes> = f72(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f78(f72(v5.clone(), f8(Str::lit("at"))))?;
                    let a309 = {
                        let a308 = v6.clone();
                        length_list(a308)
                    };
                    let a310 = 0u64;
                    if if nat_eq(a309, a310) {
                        true
                    } else {
                        f75({
                            let a313 = {
                                let a311 = f78(f64(v6.clone(), v7.clone()))?;
                                length_list(a311)
                            };
                            let a314 = {
                                let a312 = f64(v6.clone(), v7);
                                length_list(a312)
                            };
                            nat_eq(a313, a314)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f143(f89(), f144(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f121(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m315 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m315.uncons() {
        None => {
            let m316 = f146(v0, v1)?;
            match m316 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f153(f151(Some(v4))) {
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

pub fn f53(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m317 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m317.uncons() {
        None => {
            let m318 = f146(v0.clone(), v1.clone())?;
            match m318 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f75(f36(f150(v4.clone(), f8(Str::lit("schema"))), f154())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f150(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m319 = f147(v4.clone(), f8(Str::lit("source")));
                            match m319 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f155(f147(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f130(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f130(v9.clone(), f8(Str::lit("binding.json"))), f70(v0.clone(), v1));
                        let m320 = v8;
                        match m320 {
                            None => {
                                f156(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f75(f36(f150(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f156(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f130(v9, f133(f150(v7.clone(), f8(Str::lit("url"))))?);
                                    if f75(f86(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h321) => {
                                            h321
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f150(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m322 = f60(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m322.uncons() {
                                            None => {
                                                if f36(f118(f70(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f156(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f54(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m323 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m323.uncons() {
        None => {
            let m324 = f146(v0.clone(), v1.clone())?;
            match m324 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f148(v0.clone(), f70(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f55() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f56(v0: Fn0, p329: u64, p330: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m328 = (p329, p330);
    {
        let mut m328 = m328;
        loop {
            let step = {
                let (v1, v2) = m328;
                let m325 = v1;
                if m325 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m325 - 1;
                    let m327 = {
                        let c326: Fn0 = v0.clone();
                        c326.apply(v2.clone())?
                    };
                    match m327 {
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
                Ok(next) => { m328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f157(v1, v0, v2)
}

pub fn f58(v0: List<Bytes>) -> List<Bytes> {
    f158(Fn1::F159, List::<Bytes>::nil(), v0)
}

pub fn f59(v0: Bytes, p386: u64, p387: u64) -> R<bool> {
    let m385 = (p386, p387);
    {
        let mut m385 = m385;
        loop {
            let step = {
                let (v1, v2) = m385;
                let m331 = v2;
                if m331 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m331 - 1;
                    let m334 = {
                        let a332 = v0.clone();
                        let a333 = v1;
                        index_bytes(a332, a333)
                    };
                    match m334 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a381 = {
                                let v5: u8 = v4;
                                let a379 = {
                                    let a355 = {
                                        let a343 = {
                                            let a337 = {
                                                let a335 = v5;
                                                let a336 = 128u8;
                                                fixed_u8::bit_and(a335, a336)
                                            };
                                            let a338 = 128u8;
                                            if equal(a337, a338) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a344 = {
                                            let a341 = {
                                                let a339 = v5;
                                                let a340 = 64u8;
                                                fixed_u8::bit_and(a339, a340)
                                            };
                                            let a342 = 64u8;
                                            if equal(a341, a342) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a343, a344)?
                                    };
                                    let a356 = {
                                        let a353 = {
                                            let a347 = {
                                                let a345 = v5;
                                                let a346 = 32u8;
                                                fixed_u8::bit_and(a345, a346)
                                            };
                                            let a348 = 32u8;
                                            if equal(a347, a348) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a354 = {
                                            let a351 = {
                                                let a349 = v5;
                                                let a350 = 16u8;
                                                fixed_u8::bit_and(a349, a350)
                                            };
                                            let a352 = 16u8;
                                            if equal(a351, a352) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a353, a354)?
                                    };
                                    nat_add(a355, a356)?
                                };
                                let a380 = {
                                    let a377 = {
                                        let a365 = {
                                            let a359 = {
                                                let a357 = v5;
                                                let a358 = 8u8;
                                                fixed_u8::bit_and(a357, a358)
                                            };
                                            let a360 = 8u8;
                                            if equal(a359, a360) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a366 = {
                                            let a363 = {
                                                let a361 = v5;
                                                let a362 = 4u8;
                                                fixed_u8::bit_and(a361, a362)
                                            };
                                            let a364 = 4u8;
                                            if equal(a363, a364) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a365, a366)?
                                    };
                                    let a378 = {
                                        let a375 = {
                                            let a369 = {
                                                let a367 = v5;
                                                let a368 = 2u8;
                                                fixed_u8::bit_and(a367, a368)
                                            };
                                            let a370 = 2u8;
                                            if equal(a369, a370) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a376 = {
                                            let a373 = {
                                                let a371 = v5;
                                                let a372 = 1u8;
                                                fixed_u8::bit_and(a371, a372)
                                            };
                                            let a374 = 1u8;
                                            if equal(a373, a374) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a375, a376)?
                                    };
                                    nat_add(a377, a378)?
                                };
                                nat_add(a379, a380)?
                            };
                            let a382 = 127u64;
                            if nat_le(a381, a382) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a383 = v1;
                                    let a384 = 1u64;
                                    nat_add(a383, a384)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m388 = v1;
    match m388.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m390 = f160(match v0.clone() {
                Adt1::C0(h389, _, _) => {
                    h389
                }
            }, v2.clone());
            match m390 {
                None => {
                    List::cons(v2, f60(v0, v3))
                }
                Some(_) => {
                    f60(v0, v3)
                }
            }
        }
    }
}

pub fn f61() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f117(), List::<Bytes>::nil())))))))
}

pub fn f62(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m391 = v1;
    match m391.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f161(v0.clone(), v2.clone()) {
                f134(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m392 = f146(v0.clone(), v2.clone())?;
                match m392 {
                    None => {
                        Ok(List::cons(v2, f62(v0, v3)?))
                    }
                    Some(_) => {
                        f62(v0, v3)
                    }
                }
            } else {
                f62(v0, v3)
            }
        }
    }
}

pub fn f63(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m393 = v1;
    match m393.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f76(v0.clone(), v2.clone()) {
                f63(v0, v3)
            } else {
                List::cons(v2, f63(v0, v3))
            }
        }
    }
}

pub fn f64(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m394 = v0;
    match m394.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f64(v3, v1))
        }
    }
}

pub fn f65() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f66(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m395 = v0;
    match m395.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f64(f162({
                let (h396, _) = v2;
                h396
            }, {
                let (_, h397) = v2;
                h397
            }, v1), f66(v3, v1))
        }
    }
}

pub fn f67(v0: Adt1, p401: List<Bytes>) -> R<Option<Bytes>> {
    let m400 = p401;
    {
        let mut m400 = m400;
        loop {
            let step = {
                let v1 = m400;
                let m398 = v1;
                match m398.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m399 = f163(f70(v0.clone(), v2.clone()), f164())?;
                        match m399 {
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
                Ok(next) => { m400 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h402, _, _, _, _) => {
            h402
        }
    };
    let v2: Adt16 = f165(v1.clone())?;
    let v3: List<Bytes> = f166(v1.clone());
    let v4: List<Bytes> = f72(match v0.clone() {
        Adt13::C0(_, _, h403, _, _) => {
            h403
        }
    }, f8(Str::lit("path")));
    let m405 = f167(v2, List::<u64>::nil(), f168(v1.clone(), match v0.clone() {
        Adt13::C0(_, h404, _, _, _) => {
            h404
        }
    }, 0u64)?);
    match m405 {
        None => {
            if f170(f171(match v0.clone() {
                Adt13::C0(_, _, _, _, h407) => {
                    h407
                }
            }, f64(match v0.clone() {
                Adt13::C0(_, _, _, _, h408) => {
                    h408
                }
            }, v4.clone()))?, f171(v4.clone(), f64(match v0.clone() {
                Adt13::C0(_, _, _, _, h409) => {
                    h409
                }
            }, v4.clone()))?, f172(match v0.clone() {
                Adt13::C0(_, _, h410, _, _) => {
                    h410
                }
            })) {
                let m414 = f173(f171(match v0.clone() {
                    Adt13::C0(_, _, _, h411, _) => {
                        h411
                    }
                }, f64(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h412, _) => {
                        h412
                    }
                }))?, f171(v3.clone(), f64(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h413, _) => {
                        h413
                    }
                }))?);
                match m414 {
                    None => {
                        let m417 = f174(f72(match v0 {
                            Adt13::C0(_, h416, _, _, _) => {
                                h416
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m417 {
                            None => {
                                let m418 = f174(v4, List::<Bytes>::nil());
                                match m418 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f121(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f169(f64(v3, match v0 {
                            Adt13::C0(_, _, _, h415, _) => {
                                h415
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f169(match v1 {
                Adt12::C0(_, _, _, _, h406) => {
                    h406
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f69(v0: Bytes) -> R<List<Bytes>> {
    f175(f83(v0)?, f8(Str::lit("id = \"")))
}

pub fn f70(v0: Adt1, v1: Bytes) -> Bytes {
    let m420 = f160(match v0 {
        Adt1::C0(h419, _, _) => {
            h419
        }
    }, v1);
    match m420 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m421 = v2;
            match m421 {
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

pub fn f71(v0: Adt1) -> R<List<Bytes>> {
    let m422 = f112(f70(v0, f117()))?;
    match m422.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f122(v2))
        }
    }
}

pub fn f72(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m423 = v0;
    match m423.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f150(v2, v1.clone()), f72(v3, v1))
        }
    }
}

pub fn f73(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f151(f146(v0, v1)?))
}

pub fn f74(v0: List<Adt14>) -> List<Bytes> {
    let m424 = v0;
    match m424.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m425 = f155(Some(v1));
            match m425 {
                None => {
                    f74(v2)
                }
                Some(v3) => {
                    List::cons(v3, f74(v2))
                }
            }
        }
    }
}

pub fn f75(v0: bool) -> bool {
    !v0
}

pub fn f76(p429: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m428 = p429;
    {
        let mut m428 = m428;
        loop {
            let step = {
                let v0 = m428;
                let m426 = v0;
                match m426.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h427, _) = v2;
                            h427
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m428 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f78(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f176(f158(Fn1::F177, List::<Bytes>::nil(), f178(v0)?)))
}

pub fn f79(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m430 = v1;
    match m430.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f134(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f130(v0.clone(), v2), f79(v0, v3))
            } else {
                f79(v0, v3)
            }
        }
    }
}

pub fn f80(p435: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m434 = p435;
    {
        let mut m434 = m434;
        loop {
            let step = {
                let v0 = m434;
                let m431 = v0;
                match m431.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h432, _) = v2;
                            h432
                        }, v1.clone()) {
                            let (_, h433) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h433)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Adt1, v1: List<List<Bytes>>, p438: List<Bytes>) -> R<Option<Bytes>> {
    let m437 = p438;
    {
        let mut m437 = m437;
        loop {
            let step = {
                let v2 = m437;
                let m436 = v2;
                match m436.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f179(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(p441: List<Bytes>, p442: List<Bytes>, p443: List<List<Bytes>>) -> List<List<Bytes>> {
    let m440 = (p441, (p442, p443));
    {
        let mut m440 = m440;
        loop {
            let step = {
                let (v0, (v1, v2)) = m440;
                let m439 = v0;
                match m439.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f180(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f36(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f180(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a444 = v0.clone();
        length_bytes(a444)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h445, _) = f56(Fn0::F181(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h445
    };
    let v3: u64 = {
        let (h446, _) = {
            let (_, h447) = v2.clone();
            h447
        };
        h446
    };
    let v4: List<Bytes> = {
        let (_, h448) = {
            let (_, h449) = v2;
            h449
        };
        h448
    };
    let a450 = v3;
    let a451 = v1;
    if nat_lt(a450, a451) {
        Ok(f58(List::cons(f32(v0, v3, {
            let a452 = v1;
            let a453 = v3;
            nat_sub(a452, a453)
        }), v4)))
    } else {
        Ok(f58(v4))
    }
}

pub fn f84(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m454 = v0;
    match m454.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f84(v3, v1)
            } else {
                List::cons(v2, f84(v3, v1))
            }
        }
    }
}

pub fn f85(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f75(f76(match v0.clone() {
        Adt1::C0(_, h455, _) => {
            h455
        }
    }, f182())) {
        Ok(Adt11::C1(List::cons(f182(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f84(f78(f79(f182(), f80(match v0.clone() {
            Adt1::C0(_, h456, _) => {
                h456
            }
        }, f182())))?, v1.clone());
        let m457 = f60(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m457.uncons() {
            None => {
                let v5: List<Bytes> = f72(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m458 = f60(v0.clone(), f183(v0.clone(), v2.clone(), v5.clone())?);
                match m458.uncons() {
                    None => {
                        let m459 = f184(v0.clone(), v2, v5)?;
                        match m459 {
                            None => {
                                f185(v0, v1)
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

pub fn f86(v0: Bytes, p462: List<Bytes>) -> bool {
    let m461 = p462;
    {
        let mut m461 = m461;
        loop {
            let step = {
                let v1 = m461;
                let m460 = v1;
                match m460.uncons() {
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
                Ok(next) => { m461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f88(v0: Adt1) -> List<Bytes> {
    if f76(match v0.clone() {
        Adt1::C0(_, h463, _) => {
            h463
        }
    }, f132()) {
        if f76(match v0 {
            Adt1::C0(_, h464, _) => {
                h464
            }
        }, f131()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f131(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f132(), List::<Bytes>::nil())
    }
}

pub fn f89() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f90(v0: Adt1) -> R<List<Bytes>> {
    Ok(f60(v0.clone(), List::cons(f117(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f111(v0)?)))))
}

pub fn f91(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m465 = v0;
    match m465.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f91(v3, v1))
        }
    }
}

pub fn f92(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m466 = f73(v0.clone(), f89())?;
    match m466.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f186(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f93() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f94() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f95() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f187(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f96() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught when it opens, too.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  pull_request:\u{a}    types: [opened]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  refresh:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply each change to the new main, export it again, and push it to its own pull request\u{a}        if: steps.stale.outputs.stale != ''\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ steps.stale.outputs.stale }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again either way.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f187(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f97() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f98(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f102(v0.clone())?;
    let v2: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f109(v1.clone(), f110(v0.clone(), f111(v0.clone())?)?);
    let v4: List<List<Bytes>> = f123(f112(f70(v0, f117()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f188(v4.clone(), f122(v4), f189(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f108(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f99(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f190(f102(v0.clone())?, f110(v0.clone(), f111(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f100(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f191(f73(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f192(f73(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f101() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f102(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f71(v0.clone())?, f69(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f72(f73(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f103(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m467 = v0;
    match m467.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f103(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f103(v3, v1))
            }
        }
    }
}

pub fn f104(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m468 = v0;
    match m468.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f104(v3, v1)
            } else {
                List::cons(v2, f104(v3, v1))
            }
        }
    }
}

pub fn f105(v0: List<Adt14>) -> List<Bytes> {
    let m469 = v0;
    match m469.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f64(f74(f151(f147(v1, f8(Str::lit("defers"))))), f105(v2))
        }
    }
}

pub fn f106(v0: List<Adt14>) -> List<Bytes> {
    let m470 = v0;
    match m470.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m471 = f147(v1, f8(Str::lit("std")));
            match m471 {
                None => {
                    f106(v2)
                }
                Some(v3) => {
                    List::cons(f150(v3, f8(Str::lit("row"))), f106(v2))
                }
            }
        }
    }
}

pub fn f107(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m472 = v0;
    match m472.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f107(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f107(v3, v1))
            }
        }
    }
}

pub fn f108(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f91(f193(v2.clone()), f194(f189(v0, v1.clone(), v2)?));
    f195(f78(f196(v1))?, v3.clone(), {
        let a474 = {
            let a473 = v3;
            length_list(a473)
        };
        let a475 = 1u64;
        nat_add(a474, a475)?
    })
}

pub fn f109(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m476 = v1;
    match m476.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f197(v2.clone(), f8(Str::lit("subject"))) {
                f198(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f109(v0, v3))
            } else {
                f109(v0, v3)
            }
        }
    }
}

pub fn f110(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m477 = v1;
    match m477.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m478 = f135(f83(f70(v0.clone(), v2.clone()))?, f199(v2, f131()))?;
            match m478 {
                None => {
                    f110(v0, v3)
                }
                Some(v4) => {
                    Ok(f144(v4, f110(v0, v3)?))
                }
            }
        }
    }
}

pub fn f111(v0: Adt1) -> R<List<Bytes>> {
    Ok(f64(f78(f200(f132(), f80(match v0.clone() {
        Adt1::C0(_, h479, _) => {
            h479
        }
    }, f132())))?, f78(f200(f131(), f80(match v0 {
        Adt1::C0(_, h480, _) => {
            h480
        }
    }, f131())))?))
}

pub fn f112(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f201(v0.clone(), 0u64, {
        let a481 = v0;
        length_bytes(a481)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a484 = {
            let a483 = match v1.clone() {
                Adt17::C0(_, _, h482, _, _) => {
                    h482
                }
            };
            length_bytes(a483)
        };
        let a485 = 0u64;
        if if nat_eq(a484, a485) {
            let m487 = match v1.clone() {
                Adt17::C0(_, _, _, h486, _) => {
                    h486
                }
            };
            match m487.uncons() {
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
            f202(v1)
        }
    };
    Ok(f203(match v4 {
        Adt17::C0(_, _, _, _, h488) => {
            h488
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f113(p493: List<Bytes>, p494: List<Bytes>) -> bool {
    let m492 = (p493, p494);
    {
        let mut m492 = m492;
        loop {
            let step = {
                let (v0, v1) = m492;
                let m489 = v0;
                match m489.uncons() {
                    None => {
                        let m490 = v1;
                        match m490.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m491 = v1;
                        match m491.uncons() {
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
                Ok(next) => { m492 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<List<Bytes>>) -> List<Bytes> {
    let m495 = v0;
    match m495.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f115() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f116(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f204(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f117() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f118(v0: Bytes) -> R<Bytes> {
    Ok(f205(f206(v0)?))
}

pub fn f119(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f207(f82(f83(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f208(f209(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f120(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f149(Adt14::C3(std::rc::Rc::new(f210(f78(f64(v1, f74(f73(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f121(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m496 = f211(Fn2::F212(v1), None::<Bytes>, v0);
    match m496 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f122(v0: List<List<Bytes>>) -> List<Bytes> {
    let m497 = v0;
    match m497.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m498 = v1;
            match m498.uncons() {
                None => {
                    f122(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f122(v2))
                }
            }
        }
    }
}

pub fn f123(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m499 = v0;
    match m499.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f124(v0: Bytes) -> R<Result<Adt18, Bytes>> {
    let v1: Bytes = if f213(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f32(v0.clone(), 3u64, {
            let a501 = {
                let a500 = v0;
                length_bytes(a500)
            };
            let a502 = 3u64;
            nat_sub(a501, a502)
        })
    } else {
        v0
    };
    let v2: Bytes = f214(v1)?;
    let v3: u64 = {
        let a503 = v2.clone();
        length_bytes(a503)
    };
    let v4: u64 = f215(v2.clone())?;
    let a504 = v4;
    let a505 = v3;
    if nat_lt(a504, a505) {
        Ok(Err::<Adt18, Bytes>(f216(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m506 = f217(v2.clone())?;
        match m506 {
            Ok(v6) => {
                let m509 = f218(v2.clone(), v6, {
                    let a507 = v3;
                    let a508 = 2u64;
                    nat_add(a507, a508)?
                })?;
                match m509 {
                    Ok(v8) => {
                        if f219(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(Err::<Adt18, Bytes>(f216(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a510 = f220(v2.clone(), v8)?;
                            let a511 = 60u64;
                            if !nat_eq(a510, a511) {
                                Ok(Err::<Adt18, Bytes>(f216(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            } else {
                                let m514 = f221(v2.clone(), v8, {
                                    let a512 = v3;
                                    let a513 = 2u64;
                                    nat_add(a512, a513)?
                                }, 0u64)?;
                                match m514 {
                                    Ok(v10) => {
                                        let m518 = f218(v2, {
                                            let (_, h515) = v10;
                                            h515
                                        }, {
                                            let a516 = v3;
                                            let a517 = 2u64;
                                            nat_add(a516, a517)?
                                        })?;
                                        match m518 {
                                            Ok(v12) => {
                                                let a519 = v12;
                                                let a520 = v3;
                                                if nat_eq(a519, a520) {
                                                    Ok(Ok::<Adt18, Bytes>({
                                                        let (h521, _) = v10;
                                                        h521
                                                    }))
                                                } else {
                                                    Ok(Err::<Adt18, Bytes>(f216(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
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

pub fn f125(v0: Adt18) -> Bytes {
    let m522 = v0;
    match m522 {
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

pub fn f126(v0: u64) -> R<Bytes> {
    f222(f223(), 10u64, v0)
}

pub fn f127(v0: Adt18) -> R<u64> {
    let m523 = v0;
    match m523 {
        Adt18::C0(_, _, r528) => {
            let v3 = (*r528).clone();
            let a524 = 1u64;
            let a525 = f127(v3)?;
            nat_add(a524, a525)
        }
        Adt18::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt18::C2(_) => {
            Ok(0u64)
        }
        Adt18::C3(r529, r530) => {
            let v8 = (*r529).clone();
            let v9 = (*r530).clone();
            let a526 = f127(v8)?;
            let a527 = f127(v9)?;
            nat_add(a526, a527)
        }
        Adt18::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f128(p536: List<Bytes>) -> R<bool> {
    let m535 = p536;
    {
        let mut m535 = m535;
        loop {
            let step = {
                let v0 = m535;
                let m531 = v0;
                match m531.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a533 = {
                            let a532 = f224(v1)?;
                            length_bytes(a532)
                        };
                        let a534 = 0u64;
                        if nat_eq(a533, a534) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
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

pub fn f129(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m537 = v0;
    match m537.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f224(v1)?;
            let a539 = {
                let a538 = v3.clone();
                length_bytes(a538)
            };
            let a540 = 0u64;
            if nat_eq(a539, a540) {
                f129(v2)
            } else {
                let m541 = f225(v3.clone())?;
                match m541 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f150(v4.clone(), f8(Str::lit("id")));
                        if if f36(f150(v4, f8(Str::lit("schema"))), f154()) {
                            f75({
                                let a543 = {
                                    let a542 = v5.clone();
                                    length_bytes(a542)
                                };
                                let a544 = 0u64;
                                nat_eq(a543, a544)
                            })
                        } else {
                            false
                        } {
                            let m545 = f129(v2)?;
                            match m545 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f226(v5), {
                                        let a546 = v3;
                                        let a547 = Bytes::lit(&[0x0a]);
                                        append_bytes(a546, a547)
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

pub fn f130(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f131() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f132() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f133(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f134(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a548 = v0.clone();
        length_bytes(a548)
    };
    let v3: u64 = {
        let a549 = v1.clone();
        length_bytes(a549)
    };
    let a550 = v3;
    let a551 = v2;
    if nat_le(a550, a551) {
        let m557 = {
            let a554 = v0;
            let a555 = {
                let a552 = v2;
                let a553 = v3;
                nat_sub(a552, a553)
            };
            let a556 = v3;
            slice_bytes(a554, a555, a556)
        };
        match m557 {
            None => {
                false
            }
            Some(v4) => {
                let a558 = v4;
                let a559 = v1;
                equal(a558, a559)
            }
        }
    } else {
        false
    }
}

pub fn f135(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m560 = v0;
    match m560.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f224(v2)?;
            let a562 = {
                let a561 = v4.clone();
                length_bytes(a561)
            };
            let a563 = 0u64;
            if nat_eq(a562, a563) {
                f135(v3, v1)
            } else {
                let m564 = f225(v4)?;
                match m564 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f227(v5.clone())
                        } else {
                            f228(v5.clone())
                        } {
                            let m565 = f135(v3, v1)?;
                            match m565 {
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

pub fn f136(v0: List<Bytes>) -> R<bool> {
    if f229(f152(v0.clone())?) {
        Ok(f75({
            let a567 = {
                let a566 = f230(f152(v0)?);
                length_list(a566)
            };
            let a568 = 0u64;
            nat_eq(a567, a568)
        }))
    } else {
        Ok(false)
    }
}

pub fn f137(v0: List<Bytes>) -> R<bool> {
    if f229(f140(v0.clone(), f8(Str::lit("requirement")))?) {
        if f229(f140(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f75({
                let a570 = {
                    let a569 = f144(f230(f140(v0.clone(), f8(Str::lit("requirement")))?), f230(f140(v0, f8(Str::lit("admission")))?));
                    length_list(a569)
                };
                let a571 = 0u64;
                nat_eq(a570, a571)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f138() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f139() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f140(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m572 = v0;
    match m572.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f224(v2)?;
            let a574 = {
                let a573 = v4.clone();
                length_bytes(a573)
            };
            let a575 = 0u64;
            if nat_eq(a574, a575) {
                f140(v3, v1)
            } else {
                let m576 = f225(v4)?;
                match m576 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f231(v5.clone());
                        let a578 = {
                            let a577 = v6.clone();
                            length_bytes(a577)
                        };
                        let a579 = 0u64;
                        if nat_eq(a578, a579) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m580 = f140(v3, v1.clone())?;
                            match m580 {
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

pub fn f141(p583: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m582 = p583;
    {
        let mut m582 = m582;
        loop {
            let step = {
                let v0 = m582;
                let m581 = v0;
                match m581.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f86(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f143(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f149(Adt14::C3(std::rc::Rc::new(f232(v1))))?))
}

pub fn f144(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m584 = v0;
    match m584.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f144(v3, v1))
        }
    }
}

pub fn f145(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m585 = v0;
    match m585.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f224(v1)?;
            let a587 = {
                let a586 = v3.clone();
                length_bytes(a586)
            };
            let a588 = 0u64;
            if nat_eq(a587, a588) {
                f145(v2)
            } else {
                let m589 = f225(v3.clone())?;
                match m589 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f233(v4.clone()) {
                            let m590 = f145(v2)?;
                            match m590 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f182(), List::cons(f8(Str::lit("/")), List::cons(f234(f150(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a591 = v3;
                                        let a592 = Bytes::lit(&[0x0a]);
                                        append_bytes(a591, a592)
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

pub fn f146(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f225(f70(v0, v1))
}

pub fn f147(p598: Adt14, v1: Bytes) -> Option<Adt14> {
    let m597 = p598;
    {
        let mut m597 = m597;
        loop {
            let step = {
                let v0 = m597;
                let m593 = v0;
                match m593 {
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
                    Adt14::C4(r594) => {
                        let v5 = (*r594).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r595, r596) => {
                        let v3 = (*r595).clone();
                        let v4 = (*r596).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m599 = f60(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m599.uncons() {
        None => {
            let m601 = f141(f106(List::cons(Adt14::C4(std::rc::Rc::new({
                let m600 = f235(v4.clone());
                match m600 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m601 {
                None => {
                    f236(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f142(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f149(v0: Adt14) -> R<Bytes> {
    let a602 = f237(v0)?;
    let a603 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a602, a603))
}

pub fn f150(v0: Adt14, v1: Bytes) -> Bytes {
    let m604 = f155(f147(v0, v1));
    match m604 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f151(v0: Option<Adt14>) -> List<Adt14> {
    let m605 = v0;
    match m605 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f238(v1)
        }
    }
}

pub fn f152(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m606 = v0;
    match m606.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f224(v1)?;
            let a608 = {
                let a607 = v3.clone();
                length_bytes(a607)
            };
            let a609 = 0u64;
            if nat_eq(a608, a609) {
                f152(v2)
            } else {
                let m610 = f225(v3)?;
                match m610 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f197(v4.clone(), f8(Str::lit("agent"))) {
                            if f197(v4.clone(), f8(Str::lit("at"))) {
                                if f197(v4.clone(), f8(Str::lit("quote"))) {
                                    f239(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m611 = f152(v2)?;
                            match m611 {
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

pub fn f153(p614: List<Adt14>) -> bool {
    let m613 = p614;
    {
        let mut m613 = m613;
        loop {
            let step = {
                let v0 = m613;
                let m612 = v0;
                match m612.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f150(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f86(f8(Str::lit("studio")), f72(f240(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f155(v0: Option<Adt14>) -> Option<Bytes> {
    let m615 = v0;
    match m615 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m616 = v1;
            match m616 {
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

pub fn f156(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f119(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f241(v1, v4)?)?), List::cons(f120(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f157(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h617, _) = v2.clone();
        h617
    };
    let v4: u64 = {
        let (h618, _) = {
            let (_, h619) = v2.clone();
            h619
        };
        h618
    };
    let v5: List<Bytes> = {
        let (_, h620) = {
            let (_, h621) = v2;
            h621
        };
        h620
    };
    let m622 = f242(v0.clone(), v3);
    match m622 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a623 = v6;
            let a624 = v1;
            if equal(a623, a624) {
                Ok(Some(({
                    let a625 = v3;
                    let a626 = 1u64;
                    nat_add(a625, a626)?
                }, ({
                    let a627 = v3;
                    let a628 = 1u64;
                    nat_add(a627, a628)?
                }, List::cons(f32(v0, v4, {
                    let a629 = v3;
                    let a630 = v4;
                    nat_sub(a629, a630)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a631 = v3;
                    let a632 = 1u64;
                    nat_add(a631, a632)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f158(v0: Fn1, p636: List<Bytes>, p637: List<Bytes>) -> List<Bytes> {
    let m635 = (p636, p637);
    {
        let mut m635 = m635;
        loop {
            let step = {
                let (v1, v2) = m635;
                let m633 = v2;
                match m633.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c634: Fn1 = v0.clone();
                            c634.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f160(p642: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m641 = p642;
    {
        let mut m641 = m641;
        loop {
            let step = {
                let v0 = m641;
                let m638 = v0;
                match m638.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h639, _) = v2;
                            h639
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h640) = v2;
                                h640
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: Adt1, v1: Bytes) -> bool {
    let m644 = f160(match v0 {
        Adt1::C0(h643, _, _) => {
            h643
        }
    }, v1);
    match m644 {
        None => {
            false
        }
        Some(v2) => {
            let m645 = v2;
            match m645 {
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

pub fn f162(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m646 = v1;
    match m646.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f243(v3.clone())
            } else {
                f75(f243(v3.clone()))
            } {
                List::cons(f130(v0.clone(), v3), f162(v0, v4, v2))
            } else {
                f162(v0, v4, v2)
            }
        }
    }
}

pub fn f163(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a647 = v0.clone();
        length_bytes(a647)
    };
    let v3: u64 = {
        let a648 = v1.clone();
        length_bytes(a648)
    };
    let v4: u64 = {
        let (h649, _) = f244(Fn3::F245(v1.clone(), v3, v0.clone(), v2), {
            let a650 = v2;
            let a651 = 1u64;
            nat_add(a650, a651)?
        }, 0u64)?;
        h649
    };
    if f213(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f164() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f165(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f246(match v0.clone() {
        Adt12::C0(h652, _, _, _, _) => {
            h652
        }
    }, 0u64)?, f246(match v0.clone() {
        Adt12::C0(_, h653, _, _, _) => {
            h653
        }
    }, 0u64)?, f246(match v0.clone() {
        Adt12::C0(_, _, h654, _, _) => {
            h654
        }
    }, 0u64)?, f246(match v0 {
        Adt12::C0(_, _, _, h655, _) => {
            h655
        }
    }, 0u64)?))
}

pub fn f166(v0: Adt12) -> List<Bytes> {
    f64(match v0.clone() {
        Adt12::C0(h656, _, _, _, _) => {
            h656
        }
    }, f64(match v0.clone() {
        Adt12::C0(_, _, h657, _, _) => {
            h657
        }
    }, f64(match v0.clone() {
        Adt12::C0(_, _, _, h658, _) => {
            h658
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h659) => {
            h659
        }
    })))
}

pub fn f167(v0: Adt16, p665: List<u64>, p666: List<Adt19>) -> Option<u64> {
    let m664 = (p665, p666);
    {
        let mut m664 = m664;
        loop {
            let step = {
                let (v1, v2) = m664;
                let m660 = v2;
                match m660.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt19>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f247(v0.clone(), v1.clone(), match v3.clone() {
                            Adt19::C0(_, h661) => {
                                h661
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt19>), Option<u64>>((List::cons(match v3 {
                                Adt19::C0(h662, _) => {
                                    h662
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt19>), Option<u64>>(Some(match v3 {
                                Adt19::C0(h663, _) => {
                                    h663
                                }
                            }))
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

pub fn f168(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt19>> {
    let m667 = v1;
    match m667.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt19::C0(v2, f248(v0.clone(), v3)?), f168(v0, v4, {
                let a668 = v2;
                let a669 = 1u64;
                nat_add(a668, a669)?
            })?))
        }
    }
}

pub fn f169(p673: List<Bytes>, p674: u64) -> Bytes {
    let m672 = (p673, p674);
    {
        let mut m672 = m672;
        loop {
            let step = {
                let (v0, v1) = m672;
                let m670 = v0;
                match m670.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m671 = v1;
                        if m671 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m671 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m672 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f249(v0.clone(), v1.clone()) {
        if f249(v1, v0) {
            f250(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f171(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m675 = v0;
    match m675.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f251(v2, v1.clone())?, f171(v3, v1)?))
        }
    }
}

pub fn f172(v0: List<Adt14>) -> List<u64> {
    let m676 = v0;
    match m676.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f252(v1), f172(v2))
        }
    }
}

pub fn f173(p679: List<u64>, v1: List<u64>) -> Option<u64> {
    let m678 = p679;
    {
        let mut m678 = m678;
        loop {
            let step = {
                let v0 = m678;
                let m677 = v0;
                match m677.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f253(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f174(p682: List<Bytes>, p683: List<Bytes>) -> Option<Bytes> {
    let m681 = (p682, p683);
    {
        let mut m681 = m681;
        loop {
            let step = {
                let (v0, v1) = m681;
                let m680 = v0;
                match m680.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f86(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m681 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m684 = v0;
    match m684.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m685 = f254(v2, v1.clone())?;
            match m685 {
                None => {
                    f175(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f175(v3, v1)?))
                }
            }
        }
    }
}

pub fn f176(v0: List<Bytes>) -> List<Bytes> {
    f158(Fn1::F255, List::<Bytes>::nil(), v0)
}

pub fn f177(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f256(v0, v1)
}

pub fn f178(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f257(f258(f259(v0))?))
}

pub fn f179(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m686 = f146(v0, v2)?;
    match m686 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f260(v1, f150(v3.clone(), f8(Str::lit("id"))))?;
            let m687 = v4.clone();
            match m687.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m689 = f155(f147({
                        let m688 = f147(v3.clone(), f8(Str::lit("source")));
                        match m688 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m689 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f75(f86(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f204(f261(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f180(p692: List<Bytes>, p693: List<Bytes>) -> List<Bytes> {
    let m691 = (p692, p693);
    {
        let mut m691 = m691;
        loop {
            let step = {
                let (v0, v1) = m691;
                let m690 = v0;
                match m690.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f181(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f262(v0, v1)
}

pub fn f182() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f183(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m694 = v1;
    match m694.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt14 = f263(v0.clone(), v3)?;
            if f86(f264(f150(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f265(v5)?, f183(v0, v4, v2)?))
            } else {
                f183(v0, v4, v2)
            }
        }
    }
}

pub fn f184(v0: Adt1, p698: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m697 = p698;
    {
        let mut m697 = m697;
        loop {
            let step = {
                let v1 = m697;
                let m695 = v1;
                match m695.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt14 = f263(v0.clone(), v3.clone())?;
                        if if f86(f264(f150(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f36(f149({
                                let m696 = f147(v5.clone(), f8(Str::lit("schema")));
                                match m696 {
                                    None => {
                                        Adt14::C5
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f70(v0.clone(), f265(v5)?))
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
                Ok(next) => { m697 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f75(f76(match v0.clone() {
        Adt1::C0(_, h699, _) => {
            h699
        }
    }, f138())) {
        Ok(Adt11::C1(List::cons(f138(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f84(f78(f266(f80(match v0.clone() {
            Adt1::C0(_, h700, _) => {
                h700
            }
        }, f138())))?, v1);
        let m701 = f60(v0.clone(), List::cons(f89(), List::cons(f117(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m701.uncons() {
            None => {
                let v5: (bool, Bytes) = f267(v0.clone(), v2)?;
                let (h702, _) = v5;
                if h702 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h703) = v5;
                        h703
                    }))
                } else {
                    let m704 = f88(v0.clone());
                    match m704.uncons() {
                        None => {
                            let m705 = f90(v0.clone())?;
                            match m705.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h706) = v5;
                                        h706
                                    }, List::cons(f7(f78(f268(f102(v0.clone())?, f110(v0.clone(), f111(v0.clone())?)?))?), List::cons(f269(v0)?, List::<Bytes>::nil()))))))
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

pub fn f186(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f270(f73(v0, f89())?)?)))
}

pub fn f187() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f188(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt22>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m707 = v1;
    match m707.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f64(f271(v5.clone(), v2.clone()), f272(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f273(v0.clone(), v5.clone()), List::cons(if f86(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a709 = {
                    let a708 = v7.clone();
                    length_list(a708)
                };
                let a710 = 0u64;
                if nat_eq(a709, a710) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f121(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f188(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f189(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt22>> {
    let v3: List<(Bytes, Bytes)> = f274(v1);
    let v4: List<(Bytes, Bytes)> = f275(v0, v2);
    f276(v3.clone(), f91(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f190(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m711 = v1;
    match m711.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f197(v2.clone(), f8(Str::lit("domain"))) {
                f277(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f150(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f150(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m712 = f147(v2.clone(), f8(Str::lit("role")));
                    match m712 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f150(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f150(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f150(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f278(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m713 = f147(v2.clone(), f8(Str::lit("origin")));
                    match m713 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f150(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m714 = f147(v2, f8(Str::lit("evidence")));
                    match m714 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f150(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f190(v0, v3))
            } else {
                f190(v0, v3)
            }
        }
    }
}

pub fn f191(v0: List<Adt14>) -> List<Bytes> {
    let m715 = v0;
    match m715.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f150(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f150(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f150(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f191(v2))
        }
    }
}

pub fn f192(v0: List<Adt14>) -> List<Bytes> {
    let m716 = v0;
    match m716.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f150(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f279(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f192(v2))
        }
    }
}

pub fn f193(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m717 = v0;
    match m717.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f150(v1.clone(), f8(Str::lit("subject"))), f150(v1, f8(Str::lit("object")))), f193(v2))
        }
    }
}

pub fn f194(v0: List<Adt22>) -> List<(Bytes, Bytes)> {
    let m718 = v0;
    match m718.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt22::C0(h719, _, _) => {
                    h719
                }
            }, match v1 {
                Adt22::C0(_, _, h720) => {
                    h720
                }
            }), f194(v2))
        }
    }
}

pub fn f195(p727: List<Bytes>, v1: List<(Bytes, Bytes)>, p728: u64) -> R<List<Bytes>> {
    let m726 = (p727, p728);
    {
        let mut m726 = m726;
        loop {
            let step = {
                let (v0, v2) = m726;
                let m721 = v2;
                if m721 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m721 - 1;
                    let v4: List<Bytes> = f78(f64(v0.clone(), f280(v0.clone(), v1.clone())))?;
                    let a724 = {
                        let a722 = v4.clone();
                        length_list(a722)
                    };
                    let a725 = {
                        let a723 = v0.clone();
                        length_list(a723)
                    };
                    if nat_eq(a724, a725) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: List<Adt14>) -> List<Bytes> {
    let m729 = v0;
    match m729.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m730 = f147(v1, f8(Str::lit("std")));
            match m730 {
                None => {
                    f196(v2)
                }
                Some(v3) => {
                    List::cons(f150(v3.clone(), f8(Str::lit("authority"))), if f197(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f150(v3, f8(Str::lit("row"))), f196(v2))
                    } else {
                        f196(v2)
                    })
                }
            }
        }
    }
}

pub fn f197(v0: Adt14, v1: Bytes) -> bool {
    f75({
        let a732 = {
            let a731 = f150(v0, v1);
            length_bytes(a731)
        };
        let a733 = 0u64;
        nat_eq(a732, a733)
    })
}

pub fn f198(v0: Adt15, v1: Adt14) -> bool {
    if f281(v0.clone(), v1.clone(), f150(v1.clone(), f8(Str::lit("subject")))) {
        f281(v0, v1.clone(), f150(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f199(v0: Bytes, v1: Bytes) -> bool {
    let m738 = {
        let a735 = v0;
        let a736 = 0u64;
        let a737 = {
            let a734 = v1.clone();
            length_bytes(a734)
        };
        slice_bytes(a735, a736, a737)
    };
    match m738 {
        None => {
            false
        }
        Some(v2) => {
            let a739 = v2;
            let a740 = v1;
            equal(a739, a740)
        }
    }
}

pub fn f200(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m741 = v1;
    match m741.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f134(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f130(v0.clone(), v2), f200(v0, v3))
            } else {
                f200(v0, v3)
            }
        }
    }
}

pub fn f201(v0: Bytes, p799: u64, p800: u64, p801: Adt17) -> R<Adt17> {
    let m798 = (p799, (p800, p801));
    {
        let mut m798 = m798;
        loop {
            let step = {
                let (v1, (v2, v3)) = m798;
                let m742 = v2;
                if m742 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m742 - 1;
                    let v5: u64 = f282(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h743, _, _, _) => {
                            h743
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a744 = v1;
                            let a745 = 1u64;
                            nat_add(a744, a745)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h746, _, _) => {
                                h746
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h747, _) => {
                                h747
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h748) => {
                                h748
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h749, _, _, _, _) => {
                                h749
                            }
                        } {
                            let a750 = v5;
                            let a751 = 34u64;
                            if nat_eq(a750, a751) {
                                let a754 = f282(v0.clone(), {
                                    let a752 = v1;
                                    let a753 = 1u64;
                                    nat_add(a752, a753)?
                                })?;
                                let a755 = 34u64;
                                if nat_eq(a754, a755) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a756 = v1;
                                        let a757 = 1u64;
                                        nat_add(a756, a757)?
                                    }, (v4, Adt17::C0(true, true, f283(match v3.clone() {
                                        Adt17::C0(_, _, h758, _, _) => {
                                            h758
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h759, _) => {
                                            h759
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h760) => {
                                            h760
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a761 = v1;
                                        let a762 = 1u64;
                                        nat_add(a761, a762)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h763, _, _) => {
                                            h763
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h764, _) => {
                                            h764
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h765) => {
                                            h765
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a766 = v1;
                                    let a767 = 1u64;
                                    nat_add(a766, a767)?
                                }, (v4, Adt17::C0(true, false, f283(match v3.clone() {
                                    Adt17::C0(_, _, h768, _, _) => {
                                        h768
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h769, _) => {
                                        h769
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h770) => {
                                        h770
                                    }
                                }))))
                            }
                        } else {
                            let a771 = v5;
                            let a772 = 34u64;
                            if nat_eq(a771, a772) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a773 = v1;
                                    let a774 = 1u64;
                                    nat_add(a773, a774)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h775, _, _) => {
                                        h775
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h776, _) => {
                                        h776
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h777) => {
                                        h777
                                    }
                                }))))
                            } else {
                                let a778 = v5;
                                let a779 = 44u64;
                                if nat_eq(a778, a779) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a780 = v1;
                                        let a781 = 1u64;
                                        nat_add(a780, a781)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h782, _, _) => {
                                            h782
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h783, _) => {
                                            h783
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h784) => {
                                            h784
                                        }
                                    }))))
                                } else {
                                    let a785 = v5;
                                    let a786 = 13u64;
                                    if nat_eq(a785, a786) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a787 = v1;
                                            let a788 = 1u64;
                                            nat_add(a787, a788)?
                                        }, (v4, v3)))
                                    } else {
                                        let a789 = v5;
                                        let a790 = 10u64;
                                        if nat_eq(a789, a790) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a791 = v1;
                                                let a792 = 1u64;
                                                nat_add(a791, a792)?
                                            }, (v4, f202(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a793 = v1;
                                                let a794 = 1u64;
                                                nat_add(a793, a794)?
                                            }, (v4, Adt17::C0(false, false, f283(match v3.clone() {
                                                Adt17::C0(_, _, h795, _, _) => {
                                                    h795
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h796, _) => {
                                                    h796
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h797) => {
                                                    h797
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
                Ok(next) => { m798 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f284(List::cons(match v0.clone() {
        Adt17::C0(_, _, h802, _, _) => {
            h802
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h803, _) => {
            h803
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h804) => {
            h804
        }
    }))
}

pub fn f203(p807: List<List<Bytes>>, p808: List<List<Bytes>>) -> List<List<Bytes>> {
    let m806 = (p807, p808);
    {
        let mut m806 = m806;
        loop {
            let step = {
                let (v0, v1) = m806;
                let m805 = v0;
                match m805.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f285(v0.clone(), 0u64, {
        let a809 = v0;
        length_bytes(a809)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f205(v0: Adt23) -> Bytes {
    let a830 = f286(match v0.clone() {
        Adt23::C0(h810, _, _, _, _, _, _, _) => {
            h810
        }
    });
    let a831 = {
        let a828 = f286(match v0.clone() {
            Adt23::C0(_, h811, _, _, _, _, _, _) => {
                h811
            }
        });
        let a829 = {
            let a826 = f286(match v0.clone() {
                Adt23::C0(_, _, h812, _, _, _, _, _) => {
                    h812
                }
            });
            let a827 = {
                let a824 = f286(match v0.clone() {
                    Adt23::C0(_, _, _, h813, _, _, _, _) => {
                        h813
                    }
                });
                let a825 = {
                    let a822 = f286(match v0.clone() {
                        Adt23::C0(_, _, _, _, h814, _, _, _) => {
                            h814
                        }
                    });
                    let a823 = {
                        let a820 = f286(match v0.clone() {
                            Adt23::C0(_, _, _, _, _, h815, _, _) => {
                                h815
                            }
                        });
                        let a821 = {
                            let a818 = f286(match v0.clone() {
                                Adt23::C0(_, _, _, _, _, _, h816, _) => {
                                    h816
                                }
                            });
                            let a819 = f286(match v0 {
                                Adt23::C0(_, _, _, _, _, _, _, h817) => {
                                    h817
                                }
                            });
                            append_bytes(a818, a819)
                        };
                        append_bytes(a820, a821)
                    };
                    append_bytes(a822, a823)
                };
                append_bytes(a824, a825)
            };
            append_bytes(a826, a827)
        };
        append_bytes(a828, a829)
    };
    append_bytes(a830, a831)
}

pub fn f206(v0: Bytes) -> R<Adt23> {
    let v1: u64 = {
        let a832 = v0.clone();
        length_bytes(a832)
    };
    let v2: u64 = {
        let a838 = {
            let a835 = {
                let a833 = v1;
                let a834 = 72u64;
                nat_add(a833, a834)?
            };
            let a836 = 64u64;
            let a837 = 0u64;
            nat_quot(a835, a836, a837)
        };
        let a839 = 64u64;
        nat_mul(a838, a839)?
    };
    let (_, h840) = f287(Fn4::F288(v0, v1, v2), {
        let a841 = v2;
        let a842 = 64u64;
        let a843 = 0u64;
        nat_quot(a841, a842, a843)
    }, (0u64, f289()))?;
    Ok(h840)
}

pub fn f207(p846: List<List<Bytes>>, v1: Bytes, p847: List<Bytes>) -> R<List<Bytes>> {
    let m845 = (p846, p847);
    {
        let mut m845 = m845;
        loop {
            let step = {
                let (v0, v2) = m845;
                let m844 = v0;
                match m844.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f290(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f209(f291(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f292(v0)?))
}

pub fn f209(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m848 = v0;
    match m848.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m849 = v3.clone();
            match m849.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a852 = v2;
                    let a853 = {
                        let a850 = v1.clone();
                        let a851 = f209(v3, v1);
                        append_bytes(a850, a851)
                    };
                    append_bytes(a852, a853)
                }
            }
        }
    }
}

pub fn f210(v0: List<Bytes>) -> Adt14 {
    let m854 = v0;
    match m854.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f210(v2)))
        }
    }
}

pub fn f211(v0: Fn2, p858: Option<Bytes>, p859: List<Bytes>) -> Option<Bytes> {
    let m857 = (p858, p859);
    {
        let mut m857 = m857;
        loop {
            let step = {
                let (v1, v2) = m857;
                let m855 = v2;
                match m855.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c856: Fn2 = v0.clone();
                            c856.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m857 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m860 = v1;
    match m860 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a863 = {
                    let a861 = v3;
                    let a862 = v0;
                    append_bytes(a861, a862)
                };
                let a864 = v2;
                append_bytes(a863, a864)
            })
        }
    }
}

pub fn f213(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m869 = {
        let a866 = v0;
        let a867 = v2;
        let a868 = {
            let a865 = v1.clone();
            length_bytes(a865)
        };
        slice_bytes(a866, a867, a868)
    };
    match m869 {
        None => {
            false
        }
        Some(v3) => {
            let a870 = v3;
            let a871 = v1;
            equal(a870, a871)
        }
    }
}

pub fn f214(v0: Bytes) -> R<Bytes> {
    let (_, h872) = {
        let (h873, _) = f293(Fn5::F294(v0.clone()), {
            let a875 = {
                let a874 = v0;
                length_bytes(a874)
            };
            let a876 = 1u64;
            nat_add(a875, a876)?
        }, (0u64, Bytes::lit(&[])))?;
        h873
    };
    Ok(h872)
}

pub fn f215(v0: Bytes) -> R<u64> {
    let (h877, _) = f244(Fn3::F295(v0.clone()), {
        let a879 = {
            let a878 = v0;
            length_bytes(a878)
        };
        let a880 = 1u64;
        nat_add(a879, a880)?
    }, 0u64)?;
    Ok(h877)
}

pub fn f216(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a881 = v0;
        utf8_encode(a881)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f126(v1)?, List::<Bytes>::nil())))))
}

pub fn f217(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f219(v0.clone(), 0u64, Str::lit("<?xml")) {
        f296(f220(v0.clone(), 5u64)?)
    } else {
        false
    } {
        let m883 = f297(v0.clone(), {
            let a882 = Str::lit("?>");
            utf8_encode(a882)
        }, 5u64)?;
        match m883 {
            None => {
                Ok(Err::<u64, Bytes>(f216(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m885 = f297(f32(v0.clone(), 0u64, v1), {
                    let a884 = Str::lit("encoding");
                    utf8_encode(a884)
                }, 5u64)?;
                match m885 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a886 = v1;
                            let a887 = 2u64;
                            nat_add(a886, a887)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f298(v0.clone(), {
                            let a888 = v2;
                            let a889 = 8u64;
                            nat_add(a888, a889)?
                        })?;
                        let v4: u64 = f298(v0.clone(), {
                            let a890 = v3;
                            let a891 = 1u64;
                            nat_add(a890, a891)?
                        })?;
                        let _: u64 = f220(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m894 = f297(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a892 = v4;
                                let a893 = 1u64;
                                nat_add(a892, a893)?
                            })?;
                            match m894 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f299(f32(v0, {
                            let a895 = v4;
                            let a896 = 1u64;
                            nat_add(a895, a896)?
                        }, {
                            let a899 = v7;
                            let a900 = {
                                let a897 = v4;
                                let a898 = 1u64;
                                nat_add(a897, a898)?
                            };
                            nat_sub(a899, a900)
                        }))? {
                            Ok(Ok::<u64, Bytes>({
                                let a901 = v1;
                                let a902 = 2u64;
                                nat_add(a901, a902)?
                            }))
                        } else {
                            Ok(Err::<u64, Bytes>(f216(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f218(v0: Bytes, p907: u64, p908: u64) -> R<Result<u64, Bytes>> {
    let m906 = (p907, p908);
    {
        let mut m906 = m906;
        loop {
            let step = {
                let (v1, v2) = m906;
                let m903 = v2;
                if m903 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(f216(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m903 - 1;
                    let v4: u64 = f298(v0.clone(), v1)?;
                    if f219(v0.clone(), v4, Str::lit("<!--")) {
                        let m904 = f300(v0.clone(), v4)?;
                        match m904 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v5)))
                            }
                        }
                    } else {
                        if f219(v0.clone(), v4, Str::lit("<?")) {
                            let m905 = f301(v0.clone(), v4)?;
                            match m905 {
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
                Ok(next) => { m906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: Bytes, v1: u64, v2: Str) -> bool {
    f213(v0, {
        let a909 = v2;
        utf8_encode(a909)
    }, v1)
}

pub fn f220(v0: Bytes, v1: u64) -> R<u64> {
    f302(v0, v1)
}

pub fn f221(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt18, u64), Bytes>> {
    let m910 = v2;
    if m910 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m910 - 1;
        let a911 = v3;
        let a912 = 0u64;
        if nat_eq(a911, a912) {
            let v5: u64 = f303(v0.clone(), {
                let a913 = v1;
                let a914 = 1u64;
                nat_add(a913, a914)?
            })?;
            let a917 = v5;
            let a918 = {
                let a915 = v1;
                let a916 = 1u64;
                nat_add(a915, a916)?
            };
            if nat_eq(a917, a918) {
                Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a919 = v1;
                    let a920 = 1u64;
                    nat_add(a919, a920)?
                }, {
                    let a923 = v5;
                    let a924 = {
                        let a921 = v1;
                        let a922 = 1u64;
                        nat_add(a921, a922)?
                    };
                    nat_sub(a923, a924)
                });
                let m925 = f304(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m925 {
                    Ok(v8) => {
                        if f219(v0.clone(), {
                            let (_, h926) = v8.clone();
                            h926
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                let (h927, _) = v8;
                                h927
                            }), std::rc::Rc::new(Adt18::C4)), {
                                let a929 = {
                                    let (_, h928) = v8;
                                    h928
                                };
                                let a930 = 2u64;
                                nat_add(a929, a930)?
                            })))
                        } else {
                            let m934 = f221(v0.clone(), {
                                let a932 = {
                                    let (_, h931) = v8;
                                    h931
                                };
                                let a933 = 1u64;
                                nat_add(a932, a933)?
                            }, v4, 1u64)?;
                            match m934 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a936 = {
                                            let (_, h935) = v10.clone();
                                            h935
                                        };
                                        let a937 = 2u64;
                                        nat_add(a936, a937)?
                                    };
                                    let v12: u64 = f303(v0.clone(), v11)?;
                                    if !f36(f32(v0.clone(), v11, {
                                        let a938 = v12;
                                        let a939 = v11;
                                        nat_sub(a938, a939)
                                    }), v6.clone()) {
                                        Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h940) = v10;
                                            h940
                                        })?))
                                    } else {
                                        let v13: u64 = f298(v0.clone(), v12)?;
                                        let a941 = f220(v0, v13)?;
                                        let a942 = 62u64;
                                        if nat_eq(a941, a942) {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                                let (h943, _) = v8;
                                                h943
                                            }), std::rc::Rc::new({
                                                let (h944, _) = v10;
                                                h944
                                            })), {
                                                let a945 = v13;
                                                let a946 = 1u64;
                                                nat_add(a945, a946)?
                                            })))
                                        } else {
                                            Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
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
            let a947 = f220(v0.clone(), v1)?;
            let a948 = 256u64;
            if nat_eq(a947, a948) {
                Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f219(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v1)))
                } else {
                    if f219(v0.clone(), v1, Str::lit("<!--")) {
                        let m949 = f300(v0.clone(), v1)?;
                        match m949 {
                            Ok(v15) => {
                                f221(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(Err::<(Adt18, u64), Bytes>(v14))
                            }
                        }
                    } else {
                        if f219(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m953 = f297(v0.clone(), {
                                let a950 = Str::lit("]]>");
                                utf8_encode(a950)
                            }, {
                                let a951 = v1;
                                let a952 = 9u64;
                                nat_add(a951, a952)?
                            })?;
                            match m953 {
                                None => {
                                    Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m956 = f221(v0.clone(), {
                                        let a954 = v16;
                                        let a955 = 3u64;
                                        nat_add(a954, a955)?
                                    }, v4, 1u64)?;
                                    match m956 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2(f32(v0, {
                                                let a957 = v1;
                                                let a958 = 9u64;
                                                nat_add(a957, a958)?
                                            }, {
                                                let a961 = v16;
                                                let a962 = {
                                                    let a959 = v1;
                                                    let a960 = 9u64;
                                                    nat_add(a959, a960)?
                                                };
                                                nat_sub(a961, a962)
                                            }))), std::rc::Rc::new({
                                                let (h963, _) = v18;
                                                h963
                                            })), {
                                                let (_, h964) = v18;
                                                h964
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(Err::<(Adt18, u64), Bytes>(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f219(v0.clone(), v1, Str::lit("<?")) {
                                let m965 = f301(v0.clone(), v1)?;
                                match m965 {
                                    Ok(v20) => {
                                        f221(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v19))
                                    }
                                }
                            } else {
                                if f219(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a966 = f220(v0.clone(), v1)?;
                                    let a967 = 60u64;
                                    if nat_eq(a966, a967) {
                                        let m968 = f221(v0.clone(), v1, v4, 0u64)?;
                                        match m968 {
                                            Ok(v22) => {
                                                let m970 = f221(v0, {
                                                    let (_, h969) = v22;
                                                    h969
                                                }, v4, 1u64)?;
                                                match m970 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new({
                                                            let (h971, _) = v22;
                                                            h971
                                                        }), std::rc::Rc::new({
                                                            let (h972, _) = v24;
                                                            h972
                                                        })), {
                                                            let (_, h973) = v24;
                                                            h973
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
                                        let m974 = f305(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m974 {
                                            Ok(v26) => {
                                                let m976 = f221(v0, {
                                                    let (_, h975) = v26;
                                                    h975
                                                }, v4, 1u64)?;
                                                match m976 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2({
                                                            let (h977, _) = v26;
                                                            h977
                                                        })), std::rc::Rc::new({
                                                            let (h978, _) = v28;
                                                            h978
                                                        })), {
                                                            let (_, h979) = v28;
                                                            h979
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

pub fn f222(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a980 = v2;
    let a981 = 0u64;
    if nat_eq(a980, a981) {
        Ok(f306(v0, 0u64))
    } else {
        let (_, h982) = {
            let (h983, _) = f293(Fn5::F307(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h983
        };
        Ok(h982)
    }
}

pub fn f223() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f224(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f292(v0.clone())?;
    let v2: u64 = f308(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a984 = v1;
        let a985 = v2;
        nat_sub(a984, a985)
    }))
}

pub fn f225(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f309(v0)?;
    let m989 = f310(v1.clone(), 0u64, {
        let a987 = {
            let a986 = v1.clone();
            length_bytes(a986)
        };
        let a988 = 2u64;
        nat_add(a987, a988)?
    }, 0u64)?;
    match m989 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a992 = {
                let (_, h990) = v2;
                h990
            };
            let a993 = {
                let a991 = v1;
                length_bytes(a991)
            };
            if nat_eq(a992, a993) {
                Ok(Some({
                    let (h994, _) = v2;
                    h994
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f226(v0: Bytes) -> Bytes {
    f7(List::cons(f77(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f227(v0: Adt14) -> bool {
    if f197(v0.clone(), f8(Str::lit("domain"))) {
        if f197(v0.clone(), f8(Str::lit("row"))) {
            if f86(f150(v0.clone(), f8(Str::lit("relation"))), f311()) {
                if f312(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f312(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f312(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f312(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f228(v0: Adt14) -> bool {
    if f197(v0.clone(), f8(Str::lit("subject"))) {
        if f197(v0.clone(), f8(Str::lit("object"))) {
            if f197(v0.clone(), f8(Str::lit("evidence"))) {
                f86(f150(v0, f8(Str::lit("relation"))), f313())
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

pub fn f229(v0: Option<List<Adt14>>) -> bool {
    let m995 = v0;
    match m995 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f230(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m996 = v0;
    match m996 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f231(v0: Adt14) -> Bytes {
    if f197(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f197(v0.clone(), f8(Str::lit("id"))) {
            if f197(v0.clone(), f8(Str::lit("statement"))) {
                if f197(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f239(v0, f8(Str::lit("evidence")))
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

pub fn f232(v0: List<Adt14>) -> Adt14 {
    let m997 = v0;
    match m997.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f232(v2)))
        }
    }
}

pub fn f233(v0: Adt14) -> bool {
    if f197(v0.clone(), f8(Str::lit("name"))) {
        if f239(v0.clone(), f8(Str::lit("schema"))) {
            let m998 = f235(f74(f151(f147(v0, f8(Str::lit("citation"))))));
            match m998 {
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

pub fn f234(v0: Bytes) -> R<Bytes> {
    f314(v0.clone(), 0u64, {
        let a999 = v0;
        length_bytes(a999)
    }, true, Bytes::lit(&[]))
}

pub fn f235(v0: List<Bytes>) -> Option<Adt14> {
    let m1000 = v0;
    match m1000.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m1001 = v2;
                match m1001.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m1002 = v2;
                    match m1002.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1003 = v6;
                                match m1003.uncons() {
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
                        let m1004 = v2;
                        match m1004.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1005 = v10;
                                match m1005.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f210(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f236(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1006 = f60(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1006.uncons() {
        None => {
            let m1007 = f235(v4);
            match m1007 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1008 = f8(Str::lit("Catalog."));
                        let a1009 = f234(v3.clone())?;
                        append_bytes(a1008, a1009)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f234(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1010 = f315(v2, v9.clone(), f234(v3.clone())?)?;
                    match m1010 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f234(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f149(Adt14::C3(std::rc::Rc::new(f232(f316(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f149(Adt14::C3(std::rc::Rc::new(f232(f316(f73(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1014 = f317(f70(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a1012 = {
                                        let a1011 = v10;
                                        length_bytes(a1011)
                                    };
                                    let a1013 = 4u64;
                                    nat_sub(a1012, a1013)
                                }))?;
                                let a1015 = Bytes::lit(&[0x0a]);
                                append_bytes(a1014, a1015)
                            }), List::cons(f120(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f237(v0: Adt14) -> R<Bytes> {
    let m1016 = v0;
    match m1016 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f318(v2, v3)
        }
        Adt14::C2(v4) => {
            f319(v4)
        }
        Adt14::C3(r1017) => {
            let v5 = (*r1017).clone();
            Ok(f320(f321(v5)?))
        }
        Adt14::C4(r1018) => {
            let v6 = (*r1018).clone();
            f322(f323(v6)?)
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

pub fn f238(v0: Adt14) -> List<Adt14> {
    let m1019 = v0;
    match m1019 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1020) => {
            let v3 = (*r1020).clone();
            f238(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1021, r1022) => {
            let v1 = (*r1021).clone();
            let v2 = (*r1022).clone();
            List::cons(v1, f238(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f239(v0: Adt14, v1: Bytes) -> bool {
    let m1023 = f147(v0, v1);
    match m1023 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f240(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1024 = f147(v0, v1);
    match m1024 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f151(f147(v3, v2))
        }
    }
}

pub fn f241(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f147(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f116(Str::lit("id"), f150(v0.clone(), f8(Str::lit("id"))))?, List::cons(f116(Str::lit("name"), f150(v0.clone(), f8(Str::lit("name"))))?, List::cons(f116(Str::lit("issuer"), f150(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f116(Str::lit("edition"), f150(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f116(Str::lit("canonical_id"), f150(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f116(Str::lit("source_role"), f150(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f116(Str::lit("retrieval_date"), f150(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f116(Str::lit("url"), f324(v2.clone(), f8(Str::lit("url"))))?, List::cons(f116(Str::lit("media_type"), f324(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f116(Str::lit("license"), f324(v2.clone(), f8(Str::lit("license"))))?, List::cons(f116(Str::lit("redistribution"), f324(v2, f8(Str::lit("redistribution"))))?, List::cons(f116(Str::lit("statement"), f150(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1025 = v1;
        match m1025 {
            None => {
                f7(List::cons(f116(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f116(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f116(Str::lit("artifact"), {
                    let (h1026, _) = v3;
                    h1026
                })?, List::cons(f116(Str::lit("checksum"), {
                    let (_, h1027) = v3;
                    h1027
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f325(f74(f151(f147(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f242(v0: Bytes, v1: u64) -> Option<u8> {
    let a1028 = v0;
    let a1029 = v1;
    index_bytes(a1028, a1029)
}

pub fn f243(v0: Bytes) -> bool {
    f134(v0, f8(Str::lit(".lex.tex")))
}

pub fn f244(v0: Fn3, p1034: u64, p1035: u64) -> R<(u64, bool)> {
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

pub fn f245(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f213(v2, v0, v4) {
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

pub fn f246(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1042 = v0;
    match m1042.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f246(v3, {
                let a1043 = v1;
                let a1044 = 1u64;
                nat_add(a1043, a1044)?
            })?))
        }
    }
}

pub fn f247(v0: Adt16, v1: List<u64>, v2: Adt20) -> bool {
    let m1045 = v2;
    match m1045 {
        Adt20::C0(v3, v4) => {
            if f253(v3, match v0.clone() {
                Adt16::C0(h1046, _, _, _) => {
                    h1046
                }
            }) {
                f326(v0, v4)
            } else {
                false
            }
        }
        Adt20::C1(v5) => {
            f253(v5, match v0 {
                Adt16::C0(_, _, h1047, _) => {
                    h1047
                }
            })
        }
        Adt20::C2(_, v7, v8) => {
            if f249(v7, v1) {
                f253(v8, match v0 {
                    Adt16::C0(_, _, _, h1048) => {
                        h1048
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f248(v0: Adt12, v1: Adt14) -> R<Adt20> {
    let m1049 = f147(v1.clone(), f8(Str::lit("std")));
    match m1049 {
        None => {
            let m1054 = f155(f147(v1.clone(), f8(Str::lit("req"))));
            match m1054 {
                None => {
                    let m1056 = f147(v1, f8(Str::lit("der")));
                    match m1056 {
                        None => {
                            Ok(f327(v0))
                        }
                        Some(v5) => {
                            let m1057 = f328(f150(v5.clone(), f8(Str::lit("rule"))));
                            match m1057 {
                                None => {
                                    Ok(f327(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt20::C2(v6, f171(f74(f151(f147(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h1058) => {
                                            h1058
                                        }
                                    })?, f251(f150(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h1059, _) => {
                                            h1059
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt20::C1(f251(v4, match v0 {
                        Adt12::C0(_, _, h1055, _, _) => {
                            h1055
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1050 = f155(f147(v2.clone(), f8(Str::lit("row"))));
            match m1050 {
                None => {
                    Ok(Adt20::C0(f251(f150(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h1051, _, _, _, _) => {
                            h1051
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt20::C0(f251(f150(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h1052, _, _, _, _) => {
                            h1052
                        }
                    })?, Some(f251(v3, match v0 {
                        Adt12::C0(_, h1053, _, _, _) => {
                            h1053
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f249(p1062: List<u64>, v1: List<u64>) -> bool {
    let m1061 = p1062;
    {
        let mut m1061 = m1061;
        loop {
            let step = {
                let v0 = m1061;
                let m1060 = v0;
                match m1060.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f253(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1061 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(p1067: List<u64>) -> bool {
    let m1066 = p1067;
    {
        let mut m1066 = m1066;
        loop {
            let step = {
                let v0 = m1066;
                let m1063 = v0;
                match m1063.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1064 = v1;
                        let a1065 = 7u64;
                        if nat_le(a1064, a1065) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f251(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1068 = f329(v0, v1.clone(), 0u64)?;
    match m1068 {
        None => {
            let a1069 = v1;
            Ok(length_list(a1069))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f252(v0: Adt14) -> u64 {
    let m1070 = f330(f147(v0, f8(Str::lit("stratum"))));
    match m1070 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f253(v0: u64, p1075: List<u64>) -> bool {
    let m1074 = p1075;
    {
        let mut m1074 = m1074;
        loop {
            let step = {
                let v1 = m1074;
                let m1071 = v1;
                match m1071.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1072 = v2;
                        let a1073 = v0;
                        if nat_eq(a1072, a1073) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
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

pub fn f254(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f199(v0.clone(), v1) {
        let m1076 = f30(v0, 34u8)?;
        match m1076.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1077 = v3;
                match m1077.uncons() {
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

pub fn f255(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f256(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1078 = v0.clone();
    match m1078.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1079 = v2;
            let a1080 = v1.clone();
            if equal(a1079, a1080) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f257(v0: List<(Bytes, ())>) -> List<Bytes> {
    f176(f331(Fn6::F332, List::<Bytes>::nil(), v0))
}

pub fn f258(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f333(f334(Fn7::F335, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1085 = {
        let (h1081, _) = f336(Fn8::F337, {
            let a1083 = {
                let a1082 = v0;
                length_list(a1082)
            };
            let a1084 = 1u64;
            nat_add(a1083, a1084)?
        }, v1)?;
        h1081
    };
    match m1085.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f259(v0: List<Bytes>) -> List<(Bytes, ())> {
    f338(f339(Fn9::F340, List::<(Bytes, ())>::nil(), v0))
}

pub fn f260(p1088: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1087 = p1088;
    {
        let mut m1087 = m1087;
        loop {
            let step = {
                let v0 = m1087;
                let m1086 = v0;
                match m1086.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f290(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
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

pub fn f261(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f324(f147(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f262(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1089, _) = v1.clone();
        h1089
    };
    let v3: u64 = {
        let (h1090, _) = {
            let (_, h1091) = v1.clone();
            h1091
        };
        h1090
    };
    let v4: List<Bytes> = {
        let (_, h1092) = {
            let (_, h1093) = v1;
            h1093
        };
        h1092
    };
    let m1094 = f242(v0.clone(), v2);
    match m1094 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1095 = v5;
            let a1096 = 10u8;
            if equal(a1095, a1096) {
                Ok(Some(({
                    let a1097 = v2;
                    let a1098 = 1u64;
                    nat_add(a1097, a1098)?
                }, ({
                    let a1099 = v2;
                    let a1100 = 1u64;
                    nat_add(a1099, a1100)?
                }, List::cons(f341(f32(v0, v3, {
                    let a1101 = v2;
                    let a1102 = v3;
                    nat_sub(a1101, a1102)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1103 = v2;
                    let a1104 = 1u64;
                    nat_add(a1103, a1104)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f263(v0: Adt1, v1: Bytes) -> R<Adt14> {
    let m1105 = f146(v0, v1)?;
    match m1105 {
        None => {
            Ok(Adt14::C5)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f264(v0: Bytes) -> R<Bytes> {
    let a1106 = f8(Str::lit("Catalog."));
    let a1107 = f234(v0)?;
    Ok(append_bytes(a1106, a1107))
}

pub fn f265(v0: Adt14) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f234(f150(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f266(v0: List<Bytes>) -> List<Bytes> {
    let m1108 = v0;
    match m1108.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f134(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f130(f138(), v1), f266(v2))
            } else {
                f266(v2)
            }
        }
    }
}

pub fn f267(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1109 = v1;
    match m1109.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f342(v0.clone(), v2)?;
            let (h1110, _) = v4.clone();
            if h1110 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f267(v0, v3)?;
                let (h1111, _) = v5.clone();
                if h1111 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1114 = {
                            let (_, h1112) = v4;
                            h1112
                        };
                        let a1115 = {
                            let (_, h1113) = v5;
                            h1113
                        };
                        append_bytes(a1114, a1115)
                    }))
                }
            }
        }
    }
}

pub fn f268(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1116 = v1;
    match m1116.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f197(v2.clone(), f8(Str::lit("subject"))) {
                if f198(v0.clone(), v2.clone()) {
                    f268(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f150(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f150(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f150(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f268(v0, v3))
                }
            } else {
                if f277(v0.clone(), v2.clone()) {
                    f268(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f150(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f278(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f268(v0, v3))
                }
            }
        }
    }
}

pub fn f269(v0: Adt1) -> R<Bytes> {
    let a1118 = {
        let a1117 = f103(f104(f122(f123(f112(f70(v0.clone(), f117()))?)), f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f106(f73(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1117)
    };
    let a1119 = 0u64;
    if nat_eq(a1118, a1119) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f270(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1120 = v0;
    match m1120.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f121(f74(f151(f147(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f150(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f150(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f343(f83(f150(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f270(v2)?))
        }
    }
}

pub fn f271(v0: Bytes, v1: List<Adt22>) -> List<Bytes> {
    let m1121 = v1;
    match m1121.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(match v2.clone() {
                Adt22::C0(h1122, _, _) => {
                    h1122
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt22::C0(_, h1123, _) => {
                        h1123
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt22::C0(_, _, h1124) => {
                        h1124
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f271(v0, v3))
            } else {
                f271(v0, v3)
            }
        }
    }
}

pub fn f272(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1125 = v1;
    match m1125.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(f150(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f150(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f150(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f272(v0, v3))
            } else {
                if f36(f150(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f150(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f150(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f272(v0, v3))
                } else {
                    f272(v0, v3)
                }
            }
        }
    }
}

pub fn f273(p1130: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1129 = p1130;
    {
        let mut m1129 = m1129;
        loop {
            let step = {
                let v0 = m1129;
                let m1126 = v0;
                match m1126.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1127 = v2;
                        match m1127.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f36(v4, v1.clone()) {
                                    let m1128 = v5;
                                    match m1128.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f344(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f274(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1131 = v0;
    match m1131.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1132 = f147(v1, f8(Str::lit("std")));
            match m1132 {
                None => {
                    f274(v2)
                }
                Some(v3) => {
                    if f197(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f150(v3.clone(), f8(Str::lit("row"))), f150(v3, f8(Str::lit("authority")))), f274(v2))
                    } else {
                        f274(v2)
                    }
                }
            }
        }
    }
}

pub fn f275(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1133 = v1;
    match m1133.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f150(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f150(v2, f8(Str::lit("object")));
            if if f86(v4.clone(), match v0.clone() {
                Adt15::C0(h1134, _, _) => {
                    h1134
                }
            }) {
                f86(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1135, _) => {
                        h1135
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f275(v0, v3))
            } else {
                if if f86(v5.clone(), match v0.clone() {
                    Adt15::C0(h1136, _, _) => {
                        h1136
                    }
                }) {
                    f86(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1137, _) => {
                            h1137
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f275(v0, v3))
                } else {
                    f275(v0, v3)
                }
            }
        }
    }
}

pub fn f276(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt22>> {
    let m1138 = v0;
    match m1138.uncons() {
        None => {
            Ok(List::<Adt22>::nil())
        }
        Some((v4, v5)) => {
            if if f345(v4.clone(), v2.clone()) {
                true
            } else {
                f345(v4.clone(), v3.clone())
            } {
                f276(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt22::C0({
                    let (h1139, _) = v4.clone();
                    h1139
                }, {
                    let a1142 = {
                        let a1141 = f78(f346({
                            let (h1140, _) = v4.clone();
                            h1140
                        }, v1.clone()))?;
                        length_list(a1141)
                    };
                    let a1143 = 1u64;
                    if nat_eq(a1142, a1143) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1144) = v4.clone();
                    h1144
                }), f276(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f277(v0: Adt15, v1: Adt14) -> bool {
    if f86(f150(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1145, _, _) => {
            h1145
        }
    }) {
        f86(f278(v1), match v0 {
            Adt15::C0(_, h1146, _) => {
                h1146
            }
        })
    } else {
        false
    }
}

pub fn f278(v0: Adt14) -> Bytes {
    let m1147 = f147(v0, f8(Str::lit("evidence")));
    match m1147 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f150(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f279(v0: Adt14) -> Bytes {
    if f197(v0.clone(), f8(Str::lit("req"))) {
        let a1148 = f8(Str::lit("req "));
        let a1149 = f150(v0, f8(Str::lit("req")));
        append_bytes(a1148, a1149)
    } else {
        let m1150 = f147(v0.clone(), f8(Str::lit("std")));
        match m1150 {
            None => {
                let m1153 = f147(v0, f8(Str::lit("der")));
                match m1153 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f150(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f121(f74(f151(f147(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f150(v1.clone(), f8(Str::lit("authority"))), List::cons(if f197(v1.clone(), f8(Str::lit("row"))) {
                    let a1151 = f8(Str::lit(" row "));
                    let a1152 = f150(v1, f8(Str::lit("row")));
                    append_bytes(a1151, a1152)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f280(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1154 = v1;
    match m1154.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86({
                let (h1155, _) = v2.clone();
                h1155
            }, v0.clone()) {
                List::cons({
                    let (_, h1156) = v2;
                    h1156
                }, f280(v0, v3))
            } else {
                if f86({
                    let (_, h1157) = v2;
                    h1157
                }, v0.clone()) {
                    List::cons({
                        let (h1158, _) = v2;
                        h1158
                    }, f280(v0, v3))
                } else {
                    f280(v0, v3)
                }
            }
        }
    }
}

pub fn f281(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f86(v2.clone(), match v0.clone() {
        Adt15::C0(h1159, _, _) => {
            h1159
        }
    }) {
        true
    } else {
        if f86(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1160, _) => {
                h1160
            }
        }) {
            true
        } else {
            if f86(v2, match v0.clone() {
                Adt15::C0(_, _, h1161) => {
                    h1161
                }
            }) {
                true
            } else {
                if f197(v1.clone(), f8(Str::lit("scheme"))) {
                    f86(f150(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1162, _) => {
                            h1162
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f282(v0: Bytes, v1: u64) -> R<u64> {
    let m1165 = {
        let a1163 = v0;
        let a1164 = v1;
        index_bytes(a1163, a1164)
    };
    match m1165 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1210 = {
                let a1186 = {
                    let a1174 = {
                        let a1168 = {
                            let a1166 = v3;
                            let a1167 = 128u8;
                            fixed_u8::bit_and(a1166, a1167)
                        };
                        let a1169 = 128u8;
                        if equal(a1168, a1169) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1175 = {
                        let a1172 = {
                            let a1170 = v3;
                            let a1171 = 64u8;
                            fixed_u8::bit_and(a1170, a1171)
                        };
                        let a1173 = 64u8;
                        if equal(a1172, a1173) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1174, a1175)?
                };
                let a1187 = {
                    let a1184 = {
                        let a1178 = {
                            let a1176 = v3;
                            let a1177 = 32u8;
                            fixed_u8::bit_and(a1176, a1177)
                        };
                        let a1179 = 32u8;
                        if equal(a1178, a1179) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1185 = {
                        let a1182 = {
                            let a1180 = v3;
                            let a1181 = 16u8;
                            fixed_u8::bit_and(a1180, a1181)
                        };
                        let a1183 = 16u8;
                        if equal(a1182, a1183) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1184, a1185)?
                };
                nat_add(a1186, a1187)?
            };
            let a1211 = {
                let a1208 = {
                    let a1196 = {
                        let a1190 = {
                            let a1188 = v3;
                            let a1189 = 8u8;
                            fixed_u8::bit_and(a1188, a1189)
                        };
                        let a1191 = 8u8;
                        if equal(a1190, a1191) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1197 = {
                        let a1194 = {
                            let a1192 = v3;
                            let a1193 = 4u8;
                            fixed_u8::bit_and(a1192, a1193)
                        };
                        let a1195 = 4u8;
                        if equal(a1194, a1195) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1196, a1197)?
                };
                let a1209 = {
                    let a1206 = {
                        let a1200 = {
                            let a1198 = v3;
                            let a1199 = 2u8;
                            fixed_u8::bit_and(a1198, a1199)
                        };
                        let a1201 = 2u8;
                        if equal(a1200, a1201) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1207 = {
                        let a1204 = {
                            let a1202 = v3;
                            let a1203 = 1u8;
                            fixed_u8::bit_and(a1202, a1203)
                        };
                        let a1205 = 1u8;
                        if equal(a1204, a1205) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1206, a1207)?
                };
                nat_add(a1208, a1209)?
            };
            nat_add(a1210, a1211)
        }
    }
}

pub fn f283(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1215 = {
        let a1212 = v1;
        let a1213 = v2;
        let a1214 = 1u64;
        slice_bytes(a1212, a1213, a1214)
    };
    match m1215 {
        None => {
            v0
        }
        Some(v3) => {
            let a1216 = v0;
            let a1217 = v3;
            append_bytes(a1216, a1217)
        }
    }
}

pub fn f284(p1220: List<Bytes>, p1221: List<Bytes>) -> List<Bytes> {
    let m1219 = (p1220, p1221);
    {
        let mut m1219 = m1219;
        loop {
            let step = {
                let (v0, v1) = m1219;
                let m1218 = v0;
                match m1218.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1219 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: Bytes, p1293: u64, p1294: u64, p1295: Bytes) -> R<Bytes> {
    let m1292 = (p1293, (p1294, p1295));
    {
        let mut m1292 = m1292;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1292;
                let m1222 = v2;
                if m1222 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1222 - 1;
                    let m1225 = {
                        let a1223 = v0.clone();
                        let a1224 = v1;
                        index_bytes(a1223, a1224)
                    };
                    match m1225 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1270 = {
                                    let a1246 = {
                                        let a1234 = {
                                            let a1228 = {
                                                let a1226 = v6;
                                                let a1227 = 128u8;
                                                fixed_u8::bit_and(a1226, a1227)
                                            };
                                            let a1229 = 128u8;
                                            if equal(a1228, a1229) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1235 = {
                                            let a1232 = {
                                                let a1230 = v6;
                                                let a1231 = 64u8;
                                                fixed_u8::bit_and(a1230, a1231)
                                            };
                                            let a1233 = 64u8;
                                            if equal(a1232, a1233) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1234, a1235)?
                                    };
                                    let a1247 = {
                                        let a1244 = {
                                            let a1238 = {
                                                let a1236 = v6;
                                                let a1237 = 32u8;
                                                fixed_u8::bit_and(a1236, a1237)
                                            };
                                            let a1239 = 32u8;
                                            if equal(a1238, a1239) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1245 = {
                                            let a1242 = {
                                                let a1240 = v6;
                                                let a1241 = 16u8;
                                                fixed_u8::bit_and(a1240, a1241)
                                            };
                                            let a1243 = 16u8;
                                            if equal(a1242, a1243) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1244, a1245)?
                                    };
                                    nat_add(a1246, a1247)?
                                };
                                let a1271 = {
                                    let a1268 = {
                                        let a1256 = {
                                            let a1250 = {
                                                let a1248 = v6;
                                                let a1249 = 8u8;
                                                fixed_u8::bit_and(a1248, a1249)
                                            };
                                            let a1251 = 8u8;
                                            if equal(a1250, a1251) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1257 = {
                                            let a1254 = {
                                                let a1252 = v6;
                                                let a1253 = 4u8;
                                                fixed_u8::bit_and(a1252, a1253)
                                            };
                                            let a1255 = 4u8;
                                            if equal(a1254, a1255) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1256, a1257)?
                                    };
                                    let a1269 = {
                                        let a1266 = {
                                            let a1260 = {
                                                let a1258 = v6;
                                                let a1259 = 2u8;
                                                fixed_u8::bit_and(a1258, a1259)
                                            };
                                            let a1261 = 2u8;
                                            if equal(a1260, a1261) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1267 = {
                                            let a1264 = {
                                                let a1262 = v6;
                                                let a1263 = 1u8;
                                                fixed_u8::bit_and(a1262, a1263)
                                            };
                                            let a1265 = 1u8;
                                            if equal(a1264, a1265) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1266, a1267)?
                                    };
                                    nat_add(a1268, a1269)?
                                };
                                nat_add(a1270, a1271)?
                            };
                            let a1272 = v7;
                            let a1273 = 34u64;
                            if if nat_eq(a1272, a1273) {
                                true
                            } else {
                                let a1274 = v7;
                                let a1275 = 92u64;
                                nat_eq(a1274, a1275)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1276 = v1;
                                    let a1277 = 1u64;
                                    nat_add(a1276, a1277)?
                                }, (v4, {
                                    let a1280 = v3;
                                    let a1281 = {
                                        let a1278 = Bytes::lit(&[0x5c]);
                                        let a1279 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1278, a1279)
                                    };
                                    append_bytes(a1280, a1281)
                                })))
                            } else {
                                let a1282 = v7;
                                let a1283 = 10u64;
                                if nat_eq(a1282, a1283) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1284 = v1;
                                        let a1285 = 1u64;
                                        nat_add(a1284, a1285)?
                                    }, (v4, {
                                        let a1286 = v3;
                                        let a1287 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1286, a1287)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1288 = v1;
                                        let a1289 = 1u64;
                                        nat_add(a1288, a1289)?
                                    }, (v4, {
                                        let a1290 = v3;
                                        let a1291 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1290, a1291)
                                    })))
                                }
                            }
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

pub fn f286(v0: u32) -> Bytes {
    let a1324 = f347({
        let a1296 = f348(v0, 28u32);
        let a1297 = 15u32;
        fixed_u32::bit_and(a1296, a1297)
    });
    let a1325 = {
        let a1322 = f347({
            let a1298 = f348(v0, 24u32);
            let a1299 = 15u32;
            fixed_u32::bit_and(a1298, a1299)
        });
        let a1323 = {
            let a1320 = f347({
                let a1300 = f348(v0, 20u32);
                let a1301 = 15u32;
                fixed_u32::bit_and(a1300, a1301)
            });
            let a1321 = {
                let a1318 = f347({
                    let a1302 = f348(v0, 16u32);
                    let a1303 = 15u32;
                    fixed_u32::bit_and(a1302, a1303)
                });
                let a1319 = {
                    let a1316 = f347({
                        let a1304 = f348(v0, 12u32);
                        let a1305 = 15u32;
                        fixed_u32::bit_and(a1304, a1305)
                    });
                    let a1317 = {
                        let a1314 = f347({
                            let a1306 = f348(v0, 8u32);
                            let a1307 = 15u32;
                            fixed_u32::bit_and(a1306, a1307)
                        });
                        let a1315 = {
                            let a1312 = f347({
                                let a1308 = f348(v0, 4u32);
                                let a1309 = 15u32;
                                fixed_u32::bit_and(a1308, a1309)
                            });
                            let a1313 = f347({
                                let a1310 = v0;
                                let a1311 = 15u32;
                                fixed_u32::bit_and(a1310, a1311)
                            });
                            append_bytes(a1312, a1313)
                        };
                        append_bytes(a1314, a1315)
                    };
                    append_bytes(a1316, a1317)
                };
                append_bytes(a1318, a1319)
            };
            append_bytes(a1320, a1321)
        };
        append_bytes(a1322, a1323)
    };
    append_bytes(a1324, a1325)
}

pub fn f287(v0: Fn4, p1329: u64, p1330: (u64, Adt23)) -> R<(u64, Adt23)> {
    let m1328 = (p1329, p1330);
    {
        let mut m1328 = m1328;
        loop {
            let step = {
                let (v1, v2) = m1328;
                let m1326 = v1;
                if m1326 == 0 {
                    Err::<(u64, (u64, Adt23)), R<(u64, Adt23)>>(Ok(v2))
                } else {
                    let v3 = m1326 - 1;
                    Ok::<(u64, (u64, Adt23)), R<(u64, Adt23)>>((v3, {
                        let c1327: Fn4 = v0.clone();
                        c1327.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt23)) -> R<(u64, Adt23)> {
    Ok(({
        let a1332 = {
            let (h1331, _) = v3.clone();
            h1331
        };
        let a1333 = 64u64;
        nat_add(a1332, a1333)?
    }, f349({
        let (_, h1334) = v3;
        h1334
    }, f350(v0, v1, v2, {
        let (h1335, _) = v3;
        h1335
    })?)?))
}

pub fn f289() -> Adt23 {
    Adt23::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f290(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1336 = f30(v1, 34u8)?;
    match m1336.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1337 = v3;
            match m1337.uncons() {
                None => {
                    Ok(f351(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f291(v0: List<Bytes>) -> List<Bytes> {
    let m1338 = v0;
    match m1338.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f291(v2)
            } else {
                List::cons(v1, f291(v2))
            }
        }
    }
}

pub fn f292(v0: Bytes) -> R<u64> {
    let (h1339, _) = f244(Fn3::F352(v0.clone()), {
        let a1340 = v0.clone();
        length_bytes(a1340)
    }, {
        let a1341 = v0;
        length_bytes(a1341)
    })?;
    Ok(h1339)
}

pub fn f293(v0: Fn5, p1346: u64, p1347: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1345 = (p1346, p1347);
    {
        let mut m1345 = m1345;
        loop {
            let step = {
                let (v1, v2) = m1345;
                let m1342 = v1;
                if m1342 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1342 - 1;
                    let m1344 = {
                        let c1343: Fn5 = v0.clone();
                        c1343.apply(v2.clone())?
                    };
                    match m1344 {
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
                Ok(next) => { m1345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f353(v0, v1)
}

pub fn f295(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f354(v0, v1)
}

pub fn f296(v0: u64) -> bool {
    let a1348 = v0;
    let a1349 = 32u64;
    if if nat_eq(a1348, a1349) {
        true
    } else {
        let a1350 = v0;
        let a1351 = 9u64;
        nat_eq(a1350, a1351)
    } {
        true
    } else {
        let a1352 = v0;
        let a1353 = 10u64;
        if nat_eq(a1352, a1353) {
            true
        } else {
            let a1354 = v0;
            let a1355 = 13u64;
            nat_eq(a1354, a1355)
        }
    }
}

pub fn f297(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1356, _) = f244(Fn3::F355(v1.clone(), v0.clone()), {
            let a1358 = {
                let a1357 = v0.clone();
                length_bytes(a1357)
            };
            let a1359 = 1u64;
            nat_add(a1358, a1359)?
        }, v2)?;
        h1356
    };
    if f213(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f298(v0: Bytes, v1: u64) -> R<u64> {
    let (h1360, _) = f244(Fn3::F356(v0.clone()), {
        let a1362 = {
            let a1361 = v0;
            length_bytes(a1361)
        };
        let a1363 = 1u64;
        nat_add(a1362, a1363)?
    }, v1)?;
    Ok(h1360)
}

pub fn f299(v0: Bytes) -> R<bool> {
    let a1365 = {
        let a1364 = v0.clone();
        length_bytes(a1364)
    };
    let a1366 = 5u64;
    if nat_eq(a1365, a1366) {
        let a1367 = f302(v0.clone(), 0u64)?;
        let a1368 = 85u64;
        if if nat_eq(a1367, a1368) {
            true
        } else {
            let a1369 = f302(v0.clone(), 0u64)?;
            let a1370 = 117u64;
            nat_eq(a1369, a1370)
        } {
            let a1371 = f302(v0.clone(), 1u64)?;
            let a1372 = 84u64;
            if if nat_eq(a1371, a1372) {
                true
            } else {
                let a1373 = f302(v0.clone(), 1u64)?;
                let a1374 = 116u64;
                nat_eq(a1373, a1374)
            } {
                let a1375 = f302(v0.clone(), 2u64)?;
                let a1376 = 70u64;
                if if nat_eq(a1375, a1376) {
                    true
                } else {
                    let a1377 = f302(v0.clone(), 2u64)?;
                    let a1378 = 102u64;
                    nat_eq(a1377, a1378)
                } {
                    let a1379 = f302(v0.clone(), 3u64)?;
                    let a1380 = 45u64;
                    if nat_eq(a1379, a1380) {
                        let a1381 = f302(v0, 4u64)?;
                        let a1382 = 56u64;
                        Ok(nat_eq(a1381, a1382))
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

pub fn f300(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1386 = f297(v0.clone(), {
        let a1383 = Str::lit("--");
        utf8_encode(a1383)
    }, {
        let a1384 = v1;
        let a1385 = 4u64;
        nat_add(a1384, a1385)?
    })?;
    match m1386 {
        None => {
            Ok(Err::<u64, Bytes>(f216(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1389 = f220(v0, {
                let a1387 = v2;
                let a1388 = 2u64;
                nat_add(a1387, a1388)?
            })?;
            let a1390 = 62u64;
            if nat_eq(a1389, a1390) {
                Ok(Ok::<u64, Bytes>({
                    let a1391 = v2;
                    let a1392 = 3u64;
                    nat_add(a1391, a1392)?
                }))
            } else {
                Ok(Err::<u64, Bytes>(f216(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f301(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f303(v0.clone(), {
        let a1393 = v1;
        let a1394 = 2u64;
        nat_add(a1393, a1394)?
    })?;
    let a1397 = v2;
    let a1398 = {
        let a1395 = v1;
        let a1396 = 2u64;
        nat_add(a1395, a1396)?
    };
    if nat_eq(a1397, a1398) {
        Ok(Err::<u64, Bytes>(f216(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1401 = v2;
        let a1402 = {
            let a1399 = v1;
            let a1400 = 5u64;
            nat_add(a1399, a1400)?
        };
        if if nat_eq(a1401, a1402) {
            let a1405 = f220(v0.clone(), {
                let a1403 = v1;
                let a1404 = 2u64;
                nat_add(a1403, a1404)?
            })?;
            let a1406 = 120u64;
            if if nat_eq(a1405, a1406) {
                true
            } else {
                let a1409 = f220(v0.clone(), {
                    let a1407 = v1;
                    let a1408 = 2u64;
                    nat_add(a1407, a1408)?
                })?;
                let a1410 = 88u64;
                nat_eq(a1409, a1410)
            } {
                let a1413 = f220(v0.clone(), {
                    let a1411 = v1;
                    let a1412 = 3u64;
                    nat_add(a1411, a1412)?
                })?;
                let a1414 = 109u64;
                if if nat_eq(a1413, a1414) {
                    true
                } else {
                    let a1417 = f220(v0.clone(), {
                        let a1415 = v1;
                        let a1416 = 3u64;
                        nat_add(a1415, a1416)?
                    })?;
                    let a1418 = 77u64;
                    nat_eq(a1417, a1418)
                } {
                    let a1421 = f220(v0.clone(), {
                        let a1419 = v1;
                        let a1420 = 4u64;
                        nat_add(a1419, a1420)?
                    })?;
                    let a1422 = 108u64;
                    if nat_eq(a1421, a1422) {
                        true
                    } else {
                        let a1425 = f220(v0.clone(), {
                            let a1423 = v1;
                            let a1424 = 4u64;
                            nat_add(a1423, a1424)?
                        })?;
                        let a1426 = 76u64;
                        nat_eq(a1425, a1426)
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
            Ok(Err::<u64, Bytes>(f216(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1428 = f297(v0, {
                let a1427 = Str::lit("?>");
                utf8_encode(a1427)
            }, v2)?;
            match m1428 {
                None => {
                    Ok(Err::<u64, Bytes>(f216(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1429 = v3;
                        let a1430 = 2u64;
                        nat_add(a1429, a1430)?
                    }))
                }
            }
        }
    }
}

pub fn f302(v0: Bytes, v1: u64) -> R<u64> {
    let m1433 = {
        let a1431 = v0;
        let a1432 = v1;
        index_bytes(a1431, a1432)
    };
    match m1433 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1478 = {
                let a1454 = {
                    let a1442 = {
                        let a1436 = {
                            let a1434 = v3;
                            let a1435 = 128u8;
                            fixed_u8::bit_and(a1434, a1435)
                        };
                        let a1437 = 128u8;
                        if equal(a1436, a1437) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1443 = {
                        let a1440 = {
                            let a1438 = v3;
                            let a1439 = 64u8;
                            fixed_u8::bit_and(a1438, a1439)
                        };
                        let a1441 = 64u8;
                        if equal(a1440, a1441) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1442, a1443)?
                };
                let a1455 = {
                    let a1452 = {
                        let a1446 = {
                            let a1444 = v3;
                            let a1445 = 32u8;
                            fixed_u8::bit_and(a1444, a1445)
                        };
                        let a1447 = 32u8;
                        if equal(a1446, a1447) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1453 = {
                        let a1450 = {
                            let a1448 = v3;
                            let a1449 = 16u8;
                            fixed_u8::bit_and(a1448, a1449)
                        };
                        let a1451 = 16u8;
                        if equal(a1450, a1451) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1452, a1453)?
                };
                nat_add(a1454, a1455)?
            };
            let a1479 = {
                let a1476 = {
                    let a1464 = {
                        let a1458 = {
                            let a1456 = v3;
                            let a1457 = 8u8;
                            fixed_u8::bit_and(a1456, a1457)
                        };
                        let a1459 = 8u8;
                        if equal(a1458, a1459) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1465 = {
                        let a1462 = {
                            let a1460 = v3;
                            let a1461 = 4u8;
                            fixed_u8::bit_and(a1460, a1461)
                        };
                        let a1463 = 4u8;
                        if equal(a1462, a1463) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1464, a1465)?
                };
                let a1477 = {
                    let a1474 = {
                        let a1468 = {
                            let a1466 = v3;
                            let a1467 = 2u8;
                            fixed_u8::bit_and(a1466, a1467)
                        };
                        let a1469 = 2u8;
                        if equal(a1468, a1469) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1475 = {
                        let a1472 = {
                            let a1470 = v3;
                            let a1471 = 1u8;
                            fixed_u8::bit_and(a1470, a1471)
                        };
                        let a1473 = 1u8;
                        if equal(a1472, a1473) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1474, a1475)?
                };
                nat_add(a1476, a1477)?
            };
            nat_add(a1478, a1479)
        }
    }
}

pub fn f303(v0: Bytes, v1: u64) -> R<u64> {
    let (h1480, _) = f244(Fn3::F357(v0.clone(), v1), {
        let a1482 = {
            let a1481 = v0;
            length_bytes(a1481)
        };
        let a1483 = 1u64;
        nat_add(a1482, a1483)?
    }, v1)?;
    Ok(h1480)
}

pub fn f304(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt18, u64), Bytes>> {
    let m1484 = v2;
    if m1484 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1484 - 1;
        let v5: u64 = f298(v0.clone(), v1)?;
        let a1485 = f220(v0.clone(), v5)?;
        let a1486 = 62u64;
        if if nat_eq(a1485, a1486) {
            true
        } else {
            f219(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v5)))
        } else {
            let a1487 = v5;
            let a1488 = v1;
            if nat_eq(a1487, a1488) {
                Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f303(v0.clone(), v5)?;
                let a1489 = v6;
                let a1490 = v5;
                if nat_eq(a1489, a1490) {
                    Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a1491 = v6;
                        let a1492 = v5;
                        nat_sub(a1491, a1492)
                    });
                    if f358(v7.clone(), v3.clone()) {
                        Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f298(v0.clone(), v6)?;
                        let a1493 = f220(v0.clone(), v8)?;
                        let a1494 = 61u64;
                        if !nat_eq(a1493, a1494) {
                            Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        } else {
                            let v9: u64 = f298(v0.clone(), {
                                let a1495 = v8;
                                let a1496 = 1u64;
                                nat_add(a1495, a1496)?
                            })?;
                            let v10: u64 = f220(v0.clone(), v9)?;
                            let a1497 = v10;
                            let a1498 = 34u64;
                            if !if nat_eq(a1497, a1498) {
                                true
                            } else {
                                let a1499 = v10;
                                let a1500 = 39u64;
                                nat_eq(a1499, a1500)
                            } {
                                Ok(Err::<(Adt18, u64), Bytes>(f216(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            } else {
                                let m1503 = f305(v0.clone(), {
                                    let a1501 = v9;
                                    let a1502 = 1u64;
                                    nat_add(a1501, a1502)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1503 {
                                    Ok(v12) => {
                                        let m1505 = f304(v0, {
                                            let (_, h1504) = v12;
                                            h1504
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1505 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C1(v7, {
                                                    let (h1506, _) = v12;
                                                    h1506
                                                }, std::rc::Rc::new({
                                                    let (h1507, _) = v14;
                                                    h1507
                                                })), {
                                                    let (_, h1508) = v14;
                                                    h1508
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

pub fn f305(v0: Bytes, p1548: u64, p1549: u64, p1550: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1547 = (p1548, (p1549, p1550));
    {
        let mut m1547 = m1547;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1547;
                let m1509 = v2;
                if m1509 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f216(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1509 - 1;
                    let v7: u64 = f220(v0.clone(), v1)?;
                    if if v4 {
                        let a1510 = v7;
                        let a1511 = v5;
                        nat_eq(a1510, a1511)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1512 = v1;
                            let a1513 = 1u64;
                            nat_add(a1512, a1513)?
                        }))))
                    } else {
                        let a1514 = v7;
                        let a1515 = 256u64;
                        if nat_eq(a1514, a1515) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f216(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1516 = v7;
                            let a1517 = 60u64;
                            if nat_eq(a1516, a1517) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f216(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1518 = v7;
                                let a1519 = 38u64;
                                if nat_eq(a1518, a1519) {
                                    let m1520 = f359(v0.clone(), v1)?;
                                    match m1520 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1521) = v9;
                                                h1521
                                            }, (v6, {
                                                let a1523 = v3;
                                                let a1524 = {
                                                    let (h1522, _) = v9;
                                                    h1522
                                                };
                                                append_bytes(a1523, a1524)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1525 = v7;
                                        let a1526 = 9u64;
                                        if if nat_eq(a1525, a1526) {
                                            true
                                        } else {
                                            let a1527 = v7;
                                            let a1528 = 10u64;
                                            nat_eq(a1527, a1528)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1529 = v1;
                                                let a1530 = 1u64;
                                                nat_add(a1529, a1530)?
                                            }, (v6, {
                                                let a1531 = v3;
                                                let a1532 = Bytes::lit(&[0x20]);
                                                append_bytes(a1531, a1532)
                                            })))
                                        } else {
                                            let v10: u64 = f360(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1535 = v3;
                                                let a1536 = f32(v0.clone(), v1, {
                                                    let a1533 = v10;
                                                    let a1534 = v1;
                                                    nat_sub(a1533, a1534)
                                                });
                                                append_bytes(a1535, a1536)
                                            })))
                                        }
                                    } else {
                                        let a1537 = v7;
                                        let a1538 = 93u64;
                                        if nat_eq(a1537, a1538) {
                                            if f219(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f216(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1539 = v1;
                                                    let a1540 = 1u64;
                                                    nat_add(a1539, a1540)?
                                                }, (v6, {
                                                    let a1541 = v3;
                                                    let a1542 = Bytes::lit(&[0x5d]);
                                                    append_bytes(a1541, a1542)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f360(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1545 = v3;
                                                let a1546 = f32(v0.clone(), v1, {
                                                    let a1543 = v11;
                                                    let a1544 = v1;
                                                    nat_sub(a1543, a1544)
                                                });
                                                append_bytes(a1545, a1546)
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
                Ok(next) => { m1547 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Bytes, v1: u64) -> Bytes {
    let m1554 = {
        let a1551 = v0;
        let a1552 = v1;
        let a1553 = 1u64;
        slice_bytes(a1551, a1552, a1553)
    };
    match m1554 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f307(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f361(v1, v0, v2)
}

pub fn f308(v0: Bytes, v1: u64) -> R<u64> {
    let (h1555, _) = f244(Fn3::F362(v1, v0), v1, 0u64)?;
    Ok(h1555)
}

pub fn f309(v0: Bytes) -> R<Bytes> {
    f363(v0.clone(), 0u64, {
        let a1556 = v0;
        length_bytes(a1556)
    }, false, false, Bytes::lit(&[]))
}

pub fn f310(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1557 = v2;
    if m1557 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1557 - 1;
        let v5: u64 = f302(v0.clone(), v1)?;
        let a1558 = v3;
        let a1559 = 1u64;
        if nat_eq(a1558, a1559) {
            let m1560 = f310(v0.clone(), v1, v4, 0u64)?;
            match m1560 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f302(v0.clone(), {
                        let (_, h1561) = v6.clone();
                        h1561
                    })?;
                    let a1562 = v7;
                    let a1563 = 44u64;
                    if nat_eq(a1562, a1563) {
                        let m1567 = f310(v0, {
                            let a1565 = {
                                let (_, h1564) = v6;
                                h1564
                            };
                            let a1566 = 1u64;
                            nat_add(a1565, a1566)?
                        }, v4, 1u64)?;
                        match m1567 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1568, _) = v6;
                                    h1568
                                }), std::rc::Rc::new({
                                    let (h1569, _) = v8;
                                    h1569
                                })), {
                                    let (_, h1570) = v8;
                                    h1570
                                })))
                            }
                        }
                    } else {
                        let a1571 = v7;
                        let a1572 = 93u64;
                        if nat_eq(a1571, a1572) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1573, _) = v6;
                                h1573
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1575 = {
                                    let (_, h1574) = v6;
                                    h1574
                                };
                                let a1576 = 1u64;
                                nat_add(a1575, a1576)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1577 = v3;
            let a1578 = 2u64;
            if nat_eq(a1577, a1578) {
                let m1581 = f364(v0.clone(), {
                    let a1579 = v1;
                    let a1580 = 1u64;
                    nat_add(a1579, a1580)?
                })?;
                match m1581 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1582 = v5;
                        let a1583 = 34u64;
                        if if nat_eq(a1582, a1583) {
                            let a1585 = f302(v0.clone(), {
                                let (h1584, _) = v9.clone();
                                h1584
                            })?;
                            let a1586 = 58u64;
                            nat_eq(a1585, a1586)
                        } else {
                            false
                        } {
                            let m1590 = f310(v0.clone(), {
                                let a1588 = {
                                    let (h1587, _) = v9;
                                    h1587
                                };
                                let a1589 = 1u64;
                                nat_add(a1588, a1589)?
                            }, v4, 0u64)?;
                            match m1590 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f302(v0.clone(), {
                                        let (_, h1591) = v10.clone();
                                        h1591
                                    })?;
                                    let a1592 = v11;
                                    let a1593 = 44u64;
                                    if nat_eq(a1592, a1593) {
                                        let m1597 = f310(v0, {
                                            let a1595 = {
                                                let (_, h1594) = v10;
                                                h1594
                                            };
                                            let a1596 = 1u64;
                                            nat_add(a1595, a1596)?
                                        }, v4, 2u64)?;
                                        match m1597 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1598) = v9;
                                                    h1598
                                                }, std::rc::Rc::new({
                                                    let (h1599, _) = v10;
                                                    h1599
                                                }), std::rc::Rc::new({
                                                    let (h1600, _) = v12;
                                                    h1600
                                                })), {
                                                    let (_, h1601) = v12;
                                                    h1601
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1602 = v11;
                                        let a1603 = 125u64;
                                        if nat_eq(a1602, a1603) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1604) = v9;
                                                h1604
                                            }, std::rc::Rc::new({
                                                let (h1605, _) = v10;
                                                h1605
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1607 = {
                                                    let (_, h1606) = v10;
                                                    h1606
                                                };
                                                let a1608 = 1u64;
                                                nat_add(a1607, a1608)?
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
                let a1609 = v5;
                let a1610 = 34u64;
                if nat_eq(a1609, a1610) {
                    let m1613 = f364(v0, {
                        let a1611 = v1;
                        let a1612 = 1u64;
                        nat_add(a1611, a1612)?
                    })?;
                    match m1613 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1614) = v13;
                                h1614
                            }), {
                                let (h1615, _) = v13;
                                h1615
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1616 = v1;
                            let a1617 = 4u64;
                            nat_add(a1616, a1617)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1618 = v1;
                                let a1619 = 5u64;
                                nat_add(a1618, a1619)?
                            })))
                        } else {
                            let a1620 = v5;
                            let a1621 = 91u64;
                            if nat_eq(a1620, a1621) {
                                let a1624 = f302(v0.clone(), {
                                    let a1622 = v1;
                                    let a1623 = 1u64;
                                    nat_add(a1622, a1623)?
                                })?;
                                let a1625 = 93u64;
                                if nat_eq(a1624, a1625) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1626 = v1;
                                        let a1627 = 2u64;
                                        nat_add(a1626, a1627)?
                                    })))
                                } else {
                                    let m1630 = f310(v0, {
                                        let a1628 = v1;
                                        let a1629 = 1u64;
                                        nat_add(a1628, a1629)?
                                    }, v4, 1u64)?;
                                    match m1630 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1631, _) = v14;
                                                h1631
                                            })), {
                                                let (_, h1632) = v14;
                                                h1632
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1633 = v5;
                                let a1634 = 123u64;
                                if nat_eq(a1633, a1634) {
                                    let a1637 = f302(v0.clone(), {
                                        let a1635 = v1;
                                        let a1636 = 1u64;
                                        nat_add(a1635, a1636)?
                                    })?;
                                    let a1638 = 125u64;
                                    if nat_eq(a1637, a1638) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1639 = v1;
                                            let a1640 = 2u64;
                                            nat_add(a1639, a1640)?
                                        })))
                                    } else {
                                        let m1643 = f310(v0, {
                                            let a1641 = v1;
                                            let a1642 = 1u64;
                                            nat_add(a1641, a1642)?
                                        }, v4, 2u64)?;
                                        match m1643 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1644, _) = v15;
                                                    h1644
                                                })), {
                                                    let (_, h1645) = v15;
                                                    h1645
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1646 = f365(v0, v1)?;
                                    match m1646 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1647) = v16;
                                                h1647
                                            }, {
                                                let (h1648, _) = v16;
                                                h1648
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

pub fn f311() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f312(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1649 = f147(v0, v1);
    match m1649 {
        None => {
            false
        }
        Some(v3) => {
            f197(v3, v2)
        }
    }
}

pub fn f313() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f314(v0: Bytes, p1716: u64, p1717: u64, p1718: bool, p1719: Bytes) -> R<Bytes> {
    let m1715 = (p1716, (p1717, (p1718, p1719)));
    {
        let mut m1715 = m1715;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1715;
                let m1650 = v2;
                if m1650 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1650 - 1;
                    let m1653 = {
                        let a1651 = v0.clone();
                        let a1652 = v1;
                        index_bytes(a1651, a1652)
                    };
                    match m1653 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1698 = {
                                    let a1674 = {
                                        let a1662 = {
                                            let a1656 = {
                                                let a1654 = v7;
                                                let a1655 = 128u8;
                                                fixed_u8::bit_and(a1654, a1655)
                                            };
                                            let a1657 = 128u8;
                                            if equal(a1656, a1657) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1663 = {
                                            let a1660 = {
                                                let a1658 = v7;
                                                let a1659 = 64u8;
                                                fixed_u8::bit_and(a1658, a1659)
                                            };
                                            let a1661 = 64u8;
                                            if equal(a1660, a1661) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1662, a1663)?
                                    };
                                    let a1675 = {
                                        let a1672 = {
                                            let a1666 = {
                                                let a1664 = v7;
                                                let a1665 = 32u8;
                                                fixed_u8::bit_and(a1664, a1665)
                                            };
                                            let a1667 = 32u8;
                                            if equal(a1666, a1667) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1673 = {
                                            let a1670 = {
                                                let a1668 = v7;
                                                let a1669 = 16u8;
                                                fixed_u8::bit_and(a1668, a1669)
                                            };
                                            let a1671 = 16u8;
                                            if equal(a1670, a1671) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1672, a1673)?
                                    };
                                    nat_add(a1674, a1675)?
                                };
                                let a1699 = {
                                    let a1696 = {
                                        let a1684 = {
                                            let a1678 = {
                                                let a1676 = v7;
                                                let a1677 = 8u8;
                                                fixed_u8::bit_and(a1676, a1677)
                                            };
                                            let a1679 = 8u8;
                                            if equal(a1678, a1679) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1685 = {
                                            let a1682 = {
                                                let a1680 = v7;
                                                let a1681 = 4u8;
                                                fixed_u8::bit_and(a1680, a1681)
                                            };
                                            let a1683 = 4u8;
                                            if equal(a1682, a1683) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1684, a1685)?
                                    };
                                    let a1697 = {
                                        let a1694 = {
                                            let a1688 = {
                                                let a1686 = v7;
                                                let a1687 = 2u8;
                                                fixed_u8::bit_and(a1686, a1687)
                                            };
                                            let a1689 = 2u8;
                                            if equal(a1688, a1689) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1695 = {
                                            let a1692 = {
                                                let a1690 = v7;
                                                let a1691 = 1u8;
                                                fixed_u8::bit_and(a1690, a1691)
                                            };
                                            let a1693 = 1u8;
                                            if equal(a1692, a1693) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1694, a1695)?
                                    };
                                    nat_add(a1696, a1697)?
                                };
                                nat_add(a1698, a1699)?
                            };
                            if f366(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1700 = v1;
                                    let a1701 = 1u64;
                                    nat_add(a1700, a1701)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f367(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1702 = v1;
                                        let a1703 = 1u64;
                                        nat_add(a1702, a1703)?
                                    }, (v5, (false, {
                                        let a1707 = v4;
                                        let a1708 = f32({
                                            let a1704 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1704)
                                        }, {
                                            let a1705 = v8;
                                            let a1706 = 97u64;
                                            nat_sub(a1705, a1706)
                                        }, 1u64);
                                        append_bytes(a1707, a1708)
                                    }))))
                                } else {
                                    if if f367(v8) {
                                        true
                                    } else {
                                        if f368(v8) {
                                            true
                                        } else {
                                            f369(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1709 = v1;
                                            let a1710 = 1u64;
                                            nat_add(a1709, a1710)?
                                        }, (v5, (false, {
                                            let a1711 = v4;
                                            let a1712 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1711, a1712)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1713 = v1;
                                            let a1714 = 1u64;
                                            nat_add(a1713, a1714)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1715 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt14)> = f370(f371(f147(v0.clone(), f8(Str::lit("$defs")))), f371(f147(v0.clone(), f8(Str::lit("definitions")))));
    let m1720 = f372(4096u64, v3.clone(), List::<Bytes>::nil(), f373(v3)?)?;
    match m1720 {
        Ok(v5) => {
            let m1721 = f374(4096u64, f375(f376(v5)?, List::cons(Adt24::C0(v2, v0), List::<Adt24>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1721 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f209(f180(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f316(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1722 = v0;
    match m1722.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f36(f150(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f377(v5, v1, v2))
            } else {
                List::cons(v4, f316(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f209(f378(f83(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f318(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1723 = 0u64;
        let a1724 = v1;
        nat_lt(a1723, a1724)
    } else {
        false
    } {
        let a1725 = Bytes::lit(&[0x2d]);
        let a1726 = f126(v1)?;
        Ok(append_bytes(a1725, a1726))
    } else {
        f126(v1)
    }
}

pub fn f319(v0: Bytes) -> R<Bytes> {
    let a1732 = {
        let a1730 = Bytes::lit(&[0x22]);
        let a1731 = {
            let (_, h1727) = {
                let (h1728, _) = f293(Fn5::F379(v0.clone()), {
                    let a1729 = v0;
                    length_bytes(a1729)
                }, (0u64, Bytes::lit(&[])))?;
                h1728
            };
            h1727
        };
        append_bytes(a1730, a1731)
    };
    let a1733 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1732, a1733))
}

pub fn f320(v0: List<Bytes>) -> Bytes {
    let a1736 = {
        let a1734 = Bytes::lit(&[0x5b]);
        let a1735 = f121(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1734, a1735)
    };
    let a1737 = Bytes::lit(&[0x5d]);
    append_bytes(a1736, a1737)
}

pub fn f321(v0: Adt14) -> R<List<Bytes>> {
    let m1738 = v0;
    match m1738 {
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
        Adt14::C6(r1739, r1740) => {
            let v7 = (*r1739).clone();
            let v8 = (*r1740).clone();
            Ok(List::cons(f237(v7)?, f321(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f322(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f380(Fn10::F381, List::<(Bytes, Bytes)>::nil(), f382(v0)?);
    let v2: List<Bytes> = f383(Fn11::F384, List::<Bytes>::nil(), v1)?;
    let a1743 = {
        let a1741 = Bytes::lit(&[0x7b]);
        let a1742 = f121(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1741, a1742)
    };
    let a1744 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1743, a1744))
}

pub fn f323(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1745 = v0;
    match m1745 {
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
        Adt14::C7(v9, r1746, r1747) => {
            let v10 = (*r1746).clone();
            let v11 = (*r1747).clone();
            Ok(List::cons((v9, f237(v10)?), f323(v11)?))
        }
    }
}

pub fn f324(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1748 = v0;
    match m1748 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f150(v2, v1)
        }
    }
}

pub fn f325(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f209(f385(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f326(v0: Adt16, v1: Option<u64>) -> bool {
    let m1749 = v1;
    match m1749 {
        None => {
            true
        }
        Some(v2) => {
            f253(v2, match v0 {
                Adt16::C0(_, h1750, _, _) => {
                    h1750
                }
            })
        }
    }
}

pub fn f327(v0: Adt12) -> Adt20 {
    Adt20::C1({
        let a1752 = match v0 {
            Adt12::C0(_, _, h1751, _, _) => {
                h1751
            }
        };
        length_list(a1752)
    })
}

pub fn f328(v0: Bytes) -> Option<Adt21> {
    if f36(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt21::C0)
    } else {
        if f36(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt21::C1)
        } else {
            if f36(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt21::C2)
            } else {
                None::<Adt21>
            }
        }
    }
}

pub fn f329(v0: Bytes, p1757: List<Bytes>, p1758: u64) -> R<Option<u64>> {
    let m1756 = (p1757, p1758);
    {
        let mut m1756 = m1756;
        loop {
            let step = {
                let (v1, v2) = m1756;
                let m1753 = v1;
                match m1753.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1754 = v2;
                                let a1755 = 1u64;
                                nat_add(a1754, a1755)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330(v0: Option<Adt14>) -> Option<u64> {
    let m1759 = v0;
    match m1759 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1760 = v1;
            match m1760 {
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

pub fn f331(v0: Fn6, p1764: List<Bytes>, p1765: List<(Bytes, ())>) -> List<Bytes> {
    let m1763 = (p1764, p1765);
    {
        let mut m1763 = m1763;
        loop {
            let step = {
                let (v1, v2) = m1763;
                let m1761 = v2;
                match m1761.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1762: Fn6 = v0.clone();
                            c1762.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1763 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1766, _) = v1;
        h1766
    }, v0)
}

pub fn f333(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f386(Fn12::F387, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f334(v0: Fn7, p1770: List<List<(Bytes, ())>>, p1771: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1769 = (p1770, p1771);
    {
        let mut m1769 = m1769;
        loop {
            let step = {
                let (v1, v2) = m1769;
                let m1767 = v2;
                match m1767.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1768: Fn7 = v0.clone();
                            c1768.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f336(v0: Fn8, p1776: u64, p1777: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1775 = (p1776, p1777);
    {
        let mut m1775 = m1775;
        loop {
            let step = {
                let (v1, v2) = m1775;
                let m1772 = v1;
                if m1772 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1772 - 1;
                    let m1774 = {
                        let c1773: Fn8 = v0.clone();
                        c1773.apply(v2.clone())?
                    };
                    match m1774 {
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
                Ok(next) => { m1775 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f388(v0)
}

pub fn f338(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f389(Fn13::F390, List::<(Bytes, ())>::nil(), v0)
}

pub fn f339(v0: Fn9, p1781: List<(Bytes, ())>, p1782: List<Bytes>) -> List<(Bytes, ())> {
    let m1780 = (p1781, p1782);
    {
        let mut m1780 = m1780;
        loop {
            let step = {
                let (v1, v2) = m1780;
                let m1778 = v2;
                match m1778.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1779: Fn9 = v0.clone();
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

pub fn f340(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f341(v0: Bytes) -> Bytes {
    if f134(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1784 = {
                let a1783 = v0;
                length_bytes(a1783)
            };
            let a1785 = 1u64;
            nat_sub(a1784, a1785)
        })
    } else {
        v0
    }
}

pub fn f342(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f83(f70(v0.clone(), v1.clone()))?;
    let m1786 = f152(v2.clone())?;
    match m1786 {
        None => {
            f392(v0, v1, v2)
        }
        Some(v3) => {
            f391(v0, v1, v3)
        }
    }
}

pub fn f343(v0: List<Bytes>) -> List<Bytes> {
    let m1787 = v0;
    match m1787.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1789 = {
                    let a1788 = v1.clone();
                    length_bytes(a1788)
                };
                let a1790 = 0u64;
                if nat_eq(a1789, a1790) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f343(v2))
        }
    }
}

pub fn f344(v0: List<Bytes>) -> Bytes {
    let m1791 = v0;
    match m1791.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f345(v0: (Bytes, Bytes), p1794: List<(Bytes, Bytes)>) -> bool {
    let m1793 = p1794;
    {
        let mut m1793 = m1793;
        loop {
            let step = {
                let v1 = m1793;
                let m1792 = v1;
                match m1792.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f393(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1793 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f346(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1795 = v1;
    match m1795.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1796, _) = v2;
                h1796
            }, v0.clone()) {
                List::cons({
                    let (_, h1797) = v2;
                    h1797
                }, f346(v0, v3))
            } else {
                f346(v0, v3)
            }
        }
    }
}

pub fn f347(v0: u32) -> Bytes {
    let a1800 = {
        let a1798 = v0;
        let a1799 = 8u32;
        fixed_u32::bit_and(a1798, a1799)
    };
    let a1801 = 0u32;
    if equal(a1800, a1801) {
        let a1804 = {
            let a1802 = v0;
            let a1803 = 4u32;
            fixed_u32::bit_and(a1802, a1803)
        };
        let a1805 = 0u32;
        if equal(a1804, a1805) {
            let a1808 = {
                let a1806 = v0;
                let a1807 = 2u32;
                fixed_u32::bit_and(a1806, a1807)
            };
            let a1809 = 0u32;
            if equal(a1808, a1809) {
                let a1812 = {
                    let a1810 = v0;
                    let a1811 = 1u32;
                    fixed_u32::bit_and(a1810, a1811)
                };
                let a1813 = 0u32;
                if equal(a1812, a1813) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1816 = {
                    let a1814 = v0;
                    let a1815 = 1u32;
                    fixed_u32::bit_and(a1814, a1815)
                };
                let a1817 = 0u32;
                if equal(a1816, a1817) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1820 = {
                let a1818 = v0;
                let a1819 = 2u32;
                fixed_u32::bit_and(a1818, a1819)
            };
            let a1821 = 0u32;
            if equal(a1820, a1821) {
                let a1824 = {
                    let a1822 = v0;
                    let a1823 = 1u32;
                    fixed_u32::bit_and(a1822, a1823)
                };
                let a1825 = 0u32;
                if equal(a1824, a1825) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1828 = {
                    let a1826 = v0;
                    let a1827 = 1u32;
                    fixed_u32::bit_and(a1826, a1827)
                };
                let a1829 = 0u32;
                if equal(a1828, a1829) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1832 = {
            let a1830 = v0;
            let a1831 = 4u32;
            fixed_u32::bit_and(a1830, a1831)
        };
        let a1833 = 0u32;
        if equal(a1832, a1833) {
            let a1836 = {
                let a1834 = v0;
                let a1835 = 2u32;
                fixed_u32::bit_and(a1834, a1835)
            };
            let a1837 = 0u32;
            if equal(a1836, a1837) {
                let a1840 = {
                    let a1838 = v0;
                    let a1839 = 1u32;
                    fixed_u32::bit_and(a1838, a1839)
                };
                let a1841 = 0u32;
                if equal(a1840, a1841) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1844 = {
                    let a1842 = v0;
                    let a1843 = 1u32;
                    fixed_u32::bit_and(a1842, a1843)
                };
                let a1845 = 0u32;
                if equal(a1844, a1845) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1848 = {
                let a1846 = v0;
                let a1847 = 2u32;
                fixed_u32::bit_and(a1846, a1847)
            };
            let a1849 = 0u32;
            if equal(a1848, a1849) {
                let a1852 = {
                    let a1850 = v0;
                    let a1851 = 1u32;
                    fixed_u32::bit_and(a1850, a1851)
                };
                let a1853 = 0u32;
                if equal(a1852, a1853) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1856 = {
                    let a1854 = v0;
                    let a1855 = 1u32;
                    fixed_u32::bit_and(a1854, a1855)
                };
                let a1857 = 0u32;
                if equal(a1856, a1857) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f348(v0: u32, v1: u32) -> u32 {
    let m1860 = {
        let a1858 = v0;
        let a1859 = v1;
        fixed_u32::shift_right(a1858, a1859)
    };
    match m1860 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f349(v0: Adt23, v1: Adt25) -> R<Adt23> {
    let v2: Adt26 = f394(Fn14::F395, 64u64, Adt26::C0(match v0.clone() {
        Adt23::C0(h1861, _, _, _, _, _, _, _) => {
            h1861
        }
    }, match v0.clone() {
        Adt23::C0(_, h1862, _, _, _, _, _, _) => {
            h1862
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1863, _, _, _, _, _) => {
            h1863
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h1864, _, _, _, _) => {
            h1864
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, h1865, _, _, _) => {
            h1865
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1866, _, _) => {
            h1866
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1867, _) => {
            h1867
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h1868) => {
            h1868
        }
    }, v1, 0u64))?;
    Ok(Adt23::C0(f396(match v0.clone() {
        Adt23::C0(h1869, _, _, _, _, _, _, _) => {
            h1869
        }
    }, match v2.clone() {
        Adt26::C0(h1870, _, _, _, _, _, _, _, _, _) => {
            h1870
        }
    }), f396(match v0.clone() {
        Adt23::C0(_, h1871, _, _, _, _, _, _) => {
            h1871
        }
    }, match v2.clone() {
        Adt26::C0(_, h1872, _, _, _, _, _, _, _, _) => {
            h1872
        }
    }), f396(match v0.clone() {
        Adt23::C0(_, _, h1873, _, _, _, _, _) => {
            h1873
        }
    }, match v2.clone() {
        Adt26::C0(_, _, h1874, _, _, _, _, _, _, _) => {
            h1874
        }
    }), f396(match v0.clone() {
        Adt23::C0(_, _, _, h1875, _, _, _, _) => {
            h1875
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, h1876, _, _, _, _, _, _) => {
            h1876
        }
    }), f396(match v0.clone() {
        Adt23::C0(_, _, _, _, h1877, _, _, _) => {
            h1877
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, h1878, _, _, _, _, _) => {
            h1878
        }
    }), f396(match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1879, _, _) => {
            h1879
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, _, h1880, _, _, _, _) => {
            h1880
        }
    }), f396(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1881, _) => {
            h1881
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, _, _, h1882, _, _, _) => {
            h1882
        }
    }), f396(match v0 {
        Adt23::C0(_, _, _, _, _, _, _, h1883) => {
            h1883
        }
    }, match v2 {
        Adt26::C0(_, _, _, _, _, _, _, h1884, _, _) => {
            h1884
        }
    })))
}

pub fn f350(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt25> {
    Ok(Adt25::C0(f397(v0.clone(), v1, v2, v3)?, f397(v0.clone(), v1, v2, {
        let a1885 = v3;
        let a1886 = 4u64;
        nat_add(a1885, a1886)?
    })?, f397(v0.clone(), v1, v2, {
        let a1887 = v3;
        let a1888 = 8u64;
        nat_add(a1887, a1888)?
    })?, f397(v0.clone(), v1, v2, {
        let a1889 = v3;
        let a1890 = 12u64;
        nat_add(a1889, a1890)?
    })?, f397(v0.clone(), v1, v2, {
        let a1891 = v3;
        let a1892 = 16u64;
        nat_add(a1891, a1892)?
    })?, f397(v0.clone(), v1, v2, {
        let a1893 = v3;
        let a1894 = 20u64;
        nat_add(a1893, a1894)?
    })?, f397(v0.clone(), v1, v2, {
        let a1895 = v3;
        let a1896 = 24u64;
        nat_add(a1895, a1896)?
    })?, f397(v0.clone(), v1, v2, {
        let a1897 = v3;
        let a1898 = 28u64;
        nat_add(a1897, a1898)?
    })?, f397(v0.clone(), v1, v2, {
        let a1899 = v3;
        let a1900 = 32u64;
        nat_add(a1899, a1900)?
    })?, f397(v0.clone(), v1, v2, {
        let a1901 = v3;
        let a1902 = 36u64;
        nat_add(a1901, a1902)?
    })?, f397(v0.clone(), v1, v2, {
        let a1903 = v3;
        let a1904 = 40u64;
        nat_add(a1903, a1904)?
    })?, f397(v0.clone(), v1, v2, {
        let a1905 = v3;
        let a1906 = 44u64;
        nat_add(a1905, a1906)?
    })?, f397(v0.clone(), v1, v2, {
        let a1907 = v3;
        let a1908 = 48u64;
        nat_add(a1907, a1908)?
    })?, f397(v0.clone(), v1, v2, {
        let a1909 = v3;
        let a1910 = 52u64;
        nat_add(a1909, a1910)?
    })?, f397(v0.clone(), v1, v2, {
        let a1911 = v3;
        let a1912 = 56u64;
        nat_add(a1911, a1912)?
    })?, f397(v0, v1, v2, {
        let a1913 = v3;
        let a1914 = 60u64;
        nat_add(a1913, a1914)?
    })?))
}

pub fn f351(p1917: List<Bytes>, v1: Bytes) -> bool {
    let m1916 = p1917;
    {
        let mut m1916 = m1916;
        loop {
            let step = {
                let v0 = m1916;
                let m1915 = v0;
                match m1915.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f199(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(v0: Bytes, v1: u64) -> Option<u64> {
    let a1918 = 0u64;
    let a1919 = v1;
    if nat_lt(a1918, a1919) {
        if f398(v0, {
            let a1920 = v1;
            let a1921 = 1u64;
            nat_sub(a1920, a1921)
        }) {
            Some({
                let a1922 = v1;
                let a1923 = 1u64;
                nat_sub(a1922, a1923)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f353(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1924, _) = v1;
        h1924
    };
    let a1926 = {
        let a1925 = v0.clone();
        length_bytes(a1925)
    };
    let a1927 = v2;
    if nat_le(a1926, a1927) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1928 = f220(v0.clone(), v2)?;
        let a1929 = 13u64;
        if nat_eq(a1928, a1929) {
            Ok(Some(({
                let a1932 = f220(v0, {
                    let a1930 = v2;
                    let a1931 = 1u64;
                    nat_add(a1930, a1931)?
                })?;
                let a1933 = 10u64;
                if nat_eq(a1932, a1933) {
                    let a1934 = v2;
                    let a1935 = 2u64;
                    nat_add(a1934, a1935)?
                } else {
                    let a1936 = v2;
                    let a1937 = 1u64;
                    nat_add(a1936, a1937)?
                }
            }, {
                let a1939 = {
                    let (_, h1938) = v1;
                    h1938
                };
                let a1940 = Bytes::lit(&[0x0a]);
                append_bytes(a1939, a1940)
            })))
        } else {
            let v4: u64 = {
                let m1941 = f297(v0.clone(), Bytes::lit(&[0x0d]), v2)?;
                match m1941 {
                    None => {
                        let a1942 = v0.clone();
                        length_bytes(a1942)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1946 = {
                    let (_, h1943) = v1;
                    h1943
                };
                let a1947 = f32(v0, v2, {
                    let a1944 = v4;
                    let a1945 = v2;
                    nat_sub(a1944, a1945)
                });
                append_bytes(a1946, a1947)
            })))
        }
    }
}

pub fn f354(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1948 = f399(v0, v1)?;
    match m1948 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f400({
                let (h1949, _) = v2;
                h1949
            }) {
                Ok(Some({
                    let a1951 = v1;
                    let a1952 = {
                        let (_, h1950) = v2;
                        h1950
                    };
                    nat_add(a1951, a1952)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f355(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f401(v1, v0, v2)
}

pub fn f356(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f402(v0, v1)
}

pub fn f357(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f403(v0, v1, v2)
}

pub fn f358(v0: Bytes, p1955: List<Bytes>) -> bool {
    let m1954 = p1955;
    {
        let mut m1954 = m1954;
        loop {
            let step = {
                let v1 = m1954;
                let m1953 = v1;
                match m1953.uncons() {
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
                Ok(next) => { m1954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f219(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1958 = f220(v0.clone(), {
                let a1956 = v1;
                let a1957 = 2u64;
                nat_add(a1956, a1957)?
            })?;
            let a1959 = 120u64;
            nat_eq(a1958, a1959)
        };
        let v3: u64 = if v2 {
            let a1960 = v1;
            let a1961 = 3u64;
            nat_add(a1960, a1961)?
        } else {
            let a1962 = v1;
            let a1963 = 2u64;
            nat_add(a1962, a1963)?
        };
        let v4: (u64, u64) = f404(v0.clone(), v2, v3)?;
        let a1965 = {
            let (h1964, _) = v4;
            h1964
        };
        let a1966 = v3;
        if if nat_eq(a1965, a1966) {
            true
        } else {
            let a1968 = f220(v0, {
                let (h1967, _) = v4;
                h1967
            })?;
            let a1969 = 59u64;
            !nat_eq(a1968, a1969)
        } {
            Ok(Err::<(Bytes, u64), Bytes>(f216(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f400({
                let (_, h1970) = v4;
                h1970
            }) {
                Ok(Ok::<(Bytes, u64), Bytes>((f405({
                    let (_, h1971) = v4;
                    h1971
                })?, {
                    let a1973 = {
                        let (h1972, _) = v4;
                        h1972
                    };
                    let a1974 = 1u64;
                    nat_add(a1973, a1974)?
                })))
            } else {
                Ok(Err::<(Bytes, u64), Bytes>(f216(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f219(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3c]), {
                let a1975 = v1;
                let a1976 = 4u64;
                nat_add(a1975, a1976)?
            })))
        } else {
            if f219(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3e]), {
                    let a1977 = v1;
                    let a1978 = 4u64;
                    nat_add(a1977, a1978)?
                })))
            } else {
                if f219(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x26]), {
                        let a1979 = v1;
                        let a1980 = 5u64;
                        nat_add(a1979, a1980)?
                    })))
                } else {
                    if f219(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x27]), {
                            let a1981 = v1;
                            let a1982 = 6u64;
                            nat_add(a1981, a1982)?
                        })))
                    } else {
                        if f219(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x22]), {
                                let a1983 = v1;
                                let a1984 = 6u64;
                                nat_add(a1983, a1984)?
                            })))
                        } else {
                            Ok(Err::<(Bytes, u64), Bytes>(f216(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f360(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1985, _) = f244(Fn3::F406(v1, v2, v0.clone()), {
        let a1987 = {
            let a1986 = v0;
            length_bytes(a1986)
        };
        let a1988 = 1u64;
        nat_add(a1987, a1988)?
    }, v3)?;
    Ok(h1985)
}

pub fn f361(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1989, _) = v2;
        h1989
    };
    let a1990 = v3;
    let a1991 = 0u64;
    if nat_eq(a1990, a1991) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1992 = v3;
            let a1993 = v1;
            let a1994 = 0u64;
            nat_quot(a1992, a1993, a1994)
        }, {
            let a1999 = f306(v0, {
                let a1995 = v3;
                let a1996 = v1;
                let a1997 = 0u64;
                nat_rem(a1995, a1996, a1997)
            });
            let a2000 = {
                let (_, h1998) = v2;
                h1998
            };
            append_bytes(a1999, a2000)
        }))
    }
}

pub fn f362(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a2001 = v2;
    let a2002 = v0;
    if nat_lt(a2001, a2002) {
        if f398(v1, v2) {
            Ok(Some({
                let a2003 = v2;
                let a2004 = 1u64;
                nat_add(a2003, a2004)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f363(v0: Bytes, p2084: u64, p2085: u64, p2086: bool, p2087: bool, p2088: Bytes) -> R<Bytes> {
    let m2083 = (p2084, (p2085, (p2086, (p2087, p2088))));
    {
        let mut m2083 = m2083;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m2083;
                let m2005 = v2;
                if m2005 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m2005 - 1;
                    let m2008 = {
                        let a2006 = v0.clone();
                        let a2007 = v1;
                        index_bytes(a2006, a2007)
                    };
                    match m2008 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a2053 = {
                                    let a2029 = {
                                        let a2017 = {
                                            let a2011 = {
                                                let a2009 = v8;
                                                let a2010 = 128u8;
                                                fixed_u8::bit_and(a2009, a2010)
                                            };
                                            let a2012 = 128u8;
                                            if equal(a2011, a2012) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2018 = {
                                            let a2015 = {
                                                let a2013 = v8;
                                                let a2014 = 64u8;
                                                fixed_u8::bit_and(a2013, a2014)
                                            };
                                            let a2016 = 64u8;
                                            if equal(a2015, a2016) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2017, a2018)?
                                    };
                                    let a2030 = {
                                        let a2027 = {
                                            let a2021 = {
                                                let a2019 = v8;
                                                let a2020 = 32u8;
                                                fixed_u8::bit_and(a2019, a2020)
                                            };
                                            let a2022 = 32u8;
                                            if equal(a2021, a2022) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2028 = {
                                            let a2025 = {
                                                let a2023 = v8;
                                                let a2024 = 16u8;
                                                fixed_u8::bit_and(a2023, a2024)
                                            };
                                            let a2026 = 16u8;
                                            if equal(a2025, a2026) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2027, a2028)?
                                    };
                                    nat_add(a2029, a2030)?
                                };
                                let a2054 = {
                                    let a2051 = {
                                        let a2039 = {
                                            let a2033 = {
                                                let a2031 = v8;
                                                let a2032 = 8u8;
                                                fixed_u8::bit_and(a2031, a2032)
                                            };
                                            let a2034 = 8u8;
                                            if equal(a2033, a2034) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2040 = {
                                            let a2037 = {
                                                let a2035 = v8;
                                                let a2036 = 4u8;
                                                fixed_u8::bit_and(a2035, a2036)
                                            };
                                            let a2038 = 4u8;
                                            if equal(a2037, a2038) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2039, a2040)?
                                    };
                                    let a2052 = {
                                        let a2049 = {
                                            let a2043 = {
                                                let a2041 = v8;
                                                let a2042 = 2u8;
                                                fixed_u8::bit_and(a2041, a2042)
                                            };
                                            let a2044 = 2u8;
                                            if equal(a2043, a2044) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2050 = {
                                            let a2047 = {
                                                let a2045 = v8;
                                                let a2046 = 1u8;
                                                fixed_u8::bit_and(a2045, a2046)
                                            };
                                            let a2048 = 1u8;
                                            if equal(a2047, a2048) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2049, a2050)?
                                    };
                                    nat_add(a2051, a2052)?
                                };
                                nat_add(a2053, a2054)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a2055 = v1;
                                    let a2056 = 1u64;
                                    nat_add(a2055, a2056)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a2057 = v9;
                                    let a2058 = 34u64;
                                    !nat_eq(a2057, a2058)
                                }, (if v4 {
                                    false
                                } else {
                                    let a2059 = v9;
                                    let a2060 = 92u64;
                                    nat_eq(a2059, a2060)
                                }, {
                                    let a2061 = v5;
                                    let a2062 = v10;
                                    append_bytes(a2061, a2062)
                                })))))
                            } else {
                                let a2063 = v9;
                                let a2064 = 32u64;
                                if if if if nat_eq(a2063, a2064) {
                                    true
                                } else {
                                    let a2065 = v9;
                                    let a2066 = 9u64;
                                    nat_eq(a2065, a2066)
                                } {
                                    true
                                } else {
                                    let a2067 = v9;
                                    let a2068 = 10u64;
                                    nat_eq(a2067, a2068)
                                } {
                                    true
                                } else {
                                    let a2069 = v9;
                                    let a2070 = 13u64;
                                    nat_eq(a2069, a2070)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a2071 = v1;
                                        let a2072 = 1u64;
                                        nat_add(a2071, a2072)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2073 = v1;
                                            let a2074 = 4u64;
                                            nat_add(a2073, a2074)?
                                        }, (v6, (false, (false, {
                                            let a2075 = v5;
                                            let a2076 = f8(Str::lit("[]"));
                                            append_bytes(a2075, a2076)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2077 = v1;
                                            let a2078 = 1u64;
                                            nat_add(a2077, a2078)?
                                        }, (v6, ({
                                            let a2079 = v9;
                                            let a2080 = 34u64;
                                            nat_eq(a2079, a2080)
                                        }, (false, {
                                            let a2081 = v5;
                                            let a2082 = v10;
                                            append_bytes(a2081, a2082)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f407(Fn15::F408(v0.clone()), {
        let a2090 = {
            let a2089 = v0;
            length_bytes(a2089)
        };
        let a2091 = 1u64;
        nat_add(a2090, a2091)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a2095 = {
        let (_, h2092) = {
            let (_, h2093) = {
                let (h2094, _) = v2.clone();
                h2094
            };
            h2093
        };
        h2092
    };
    let a2096 = 1u64;
    if nat_eq(a2095, a2096) {
        Ok(Some(({
            let (h2097, _) = {
                let (h2098, _) = v2.clone();
                h2098
            };
            h2097
        }, {
            let (h2099, _) = {
                let (_, h2100) = {
                    let (h2101, _) = v2;
                    h2101
                };
                h2100
            };
            h2099
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f365(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2102 = f302(v0.clone(), v1)?;
        let a2103 = 45u64;
        nat_eq(a2102, a2103)
    };
    let v3: u64 = if v2 {
        let a2104 = v1;
        let a2105 = 1u64;
        nat_add(a2104, a2105)?
    } else {
        v1
    };
    let v4: u64 = f409(v0.clone(), v3)?;
    let v5: u64 = {
        let a2106 = v4;
        let a2107 = v3;
        nat_sub(a2106, a2107)
    };
    let v6: bool = {
        let a2108 = f302(v0.clone(), v4)?;
        let a2109 = 46u64;
        nat_eq(a2108, a2109)
    };
    let v7: u64 = if v6 {
        let a2110 = v4;
        let a2111 = 1u64;
        nat_add(a2110, a2111)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f409(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f302(v0.clone(), v8)?;
    let v10: bool = {
        let a2112 = v9;
        let a2113 = 101u64;
        if nat_eq(a2112, a2113) {
            true
        } else {
            let a2114 = v9;
            let a2115 = 69u64;
            nat_eq(a2114, a2115)
        }
    };
    let v11: u64 = f302(v0.clone(), {
        let a2116 = v8;
        let a2117 = 1u64;
        nat_add(a2116, a2117)?
    })?;
    let v12: bool = if v10 {
        let a2118 = v11;
        let a2119 = 43u64;
        if nat_eq(a2118, a2119) {
            true
        } else {
            let a2120 = v11;
            let a2121 = 45u64;
            nat_eq(a2120, a2121)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2122 = v8;
            let a2123 = 2u64;
            nat_add(a2122, a2123)?
        } else {
            let a2124 = v8;
            let a2125 = 1u64;
            nat_add(a2124, a2125)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f409(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2126 = v14;
        let a2127 = v13;
        nat_sub(a2126, a2127)
    });
    let v16: u64 = f410(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2129 = {
            let a2128 = v15;
            length_bytes(a2128)
        };
        let a2130 = v16;
        nat_sub(a2129, a2130)
    });
    let a2131 = v5;
    let a2132 = 0u64;
    if if nat_eq(a2131, a2132) {
        true
    } else {
        let a2133 = 1u64;
        let a2134 = v5;
        if if nat_lt(a2133, a2134) {
            let a2135 = f302(v0.clone(), v3)?;
            let a2136 = 48u64;
            nat_eq(a2135, a2136)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2137 = v8;
                let a2138 = v7;
                nat_eq(a2137, a2138)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2139 = v14;
                    let a2140 = v13;
                    nat_eq(a2139, a2140)
                } else {
                    false
                } {
                    true
                } else {
                    let a2142 = 18u64;
                    let a2143 = {
                        let a2141 = v17.clone();
                        length_bytes(a2141)
                    };
                    nat_lt(a2142, a2143)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2147 = {
            let a2145 = {
                let a2144 = v17.clone();
                length_bytes(a2144)
            };
            let a2146 = 0u64;
            if nat_eq(a2145, a2146) {
                Some(0u64)
            } else {
                f411(v17)?
            }
        };
        match m2147 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2152 = f412(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2148 = v8;
                    let a2149 = v7;
                    nat_sub(a2148, a2149)
                }), if v12 {
                    let a2150 = v11;
                    let a2151 = 45u64;
                    nat_eq(a2150, a2151)
                } else {
                    false
                }, v18)?;
                match m2152 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2154 = 19u64;
                        let a2155 = {
                            let a2153 = v19.clone();
                            length_bytes(a2153)
                        };
                        if nat_lt(a2154, a2155) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2156 = f411(v19)?;
                            match m2156 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2157 = 0u64;
                                        let a2158 = v20;
                                        nat_lt(a2157, a2158)
                                    } else {
                                        false
                                    };
                                    if f413(v21, v20) {
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

pub fn f366(v0: u64) -> bool {
    let a2159 = v0;
    let a2160 = 95u64;
    if nat_eq(a2159, a2160) {
        true
    } else {
        let a2161 = v0;
        let a2162 = 45u64;
        if nat_eq(a2161, a2162) {
            true
        } else {
            let a2163 = v0;
            let a2164 = 32u64;
            if nat_eq(a2163, a2164) {
                true
            } else {
                let a2165 = v0;
                let a2166 = 46u64;
                if nat_eq(a2165, a2166) {
                    true
                } else {
                    let a2167 = v0;
                    let a2168 = 47u64;
                    nat_eq(a2167, a2168)
                }
            }
        }
    }
}

pub fn f367(v0: u64) -> bool {
    let a2169 = 97u64;
    let a2170 = v0;
    if nat_le(a2169, a2170) {
        let a2171 = v0;
        let a2172 = 122u64;
        nat_le(a2171, a2172)
    } else {
        false
    }
}

pub fn f368(v0: u64) -> bool {
    let a2173 = 65u64;
    let a2174 = v0;
    if nat_le(a2173, a2174) {
        let a2175 = v0;
        let a2176 = 90u64;
        nat_le(a2175, a2176)
    } else {
        false
    }
}

pub fn f369(v0: u64) -> bool {
    let a2177 = 48u64;
    let a2178 = v0;
    if nat_le(a2177, a2178) {
        let a2179 = v0;
        let a2180 = 57u64;
        nat_le(a2179, a2180)
    } else {
        false
    }
}

pub fn f370(v0: List<(Bytes, Adt14)>, v1: List<(Bytes, Adt14)>) -> List<(Bytes, Adt14)> {
    let m2181 = v0;
    match m2181.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f370(v3, v1))
        }
    }
}

pub fn f371(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m2182 = v0;
    match m2182 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f414(v1)
        }
    }
}

pub fn f372(v0: u64, v1: List<(Bytes, Adt14)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt14)>, Bytes>> {
    let m2183 = v0;
    if m2183 == 0 {
        let m2184 = v1;
        match m2184.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt14)>, Bytes>(List::<(Bytes, Adt14)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt14)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2183 - 1;
        let m2185 = v1.clone();
        match m2185.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt14)>, Bytes>(List::<(Bytes, Adt14)>::nil()))
            }
            Some((_, _)) => {
                let m2186 = f415(v1.clone(), v2.clone(), v3.clone())?;
                match m2186 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt14)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f209(f373(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2189 = f372(v6, f416(v1, {
                            let (h2187, _) = v9.clone();
                            h2187
                        }), List::cons(f234({
                            let (h2188, _) = v9.clone();
                            h2188
                        })?, v2), v3)?;
                        match m2189 {
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

pub fn f373(v0: List<(Bytes, Adt14)>) -> R<List<Bytes>> {
    let m2190 = v0;
    match m2190.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f234({
                let (h2191, _) = v1;
                h2191
            })?, f373(v2)?))
        }
    }
}

pub fn f374(p2199: u64, p2200: List<Adt24>, p2201: List<Bytes>, p2202: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2198 = (p2199, (p2200, (p2201, p2202)));
    {
        let mut m2198 = m2198;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2198;
                let m2192 = v0;
                if m2192 == 0 {
                    Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2192 - 1;
                    let m2193 = v1;
                    match m2193.uncons() {
                        None => {
                            Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2194 = v5;
                            match m2194 {
                                Adt24::C0(v8, v9) => {
                                    if f36(v8.clone(), f417()) {
                                        Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f86(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2195 = f419(v8.clone(), v9)?;
                                            match m2195 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f375({
                                                        let (_, h2196) = v11;
                                                        h2196
                                                    }, List::cons(Adt24::C1({
                                                        let (h2197, _) = v11;
                                                        h2197
                                                    }), v6)), (List::cons(v8, v2), v3))))
                                                }
                                                Err(v10) => {
                                                    Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt24::C1(v7) => {
                                    Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                                Adt24::C2 => {
                                    if f86(f417(), v2.clone()) {
                                        Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f417(), v2), List::cons(f418(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f375(v0: List<Adt24>, v1: List<Adt24>) -> List<Adt24> {
    let m2203 = v0;
    match m2203.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f375(v3, v1))
        }
    }
}

pub fn f376(v0: List<(Bytes, Adt14)>) -> R<List<Adt24>> {
    let m2204 = v0;
    match m2204.uncons() {
        None => {
            Ok(List::<Adt24>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt24::C0(f234({
                let (h2205, _) = v1;
                h2205
            })?, {
                let (_, h2206) = v1;
                h2206
            }), f376(v2)?))
        }
    }
}

pub fn f377(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2207 = v0;
    match m2207.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f36(f150(v3.clone(), v1.clone()), v2.clone()) {
                f377(v4, v1, v2)
            } else {
                List::cons(v3, f377(v4, v1, v2))
            }
        }
    }
}

pub fn f378(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2208 = v0;
    match m2208.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f199(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f209(f385(f78(List::cons(v1, f420(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f378(v3, v1)?))
            }
        }
    }
}

pub fn f379(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f421(v0, v1)
}

pub fn f380(v0: Fn10, p2212: List<(Bytes, Bytes)>, p2213: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2211 = (p2212, p2213);
    {
        let mut m2211 = m2211;
        loop {
            let step = {
                let (v1, v2) = m2211;
                let m2209 = v2;
                match m2209.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2210: Fn10 = v0.clone();
                            c2210.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2211 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f381(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f422(v0, v1)
}

pub fn f382(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f423(v0)
}

pub fn f383(v0: Fn11, p2217: List<Bytes>, p2218: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2216 = (p2217, p2218);
    {
        let mut m2216 = m2216;
        loop {
            let step = {
                let (v1, v2) = m2216;
                let m2214 = v2;
                match m2214.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2215: Fn11 = v0.clone();
                            c2215.apply(v1, v3)?
                        }, v4))
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

pub fn f384(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f424(v1)?, v0))
}

pub fn f385(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2219 = v0;
    match m2219.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f204(v1)?, f385(v2)?))
        }
    }
}

pub fn f386(v0: Fn12, p2223: List<List<(Bytes, ())>>, p2224: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2222 = (p2223, p2224);
    {
        let mut m2222 = m2222;
        loop {
            let step = {
                let (v1, v2) = m2222;
                let m2220 = v2;
                match m2220.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2221: Fn12 = v0.clone();
                            c2221.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f388(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2226 = {
        let a2225 = v0.clone();
        length_list(a2225)
    };
    let a2227 = 1u64;
    if nat_le(a2226, a2227) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f425(v0)?))
    }
}

pub fn f389(v0: Fn13, p2231: List<(Bytes, ())>, p2232: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2230 = (p2231, p2232);
    {
        let mut m2230 = m2230;
        loop {
            let step = {
                let (v1, v2) = m2230;
                let m2228 = v2;
                match m2228.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2229: Fn13 = v0.clone();
                            c2229.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2230 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f391(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f72(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f426(v3.clone(), f72(f73(v0, f89())?, f8(Str::lit("at"))));
    let a2234 = {
        let a2233 = v4.clone();
        length_list(a2233)
    };
    let a2235 = 0u64;
    if nat_eq(a2234, a2235) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2238 = {
            let a2236 = v4;
            length_list(a2236)
        };
        let a2239 = {
            let a2237 = v3;
            length_list(a2237)
        };
        if nat_eq(a2238, a2239) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f427(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f392(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f230(f140(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f230(f140(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f64(f72(v3.clone(), f8(Str::lit("id"))), f72(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f72(f73(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f426(v5.clone(), f64(v6.clone(), f72(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2241 = {
        let a2240 = v7.clone();
        length_list(a2240)
    };
    let a2242 = 0u64;
    if nat_eq(a2241, a2242) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f75({
            let a2245 = {
                let a2243 = v7;
                length_list(a2243)
            };
            let a2246 = {
                let a2244 = v5;
                length_list(a2244)
            };
            nat_eq(a2245, a2246)
        }) {
            Ok((false, f427(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f78(f64(f426(f428(v4.clone()), f69(f70(v0, f8(Str::lit("model/authorities.toml"))))?), f426(f429(v4), f64(v6, f72(v3, f8(Str::lit("id")))))))?;
            let a2248 = {
                let a2247 = v8.clone();
                length_list(a2247)
            };
            let a2249 = 0u64;
            if nat_eq(a2248, a2249) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f427(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f121(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f393(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h2250, _) = v0;
        h2250
    }, {
        let (h2251, _) = v1;
        h2251
    }) {
        f36({
            let (_, h2252) = v0;
            h2252
        }, {
            let (_, h2253) = v1;
            h2253
        })
    } else {
        false
    }
}

pub fn f394(v0: Fn14, p2257: u64, p2258: Adt26) -> R<Adt26> {
    let m2256 = (p2257, p2258);
    {
        let mut m2256 = m2256;
        loop {
            let step = {
                let (v1, v2) = m2256;
                let m2254 = v1;
                if m2254 == 0 {
                    Err::<(u64, Adt26), R<Adt26>>(Ok(v2))
                } else {
                    let v3 = m2254 - 1;
                    Ok::<(u64, Adt26), R<Adt26>>((v3, {
                        let c2255: Fn14 = v0.clone();
                        c2255.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2256 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: Adt26) -> R<Adt26> {
    f430(v0)
}

pub fn f396(v0: u32, v1: u32) -> u32 {
    let m2261 = {
        let a2259 = f431(v0);
        let a2260 = f431(v1);
        fixed_u64::checked_add(a2259, a2260)
    };
    match m2261 {
        None => {
            0u32
        }
        Some(v2) => {
            f432(v2)
        }
    }
}

pub fn f397(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2264 = {
        let a2262 = v3;
        let a2263 = 8u64;
        nat_add(a2262, a2263)?
    };
    let a2265 = v2;
    if nat_eq(a2264, a2265) {
        Ok(f433({
            let a2268 = {
                let a2266 = v1;
                let a2267 = 8u64;
                nat_mul(a2266, a2267)?
            };
            let a2269 = 4294967296u64;
            let a2270 = 0u64;
            nat_quot(a2268, a2269, a2270)
        }))
    } else {
        let a2273 = {
            let a2271 = v3;
            let a2272 = 4u64;
            nat_add(a2271, a2272)?
        };
        let a2274 = v2;
        if nat_eq(a2273, a2274) {
            Ok(f433({
                let a2277 = {
                    let a2275 = v1;
                    let a2276 = 8u64;
                    nat_mul(a2275, a2276)?
                };
                let a2278 = 4294967296u64;
                let a2279 = 0u64;
                nat_rem(a2277, a2278, a2279)
            }))
        } else {
            Ok(f434(f435(v0.clone(), v1, v3), f435(v0.clone(), v1, {
                let a2280 = v3;
                let a2281 = 1u64;
                nat_add(a2280, a2281)?
            }), f435(v0.clone(), v1, {
                let a2282 = v3;
                let a2283 = 2u64;
                nat_add(a2282, a2283)?
            }), f435(v0, v1, {
                let a2284 = v3;
                let a2285 = 3u64;
                nat_add(a2284, a2285)?
            })))
        }
    }
}

pub fn f398(v0: Bytes, v1: u64) -> bool {
    let m2286 = f242(v0, v1);
    match m2286 {
        None => {
            false
        }
        Some(v2) => {
            f436(v2)
        }
    }
}

pub fn f399(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2289 = {
        let a2287 = v0.clone();
        let a2288 = v1;
        index_bytes(a2287, a2288)
    };
    match m2289 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2334 = {
                    let a2310 = {
                        let a2298 = {
                            let a2292 = {
                                let a2290 = v3;
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
                                let a2294 = v3;
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
                                let a2300 = v3;
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
                                let a2304 = v3;
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
                                let a2312 = v3;
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
                                let a2316 = v3;
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
                                let a2322 = v3;
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
                                let a2326 = v3;
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
            let a2336 = v4;
            let a2337 = 128u64;
            if nat_lt(a2336, a2337) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2338 = 194u64;
                let a2339 = v4;
                if if nat_le(a2338, a2339) {
                    let a2340 = v4;
                    let a2341 = 223u64;
                    nat_le(a2340, a2341)
                } else {
                    false
                } {
                    let m2344 = f437(v0, {
                        let a2342 = v1;
                        let a2343 = 1u64;
                        nat_add(a2342, a2343)?
                    })?;
                    match m2344 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f438({
                                let a2349 = {
                                    let a2347 = {
                                        let a2345 = v4;
                                        let a2346 = 192u64;
                                        nat_sub(a2345, a2346)
                                    };
                                    let a2348 = 64u64;
                                    nat_mul(a2347, a2348)?
                                };
                                let a2350 = v5;
                                nat_add(a2349, a2350)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2351 = 224u64;
                    let a2352 = v4;
                    if if nat_le(a2351, a2352) {
                        let a2353 = v4;
                        let a2354 = 239u64;
                        nat_le(a2353, a2354)
                    } else {
                        false
                    } {
                        let m2357 = f437(v0.clone(), {
                            let a2355 = v1;
                            let a2356 = 1u64;
                            nat_add(a2355, a2356)?
                        })?;
                        match m2357 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2360 = f437(v0, {
                                    let a2358 = v1;
                                    let a2359 = 2u64;
                                    nat_add(a2358, a2359)?
                                })?;
                                match m2360 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f438({
                                            let a2369 = {
                                                let a2363 = {
                                                    let a2361 = v4;
                                                    let a2362 = 224u64;
                                                    nat_sub(a2361, a2362)
                                                };
                                                let a2364 = 4096u64;
                                                nat_mul(a2363, a2364)?
                                            };
                                            let a2370 = {
                                                let a2367 = {
                                                    let a2365 = v6;
                                                    let a2366 = 64u64;
                                                    nat_mul(a2365, a2366)?
                                                };
                                                let a2368 = v7;
                                                nat_add(a2367, a2368)?
                                            };
                                            nat_add(a2369, a2370)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2371 = 240u64;
                        let a2372 = v4;
                        if if nat_le(a2371, a2372) {
                            let a2373 = v4;
                            let a2374 = 244u64;
                            nat_le(a2373, a2374)
                        } else {
                            false
                        } {
                            let m2377 = f437(v0.clone(), {
                                let a2375 = v1;
                                let a2376 = 1u64;
                                nat_add(a2375, a2376)?
                            })?;
                            match m2377 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2380 = f437(v0.clone(), {
                                        let a2378 = v1;
                                        let a2379 = 2u64;
                                        nat_add(a2378, a2379)?
                                    })?;
                                    match m2380 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m2383 = f437(v0, {
                                                let a2381 = v1;
                                                let a2382 = 3u64;
                                                nat_add(a2381, a2382)?
                                            })?;
                                            match m2383 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f438({
                                                        let a2396 = {
                                                            let a2386 = {
                                                                let a2384 = v4;
                                                                let a2385 = 240u64;
                                                                nat_sub(a2384, a2385)
                                                            };
                                                            let a2387 = 262144u64;
                                                            nat_mul(a2386, a2387)?
                                                        };
                                                        let a2397 = {
                                                            let a2394 = {
                                                                let a2388 = v8;
                                                                let a2389 = 4096u64;
                                                                nat_mul(a2388, a2389)?
                                                            };
                                                            let a2395 = {
                                                                let a2392 = {
                                                                    let a2390 = v9;
                                                                    let a2391 = 64u64;
                                                                    nat_mul(a2390, a2391)?
                                                                };
                                                                let a2393 = v10;
                                                                nat_add(a2392, a2393)?
                                                            };
                                                            nat_add(a2394, a2395)?
                                                        };
                                                        nat_add(a2396, a2397)?
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

pub fn f400(v0: u64) -> bool {
    let a2398 = v0;
    let a2399 = 9u64;
    if if if nat_eq(a2398, a2399) {
        true
    } else {
        let a2400 = v0;
        let a2401 = 10u64;
        nat_eq(a2400, a2401)
    } {
        true
    } else {
        let a2402 = v0;
        let a2403 = 13u64;
        nat_eq(a2402, a2403)
    } {
        true
    } else {
        let a2404 = 32u64;
        let a2405 = v0;
        if if if nat_le(a2404, a2405) {
            let a2406 = v0;
            let a2407 = 55295u64;
            nat_le(a2406, a2407)
        } else {
            false
        } {
            true
        } else {
            let a2408 = 57344u64;
            let a2409 = v0;
            if nat_le(a2408, a2409) {
                let a2410 = v0;
                let a2411 = 65533u64;
                nat_le(a2410, a2411)
            } else {
                false
            }
        } {
            true
        } else {
            let a2412 = 65536u64;
            let a2413 = v0;
            if nat_le(a2412, a2413) {
                let a2414 = v0;
                let a2415 = 1114111u64;
                nat_le(a2414, a2415)
            } else {
                false
            }
        }
    }
}

pub fn f401(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f213(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2417 = v2;
        let a2418 = {
            let a2416 = v0;
            length_bytes(a2416)
        };
        if nat_lt(a2417, a2418) {
            Ok(Some({
                let a2419 = v2;
                let a2420 = 1u64;
                nat_add(a2419, a2420)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f402(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f296(f220(v0, v1)?) {
        Ok(Some({
            let a2421 = v1;
            let a2422 = 1u64;
            nat_add(a2421, a2422)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f403(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2423 = f399(v0, v2)?;
    match m2423 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2424 = v2;
            let a2425 = v1;
            if if nat_eq(a2424, a2425) {
                f439({
                    let (h2426, _) = v3;
                    h2426
                })
            } else {
                f440({
                    let (h2427, _) = v3;
                    h2427
                })
            } {
                Ok(Some({
                    let a2429 = v2;
                    let a2430 = {
                        let (_, h2428) = v3;
                        h2428
                    };
                    nat_add(a2429, a2430)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f404(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2431, _) = f441(Fn16::F442(v1, v0.clone()), {
        let a2433 = {
            let a2432 = v0;
            length_bytes(a2432)
        };
        let a2434 = 1u64;
        nat_add(a2433, a2434)?
    }, (v2, 0u64))?;
    Ok(h2431)
}

pub fn f405(v0: u64) -> R<Bytes> {
    let a2435 = v0;
    let a2436 = 128u64;
    if nat_lt(a2435, a2436) {
        Ok(f443(v0))
    } else {
        let a2437 = v0;
        let a2438 = 2048u64;
        if nat_lt(a2437, a2438) {
            let a2449 = f443({
                let a2442 = 192u64;
                let a2443 = {
                    let a2439 = v0;
                    let a2440 = 64u64;
                    let a2441 = 0u64;
                    nat_quot(a2439, a2440, a2441)
                };
                nat_add(a2442, a2443)?
            });
            let a2450 = f443({
                let a2447 = 128u64;
                let a2448 = {
                    let a2444 = v0;
                    let a2445 = 64u64;
                    let a2446 = 0u64;
                    nat_rem(a2444, a2445, a2446)
                };
                nat_add(a2447, a2448)?
            });
            Ok(append_bytes(a2449, a2450))
        } else {
            let a2451 = v0;
            let a2452 = 65536u64;
            if nat_lt(a2451, a2452) {
                let a2473 = f443({
                    let a2456 = 224u64;
                    let a2457 = {
                        let a2453 = v0;
                        let a2454 = 4096u64;
                        let a2455 = 0u64;
                        nat_quot(a2453, a2454, a2455)
                    };
                    nat_add(a2456, a2457)?
                });
                let a2474 = {
                    let a2471 = f443({
                        let a2464 = 128u64;
                        let a2465 = {
                            let a2461 = {
                                let a2458 = v0;
                                let a2459 = 64u64;
                                let a2460 = 0u64;
                                nat_quot(a2458, a2459, a2460)
                            };
                            let a2462 = 64u64;
                            let a2463 = 0u64;
                            nat_rem(a2461, a2462, a2463)
                        };
                        nat_add(a2464, a2465)?
                    });
                    let a2472 = f443({
                        let a2469 = 128u64;
                        let a2470 = {
                            let a2466 = v0;
                            let a2467 = 64u64;
                            let a2468 = 0u64;
                            nat_rem(a2466, a2467, a2468)
                        };
                        nat_add(a2469, a2470)?
                    });
                    append_bytes(a2471, a2472)
                };
                Ok(append_bytes(a2473, a2474))
            } else {
                let a2505 = f443({
                    let a2478 = 240u64;
                    let a2479 = {
                        let a2475 = v0;
                        let a2476 = 262144u64;
                        let a2477 = 0u64;
                        nat_quot(a2475, a2476, a2477)
                    };
                    nat_add(a2478, a2479)?
                });
                let a2506 = {
                    let a2503 = f443({
                        let a2486 = 128u64;
                        let a2487 = {
                            let a2483 = {
                                let a2480 = v0;
                                let a2481 = 4096u64;
                                let a2482 = 0u64;
                                nat_quot(a2480, a2481, a2482)
                            };
                            let a2484 = 64u64;
                            let a2485 = 0u64;
                            nat_rem(a2483, a2484, a2485)
                        };
                        nat_add(a2486, a2487)?
                    });
                    let a2504 = {
                        let a2501 = f443({
                            let a2494 = 128u64;
                            let a2495 = {
                                let a2491 = {
                                    let a2488 = v0;
                                    let a2489 = 64u64;
                                    let a2490 = 0u64;
                                    nat_quot(a2488, a2489, a2490)
                                };
                                let a2492 = 64u64;
                                let a2493 = 0u64;
                                nat_rem(a2491, a2492, a2493)
                            };
                            nat_add(a2494, a2495)?
                        });
                        let a2502 = f443({
                            let a2499 = 128u64;
                            let a2500 = {
                                let a2496 = v0;
                                let a2497 = 64u64;
                                let a2498 = 0u64;
                                nat_rem(a2496, a2497, a2498)
                            };
                            nat_add(a2499, a2500)?
                        });
                        append_bytes(a2501, a2502)
                    };
                    append_bytes(a2503, a2504)
                };
                Ok(append_bytes(a2505, a2506))
            }
        }
    }
}

pub fn f406(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f444(v2, v0, v1, v3)
}

pub fn f407(v0: Fn15, p2511: u64, p2512: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2510 = (p2511, p2512);
    {
        let mut m2510 = m2510;
        loop {
            let step = {
                let (v1, v2) = m2510;
                let m2507 = v1;
                if m2507 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2507 - 1;
                    let m2509 = {
                        let c2508: Fn15 = v0.clone();
                        c2508.apply(v2.clone())?
                    };
                    match m2509 {
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
                Ok(next) => { m2510 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f445(v0, v1)
}

pub fn f409(v0: Bytes, v1: u64) -> R<u64> {
    let (h2513, _) = f244(Fn3::F446(v0.clone()), {
        let a2515 = {
            let a2514 = v0;
            length_bytes(a2514)
        };
        let a2516 = 1u64;
        nat_add(a2515, a2516)?
    }, v1)?;
    Ok(h2513)
}

pub fn f410(v0: Bytes) -> R<u64> {
    let (h2517, _) = f244(Fn3::F447(v0.clone()), {
        let a2519 = {
            let a2518 = v0;
            length_bytes(a2518)
        };
        let a2520 = 1u64;
        nat_add(a2519, a2520)?
    }, 0u64)?;
    Ok(h2517)
}

pub fn f411(v0: Bytes) -> R<Option<u64>> {
    if f448(v0.clone())? {
        Ok(Some({
            let (_, h2521) = {
                let (h2522, _) = f441(Fn16::F449(v0.clone()), {
                    let a2523 = v0;
                    length_bytes(a2523)
                }, (0u64, 0u64))?;
                h2522
            };
            h2521
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f412(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<Bytes>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2525 = {
            let a2524 = v1.clone();
            length_bytes(a2524)
        };
        let a2526 = f450(v1)?;
        nat_sub(a2525, a2526)
    });
    let v5: Bytes = {
        let a2527 = v0;
        let a2528 = v4.clone();
        append_bytes(a2527, a2528)
    };
    let v6: u64 = f410(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2530 = {
            let a2529 = v5;
            length_bytes(a2529)
        };
        let a2531 = v6;
        nat_sub(a2530, a2531)
    });
    let v8: u64 = {
        let a2532 = v4;
        length_bytes(a2532)
    };
    let v9: u64 = {
        let a2533 = v7.clone();
        length_bytes(a2533)
    };
    let a2534 = v9;
    let a2535 = 0u64;
    if nat_eq(a2534, a2535) {
        Ok(Some(f8(Str::lit("0"))))
    } else {
        if if v2 {
            true
        } else {
            let a2536 = v3;
            let a2537 = v8;
            nat_lt(a2536, a2537)
        } {
            let v10: u64 = if v2 {
                let a2538 = v3;
                let a2539 = v8;
                nat_add(a2538, a2539)?
            } else {
                let a2540 = v8;
                let a2541 = v3;
                nat_sub(a2540, a2541)
            };
            let a2542 = v10;
            let a2543 = f450(v7.clone())?;
            if nat_le(a2542, a2543) {
                Ok(Some(f32(v7, 0u64, {
                    let a2544 = v9;
                    let a2545 = v10;
                    nat_sub(a2544, a2545)
                })))
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            let v11: u64 = {
                let a2546 = v3;
                let a2547 = v8;
                nat_sub(a2546, a2547)
            };
            let a2550 = {
                let a2548 = v9;
                let a2549 = v11;
                nat_add(a2548, a2549)?
            };
            let a2551 = 19u64;
            if nat_le(a2550, a2551) {
                Ok(Some({
                    let a2552 = v7;
                    let a2553 = f451(v11);
                    append_bytes(a2552, a2553)
                }))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f413(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2554 = 0u64;
        let a2555 = v1;
        if nat_lt(a2554, a2555) {
            let a2556 = v1;
            let a2557 = 9223372036854775808u64;
            nat_le(a2556, a2557)
        } else {
            false
        }
    } else {
        let a2558 = v1;
        let a2559 = 9223372036854775807u64;
        nat_le(a2558, a2559)
    }
}

pub fn f414(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m2560 = v0;
    match m2560 {
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
        Adt14::C4(r2561) => {
            let v4 = (*r2561).clone();
            f414(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r2562, r2563) => {
            let v2 = (*r2562).clone();
            let v3 = (*r2563).clone();
            List::cons((v1, v2), f414(v3))
        }
    }
}

pub fn f415(p2568: List<(Bytes, Adt14)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt14)>> {
    let m2567 = p2568;
    {
        let mut m2567 = m2567;
        loop {
            let step = {
                let v0 = m2567;
                let m2564 = v0;
                match m2564.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, R<Option<(Bytes, Adt14)>>>(Ok(None::<(Bytes, Adt14)>))
                    }
                    Some((v3, v4)) => {
                        if f452(f453({
                            let (_, h2565) = v3.clone();
                            h2565
                        }, List::<Bytes>::nil())?, f234({
                            let (h2566, _) = v3.clone();
                            h2566
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt14)>, R<Option<(Bytes, Adt14)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, R<Option<(Bytes, Adt14)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(v0: List<(Bytes, Adt14)>, v1: Bytes) -> List<(Bytes, Adt14)> {
    let m2569 = v0;
    match m2569.uncons() {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h2570, _) = v2.clone();
                h2570
            }, v1.clone()) {
                f416(v3, v1)
            } else {
                List::cons(v2, f416(v3, v1))
            }
        }
    }
}

pub fn f417() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f418() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f419(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt24>), Bytes>> {
    f454(v0, f455(v1))
}

pub fn f420(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2571 = v0;
    match m2571.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f420(v3, false))
            } else {
                f420(v3, true)
            }
        }
    }
}

pub fn f421(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2575 = {
        let a2573 = v0.clone();
        let a2574 = {
            let (h2572, _) = v1.clone();
            h2572
        };
        index_bytes(a2573, a2574)
    };
    match m2575 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2577 = {
                    let (h2576, _) = v1.clone();
                    h2576
                };
                let a2578 = 1u64;
                nat_add(a2577, a2578)?
            }, {
                let a2581 = {
                    let (_, h2579) = v1;
                    h2579
                };
                let a2582 = f456(v0, {
                    let (h2580, _) = v1;
                    h2580
                }, v2)?;
                append_bytes(a2581, a2582)
            })))
        }
    }
}

pub fn f422(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2583 = v0.clone();
    match m2583.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2586 = {
                let (h2584, _) = v2;
                h2584
            };
            let a2587 = {
                let (h2585, _) = v1.clone();
                h2585
            };
            if equal(a2586, a2587) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f423(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f457(f458(Fn17::F459, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2592 = {
        let (h2588, _) = f460(Fn18::F461, {
            let a2590 = {
                let a2589 = v0;
                length_list(a2589)
            };
            let a2591 = 1u64;
            nat_add(a2590, a2591)?
        }, v1)?;
        h2588
    };
    match m2592.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f424(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2597 = {
        let a2594 = f319({
            let (h2593, _) = v0;
            h2593
        })?;
        let a2595 = Bytes::lit(&[0x3a]);
        append_bytes(a2594, a2595)
    };
    let a2598 = {
        let (_, h2596) = v0;
        h2596
    };
    Ok(append_bytes(a2597, a2598))
}

pub fn f425(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f333({
        let (_, h2599) = {
            let (h2600, _) = f462(Fn19::F463, {
                let a2602 = {
                    let a2601 = v0.clone();
                    length_list(a2601)
                };
                let a2603 = 1u64;
                nat_add(a2602, a2603)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2600
        };
        h2599
    }))
}

pub fn f426(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2604 = v0;
    match m2604.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f426(v3, v1)
            } else {
                List::cons(v2, f426(v3, v1))
            }
        }
    }
}

pub fn f427(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f428(v0: List<Adt14>) -> List<Bytes> {
    let m2605 = v0;
    match m2605.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2606 = f147(v1, f8(Str::lit("std")));
            match m2606 {
                None => {
                    f428(v2)
                }
                Some(v3) => {
                    List::cons(f150(v3, f8(Str::lit("authority"))), f428(v2))
                }
            }
        }
    }
}

pub fn f429(v0: List<Adt14>) -> List<Bytes> {
    let m2607 = v0;
    match m2607.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2608 = f155(f147(v1, f8(Str::lit("req"))));
            match m2608 {
                None => {
                    f429(v2)
                }
                Some(v3) => {
                    List::cons(v3, f429(v2))
                }
            }
        }
    }
}

pub fn f430(v0: Adt26) -> R<Adt26> {
    let v1: u32 = f464(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2609) => {
            h2609
        }
    });
    let v2: u32 = f396(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h2610, _, _) => {
            h2610
        }
    }, f396(f465(match v0.clone() {
        Adt26::C0(_, _, _, _, h2611, _, _, _, _, _) => {
            h2611
        }
    }), f396(f466(match v0.clone() {
        Adt26::C0(_, _, _, _, h2612, _, _, _, _, _) => {
            h2612
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2613, _, _, _, _) => {
            h2613
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2614, _, _, _) => {
            h2614
        }
    }), f396(v1, {
        let m2617 = match v0.clone() {
            Adt26::C0(_, _, _, _, _, _, _, _, h2616, _) => {
                h2616
            }
        };
        match m2617 {
            Adt25::C0(h2615, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2615
            }
        }
    }))));
    let v3: u32 = f396(f467(match v0.clone() {
        Adt26::C0(h2618, _, _, _, _, _, _, _, _, _) => {
            h2618
        }
    }), f468(match v0.clone() {
        Adt26::C0(h2619, _, _, _, _, _, _, _, _, _) => {
            h2619
        }
    }, match v0.clone() {
        Adt26::C0(_, h2620, _, _, _, _, _, _, _, _) => {
            h2620
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2621, _, _, _, _, _, _, _) => {
            h2621
        }
    }));
    Ok(Adt26::C0(f396(v2, v3), match v0.clone() {
        Adt26::C0(h2622, _, _, _, _, _, _, _, _, _) => {
            h2622
        }
    }, match v0.clone() {
        Adt26::C0(_, h2623, _, _, _, _, _, _, _, _) => {
            h2623
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2624, _, _, _, _, _, _, _) => {
            h2624
        }
    }, f396(match v0.clone() {
        Adt26::C0(_, _, _, h2625, _, _, _, _, _, _) => {
            h2625
        }
    }, v2), match v0.clone() {
        Adt26::C0(_, _, _, _, h2626, _, _, _, _, _) => {
            h2626
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2627, _, _, _, _) => {
            h2627
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2628, _, _, _) => {
            h2628
        }
    }, f469(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h2629, _) => {
            h2629
        }
    }), {
        let a2631 = match v0 {
            Adt26::C0(_, _, _, _, _, _, _, _, _, h2630) => {
                h2630
            }
        };
        let a2632 = 1u64;
        nat_add(a2631, a2632)?
    }))
}

pub fn f431(v0: u32) -> u64 {
    let m2634 = {
        let a2633 = v0;
        fixed_u64::convert(i128::from(a2633))
    };
    match m2634 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f432(v0: u64) -> u32 {
    let m2638 = {
        let a2637 = {
            let a2635 = v0;
            let a2636 = 4294967295u64;
            fixed_u64::bit_and(a2635, a2636)
        };
        fixed_u32::convert(i128::from(a2637))
    };
    match m2638 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f433(v0: u64) -> u32 {
    let (_, h2639) = {
        let (_, h2640) = f470(Fn20::F471, 32u64, (v0, (1u32, 0u32)));
        h2640
    };
    h2639
}

pub fn f434(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2645 = f472(v0, 24u32);
    let a2646 = {
        let a2643 = f472(v1, 16u32);
        let a2644 = {
            let a2641 = f472(v2, 8u32);
            let a2642 = v3;
            fixed_u32::bit_or(a2641, a2642)
        };
        fixed_u32::bit_or(a2643, a2644)
    };
    fixed_u32::bit_or(a2645, a2646)
}

pub fn f435(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2647 = v2;
    let a2648 = v1;
    if nat_lt(a2647, a2648) {
        let m2651 = {
            let a2649 = v0;
            let a2650 = v2;
            index_bytes(a2649, a2650)
        };
        match m2651 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2653 = {
                    let a2652 = v3;
                    fixed_u32::convert(i128::from(a2652))
                };
                match m2653 {
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
        let a2654 = v2;
        let a2655 = v1;
        if nat_eq(a2654, a2655) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f436(v0: u8) -> bool {
    let a2656 = v0;
    let a2657 = 32u8;
    if if if equal(a2656, a2657) {
        true
    } else {
        let a2658 = v0;
        let a2659 = 9u8;
        equal(a2658, a2659)
    } {
        true
    } else {
        let a2660 = v0;
        let a2661 = 10u8;
        equal(a2660, a2661)
    } {
        true
    } else {
        let a2662 = v0;
        let a2663 = 12u8;
        if equal(a2662, a2663) {
            true
        } else {
            let a2664 = v0;
            let a2665 = 13u8;
            equal(a2664, a2665)
        }
    }
}

pub fn f437(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2668 = {
        let a2666 = v0;
        let a2667 = v1;
        index_bytes(a2666, a2667)
    };
    match m2668 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2713 = {
                    let a2689 = {
                        let a2677 = {
                            let a2671 = {
                                let a2669 = v3;
                                let a2670 = 128u8;
                                fixed_u8::bit_and(a2669, a2670)
                            };
                            let a2672 = 128u8;
                            if equal(a2671, a2672) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2678 = {
                            let a2675 = {
                                let a2673 = v3;
                                let a2674 = 64u8;
                                fixed_u8::bit_and(a2673, a2674)
                            };
                            let a2676 = 64u8;
                            if equal(a2675, a2676) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2677, a2678)?
                    };
                    let a2690 = {
                        let a2687 = {
                            let a2681 = {
                                let a2679 = v3;
                                let a2680 = 32u8;
                                fixed_u8::bit_and(a2679, a2680)
                            };
                            let a2682 = 32u8;
                            if equal(a2681, a2682) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2688 = {
                            let a2685 = {
                                let a2683 = v3;
                                let a2684 = 16u8;
                                fixed_u8::bit_and(a2683, a2684)
                            };
                            let a2686 = 16u8;
                            if equal(a2685, a2686) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2687, a2688)?
                    };
                    nat_add(a2689, a2690)?
                };
                let a2714 = {
                    let a2711 = {
                        let a2699 = {
                            let a2693 = {
                                let a2691 = v3;
                                let a2692 = 8u8;
                                fixed_u8::bit_and(a2691, a2692)
                            };
                            let a2694 = 8u8;
                            if equal(a2693, a2694) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2700 = {
                            let a2697 = {
                                let a2695 = v3;
                                let a2696 = 4u8;
                                fixed_u8::bit_and(a2695, a2696)
                            };
                            let a2698 = 4u8;
                            if equal(a2697, a2698) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2699, a2700)?
                    };
                    let a2712 = {
                        let a2709 = {
                            let a2703 = {
                                let a2701 = v3;
                                let a2702 = 2u8;
                                fixed_u8::bit_and(a2701, a2702)
                            };
                            let a2704 = 2u8;
                            if equal(a2703, a2704) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2710 = {
                            let a2707 = {
                                let a2705 = v3;
                                let a2706 = 1u8;
                                fixed_u8::bit_and(a2705, a2706)
                            };
                            let a2708 = 1u8;
                            if equal(a2707, a2708) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2709, a2710)?
                    };
                    nat_add(a2711, a2712)?
                };
                nat_add(a2713, a2714)?
            };
            let a2715 = 128u64;
            let a2716 = v4;
            if if nat_le(a2715, a2716) {
                let a2717 = v4;
                let a2718 = 191u64;
                nat_le(a2717, a2718)
            } else {
                false
            } {
                Ok(Some({
                    let a2719 = v4;
                    let a2720 = 128u64;
                    nat_sub(a2719, a2720)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f438(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a2721 = v1;
    let a2722 = v0;
    if if nat_le(a2721, a2722) {
        let a2723 = v0;
        let a2724 = v2;
        if nat_le(a2723, a2724) {
            let a2725 = v0;
            let a2726 = 55296u64;
            if nat_lt(a2725, a2726) {
                true
            } else {
                let a2727 = 57343u64;
                let a2728 = v0;
                nat_lt(a2727, a2728)
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

pub fn f439(v0: u64) -> bool {
    let a2729 = v0;
    let a2730 = 58u64;
    if if if nat_eq(a2729, a2730) {
        true
    } else {
        let a2731 = v0;
        let a2732 = 95u64;
        nat_eq(a2731, a2732)
    } {
        true
    } else {
        let a2733 = 65u64;
        let a2734 = v0;
        if if nat_le(a2733, a2734) {
            let a2735 = v0;
            let a2736 = 90u64;
            nat_le(a2735, a2736)
        } else {
            false
        } {
            true
        } else {
            let a2737 = 97u64;
            let a2738 = v0;
            if nat_le(a2737, a2738) {
                let a2739 = v0;
                let a2740 = 122u64;
                nat_le(a2739, a2740)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2741 = 192u64;
        let a2742 = v0;
        if if if if nat_le(a2741, a2742) {
            let a2743 = v0;
            let a2744 = 214u64;
            nat_le(a2743, a2744)
        } else {
            false
        } {
            true
        } else {
            let a2745 = 216u64;
            let a2746 = v0;
            if nat_le(a2745, a2746) {
                let a2747 = v0;
                let a2748 = 246u64;
                nat_le(a2747, a2748)
            } else {
                false
            }
        } {
            true
        } else {
            let a2749 = 248u64;
            let a2750 = v0;
            if if nat_le(a2749, a2750) {
                let a2751 = v0;
                let a2752 = 767u64;
                nat_le(a2751, a2752)
            } else {
                false
            } {
                true
            } else {
                let a2753 = 880u64;
                let a2754 = v0;
                if nat_le(a2753, a2754) {
                    let a2755 = v0;
                    let a2756 = 893u64;
                    nat_le(a2755, a2756)
                } else {
                    false
                }
            }
        } {
            true
        } else {
            let a2757 = 895u64;
            let a2758 = v0;
            if if if if nat_le(a2757, a2758) {
                let a2759 = v0;
                let a2760 = 8191u64;
                nat_le(a2759, a2760)
            } else {
                false
            } {
                true
            } else {
                let a2761 = 8204u64;
                let a2762 = v0;
                if nat_le(a2761, a2762) {
                    let a2763 = v0;
                    let a2764 = 8205u64;
                    nat_le(a2763, a2764)
                } else {
                    false
                }
            } {
                true
            } else {
                let a2765 = 8304u64;
                let a2766 = v0;
                if if nat_le(a2765, a2766) {
                    let a2767 = v0;
                    let a2768 = 8591u64;
                    nat_le(a2767, a2768)
                } else {
                    false
                } {
                    true
                } else {
                    let a2769 = 11264u64;
                    let a2770 = v0;
                    if nat_le(a2769, a2770) {
                        let a2771 = v0;
                        let a2772 = 12271u64;
                        nat_le(a2771, a2772)
                    } else {
                        false
                    }
                }
            } {
                true
            } else {
                let a2773 = 12289u64;
                let a2774 = v0;
                if if if nat_le(a2773, a2774) {
                    let a2775 = v0;
                    let a2776 = 55295u64;
                    nat_le(a2775, a2776)
                } else {
                    false
                } {
                    true
                } else {
                    let a2777 = 63744u64;
                    let a2778 = v0;
                    if nat_le(a2777, a2778) {
                        let a2779 = v0;
                        let a2780 = 64975u64;
                        nat_le(a2779, a2780)
                    } else {
                        false
                    }
                } {
                    true
                } else {
                    let a2781 = 65008u64;
                    let a2782 = v0;
                    if if nat_le(a2781, a2782) {
                        let a2783 = v0;
                        let a2784 = 65533u64;
                        nat_le(a2783, a2784)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a2785 = 65536u64;
                        let a2786 = v0;
                        if nat_le(a2785, a2786) {
                            let a2787 = v0;
                            let a2788 = 983039u64;
                            nat_le(a2787, a2788)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f440(v0: u64) -> bool {
    if f439(v0) {
        true
    } else {
        let a2789 = v0;
        let a2790 = 45u64;
        if if if nat_eq(a2789, a2790) {
            true
        } else {
            let a2791 = v0;
            let a2792 = 46u64;
            nat_eq(a2791, a2792)
        } {
            true
        } else {
            let a2793 = 48u64;
            let a2794 = v0;
            if nat_le(a2793, a2794) {
                let a2795 = v0;
                let a2796 = 57u64;
                nat_le(a2795, a2796)
            } else {
                false
            }
        } {
            true
        } else {
            let a2797 = v0;
            let a2798 = 183u64;
            if if nat_eq(a2797, a2798) {
                true
            } else {
                let a2799 = 768u64;
                let a2800 = v0;
                if nat_le(a2799, a2800) {
                    let a2801 = v0;
                    let a2802 = 879u64;
                    nat_le(a2801, a2802)
                } else {
                    false
                }
            } {
                true
            } else {
                let a2803 = 8255u64;
                let a2804 = v0;
                if nat_le(a2803, a2804) {
                    let a2805 = v0;
                    let a2806 = 8256u64;
                    nat_le(a2805, a2806)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f441(v0: Fn16, p2811: u64, p2812: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2810 = (p2811, p2812);
    {
        let mut m2810 = m2810;
        loop {
            let step = {
                let (v1, v2) = m2810;
                let m2807 = v1;
                if m2807 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2807 - 1;
                    let m2809 = {
                        let c2808: Fn16 = v0.clone();
                        c2808.apply(v2)?
                    };
                    match m2809 {
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
                Ok(next) => { m2810 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f473(v1, v0, v2)
}

pub fn f443(v0: u64) -> Bytes {
    f474(f475({
        let a2813 = v0;
        let a2814 = 16u64;
        let a2815 = 0u64;
        nat_quot(a2813, a2814, a2815)
    }), {
        let a2816 = v0;
        let a2817 = 16u64;
        let a2818 = 0u64;
        nat_rem(a2816, a2817, a2818)
    })
}

pub fn f444(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f476(f220(v0, v3)?, v1, v2) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2819 = v3;
            let a2820 = 1u64;
            nat_add(a2819, a2820)?
        }))
    }
}

pub fn f445(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2821, _) = v1.clone();
        h2821
    };
    let v3: Bytes = {
        let (h2822, _) = {
            let (_, h2823) = v1.clone();
            h2823
        };
        h2822
    };
    let a2826 = {
        let (_, h2824) = {
            let (_, h2825) = v1;
            h2825
        };
        h2824
    };
    let a2827 = 0u64;
    if nat_eq(a2826, a2827) {
        let v4: u64 = f302(v0.clone(), v2)?;
        let a2828 = v4;
        let a2829 = 34u64;
        if nat_eq(a2828, a2829) {
            Ok(Some(({
                let a2830 = v2;
                let a2831 = 1u64;
                nat_add(a2830, a2831)?
            }, (v3, 1u64))))
        } else {
            let a2832 = v4;
            let a2833 = 92u64;
            if nat_eq(a2832, a2833) {
                f477(v0, v2, v3)
            } else {
                let a2834 = v4;
                let a2835 = 32u64;
                if if nat_lt(a2834, a2835) {
                    true
                } else {
                    let a2836 = v4;
                    let a2837 = 256u64;
                    nat_eq(a2836, a2837)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2838 = v2;
                        let a2839 = 1u64;
                        nat_add(a2838, a2839)?
                    }, ({
                        let a2840 = v3;
                        let a2841 = f32(v0, v2, 1u64);
                        append_bytes(a2840, a2841)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f446(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f478(v0, v1)
}

pub fn f447(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f479(v0, v1)
}

pub fn f448(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2842 = v0.clone();
        length_bytes(a2842)
    };
    let a2843 = 0u64;
    let a2844 = v1;
    if nat_lt(a2843, a2844) {
        let a2845 = f480(v0.clone())?;
        let a2846 = v1;
        if nat_eq(a2845, a2846) {
            let a2847 = 1u64;
            let a2848 = v1;
            if nat_lt(a2847, a2848) {
                let m2851 = {
                    let a2849 = v0;
                    let a2850 = 0u64;
                    index_bytes(a2849, a2850)
                };
                match m2851 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2852 = v2;
                        let a2853 = 48u8;
                        Ok(!equal(a2852, a2853))
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

pub fn f449(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f481(v0, v1)
}

pub fn f450(v0: Bytes) -> R<u64> {
    let (h2854, _) = f244(Fn3::F482(v0.clone()), {
        let a2856 = {
            let a2855 = v0;
            length_bytes(a2855)
        };
        let a2857 = 1u64;
        nat_add(a2856, a2857)?
    }, 0u64)?;
    Ok(h2854)
}

pub fn f451(v0: u64) -> Bytes {
    let m2858 = v0;
    if m2858 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2858 - 1;
        let a2859 = f8(Str::lit("0"));
        let a2860 = f451(v1);
        append_bytes(a2859, a2860)
    }
}

pub fn f452(p2863: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2862 = p2863;
    {
        let mut m2862 = m2862;
        loop {
            let step = {
                let v0 = m2862;
                let m2861 = v0;
                match m2861.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f36(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f86(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f86(v4, v3.clone())
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
                Ok(next) => { m2862 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: Adt14, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2864 = v0;
    match m2864 {
        Adt14::C0(_) => {
            Ok(v1)
        }
        Adt14::C1(_, _) => {
            Ok(v1)
        }
        Adt14::C2(_) => {
            Ok(v1)
        }
        Adt14::C3(r2867) => {
            let v2 = (*r2867).clone();
            f453(v2, v1)
        }
        Adt14::C4(r2868) => {
            let v3 = (*r2868).clone();
            f453(v3, v1)
        }
        Adt14::C5 => {
            Ok(v1)
        }
        Adt14::C6(r2869, r2870) => {
            let v4 = (*r2869).clone();
            let v5 = (*r2870).clone();
            f453(v4, f453(v5, v1)?)
        }
        Adt14::C7(v6, r2871, r2872) => {
            let v7 = (*r2871).clone();
            let v8 = (*r2872).clone();
            let v9: List<Bytes> = f453(v7.clone(), f453(v8, v1)?)?;
            if f36(v6, f8(Str::lit("$ref"))) {
                let m2865 = f155(Some(v7));
                match m2865 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2866 = f483(v10)?;
                        match m2866 {
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

pub fn f454(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt24>), Bytes>> {
    let m2873 = f147(v1.clone(), f8(Str::lit("enum")));
    match m2873 {
        None => {
            let m2875 = f147(v1.clone(), f8(Str::lit("oneOf")));
            match m2875 {
                None => {
                    if f488(v1.clone(), v0.clone())? {
                        let m2876 = f489(v0.clone(), f371(f147(v1.clone(), f8(Str::lit("properties")))), f74(f151(f147(v1, f8(Str::lit("required"))))), true)?;
                        match m2876 {
                            Ok(v7) => {
                                Ok(f486(f490(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f209({
                                    let (h2877, _) = v7;
                                    h2877
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h2878) = v7;
                                    h2878
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt24>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m2879 = f371(f147(v1.clone(), f8(Str::lit("properties"))));
                        match m2879.uncons() {
                            None => {
                                let m2882 = f491(v1, {
                                    let a2880 = v0.clone();
                                    let a2881 = f8(Str::lit("Value"));
                                    append_bytes(a2880, a2881)
                                })?;
                                match m2882 {
                                    Ok(v9) => {
                                        Ok(f486(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h2883, _) = v9;
                                            h2883
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h2884) = v9;
                                            h2884
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt24>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2885 = f489(v0.clone(), f371(f147(v1.clone(), f8(Str::lit("properties")))), f74(f151(f147(v1, f8(Str::lit("required"))))), false)?;
                                match m2885 {
                                    Ok(v13) => {
                                        Ok(f486(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2887 = {
                                                let (h2886, _) = v13.clone();
                                                h2886
                                            };
                                            match m2887.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f209({
                                                        let (h2888, _) = v13;
                                                        h2888
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h2889) = v13;
                                            h2889
                                        }))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<(Bytes, List<Adt24>), Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f487(v0, f238(v5))
                }
            }
        }
        Some(v2) => {
            let m2874 = f484(f238(v2), List::<Bytes>::nil())?;
            match m2874 {
                Ok(v4) => {
                    Ok(f486(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f209(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt24>::nil()))
                }
                Err(v3) => {
                    Ok(f485(v3, v0))
                }
            }
        }
    }
}

pub fn f455(v0: Adt14) -> Adt14 {
    let v2: Adt14 = {
        let m2890 = f147(v0.clone(), f8(Str::lit("anyOf")));
        match m2890 {
            None => {
                v0
            }
            Some(v1) => {
                if f492(f238(v1.clone())) {
                    f493(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f494(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f495(f238(v1.clone()))
                    } {
                        v0
                    } else {
                        f496(f493(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt14 = {
        let m2891 = f147(v2.clone(), f8(Str::lit("oneOf")));
        match m2891 {
            None => {
                v2
            }
            Some(v3) => {
                if f492(f238(v3)) {
                    f493(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2892 = f147(v4.clone(), f8(Str::lit("allOf")));
    match m2892 {
        None => {
            v4
        }
        Some(v5) => {
            if f497(f238(v5)) {
                f493(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f456(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
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
    let a2939 = v4;
    let a2940 = 34u64;
    if nat_eq(a2939, a2940) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2941 = v4;
        let a2942 = 92u64;
        if nat_eq(a2941, a2942) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2943 = v4;
            let a2944 = 8u64;
            if nat_eq(a2943, a2944) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2945 = v4;
                let a2946 = 12u64;
                if nat_eq(a2945, a2946) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2947 = v4;
                    let a2948 = 10u64;
                    if nat_eq(a2947, a2948) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2949 = v4;
                        let a2950 = 13u64;
                        if nat_eq(a2949, a2950) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2951 = v4;
                            let a2952 = 9u64;
                            if nat_eq(a2951, a2952) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2953 = v4;
                                let a2954 = 32u64;
                                if nat_lt(a2953, a2954) {
                                    let a2955 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2956 = f498(v2)?;
                                    Ok(append_bytes(a2955, a2956))
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

pub fn f457(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f499(Fn21::F500, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f458(v0: Fn17, p2960: List<List<(Bytes, Bytes)>>, p2961: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2959 = (p2960, p2961);
    {
        let mut m2959 = m2959;
        loop {
            let step = {
                let (v1, v2) = m2959;
                let m2957 = v2;
                match m2957.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2958: Fn17 = v0.clone();
                            c2958.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2959 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f460(v0: Fn18, p2966: u64, p2967: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2965 = (p2966, p2967);
    {
        let mut m2965 = m2965;
        loop {
            let step = {
                let (v1, v2) = m2965;
                let m2962 = v1;
                if m2962 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2962 - 1;
                    let m2964 = {
                        let c2963: Fn18 = v0.clone();
                        c2963.apply(v2.clone())?
                    };
                    match m2964 {
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
                Ok(next) => { m2965 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f501(v0)
}

pub fn f462(v0: Fn19, p2972: u64, p2973: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2971 = (p2972, p2973);
    {
        let mut m2971 = m2971;
        loop {
            let step = {
                let (v1, v2) = m2971;
                let m2968 = v1;
                if m2968 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2968 - 1;
                    let m2970 = {
                        let c2969: Fn19 = v0.clone();
                        c2969.apply(v2.clone())?
                    };
                    match m2970 {
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
                Ok(next) => { m2971 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f502(v0)
}

pub fn f464(v0: u64) -> u32 {
    let a2974 = v0;
    let a2975 = 32u64;
    if nat_lt(a2974, a2975) {
        let a2976 = v0;
        let a2977 = 16u64;
        if nat_lt(a2976, a2977) {
            let a2978 = v0;
            let a2979 = 8u64;
            if nat_lt(a2978, a2979) {
                let a2980 = v0;
                let a2981 = 4u64;
                if nat_lt(a2980, a2981) {
                    let a2982 = v0;
                    let a2983 = 2u64;
                    if nat_lt(a2982, a2983) {
                        let a2984 = v0;
                        let a2985 = 1u64;
                        if nat_lt(a2984, a2985) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2986 = v0;
                        let a2987 = 3u64;
                        if nat_lt(a2986, a2987) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2988 = v0;
                    let a2989 = 6u64;
                    if nat_lt(a2988, a2989) {
                        let a2990 = v0;
                        let a2991 = 5u64;
                        if nat_lt(a2990, a2991) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2992 = v0;
                        let a2993 = 7u64;
                        if nat_lt(a2992, a2993) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2994 = v0;
                let a2995 = 12u64;
                if nat_lt(a2994, a2995) {
                    let a2996 = v0;
                    let a2997 = 10u64;
                    if nat_lt(a2996, a2997) {
                        let a2998 = v0;
                        let a2999 = 9u64;
                        if nat_lt(a2998, a2999) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3000 = v0;
                        let a3001 = 11u64;
                        if nat_lt(a3000, a3001) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3002 = v0;
                    let a3003 = 14u64;
                    if nat_lt(a3002, a3003) {
                        let a3004 = v0;
                        let a3005 = 13u64;
                        if nat_lt(a3004, a3005) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3006 = v0;
                        let a3007 = 15u64;
                        if nat_lt(a3006, a3007) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3008 = v0;
            let a3009 = 24u64;
            if nat_lt(a3008, a3009) {
                let a3010 = v0;
                let a3011 = 20u64;
                if nat_lt(a3010, a3011) {
                    let a3012 = v0;
                    let a3013 = 18u64;
                    if nat_lt(a3012, a3013) {
                        let a3014 = v0;
                        let a3015 = 17u64;
                        if nat_lt(a3014, a3015) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3016 = v0;
                        let a3017 = 19u64;
                        if nat_lt(a3016, a3017) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3018 = v0;
                    let a3019 = 22u64;
                    if nat_lt(a3018, a3019) {
                        let a3020 = v0;
                        let a3021 = 21u64;
                        if nat_lt(a3020, a3021) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3022 = v0;
                        let a3023 = 23u64;
                        if nat_lt(a3022, a3023) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3024 = v0;
                let a3025 = 28u64;
                if nat_lt(a3024, a3025) {
                    let a3026 = v0;
                    let a3027 = 26u64;
                    if nat_lt(a3026, a3027) {
                        let a3028 = v0;
                        let a3029 = 25u64;
                        if nat_lt(a3028, a3029) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3030 = v0;
                        let a3031 = 27u64;
                        if nat_lt(a3030, a3031) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3032 = v0;
                    let a3033 = 30u64;
                    if nat_lt(a3032, a3033) {
                        let a3034 = v0;
                        let a3035 = 29u64;
                        if nat_lt(a3034, a3035) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3036 = v0;
                        let a3037 = 31u64;
                        if nat_lt(a3036, a3037) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3038 = v0;
        let a3039 = 48u64;
        if nat_lt(a3038, a3039) {
            let a3040 = v0;
            let a3041 = 40u64;
            if nat_lt(a3040, a3041) {
                let a3042 = v0;
                let a3043 = 36u64;
                if nat_lt(a3042, a3043) {
                    let a3044 = v0;
                    let a3045 = 34u64;
                    if nat_lt(a3044, a3045) {
                        let a3046 = v0;
                        let a3047 = 33u64;
                        if nat_lt(a3046, a3047) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3048 = v0;
                        let a3049 = 35u64;
                        if nat_lt(a3048, a3049) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3050 = v0;
                    let a3051 = 38u64;
                    if nat_lt(a3050, a3051) {
                        let a3052 = v0;
                        let a3053 = 37u64;
                        if nat_lt(a3052, a3053) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3054 = v0;
                        let a3055 = 39u64;
                        if nat_lt(a3054, a3055) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3056 = v0;
                let a3057 = 44u64;
                if nat_lt(a3056, a3057) {
                    let a3058 = v0;
                    let a3059 = 42u64;
                    if nat_lt(a3058, a3059) {
                        let a3060 = v0;
                        let a3061 = 41u64;
                        if nat_lt(a3060, a3061) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3062 = v0;
                        let a3063 = 43u64;
                        if nat_lt(a3062, a3063) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3064 = v0;
                    let a3065 = 46u64;
                    if nat_lt(a3064, a3065) {
                        let a3066 = v0;
                        let a3067 = 45u64;
                        if nat_lt(a3066, a3067) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3068 = v0;
                        let a3069 = 47u64;
                        if nat_lt(a3068, a3069) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3070 = v0;
            let a3071 = 56u64;
            if nat_lt(a3070, a3071) {
                let a3072 = v0;
                let a3073 = 52u64;
                if nat_lt(a3072, a3073) {
                    let a3074 = v0;
                    let a3075 = 50u64;
                    if nat_lt(a3074, a3075) {
                        let a3076 = v0;
                        let a3077 = 49u64;
                        if nat_lt(a3076, a3077) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3078 = v0;
                        let a3079 = 51u64;
                        if nat_lt(a3078, a3079) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3080 = v0;
                    let a3081 = 54u64;
                    if nat_lt(a3080, a3081) {
                        let a3082 = v0;
                        let a3083 = 53u64;
                        if nat_lt(a3082, a3083) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3084 = v0;
                        let a3085 = 55u64;
                        if nat_lt(a3084, a3085) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3086 = v0;
                let a3087 = 60u64;
                if nat_lt(a3086, a3087) {
                    let a3088 = v0;
                    let a3089 = 58u64;
                    if nat_lt(a3088, a3089) {
                        let a3090 = v0;
                        let a3091 = 57u64;
                        if nat_lt(a3090, a3091) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3092 = v0;
                        let a3093 = 59u64;
                        if nat_lt(a3092, a3093) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3094 = v0;
                    let a3095 = 62u64;
                    if nat_lt(a3094, a3095) {
                        let a3096 = v0;
                        let a3097 = 61u64;
                        if nat_lt(a3096, a3097) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3098 = v0;
                        let a3099 = 63u64;
                        if nat_lt(a3098, a3099) {
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

pub fn f465(v0: u32) -> u32 {
    let a3102 = {
        let a3100 = f503(v0, 6u32, 26u32);
        let a3101 = f503(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3100, a3101)
    };
    let a3103 = f503(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3102, a3103)
}

pub fn f466(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3109 = {
        let a3104 = v0;
        let a3105 = v1;
        fixed_u32::bit_and(a3104, a3105)
    };
    let a3110 = {
        let a3107 = {
            let a3106 = v0;
            fixed_u32::bit_not(a3106)
        };
        let a3108 = v2;
        fixed_u32::bit_and(a3107, a3108)
    };
    fixed_u32::bit_xor(a3109, a3110)
}

pub fn f467(v0: u32) -> u32 {
    let a3113 = {
        let a3111 = f503(v0, 2u32, 30u32);
        let a3112 = f503(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3111, a3112)
    };
    let a3114 = f503(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3113, a3114)
}

pub fn f468(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3123 = {
        let a3119 = {
            let a3115 = v0;
            let a3116 = v1;
            fixed_u32::bit_and(a3115, a3116)
        };
        let a3120 = {
            let a3117 = v0;
            let a3118 = v2;
            fixed_u32::bit_and(a3117, a3118)
        };
        fixed_u32::bit_xor(a3119, a3120)
    };
    let a3124 = {
        let a3121 = v1;
        let a3122 = v2;
        fixed_u32::bit_and(a3121, a3122)
    };
    fixed_u32::bit_xor(a3123, a3124)
}

pub fn f469(v0: Adt25) -> Adt25 {
    Adt25::C0(match v0.clone() {
        Adt25::C0(_, h3125, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3125
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h3126, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3126
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, h3127, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3127
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, h3128, _, _, _, _, _, _, _, _, _, _, _) => {
            h3128
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h3129, _, _, _, _, _, _, _, _, _, _) => {
            h3129
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h3130, _, _, _, _, _, _, _, _, _) => {
            h3130
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h3131, _, _, _, _, _, _, _, _) => {
            h3131
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h3132, _, _, _, _, _, _, _) => {
            h3132
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h3133, _, _, _, _, _, _) => {
            h3133
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, h3134, _, _, _, _, _) => {
            h3134
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, h3135, _, _, _, _) => {
            h3135
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3136, _, _, _) => {
            h3136
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3137, _, _) => {
            h3137
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3138, _) => {
            h3138
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3139) => {
            h3139
        }
    }, f396(f504(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3140, _) => {
            h3140
        }
    }), f396(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h3141, _, _, _, _, _, _) => {
            h3141
        }
    }, f396(f505(match v0.clone() {
        Adt25::C0(_, h3142, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3142
        }
    }), match v0 {
        Adt25::C0(h3143, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3143
        }
    }))))
}

pub fn f470(v0: Fn20, p3147: u64, p3148: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3146 = (p3147, p3148);
    {
        let mut m3146 = m3146;
        loop {
            let step = {
                let (v1, v2) = m3146;
                let m3144 = v1;
                if m3144 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3144 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3145: Fn20 = v0.clone();
                        c3145.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3149, _) = v0;
        h3149
    };
    let v2: u32 = {
        let (h3150, _) = {
            let (_, h3151) = v0;
            h3151
        };
        h3150
    };
    let v3: u32 = {
        let (_, h3152) = {
            let (_, h3153) = v0;
            h3153
        };
        h3152
    };
    ({
        let a3154 = v1;
        let a3155 = 2u64;
        let a3156 = 0u64;
        nat_quot(a3154, a3155, a3156)
    }, (f472(v2, 1u32), {
        let a3160 = {
            let a3157 = v1;
            let a3158 = 2u64;
            let a3159 = 0u64;
            nat_rem(a3157, a3158, a3159)
        };
        let a3161 = 1u64;
        if nat_eq(a3160, a3161) {
            let a3162 = v3;
            let a3163 = v2;
            fixed_u32::bit_or(a3162, a3163)
        } else {
            v3
        }
    }))
}

pub fn f472(v0: u32, v1: u32) -> u32 {
    let m3166 = {
        let a3164 = v0;
        let a3165 = v1;
        fixed_u32::shift_left(a3164, a3165)
    };
    match m3166 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f473(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f220(v0, {
        let (h3167, _) = v2;
        h3167
    })?;
    let a3169 = 1114111u64;
    let a3170 = {
        let (_, h3168) = v2;
        h3168
    };
    if nat_lt(a3169, a3170) {
        Ok(None::<(u64, u64)>)
    } else {
        let a3171 = 48u64;
        let a3172 = v3;
        if if nat_le(a3171, a3172) {
            let a3173 = v3;
            let a3174 = 57u64;
            nat_le(a3173, a3174)
        } else {
            false
        } {
            Ok(Some(({
                let a3176 = {
                    let (h3175, _) = v2;
                    h3175
                };
                let a3177 = 1u64;
                nat_add(a3176, a3177)?
            }, {
                let a3183 = {
                    let a3179 = {
                        let (_, h3178) = v2;
                        h3178
                    };
                    let a3180 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3179, a3180)?
                };
                let a3184 = {
                    let a3181 = v3;
                    let a3182 = 48u64;
                    nat_sub(a3181, a3182)
                };
                nat_add(a3183, a3184)?
            })))
        } else {
            if if v1 {
                let a3185 = 97u64;
                let a3186 = v3;
                if nat_le(a3185, a3186) {
                    let a3187 = v3;
                    let a3188 = 102u64;
                    nat_le(a3187, a3188)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some(({
                    let a3190 = {
                        let (h3189, _) = v2;
                        h3189
                    };
                    let a3191 = 1u64;
                    nat_add(a3190, a3191)?
                }, {
                    let a3197 = {
                        let a3193 = {
                            let (_, h3192) = v2;
                            h3192
                        };
                        let a3194 = 16u64;
                        nat_mul(a3193, a3194)?
                    };
                    let a3198 = {
                        let a3195 = v3;
                        let a3196 = 87u64;
                        nat_sub(a3195, a3196)
                    };
                    nat_add(a3197, a3198)?
                })))
            } else {
                if if v1 {
                    let a3199 = 65u64;
                    let a3200 = v3;
                    if nat_le(a3199, a3200) {
                        let a3201 = v3;
                        let a3202 = 70u64;
                        nat_le(a3201, a3202)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(({
                        let a3204 = {
                            let (h3203, _) = v2;
                            h3203
                        };
                        let a3205 = 1u64;
                        nat_add(a3204, a3205)?
                    }, {
                        let a3211 = {
                            let a3207 = {
                                let (_, h3206) = v2;
                                h3206
                            };
                            let a3208 = 16u64;
                            nat_mul(a3207, a3208)?
                        };
                        let a3212 = {
                            let a3209 = v3;
                            let a3210 = 55u64;
                            nat_sub(a3209, a3210)
                        };
                        nat_add(a3211, a3212)?
                    })))
                } else {
                    Ok(None::<(u64, u64)>)
                }
            }
        }
    }
}

pub fn f474(p3219: List<Bytes>, p3220: u64) -> Bytes {
    let m3218 = (p3219, p3220);
    {
        let mut m3218 = m3218;
        loop {
            let step = {
                let (v0, v1) = m3218;
                let m3213 = v0;
                match m3213.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a3214 = v1;
                        let a3215 = 0u64;
                        if nat_eq(a3214, a3215) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a3216 = v1;
                                let a3217 = 1u64;
                                nat_sub(a3216, a3217)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475(v0: u64) -> List<Bytes> {
    let a3221 = v0;
    let a3222 = 0u64;
    if nat_eq(a3221, a3222) {
        f506()
    } else {
        let a3223 = v0;
        let a3224 = 1u64;
        if nat_eq(a3223, a3224) {
            f507()
        } else {
            let a3225 = v0;
            let a3226 = 2u64;
            if nat_eq(a3225, a3226) {
                f508()
            } else {
                let a3227 = v0;
                let a3228 = 3u64;
                if nat_eq(a3227, a3228) {
                    f509()
                } else {
                    let a3229 = v0;
                    let a3230 = 4u64;
                    if nat_eq(a3229, a3230) {
                        f510()
                    } else {
                        let a3231 = v0;
                        let a3232 = 5u64;
                        if nat_eq(a3231, a3232) {
                            f511()
                        } else {
                            let a3233 = v0;
                            let a3234 = 6u64;
                            if nat_eq(a3233, a3234) {
                                f512()
                            } else {
                                let a3235 = v0;
                                let a3236 = 7u64;
                                if nat_eq(a3235, a3236) {
                                    f513()
                                } else {
                                    let a3237 = v0;
                                    let a3238 = 8u64;
                                    if nat_eq(a3237, a3238) {
                                        f514()
                                    } else {
                                        let a3239 = v0;
                                        let a3240 = 9u64;
                                        if nat_eq(a3239, a3240) {
                                            f515()
                                        } else {
                                            let a3241 = v0;
                                            let a3242 = 10u64;
                                            if nat_eq(a3241, a3242) {
                                                f516()
                                            } else {
                                                let a3243 = v0;
                                                let a3244 = 11u64;
                                                if nat_eq(a3243, a3244) {
                                                    f517()
                                                } else {
                                                    let a3245 = v0;
                                                    let a3246 = 12u64;
                                                    if nat_eq(a3245, a3246) {
                                                        f518()
                                                    } else {
                                                        let a3247 = v0;
                                                        let a3248 = 13u64;
                                                        if nat_eq(a3247, a3248) {
                                                            f519()
                                                        } else {
                                                            let a3249 = v0;
                                                            let a3250 = 14u64;
                                                            if nat_eq(a3249, a3250) {
                                                                f520()
                                                            } else {
                                                                f521()
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

pub fn f476(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        let a3251 = v0;
        let a3252 = v2;
        if if if nat_eq(a3251, a3252) {
            true
        } else {
            let a3253 = v0;
            let a3254 = 60u64;
            nat_eq(a3253, a3254)
        } {
            true
        } else {
            let a3255 = v0;
            let a3256 = 38u64;
            if nat_eq(a3255, a3256) {
                true
            } else {
                let a3257 = v0;
                let a3258 = 256u64;
                nat_eq(a3257, a3258)
            }
        } {
            true
        } else {
            let a3259 = v0;
            let a3260 = 9u64;
            if nat_eq(a3259, a3260) {
                true
            } else {
                let a3261 = v0;
                let a3262 = 10u64;
                nat_eq(a3261, a3262)
            }
        }
    } else {
        let a3263 = v0;
        let a3264 = 60u64;
        if if nat_eq(a3263, a3264) {
            true
        } else {
            let a3265 = v0;
            let a3266 = 38u64;
            nat_eq(a3265, a3266)
        } {
            true
        } else {
            let a3267 = v0;
            let a3268 = 93u64;
            if nat_eq(a3267, a3268) {
                true
            } else {
                let a3269 = v0;
                let a3270 = 256u64;
                nat_eq(a3269, a3270)
            }
        }
    }
}

pub fn f477(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f302(v0.clone(), {
        let a3271 = v1;
        let a3272 = 1u64;
        nat_add(a3271, a3272)?
    })?;
    let v4: Bytes = {
        let a3273 = v3;
        let a3274 = 117u64;
        if nat_eq(a3273, a3274) {
            f522(v0, v1)?
        } else {
            f523(v3)
        }
    };
    let a3275 = v4.clone();
    let a3276 = Bytes::lit(&[]);
    if equal(a3275, a3276) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a3277 = v3;
            let a3278 = 117u64;
            if nat_eq(a3277, a3278) {
                let a3279 = v1;
                let a3280 = 6u64;
                nat_add(a3279, a3280)?
            } else {
                let a3281 = v1;
                let a3282 = 2u64;
                nat_add(a3281, a3282)?
            }
        }, ({
            let a3283 = v2;
            let a3284 = v4;
            append_bytes(a3283, a3284)
        }, 0u64))))
    }
}

pub fn f478(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f302(v0, v1)?;
    let a3285 = 47u64;
    let a3286 = v2;
    if if nat_lt(a3285, a3286) {
        let a3287 = v2;
        let a3288 = 58u64;
        nat_lt(a3287, a3288)
    } else {
        false
    } {
        Ok(Some({
            let a3289 = v1;
            let a3290 = 1u64;
            nat_add(a3289, a3290)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f479(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3291 = f302(v0, v1)?;
    let a3292 = 48u64;
    if nat_eq(a3291, a3292) {
        Ok(Some({
            let a3293 = v1;
            let a3294 = 1u64;
            nat_add(a3293, a3294)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f480(v0: Bytes) -> R<u64> {
    let (h3295, _) = f244(Fn3::F524(v0.clone()), {
        let a3296 = v0;
        length_bytes(a3296)
    }, 0u64)?;
    Ok(h3295)
}

pub fn f481(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3300 = {
        let a3298 = v0;
        let a3299 = {
            let (h3297, _) = v1;
            h3297
        };
        index_bytes(a3298, a3299)
    };
    match m3300 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3302 = {
                    let (h3301, _) = v1;
                    h3301
                };
                let a3303 = 1u64;
                nat_add(a3302, a3303)?
            }, {
                let a3307 = {
                    let a3305 = {
                        let (_, h3304) = v1;
                        h3304
                    };
                    let a3306 = 10u64;
                    nat_mul(a3305, a3306)?
                };
                let a3308 = f525(v2)?;
                nat_add(a3307, a3308)?
            })))
        }
    }
}

pub fn f482(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f526(v0, v1)
}

pub fn f483(v0: Bytes) -> R<Option<Bytes>> {
    if f199(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f234(f32(v0.clone(), 8u64, {
            let a3310 = {
                let a3309 = v0;
                length_bytes(a3309)
            };
            let a3311 = 8u64;
            nat_sub(a3310, a3311)
        }))?))
    } else {
        if f199(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f234(f32(v0.clone(), 14u64, {
                let a3313 = {
                    let a3312 = v0;
                    length_bytes(a3312)
                };
                let a3314 = 14u64;
                nat_sub(a3313, a3314)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f484(v0: List<Adt14>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f527(f528(f74(v0.clone()))?, v0, v1)
}

pub fn f485(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt24>), Bytes> {
    Err::<(Bytes, List<Adt24>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f486(v0: Bytes, v1: List<Adt24>) -> Result<(Bytes, List<Adt24>), Bytes> {
    Ok::<(Bytes, List<Adt24>), Bytes>((v0, v1))
}

pub fn f487(v0: Bytes, v1: List<Adt14>) -> R<Result<(Bytes, List<Adt24>), Bytes>> {
    let m3315 = v1.clone();
    match m3315.uncons() {
        None => {
            Ok(f485(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3316 = f529(f371(f147(v2, f8(Str::lit("properties")))), v1.clone());
            match m3316.uncons() {
                None => {
                    let m3317 = f530(f528(f531(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3317 {
                        Ok(v5) => {
                            Ok(f486(f490(v0, {
                                let (h3318, _) = v5;
                                h3318
                            }), {
                                let (_, h3319) = v5;
                                h3319
                            }))
                        }
                        Err(v4) => {
                            Ok(f485(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3320 = f532(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3320 {
                        Ok(v9) => {
                            Ok(f486(f490(v0, {
                                let (h3321, _) = v9;
                                h3321
                            }), {
                                let (_, h3322) = v9;
                                h3322
                            }))
                        }
                        Err(v8) => {
                            Ok(f485(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f488(v0: Adt14, v1: Bytes) -> R<bool> {
    let m3323 = v0;
    match m3323 {
        Adt14::C0(_) => {
            Ok(false)
        }
        Adt14::C1(_, _) => {
            Ok(false)
        }
        Adt14::C2(_) => {
            Ok(false)
        }
        Adt14::C3(r3324) => {
            let v2 = (*r3324).clone();
            f488(v2, v1)
        }
        Adt14::C4(r3325) => {
            let v3 = (*r3325).clone();
            f488(v3, v1)
        }
        Adt14::C5 => {
            Ok(false)
        }
        Adt14::C6(r3326, r3327) => {
            let v4 = (*r3326).clone();
            let v5 = (*r3327).clone();
            if f488(v4, v1.clone())? {
                Ok(true)
            } else {
                f488(v5, v1)
            }
        }
        Adt14::C7(v6, r3328, r3329) => {
            let v7 = (*r3328).clone();
            let v8 = (*r3329).clone();
            if if f36(v6, f8(Str::lit("$ref"))) {
                f533(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f488(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f488(v8, v1)
                }
            }
        }
    }
}

pub fn f489(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt24>), Bytes>> {
    f534(f528(f535(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f490(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f209(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f491(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt24>), Bytes>> {
    let m3330 = f536(v0.clone());
    match m3330 {
        None => {
            if f36(f150(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3331 = f147(v0, f8(Str::lit("items")));
                match m3331 {
                    None => {
                        Ok(f537(f8(Str::lit("List")), f539()))
                    }
                    Some(v3) => {
                        Ok(f537(f8(Str::lit("List")), f538(v3, {
                            let a3332 = v1;
                            let a3333 = f8(Str::lit("Item"));
                            append_bytes(a3332, a3333)
                        })?))
                    }
                }
            } else {
                f538(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f537(f8(Str::lit("Option")), f538(v2, v1)?))
        }
    }
}

pub fn f492(p3336: List<Adt14>) -> bool {
    let m3335 = p3336;
    {
        let mut m3335 = m3335;
        loop {
            let step = {
                let v0 = m3335;
                let m3334 = v0;
                match m3334.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f540(f414(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt14>, bool>(v2)
                        } else {
                            Err::<List<Adt14>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3335 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(v0: Adt14, v1: Bytes) -> Adt14 {
    let m3337 = v0;
    match m3337 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r3338) => {
            let v10 = (*r3338).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r3339) => {
            let v5 = (*r3339).clone();
            Adt14::C4(std::rc::Rc::new(f493(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r3340, r3341) => {
            let v11 = (*r3340).clone();
            let v12 = (*r3341).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r3342, r3343) => {
            let v3 = (*r3342).clone();
            let v4 = (*r3343).clone();
            if f36(v2.clone(), v1.clone()) {
                f493(v4, v1)
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f493(v4, v1)))
            }
        }
    }
}

pub fn f494(v0: Adt14, v1: Bytes) -> bool {
    let m3344 = f147(v0, v1);
    match m3344 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f495(v0: List<Adt14>) -> bool {
    let m3345 = v0.clone();
    match m3345.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3346 = f529(f371(f147(v1, f8(Str::lit("properties")))), v0.clone());
            match m3346.uncons() {
                None => {
                    f542(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f541(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f496(v0: Adt14, v1: Bytes, v2: Adt14) -> Adt14 {
    let m3347 = v0;
    match m3347 {
        Adt14::C0(v7) => {
            Adt14::C0(v7)
        }
        Adt14::C1(v8, v9) => {
            Adt14::C1(v8, v9)
        }
        Adt14::C2(v10) => {
            Adt14::C2(v10)
        }
        Adt14::C3(r3348) => {
            let v11 = (*r3348).clone();
            Adt14::C3(std::rc::Rc::new(v11))
        }
        Adt14::C4(r3349) => {
            let v3 = (*r3349).clone();
            Adt14::C4(std::rc::Rc::new(Adt14::C7(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r3350, r3351) => {
            let v12 = (*r3350).clone();
            let v13 = (*r3351).clone();
            Adt14::C6(std::rc::Rc::new(v12), std::rc::Rc::new(v13))
        }
        Adt14::C7(v4, r3352, r3353) => {
            let v5 = (*r3352).clone();
            let v6 = (*r3353).clone();
            Adt14::C7(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f497(p3357: List<Adt14>) -> bool {
    let m3356 = p3357;
    {
        let mut m3356 = m3356;
        loop {
            let step = {
                let v0 = m3356;
                let m3354 = v0;
                match m3354.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f540(f414(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f492(f238({
                                let m3355 = f147(v1, f8(Str::lit("anyOf")));
                                match m3355 {
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
                Ok(next) => { m3356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3402 = {
            let a3378 = {
                let a3366 = {
                    let a3360 = {
                        let a3358 = v1;
                        let a3359 = 128u8;
                        fixed_u8::bit_and(a3358, a3359)
                    };
                    let a3361 = 128u8;
                    if equal(a3360, a3361) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3367 = {
                    let a3364 = {
                        let a3362 = v1;
                        let a3363 = 64u8;
                        fixed_u8::bit_and(a3362, a3363)
                    };
                    let a3365 = 64u8;
                    if equal(a3364, a3365) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3366, a3367)?
            };
            let a3379 = {
                let a3376 = {
                    let a3370 = {
                        let a3368 = v1;
                        let a3369 = 32u8;
                        fixed_u8::bit_and(a3368, a3369)
                    };
                    let a3371 = 32u8;
                    if equal(a3370, a3371) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3377 = {
                    let a3374 = {
                        let a3372 = v1;
                        let a3373 = 16u8;
                        fixed_u8::bit_and(a3372, a3373)
                    };
                    let a3375 = 16u8;
                    if equal(a3374, a3375) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3376, a3377)?
            };
            nat_add(a3378, a3379)?
        };
        let a3403 = {
            let a3400 = {
                let a3388 = {
                    let a3382 = {
                        let a3380 = v1;
                        let a3381 = 8u8;
                        fixed_u8::bit_and(a3380, a3381)
                    };
                    let a3383 = 8u8;
                    if equal(a3382, a3383) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3389 = {
                    let a3386 = {
                        let a3384 = v1;
                        let a3385 = 4u8;
                        fixed_u8::bit_and(a3384, a3385)
                    };
                    let a3387 = 4u8;
                    if equal(a3386, a3387) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3388, a3389)?
            };
            let a3401 = {
                let a3398 = {
                    let a3392 = {
                        let a3390 = v1;
                        let a3391 = 2u8;
                        fixed_u8::bit_and(a3390, a3391)
                    };
                    let a3393 = 2u8;
                    if equal(a3392, a3393) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3399 = {
                    let a3396 = {
                        let a3394 = v1;
                        let a3395 = 1u8;
                        fixed_u8::bit_and(a3394, a3395)
                    };
                    let a3397 = 1u8;
                    if equal(a3396, a3397) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3398, a3399)?
            };
            nat_add(a3400, a3401)?
        };
        nat_add(a3402, a3403)?
    };
    let a3410 = f306(f543(), {
        let a3404 = v2;
        let a3405 = 16u64;
        let a3406 = 0u64;
        nat_quot(a3404, a3405, a3406)
    });
    let a3411 = f306(f543(), {
        let a3407 = v2;
        let a3408 = 16u64;
        let a3409 = 0u64;
        nat_rem(a3407, a3408, a3409)
    });
    Ok(append_bytes(a3410, a3411))
}

pub fn f499(v0: Fn21, p3415: List<List<(Bytes, Bytes)>>, p3416: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3414 = (p3415, p3416);
    {
        let mut m3414 = m3414;
        loop {
            let step = {
                let (v1, v2) = m3414;
                let m3412 = v2;
                match m3412.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3413: Fn21 = v0.clone();
                            c3413.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f500(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f501(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3418 = {
        let a3417 = v0.clone();
        length_list(a3417)
    };
    let a3419 = 1u64;
    if nat_le(a3418, a3419) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f544(v0)?))
    }
}

pub fn f502(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3420, _) = v0;
        h3420
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3421) = v0;
        h3421
    };
    let m3422 = v1;
    match m3422.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3423 = v4;
            match m3423.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f545(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f503(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3424 = f348(v0, v1);
    let a3425 = f472(v0, v2);
    fixed_u32::bit_or(a3424, a3425)
}

pub fn f504(v0: u32) -> u32 {
    let a3428 = {
        let a3426 = f503(v0, 17u32, 15u32);
        let a3427 = f503(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3426, a3427)
    };
    let a3429 = f348(v0, 10u32);
    fixed_u32::bit_xor(a3428, a3429)
}

pub fn f505(v0: u32) -> u32 {
    let a3432 = {
        let a3430 = f503(v0, 7u32, 25u32);
        let a3431 = f503(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3430, a3431)
    };
    let a3433 = f348(v0, 3u32);
    fixed_u32::bit_xor(a3432, a3433)
}

pub fn f506() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x00]), List::cons(Bytes::lit(&[0x01]), List::cons(Bytes::lit(&[0x02]), List::cons(Bytes::lit(&[0x03]), List::cons(Bytes::lit(&[0x04]), List::cons(Bytes::lit(&[0x05]), List::cons(Bytes::lit(&[0x06]), List::cons(Bytes::lit(&[0x07]), List::cons(Bytes::lit(&[0x08]), List::cons(Bytes::lit(&[0x09]), List::cons(Bytes::lit(&[0x0a]), List::cons(Bytes::lit(&[0x0b]), List::cons(Bytes::lit(&[0x0c]), List::cons(Bytes::lit(&[0x0d]), List::cons(Bytes::lit(&[0x0e]), List::cons(Bytes::lit(&[0x0f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f507() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x10]), List::cons(Bytes::lit(&[0x11]), List::cons(Bytes::lit(&[0x12]), List::cons(Bytes::lit(&[0x13]), List::cons(Bytes::lit(&[0x14]), List::cons(Bytes::lit(&[0x15]), List::cons(Bytes::lit(&[0x16]), List::cons(Bytes::lit(&[0x17]), List::cons(Bytes::lit(&[0x18]), List::cons(Bytes::lit(&[0x19]), List::cons(Bytes::lit(&[0x1a]), List::cons(Bytes::lit(&[0x1b]), List::cons(Bytes::lit(&[0x1c]), List::cons(Bytes::lit(&[0x1d]), List::cons(Bytes::lit(&[0x1e]), List::cons(Bytes::lit(&[0x1f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f508() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x20]), List::cons(Bytes::lit(&[0x21]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x27]), List::cons(Bytes::lit(&[0x28]), List::cons(Bytes::lit(&[0x29]), List::cons(Bytes::lit(&[0x2a]), List::cons(Bytes::lit(&[0x2b]), List::cons(Bytes::lit(&[0x2c]), List::cons(Bytes::lit(&[0x2d]), List::cons(Bytes::lit(&[0x2e]), List::cons(Bytes::lit(&[0x2f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f509() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x30]), List::cons(Bytes::lit(&[0x31]), List::cons(Bytes::lit(&[0x32]), List::cons(Bytes::lit(&[0x33]), List::cons(Bytes::lit(&[0x34]), List::cons(Bytes::lit(&[0x35]), List::cons(Bytes::lit(&[0x36]), List::cons(Bytes::lit(&[0x37]), List::cons(Bytes::lit(&[0x38]), List::cons(Bytes::lit(&[0x39]), List::cons(Bytes::lit(&[0x3a]), List::cons(Bytes::lit(&[0x3b]), List::cons(Bytes::lit(&[0x3c]), List::cons(Bytes::lit(&[0x3d]), List::cons(Bytes::lit(&[0x3e]), List::cons(Bytes::lit(&[0x3f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f510() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x40]), List::cons(Bytes::lit(&[0x41]), List::cons(Bytes::lit(&[0x42]), List::cons(Bytes::lit(&[0x43]), List::cons(Bytes::lit(&[0x44]), List::cons(Bytes::lit(&[0x45]), List::cons(Bytes::lit(&[0x46]), List::cons(Bytes::lit(&[0x47]), List::cons(Bytes::lit(&[0x48]), List::cons(Bytes::lit(&[0x49]), List::cons(Bytes::lit(&[0x4a]), List::cons(Bytes::lit(&[0x4b]), List::cons(Bytes::lit(&[0x4c]), List::cons(Bytes::lit(&[0x4d]), List::cons(Bytes::lit(&[0x4e]), List::cons(Bytes::lit(&[0x4f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f511() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x50]), List::cons(Bytes::lit(&[0x51]), List::cons(Bytes::lit(&[0x52]), List::cons(Bytes::lit(&[0x53]), List::cons(Bytes::lit(&[0x54]), List::cons(Bytes::lit(&[0x55]), List::cons(Bytes::lit(&[0x56]), List::cons(Bytes::lit(&[0x57]), List::cons(Bytes::lit(&[0x58]), List::cons(Bytes::lit(&[0x59]), List::cons(Bytes::lit(&[0x5a]), List::cons(Bytes::lit(&[0x5b]), List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x5d]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f512() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x60]), List::cons(Bytes::lit(&[0x61]), List::cons(Bytes::lit(&[0x62]), List::cons(Bytes::lit(&[0x63]), List::cons(Bytes::lit(&[0x64]), List::cons(Bytes::lit(&[0x65]), List::cons(Bytes::lit(&[0x66]), List::cons(Bytes::lit(&[0x67]), List::cons(Bytes::lit(&[0x68]), List::cons(Bytes::lit(&[0x69]), List::cons(Bytes::lit(&[0x6a]), List::cons(Bytes::lit(&[0x6b]), List::cons(Bytes::lit(&[0x6c]), List::cons(Bytes::lit(&[0x6d]), List::cons(Bytes::lit(&[0x6e]), List::cons(Bytes::lit(&[0x6f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f513() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x70]), List::cons(Bytes::lit(&[0x71]), List::cons(Bytes::lit(&[0x72]), List::cons(Bytes::lit(&[0x73]), List::cons(Bytes::lit(&[0x74]), List::cons(Bytes::lit(&[0x75]), List::cons(Bytes::lit(&[0x76]), List::cons(Bytes::lit(&[0x77]), List::cons(Bytes::lit(&[0x78]), List::cons(Bytes::lit(&[0x79]), List::cons(Bytes::lit(&[0x7a]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7c]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x7f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f514() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x80]), List::cons(Bytes::lit(&[0x81]), List::cons(Bytes::lit(&[0x82]), List::cons(Bytes::lit(&[0x83]), List::cons(Bytes::lit(&[0x84]), List::cons(Bytes::lit(&[0x85]), List::cons(Bytes::lit(&[0x86]), List::cons(Bytes::lit(&[0x87]), List::cons(Bytes::lit(&[0x88]), List::cons(Bytes::lit(&[0x89]), List::cons(Bytes::lit(&[0x8a]), List::cons(Bytes::lit(&[0x8b]), List::cons(Bytes::lit(&[0x8c]), List::cons(Bytes::lit(&[0x8d]), List::cons(Bytes::lit(&[0x8e]), List::cons(Bytes::lit(&[0x8f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f515() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x90]), List::cons(Bytes::lit(&[0x91]), List::cons(Bytes::lit(&[0x92]), List::cons(Bytes::lit(&[0x93]), List::cons(Bytes::lit(&[0x94]), List::cons(Bytes::lit(&[0x95]), List::cons(Bytes::lit(&[0x96]), List::cons(Bytes::lit(&[0x97]), List::cons(Bytes::lit(&[0x98]), List::cons(Bytes::lit(&[0x99]), List::cons(Bytes::lit(&[0x9a]), List::cons(Bytes::lit(&[0x9b]), List::cons(Bytes::lit(&[0x9c]), List::cons(Bytes::lit(&[0x9d]), List::cons(Bytes::lit(&[0x9e]), List::cons(Bytes::lit(&[0x9f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f516() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xa0]), List::cons(Bytes::lit(&[0xa1]), List::cons(Bytes::lit(&[0xa2]), List::cons(Bytes::lit(&[0xa3]), List::cons(Bytes::lit(&[0xa4]), List::cons(Bytes::lit(&[0xa5]), List::cons(Bytes::lit(&[0xa6]), List::cons(Bytes::lit(&[0xa7]), List::cons(Bytes::lit(&[0xa8]), List::cons(Bytes::lit(&[0xa9]), List::cons(Bytes::lit(&[0xaa]), List::cons(Bytes::lit(&[0xab]), List::cons(Bytes::lit(&[0xac]), List::cons(Bytes::lit(&[0xad]), List::cons(Bytes::lit(&[0xae]), List::cons(Bytes::lit(&[0xaf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f517() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xb0]), List::cons(Bytes::lit(&[0xb1]), List::cons(Bytes::lit(&[0xb2]), List::cons(Bytes::lit(&[0xb3]), List::cons(Bytes::lit(&[0xb4]), List::cons(Bytes::lit(&[0xb5]), List::cons(Bytes::lit(&[0xb6]), List::cons(Bytes::lit(&[0xb7]), List::cons(Bytes::lit(&[0xb8]), List::cons(Bytes::lit(&[0xb9]), List::cons(Bytes::lit(&[0xba]), List::cons(Bytes::lit(&[0xbb]), List::cons(Bytes::lit(&[0xbc]), List::cons(Bytes::lit(&[0xbd]), List::cons(Bytes::lit(&[0xbe]), List::cons(Bytes::lit(&[0xbf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f518() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xc0]), List::cons(Bytes::lit(&[0xc1]), List::cons(Bytes::lit(&[0xc2]), List::cons(Bytes::lit(&[0xc3]), List::cons(Bytes::lit(&[0xc4]), List::cons(Bytes::lit(&[0xc5]), List::cons(Bytes::lit(&[0xc6]), List::cons(Bytes::lit(&[0xc7]), List::cons(Bytes::lit(&[0xc8]), List::cons(Bytes::lit(&[0xc9]), List::cons(Bytes::lit(&[0xca]), List::cons(Bytes::lit(&[0xcb]), List::cons(Bytes::lit(&[0xcc]), List::cons(Bytes::lit(&[0xcd]), List::cons(Bytes::lit(&[0xce]), List::cons(Bytes::lit(&[0xcf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f519() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xd0]), List::cons(Bytes::lit(&[0xd1]), List::cons(Bytes::lit(&[0xd2]), List::cons(Bytes::lit(&[0xd3]), List::cons(Bytes::lit(&[0xd4]), List::cons(Bytes::lit(&[0xd5]), List::cons(Bytes::lit(&[0xd6]), List::cons(Bytes::lit(&[0xd7]), List::cons(Bytes::lit(&[0xd8]), List::cons(Bytes::lit(&[0xd9]), List::cons(Bytes::lit(&[0xda]), List::cons(Bytes::lit(&[0xdb]), List::cons(Bytes::lit(&[0xdc]), List::cons(Bytes::lit(&[0xdd]), List::cons(Bytes::lit(&[0xde]), List::cons(Bytes::lit(&[0xdf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f520() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xe0]), List::cons(Bytes::lit(&[0xe1]), List::cons(Bytes::lit(&[0xe2]), List::cons(Bytes::lit(&[0xe3]), List::cons(Bytes::lit(&[0xe4]), List::cons(Bytes::lit(&[0xe5]), List::cons(Bytes::lit(&[0xe6]), List::cons(Bytes::lit(&[0xe7]), List::cons(Bytes::lit(&[0xe8]), List::cons(Bytes::lit(&[0xe9]), List::cons(Bytes::lit(&[0xea]), List::cons(Bytes::lit(&[0xeb]), List::cons(Bytes::lit(&[0xec]), List::cons(Bytes::lit(&[0xed]), List::cons(Bytes::lit(&[0xee]), List::cons(Bytes::lit(&[0xef]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f521() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xf0]), List::cons(Bytes::lit(&[0xf1]), List::cons(Bytes::lit(&[0xf2]), List::cons(Bytes::lit(&[0xf3]), List::cons(Bytes::lit(&[0xf4]), List::cons(Bytes::lit(&[0xf5]), List::cons(Bytes::lit(&[0xf6]), List::cons(Bytes::lit(&[0xf7]), List::cons(Bytes::lit(&[0xf8]), List::cons(Bytes::lit(&[0xf9]), List::cons(Bytes::lit(&[0xfa]), List::cons(Bytes::lit(&[0xfb]), List::cons(Bytes::lit(&[0xfc]), List::cons(Bytes::lit(&[0xfd]), List::cons(Bytes::lit(&[0xfe]), List::cons(Bytes::lit(&[0xff]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f522(v0: Bytes, v1: u64) -> R<Bytes> {
    let a3436 = f32(v0.clone(), {
        let a3434 = v1;
        let a3435 = 2u64;
        nat_add(a3434, a3435)?
    }, 2u64);
    let a3437 = Bytes::lit(&[0x30, 0x30]);
    if equal(a3436, a3437) {
        let m3440 = f546(f32(v0, {
            let a3438 = v1;
            let a3439 = 4u64;
            nat_add(a3438, a3439)?
        }, 2u64))?;
        match m3440 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f302(v2.clone(), 0u64)?;
                let a3441 = v3;
                let a3442 = 32u64;
                if nat_lt(a3441, a3442) {
                    if f547(v3) {
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

pub fn f523(v0: u64) -> Bytes {
    let a3443 = v0;
    let a3444 = 34u64;
    if nat_eq(a3443, a3444) {
        Bytes::lit(&[0x22])
    } else {
        let a3445 = v0;
        let a3446 = 92u64;
        if nat_eq(a3445, a3446) {
            Bytes::lit(&[0x5c])
        } else {
            let a3447 = v0;
            let a3448 = 98u64;
            if nat_eq(a3447, a3448) {
                Bytes::lit(&[0x08])
            } else {
                let a3449 = v0;
                let a3450 = 102u64;
                if nat_eq(a3449, a3450) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a3451 = v0;
                    let a3452 = 110u64;
                    if nat_eq(a3451, a3452) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a3453 = v0;
                        let a3454 = 114u64;
                        if nat_eq(a3453, a3454) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a3455 = v0;
                            let a3456 = 116u64;
                            if nat_eq(a3455, a3456) {
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

pub fn f524(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3459 = {
        let a3457 = v0;
        let a3458 = v1;
        index_bytes(a3457, a3458)
    };
    match m3459 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f548(v2)? {
                Ok(Some({
                    let a3460 = v1;
                    let a3461 = 1u64;
                    nat_add(a3460, a3461)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f525(v0: u8) -> R<u64> {
    let a3508 = {
        let v1: u8 = v0;
        let a3506 = {
            let a3482 = {
                let a3470 = {
                    let a3464 = {
                        let a3462 = v1;
                        let a3463 = 128u8;
                        fixed_u8::bit_and(a3462, a3463)
                    };
                    let a3465 = 128u8;
                    if equal(a3464, a3465) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3471 = {
                    let a3468 = {
                        let a3466 = v1;
                        let a3467 = 64u8;
                        fixed_u8::bit_and(a3466, a3467)
                    };
                    let a3469 = 64u8;
                    if equal(a3468, a3469) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3470, a3471)?
            };
            let a3483 = {
                let a3480 = {
                    let a3474 = {
                        let a3472 = v1;
                        let a3473 = 32u8;
                        fixed_u8::bit_and(a3472, a3473)
                    };
                    let a3475 = 32u8;
                    if equal(a3474, a3475) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3481 = {
                    let a3478 = {
                        let a3476 = v1;
                        let a3477 = 16u8;
                        fixed_u8::bit_and(a3476, a3477)
                    };
                    let a3479 = 16u8;
                    if equal(a3478, a3479) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3480, a3481)?
            };
            nat_add(a3482, a3483)?
        };
        let a3507 = {
            let a3504 = {
                let a3492 = {
                    let a3486 = {
                        let a3484 = v1;
                        let a3485 = 8u8;
                        fixed_u8::bit_and(a3484, a3485)
                    };
                    let a3487 = 8u8;
                    if equal(a3486, a3487) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3493 = {
                    let a3490 = {
                        let a3488 = v1;
                        let a3489 = 4u8;
                        fixed_u8::bit_and(a3488, a3489)
                    };
                    let a3491 = 4u8;
                    if equal(a3490, a3491) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3492, a3493)?
            };
            let a3505 = {
                let a3502 = {
                    let a3496 = {
                        let a3494 = v1;
                        let a3495 = 2u8;
                        fixed_u8::bit_and(a3494, a3495)
                    };
                    let a3497 = 2u8;
                    if equal(a3496, a3497) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3503 = {
                    let a3500 = {
                        let a3498 = v1;
                        let a3499 = 1u8;
                        fixed_u8::bit_and(a3498, a3499)
                    };
                    let a3501 = 1u8;
                    if equal(a3500, a3501) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3502, a3503)?
            };
            nat_add(a3504, a3505)?
        };
        nat_add(a3506, a3507)?
    };
    let a3509 = 48u64;
    Ok(nat_sub(a3508, a3509))
}

pub fn f526(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3510 = v0.clone();
        length_bytes(a3510)
    };
    let a3511 = v1;
    let a3512 = v2;
    if if nat_lt(a3511, a3512) {
        let a3517 = f302(v0, {
            let a3515 = v2;
            let a3516 = {
                let a3513 = v1;
                let a3514 = 1u64;
                nat_add(a3513, a3514)?
            };
            nat_sub(a3515, a3516)
        })?;
        let a3518 = 48u64;
        nat_eq(a3517, a3518)
    } else {
        false
    } {
        Ok(Some({
            let a3519 = v1;
            let a3520 = 1u64;
            nat_add(a3519, a3520)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f527(v0: bool, v1: List<Adt14>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3521 = v1;
    match m3521.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3522 = f549(v0, v3.clone())?;
            match m3522 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f550(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f86(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3523 = f527(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3523 {
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

pub fn f528(p3527: List<Bytes>) -> R<bool> {
    let m3526 = p3527;
    {
        let mut m3526 = m3526;
        loop {
            let step = {
                let v0 = m3526;
                let m3524 = v0;
                match m3524.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3525 = f551(v1)?;
                        match m3525 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f86(v3, f552(false, v2.clone())?) {
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
                Ok(next) => { m3526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(v0: List<(Bytes, Adt14)>, v1: List<Adt14>) -> List<Bytes> {
    let m3528 = v0;
    match m3528.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f553(v1.clone(), {
                let (h3529, _) = v2.clone();
                h3529
            }) {
                List::cons({
                    let (h3530, _) = v2;
                    h3530
                }, f529(v3, v1))
            } else {
                f529(v3, v1)
            }
        }
    }
}

pub fn f530(v0: bool, v1: Bytes, v2: List<Adt14>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt24>), Bytes>> {
    let m3531 = v2;
    match m3531.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt24>), Bytes>((List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v4, v5)) => {
            let m3532 = f554(v4.clone())?;
            match m3532 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3533 = f555(v0, v6.clone())?;
                    match m3533 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f86(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3536 = f556(v4, {
                                    let a3534 = v1.clone();
                                    let a3535 = f234(v6)?;
                                    append_bytes(a3534, a3535)
                                })?;
                                match m3536 {
                                    Ok(v9) => {
                                        let m3537 = f530(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3537 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt24>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h3538, _) = v9;
                                                    h3538
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h3539, _) = v11;
                                                    h3539
                                                }), f375({
                                                    let (_, h3540) = v9;
                                                    h3540
                                                }, {
                                                    let (_, h3541) = v11;
                                                    h3541
                                                }))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(v8))
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

pub fn f531(v0: List<Adt14>) -> R<List<Bytes>> {
    let m3542 = v0;
    match m3542.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3543 = f554(v1)?;
            match m3543 {
                None => {
                    f531(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f531(v2)?))
                }
            }
        }
    }
}

pub fn f532(v0: Bytes, v1: List<Adt14>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt24>), Bytes>> {
    f557(f528(f558(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f533(v0: Adt14, v1: Bytes) -> R<bool> {
    let m3544 = f155(Some(v0));
    match m3544 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3545 = f483(v2)?;
            match m3545 {
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

pub fn f534(v0: bool, v1: Bytes, v2: List<(Bytes, Adt14)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt24>), Bytes>> {
    let m3546 = v2;
    match m3546.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt24>), Bytes>((List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v5, v6)) => {
            let m3548 = f555(v0, {
                let (h3547, _) = v5.clone();
                h3547
            })?;
            match m3548 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3549, _) = v5;
                        h3549
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f86(v7.clone(), f552(v0, f535(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3554 = f491({
                            let (_, h3550) = v5.clone();
                            h3550
                        }, {
                            let a3552 = v1.clone();
                            let a3553 = f234({
                                let (h3551, _) = v5.clone();
                                h3551
                            })?;
                            append_bytes(a3552, a3553)
                        })?;
                        match m3554 {
                            Ok(v9) => {
                                let v10: Bytes = if f86({
                                    let (h3555, _) = v5;
                                    h3555
                                }, v3.clone()) {
                                    let (h3556, _) = v9;
                                    h3556
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h3557, _) = v9;
                                        h3557
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3558 = f534(v0, v1, v6, v3, v4)?;
                                match m3558 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt24>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h3559, _) = v12;
                                            h3559
                                        }), f375({
                                            let (_, h3560) = v9;
                                            h3560
                                        }, {
                                            let (_, h3561) = v12;
                                            h3561
                                        }))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f535(v0: List<(Bytes, Adt14)>) -> List<Bytes> {
    let m3562 = v0;
    match m3562.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h3563, _) = v1;
                h3563
            }, f535(v2))
        }
    }
}

pub fn f536(v0: Adt14) -> Option<Adt14> {
    let m3564 = f151(f147(v0, f8(Str::lit("oneOf"))));
    match m3564.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m3565 = v2;
            match m3565.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m3566 = v4;
                    match m3566.uncons() {
                        None => {
                            if f559(v3.clone()) {
                                Some(v1)
                            } else {
                                if f559(v1) {
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

pub fn f537(v0: Bytes, v1: Result<(Bytes, List<Adt24>), Bytes>) -> Result<(Bytes, List<Adt24>), Bytes> {
    let m3567 = v1;
    match m3567 {
        Ok(v3) => {
            f486(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h3568, _) = v3;
                h3568
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h3569) = v3;
                h3569
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt24>), Bytes>(v2)
        }
    }
}

pub fn f538(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt24>), Bytes>> {
    let m3570 = f536(v0.clone());
    match m3570 {
        None => {
            if f36(f150(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3571 = f147(v0, f8(Str::lit("items")));
                match m3571 {
                    None => {
                        Ok(f537(f8(Str::lit("List")), f539()))
                    }
                    Some(v3) => {
                        Ok(f537(f8(Str::lit("List")), f556(v3, {
                            let a3572 = v1;
                            let a3573 = f8(Str::lit("Item"));
                            append_bytes(a3572, a3573)
                        })?))
                    }
                }
            } else {
                f556(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f537(f8(Str::lit("Option")), f556(v2, v1)?))
        }
    }
}

pub fn f539() -> Result<(Bytes, List<Adt24>), Bytes> {
    Ok::<(Bytes, List<Adt24>), Bytes>((f417(), List::cons(Adt24::C2, List::<Adt24>::nil())))
}

pub fn f540(p3577: List<(Bytes, Adt14)>, v1: List<Bytes>) -> bool {
    let m3576 = p3577;
    {
        let mut m3576 = m3576;
        loop {
            let step = {
                let v0 = m3576;
                let m3574 = v0;
                match m3574.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f86({
                            let (h3575, _) = v2;
                            h3575
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt14)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt14)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f541(p3581: List<Adt14>, v1: Bytes, p3582: List<Bytes>) -> bool {
    let m3580 = (p3581, p3582);
    {
        let mut m3580 = m3580;
        loop {
            let step = {
                let (v0, v2) = m3580;
                let m3578 = v0;
                match m3578.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3579 = f560(v3, v1.clone());
                        match m3579 {
                            None => {
                                Err::<(List<Adt14>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f86(v5.clone(), v2.clone()) {
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
                Ok(next) => { m3580 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f542(p3585: List<Adt14>, p3586: List<Bytes>) -> bool {
    let m3584 = (p3585, p3586);
    {
        let mut m3584 = m3584;
        loop {
            let step = {
                let (v0, v1) = m3584;
                let m3583 = v0;
                match m3583.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f150(v2, f8(Str::lit("type")));
                        if !f36(v4.clone(), Bytes::lit(&[])) {
                            if !f86(v4.clone(), v1.clone()) {
                                if !if if f36(v4.clone(), f8(Str::lit("integer"))) {
                                    f86(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f36(v4.clone(), f8(Str::lit("number"))) {
                                        f86(f8(Str::lit("integer")), v1.clone())
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
                Ok(next) => { m3584 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f543() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f544(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f457({
        let (_, h3587) = {
            let (h3588, _) = f561(Fn22::F562, {
                let a3590 = {
                    let a3589 = v0.clone();
                    length_list(a3589)
                };
                let a3591 = 1u64;
                nat_add(a3590, a3591)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3588
        };
        h3587
    }))
}

pub fn f545(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3596 = {
            let a3594 = {
                let a3592 = v0.clone();
                length_list(a3592)
            };
            let a3595 = {
                let a3593 = v1.clone();
                length_list(a3593)
            };
            nat_add(a3594, a3595)?
        };
        let a3597 = 1u64;
        nat_add(a3596, a3597)?
    };
    Ok(f338({
        let (_, h3598) = {
            let (_, h3599) = {
                let (h3600, _) = f563(Fn23::F564, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3600
            };
            h3599
        };
        h3598
    }))
}

pub fn f546(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3601 = v0.clone();
        length_bytes(a3601)
    };
    let a3605 = {
        let a3602 = v1;
        let a3603 = 2u64;
        let a3604 = 0u64;
        nat_rem(a3602, a3603, a3604)
    };
    let a3606 = 0u64;
    if nat_eq(a3605, a3606) {
        let (_, h3607) = {
            let (h3608, _) = f565(Fn24::F566(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3608
        };
        Ok(h3607)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f547(v0: u64) -> bool {
    let a3609 = v0;
    let a3610 = 8u64;
    if nat_eq(a3609, a3610) {
        true
    } else {
        let a3611 = v0;
        let a3612 = 9u64;
        if nat_eq(a3611, a3612) {
            true
        } else {
            let a3613 = v0;
            let a3614 = 10u64;
            if nat_eq(a3613, a3614) {
                true
            } else {
                let a3615 = v0;
                let a3616 = 12u64;
                if nat_eq(a3615, a3616) {
                    true
                } else {
                    let a3617 = v0;
                    let a3618 = 13u64;
                    nat_eq(a3617, a3618)
                }
            }
        }
    }
}

pub fn f548(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3663 = {
            let a3639 = {
                let a3627 = {
                    let a3621 = {
                        let a3619 = v1;
                        let a3620 = 128u8;
                        fixed_u8::bit_and(a3619, a3620)
                    };
                    let a3622 = 128u8;
                    if equal(a3621, a3622) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3628 = {
                    let a3625 = {
                        let a3623 = v1;
                        let a3624 = 64u8;
                        fixed_u8::bit_and(a3623, a3624)
                    };
                    let a3626 = 64u8;
                    if equal(a3625, a3626) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3627, a3628)?
            };
            let a3640 = {
                let a3637 = {
                    let a3631 = {
                        let a3629 = v1;
                        let a3630 = 32u8;
                        fixed_u8::bit_and(a3629, a3630)
                    };
                    let a3632 = 32u8;
                    if equal(a3631, a3632) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3638 = {
                    let a3635 = {
                        let a3633 = v1;
                        let a3634 = 16u8;
                        fixed_u8::bit_and(a3633, a3634)
                    };
                    let a3636 = 16u8;
                    if equal(a3635, a3636) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3637, a3638)?
            };
            nat_add(a3639, a3640)?
        };
        let a3664 = {
            let a3661 = {
                let a3649 = {
                    let a3643 = {
                        let a3641 = v1;
                        let a3642 = 8u8;
                        fixed_u8::bit_and(a3641, a3642)
                    };
                    let a3644 = 8u8;
                    if equal(a3643, a3644) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3650 = {
                    let a3647 = {
                        let a3645 = v1;
                        let a3646 = 4u8;
                        fixed_u8::bit_and(a3645, a3646)
                    };
                    let a3648 = 4u8;
                    if equal(a3647, a3648) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3649, a3650)?
            };
            let a3662 = {
                let a3659 = {
                    let a3653 = {
                        let a3651 = v1;
                        let a3652 = 2u8;
                        fixed_u8::bit_and(a3651, a3652)
                    };
                    let a3654 = 2u8;
                    if equal(a3653, a3654) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3660 = {
                    let a3657 = {
                        let a3655 = v1;
                        let a3656 = 1u8;
                        fixed_u8::bit_and(a3655, a3656)
                    };
                    let a3658 = 1u8;
                    if equal(a3657, a3658) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3659, a3660)?
            };
            nat_add(a3661, a3662)?
        };
        nat_add(a3663, a3664)?
    };
    let a3665 = 48u64;
    let a3666 = v2;
    if nat_le(a3665, a3666) {
        let a3667 = v2;
        let a3668 = 57u64;
        Ok(nat_le(a3667, a3668))
    } else {
        Ok(false)
    }
}

pub fn f549(v0: bool, v1: Adt14) -> R<Option<Bytes>> {
    let m3669 = v1;
    match m3669 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v3, v4) => {
            Ok(Some({
                let a3672 = if if v3 {
                    let a3670 = 0u64;
                    let a3671 = v4;
                    nat_lt(a3670, a3671)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3673 = f318(false, v4)?;
                append_bytes(a3672, a3673)
            }))
        }
        Adt14::C2(v2) => {
            f555(v0, v2)
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

pub fn f550(v0: Adt14) -> R<Bytes> {
    f237(v0)
}

pub fn f551(v0: Bytes) -> R<Option<Bytes>> {
    if f567({
        let a3674 = v0.clone();
        length_bytes(a3674)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f568(v0)?;
        if f569(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3677 = {
                let a3675 = v1.clone();
                let a3676 = 0u64;
                index_bytes(a3675, a3676)
            };
            match m3677 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f369({
                        let v3: u8 = v2;
                        let a3722 = {
                            let a3698 = {
                                let a3686 = {
                                    let a3680 = {
                                        let a3678 = v3;
                                        let a3679 = 128u8;
                                        fixed_u8::bit_and(a3678, a3679)
                                    };
                                    let a3681 = 128u8;
                                    if equal(a3680, a3681) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3687 = {
                                    let a3684 = {
                                        let a3682 = v3;
                                        let a3683 = 64u8;
                                        fixed_u8::bit_and(a3682, a3683)
                                    };
                                    let a3685 = 64u8;
                                    if equal(a3684, a3685) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3686, a3687)?
                            };
                            let a3699 = {
                                let a3696 = {
                                    let a3690 = {
                                        let a3688 = v3;
                                        let a3689 = 32u8;
                                        fixed_u8::bit_and(a3688, a3689)
                                    };
                                    let a3691 = 32u8;
                                    if equal(a3690, a3691) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3697 = {
                                    let a3694 = {
                                        let a3692 = v3;
                                        let a3693 = 16u8;
                                        fixed_u8::bit_and(a3692, a3693)
                                    };
                                    let a3695 = 16u8;
                                    if equal(a3694, a3695) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3696, a3697)?
                            };
                            nat_add(a3698, a3699)?
                        };
                        let a3723 = {
                            let a3720 = {
                                let a3708 = {
                                    let a3702 = {
                                        let a3700 = v3;
                                        let a3701 = 8u8;
                                        fixed_u8::bit_and(a3700, a3701)
                                    };
                                    let a3703 = 8u8;
                                    if equal(a3702, a3703) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3709 = {
                                    let a3706 = {
                                        let a3704 = v3;
                                        let a3705 = 4u8;
                                        fixed_u8::bit_and(a3704, a3705)
                                    };
                                    let a3707 = 4u8;
                                    if equal(a3706, a3707) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3708, a3709)?
                            };
                            let a3721 = {
                                let a3718 = {
                                    let a3712 = {
                                        let a3710 = v3;
                                        let a3711 = 2u8;
                                        fixed_u8::bit_and(a3710, a3711)
                                    };
                                    let a3713 = 2u8;
                                    if equal(a3712, a3713) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3719 = {
                                    let a3716 = {
                                        let a3714 = v3;
                                        let a3715 = 1u8;
                                        fixed_u8::bit_and(a3714, a3715)
                                    };
                                    let a3717 = 1u8;
                                    if equal(a3716, a3717) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3718, a3719)?
                            };
                            nat_add(a3720, a3721)?
                        };
                        nat_add(a3722, a3723)?
                    }) {
                        Ok(Some({
                            let a3724 = f8(Str::lit("n"));
                            let a3725 = v1;
                            append_bytes(a3724, a3725)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f570({
            let a3726 = v0.clone();
            length_bytes(a3726)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f552(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3727 = v1;
    match m3727.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3728 = f555(v0, v2)?;
            match m3728 {
                None => {
                    f552(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f552(v0, v3)?))
                }
            }
        }
    }
}

pub fn f553(p3732: List<Adt14>, v1: Bytes) -> bool {
    let m3731 = p3732;
    {
        let mut m3731 = m3731;
        loop {
            let step = {
                let v0 = m3731;
                let m3729 = v0;
                match m3729.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3730 = f560(v2, v1.clone());
                        if match m3730 {
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
                Ok(next) => { m3731 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: Adt14) -> R<Option<Bytes>> {
    let m3733 = f155(f147(v0.clone(), f8(Str::lit("$ref"))));
    match m3733 {
        None => {
            Ok(f155(f147(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f483(v1)
        }
    }
}

pub fn f555(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f571(v1)
    } else {
        f551(v1)
    }
}

pub fn f556(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt24>), Bytes>> {
    f572(f455(v0), v1)
}

pub fn f557(v0: bool, v1: Bytes, v2: List<Adt14>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt24>), Bytes>> {
    let m3734 = v2;
    match m3734.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt24>), Bytes>((List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v5, v6)) => {
            let m3735 = f560(v5.clone(), v3.clone());
            match m3735 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3736 = f555(v0, v7.clone())?;
                    match m3736 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f86(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3739 = f489({
                                    let a3737 = v1.clone();
                                    let a3738 = f234(v7)?;
                                    append_bytes(a3737, a3738)
                                }, f416(f371(f147(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f74(f151(f147(v5, f8(Str::lit("required"))))), true)?;
                                match m3739 {
                                    Ok(v10) => {
                                        let m3740 = f557(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3740 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt24>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f209({
                                                    let (h3741, _) = v10;
                                                    h3741
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h3742, _) = v12;
                                                    h3742
                                                }), f375({
                                                    let (_, h3743) = v10;
                                                    h3743
                                                }, {
                                                    let (_, h3744) = v12;
                                                    h3744
                                                }))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(List<Bytes>, List<Adt24>), Bytes>(v9))
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

pub fn f558(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m3745 = v0;
    match m3745.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3746 = f560(v2, v1.clone());
            match m3746 {
                None => {
                    f558(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f558(v3, v1))
                }
            }
        }
    }
}

pub fn f559(v0: Adt14) -> bool {
    f36(f150(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f560(v0: Adt14, v1: Bytes) -> Option<Bytes> {
    if f86(v1.clone(), f74(f151(f147(v0.clone(), f8(Str::lit("required")))))) {
        let m3747 = f147(v0, f8(Str::lit("properties")));
        match m3747 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3748 = f147(v2, v1);
                match m3748 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f155(f147(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f561(v0: Fn22, p3753: u64, p3754: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3752 = (p3753, p3754);
    {
        let mut m3752 = m3752;
        loop {
            let step = {
                let (v1, v2) = m3752;
                let m3749 = v1;
                if m3749 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3749 - 1;
                    let m3751 = {
                        let c3750: Fn22 = v0.clone();
                        c3750.apply(v2.clone())?
                    };
                    match m3751 {
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
                Ok(next) => { m3752 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f562(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f573(v0)
}

pub fn f563(v0: Fn23, p3759: u64, p3760: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3758 = (p3759, p3760);
    {
        let mut m3758 = m3758;
        loop {
            let step = {
                let (v1, v2) = m3758;
                let m3755 = v1;
                if m3755 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3755 - 1;
                    let m3757 = {
                        let c3756: Fn23 = v0.clone();
                        c3756.apply(v2.clone())
                    };
                    match m3757 {
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
                Ok(next) => { m3758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f564(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f574(v0)
}

pub fn f565(v0: Fn24, p3765: u64, p3766: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3764 = (p3765, p3766);
    {
        let mut m3764 = m3764;
        loop {
            let step = {
                let (v1, v2) = m3764;
                let m3761 = v1;
                if m3761 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3761 - 1;
                    let m3763 = {
                        let c3762: Fn24 = v0.clone();
                        c3762.apply(v2.clone())?
                    };
                    match m3763 {
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
                Ok(next) => { m3764 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f566(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f575(v0, v1)
}

pub fn f567(p3822: u64, v1: Bytes, p3823: u64) -> R<bool> {
    let m3821 = (p3822, p3823);
    {
        let mut m3821 = m3821;
        loop {
            let step = {
                let (v0, v2) = m3821;
                let m3767 = v0;
                if m3767 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3767 - 1;
                    let m3770 = {
                        let a3768 = v1.clone();
                        let a3769 = v2;
                        index_bytes(a3768, a3769)
                    };
                    match m3770 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3817 = {
                                let v5: u8 = v4;
                                let a3815 = {
                                    let a3791 = {
                                        let a3779 = {
                                            let a3773 = {
                                                let a3771 = v5;
                                                let a3772 = 128u8;
                                                fixed_u8::bit_and(a3771, a3772)
                                            };
                                            let a3774 = 128u8;
                                            if equal(a3773, a3774) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3780 = {
                                            let a3777 = {
                                                let a3775 = v5;
                                                let a3776 = 64u8;
                                                fixed_u8::bit_and(a3775, a3776)
                                            };
                                            let a3778 = 64u8;
                                            if equal(a3777, a3778) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3779, a3780)?
                                    };
                                    let a3792 = {
                                        let a3789 = {
                                            let a3783 = {
                                                let a3781 = v5;
                                                let a3782 = 32u8;
                                                fixed_u8::bit_and(a3781, a3782)
                                            };
                                            let a3784 = 32u8;
                                            if equal(a3783, a3784) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3790 = {
                                            let a3787 = {
                                                let a3785 = v5;
                                                let a3786 = 16u8;
                                                fixed_u8::bit_and(a3785, a3786)
                                            };
                                            let a3788 = 16u8;
                                            if equal(a3787, a3788) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3789, a3790)?
                                    };
                                    nat_add(a3791, a3792)?
                                };
                                let a3816 = {
                                    let a3813 = {
                                        let a3801 = {
                                            let a3795 = {
                                                let a3793 = v5;
                                                let a3794 = 8u8;
                                                fixed_u8::bit_and(a3793, a3794)
                                            };
                                            let a3796 = 8u8;
                                            if equal(a3795, a3796) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3802 = {
                                            let a3799 = {
                                                let a3797 = v5;
                                                let a3798 = 4u8;
                                                fixed_u8::bit_and(a3797, a3798)
                                            };
                                            let a3800 = 4u8;
                                            if equal(a3799, a3800) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3801, a3802)?
                                    };
                                    let a3814 = {
                                        let a3811 = {
                                            let a3805 = {
                                                let a3803 = v5;
                                                let a3804 = 2u8;
                                                fixed_u8::bit_and(a3803, a3804)
                                            };
                                            let a3806 = 2u8;
                                            if equal(a3805, a3806) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3812 = {
                                            let a3809 = {
                                                let a3807 = v5;
                                                let a3808 = 1u8;
                                                fixed_u8::bit_and(a3807, a3808)
                                            };
                                            let a3810 = 1u8;
                                            if equal(a3809, a3810) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3811, a3812)?
                                    };
                                    nat_add(a3813, a3814)?
                                };
                                nat_add(a3815, a3816)?
                            };
                            let a3818 = 128u64;
                            if nat_lt(a3817, a3818) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3819 = v2;
                                    let a3820 = 1u64;
                                    nat_add(a3819, a3820)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3821 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f568(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f314(v0.clone(), 0u64, {
        let a3824 = v0;
        length_bytes(a3824)
    }, false, Bytes::lit(&[]))?;
    let m3827 = {
        let a3825 = v1.clone();
        let a3826 = 0u64;
        index_bytes(a3825, a3826)
    };
    match m3827 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3872 = {
                    let a3848 = {
                        let a3836 = {
                            let a3830 = {
                                let a3828 = v3;
                                let a3829 = 128u8;
                                fixed_u8::bit_and(a3828, a3829)
                            };
                            let a3831 = 128u8;
                            if equal(a3830, a3831) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3837 = {
                            let a3834 = {
                                let a3832 = v3;
                                let a3833 = 64u8;
                                fixed_u8::bit_and(a3832, a3833)
                            };
                            let a3835 = 64u8;
                            if equal(a3834, a3835) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3836, a3837)?
                    };
                    let a3849 = {
                        let a3846 = {
                            let a3840 = {
                                let a3838 = v3;
                                let a3839 = 32u8;
                                fixed_u8::bit_and(a3838, a3839)
                            };
                            let a3841 = 32u8;
                            if equal(a3840, a3841) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3847 = {
                            let a3844 = {
                                let a3842 = v3;
                                let a3843 = 16u8;
                                fixed_u8::bit_and(a3842, a3843)
                            };
                            let a3845 = 16u8;
                            if equal(a3844, a3845) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3846, a3847)?
                    };
                    nat_add(a3848, a3849)?
                };
                let a3873 = {
                    let a3870 = {
                        let a3858 = {
                            let a3852 = {
                                let a3850 = v3;
                                let a3851 = 8u8;
                                fixed_u8::bit_and(a3850, a3851)
                            };
                            let a3853 = 8u8;
                            if equal(a3852, a3853) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3859 = {
                            let a3856 = {
                                let a3854 = v3;
                                let a3855 = 4u8;
                                fixed_u8::bit_and(a3854, a3855)
                            };
                            let a3857 = 4u8;
                            if equal(a3856, a3857) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3858, a3859)?
                    };
                    let a3871 = {
                        let a3868 = {
                            let a3862 = {
                                let a3860 = v3;
                                let a3861 = 2u8;
                                fixed_u8::bit_and(a3860, a3861)
                            };
                            let a3863 = 2u8;
                            if equal(a3862, a3863) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3869 = {
                            let a3866 = {
                                let a3864 = v3;
                                let a3865 = 1u8;
                                fixed_u8::bit_and(a3864, a3865)
                            };
                            let a3867 = 1u8;
                            if equal(a3866, a3867) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3868, a3869)?
                    };
                    nat_add(a3870, a3871)?
                };
                nat_add(a3872, a3873)?
            };
            if f368(v4) {
                let a3880 = f32({
                    let a3874 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3874)
                }, {
                    let a3875 = v4;
                    let a3876 = 65u64;
                    nat_sub(a3875, a3876)
                }, 1u64);
                let a3881 = f32(v1.clone(), 1u64, {
                    let a3878 = {
                        let a3877 = v1;
                        length_bytes(a3877)
                    };
                    let a3879 = 1u64;
                    nat_sub(a3878, a3879)
                });
                Ok(append_bytes(a3880, a3881))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f569(v0: Bytes) -> R<bool> {
    let m3884 = {
        let a3882 = v0;
        let a3883 = 0u64;
        index_bytes(a3882, a3883)
    };
    match m3884 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f367({
                let v2: u8 = v1;
                let a3929 = {
                    let a3905 = {
                        let a3893 = {
                            let a3887 = {
                                let a3885 = v2;
                                let a3886 = 128u8;
                                fixed_u8::bit_and(a3885, a3886)
                            };
                            let a3888 = 128u8;
                            if equal(a3887, a3888) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3894 = {
                            let a3891 = {
                                let a3889 = v2;
                                let a3890 = 64u8;
                                fixed_u8::bit_and(a3889, a3890)
                            };
                            let a3892 = 64u8;
                            if equal(a3891, a3892) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3893, a3894)?
                    };
                    let a3906 = {
                        let a3903 = {
                            let a3897 = {
                                let a3895 = v2;
                                let a3896 = 32u8;
                                fixed_u8::bit_and(a3895, a3896)
                            };
                            let a3898 = 32u8;
                            if equal(a3897, a3898) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3904 = {
                            let a3901 = {
                                let a3899 = v2;
                                let a3900 = 16u8;
                                fixed_u8::bit_and(a3899, a3900)
                            };
                            let a3902 = 16u8;
                            if equal(a3901, a3902) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3903, a3904)?
                    };
                    nat_add(a3905, a3906)?
                };
                let a3930 = {
                    let a3927 = {
                        let a3915 = {
                            let a3909 = {
                                let a3907 = v2;
                                let a3908 = 8u8;
                                fixed_u8::bit_and(a3907, a3908)
                            };
                            let a3910 = 8u8;
                            if equal(a3909, a3910) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3916 = {
                            let a3913 = {
                                let a3911 = v2;
                                let a3912 = 4u8;
                                fixed_u8::bit_and(a3911, a3912)
                            };
                            let a3914 = 4u8;
                            if equal(a3913, a3914) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3915, a3916)?
                    };
                    let a3928 = {
                        let a3925 = {
                            let a3919 = {
                                let a3917 = v2;
                                let a3918 = 2u8;
                                fixed_u8::bit_and(a3917, a3918)
                            };
                            let a3920 = 2u8;
                            if equal(a3919, a3920) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3926 = {
                            let a3923 = {
                                let a3921 = v2;
                                let a3922 = 1u8;
                                fixed_u8::bit_and(a3921, a3922)
                            };
                            let a3924 = 1u8;
                            if equal(a3923, a3924) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3925, a3926)?
                    };
                    nat_add(a3927, a3928)?
                };
                nat_add(a3929, a3930)?
            }) {
                Ok(true)
            } else {
                Ok(f368({
                    let v3: u8 = v1;
                    let a3975 = {
                        let a3951 = {
                            let a3939 = {
                                let a3933 = {
                                    let a3931 = v3;
                                    let a3932 = 128u8;
                                    fixed_u8::bit_and(a3931, a3932)
                                };
                                let a3934 = 128u8;
                                if equal(a3933, a3934) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a3940 = {
                                let a3937 = {
                                    let a3935 = v3;
                                    let a3936 = 64u8;
                                    fixed_u8::bit_and(a3935, a3936)
                                };
                                let a3938 = 64u8;
                                if equal(a3937, a3938) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3939, a3940)?
                        };
                        let a3952 = {
                            let a3949 = {
                                let a3943 = {
                                    let a3941 = v3;
                                    let a3942 = 32u8;
                                    fixed_u8::bit_and(a3941, a3942)
                                };
                                let a3944 = 32u8;
                                if equal(a3943, a3944) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a3950 = {
                                let a3947 = {
                                    let a3945 = v3;
                                    let a3946 = 16u8;
                                    fixed_u8::bit_and(a3945, a3946)
                                };
                                let a3948 = 16u8;
                                if equal(a3947, a3948) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3949, a3950)?
                        };
                        nat_add(a3951, a3952)?
                    };
                    let a3976 = {
                        let a3973 = {
                            let a3961 = {
                                let a3955 = {
                                    let a3953 = v3;
                                    let a3954 = 8u8;
                                    fixed_u8::bit_and(a3953, a3954)
                                };
                                let a3956 = 8u8;
                                if equal(a3955, a3956) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a3962 = {
                                let a3959 = {
                                    let a3957 = v3;
                                    let a3958 = 4u8;
                                    fixed_u8::bit_and(a3957, a3958)
                                };
                                let a3960 = 4u8;
                                if equal(a3959, a3960) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3961, a3962)?
                        };
                        let a3974 = {
                            let a3971 = {
                                let a3965 = {
                                    let a3963 = v3;
                                    let a3964 = 2u8;
                                    fixed_u8::bit_and(a3963, a3964)
                                };
                                let a3966 = 2u8;
                                if equal(a3965, a3966) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a3972 = {
                                let a3969 = {
                                    let a3967 = v3;
                                    let a3968 = 1u8;
                                    fixed_u8::bit_and(a3967, a3968)
                                };
                                let a3970 = 1u8;
                                if equal(a3969, a3970) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3971, a3972)?
                        };
                        nat_add(a3973, a3974)?
                    };
                    nat_add(a3975, a3976)?
                }))
            }
        }
    }
}

pub fn f570(p3989: u64, v1: Bytes, p3990: u64, p3991: Bytes) -> R<Option<Bytes>> {
    let m3988 = (p3989, (p3990, p3991));
    {
        let mut m3988 = m3988;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3988;
                let m3977 = v0;
                if m3977 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3977 - 1;
                    let a3979 = {
                        let a3978 = v1.clone();
                        length_bytes(a3978)
                    };
                    let a3980 = v2;
                    if nat_le(a3979, a3980) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3981 = f399(v1.clone(), v2)?;
                        match m3981 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3983 = v2;
                                    let a3984 = {
                                        let (_, h3982) = v5;
                                        h3982
                                    };
                                    nat_add(a3983, a3984)?
                                }, {
                                    let a3986 = v3;
                                    let a3987 = f576({
                                        let (h3985, _) = v5;
                                        h3985
                                    });
                                    append_bytes(a3986, a3987)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f571(v0: Bytes) -> R<Option<Bytes>> {
    let m3993 = f577({
        let a3992 = v0.clone();
        length_bytes(a3992)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m3993 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3995 = {
                let a3994 = v1.clone();
                length_bytes(a3994)
            };
            let a3996 = 0u64;
            if nat_eq(a3995, a3996) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f572(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt24>), Bytes>> {
    let m3997 = v0.clone();
    match m3997 {
        Adt14::C0(v2) => {
            if v2 {
                Ok(f539())
            } else {
                Ok(f485(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt14::C1(_, _) => {
            Ok(f485(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C2(_) => {
            Ok(f485(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C3(_) => {
            Ok(f485(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C4(_) => {
            let m3998 = f578(f414(v0.clone()));
            match m3998 {
                None => {
                    let m3999 = f155(f147(v0.clone(), f8(Str::lit("$ref"))));
                    match m3999 {
                        None => {
                            let m4001 = f147(v0.clone(), f8(Str::lit("allOf")));
                            match m4001 {
                                None => {
                                    let m4006 = f147(v0.clone(), f8(Str::lit("enum")));
                                    match m4006 {
                                        None => {
                                            let m4007 = f147(v0.clone(), f8(Str::lit("const")));
                                            match m4007 {
                                                None => {
                                                    let m4009 = f147(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m4009 {
                                                        None => {
                                                            let v27: Bytes = f150(v0.clone(), f8(Str::lit("type")));
                                                            if if f36(v27.clone(), Bytes::lit(&[])) {
                                                                !f579(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f539())
                                                            } else {
                                                                if f36(v27.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f486(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                                                } else {
                                                                    if f36(v27.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f486(f580(v0), List::<Adt24>::nil()))
                                                                    } else {
                                                                        if f36(v27.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f486(f581(), List::<Adt24>::nil()))
                                                                        } else {
                                                                            if f36(v27.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f486(f8(Str::lit("Bool")), List::<Adt24>::nil()))
                                                                            } else {
                                                                                if f36(v27.clone(), f8(Str::lit("object"))) {
                                                                                    let m4010 = f371(f147(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m4010.uncons() {
                                                                                        None => {
                                                                                            let m4011 = f582(v0);
                                                                                            match m4011 {
                                                                                                None => {
                                                                                                    Ok(f537(f8(Str::lit("List")), f583(f539())))
                                                                                                }
                                                                                                Some(v30) => {
                                                                                                    Ok(f537(f8(Str::lit("List")), f583(f584(v30, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f486(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f485(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v27, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f486(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v24) => {
                                                    let m4008 = f155(Some(v24));
                                                    match m4008 {
                                                        None => {
                                                            Ok(f485(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f486(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f486(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                        }
                                    }
                                }
                                Some(v16) => {
                                    let m4002 = f238(v16);
                                    match m4002.uncons() {
                                        None => {
                                            Ok(f485(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v17, v18)) => {
                                            let m4003 = v18;
                                            match m4003.uncons() {
                                                None => {
                                                    let m4004 = f155(f147(v17, f8(Str::lit("$ref"))));
                                                    match m4004 {
                                                        None => {
                                                            Ok(f485(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v21) => {
                                                            let m4005 = f483(v21)?;
                                                            match m4005 {
                                                                None => {
                                                                    Ok(f485(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v22) => {
                                                                    Ok(f486(v22, List::<Adt24>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f485(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v14) => {
                            let m4000 = f483(v14)?;
                            match m4000 {
                                None => {
                                    Ok(f485(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v15) => {
                                    Ok(f486(v15, List::<Adt24>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v13) => {
                    Ok(f485(if f36(v13.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v13, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt14::C5 => {
            Ok(f539())
        }
        Adt14::C6(_, _) => {
            Ok(f485(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C7(_, _, _) => {
            Ok(f485(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f573(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4012, _) = v0;
        h4012
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4013) = v0;
        h4013
    };
    let m4014 = v1;
    match m4014.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4015 = v4;
            match m4015.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f585(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f574(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4016, _) = v0.clone();
        h4016
    };
    let v2: List<(Bytes, ())> = {
        let (h4017, _) = {
            let (_, h4018) = v0.clone();
            h4018
        };
        h4017
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4019) = {
            let (_, h4020) = v0;
            h4020
        };
        h4019
    };
    let m4021 = v1.clone();
    match m4021.uncons() {
        None => {
            let m4022 = v2;
            match m4022.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4023 = v2.clone();
            match m4023.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f586({
                        let (h4024, _) = v8.clone();
                        h4024
                    }, {
                        let (h4025, _) = v6.clone();
                        h4025
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

pub fn f575(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4026, _) = v1;
        h4026
    };
    let m4028 = {
        let (_, h4027) = v1;
        h4027
    };
    match m4028 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4031 = {
                let a4029 = v0.clone();
                let a4030 = v2;
                index_bytes(a4029, a4030)
            };
            match m4031 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4036 = {
                        let a4034 = v0;
                        let a4035 = {
                            let a4032 = v2;
                            let a4033 = 1u64;
                            nat_add(a4032, a4033)?
                        };
                        index_bytes(a4034, a4035)
                    };
                    match m4036 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f587(v4)? {
                                f587(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4037 = v2;
                                    let a4038 = 2u64;
                                    nat_add(a4037, a4038)?
                                }, Some({
                                    let a4043 = v3;
                                    let a4044 = f306(f588(), {
                                        let a4041 = {
                                            let a4039 = f589(v4)?;
                                            let a4040 = 16u64;
                                            nat_mul(a4039, a4040)?
                                        };
                                        let a4042 = f589(v5)?;
                                        nat_add(a4041, a4042)?
                                    });
                                    append_bytes(a4043, a4044)
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

pub fn f576(v0: u64) -> Bytes {
    let a4045 = f8(Str::lit("u"));
    let a4046 = f590(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4045, a4046)
}

pub fn f577(p4059: u64, v1: Bytes, p4060: u64, p4061: Bytes) -> R<Option<Bytes>> {
    let m4058 = (p4059, (p4060, p4061));
    {
        let mut m4058 = m4058;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4058;
                let m4047 = v0;
                if m4047 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4047 - 1;
                    let a4049 = {
                        let a4048 = v1.clone();
                        length_bytes(a4048)
                    };
                    let a4050 = v2;
                    if nat_le(a4049, a4050) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4051 = f399(v1.clone(), v2)?;
                        match m4051 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4053 = v2;
                                    let a4054 = {
                                        let (_, h4052) = v5;
                                        h4052
                                    };
                                    nat_add(a4053, a4054)?
                                }, {
                                    let a4056 = v3;
                                    let a4057 = f591(v1.clone(), v2, {
                                        let (h4055, _) = v5;
                                        h4055
                                    });
                                    append_bytes(a4056, a4057)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4058 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(p4066: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m4065 = p4066;
    {
        let mut m4065 = m4065;
        loop {
            let step = {
                let v0 = m4065;
                let m4062 = v0;
                match m4062.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f86({
                            let (h4063, _) = v1.clone();
                            h4063
                        }, f592()) {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h4064, _) = v1;
                                h4064
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: Adt14) -> bool {
    if f494(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f494(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f494(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f494(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f494(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f494(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f580(v0: Adt14) -> Bytes {
    let m4067 = f330(f147(v0, f8(Str::lit("minimum"))));
    match m4067 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f581() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f582(v0: Adt14) -> Option<Adt14> {
    let m4068 = f147(v0.clone(), f8(Str::lit("additionalProperties")));
    match m4068 {
        None => {
            let m4069 = f371(f147(v0, f8(Str::lit("patternProperties"))));
            match m4069.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v2, v3)) => {
                    let m4070 = v3;
                    match m4070.uncons() {
                        None => {
                            Some({
                                let (_, h4071) = v2;
                                h4071
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

pub fn f583(v0: Result<(Bytes, List<Adt24>), Bytes>) -> Result<(Bytes, List<Adt24>), Bytes> {
    let m4072 = v0;
    match m4072 {
        Ok(v2) => {
            f486(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h4073, _) = v2;
                h4073
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h4074) = v2;
                h4074
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt24>), Bytes>(v1)
        }
    }
}

pub fn f584(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt24>), Bytes>> {
    f593(16u64, v0, v1)
}

pub fn f585(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4079 = {
            let a4077 = {
                let a4075 = v0.clone();
                length_list(a4075)
            };
            let a4078 = {
                let a4076 = v1.clone();
                length_list(a4076)
            };
            nat_add(a4077, a4078)?
        };
        let a4080 = 1u64;
        nat_add(a4079, a4080)?
    };
    Ok(f594({
        let (_, h4081) = {
            let (_, h4082) = {
                let (h4083, _) = f595(Fn25::F596, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4083
            };
            h4082
        };
        h4081
    }))
}

pub fn f586(v0: Bytes, v1: Bytes) -> bool {
    let a4088 = {
        let a4084 = v0;
        let a4085 = v1;
        compare_bytes(a4084, a4085)
    };
    let a4089 = {
        let a4086 = Bytes::lit(&[]);
        let a4087 = Bytes::lit(&[0x01]);
        compare_bytes(a4086, a4087)
    };
    equal(a4088, a4089)
}

pub fn f587(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4134 = {
            let a4110 = {
                let a4098 = {
                    let a4092 = {
                        let a4090 = v1;
                        let a4091 = 128u8;
                        fixed_u8::bit_and(a4090, a4091)
                    };
                    let a4093 = 128u8;
                    if equal(a4092, a4093) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4099 = {
                    let a4096 = {
                        let a4094 = v1;
                        let a4095 = 64u8;
                        fixed_u8::bit_and(a4094, a4095)
                    };
                    let a4097 = 64u8;
                    if equal(a4096, a4097) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4098, a4099)?
            };
            let a4111 = {
                let a4108 = {
                    let a4102 = {
                        let a4100 = v1;
                        let a4101 = 32u8;
                        fixed_u8::bit_and(a4100, a4101)
                    };
                    let a4103 = 32u8;
                    if equal(a4102, a4103) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4109 = {
                    let a4106 = {
                        let a4104 = v1;
                        let a4105 = 16u8;
                        fixed_u8::bit_and(a4104, a4105)
                    };
                    let a4107 = 16u8;
                    if equal(a4106, a4107) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4108, a4109)?
            };
            nat_add(a4110, a4111)?
        };
        let a4135 = {
            let a4132 = {
                let a4120 = {
                    let a4114 = {
                        let a4112 = v1;
                        let a4113 = 8u8;
                        fixed_u8::bit_and(a4112, a4113)
                    };
                    let a4115 = 8u8;
                    if equal(a4114, a4115) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4121 = {
                    let a4118 = {
                        let a4116 = v1;
                        let a4117 = 4u8;
                        fixed_u8::bit_and(a4116, a4117)
                    };
                    let a4119 = 4u8;
                    if equal(a4118, a4119) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4120, a4121)?
            };
            let a4133 = {
                let a4130 = {
                    let a4124 = {
                        let a4122 = v1;
                        let a4123 = 2u8;
                        fixed_u8::bit_and(a4122, a4123)
                    };
                    let a4125 = 2u8;
                    if equal(a4124, a4125) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4131 = {
                    let a4128 = {
                        let a4126 = v1;
                        let a4127 = 1u8;
                        fixed_u8::bit_and(a4126, a4127)
                    };
                    let a4129 = 1u8;
                    if equal(a4128, a4129) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4130, a4131)?
            };
            nat_add(a4132, a4133)?
        };
        nat_add(a4134, a4135)?
    };
    let a4136 = 48u64;
    let a4137 = v2;
    if if nat_le(a4136, a4137) {
        let a4138 = v2;
        let a4139 = 57u64;
        nat_le(a4138, a4139)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4140 = 97u64;
        let a4141 = v2;
        if nat_le(a4140, a4141) {
            let a4142 = v2;
            let a4143 = 102u64;
            Ok(nat_le(a4142, a4143))
        } else {
            Ok(false)
        }
    }
}

pub fn f588() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f589(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4188 = {
            let a4164 = {
                let a4152 = {
                    let a4146 = {
                        let a4144 = v1;
                        let a4145 = 128u8;
                        fixed_u8::bit_and(a4144, a4145)
                    };
                    let a4147 = 128u8;
                    if equal(a4146, a4147) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4153 = {
                    let a4150 = {
                        let a4148 = v1;
                        let a4149 = 64u8;
                        fixed_u8::bit_and(a4148, a4149)
                    };
                    let a4151 = 64u8;
                    if equal(a4150, a4151) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4152, a4153)?
            };
            let a4165 = {
                let a4162 = {
                    let a4156 = {
                        let a4154 = v1;
                        let a4155 = 32u8;
                        fixed_u8::bit_and(a4154, a4155)
                    };
                    let a4157 = 32u8;
                    if equal(a4156, a4157) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4163 = {
                    let a4160 = {
                        let a4158 = v1;
                        let a4159 = 16u8;
                        fixed_u8::bit_and(a4158, a4159)
                    };
                    let a4161 = 16u8;
                    if equal(a4160, a4161) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4162, a4163)?
            };
            nat_add(a4164, a4165)?
        };
        let a4189 = {
            let a4186 = {
                let a4174 = {
                    let a4168 = {
                        let a4166 = v1;
                        let a4167 = 8u8;
                        fixed_u8::bit_and(a4166, a4167)
                    };
                    let a4169 = 8u8;
                    if equal(a4168, a4169) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4175 = {
                    let a4172 = {
                        let a4170 = v1;
                        let a4171 = 4u8;
                        fixed_u8::bit_and(a4170, a4171)
                    };
                    let a4173 = 4u8;
                    if equal(a4172, a4173) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4174, a4175)?
            };
            let a4187 = {
                let a4184 = {
                    let a4178 = {
                        let a4176 = v1;
                        let a4177 = 2u8;
                        fixed_u8::bit_and(a4176, a4177)
                    };
                    let a4179 = 2u8;
                    if equal(a4178, a4179) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4185 = {
                    let a4182 = {
                        let a4180 = v1;
                        let a4181 = 1u8;
                        fixed_u8::bit_and(a4180, a4181)
                    };
                    let a4183 = 1u8;
                    if equal(a4182, a4183) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4184, a4185)?
            };
            nat_add(a4186, a4187)?
        };
        nat_add(a4188, a4189)?
    };
    let a4190 = v2;
    let a4191 = 57u64;
    if nat_le(a4190, a4191) {
        let a4192 = v2;
        let a4193 = 48u64;
        Ok(nat_sub(a4192, a4193))
    } else {
        let a4194 = v2;
        let a4195 = 87u64;
        Ok(nat_sub(a4194, a4195))
    }
}

pub fn f590(p4212: u64, p4213: u64, p4214: u64, p4215: Bytes) -> Bytes {
    let m4211 = (p4212, (p4213, (p4214, p4215)));
    {
        let mut m4211 = m4211;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m4211;
                let m4196 = v0;
                if m4196 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m4196 - 1;
                    let a4197 = v1;
                    let a4198 = 0u64;
                    if if nat_eq(a4197, a4198) {
                        let a4199 = v2;
                        let a4200 = 0u64;
                        nat_eq(a4199, a4200)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a4201 = v1;
                            let a4202 = 16u64;
                            let a4203 = 0u64;
                            nat_quot(a4201, a4202, a4203)
                        }, ({
                            let a4204 = v2;
                            let a4205 = 1u64;
                            nat_sub(a4204, a4205)
                        }, {
                            let a4209 = f597({
                                let a4206 = v1;
                                let a4207 = 16u64;
                                let a4208 = 0u64;
                                nat_rem(a4206, a4207, a4208)
                            });
                            let a4210 = v3;
                            append_bytes(a4209, a4210)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m4211 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f591(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f367(v2) {
        true
    } else {
        f368(v2)
    } {
        let a4216 = v2;
        let a4217 = 117u64;
        if !nat_eq(a4216, a4217) {
            let a4218 = v2;
            let a4219 = 85u64;
            !nat_eq(a4218, a4219)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f369(v2) {
            let a4220 = 0u64;
            let a4221 = v1;
            nat_lt(a4220, a4221)
        } else {
            false
        }
    } {
        f32(v0, v1, 1u64)
    } else {
        let a4222 = v2;
        let a4223 = 65536u64;
        if nat_lt(a4222, a4223) {
            let a4224 = f8(Str::lit("u"));
            let a4225 = f590(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4224, a4225)
        } else {
            let a4226 = f8(Str::lit("U"));
            let a4227 = f590(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4226, a4227)
        }
    }
}

pub fn f592() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f593(v0: u64, v1: Adt14, v2: Bytes) -> R<Result<(Bytes, List<Adt24>), Bytes>> {
    let m4228 = v0;
    if m4228 == 0 {
        Ok(f485(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4228 - 1;
        let m4229 = v1.clone();
        match m4229 {
            Adt14::C0(v4) => {
                if v4 {
                    Ok(f539())
                } else {
                    Ok(f485(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt14::C1(_, _) => {
                Ok(f485(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C2(_) => {
                Ok(f485(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C3(_) => {
                Ok(f485(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C4(_) => {
                let m4230 = f155(f147(v1.clone(), f8(Str::lit("$ref"))));
                match m4230 {
                    None => {
                        if if f494(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f494(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f494(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f486(v2.clone(), List::cons(Adt24::C0(v2, v1), List::<Adt24>::nil())))
                        } else {
                            let v17: Bytes = f150(v1.clone(), f8(Str::lit("type")));
                            if if f36(v17.clone(), Bytes::lit(&[])) {
                                !f579(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f539())
                            } else {
                                if f36(v17.clone(), f8(Str::lit("string"))) {
                                    Ok(f486(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                } else {
                                    if f36(v17.clone(), f8(Str::lit("integer"))) {
                                        Ok(f486(f580(v1), List::<Adt24>::nil()))
                                    } else {
                                        if f36(v17.clone(), f8(Str::lit("number"))) {
                                            Ok(f486(f581(), List::<Adt24>::nil()))
                                        } else {
                                            if f36(v17.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f486(f8(Str::lit("Bool")), List::<Adt24>::nil()))
                                            } else {
                                                if f36(v17.clone(), f8(Str::lit("array"))) {
                                                    let m4232 = f147(v1, f8(Str::lit("items")));
                                                    match m4232 {
                                                        None => {
                                                            Ok(f537(f8(Str::lit("List")), f539()))
                                                        }
                                                        Some(v18) => {
                                                            Ok(f537(f8(Str::lit("List")), f593(v3, v18, {
                                                                let a4233 = v2;
                                                                let a4234 = f8(Str::lit("Item"));
                                                                append_bytes(a4233, a4234)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f36(v17.clone(), f8(Str::lit("object"))) {
                                                        let m4235 = f582(v1);
                                                        match m4235 {
                                                            None => {
                                                                Ok(f537(f8(Str::lit("List")), f583(f539())))
                                                            }
                                                            Some(v19) => {
                                                                Ok(f537(f8(Str::lit("List")), f583(f593(v3, v19, {
                                                                    let a4236 = v2;
                                                                    let a4237 = f8(Str::lit("Value"));
                                                                    append_bytes(a4236, a4237)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f485(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v17, List::<Bytes>::nil()))), v2))
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
                        let m4231 = f483(v15)?;
                        match m4231 {
                            None => {
                                Ok(f485(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v16) => {
                                Ok(f486(v16, List::<Adt24>::nil()))
                            }
                        }
                    }
                }
            }
            Adt14::C5 => {
                Ok(f539())
            }
            Adt14::C6(_, _) => {
                Ok(f485(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C7(_, _, _) => {
                Ok(f485(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f594(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f380(Fn10::F598, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f595(v0: Fn25, p4242: u64, p4243: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4241 = (p4242, p4243);
    {
        let mut m4241 = m4241;
        loop {
            let step = {
                let (v1, v2) = m4241;
                let m4238 = v1;
                if m4238 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4238 - 1;
                    let m4240 = {
                        let c4239: Fn25 = v0.clone();
                        c4239.apply(v2.clone())
                    };
                    match m4240 {
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
                Ok(next) => { m4241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f599(v0)
}

pub fn f597(v0: u64) -> Bytes {
    f32({
        let a4244 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4244)
    }, v0, 1u64)
}

pub fn f598(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f599(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4245, _) = v0.clone();
        h4245
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4246, _) = {
            let (_, h4247) = v0.clone();
            h4247
        };
        h4246
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4248) = {
            let (_, h4249) = v0;
            h4249
        };
        h4248
    };
    let m4250 = v1.clone();
    match m4250.uncons() {
        None => {
            let m4251 = v2;
            match m4251.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4252 = v2.clone();
            match m4252.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f586({
                        let (h4253, _) = v8.clone();
                        h4253
                    }, {
                        let (h4254, _) = v6.clone();
                        h4254
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
