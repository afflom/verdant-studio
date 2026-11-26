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
    C0,
    C1(Bytes, std::rc::Rc<Adt2>),
    C2(bool),
    C3(bool, u64),
    C4(bool, u64, u64),
    C5(bool, u64),
    C6(Bytes),
    C7(std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>),
    C9,
    C10(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C11(Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
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
    C0(List<Bytes>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(List<Bytes>, List<Bytes>, List<Bytes>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Adt15, List<Adt2>, List<Adt2>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(bool, bool, Bytes, List<Bytes>, List<List<Bytes>>),
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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Bytes, Adt2),
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
pub enum Adt27 {
    C0(Bytes, List<Adt24>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(List<Bytes>, List<Adt24>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(bool, List<(u64, u64)>, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F77(u8, Bytes),
    F186(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F77(k0, k1) => f77(*k0, k1.clone(), p0),
            Fn0::F186(k0) => f186(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F121,
    F237,
    F352,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F121 => f121(p0, p1),
            Fn1::F237 => f237(p0, p1),
            Fn1::F352 => f352(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F209(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F209(k0) => f209(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F322(Bytes),
    F323(Bytes),
    F426(Bytes),
    F427(u64, Bytes),
    F434(Bytes),
    F442(Bytes, u64, Bytes, u64),
    F508(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F322(k0) => f322(k0.clone(), p0),
            Fn3::F323(k0) => f323(k0.clone(), p0),
            Fn3::F426(k0) => Ok(f426(k0.clone(), p0)),
            Fn3::F427(k0, k1) => f427(*k0, k1.clone(), p0),
            Fn3::F434(k0) => f434(k0.clone(), p0),
            Fn3::F442(k0, k1, k2, k3) => f442(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F508(k0) => f508(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F326(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F326(k0) => f326(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F418,
    F755,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F418 => f418(p0, p1),
            Fn5::F755 => f755(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F421,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F421 => f421(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F453,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F453 => f453(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F456,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F456 => f456(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F458,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F458 => f458(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F461,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F461 => f461(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F480(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt23)) -> R<(u64, Adt23)> {
        match self {
            Fn11::F480(k0, k1, k2) => f480(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F519,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F519 => f519(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F522,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F522 => f522(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F553(u64, Bytes),
    F564(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn14::F553(k0, k1) => Ok(f553(*k0, k1.clone(), p0)),
            Fn14::F564(k0) => f564(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F561,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F561 => f561(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F563,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F563 => f563(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F567(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F567(k0) => f567(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F583,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt26) -> R<Adt26> {
        match self {
            Fn18::F583 => f583(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F604,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F604 => f604(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F609,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F609 => f609(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F671,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F671 => f671(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F682,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F682 => f682(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F713,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F713 => f713(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F741,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F741 => f741(p0),
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
                                        if if f85(v2.clone(), f8(Str::lit("read-"))) {
                                            true
                                        } else {
                                            f32(v2.clone(), f8(Str::lit("check-yaml")))
                                        } {
                                            Ok(Adt12::C4(f8(Str::lit("check-yaml, read-webidl, read-xml, read-xsd, read-xsd-components and read-yaml run in verdant_readers, the command of the readers project\u{a}"))))
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

pub fn f57(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    f91(v0, v1)
}

pub fn f58(v0: Adt2, v1: Bytes) -> Bytes {
    let m276 = f92(f93(v0, v1));
    match m276 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f59(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m277 = v0;
    match m277.uncons() {
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
            let a279 = {
                let a278 = v0;
                length_bytes(a278)
            };
            let a280 = 1u64;
            nat_sub(a279, a280)
        })
    } else {
        v0
    }
}

pub fn f62(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m281 = v0;
    match m281.uncons() {
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
    f94(f95(v0)?, f8(Str::lit("id = \"")))
}

pub fn f64(v0: Adt1, v1: Bytes) -> Bytes {
    let m282 = f89(f90(v0, v1.clone()), v1);
    match m282 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m283 = v2;
            match m283 {
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
    let v2: List<Adt2> = f96(v0, f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt2> = f97(v2.clone(), f61(v1.clone()));
    if f98(f99(v2), f99(v3.clone())) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" is pinned, and every admission of its registers cites it\u{a}")), List::<Bytes>::nil()))))))
    } else {
        Ok(Adt12::C3(List::cons((f8(Str::lit("model/admissions.json")), f100(Adt2::C7(std::rc::Rc::new(f101(v3))))?), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("re-cited the admissions of the verdant-engine registers at ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
    }
}

pub fn f66(v0: List<Bytes>) -> List<Bytes> {
    let m284 = v0;
    match m284.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f68(v1), f66(v2))
        }
    }
}

pub fn f67(v0: Adt1, v1: Bytes) -> bool {
    let m285 = f89(f90(v0, v1.clone()), v1);
    match m285 {
        None => {
            false
        }
        Some(v2) => {
            let m286 = v2;
            match m286 {
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
    f7(List::cons(f102(v0), List::cons(v1, List::<Bytes>::nil())))
}

pub fn f70(v0: Adt1, v1: Bytes) -> R<Adt12> {
    if f51(f52(f103(), match v0.clone() {
        Adt1::C0(_, _, h287, _) => {
            h287
        }
    })) {
        Ok(f54(f103(), f55(f7(List::cons(f8(Str::lit("git/ref/tags/")), List::cons(v1, List::<Bytes>::nil()))))))
    } else {
        let m288 = f56(v0.clone(), List::cons(f103(), List::<Bytes>::nil()));
        match m288.uncons() {
            None => {
                let v6: Adt2 = {
                    let m290 = f93({
                        let m289 = f57(v0.clone(), f103())?;
                        match m289 {
                            None => {
                                Adt2::C9
                            }
                            Some(v4) => {
                                v4
                            }
                        }
                    }, f8(Str::lit("object")));
                    match m290 {
                        None => {
                            Adt2::C9
                        }
                        Some(v5) => {
                            v5
                        }
                    }
                };
                if f32(f58(v6.clone(), f8(Str::lit("type"))), f8(Str::lit("tag"))) {
                    if f51(f52(f104(), match v0.clone() {
                        Adt1::C0(_, _, h291, _) => {
                            h291
                        }
                    })) {
                        Ok(f54(f104(), f55(f7(List::cons(f8(Str::lit("git/tags/")), List::cons(f58(v6, f8(Str::lit("sha"))), List::<Bytes>::nil()))))))
                    } else {
                        let m292 = f56(v0.clone(), List::cons(f104(), List::<Bytes>::nil()));
                        match m292.uncons() {
                            None => {
                                f105(v0.clone(), v1, {
                                    let m294 = f93({
                                        let m293 = f57(v0, f104())?;
                                        match m293 {
                                            None => {
                                                Adt2::C9
                                            }
                                            Some(v9) => {
                                                v9
                                            }
                                        }
                                    }, f8(Str::lit("object")));
                                    match m294 {
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
                    f105(v0, v1, f58(v6, f8(Str::lit("sha"))))
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
    let a339 = {
        let a315 = {
            let a303 = {
                let a297 = {
                    let a295 = v1;
                    let a296 = 128u8;
                    fixed_u8::bit_and(a295, a296)
                };
                let a298 = 128u8;
                if equal(a297, a298) {
                    128u64
                } else {
                    0u64
                }
            };
            let a304 = {
                let a301 = {
                    let a299 = v1;
                    let a300 = 64u8;
                    fixed_u8::bit_and(a299, a300)
                };
                let a302 = 64u8;
                if equal(a301, a302) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a303, a304)?
        };
        let a316 = {
            let a313 = {
                let a307 = {
                    let a305 = v1;
                    let a306 = 32u8;
                    fixed_u8::bit_and(a305, a306)
                };
                let a308 = 32u8;
                if equal(a307, a308) {
                    32u64
                } else {
                    0u64
                }
            };
            let a314 = {
                let a311 = {
                    let a309 = v1;
                    let a310 = 16u8;
                    fixed_u8::bit_and(a309, a310)
                };
                let a312 = 16u8;
                if equal(a311, a312) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a313, a314)?
        };
        nat_add(a315, a316)?
    };
    let a340 = {
        let a337 = {
            let a325 = {
                let a319 = {
                    let a317 = v1;
                    let a318 = 8u8;
                    fixed_u8::bit_and(a317, a318)
                };
                let a320 = 8u8;
                if equal(a319, a320) {
                    8u64
                } else {
                    0u64
                }
            };
            let a326 = {
                let a323 = {
                    let a321 = v1;
                    let a322 = 4u8;
                    fixed_u8::bit_and(a321, a322)
                };
                let a324 = 4u8;
                if equal(a323, a324) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a325, a326)?
        };
        let a338 = {
            let a335 = {
                let a329 = {
                    let a327 = v1;
                    let a328 = 2u8;
                    fixed_u8::bit_and(a327, a328)
                };
                let a330 = 2u8;
                if equal(a329, a330) {
                    2u64
                } else {
                    0u64
                }
            };
            let a336 = {
                let a333 = {
                    let a331 = v1;
                    let a332 = 1u8;
                    fixed_u8::bit_and(a331, a332)
                };
                let a334 = 1u8;
                if equal(a333, a334) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a335, a336)?
        };
        nat_add(a337, a338)?
    };
    nat_add(a339, a340)
}

pub fn f73(v0: u64) -> List<Str> {
    let a341 = v0;
    let a342 = 0u64;
    if nat_eq(a341, a342) {
        f106()
    } else {
        let a343 = v0;
        let a344 = 1u64;
        if nat_eq(a343, a344) {
            f107()
        } else {
            let a345 = v0;
            let a346 = 2u64;
            if nat_eq(a345, a346) {
                f108()
            } else {
                let a347 = v0;
                let a348 = 3u64;
                if nat_eq(a347, a348) {
                    f109()
                } else {
                    let a349 = v0;
                    let a350 = 4u64;
                    if nat_eq(a349, a350) {
                        f110()
                    } else {
                        let a351 = v0;
                        let a352 = 5u64;
                        if nat_eq(a351, a352) {
                            f111()
                        } else {
                            let a353 = v0;
                            let a354 = 6u64;
                            if nat_eq(a353, a354) {
                                f112()
                            } else {
                                let a355 = v0;
                                let a356 = 7u64;
                                if nat_eq(a355, a356) {
                                    f113()
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
    f114(v0.clone(), 0u64, {
        let a357 = v0;
        length_bytes(a357)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f75(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m358 = v2;
    if m358 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m358 - 1;
        let v5: u64 = f115(v0.clone(), v1)?;
        let a359 = v3;
        let a360 = 1u64;
        if nat_eq(a359, a360) {
            let m361 = f75(v0.clone(), v1, v4, 0u64)?;
            match m361 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f115(v0.clone(), {
                        let (_, h362) = v6.clone();
                        h362
                    })?;
                    let a363 = v7;
                    let a364 = 44u64;
                    if nat_eq(a363, a364) {
                        let m368 = f75(v0, {
                            let a366 = {
                                let (_, h365) = v6;
                                h365
                            };
                            let a367 = 1u64;
                            nat_add(a366, a367)?
                        }, v4, 1u64)?;
                        match m368 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C10(std::rc::Rc::new({
                                    let (h369, _) = v6;
                                    h369
                                }), std::rc::Rc::new({
                                    let (h370, _) = v8;
                                    h370
                                })), {
                                    let (_, h371) = v8;
                                    h371
                                })))
                            }
                        }
                    } else {
                        let a372 = v7;
                        let a373 = 93u64;
                        if nat_eq(a372, a373) {
                            Ok(Some((Adt2::C10(std::rc::Rc::new({
                                let (h374, _) = v6;
                                h374
                            }), std::rc::Rc::new(Adt2::C9)), {
                                let a376 = {
                                    let (_, h375) = v6;
                                    h375
                                };
                                let a377 = 1u64;
                                nat_add(a376, a377)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a378 = v3;
            let a379 = 2u64;
            if nat_eq(a378, a379) {
                let m382 = f116(v0.clone(), {
                    let a380 = v1;
                    let a381 = 1u64;
                    nat_add(a380, a381)?
                })?;
                match m382 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f117({
                            let a383 = v5;
                            let a384 = 34u64;
                            nat_eq(a383, a384)
                        }, {
                            let a386 = f115(v0.clone(), {
                                let (h385, _) = v9.clone();
                                h385
                            })?;
                            let a387 = 58u64;
                            nat_eq(a386, a387)
                        }) {
                            let m391 = f75(v0.clone(), {
                                let a389 = {
                                    let (h388, _) = v9;
                                    h388
                                };
                                let a390 = 1u64;
                                nat_add(a389, a390)?
                            }, v4, 0u64)?;
                            match m391 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f115(v0.clone(), {
                                        let (_, h392) = v10.clone();
                                        h392
                                    })?;
                                    let a393 = v11;
                                    let a394 = 44u64;
                                    if nat_eq(a393, a394) {
                                        let m398 = f75(v0, {
                                            let a396 = {
                                                let (_, h395) = v10;
                                                h395
                                            };
                                            let a397 = 1u64;
                                            nat_add(a396, a397)?
                                        }, v4, 2u64)?;
                                        match m398 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C11({
                                                    let (_, h399) = v9;
                                                    h399
                                                }, std::rc::Rc::new({
                                                    let (h400, _) = v10;
                                                    h400
                                                }), std::rc::Rc::new({
                                                    let (h401, _) = v12;
                                                    h401
                                                })), {
                                                    let (_, h402) = v12;
                                                    h402
                                                })))
                                            }
                                        }
                                    } else {
                                        let a403 = v11;
                                        let a404 = 125u64;
                                        if nat_eq(a403, a404) {
                                            Ok(Some((Adt2::C11({
                                                let (_, h405) = v9;
                                                h405
                                            }, std::rc::Rc::new({
                                                let (h406, _) = v10;
                                                h406
                                            }), std::rc::Rc::new(Adt2::C9)), {
                                                let a408 = {
                                                    let (_, h407) = v10;
                                                    h407
                                                };
                                                let a409 = 1u64;
                                                nat_add(a408, a409)?
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
                let a410 = v5;
                let a411 = 34u64;
                if nat_eq(a410, a411) {
                    let m414 = f116(v0, {
                        let a412 = v1;
                        let a413 = 1u64;
                        nat_add(a412, a413)?
                    })?;
                    match m414 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C6({
                                let (_, h415) = v13;
                                h415
                            }), {
                                let (h416, _) = v13;
                                h416
                            })))
                        }
                    }
                } else {
                    if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a417 = v1;
                            let a418 = 4u64;
                            nat_add(a417, a418)?
                        })))
                    } else {
                        if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a419 = v1;
                                let a420 = 4u64;
                                nat_add(a419, a420)?
                            })))
                        } else {
                            if f32(f46(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a421 = v1;
                                    let a422 = 5u64;
                                    nat_add(a421, a422)?
                                })))
                            } else {
                                let a423 = v5;
                                let a424 = 91u64;
                                if nat_eq(a423, a424) {
                                    let a427 = f115(v0.clone(), {
                                        let a425 = v1;
                                        let a426 = 1u64;
                                        nat_add(a425, a426)?
                                    })?;
                                    let a428 = 93u64;
                                    if nat_eq(a427, a428) {
                                        Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C9)), {
                                            let a429 = v1;
                                            let a430 = 2u64;
                                            nat_add(a429, a430)?
                                        })))
                                    } else {
                                        let m433 = f75(v0, {
                                            let a431 = v1;
                                            let a432 = 1u64;
                                            nat_add(a431, a432)?
                                        }, v4, 1u64)?;
                                        match m433 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                    let (h434, _) = v14;
                                                    h434
                                                })), {
                                                    let (_, h435) = v14;
                                                    h435
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a436 = v5;
                                    let a437 = 123u64;
                                    if nat_eq(a436, a437) {
                                        let a440 = f115(v0.clone(), {
                                            let a438 = v1;
                                            let a439 = 1u64;
                                            nat_add(a438, a439)?
                                        })?;
                                        let a441 = 125u64;
                                        if nat_eq(a440, a441) {
                                            Ok(Some((Adt2::C8(std::rc::Rc::new(Adt2::C9)), {
                                                let a442 = v1;
                                                let a443 = 2u64;
                                                nat_add(a442, a443)?
                                            })))
                                        } else {
                                            let m446 = f75(v0, {
                                                let a444 = v1;
                                                let a445 = 1u64;
                                                nat_add(a444, a445)?
                                            }, v4, 2u64)?;
                                            match m446 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C8(std::rc::Rc::new({
                                                        let (h447, _) = v15;
                                                        h447
                                                    })), {
                                                        let (_, h448) = v15;
                                                        h448
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m449 = f118(v0, v1)?;
                                        match m449 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h450) = v16;
                                                    h450
                                                }, {
                                                    let (h451, _) = v16;
                                                    h451
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

pub fn f76(v0: Fn0, p456: u64, p457: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m455 = (p456, p457);
    {
        let mut m455 = m455;
        loop {
            let step = {
                let (v1, v2) = m455;
                let m452 = v1;
                if m452 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m452 - 1;
                    let m454 = {
                        let c453: Fn0 = v0.clone();
                        c453.apply(v2.clone())?
                    };
                    match m454 {
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
                Ok(next) => { m455 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f119(v1, v0, v2)
}

pub fn f78(v0: List<Bytes>) -> List<Bytes> {
    f120(Fn1::F121, List::<Bytes>::nil(), v0)
}

pub fn f79() -> Bytes {
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f122(f123(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f80(v0: Adt1) -> R<Adt12> {
    let m458 = f124(v0.clone());
    match m458.uncons() {
        None => {
            let m459 = f125(f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f126(v0.clone())?);
            match m459.uncons() {
                None => {
                    let v5: Adt14 = f127(v0.clone())?;
                    let v6: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(f125(f128(f129(match v5.clone() {
                        Adt14::C0(h460, _, _) => {
                            h460
                        }
                    }, f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f131(v6.clone())), f132(f129(match v5.clone() {
                        Adt14::C0(h461, _, _) => {
                            h461
                        }
                    }, f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f133(v5.clone(), v6, f134(v5, f135(v0.clone(), f136(v0)?)?))?)))))
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
        Adt1::C0(_, _, h462, _) => {
            h462
        }
    })) {
        Adt12::C2(f8(Str::lit("npx")), f137(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f51(f52(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h463, _) => {
                h463
            }
        })) {
            Adt12::C2(f8(Str::lit("npx")), f137(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f82(v0: Adt1) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h464, _, _) => {
            h464
        }
    }, f139())) {
        Ok(Adt12::C1(List::cons(f139(), List::<Bytes>::nil())))
    } else {
        let m465 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f140(v0.clone())?)));
        match m465.uncons() {
            None => {
                let m466 = f141(v0.clone(), f140(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m466.uncons() {
                    None => {
                        f143(v0)
                    }
                    Some((v3, v4)) => {
                        Ok(Adt12::C4(f7(List::cons(v3, v4))))
                    }
                }
            }
            Some((v1, v2)) => {
                Ok(Adt12::C0(List::cons(v1, v2)))
            }
        }
    }
}

pub fn f83(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h467, _, _) => {
            h467
        }
    }, f144())) {
        Ok(Adt12::C1(List::cons(f144(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f145(f146(f144(), f147(match v0.clone() {
            Adt1::C0(_, h468, _, _) => {
                h468
            }
        }, f144())))?;
        let m469 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m469.uncons() {
            None => {
                let m470 = f148(v0.clone(), f149(f150(f95(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?, f151(v2, v1.clone()))?;
                match m470 {
                    None => {
                        f152(v0, v1)
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
    let m471 = f124(v0.clone());
    match m471.uncons() {
        None => {
            let m472 = f125(f56(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f153(), List::<Bytes>::nil())))), f126(v0.clone())?);
            match m472.uncons() {
                None => {
                    Ok(Adt12::C3(f154(f155(v0.clone())?, List::cons((f156(), f157()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f158()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f159()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f160()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f161()), List::cons((f8(Str::lit("docs/catalog.md")), f162(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f163(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f164(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f165()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m477 = {
        let a474 = v0;
        let a475 = 0u64;
        let a476 = {
            let a473 = v1.clone();
            length_bytes(a473)
        };
        slice_bytes(a474, a475, a476)
    };
    match m477 {
        None => {
            false
        }
        Some(v2) => {
            let a478 = v2;
            let a479 = v1;
            equal(a478, a479)
        }
    }
}

pub fn f86(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f166(v3.clone(), v1.clone(), f167(v3, v1, false)?)
    } else {
        if f32(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f166(v3.clone(), v1.clone(), f168(v3, v1)?)
        } else {
            if f32(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f166(v3.clone(), v1.clone(), f167(v3, v1, true)?)
            } else {
                if f32(v0.clone(), f8(Str::lit("stage-registries"))) {
                    f166(v3.clone(), v1.clone(), f169(v3, v1)?)
                } else {
                    if f32(v0.clone(), f8(Str::lit("stage-encodings"))) {
                        f166(v3.clone(), v1.clone(), f170(v3, v1)?)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("stage-mappings"))) {
                            f166(v3.clone(), v1.clone(), f171(v3, v1, false)?)
                        } else {
                            if f32(v0, f8(Str::lit("stage-matrix"))) {
                                f166(v3.clone(), v1.clone(), f171(v3, v1, true)?)
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

pub fn f87(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("admit"))) {
        f172(v3, v1)
    } else {
        if f32(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f173(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("admit-registry"))) {
                f174(v3, v1, v2)
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
                                let m480 = v2;
                                match m480.uncons() {
                                    None => {
                                        Ok(Adt12::C4(f79()))
                                    }
                                    Some((v4, v5)) => {
                                        f178(v3, v1, v4, v5)
                                    }
                                }
                            } else {
                                if f32(v0.clone(), f8(Str::lit("admit-translated"))) {
                                    let m481 = v2;
                                    match m481.uncons() {
                                        None => {
                                            Ok(Adt12::C4(f79()))
                                        }
                                        Some((v6, v7)) => {
                                            f179(v3, v1, v6, v7)
                                        }
                                    }
                                } else {
                                    if f32(v0.clone(), f8(Str::lit("record-decision"))) {
                                        f180(v3, v1)
                                    } else {
                                        if f32(v0, f8(Str::lit("record-error"))) {
                                            f181(v3, v1, v2)
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
        }
    }
}

pub fn f88(v0: Bytes) -> R<bool> {
    f182(v0.clone(), 0u64, {
        let a482 = v0;
        length_bytes(a482)
    })
}

pub fn f89(p487: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m486 = p487;
    {
        let mut m486 = m486;
        loop {
            let step = {
                let v0 = m486;
                let m483 = v0;
                match m483.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h484, _) = v2;
                            h484
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h485) = v2;
                                h485
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m486 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m491 = {
        let a489 = match v0 {
            Adt1::C0(h488, _, _, _) => {
                h488
            }
        };
        let a490 = f40(v1);
        index_list(a489, a490)
    };
    match m491 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m492 = f183(f184(v0.clone(), v1.clone()), v1.clone());
    match m492 {
        None => {
            f43(f64(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f92(v0: Option<Adt2>) -> Option<Bytes> {
    let m493 = v0;
    match m493 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m494 = v1;
            match m494 {
                Adt2::C0 => {
                    None::<Bytes>
                }
                Adt2::C1(_, _) => {
                    None::<Bytes>
                }
                Adt2::C2(_) => {
                    None::<Bytes>
                }
                Adt2::C3(_, _) => {
                    None::<Bytes>
                }
                Adt2::C4(_, _, _) => {
                    None::<Bytes>
                }
                Adt2::C5(_, _) => {
                    None::<Bytes>
                }
                Adt2::C6(v2) => {
                    Some(v2)
                }
                Adt2::C7(_) => {
                    None::<Bytes>
                }
                Adt2::C8(_) => {
                    None::<Bytes>
                }
                Adt2::C9 => {
                    None::<Bytes>
                }
                Adt2::C10(_, _) => {
                    None::<Bytes>
                }
                Adt2::C11(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f93(p500: Adt2, v1: Bytes) -> Option<Adt2> {
    let m499 = p500;
    {
        let mut m499 = m499;
        loop {
            let step = {
                let v0 = m499;
                let m495 = v0;
                match m495 {
                    Adt2::C0 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C1(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C2(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C3(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C5(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C6(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C7(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C8(r496) => {
                        let v5 = (*r496).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C9 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C11(v2, r497, r498) => {
                        let v3 = (*r497).clone();
                        let v4 = (*r498).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m499 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m501 = v0;
    match m501.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m502 = f185(v2, v1.clone())?;
            match m502 {
                None => {
                    f94(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f94(v3, v1)?))
                }
            }
        }
    }
}

pub fn f95(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a503 = v0.clone();
        length_bytes(a503)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h504, _) = f76(Fn0::F186(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h504
    };
    let v3: u64 = {
        let (h505, _) = {
            let (_, h506) = v2.clone();
            h506
        };
        h505
    };
    let v4: List<Bytes> = {
        let (_, h507) = {
            let (_, h508) = v2;
            h508
        };
        h507
    };
    let a509 = v3;
    let a510 = v1;
    if nat_lt(a509, a510) {
        Ok(f78(List::cons(f46(v0, v3, {
            let a511 = v1;
            let a512 = v3;
            nat_sub(a511, a512)
        }), v4)))
    } else {
        Ok(f78(v4))
    }
}

pub fn f96(v0: Adt1, v1: Bytes) -> R<List<Adt2>> {
    Ok(f187(f57(v0, v1)?))
}

pub fn f97(v0: List<Adt2>, v1: Bytes) -> List<Adt2> {
    let m513 = v0;
    match m513.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f188(v2, v1.clone()), f97(v3, v1))
        }
    }
}

pub fn f98(p518: List<Bytes>, p519: List<Bytes>) -> bool {
    let m517 = (p518, p519);
    {
        let mut m517 = m517;
        loop {
            let step = {
                let (v0, v1) = m517;
                let m514 = v0;
                match m514.uncons() {
                    None => {
                        let m515 = v1;
                        match m515.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m516 = v1;
                        match m516.uncons() {
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
                Ok(next) => { m517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: List<Adt2>) -> List<Bytes> {
    let m520 = v0;
    match m520.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m521 = f93(v1, f8(Str::lit("std")));
            match m521 {
                None => {
                    f99(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f99(v2))
                }
            }
        }
    }
}

pub fn f100(v0: Adt2) -> R<Bytes> {
    let a522 = f189(v0)?;
    let a523 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a522, a523))
}

pub fn f101(v0: List<Adt2>) -> Adt2 {
    let m524 = v0;
    match m524.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(v1), std::rc::Rc::new(f101(v2)))
        }
    }
}

pub fn f102(v0: Adt13) -> Bytes {
    let m525 = v0;
    match m525 {
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

pub fn f103() -> Bytes {
    f8(Str::lit("target/engine/ref.json"))
}

pub fn f104() -> Bytes {
    f8(Str::lit("target/engine/tag.json"))
}

pub fn f105(v0: Adt1, v1: Bytes, v2: Bytes) -> R<Adt12> {
    let a527 = {
        let a526 = v2.clone();
        length_bytes(a526)
    };
    let a528 = 0u64;
    if nat_eq(a527, a528) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m529 = f190(v0.clone(), f60())?;
        match m529 {
            None => {
                if f51(f52(f193(), match v0.clone() {
                    Adt1::C0(_, _, h530, _) => {
                        h530
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f193()))
                } else {
                    let m531 = f56(v0.clone(), List::cons(f194(), f195(f60())?));
                    match m531.uncons() {
                        None => {
                            Ok(Adt12::C3(f196(v0.clone(), f60(), v1.clone(), v2.clone(), f197(f64(v0, f194()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f191(v3.clone())?, f192(v3, v2)))
            }
        }
    }
}

pub fn f106() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f107() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f108() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f109() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f110() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f111() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f112() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f113() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f114(v0: Bytes, p559: u64, p560: u64, p561: bool, p562: bool, p563: u64, p564: Bytes) -> R<Bytes> {
    let m558 = (p559, (p560, (p561, (p562, (p563, p564)))));
    {
        let mut m558 = m558;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m558;
                let m532 = v2;
                if m532 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f198(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m532 - 1;
                    let m535 = {
                        let a533 = v0.clone();
                        let a534 = v1;
                        index_bytes(a533, a534)
                    };
                    match m535 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f198(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f72(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a536 = v1;
                                    let a537 = 1u64;
                                    nat_add(a536, a537)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a538 = v9;
                                    let a539 = 34u64;
                                    !nat_eq(a538, a539)
                                }, (if v4 {
                                    false
                                } else {
                                    let a540 = v9;
                                    let a541 = 92u64;
                                    nat_eq(a540, a541)
                                }, (v5, v6))))))
                            } else {
                                let a542 = v9;
                                let a543 = 32u64;
                                if if if if nat_eq(a542, a543) {
                                    true
                                } else {
                                    let a544 = v9;
                                    let a545 = 9u64;
                                    nat_eq(a544, a545)
                                } {
                                    true
                                } else {
                                    let a546 = v9;
                                    let a547 = 10u64;
                                    nat_eq(a546, a547)
                                } {
                                    true
                                } else {
                                    let a548 = v9;
                                    let a549 = 13u64;
                                    nat_eq(a548, a549)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a550 = v1;
                                        let a551 = 1u64;
                                        nat_add(a550, a551)?
                                    }, (v7, (false, (false, ({
                                        let a552 = v1;
                                        let a553 = 1u64;
                                        nat_add(a552, a553)?
                                    }, f198(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a554 = v1;
                                        let a555 = 1u64;
                                        nat_add(a554, a555)?
                                    }, (v7, ({
                                        let a556 = v9;
                                        let a557 = 34u64;
                                        nat_eq(a556, a557)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m558 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: Bytes, v1: u64) -> R<u64> {
    let m567 = {
        let a565 = v0;
        let a566 = v1;
        index_bytes(a565, a566)
    };
    match m567 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a612 = {
                let a588 = {
                    let a576 = {
                        let a570 = {
                            let a568 = v3;
                            let a569 = 128u8;
                            fixed_u8::bit_and(a568, a569)
                        };
                        let a571 = 128u8;
                        if equal(a570, a571) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a577 = {
                        let a574 = {
                            let a572 = v3;
                            let a573 = 64u8;
                            fixed_u8::bit_and(a572, a573)
                        };
                        let a575 = 64u8;
                        if equal(a574, a575) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a576, a577)?
                };
                let a589 = {
                    let a586 = {
                        let a580 = {
                            let a578 = v3;
                            let a579 = 32u8;
                            fixed_u8::bit_and(a578, a579)
                        };
                        let a581 = 32u8;
                        if equal(a580, a581) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a587 = {
                        let a584 = {
                            let a582 = v3;
                            let a583 = 16u8;
                            fixed_u8::bit_and(a582, a583)
                        };
                        let a585 = 16u8;
                        if equal(a584, a585) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a586, a587)?
                };
                nat_add(a588, a589)?
            };
            let a613 = {
                let a610 = {
                    let a598 = {
                        let a592 = {
                            let a590 = v3;
                            let a591 = 8u8;
                            fixed_u8::bit_and(a590, a591)
                        };
                        let a593 = 8u8;
                        if equal(a592, a593) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a599 = {
                        let a596 = {
                            let a594 = v3;
                            let a595 = 4u8;
                            fixed_u8::bit_and(a594, a595)
                        };
                        let a597 = 4u8;
                        if equal(a596, a597) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a598, a599)?
                };
                let a611 = {
                    let a608 = {
                        let a602 = {
                            let a600 = v3;
                            let a601 = 2u8;
                            fixed_u8::bit_and(a600, a601)
                        };
                        let a603 = 2u8;
                        if equal(a602, a603) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a609 = {
                        let a606 = {
                            let a604 = v3;
                            let a605 = 1u8;
                            fixed_u8::bit_and(a604, a605)
                        };
                        let a607 = 1u8;
                        if equal(a606, a607) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a608, a609)?
                };
                nat_add(a610, a611)?
            };
            nat_add(a612, a613)
        }
    }
}

pub fn f116(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f199(v0.clone(), v1, {
        let a615 = {
            let a614 = v0.clone();
            length_bytes(a614)
        };
        let a616 = 1u64;
        nat_add(a615, a616)?
    })?;
    let a617 = f115(v0.clone(), v2)?;
    let a618 = 34u64;
    if nat_eq(a617, a618) {
        Ok(Some(({
            let a619 = v2;
            let a620 = 1u64;
            nat_add(a619, a620)?
        }, f46(v0, v1, {
            let a621 = v2;
            let a622 = v1;
            nat_sub(a621, a622)
        }))))
    } else {
        f200(v0.clone(), v1, {
            let a624 = {
                let a623 = v0;
                length_bytes(a623)
            };
            let a625 = 1u64;
            nat_add(a624, a625)?
        }, Bytes::lit(&[]))
    }
}

pub fn f117(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f118(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a626 = f115(v0.clone(), v1)?;
        let a627 = 45u64;
        nat_eq(a626, a627)
    };
    let v3: u64 = if v2 {
        let a628 = v1;
        let a629 = 1u64;
        nat_add(a628, a629)?
    } else {
        v1
    };
    let v4: u64 = f201(v0.clone(), v3)?;
    let v5: u64 = {
        let a630 = v4;
        let a631 = v3;
        nat_sub(a630, a631)
    };
    let v6: bool = {
        let a632 = f115(v0.clone(), v4)?;
        let a633 = 46u64;
        nat_eq(a632, a633)
    };
    let v7: u64 = if v6 {
        let a634 = v4;
        let a635 = 1u64;
        nat_add(a634, a635)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f201(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f115(v0.clone(), v8)?;
    let v10: bool = {
        let a636 = v9;
        let a637 = 101u64;
        if nat_eq(a636, a637) {
            true
        } else {
            let a638 = v9;
            let a639 = 69u64;
            nat_eq(a638, a639)
        }
    };
    let v11: u64 = f115(v0.clone(), {
        let a640 = v8;
        let a641 = 1u64;
        nat_add(a640, a641)?
    })?;
    let v12: bool = f117(v10, f202({
        let a642 = v11;
        let a643 = 43u64;
        nat_eq(a642, a643)
    }, {
        let a644 = v11;
        let a645 = 45u64;
        nat_eq(a644, a645)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a646 = v8;
            let a647 = 2u64;
            nat_add(a646, a647)?
        } else {
            let a648 = v8;
            let a649 = 1u64;
            nat_add(a648, a649)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f201(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a650 = v14;
        let a651 = v13;
        nat_sub(a650, a651)
    });
    let v16: u64 = f203(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a653 = {
            let a652 = v15;
            length_bytes(a652)
        };
        let a654 = v16;
        nat_sub(a653, a654)
    });
    let a655 = v5;
    let a656 = 0u64;
    if if nat_eq(a655, a656) {
        true
    } else {
        if f117({
            let a657 = 1u64;
            let a658 = v5;
            nat_lt(a657, a658)
        }, {
            let a659 = f115(v0.clone(), v3)?;
            let a660 = 48u64;
            nat_eq(a659, a660)
        }) {
            true
        } else {
            if f117(v6, {
                let a661 = v8;
                let a662 = v7;
                nat_eq(a661, a662)
            }) {
                true
            } else {
                f202(if v10 {
                    let a663 = v14;
                    let a664 = v13;
                    nat_eq(a663, a664)
                } else {
                    false
                }, {
                    let a666 = 18u64;
                    let a667 = {
                        let a665 = v17.clone();
                        length_bytes(a665)
                    };
                    nat_lt(a666, a667)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m671 = {
            let a669 = {
                let a668 = v17.clone();
                length_bytes(a668)
            };
            let a670 = 0u64;
            if nat_eq(a669, a670) {
                Some(0u64)
            } else {
                f204(v17)?
            }
        };
        match m671 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m676 = f205(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a672 = v8;
                    let a673 = v7;
                    nat_sub(a672, a673)
                }), f117(v12, {
                    let a674 = v11;
                    let a675 = 45u64;
                    nat_eq(a674, a675)
                }), v18)?;
                match m676 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a679 = 19u64;
                        let a680 = {
                            let a678 = {
                                let (h677, _) = v19.clone();
                                h677
                            };
                            length_bytes(a678)
                        };
                        if nat_lt(a679, a680) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m682 = f204({
                                let (h681, _) = v19.clone();
                                h681
                            })?;
                            match m682 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f117(v2, {
                                        let a683 = 0u64;
                                        let a684 = v20;
                                        nat_lt(a683, a684)
                                    });
                                    if f206(v21, v20) {
                                        Ok(Some((v14, {
                                            let a686 = {
                                                let (_, h685) = v19.clone();
                                                h685
                                            };
                                            let a687 = 0u64;
                                            if nat_eq(a686, a687) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h688) = v19;
                                                    h688
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

pub fn f119(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h689, _) = v2.clone();
        h689
    };
    let v4: u64 = {
        let (h690, _) = {
            let (_, h691) = v2.clone();
            h691
        };
        h690
    };
    let v5: List<Bytes> = {
        let (_, h692) = {
            let (_, h693) = v2;
            h693
        };
        h692
    };
    let m694 = f207(v0.clone(), v3);
    match m694 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a695 = v6;
            let a696 = v1;
            if equal(a695, a696) {
                Ok(Some(({
                    let a697 = v3;
                    let a698 = 1u64;
                    nat_add(a697, a698)?
                }, ({
                    let a699 = v3;
                    let a700 = 1u64;
                    nat_add(a699, a700)?
                }, List::cons(f46(v0, v4, {
                    let a701 = v3;
                    let a702 = v4;
                    nat_sub(a701, a702)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a703 = v3;
                    let a704 = 1u64;
                    nat_add(a703, a704)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f120(v0: Fn1, p708: List<Bytes>, p709: List<Bytes>) -> List<Bytes> {
    let m707 = (p708, p709);
    {
        let mut m707 = m707;
        loop {
            let step = {
                let (v1, v2) = m707;
                let m705 = v2;
                match m705.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c706: Fn1 = v0.clone();
                            c706.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f122(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m710 = f208(Fn2::F209(v1), None::<Bytes>, v0);
    match m710 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f123() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("check-yaml FILE")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-admit NAME read-OP FILE ARG... CITATION...")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::cons(f8(Str::lit("verify-translations ADMISSIONS")), List::<Bytes>::nil())))))))))))))))))))))))))))))))
}

pub fn f124(v0: Adt1) -> List<Bytes> {
    if f138(match v0.clone() {
        Adt1::C0(_, h711, _, _) => {
            h711
        }
    }, f210()) {
        if f138(match v0 {
            Adt1::C0(_, h712, _, _) => {
                h712
            }
        }, f211()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f211(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f210(), List::<Bytes>::nil())
    }
}

pub fn f125(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m713 = v0;
    match m713.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f125(v3, v1))
        }
    }
}

pub fn f126(v0: Adt1) -> R<List<Bytes>> {
    Ok(f56(v0.clone(), List::cons(f212(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f136(v0)?)))))
}

pub fn f127(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f213(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f142(f96(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f128(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m714 = v0;
    match m714.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f128(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f128(v3, v1))
            }
        }
    }
}

pub fn f129(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m715 = v0;
    match m715.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f129(v3, v1)
            } else {
                List::cons(v2, f129(v3, v1))
            }
        }
    }
}

pub fn f130(v0: List<Adt2>) -> List<Bytes> {
    let m716 = v0;
    match m716.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f125(f214(f187(f93(v1, f8(Str::lit("defers"))))), f130(v2))
        }
    }
}

pub fn f131(v0: List<Adt2>) -> List<Bytes> {
    let m717 = v0;
    match m717.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m718 = f93(v1, f8(Str::lit("std")));
            match m718 {
                None => {
                    f131(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("row"))), f131(v2))
                }
            }
        }
    }
}

pub fn f132(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m719 = v0;
    match m719.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f132(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f132(v3, v1))
            }
        }
    }
}

pub fn f133(v0: Adt14, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f154(f215(v2.clone()), f216(f217(v0, v1.clone(), v2)?));
    f218(f145(f219(v1))?, v3.clone(), {
        let a721 = {
            let a720 = v3;
            length_list(a720)
        };
        let a722 = 1u64;
        nat_add(a721, a722)?
    })
}

pub fn f134(v0: Adt14, v1: List<Adt2>) -> List<Adt2> {
    let m723 = v1;
    match m723.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            if if f220(v2.clone(), f8(Str::lit("subject"))) {
                f221(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f134(v0, v3))
            } else {
                f134(v0, v3)
            }
        }
    }
}

pub fn f135(v0: Adt1, v1: List<Bytes>) -> R<List<Adt2>> {
    let m724 = v1;
    match m724.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let m725 = f222(f95(f64(v0.clone(), v2.clone()))?, f85(v2, f211()))?;
            match m725 {
                None => {
                    f135(v0, v3)
                }
                Some(v4) => {
                    Ok(f223(v4, f135(v0, v3)?))
                }
            }
        }
    }
}

pub fn f136(v0: Adt1) -> R<List<Bytes>> {
    Ok(f125(f145(f224(f210(), f147(match v0.clone() {
        Adt1::C0(_, h726, _, _) => {
            h726
        }
    }, f210())))?, f145(f224(f211(), f147(match v0 {
        Adt1::C0(_, h727, _, _) => {
            h727
        }
    }, f211())))?))
}

pub fn f137(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f138(p731: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m730 = p731;
    {
        let mut m730 = m730;
        loop {
            let step = {
                let v0 = m730;
                let m728 = v0;
                match m728.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h729, _) = v2;
                            h729
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
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

pub fn f139() -> Bytes {
    f8(Str::lit("model/registries"))
}

pub fn f140(v0: Adt1) -> R<List<Bytes>> {
    f145(f224(f139(), f147(match v0 {
        Adt1::C0(_, h732, _, _) => {
            h732
        }
    }, f139())))
}

pub fn f141(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m733 = v1;
    match m733.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f125(if f52(v4.clone(), v3.clone()) {
                f225(v4.clone(), f95(f64(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f226(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
            }, f141(v0, v5, v2, v3)?))
        }
    }
}

pub fn f142(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m734 = v0;
    match m734.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f142(v3, v1))
        }
    }
}

pub fn f143(v0: Adt1) -> R<Adt12> {
    let m735 = f56(v0.clone(), f227());
    match m735.uncons() {
        None => {
            let m736 = f228(v0.clone(), f227())?;
            match m736.uncons() {
                None => {
                    let m739 = f229(match v0.clone() {
                        Adt1::C0(_, h737, _, _) => {
                            h737
                        }
                    }, f125(f230(), f231(match v0.clone() {
                        Adt1::C0(_, h738, _, _) => {
                            h738
                        }
                    }, false)));
                    match m739.uncons() {
                        None => {
                            let m741 = f56(v0.clone(), f231(match v0.clone() {
                                Adt1::C0(_, h740, _, _) => {
                                    h740
                                }
                            }, true));
                            match m741.uncons() {
                                None => {
                                    let m742 = f56(v0.clone(), f232());
                                    match m742.uncons() {
                                        None => {
                                            let m743 = f233(v0.clone())?;
                                            match m743 {
                                                None => {
                                                    let m745 = f234(v0.clone(), f231(match v0.clone() {
                                                        Adt1::C0(_, h744, _, _) => {
                                                            h744
                                                        }
                                                    }, true))?;
                                                    match m745 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f235(Adt16::C0(Adt15::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f213(v0.clone())?, f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f96(v0.clone(), f8(Str::lit("model/strata.json")))?, f214(f96(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f231(match v0 {
                                                                Adt1::C0(_, h746, _, _) => {
                                                                    h746
                                                                }
                                                            }, true)))?;
                                                            let (h747, _) = v13;
                                                            if h747 {
                                                                Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h748) = v13;
                                                                    h748
                                                                }))
                                                            } else {
                                                                Ok(Adt12::C4({
                                                                    let (_, h749) = v13;
                                                                    h749
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

pub fn f144() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f145(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f236(f120(Fn1::F237, List::<Bytes>::nil(), f238(v0)?)))
}

pub fn f146(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m750 = v1;
    match m750.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f239(v0.clone(), v2), f146(v0, v3))
            } else {
                f146(v0, v3)
            }
        }
    }
}

pub fn f147(p755: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m754 = p755;
    {
        let mut m754 = m754;
        loop {
            let step = {
                let v0 = m754;
                let m751 = v0;
                match m751.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h752, _) = v2;
                            h752
                        }, v1.clone()) {
                            let (_, h753) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h753)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m754 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, p758: List<Bytes>) -> R<Option<Bytes>> {
    let m757 = p758;
    {
        let mut m757 = m757;
        loop {
            let step = {
                let v2 = m757;
                let m756 = v2;
                match m756.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f240(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m757 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: List<List<Bytes>>) -> R<List<(List<Bytes>, List<Bytes>)>> {
    let m759 = v0;
    match m759.uncons() {
        None => {
            Ok(List::<(List<Bytes>, List<Bytes>)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f241(v1.clone())?, v1), f149(v2)?))
        }
    }
}

pub fn f150(p762: List<Bytes>, p763: List<Bytes>, p764: List<List<Bytes>>) -> List<List<Bytes>> {
    let m761 = (p762, (p763, p764));
    {
        let mut m761 = m761;
        loop {
            let step = {
                let (v0, (v1, v2)) = m761;
                let m760 = v0;
                match m760.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f242(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f242(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m765 = v0;
    match m765.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f151(v3, v1)
            } else {
                List::cons(v2, f151(v3, v1))
            }
        }
    }
}

pub fn f152(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h766, _, _) => {
            h766
        }
    }, f243())) {
        Ok(Adt12::C1(List::cons(f243(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f146(f243(), f147(match v0.clone() {
            Adt1::C0(_, h767, _, _) => {
                h767
            }
        }, f243())))?, v1.clone());
        let m768 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m768.uncons() {
            None => {
                let v5: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m769 = f56(v0.clone(), f244(v0.clone(), v2.clone(), v5.clone())?);
                match m769.uncons() {
                    None => {
                        let m770 = f245(v0.clone(), v2, v5)?;
                        match m770 {
                            None => {
                                f246(v0, v1)
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

pub fn f153() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f154(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m771 = v0;
    match m771.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f154(v3, v1))
        }
    }
}

pub fn f155(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m772 = f96(v0.clone(), f153())?;
    match m772.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f247(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f156() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f157() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          # Each encoding admitted through admit-translated recorded the reader command that printed it: run it again and fail if the output differs.\u{a}          target/release/verdant_readers verify-translations model/admissions.json\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f158() -> Bytes {
    f7(List::cons(f248(), List::cons(f249(), List::cons(f250(), List::cons(f251(), List::cons(f252(), List::cons(f253(), List::<Bytes>::nil())))))))
}

pub fn f159() -> Bytes {
    f7(List::cons(f254(), List::cons(f255(), List::cons(f256(), List::cons(f257(), List::<Bytes>::nil())))))
}

pub fn f160() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, and readers when it exists.\u{a}pids=\"\"\u{a}for project in studio readers; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.$project\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}"))
}

pub fn f161() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f162(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f127(v0.clone())?;
    let v2: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt2> = f134(v1.clone(), f135(v0.clone(), f136(v0.clone())?)?);
    let v4: List<List<Bytes>> = f258(f259(f64(v0, f212()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f260(v4.clone(), f261(v4), f217(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f133(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f163(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f262(f127(v0.clone())?, f135(v0.clone(), f136(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f164(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f263(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f264(f96(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f165() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f166(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m773 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m773.uncons() {
        None => {
            if f265(f95(f64(v0, v1))?)? {
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

pub fn f167(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m774 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m774.uncons() {
        None => {
            let v5: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f266(v5)?
                } else {
                    f267(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f239(f268(), f269(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f239(f268(), f269(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f270()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f168(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m775 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m775.uncons() {
        None => {
            let m776 = f271(f95(f64(v0.clone(), v1.clone()))?)?;
            match m776 {
                None => {
                    Ok(Adt12::C4(f272()))
                }
                Some(v4) => {
                    let m777 = f273(f95(f64(v0, v1))?)?;
                    match m777 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f274(v5)))
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

pub fn f169(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let v2: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
        Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
    } else {
        if f117(f275(f276(v2.clone())?), f51({
            let a779 = {
                let a778 = f277(f276(v2)?);
                length_list(a778)
            };
            let a780 = 0u64;
            nat_eq(a779, a780)
        })) {
            Ok(Adt12::C3(List::cons((f239(f268(), f269(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f239(f268(), f269(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt12::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f170(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m781 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m781.uncons() {
        None => {
            let m782 = f278(f95(f64(v0, v1))?)?;
            match m782 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f171(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m783 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m783.uncons() {
        None => {
            let v5: Bytes = f239(if v2 {
                f211()
            } else {
                f210()
            }, f269(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m784 = f222(f95(f64(v0.clone(), v1.clone()))?, v2)?;
                match m784 {
                    None => {
                        Ok(Adt12::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a786 = {
                            let a785 = v6;
                            length_list(a785)
                        };
                        let a787 = 0u64;
                        if nat_eq(a786, a787) {
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

pub fn f172(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m788 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m788.uncons() {
        None => {
            let v4: List<Bytes> = f95(f64(v0.clone(), v1))?;
            let m789 = f279(v4.clone(), f8(Str::lit("requirement")))?;
            match m789 {
                None => {
                    Ok(Adt12::C4(f270()))
                }
                Some(v5) => {
                    let m790 = f279(v4, f8(Str::lit("admission")))?;
                    match m790 {
                        None => {
                            Ok(Adt12::C4(f270()))
                        }
                        Some(v6) => {
                            let v7: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f125(f142(v5.clone(), f8(Str::lit("id"))), f142(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f145(f125(f142(v7.clone(), f8(Str::lit("id"))), f142(v8.clone(), f8(Str::lit("construct")))))?;
                            let m791 = f280(f131(v6.clone()), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m791 {
                                None => {
                                    let a793 = {
                                        let a792 = v9.clone();
                                        length_list(a792)
                                    };
                                    let a794 = 0u64;
                                    if nat_eq(a793, a794) {
                                        Ok(Adt12::C4(f270()))
                                    } else {
                                        if f51({
                                            let a797 = {
                                                let a795 = f145(f125(v9.clone(), v10.clone()))?;
                                                length_list(a795)
                                            };
                                            let a798 = {
                                                let a796 = f125(v9.clone(), v10);
                                                length_list(a796)
                                            };
                                            nat_eq(a797, a798)
                                        }) {
                                            Ok(Adt12::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt12::C3(List::cons(f282(f8(Str::lit("model/requirements.json")), f223(v7, v5))?, List::cons(f282(f8(Str::lit("model/admissions.json")), f223(v8, v6))?, List::cons(f283(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f122(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f281(v11))
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

pub fn f173(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m799 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m799.uncons() {
        None => {
            let m800 = f91(v0.clone(), v1.clone())?;
            match m800 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m801 = f93(v4.clone(), f8(Str::lit("schema")));
                    match m801 {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f284(v0, f100(v5.clone())?, v5, f58(v4.clone(), f8(Str::lit("name"))), f214(f187(f93(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f174(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m802 = v2;
    match m802.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m803 = v4;
            match m803.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m804 = v6.clone();
                    match m804.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((v7, v8)) => {
                            let m805 = v8;
                            match m805.uncons() {
                                None => {
                                    Ok(Adt12::C4(f79()))
                                }
                                Some((v9, _)) => {
                                    if f51(f85(v7, f8(Str::lit("read-")))) {
                                        Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that reads the artifact: admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT\u{a}"))))
                                    } else {
                                        let m806 = f56(v0.clone(), List::cons(f285(v1.clone()), List::cons(v9.clone(), List::cons(f8(Str::lit("model/admissions.json")), if f32(v5.clone(), f8(Str::lit("none"))) {
                                            List::<Bytes>::nil()
                                        } else {
                                            List::cons(f7(List::cons(f8(Str::lit("encodings/")), List::cons(v5.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), List::<Bytes>::nil())
                                        }))));
                                        match m806.uncons() {
                                            None => {
                                                if f51(f67(v0.clone(), f285(v1.clone()))) {
                                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v1.clone(), List::cons(f8(Str::lit(" is not a bound authority, because ")), List::cons(f285(v1), List::cons(f8(Str::lit(" is absent, and every admitted value cites a bound standard (REQ-ADMISSION)\u{a}")), List::<Bytes>::nil()))))))))
                                                } else {
                                                    let m807 = f57(v0.clone(), f285(v1.clone()))?;
                                                    match m807 {
                                                        None => {
                                                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(f285(v1), List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                        Some(v13) => {
                                                            let v15: Bytes = {
                                                                let m808 = f93(v13, f8(Str::lit("source")));
                                                                match m808 {
                                                                    None => {
                                                                        Bytes::lit(&[])
                                                                    }
                                                                    Some(v14) => {
                                                                        f58(v14, f8(Str::lit("sha256")))
                                                                    }
                                                                }
                                                            };
                                                            let v16: Bytes = f286(f64(v0.clone(), v9.clone()))?;
                                                            if f51(f32(v16.clone(), v15.clone())) {
                                                                Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v9, List::cons(f8(Str::lit(" is not the artifact ")), List::cons(v1, List::cons(f8(Str::lit(" binds: its SHA-256 is ")), List::cons(v16, List::cons(f8(Str::lit(" and the binding's source.sha256 is ")), List::cons(v15, List::cons(f8(Str::lit(" (FIPS 180-4 section 6.2)\u{a}")), List::<Bytes>::nil()))))))))))))
                                                            } else {
                                                                f287(v0, v1, v3, v5, v6, v9)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Some((v11, v12)) => {
                                                Ok(Adt12::C0(List::cons(v11, v12)))
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

pub fn f175(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m809 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m809.uncons() {
        None => {
            let m810 = f57(v0.clone(), v1.clone())?;
            match m810 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f288())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt2 = {
                            let m811 = f93(v4.clone(), f8(Str::lit("source")));
                            match m811 {
                                None => {
                                    Adt2::C9
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f92(f93(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f239(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f239(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m812 = v8;
                        match m812 {
                            None => {
                                f289(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f289(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f239(v9, f269(f58(v7.clone(), f8(Str::lit("url"))))?);
                                    if f51(f52(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h813, _) => {
                                            h813
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f58(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m814 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m814.uncons() {
                                            None => {
                                                if f32(f286(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f289(v0, v4, v5, v10, Some((v12, v11)))
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
    let m815 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m815.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f259(v4.clone())?;
            if f51(f98(f290(v5.clone()), f291())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f292(Str::lit("id"), v6.clone())?, List::cons(f292(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f292(Str::lit("artifact"), f212())?, List::cons(f292(Str::lit("checksum"), f286(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f212(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f293(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f283(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f122(f261(f258(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f177(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m816 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m816.uncons() {
        None => {
            let m817 = f57(v0, v1)?;
            match m817 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f294(f187(Some(v4))) {
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
    let m818 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m818.uncons() {
        None => {
            let m819 = f91(v0.clone(), v1.clone())?;
            match m819 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f284(v0.clone(), f64(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f179(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt12> {
    let m820 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil())))))));
    match m820.uncons() {
        None => {
            let m821 = f295(v3.clone());
            match m821.uncons() {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                }
                Some((v6, v7)) => {
                    if f85(v6.clone(), f8(Str::lit("read-"))) {
                        f296(v0.clone(), f64(v0, v1), v2, List::cons(v6, v7), f297(v3))
                    } else {
                        Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                    }
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f180(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m822 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f153(), List::<Bytes>::nil())));
    match m822.uncons() {
        None => {
            let m823 = f298(f95(f64(v0.clone(), v1))?)?;
            match m823 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt2> = f96(v0, f153())?;
                    let v6: List<Bytes> = f142(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f145(f142(v5.clone(), f8(Str::lit("at"))))?;
                    let a825 = {
                        let a824 = v6.clone();
                        length_list(a824)
                    };
                    let a826 = 0u64;
                    if if nat_eq(a825, a826) {
                        true
                    } else {
                        f51({
                            let a829 = {
                                let a827 = f145(f125(v6.clone(), v7.clone()))?;
                                length_list(a827)
                            };
                            let a830 = {
                                let a828 = f125(v6.clone(), v7);
                                length_list(a828)
                            };
                            nat_eq(a829, a830)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f282(f153(), f223(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f122(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f181(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m831 = v2;
    match m831.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m832 = v4;
            match m832.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m833 = v6.clone();
                    match m833.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((_, _)) => {
                            let m834 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f299(), List::<Bytes>::nil())));
                            match m834.uncons() {
                                None => {
                                    let v11: Bytes = f122(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt2> = f96(v0, f299())?;
                                    if f300(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f282(f299(), f223(v13, List::cons(f301(v11.clone(), v12, v5, v3), List::<Adt2>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
                                    }
                                }
                                Some((v9, v10)) => {
                                    Ok(Adt12::C0(List::cons(v9, v10)))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f182(v0: Bytes, p888: u64, p889: u64) -> R<bool> {
    let m887 = (p888, p889);
    {
        let mut m887 = m887;
        loop {
            let step = {
                let (v1, v2) = m887;
                let m835 = v2;
                if m835 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m835 - 1;
                    let m838 = {
                        let a836 = v0.clone();
                        let a837 = v1;
                        index_bytes(a836, a837)
                    };
                    match m838 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f302({
                                let v5: u8 = v4;
                                let a883 = {
                                    let a859 = {
                                        let a847 = {
                                            let a841 = {
                                                let a839 = v5;
                                                let a840 = 128u8;
                                                fixed_u8::bit_and(a839, a840)
                                            };
                                            let a842 = 128u8;
                                            if equal(a841, a842) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a848 = {
                                            let a845 = {
                                                let a843 = v5;
                                                let a844 = 64u8;
                                                fixed_u8::bit_and(a843, a844)
                                            };
                                            let a846 = 64u8;
                                            if equal(a845, a846) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a847, a848)?
                                    };
                                    let a860 = {
                                        let a857 = {
                                            let a851 = {
                                                let a849 = v5;
                                                let a850 = 32u8;
                                                fixed_u8::bit_and(a849, a850)
                                            };
                                            let a852 = 32u8;
                                            if equal(a851, a852) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a858 = {
                                            let a855 = {
                                                let a853 = v5;
                                                let a854 = 16u8;
                                                fixed_u8::bit_and(a853, a854)
                                            };
                                            let a856 = 16u8;
                                            if equal(a855, a856) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a857, a858)?
                                    };
                                    nat_add(a859, a860)?
                                };
                                let a884 = {
                                    let a881 = {
                                        let a869 = {
                                            let a863 = {
                                                let a861 = v5;
                                                let a862 = 8u8;
                                                fixed_u8::bit_and(a861, a862)
                                            };
                                            let a864 = 8u8;
                                            if equal(a863, a864) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a870 = {
                                            let a867 = {
                                                let a865 = v5;
                                                let a866 = 4u8;
                                                fixed_u8::bit_and(a865, a866)
                                            };
                                            let a868 = 4u8;
                                            if equal(a867, a868) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a869, a870)?
                                    };
                                    let a882 = {
                                        let a879 = {
                                            let a873 = {
                                                let a871 = v5;
                                                let a872 = 2u8;
                                                fixed_u8::bit_and(a871, a872)
                                            };
                                            let a874 = 2u8;
                                            if equal(a873, a874) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a880 = {
                                            let a877 = {
                                                let a875 = v5;
                                                let a876 = 1u8;
                                                fixed_u8::bit_and(a875, a876)
                                            };
                                            let a878 = 1u8;
                                            if equal(a877, a878) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a879, a880)?
                                    };
                                    nat_add(a881, a882)?
                                };
                                nat_add(a883, a884)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a885 = v1;
                                    let a886 = 1u64;
                                    nat_add(a885, a886)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(p894: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m893 = p894;
    {
        let mut m893 = m893;
        loop {
            let step = {
                let v0 = m893;
                let m890 = v0;
                match m890.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h891, _) = v2;
                            h891
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h892) = v2;
                                h892
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m898 = {
        let a896 = match v0 {
            Adt1::C0(_, _, _, h895) => {
                h895
            }
        };
        let a897 = f40(v1);
        index_list(a896, a897)
    };
    match m898 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f185(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), v1) {
        let m899 = f44(v0, 34u8)?;
        match m899.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m900 = v3;
                match m900.uncons() {
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

pub fn f186(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f303(v0, v1)
}

pub fn f187(v0: Option<Adt2>) -> List<Adt2> {
    let m901 = v0;
    match m901 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f304(v1)
        }
    }
}

pub fn f188(v0: Adt2, v1: Bytes) -> Adt2 {
    let m902 = v0;
    match m902 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r903) => {
            let v12 = (*r903).clone();
            Adt2::C1(v11, std::rc::Rc::new(v12))
        }
        Adt2::C2(v6) => {
            Adt2::C2(v6)
        }
        Adt2::C3(v7, v8) => {
            Adt2::C3(v7, v8)
        }
        Adt2::C4(v13, v14, v15) => {
            Adt2::C4(v13, v14, v15)
        }
        Adt2::C5(v16, v17) => {
            Adt2::C5(v16, v17)
        }
        Adt2::C6(v9) => {
            Adt2::C6(v9)
        }
        Adt2::C7(r904) => {
            let v10 = (*r904).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r905) => {
            let v5 = (*r905).clone();
            Adt2::C8(std::rc::Rc::new(f188(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r906, r907) => {
            let v18 = (*r906).clone();
            let v19 = (*r907).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r908, r909) => {
            let v3 = (*r908).clone();
            let v4 = (*r909).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt2::C11(v2, std::rc::Rc::new(f305(v3, v1.clone())), std::rc::Rc::new(f188(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f188(v4, v1)))
            }
        }
    }
}

pub fn f189(v0: Adt2) -> R<Bytes> {
    let m910 = v0;
    match m910 {
        Adt2::C0 => {
            Ok(Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
        }
        Adt2::C1(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C2(v3) => {
            if v3 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt2::C3(v4, v5) => {
            f306(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f307(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f308(v9, v10)
        }
        Adt2::C6(v11) => {
            f309(v11)
        }
        Adt2::C7(r911) => {
            let v12 = (*r911).clone();
            Ok(f310(f311(v12)?))
        }
        Adt2::C8(r912) => {
            let v13 = (*r912).clone();
            f312(f313(v13)?)
        }
        Adt2::C9 => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C10(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C11(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f190(v0: Adt1, p916: List<Adt13>) -> R<Option<Adt13>> {
    let m915 = p916;
    {
        let mut m915 = m915;
        loop {
            let step = {
                let v1 = m915;
                let m913 = v1;
                match m913.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f191(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h914, _) => {
                                h914
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
                Ok(next) => { m915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Adt13) -> R<Bytes> {
    Ok(f239(f314(), f269(f315(v0))?))
}

pub fn f192(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f315(v0), List::<Bytes>::nil())))))
}

pub fn f193() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f194() -> Bytes {
    f316()
}

pub fn f195(v0: List<Adt13>) -> R<List<Bytes>> {
    let m917 = v0;
    match m917.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f191(v1)?, f195(v2)?))
        }
    }
}

pub fn f196(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m918 = v1;
    match m918.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f317(v5.clone(), v2.clone(), v3.clone(), f286(f64(v0.clone(), f191(v5)?))?, v4.clone())?), f196(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f197(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f318(v0.clone())?;
    let v2: u64 = f319(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a919 = v1;
        let a920 = v2;
        nat_sub(a919, a920)
    }))
}

pub fn f198(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a923 = v3;
    let a924 = f46(v0, v1, {
        let a921 = v2;
        let a922 = v1;
        nat_sub(a921, a922)
    });
    append_bytes(a923, a924)
}

pub fn f199(v0: Bytes, p937: u64, p938: u64) -> R<u64> {
    let m936 = (p937, p938);
    {
        let mut m936 = m936;
        loop {
            let step = {
                let (v1, v2) = m936;
                let m925 = v2;
                if m925 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m925 - 1;
                    let v4: u64 = f115(v0.clone(), v1)?;
                    let a926 = v4;
                    let a927 = 34u64;
                    if nat_eq(a926, a927) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a928 = v4;
                        let a929 = 92u64;
                        if nat_eq(a928, a929) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a930 = v4;
                            let a931 = 32u64;
                            if nat_lt(a930, a931) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a932 = v4;
                                let a933 = 256u64;
                                if nat_eq(a932, a933) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a934 = v1;
                                        let a935 = 1u64;
                                        nat_add(a934, a935)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: Bytes, p964: u64, p965: u64, p966: Bytes) -> R<Option<(u64, Bytes)>> {
    let m963 = (p964, (p965, p966));
    {
        let mut m963 = m963;
        loop {
            let step = {
                let (v1, (v2, v3)) = m963;
                let m939 = v2;
                if m939 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m939 - 1;
                    let v5: u64 = f199(v0.clone(), v1, {
                        let a943 = {
                            let a941 = {
                                let a940 = v0.clone();
                                length_bytes(a940)
                            };
                            let a942 = v1;
                            nat_sub(a941, a942)
                        };
                        let a944 = 1u64;
                        nat_add(a943, a944)?
                    })?;
                    let v6: Bytes = {
                        let a947 = v3;
                        let a948 = f46(v0.clone(), v1, {
                            let a945 = v5;
                            let a946 = v1;
                            nat_sub(a945, a946)
                        });
                        append_bytes(a947, a948)
                    };
                    let v7: u64 = f115(v0.clone(), v5)?;
                    let a949 = v7;
                    let a950 = 34u64;
                    if nat_eq(a949, a950) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a951 = v5;
                            let a952 = 1u64;
                            nat_add(a951, a952)?
                        }, v6))))
                    } else {
                        let a953 = v7;
                        let a954 = 92u64;
                        if nat_eq(a953, a954) {
                            let m955 = f320(v0.clone(), v5, v6)?;
                            match m955 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a958 = {
                                        let (_, h956) = {
                                            let (_, h957) = v8.clone();
                                            h957
                                        };
                                        h956
                                    };
                                    let a959 = 0u64;
                                    if nat_eq(a958, a959) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h960, _) = v8;
                                            h960
                                        }, (v4, {
                                            let (h961, _) = {
                                                let (_, h962) = v8;
                                                h962
                                            };
                                            h961
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
                Ok(next) => { m963 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: Bytes, v1: u64) -> R<u64> {
    let (h967, _) = f321(Fn3::F322(v0.clone()), {
        let a969 = {
            let a968 = v0;
            length_bytes(a968)
        };
        let a970 = 1u64;
        nat_add(a969, a970)?
    }, v1)?;
    Ok(h967)
}

pub fn f202(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f203(v0: Bytes) -> R<u64> {
    let (h971, _) = f321(Fn3::F323(v0.clone()), {
        let a973 = {
            let a972 = v0;
            length_bytes(a972)
        };
        let a974 = 1u64;
        nat_add(a973, a974)?
    }, 0u64)?;
    Ok(h971)
}

pub fn f204(v0: Bytes) -> R<Option<u64>> {
    if f324(v0.clone())? {
        Ok(Some({
            let (_, h975) = {
                let (h976, _) = f325(Fn4::F326(v0.clone()), {
                    let a977 = v0;
                    length_bytes(a977)
                }, (0u64, 0u64))?;
                h976
            };
            h975
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f205(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a979 = {
            let a978 = v1.clone();
            length_bytes(a978)
        };
        let a980 = f327(v1)?;
        nat_sub(a979, a980)
    });
    let v5: Bytes = {
        let a981 = v0;
        let a982 = v4.clone();
        append_bytes(a981, a982)
    };
    let v6: u64 = f203(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a984 = {
            let a983 = v5;
            length_bytes(a983)
        };
        let a985 = v6;
        nat_sub(a984, a985)
    });
    let v8: u64 = {
        let a986 = v4;
        length_bytes(a986)
    };
    let v9: u64 = {
        let a987 = v7.clone();
        length_bytes(a987)
    };
    let a988 = v9;
    let a989 = 0u64;
    if nat_eq(a988, a989) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f202(v2, {
            let a990 = v3;
            let a991 = v8;
            nat_lt(a990, a991)
        }) {
            let v10: u64 = if v2 {
                let a992 = v3;
                let a993 = v8;
                nat_add(a992, a993)?
            } else {
                let a994 = v8;
                let a995 = v3;
                nat_sub(a994, a995)
            };
            let v11: u64 = f327(v7.clone())?;
            let a996 = v10;
            let a997 = v11;
            if nat_le(a996, a997) {
                Ok(Some((f46(v7, 0u64, {
                    let a998 = v9;
                    let a999 = v10;
                    nat_sub(a998, a999)
                }), 0u64)))
            } else {
                let a1002 = {
                    let a1000 = v10;
                    let a1001 = v11;
                    nat_sub(a1000, a1001)
                };
                let a1003 = 19u64;
                if nat_le(a1002, a1003) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1004 = v9;
                        let a1005 = v11;
                        nat_sub(a1004, a1005)
                    }), {
                        let a1006 = v10;
                        let a1007 = v11;
                        nat_sub(a1006, a1007)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1008 = v3;
                let a1009 = v8;
                nat_sub(a1008, a1009)
            };
            let a1012 = {
                let a1010 = v9;
                let a1011 = v12;
                nat_add(a1010, a1011)?
            };
            let a1013 = 19u64;
            if nat_le(a1012, a1013) {
                Ok(Some(({
                    let a1014 = v7;
                    let a1015 = f328(v12);
                    append_bytes(a1014, a1015)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f206(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1016 = 0u64;
        let a1017 = v1;
        if nat_lt(a1016, a1017) {
            let a1018 = v1;
            let a1019 = 9223372036854775808u64;
            nat_le(a1018, a1019)
        } else {
            false
        }
    } else {
        let a1020 = v1;
        let a1021 = 9223372036854775807u64;
        nat_le(a1020, a1021)
    }
}

pub fn f207(v0: Bytes, v1: u64) -> Option<u8> {
    let a1022 = v0;
    let a1023 = v1;
    index_bytes(a1022, a1023)
}

pub fn f208(v0: Fn2, p1027: Option<Bytes>, p1028: List<Bytes>) -> Option<Bytes> {
    let m1026 = (p1027, p1028);
    {
        let mut m1026 = m1026;
        loop {
            let step = {
                let (v1, v2) = m1026;
                let m1024 = v2;
                match m1024.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1025: Fn2 = v0.clone();
                            c1025.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1029 = v1;
    match m1029 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1032 = {
                    let a1030 = v3;
                    let a1031 = v0;
                    append_bytes(a1030, a1031)
                };
                let a1033 = v2;
                append_bytes(a1032, a1033)
            })
        }
    }
}

pub fn f210() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f211() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f212() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f213(v0: Adt1) -> R<List<Bytes>> {
    let m1034 = f259(f64(v0, f212()))?;
    match m1034.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f261(v2))
        }
    }
}

pub fn f214(v0: List<Adt2>) -> List<Bytes> {
    let m1035 = v0;
    match m1035.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1036 = f92(Some(v1));
            match m1036 {
                None => {
                    f214(v2)
                }
                Some(v3) => {
                    List::cons(v3, f214(v2))
                }
            }
        }
    }
}

pub fn f215(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1037 = v0;
    match m1037.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f215(v2))
        }
    }
}

pub fn f216(v0: List<Adt17>) -> List<(Bytes, Bytes)> {
    let m1038 = v0;
    match m1038.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt17::C0(h1039, _, _) => {
                    h1039
                }
            }, match v1 {
                Adt17::C0(_, _, h1040) => {
                    h1040
                }
            }), f216(v2))
        }
    }
}

pub fn f217(v0: Adt14, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Adt17>> {
    let v3: List<(Bytes, Bytes)> = f329(v1);
    let v4: List<(Bytes, Bytes)> = f330(v0, v2);
    f331(v3.clone(), f154(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f218(p1047: List<Bytes>, v1: List<(Bytes, Bytes)>, p1048: u64) -> R<List<Bytes>> {
    let m1046 = (p1047, p1048);
    {
        let mut m1046 = m1046;
        loop {
            let step = {
                let (v0, v2) = m1046;
                let m1041 = v2;
                if m1041 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1041 - 1;
                    let v4: List<Bytes> = f145(f125(v0.clone(), f332(v0.clone(), v1.clone())))?;
                    let a1044 = {
                        let a1042 = v4.clone();
                        length_list(a1042)
                    };
                    let a1045 = {
                        let a1043 = v0.clone();
                        length_list(a1043)
                    };
                    if nat_eq(a1044, a1045) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1046 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: List<Adt2>) -> List<Bytes> {
    let m1049 = v0;
    match m1049.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1050 = f93(v1, f8(Str::lit("std")));
            match m1050 {
                None => {
                    f219(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f220(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f219(v2))
                    } else {
                        f219(v2)
                    })
                }
            }
        }
    }
}

pub fn f220(v0: Adt2, v1: Bytes) -> bool {
    f51({
        let a1052 = {
            let a1051 = f58(v0, v1);
            length_bytes(a1051)
        };
        let a1053 = 0u64;
        nat_eq(a1052, a1053)
    })
}

pub fn f221(v0: Adt14, v1: Adt2) -> bool {
    if f333(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f333(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f222(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt2>>> {
    let m1054 = v0;
    match m1054.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f197(v2)?;
            let a1056 = {
                let a1055 = v4.clone();
                length_bytes(a1055)
            };
            let a1057 = 0u64;
            if nat_eq(a1056, a1057) {
                f222(v3, v1)
            } else {
                let m1058 = f43(v4)?;
                match m1058 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f334(v5.clone())
                        } else {
                            f335(v5.clone())
                        } {
                            let m1059 = f222(v3, v1)?;
                            match m1059 {
                                None => {
                                    Ok(None::<List<Adt2>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons(v5, v6)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt2>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f223(v0: List<Adt2>, v1: List<Adt2>) -> List<Adt2> {
    let m1060 = v0;
    match m1060.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f223(v3, v1))
        }
    }
}

pub fn f224(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1061 = v1;
    match m1061.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f239(v0.clone(), v2), f224(v0, v3))
            } else {
                f224(v0, v3)
            }
        }
    }
}

pub fn f225(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1062 = v1;
    match m1062.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f125({
                let a1064 = {
                    let a1063 = f197(v4.clone())?;
                    length_bytes(a1063)
                };
                let a1065 = 0u64;
                if nat_eq(a1064, a1065) {
                    List::<Bytes>::nil()
                } else {
                    f336(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f225(v0, v5, v2, v3)?))
        }
    }
}

pub fn f226(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f227() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f212(), List::<Bytes>::nil())))))))
}

pub fn f228(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1066 = v1;
    match m1066.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m1067 = f57(v0.clone(), v2.clone())?;
                match m1067 {
                    None => {
                        Ok(List::cons(v2, f228(v0, v3)?))
                    }
                    Some(_) => {
                        f228(v0, v3)
                    }
                }
            } else {
                f228(v0, v3)
            }
        }
    }
}

pub fn f229(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m1068 = v1;
    match m1068.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f138(v0.clone(), v2.clone()) {
                f229(v0, v3)
            } else {
                List::cons(v2, f229(v0, v3))
            }
        }
    }
}

pub fn f230() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f231(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m1069 = v0;
    match m1069.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f125(f337({
                let (h1070, _) = v2;
                h1070
            }, {
                let (_, h1071) = v2;
                h1071
            }, v1), f231(v3, v1))
        }
    }
}

pub fn f232() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f233(v0: Adt1) -> R<Option<Bytes>> {
    f338(f339(v0, f232())?)
}

pub fn f234(v0: Adt1, p1075: List<Bytes>) -> R<Option<Bytes>> {
    let m1074 = p1075;
    {
        let mut m1074 = m1074;
        loop {
            let step = {
                let v1 = m1074;
                let m1072 = v1;
                match m1072.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1073 = f340(f64(v0.clone(), v2.clone()), f341())?;
                        match m1073 {
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
                Ok(next) => { m1074 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h1076, _, _, _, _) => {
            h1076
        }
    };
    let v2: Adt18 = f342(v1.clone())?;
    let v3: List<Bytes> = f343(v1.clone());
    let v4: List<Bytes> = f142(match v0.clone() {
        Adt16::C0(_, _, h1077, _, _) => {
            h1077
        }
    }, f8(Str::lit("path")));
    let m1079 = f344(v2, List::<u64>::nil(), f345(v1.clone(), match v0.clone() {
        Adt16::C0(_, h1078, _, _, _) => {
            h1078
        }
    }, 0u64)?);
    match m1079 {
        None => {
            if f347(f348(match v0.clone() {
                Adt16::C0(_, _, _, _, h1081) => {
                    h1081
                }
            }, f125(match v0.clone() {
                Adt16::C0(_, _, _, _, h1082) => {
                    h1082
                }
            }, v4.clone()))?, f348(v4.clone(), f125(match v0.clone() {
                Adt16::C0(_, _, _, _, h1083) => {
                    h1083
                }
            }, v4.clone()))?, f349(match v0.clone() {
                Adt16::C0(_, _, h1084, _, _) => {
                    h1084
                }
            })) {
                let m1088 = f350(f348(match v0.clone() {
                    Adt16::C0(_, _, _, h1085, _) => {
                        h1085
                    }
                }, f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h1086, _) => {
                        h1086
                    }
                }))?, f348(v3.clone(), f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h1087, _) => {
                        h1087
                    }
                }))?);
                match m1088 {
                    None => {
                        let m1091 = f351(f142(match v0 {
                            Adt16::C0(_, h1090, _, _, _) => {
                                h1090
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m1091 {
                            None => {
                                let m1092 = f351(v4, List::<Bytes>::nil());
                                match m1092 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f122(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f346(f125(v3, match v0 {
                            Adt16::C0(_, _, _, h1089, _) => {
                                h1089
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f346(match v1 {
                Adt15::C0(_, _, _, _, h1080) => {
                    h1080
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f236(v0: List<Bytes>) -> List<Bytes> {
    f120(Fn1::F352, List::<Bytes>::nil(), v0)
}

pub fn f237(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f353(v0, v1)
}

pub fn f238(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f354(f355(f356(v0))?))
}

pub fn f239(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f240(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1093 = f57(v0, v2)?;
    match m1093 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f357(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1094 = v4.clone();
            match m1094.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1096 = f92(f93({
                        let m1095 = f93(v3.clone(), f8(Str::lit("source")));
                        match m1095 {
                            None => {
                                Adt2::C9
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1096 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f358(f359(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f241(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1097 = v0;
    match m1097.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f360(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1101 = f44(f46(v1.clone(), 6u64, {
                    let a1099 = {
                        let a1098 = v1;
                        length_bytes(a1098)
                    };
                    let a1100 = 6u64;
                    nat_sub(a1099, a1100)
                }), 34u8)?;
                match m1101.uncons() {
                    None => {
                        f241(v2)
                    }
                    Some((v3, v4)) => {
                        let m1102 = v4;
                        match m1102.uncons() {
                            None => {
                                f241(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f241(v2)?))
                            }
                        }
                    }
                }
            } else {
                f241(v2)
            }
        }
    }
}

pub fn f242(p1105: List<Bytes>, p1106: List<Bytes>) -> List<Bytes> {
    let m1104 = (p1105, p1106);
    {
        let mut m1104 = m1104;
        loop {
            let step = {
                let (v0, v1) = m1104;
                let m1103 = v0;
                match m1103.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f244(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1107 = v1;
    match m1107.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f361(v0.clone(), v3)?;
            if f52(f362(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f363(v5)?, f244(v0, v4, v2)?))
            } else {
                f244(v0, v4, v2)
            }
        }
    }
}

pub fn f245(v0: Adt1, p1111: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1110 = p1111;
    {
        let mut m1110 = m1110;
        loop {
            let step = {
                let v1 = m1110;
                let m1108 = v1;
                match m1108.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f361(v0.clone(), v3.clone())?;
                        if if f52(f362(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f100({
                                let m1109 = f93(v5.clone(), f8(Str::lit("schema")));
                                match m1109 {
                                    None => {
                                        Adt2::C9
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f363(v5)?))
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
                Ok(next) => { m1110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h1112, _, _) => {
            h1112
        }
    }, f268())) {
        Ok(Adt12::C1(List::cons(f268(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f364(f147(match v0.clone() {
            Adt1::C0(_, h1113, _, _) => {
                h1113
            }
        }, f268())))?, v1.clone());
        let m1114 = f56(v0.clone(), List::cons(f153(), List::cons(f212(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1114.uncons() {
            None => {
                let m1115 = f56(v0.clone(), f365(v0.clone(), v2.clone())?);
                match m1115.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f366(v0.clone(), v2, v1)?;
                        let (h1116, _) = v7;
                        if h1116 {
                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                let (_, h1117) = v7;
                                h1117
                            }))
                        } else {
                            let m1118 = f124(v0.clone());
                            match m1118.uncons() {
                                None => {
                                    let m1119 = f126(v0.clone())?;
                                    match m1119.uncons() {
                                        None => {
                                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                                let (_, h1120) = v7;
                                                h1120
                                            }, List::cons(f7(f145(f367(f127(v0.clone())?, f135(v0.clone(), f136(v0.clone())?)?))?), List::cons(f368(v0)?, List::<Bytes>::nil()))))))
                                        }
                                        Some((v10, v11)) => {
                                            Ok(Adt12::C0(List::cons(v10, v11)))
                                        }
                                    }
                                }
                                Some((v8, v9)) => {
                                    Ok(Adt12::C1(List::cons(v8, v9)))
                                }
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        Ok(Adt12::C0(List::cons(v5, v6)))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt12::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f247(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f369(f96(v0, f153())?)?)))
}

pub fn f248() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# The one way an agent changes this repository: dispatch this workflow with an\u{a}")), List::cons(f8(Str::lit("# operation of the API. It runs the certified command, and opens a pull request\u{a}")), List::cons(f8(Str::lit("# as the Verdant Studio GitHub App, which merges it once the studio check passes.\u{a}")), List::cons(f8(Str::lit("name: operate\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}")), List::cons(f8(Str::lit("# that defers to an open advance pull request always has a successor when that merges.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("    inputs:\u{a}")), List::cons(f8(Str::lit("      args:\u{a}")), List::cons(f8(Str::lit("        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}")), List::cons(f8(Str::lit("        required: true\u{a}")), List::cons(f8(Str::lit("        default: 'advance'\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input:\u{a}")), List::cons(f8(Str::lit("        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input_name:\u{a}")), List::cons(f8(Str::lit("        description: 'File name for input'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("        default: 'input'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Only operations that write the registers share one group. GitHub keeps one\u{a}")), List::cons(f8(Str::lit("# pending run per group, so a staging or propose-command run gets its own.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  operate:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 30\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      # Main moves about every 10 minutes, and refresh's regeneration of an API change plus its check takes\u{a}")), List::cons(f8(Str::lit("      # about 12, so an operation merged in that window pushes the change behind main again (#309, #315).\u{a}")), List::cons(f8(Str::lit("      # refresh keeps a regenerated change ready with its auto-merge set, so this run waits for every such\u{a}")), List::cons(f8(Str::lit("      # change whose studio check has not failed to merge first, then starts from the new main. It waits at\u{a}")), List::cons(f8(Str::lit("      # most 10 minutes, so neither queue stalls. A new propose-command waits too, so it never merges ahead of an\u{a}")), List::cons(f8(Str::lit("      # older change that is being regenerated; revert-merge, the repair of a broken path, never waits.\u{a}")), List::cons(f8(Str::lit("      - name: wait for the API changes that are next to merge (every operation but revert-merge)\u{a}")), List::cons(f8(Str::lit("        if: ${{ !startsWith(inputs.args, 'revert-merge ') }}\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          for try in $(seq 1 20); do\u{a}")), List::cons(f8(Str::lit("            next=''\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio propose-command\")) and .auto_merge != null and (.draft | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then next=\"$next #${pr}\"; fi\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f249() -> Bytes {
    f7(List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -z \"$next\" ]; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("            echo \"waiting for${next} to merge (check $try of 20)\"\u{a}")), List::cons(f8(Str::lit("            sleep 30\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          echo \"::warning::${next# } still open after 10 minutes, so this run goes on\"\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-0.4.3\u{a}")), List::cons(f8(Str::lit("      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}")), List::cons(f8(Str::lit("        if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          mkdir -p target/input\u{a}")), List::cons(f8(Str::lit("          cat > target/input/change.patch <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}")), List::cons(f8(Str::lit("          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}")), List::cons(f8(Str::lit("          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}")), List::cons(f8(Str::lit("          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          # The one export, shared with refresh and run end to end by the studio check.\u{a}")), List::cons(f8(Str::lit("          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f370(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("            tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The owner reads this pull request, so its body leads with the proposer's own\u{a}")), List::cons(f8(Str::lit("          # account (the prose before the first diff), then the definitions it changes.\u{a}")), List::cons(f8(Str::lit("          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}")), List::cons(f8(Str::lit("          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}")), List::cons(f8(Str::lit("            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}")), List::cons(f8(Str::lit("          {\u{a}")), List::cons(f8(Str::lit("            cat target/summary.md\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"### Definitions this adds or changes\"\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f250() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            {\u{a}")), List::cons(f8(Str::lit("              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}")), List::cons(f8(Str::lit("              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}")), List::cons(f8(Str::lit("            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            slash=/\u{a}")), List::cons(f8(Str::lit("            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            head -n 8 target/export.log.phases || true\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"<${slash}details>\"\u{a}")), List::cons(f8(Str::lit("          } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("      - id: command\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: |\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_readers\u{a}")), List::cons(f8(Str::lit("          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-0.4.3\u{a}")), List::cons(f8(Str::lit("      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}")), List::cons(f8(Str::lit("        if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}")), List::cons(f8(Str::lit("          target/lexlean/bin/lexlean production verify-artifact certified\u{a}")), List::cons(f8(Str::lit("          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}")), List::cons(f8(Str::lit("          if [ -d certified-readers ]; then\u{a}")), List::cons(f8(Str::lit("            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}")), List::cons(f8(Str::lit("            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("          INPUT_NAME: ${{ inputs.input_name }}\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail -f\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}")), List::cons(f8(Str::lit("          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f251() -> Bytes {
    f7(List::cons(f8(Str::lit("          # An App pull request whose files already hold on main has nothing left to merge,\u{a}")), List::cons(f8(Str::lit("          # so it is closed; the propose-command ones change the API and stay open until they merge.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}")), List::cons(f8(Str::lit("              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}")), List::cons(f8(Str::lit("              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}")), List::cons(f8(Str::lit("              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}")), List::cons(f8(Str::lit("                exit 0\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            cat target/operation.txt\u{a}")), List::cons(f8(Str::lit("          elif [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            : > target/operation.txt\u{a}")), List::cons(f8(Str::lit("            last=''\u{a}")), List::cons(f8(Str::lit("            skipped=''\u{a}")), List::cons(f8(Str::lit("            # next prints one operation it can dispatch, or else the lines that say why the\u{a}")), List::cons(f8(Str::lit("            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}")), List::cons(f8(Str::lit("            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}")), List::cons(f8(Str::lit("              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("              if vs $op > target/attempt.txt 2>&1; then\u{a}")), List::cons(f8(Str::lit("                tee -a target/operation.txt < target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                last=$op\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                cat target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                binding=${op#* }\u{a}")), List::cons(f8(Str::lit("                echo \"::warning::$op was refused; the other bindings continue\"\u{a}")), List::cons(f8(Str::lit("                echo \"refused: $op\" >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}")), List::cons(f8(Str::lit("                skipped=\"$skipped $binding\"\u{a}")), List::cons(f8(Str::lit("                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            t0=$EPOCHREALTIME\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}")), List::cons(f8(Str::lit("            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}")), List::cons(f8(Str::lit("            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}")), List::cons(f8(Str::lit("              echo \"waiting merge: #${pr} changes the API, and merges once the studio check passes\" >> target/waiting.txt\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}")), List::cons(f8(Str::lit("          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f252() -> Bytes {
    f7(List::cons(f8(Str::lit("            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}")), List::cons(f8(Str::lit("            # good main commit, it reverts the merge on main as the App, and merges once the studio check passes.\u{a}")), List::cons(f8(Str::lit("            merge=${ARGS#revert-merge }\u{a}")), List::cons(f8(Str::lit("            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B revert origin/main\u{a}")), List::cons(f8(Str::lit("            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("            git revert -m 1 --no-commit \"$merge\"\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::cons(f8(Str::lit("            vs $ARGS | tee target/operation.txt\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("      - name: open a pull request as the Verdant Studio app\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          git add -A\u{a}")), List::cons(f8(Str::lit("          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}")), List::cons(f8(Str::lit("          verdant_studio $ARGS\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("          $(cat target/operation.txt)\u{a}")), List::cons(f8(Str::lit("          MSG\u{a}")), List::cons(f8(Str::lit("          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}")), List::cons(f8(Str::lit("          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}")), List::cons(f8(Str::lit("            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          title=\"verdant_studio $ARGS\"\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}")), List::cons(f8(Str::lit("          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}")), List::cons(f8(Str::lit("          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}")), List::cons(f8(Str::lit("          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}")), List::cons(f8(Str::lit("          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}")), List::cons(f8(Str::lit("          pr=\"\"\u{a}")), List::cons(f8(Str::lit("          for try in 1 2 3 4; do\u{a}")), List::cons(f8(Str::lit("            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            pr=\"\"\u{a}")), List::cons(f8(Str::lit("            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}")), List::cons(f8(Str::lit("            sleep $((2 ** try))\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          echo \"opened #${pr}\"\u{a}")), List::cons(f8(Str::lit("          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("          # A change to the API itself (propose-command, revert-merge) merges as every App pull request does:\u{a}")), List::cons(f8(Str::lit("          # the owner automated these approvals (2026-11-26), so the studio check, which runs the API's own\u{a}")), List::cons(f8(Str::lit("          # end-to-end propose run, is its gate. refresh keeps its auto-merge while it regenerates it.\u{a}")), List::cons(f8(Str::lit("          if [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}")), List::cons(f8(Str::lit("            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f253() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::<Bytes>::nil())))
}

pub fn f254() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# When main moves, every open propose-command pull request that no longer merges\u{a}")), List::cons(f8(Str::lit("# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}")), List::cons(f8(Str::lit("# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}")), List::cons(f8(Str::lit("# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}")), List::cons(f8(Str::lit("# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}")), List::cons(f8(Str::lit("# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}")), List::cons(f8(Str::lit("# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("# The exception is one whose auto-merge is set, the next change to merge: it stays ready and keeps its\u{a}")), List::cons(f8(Str::lit("# auto-merge, so it merges as soon as its regeneration passes the studio check. It is\u{a}")), List::cons(f8(Str::lit("# regenerated first, and only the oldest such one at a time, since two changes to the\u{a}")), List::cons(f8(Str::lit("# API conflict on their export (#309 and #315 were pushed behind main this way).\u{a}")), List::cons(f8(Str::lit("name: refresh\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}")), List::cons(f8(Str::lit("# operate run completes, which needs no token that may dispatch workflows, so every\u{a}")), List::cons(f8(Str::lit("# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_run:\u{a}")), List::cons(f8(Str::lit("    workflows: [operate]\u{a}")), List::cons(f8(Str::lit("    types: [completed]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A running refresh always finishes, because an export takes minutes and main moves more\u{a}")), List::cons(f8(Str::lit("# often than that; GitHub keeps only the newest pending run behind it.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: refresh\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  find:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 15\u{a}")), List::cons(f8(Str::lit("    outputs:\u{a}")), List::cons(f8(Str::lit("      stale: ${{ steps.stale.outputs.stale }}\u{a}")), List::cons(f8(Str::lit("      matrix: ${{ steps.stale.outputs.matrix }}\u{a}")), List::cons(f8(Str::lit("      auto: ${{ steps.stale.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - id: stale\u{a}")), List::cons(f8(Str::lit("        name: find the propose-command pull requests that conflict with main\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          stale=''\u{a}")), List::cons(f8(Str::lit("          auto=''\u{a}")), List::cons(f8(Str::lit("          # Oldest first, so the oldest one whose auto-merge is set is the one regenerated now.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}")), List::cons(f8(Str::lit("            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}")), List::cons(f8(Str::lit("            # which a cancelled run may have left behind.\u{a}")), List::cons(f8(Str::lit("            mergeable=null\u{a}")), List::cons(f8(Str::lit("            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}")), List::cons(f8(Str::lit("              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f255() -> Bytes {
    f7(List::cons(f8(Str::lit("              if [ \"$mergeable\" != null ]; then break; fi\u{a}")), List::cons(f8(Str::lit("              sleep 10\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = false ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              # Drafting would drop its auto-merge, so it stays ready. A later one waits: it is\u{a}")), List::cons(f8(Str::lit("              # regenerated once this one merges, when it would conflict again anyway.\u{a}")), List::cons(f8(Str::lit("              if [ -z \"$auto\" ]; then auto=$pr; else echo \"#${pr} waits for #${auto} to merge\"; fi\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = false ]; then\u{a}")), List::cons(f8(Str::lit("              stale=\"$stale $pr\"\u{a}")), List::cons(f8(Str::lit("              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}")), List::cons(f8(Str::lit("              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$auto\" ]; then stale=\" $auto$stale\"; fi\u{a}")), List::cons(f8(Str::lit("          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"auto=$auto\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("  regenerate:\u{a}")), List::cons(f8(Str::lit("    needs: find\u{a}")), List::cons(f8(Str::lit("    if: needs.find.outputs.stale != ''\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 60\u{a}")), List::cons(f8(Str::lit("    strategy:\u{a}")), List::cons(f8(Str::lit("      fail-fast: false\u{a}")), List::cons(f8(Str::lit("      matrix:\u{a}")), List::cons(f8(Str::lit("        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          fetch-depth: 0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-0.4.3\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          STALE: ${{ matrix.pr }}\u{a}")), List::cons(f8(Str::lit("          AUTO: ${{ needs.find.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A regeneration of the one whose auto-merge is set that fails leaves it a draft, which drops\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f256() -> Bytes {
    f7(List::cons(f8(Str::lit("          # its auto-merge, so it is no longer the next change to merge and nothing is handed over red.\u{a}")), List::cons(f8(Str::lit("          trap 'rc=$?; if [ \"$rc\" != 0 ] && [ \"$STALE\" = \"$AUTO\" ]; then gh pr ready --undo \"$STALE\" || true; fi' EXIT\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            mkdir -p target\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          failed=''\u{a}")), List::cons(f8(Str::lit("          for pr in $STALE; do\u{a}")), List::cons(f8(Str::lit("            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}")), List::cons(f8(Str::lit("            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}")), List::cons(f8(Str::lit("            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("            # The one whose auto-merge is set stays ready, so that it keeps its auto-merge.\u{a}")), List::cons(f8(Str::lit("            if [ \"$pr\" != \"$AUTO\" ]; then gh pr ready --undo \"$pr\" || true; fi\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin \"$branch\" main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B refresh origin/main\u{a}")), List::cons(f8(Str::lit("            git clean -fdq\u{a}")), List::cons(f8(Str::lit("            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}")), List::cons(f8(Str::lit("            # change, and two reader pull requests never conflict on it.\u{a}")), List::cons(f8(Str::lit("            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}")), List::cons(f8(Str::lit("            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}")), List::cons(f8(Str::lit("            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}")), List::cons(f8(Str::lit("            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}")), List::cons(f8(Str::lit("            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}")), List::cons(f8(Str::lit("              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}")), List::cons(f8(Str::lit("                base=$(git merge-base origin/main \"origin/$branch\")\u{a}")), List::cons(f8(Str::lit("                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}")), List::cons(f8(Str::lit("                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}")), List::cons(f8(Str::lit("                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}")), List::cons(f8(Str::lit("                git add model/strata.json\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}")), List::cons(f8(Str::lit("                git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("                failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}")), List::cons(f8(Str::lit("                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("                continue\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f370(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("              tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("              git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("              failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("              continue\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("            git add -A\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f257() -> Bytes {
    f7(List::cons(f8(Str::lit("            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}")), List::cons(f8(Str::lit("            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}")), List::cons(f8(Str::lit("            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}")), List::cons(f8(Str::lit("            gh pr ready \"$pr\" || true\u{a}")), List::cons(f8(Str::lit("            echo \"::notice::regenerated #${pr} on the new main\"\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))))))
}

pub fn f258(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1121 = v0;
    match m1121.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f259(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt19 = f371(v0.clone(), 0u64, {
        let a1122 = v0;
        length_bytes(a1122)
    }, Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt19 = {
        let a1125 = {
            let a1124 = match v1.clone() {
                Adt19::C0(_, _, h1123, _, _) => {
                    h1123
                }
            };
            length_bytes(a1124)
        };
        let a1126 = 0u64;
        if if nat_eq(a1125, a1126) {
            let m1128 = match v1.clone() {
                Adt19::C0(_, _, _, h1127, _) => {
                    h1127
                }
            };
            match m1128.uncons() {
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
            f372(v1)
        }
    };
    Ok(f373(match v4 {
        Adt19::C0(_, _, _, _, h1129) => {
            h1129
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f260(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt17>, v3: List<Adt2>, v4: List<Bytes>) -> List<Bytes> {
    let m1130 = v1;
    match m1130.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f125(f374(v5.clone(), v2.clone()), f375(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f376(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1132 = {
                    let a1131 = v7.clone();
                    length_list(a1131)
                };
                let a1133 = 0u64;
                if nat_eq(a1132, a1133) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f122(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f260(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f261(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1134 = v0;
    match m1134.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1135 = v1;
            match m1135.uncons() {
                None => {
                    f261(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f261(v2))
                }
            }
        }
    }
}

pub fn f262(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1136 = v1;
    match m1136.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f220(v2.clone(), f8(Str::lit("domain"))) {
                f377(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1137 = f93(v2.clone(), f8(Str::lit("role")));
                    match m1137 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f378(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1138 = f93(v2.clone(), f8(Str::lit("origin")));
                    match m1138 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1139 = f93(v2, f8(Str::lit("evidence")));
                    match m1139 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f262(v0, v3))
            } else {
                f262(v0, v3)
            }
        }
    }
}

pub fn f263(v0: List<Adt2>) -> List<Bytes> {
    let m1140 = v0;
    match m1140.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f263(v2))
        }
    }
}

pub fn f264(v0: List<Adt2>) -> List<Bytes> {
    let m1141 = v0;
    match m1141.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f379(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f264(v2))
        }
    }
}

pub fn f265(p1147: List<Bytes>) -> R<bool> {
    let m1146 = p1147;
    {
        let mut m1146 = m1146;
        loop {
            let step = {
                let v0 = m1146;
                let m1142 = v0;
                match m1142.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1144 = {
                            let a1143 = f197(v1)?;
                            length_bytes(a1143)
                        };
                        let a1145 = 0u64;
                        if nat_eq(a1144, a1145) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f266(v0: List<Bytes>) -> R<bool> {
    if f275(f298(v0.clone())?) {
        Ok(f51({
            let a1149 = {
                let a1148 = f277(f298(v0)?);
                length_list(a1148)
            };
            let a1150 = 0u64;
            nat_eq(a1149, a1150)
        }))
    } else {
        Ok(false)
    }
}

pub fn f267(v0: List<Bytes>) -> R<bool> {
    if f275(f279(v0.clone(), f8(Str::lit("requirement")))?) {
        if f275(f279(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1152 = {
                    let a1151 = f223(f277(f279(v0.clone(), f8(Str::lit("requirement")))?), f277(f279(v0, f8(Str::lit("admission")))?));
                    length_list(a1151)
                };
                let a1153 = 0u64;
                nat_eq(a1152, a1153)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f268() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f269(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f270() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f271(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1154 = v0;
    match m1154.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f197(v1)?;
            let a1156 = {
                let a1155 = v3.clone();
                length_bytes(a1155)
            };
            let a1157 = 0u64;
            if nat_eq(a1156, a1157) {
                f271(v2)
            } else {
                let m1158 = f43(v3.clone())?;
                match m1158 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f380(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1159 = f271(v2)?;
                            match m1159 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f381(v5), {
                                        let a1160 = v3;
                                        let a1161 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1160, a1161)
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

pub fn f272() -> Bytes {
    let a1162 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1162)
}

pub fn f273(p1167: List<Bytes>) -> R<Option<Bytes>> {
    let m1166 = p1167;
    {
        let mut m1166 = m1166;
        loop {
            let step = {
                let v0 = m1166;
                let m1163 = v0;
                match m1163.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1164 = f43(f197(v1)?)?;
                        match m1164 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1165 = f93(v3.clone(), f8(Str::lit("source")));
                                match m1165 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f382(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m1166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f274(v0: Bytes) -> Bytes {
    let a1172 = {
        let a1168 = Str::lit("binding ");
        utf8_encode(a1168)
    };
    let a1173 = {
        let a1170 = v0;
        let a1171 = {
            let a1169 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1169)
        };
        append_bytes(a1170, a1171)
    };
    append_bytes(a1172, a1173)
}

pub fn f275(v0: Option<List<Adt2>>) -> bool {
    let m1174 = v0;
    match m1174 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f276(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1175 = v0;
    match m1175.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f197(v1)?;
            let a1177 = {
                let a1176 = v3.clone();
                length_bytes(a1176)
            };
            let a1178 = 0u64;
            if nat_eq(a1177, a1178) {
                f276(v2)
            } else {
                let m1179 = f43(v3)?;
                match m1179 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if f117(f117(f220(v4.clone(), f8(Str::lit("authority"))), f220(v4.clone(), f8(Str::lit("row")))), f117(f117(f220(v4.clone(), f8(Str::lit("construct"))), f220(v4.clone(), f8(Str::lit("artifact")))), f85(f58(v4.clone(), f8(Str::lit("reader"))), f8(Str::lit("read-"))))) {
                            let m1180 = f276(v2)?;
                            match m1180 {
                                None => {
                                    Ok(None::<List<Adt2>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons(v4, v5)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt2>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f277(v0: Option<List<Adt2>>) -> List<Adt2> {
    let m1181 = v0;
    match m1181 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f278(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1182 = v0;
    match m1182.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f197(v1)?;
            let a1184 = {
                let a1183 = v3.clone();
                length_bytes(a1183)
            };
            let a1185 = 0u64;
            if nat_eq(a1184, a1185) {
                f278(v2)
            } else {
                let m1186 = f43(v3.clone())?;
                match m1186 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f383(v4.clone()) {
                            let m1187 = f278(v2)?;
                            match m1187 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f243(), List::cons(f8(Str::lit("/")), List::cons(f384(f58(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1188 = v3;
                                        let a1189 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1188, a1189)
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

pub fn f279(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt2>>> {
    let m1190 = v0;
    match m1190.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f197(v2)?;
            let a1192 = {
                let a1191 = v4.clone();
                length_bytes(a1191)
            };
            let a1193 = 0u64;
            if nat_eq(a1192, a1193) {
                f279(v3, v1)
            } else {
                let m1194 = f43(v4)?;
                match m1194 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f385(v5.clone());
                        let a1196 = {
                            let a1195 = v6.clone();
                            length_bytes(a1195)
                        };
                        let a1197 = 0u64;
                        if nat_eq(a1196, a1197) {
                            Ok(None::<List<Adt2>>)
                        } else {
                            let m1198 = f279(v3, v1.clone())?;
                            match m1198 {
                                None => {
                                    Ok(None::<List<Adt2>>)
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

pub fn f280(p1201: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1200 = p1201;
    {
        let mut m1200 = m1200;
        loop {
            let step = {
                let v0 = m1200;
                let m1199 = v0;
                match m1199.uncons() {
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
                Ok(next) => { m1200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f282(v0: Bytes, v1: List<Adt2>) -> R<(Bytes, Bytes)> {
    Ok((v0, f100(Adt2::C7(std::rc::Rc::new(f101(v1))))?))
}

pub fn f283(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f100(Adt2::C7(std::rc::Rc::new(f386(f145(f125(v1, f214(f96(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f284(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1202 = f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m1202.uncons() {
        None => {
            let m1204 = f280(f131(List::cons(Adt2::C8(std::rc::Rc::new({
                let m1203 = f387(v4.clone());
                match m1203 {
                    None => {
                        Adt2::C9
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt2>::nil())), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1204 {
                None => {
                    f388(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f281(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f285(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f286(v0: Bytes) -> R<Bytes> {
    Ok(f389(f390(v0)?))
}

pub fn f287(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: List<Bytes>, v5: Bytes) -> R<Adt12> {
    let v6: Bytes = {
        let a1205 = f8(Str::lit("registry-read "));
        let a1206 = v5;
        append_bytes(a1205, a1206)
    };
    if f51(f52(v6.clone(), match v0.clone() {
        Adt1::C0(_, _, h1207, _) => {
            h1207
        }
    })) {
        Ok(Adt12::C2(f391(), v4, v6))
    } else {
        let m1208 = f56(v0.clone(), List::cons(f316(), List::<Bytes>::nil()));
        match m1208.uncons() {
            None => {
                f392(v0, v1, f316(), v2, v3, v4)
            }
            Some((v7, v8)) => {
                Ok(Adt12::C0(List::cons(v7, v8)))
            }
        }
    }
}

pub fn f288() -> Bytes {
    f393()
}

pub fn f289(v0: Adt1, v1: Adt2, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f293(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f394(v1, v4)?)?), List::cons(f283(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f290(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1209 = v0;
    match m1209.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f291() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f292(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f358(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f293(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f395(f150(f95(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f396(f397(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f294(p1212: List<Adt2>) -> bool {
    let m1211 = p1212;
    {
        let mut m1211 = m1211;
        loop {
            let step = {
                let v0 = m1211;
                let m1210 = v0;
                match m1210.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f142(f398(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
                        } else {
                            false
                        } {
                            Err::<List<Adt2>, bool>(true)
                        } else {
                            Ok::<List<Adt2>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1211 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: List<Bytes>) -> List<Bytes> {
    let m1213 = v0;
    match m1213.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f399(v1.clone()) {
                List::<Bytes>::nil()
            } else {
                List::cons(v1, f295(v2))
            }
        }
    }
}

pub fn f296(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>, v4: List<Bytes>) -> R<Adt12> {
    let m1214 = f387(v4);
    match m1214 {
        None => {
            Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m1215 = f280(f131(List::cons(Adt2::C8(std::rc::Rc::new(v5.clone())), List::<Adt2>::nil())), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1215 {
                None => {
                    f400(v0, v2.clone(), Adt2::C11(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f386(v3)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C6(f286(v1.clone())?)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(v5)), f401(v2, f402(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f281(v6))
                }
            }
        }
    }
}

pub fn f297(p1218: List<Bytes>) -> List<Bytes> {
    let m1217 = p1218;
    {
        let mut m1217 = m1217;
        loop {
            let step = {
                let v0 = m1217;
                let m1216 = v0;
                match m1216.uncons() {
                    None => {
                        Err::<List<Bytes>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v1, v2)) => {
                        if f399(v1.clone()) {
                            Err::<List<Bytes>, List<Bytes>>(List::cons(v1, v2))
                        } else {
                            Ok::<List<Bytes>, List<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1219 = v0;
    match m1219.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f197(v1)?;
            let a1221 = {
                let a1220 = v3.clone();
                length_bytes(a1220)
            };
            let a1222 = 0u64;
            if nat_eq(a1221, a1222) {
                f298(v2)
            } else {
                let m1223 = f43(v3)?;
                match m1223 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if if f220(v4.clone(), f8(Str::lit("agent"))) {
                            if f220(v4.clone(), f8(Str::lit("at"))) {
                                if f220(v4.clone(), f8(Str::lit("quote"))) {
                                    f403(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1224 = f298(v2)?;
                            match m1224 {
                                None => {
                                    Ok(None::<List<Adt2>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons(v4, v5)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt2>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f299() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f300(p1227: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m1226 = p1227;
    {
        let mut m1226 = m1226;
        loop {
            let step = {
                let v0 = m1226;
                let m1225 = v0;
                match m1225.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f117(f32(f58(v3.clone(), f8(Str::lit("observedProperty"))), v1.clone()), f32(f58(v3, f8(Str::lit("hasSimpleResult"))), v2.clone())) {
                            Err::<List<Adt2>, bool>(true)
                        } else {
                            Ok::<List<Adt2>, bool>(v4)
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

pub fn f301(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt2 {
    Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt2::C6(v2)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt2::C6(v0)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9))))))))))))
}

pub fn f302(v0: u64) -> bool {
    let a1228 = v0;
    let a1229 = 127u64;
    nat_le(a1228, a1229)
}

pub fn f303(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1230, _) = v1.clone();
        h1230
    };
    let v3: u64 = {
        let (h1231, _) = {
            let (_, h1232) = v1.clone();
            h1232
        };
        h1231
    };
    let v4: List<Bytes> = {
        let (_, h1233) = {
            let (_, h1234) = v1;
            h1234
        };
        h1233
    };
    let m1235 = f207(v0.clone(), v2);
    match m1235 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1236 = v5;
            let a1237 = 10u8;
            if equal(a1236, a1237) {
                Ok(Some(({
                    let a1238 = v2;
                    let a1239 = 1u64;
                    nat_add(a1238, a1239)?
                }, ({
                    let a1240 = v2;
                    let a1241 = 1u64;
                    nat_add(a1240, a1241)?
                }, List::cons(f404(f46(v0, v3, {
                    let a1242 = v2;
                    let a1243 = v3;
                    nat_sub(a1242, a1243)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1244 = v2;
                    let a1245 = 1u64;
                    nat_add(a1244, a1245)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f304(v0: Adt2) -> List<Adt2> {
    let m1246 = v0;
    match m1246 {
        Adt2::C0 => {
            List::<Adt2>::nil()
        }
        Adt2::C1(_, _) => {
            List::<Adt2>::nil()
        }
        Adt2::C2(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C3(_, _) => {
            List::<Adt2>::nil()
        }
        Adt2::C4(_, _, _) => {
            List::<Adt2>::nil()
        }
        Adt2::C5(_, _) => {
            List::<Adt2>::nil()
        }
        Adt2::C6(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C7(r1247) => {
            let v3 = (*r1247).clone();
            f304(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r1248, r1249) => {
            let v1 = (*r1248).clone();
            let v2 = (*r1249).clone();
            List::cons(v1, f304(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f305(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1250 = v0;
    match m1250 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r1251) => {
            let v12 = (*r1251).clone();
            Adt2::C1(v11, std::rc::Rc::new(v12))
        }
        Adt2::C2(v6) => {
            Adt2::C2(v6)
        }
        Adt2::C3(v7, v8) => {
            Adt2::C3(v7, v8)
        }
        Adt2::C4(v13, v14, v15) => {
            Adt2::C4(v13, v14, v15)
        }
        Adt2::C5(v16, v17) => {
            Adt2::C5(v16, v17)
        }
        Adt2::C6(v9) => {
            Adt2::C6(v9)
        }
        Adt2::C7(r1252) => {
            let v10 = (*r1252).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r1253) => {
            let v5 = (*r1253).clone();
            Adt2::C8(std::rc::Rc::new(f305(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1254, r1255) => {
            let v18 = (*r1254).clone();
            let v19 = (*r1255).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r1256, r1257) => {
            let v3 = (*r1256).clone();
            let v4 = (*r1257).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt2::C11(v2, std::rc::Rc::new(f405(v3, v1.clone())), std::rc::Rc::new(f305(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f305(v4, v1)))
            }
        }
    }
}

pub fn f306(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1258 = 0u64;
        let a1259 = v1;
        nat_lt(a1258, a1259)
    } else {
        false
    } {
        let a1260 = Bytes::lit(&[0x2d]);
        let a1261 = f406(v1)?;
        Ok(append_bytes(a1260, a1261))
    } else {
        f406(v1)
    }
}

pub fn f307(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f406(v1)?;
    let v4: u64 = {
        let a1262 = v3.clone();
        length_bytes(a1262)
    };
    let v5: Bytes = {
        let a1263 = v2;
        let a1264 = v4;
        if nat_lt(a1263, a1264) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1265 = v4;
                let a1266 = v2;
                nat_sub(a1265, a1266)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1267 = v4;
                let a1268 = v2;
                nat_sub(a1267, a1268)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f328({
                let a1269 = v2;
                let a1270 = v4;
                nat_sub(a1269, a1270)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1271 = f8(Str::lit("-"));
        let a1272 = v5;
        Ok(append_bytes(a1271, a1272))
    } else {
        Ok(v5)
    }
}

pub fn f308(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f407(v0)?;
    let v3: bool = {
        let a1273 = f408(v0)?;
        let a1274 = v1;
        nat_le(a1273, a1274)
    };
    let v4: u64 = {
        let a1278 = {
            let a1275 = v1;
            let a1276 = v2;
            let a1277 = 0u64;
            nat_quot(a1275, a1276, a1277)
        };
        let a1279 = f409(f410(v0))?;
        let a1280 = 0u64;
        nat_rem(a1278, a1279, a1280)
    };
    let v5: u64 = {
        let a1281 = v1;
        let a1282 = v2;
        let a1283 = 0u64;
        nat_rem(a1281, a1282, a1283)
    };
    let a1284 = v4;
    let a1285 = f411(v0)?;
    if nat_eq(a1284, a1285) {
        let a1286 = v5;
        let a1287 = 0u64;
        if nat_eq(a1286, a1287) {
            Ok(f412(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f412(v3, {
                let a1291 = {
                    let a1288 = v2;
                    let a1289 = 2u64;
                    let a1290 = 0u64;
                    nat_quot(a1288, a1289, a1290)
                };
                let a1292 = v5;
                if nat_le(a1291, a1292) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f117({
            let a1293 = v4;
            let a1294 = 0u64;
            nat_eq(a1293, a1294)
        }, {
            let a1295 = v5;
            let a1296 = 0u64;
            nat_eq(a1295, a1296)
        }) {
            Ok(f412(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1297 = v4;
                let a1298 = 0u64;
                if nat_eq(a1297, a1298) {
                    v5
                } else {
                    let a1299 = v2;
                    let a1300 = v5;
                    nat_add(a1299, a1300)?
                }
            };
            let v7: u64 = {
                let a1301 = v4;
                let a1302 = 0u64;
                if nat_eq(a1301, a1302) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1309 = {
                    let a1305 = {
                        let a1303 = v7;
                        let a1304 = 2000u64;
                        nat_add(a1303, a1304)?
                    };
                    let a1306 = f413(v0);
                    nat_sub(a1305, a1306)
                };
                let a1310 = {
                    let a1307 = f414(v0);
                    let a1308 = 1u64;
                    nat_sub(a1307, a1308)
                };
                nat_sub(a1309, a1310)
            };
            Ok(f412(v3, {
                let a1311 = 2000u64;
                let a1312 = v8;
                if nat_le(a1311, a1312) {
                    f415(v6, true, {
                        let a1313 = v8;
                        let a1314 = 2000u64;
                        nat_sub(a1313, a1314)
                    })?
                } else {
                    f415(v6, false, {
                        let a1315 = 2000u64;
                        let a1316 = v8;
                        nat_sub(a1315, a1316)
                    })?
                }
            }))
        }
    }
}

pub fn f309(v0: Bytes) -> R<Bytes> {
    let a1321 = f199(v0.clone(), 0u64, {
        let a1318 = {
            let a1317 = v0.clone();
            length_bytes(a1317)
        };
        let a1319 = 1u64;
        nat_add(a1318, a1319)?
    })?;
    let a1322 = {
        let a1320 = v0.clone();
        length_bytes(a1320)
    };
    if nat_eq(a1321, a1322) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1328 = {
            let a1326 = Bytes::lit(&[0x22]);
            let a1327 = f416(v0.clone(), 0u64, {
                let a1324 = {
                    let a1323 = v0;
                    length_bytes(a1323)
                };
                let a1325 = 1u64;
                nat_add(a1324, a1325)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1326, a1327)
        };
        let a1329 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1328, a1329))
    }
}

pub fn f310(v0: List<Bytes>) -> Bytes {
    let a1332 = {
        let a1330 = Bytes::lit(&[0x5b]);
        let a1331 = f122(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1330, a1331)
    };
    let a1333 = Bytes::lit(&[0x5d]);
    append_bytes(a1332, a1333)
}

pub fn f311(v0: Adt2) -> R<List<Bytes>> {
    let m1334 = v0;
    match m1334 {
        Adt2::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C3(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C4(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C5(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C8(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C9 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C10(r1335, r1336) => {
            let v14 = (*r1335).clone();
            let v15 = (*r1336).clone();
            Ok(List::cons(f189(v14)?, f311(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f312(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f417(Fn5::F418, List::<(Bytes, Bytes)>::nil(), f419(v0)?);
    let v2: List<Bytes> = f420(Fn6::F421, List::<Bytes>::nil(), v1)?;
    let a1339 = {
        let a1337 = Bytes::lit(&[0x7b]);
        let a1338 = f122(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1337, a1338)
    };
    let a1340 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1339, a1340))
}

pub fn f313(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1341 = v0;
    match m1341 {
        Adt2::C0 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C3(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C4(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C5(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C8(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C9 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C10(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C11(v16, r1342, r1343) => {
            let v17 = (*r1342).clone();
            let v18 = (*r1343).clone();
            Ok(List::cons((v16, f189(v17)?), f313(v18)?))
        }
    }
}

pub fn f314() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f315(v0: Adt13) -> Bytes {
    let m1344 = v0;
    match m1344 {
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

pub fn f316() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f317(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f422(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f422(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f422(f8(Str::lit("name")), f423(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f422(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f422(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f422(f8(Str::lit("statement")), f424(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f422(f8(Str::lit("media_type")), f425(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f422(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f422(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f422(f8(Str::lit("url")), f192(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f318(v0: Bytes) -> R<u64> {
    let (h1345, _) = f321(Fn3::F426(v0.clone()), {
        let a1346 = v0.clone();
        length_bytes(a1346)
    }, {
        let a1347 = v0;
        length_bytes(a1347)
    })?;
    Ok(h1345)
}

pub fn f319(v0: Bytes, v1: u64) -> R<u64> {
    let (h1348, _) = f321(Fn3::F427(v1, v0), v1, 0u64)?;
    Ok(h1348)
}

pub fn f320(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f115(v0.clone(), {
        let a1349 = v1;
        let a1350 = 1u64;
        nat_add(a1349, a1350)?
    })?;
    let v4: Bytes = {
        let a1351 = v3;
        let a1352 = 117u64;
        if nat_eq(a1351, a1352) {
            f428(v0, v1)?
        } else {
            f429(v3)
        }
    };
    let a1353 = v4.clone();
    let a1354 = Bytes::lit(&[]);
    if equal(a1353, a1354) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1355 = v3;
            let a1356 = 117u64;
            if nat_eq(a1355, a1356) {
                let a1357 = v1;
                let a1358 = 6u64;
                nat_add(a1357, a1358)?
            } else {
                let a1359 = v1;
                let a1360 = 2u64;
                nat_add(a1359, a1360)?
            }
        }, ({
            let a1361 = v2;
            let a1362 = v4;
            append_bytes(a1361, a1362)
        }, 0u64))))
    }
}

pub fn f321(v0: Fn3, p1367: u64, p1368: u64) -> R<(u64, bool)> {
    let m1366 = (p1367, p1368);
    {
        let mut m1366 = m1366;
        loop {
            let step = {
                let (v1, v2) = m1366;
                let m1363 = v1;
                if m1363 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1363 - 1;
                    let m1365 = {
                        let c1364: Fn3 = v0.clone();
                        c1364.apply(v2)?
                    };
                    match m1365 {
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
                Ok(next) => { m1366 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f322(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f430(v0, v1)
}

pub fn f323(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f431(v0, v1)
}

pub fn f324(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1369 = v0.clone();
        length_bytes(a1369)
    };
    let a1370 = 0u64;
    let a1371 = v1;
    if nat_lt(a1370, a1371) {
        let a1372 = f432(v0.clone())?;
        let a1373 = v1;
        if nat_eq(a1372, a1373) {
            let a1374 = 1u64;
            let a1375 = v1;
            if nat_lt(a1374, a1375) {
                let m1378 = {
                    let a1376 = v0;
                    let a1377 = 0u64;
                    index_bytes(a1376, a1377)
                };
                match m1378 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1379 = v2;
                        let a1380 = 48u8;
                        Ok(!equal(a1379, a1380))
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

pub fn f325(v0: Fn4, p1385: u64, p1386: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1384 = (p1385, p1386);
    {
        let mut m1384 = m1384;
        loop {
            let step = {
                let (v1, v2) = m1384;
                let m1381 = v1;
                if m1381 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1381 - 1;
                    let m1383 = {
                        let c1382: Fn4 = v0.clone();
                        c1382.apply(v2)?
                    };
                    match m1383 {
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
                Ok(next) => { m1384 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f433(v0, v1)
}

pub fn f327(v0: Bytes) -> R<u64> {
    let (h1387, _) = f321(Fn3::F434(v0.clone()), {
        let a1389 = {
            let a1388 = v0;
            length_bytes(a1388)
        };
        let a1390 = 1u64;
        nat_add(a1389, a1390)?
    }, 0u64)?;
    Ok(h1387)
}

pub fn f328(v0: u64) -> Bytes {
    let m1391 = v0;
    if m1391 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1391 - 1;
        let a1392 = f8(Str::lit("0"));
        let a1393 = f328(v1);
        append_bytes(a1392, a1393)
    }
}

pub fn f329(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1394 = v0;
    match m1394.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1395 = f93(v1, f8(Str::lit("std")));
            match m1395 {
                None => {
                    f329(v2)
                }
                Some(v3) => {
                    if f220(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f329(v2))
                    } else {
                        f329(v2)
                    }
                }
            }
        }
    }
}

pub fn f330(v0: Adt14, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1396 = v1;
    match m1396.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt14::C0(h1397, _, _) => {
                    h1397
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h1398, _) => {
                        h1398
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f330(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt14::C0(h1399, _, _) => {
                        h1399
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1400, _) => {
                            h1400
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f330(v0, v3))
                } else {
                    f330(v0, v3)
                }
            }
        }
    }
}

pub fn f331(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt17>> {
    let m1401 = v0;
    match m1401.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v4, v5)) => {
            if if f435(v4.clone(), v2.clone()) {
                true
            } else {
                f435(v4.clone(), v3.clone())
            } {
                f331(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt17::C0({
                    let (h1402, _) = v4.clone();
                    h1402
                }, {
                    let a1405 = {
                        let a1404 = f145(f436({
                            let (h1403, _) = v4.clone();
                            h1403
                        }, v1.clone()))?;
                        length_list(a1404)
                    };
                    let a1406 = 1u64;
                    if nat_eq(a1405, a1406) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1407) = v4.clone();
                    h1407
                }), f331(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f332(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1408 = v1;
    match m1408.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1409, _) = v2.clone();
                h1409
            }, v0.clone()) {
                List::cons({
                    let (_, h1410) = v2;
                    h1410
                }, f332(v0, v3))
            } else {
                if f52({
                    let (_, h1411) = v2;
                    h1411
                }, v0.clone()) {
                    List::cons({
                        let (h1412, _) = v2;
                        h1412
                    }, f332(v0, v3))
                } else {
                    f332(v0, v3)
                }
            }
        }
    }
}

pub fn f333(v0: Adt14, v1: Adt2, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt14::C0(h1413, _, _) => {
            h1413
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1414, _) => {
                h1414
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt14::C0(_, _, h1415) => {
                    h1415
                }
            }) {
                true
            } else {
                if f220(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1416, _) => {
                            h1416
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f334(v0: Adt2) -> bool {
    if f220(v0.clone(), f8(Str::lit("domain"))) {
        if f220(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f437()) {
                if f438(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f438(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f438(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f438(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f335(v0: Adt2) -> bool {
    if f220(v0.clone(), f8(Str::lit("subject"))) {
        if f220(v0.clone(), f8(Str::lit("object"))) {
            if f220(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f439())
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

pub fn f336(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1417 = f43(v1)?;
    match m1417 {
        None => {
            Ok(f226(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f214(f187(f93(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1418 = f92(f93(v4.clone(), f8(Str::lit("construct"))));
                match m1418 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1419 = v5;
            match m1419.uncons() {
                None => {
                    Ok(f226(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1420 = v9;
                    match m1420.uncons() {
                        None => {
                            Ok(f226(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f51(f32(v8, f8(Str::lit("std")))) {
                                Ok(f226(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f51(f52(v10.clone(), v2)) {
                                    Ok(f226(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f117(f51(f32(v7.clone(), f8(Str::lit("none")))), f51(f52(v7.clone(), v3))) {
                                        Ok(f226(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1421 = f92(f93(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1421 {
                                            None => {
                                                Ok(f226(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1422 = f93(v4, f8(Str::lit("value")));
                                                match m1422 {
                                                    None => {
                                                        Ok(f226(v0, f8(Str::lit("a row has no value"))))
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

pub fn f337(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1423 = v1;
    match m1423.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f440(v3.clone())
            } else {
                f51(f440(v3.clone()))
            } {
                List::cons(f239(v0.clone(), v3), f337(v0, v4, v2))
            } else {
                f337(v0, v4, v2)
            }
        }
    }
}

pub fn f338(p1426: List<Bytes>) -> R<Option<Bytes>> {
    let m1425 = p1426;
    {
        let mut m1425 = m1425;
        loop {
            let step = {
                let v0 = m1425;
                let m1424 = v0;
                match m1424.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f441(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f197(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1425 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1427 = v1;
    match m1427.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f95(f64(v0.clone(), v2))?, f339(v0, v3)?))
        }
    }
}

pub fn f340(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1428 = v0.clone();
        length_bytes(a1428)
    };
    let v3: u64 = {
        let a1429 = v1.clone();
        length_bytes(a1429)
    };
    let v4: u64 = {
        let (h1430, _) = f321(Fn3::F442(v1.clone(), v3, v0.clone(), v2), {
            let a1431 = v2;
            let a1432 = 1u64;
            nat_add(a1431, a1432)?
        }, 0u64)?;
        h1430
    };
    if f443(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f341() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f342(v0: Adt15) -> R<Adt18> {
    Ok(Adt18::C0(f444(match v0.clone() {
        Adt15::C0(h1433, _, _, _, _) => {
            h1433
        }
    }, 0u64)?, f444(match v0.clone() {
        Adt15::C0(_, h1434, _, _, _) => {
            h1434
        }
    }, 0u64)?, f444(match v0.clone() {
        Adt15::C0(_, _, h1435, _, _) => {
            h1435
        }
    }, 0u64)?, f444(match v0 {
        Adt15::C0(_, _, _, h1436, _) => {
            h1436
        }
    }, 0u64)?))
}

pub fn f343(v0: Adt15) -> List<Bytes> {
    f125(match v0.clone() {
        Adt15::C0(h1437, _, _, _, _) => {
            h1437
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, h1438, _, _) => {
            h1438
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, _, h1439, _) => {
            h1439
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1440) => {
            h1440
        }
    })))
}

pub fn f344(v0: Adt18, p1446: List<u64>, p1447: List<Adt20>) -> Option<u64> {
    let m1445 = (p1446, p1447);
    {
        let mut m1445 = m1445;
        loop {
            let step = {
                let (v1, v2) = m1445;
                let m1441 = v2;
                match m1441.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f445(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1442) => {
                                h1442
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1443, _) => {
                                    h1443
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1444, _) => {
                                    h1444
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1445 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: Adt15, v1: List<Adt2>, v2: u64) -> R<List<Adt20>> {
    let m1448 = v1;
    match m1448.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f446(v0.clone(), v3)?), f345(v0, v4, {
                let a1449 = v2;
                let a1450 = 1u64;
                nat_add(a1449, a1450)?
            })?))
        }
    }
}

pub fn f346(p1454: List<Bytes>, p1455: u64) -> Bytes {
    let m1453 = (p1454, p1455);
    {
        let mut m1453 = m1453;
        loop {
            let step = {
                let (v0, v1) = m1453;
                let m1451 = v0;
                match m1451.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1452 = v1;
                        if m1452 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1452 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f447(v0.clone(), v1.clone()) {
        if f447(v1, v0) {
            f448(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f348(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1456 = v0;
    match m1456.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f449(v2, v1.clone())?, f348(v3, v1)?))
        }
    }
}

pub fn f349(v0: List<Adt2>) -> List<u64> {
    let m1457 = v0;
    match m1457.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f450(v1), f349(v2))
        }
    }
}

pub fn f350(p1460: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1459 = p1460;
    {
        let mut m1459 = m1459;
        loop {
            let step = {
                let v0 = m1459;
                let m1458 = v0;
                match m1458.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f451(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
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

pub fn f351(p1463: List<Bytes>, p1464: List<Bytes>) -> Option<Bytes> {
    let m1462 = (p1463, p1464);
    {
        let mut m1462 = m1462;
        loop {
            let step = {
                let (v0, v1) = m1462;
                let m1461 = v0;
                match m1461.uncons() {
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
                Ok(next) => { m1462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f353(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1465 = v0.clone();
    match m1465.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1466 = v2;
            let a1467 = v1.clone();
            if equal(a1466, a1467) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f354(v0: List<(Bytes, ())>) -> List<Bytes> {
    f236(f452(Fn7::F453, List::<Bytes>::nil(), v0))
}

pub fn f355(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f454(f455(Fn8::F456, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1472 = {
        let (h1468, _) = f457(Fn9::F458, {
            let a1470 = {
                let a1469 = v0;
                length_list(a1469)
            };
            let a1471 = 1u64;
            nat_add(a1470, a1471)?
        }, v1)?;
        h1468
    };
    match m1472.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f356(v0: List<Bytes>) -> List<(Bytes, ())> {
    f459(f460(Fn10::F461, List::<(Bytes, ())>::nil(), v0))
}

pub fn f357(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1473 = f44(v1, 34u8)?;
    match m1473.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1474 = v3;
            match m1474.uncons() {
                None => {
                    Ok(f462(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f358(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f463(v0.clone(), 0u64, {
        let a1475 = v0;
        length_bytes(a1475)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f359(v0: Adt2, v1: Bytes) -> Bytes {
    if f32(f464(f93(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f360(v0: Bytes, v1: Bytes, p1488: u64, p1489: u64) -> R<bool> {
    let m1487 = (p1488, p1489);
    {
        let mut m1487 = m1487;
        loop {
            let step = {
                let (v2, v3) = m1487;
                let m1476 = v3;
                if m1476 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1476 - 1;
                    let m1479 = {
                        let a1477 = v1.clone();
                        let a1478 = v2;
                        index_bytes(a1477, a1478)
                    };
                    match m1479 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1482 = {
                                let a1480 = v0.clone();
                                let a1481 = v2;
                                index_bytes(a1480, a1481)
                            };
                            match m1482 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1483 = v5;
                                    let a1484 = v6;
                                    if equal(a1483, a1484) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1485 = v2;
                                            let a1486 = 1u64;
                                            nat_add(a1485, a1486)?
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
                Ok(next) => { m1487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1490 = f91(v0, v1)?;
    match m1490 {
        None => {
            Ok(Adt2::C9)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f362(v0: Bytes) -> R<Bytes> {
    let a1491 = f8(Str::lit("Catalog."));
    let a1492 = f384(v0)?;
    Ok(append_bytes(a1491, a1492))
}

pub fn f363(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f384(f58(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f364(v0: List<Bytes>) -> List<Bytes> {
    let m1493 = v0;
    match m1493.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f239(f268(), v1), f364(v2))
            } else {
                f364(v2)
            }
        }
    }
}

pub fn f365(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1494 = v1;
    match m1494.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f465(f466(f95(f64(v0.clone(), v2))?)?), f365(v0, v3)?))
        }
    }
}

pub fn f366(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1495 = v1;
    match m1495.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f467(v0.clone(), v3, v2.clone())?;
            let (h1496, _) = v5.clone();
            if h1496 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f366(v0, v4, v2)?;
                let (h1497, _) = v6.clone();
                if h1497 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1500 = {
                            let (_, h1498) = v5;
                            h1498
                        };
                        let a1501 = {
                            let (_, h1499) = v6;
                            h1499
                        };
                        append_bytes(a1500, a1501)
                    }))
                }
            }
        }
    }
}

pub fn f367(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1502 = v1;
    match m1502.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f220(v2.clone(), f8(Str::lit("subject"))) {
                if f221(v0.clone(), v2.clone()) {
                    f367(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f367(v0, v3))
                }
            } else {
                if f377(v0.clone(), v2.clone()) {
                    f367(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f378(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f367(v0, v3))
                }
            }
        }
    }
}

pub fn f368(v0: Adt1) -> R<Bytes> {
    let a1504 = {
        let a1503 = f128(f129(f261(f258(f259(f64(v0.clone(), f212()))?)), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f131(f96(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1503)
    };
    let a1505 = 0u64;
    if nat_eq(a1504, a1505) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f369(v0: List<Adt2>) -> R<List<Bytes>> {
    let m1506 = v0;
    match m1506.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f122(f214(f187(f93(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f468(f95(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f369(v2)?))
        }
    }
}

pub fn f370() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f371(v0: Bytes, p1564: u64, p1565: u64, p1566: Adt19) -> R<Adt19> {
    let m1563 = (p1564, (p1565, p1566));
    {
        let mut m1563 = m1563;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1563;
                let m1507 = v2;
                if m1507 == 0 {
                    Err::<(u64, (u64, Adt19)), R<Adt19>>(Ok(v3))
                } else {
                    let v4 = m1507 - 1;
                    let v5: u64 = f469(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt19::C0(_, h1508, _, _, _) => {
                            h1508
                        }
                    } {
                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                            let a1509 = v1;
                            let a1510 = 1u64;
                            nat_add(a1509, a1510)?
                        }, (v4, Adt19::C0(true, false, match v3.clone() {
                            Adt19::C0(_, _, h1511, _, _) => {
                                h1511
                            }
                        }, match v3.clone() {
                            Adt19::C0(_, _, _, h1512, _) => {
                                h1512
                            }
                        }, match v3 {
                            Adt19::C0(_, _, _, _, h1513) => {
                                h1513
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt19::C0(h1514, _, _, _, _) => {
                                h1514
                            }
                        } {
                            let a1515 = v5;
                            let a1516 = 34u64;
                            if nat_eq(a1515, a1516) {
                                let a1519 = f469(v0.clone(), {
                                    let a1517 = v1;
                                    let a1518 = 1u64;
                                    nat_add(a1517, a1518)?
                                })?;
                                let a1520 = 34u64;
                                if nat_eq(a1519, a1520) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1521 = v1;
                                        let a1522 = 1u64;
                                        nat_add(a1521, a1522)?
                                    }, (v4, Adt19::C0(true, true, f470(match v3.clone() {
                                        Adt19::C0(_, _, h1523, _, _) => {
                                            h1523
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt19::C0(_, _, _, h1524, _) => {
                                            h1524
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1525) => {
                                            h1525
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1526 = v1;
                                        let a1527 = 1u64;
                                        nat_add(a1526, a1527)?
                                    }, (v4, Adt19::C0(false, false, match v3.clone() {
                                        Adt19::C0(_, _, h1528, _, _) => {
                                            h1528
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1529, _) => {
                                            h1529
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1530) => {
                                            h1530
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1531 = v1;
                                    let a1532 = 1u64;
                                    nat_add(a1531, a1532)?
                                }, (v4, Adt19::C0(true, false, f470(match v3.clone() {
                                    Adt19::C0(_, _, h1533, _, _) => {
                                        h1533
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt19::C0(_, _, _, h1534, _) => {
                                        h1534
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1535) => {
                                        h1535
                                    }
                                }))))
                            }
                        } else {
                            let a1536 = v5;
                            let a1537 = 34u64;
                            if nat_eq(a1536, a1537) {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1538 = v1;
                                    let a1539 = 1u64;
                                    nat_add(a1538, a1539)?
                                }, (v4, Adt19::C0(true, false, match v3.clone() {
                                    Adt19::C0(_, _, h1540, _, _) => {
                                        h1540
                                    }
                                }, match v3.clone() {
                                    Adt19::C0(_, _, _, h1541, _) => {
                                        h1541
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1542) => {
                                        h1542
                                    }
                                }))))
                            } else {
                                let a1543 = v5;
                                let a1544 = 44u64;
                                if nat_eq(a1543, a1544) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1545 = v1;
                                        let a1546 = 1u64;
                                        nat_add(a1545, a1546)?
                                    }, (v4, Adt19::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt19::C0(_, _, h1547, _, _) => {
                                            h1547
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1548, _) => {
                                            h1548
                                        }
                                    }), match v3 {
                                        Adt19::C0(_, _, _, _, h1549) => {
                                            h1549
                                        }
                                    }))))
                                } else {
                                    let a1550 = v5;
                                    let a1551 = 13u64;
                                    if nat_eq(a1550, a1551) {
                                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                            let a1552 = v1;
                                            let a1553 = 1u64;
                                            nat_add(a1552, a1553)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1554 = v5;
                                        let a1555 = 10u64;
                                        if nat_eq(a1554, a1555) {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1556 = v1;
                                                let a1557 = 1u64;
                                                nat_add(a1556, a1557)?
                                            }, (v4, f372(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1558 = v1;
                                                let a1559 = 1u64;
                                                nat_add(a1558, a1559)?
                                            }, (v4, Adt19::C0(false, false, f470(match v3.clone() {
                                                Adt19::C0(_, _, h1560, _, _) => {
                                                    h1560
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt19::C0(_, _, _, h1561, _) => {
                                                    h1561
                                                }
                                            }, match v3 {
                                                Adt19::C0(_, _, _, _, h1562) => {
                                                    h1562
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
                Ok(next) => { m1563 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(v0: Adt19) -> Adt19 {
    Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f471(List::cons(match v0.clone() {
        Adt19::C0(_, _, h1567, _, _) => {
            h1567
        }
    }, match v0.clone() {
        Adt19::C0(_, _, _, h1568, _) => {
            h1568
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt19::C0(_, _, _, _, h1569) => {
            h1569
        }
    }))
}

pub fn f373(p1572: List<List<Bytes>>, p1573: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1571 = (p1572, p1573);
    {
        let mut m1571 = m1571;
        loop {
            let step = {
                let (v0, v1) = m1571;
                let m1570 = v0;
                match m1570.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
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

pub fn f374(v0: Bytes, v1: List<Adt17>) -> List<Bytes> {
    let m1574 = v1;
    match m1574.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt17::C0(h1575, _, _) => {
                    h1575
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt17::C0(_, h1576, _) => {
                        h1576
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt17::C0(_, _, h1577) => {
                        h1577
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f374(v0, v3))
            } else {
                f374(v0, v3)
            }
        }
    }
}

pub fn f375(v0: Bytes, v1: List<Adt2>) -> List<Bytes> {
    let m1578 = v1;
    match m1578.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f375(v0, v3))
            } else {
                if f32(f58(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f58(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f375(v0, v3))
                } else {
                    f375(v0, v3)
                }
            }
        }
    }
}

pub fn f376(p1583: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1582 = p1583;
    {
        let mut m1582 = m1582;
        loop {
            let step = {
                let v0 = m1582;
                let m1579 = v0;
                match m1579.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1580 = v2;
                        match m1580.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1581 = v5;
                                    match m1581.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f472(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f377(v0: Adt14, v1: Adt2) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1584, _, _) => {
            h1584
        }
    }) {
        f52(f378(v1), match v0 {
            Adt14::C0(_, h1585, _) => {
                h1585
            }
        })
    } else {
        false
    }
}

pub fn f378(v0: Adt2) -> Bytes {
    let m1586 = f93(v0, f8(Str::lit("evidence")));
    match m1586 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f379(v0: Adt2) -> Bytes {
    if f220(v0.clone(), f8(Str::lit("req"))) {
        let a1587 = f8(Str::lit("req "));
        let a1588 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1587, a1588)
    } else {
        let m1589 = f93(v0.clone(), f8(Str::lit("std")));
        match m1589 {
            None => {
                let m1592 = f93(v0, f8(Str::lit("der")));
                match m1592 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f122(f214(f187(f93(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f220(v1.clone(), f8(Str::lit("row"))) {
                    let a1590 = f8(Str::lit(" row "));
                    let a1591 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1590, a1591)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f380(v0: Bytes, v1: Bytes) -> bool {
    let a1593 = v0;
    let a1594 = f393();
    if equal(a1593, a1594) {
        let a1598 = {
            let a1596 = {
                let a1595 = v1;
                length_bytes(a1595)
            };
            let a1597 = 0u64;
            nat_eq(a1596, a1597)
        };
        bool_not(a1598)
    } else {
        false
    }
}

pub fn f381(v0: Bytes) -> Bytes {
    f7(List::cons(f144(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f382(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1600 = v0;
    let a1601 = {
        let a1599 = Str::lit("redistributable");
        utf8_encode(a1599)
    };
    if equal(a1600, a1601) {
        f473(v1)
    } else {
        Ok(false)
    }
}

pub fn f383(v0: Adt2) -> bool {
    if f474(v0.clone(), f8(Str::lit("name"))) {
        if f475(v0.clone(), f8(Str::lit("schema"))) {
            let m1602 = f387(f214(f187(f93(v0, f8(Str::lit("citation"))))));
            match m1602 {
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

pub fn f384(v0: Bytes) -> R<Bytes> {
    f476(v0.clone(), 0u64, {
        let a1603 = v0;
        length_bytes(a1603)
    }, true, Bytes::lit(&[]))
}

pub fn f385(v0: Adt2) -> Bytes {
    if f220(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f220(v0.clone(), f8(Str::lit("id"))) {
            if f220(v0.clone(), f8(Str::lit("statement"))) {
                if f220(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f403(v0, f8(Str::lit("evidence")))
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

pub fn f386(v0: List<Bytes>) -> Adt2 {
    let m1604 = v0;
    match m1604.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f386(v2)))
        }
    }
}

pub fn f387(v0: List<Bytes>) -> Option<Adt2> {
    let m1605 = v0;
    match m1605.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1606 = v2;
                match m1606.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1607 = v2;
                    match m1607.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m1608 = v6;
                                match m1608.uncons() {
                                    None => {
                                        Adt2::C9
                                    }
                                    Some((v7, _)) => {
                                        Adt2::C11(f8(Str::lit("row")), std::rc::Rc::new(Adt2::C6(v7)), std::rc::Rc::new(Adt2::C9))
                                    }
                                }
                            }))))), std::rc::Rc::new(Adt2::C9)))
                        }
                    }
                } else {
                    if f32(v1, f8(Str::lit("der"))) {
                        let m1609 = v2;
                        match m1609.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m1610 = v10;
                                match m1610.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C11(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C6(v11)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f386(v12)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C6(v9)), std::rc::Rc::new(Adt2::C9))))))))), std::rc::Rc::new(Adt2::C9)))
                                    }
                                }
                            }
                        }
                    } else {
                        None::<Adt2>
                    }
                }
            }
        }
    }
}

pub fn f388(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1611 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1611.uncons() {
        None => {
            let m1612 = f387(v4);
            match m1612 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1613 = f8(Str::lit("Catalog."));
                        let a1614 = f384(v3.clone())?;
                        append_bytes(a1613, a1614)
                    };
                    let m1615 = f477(v2, v9, f384(v3.clone())?)?;
                    match m1615 {
                        Ok(v11) => {
                            f400(v0, v3.clone(), v8, v11, f8(Str::lit("Encoding entered through propose-encoding.")), List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f384(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::<(Bytes, Bytes)>::nil()))
                        }
                        Err(v10) => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v10, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f389(v0: Adt23) -> Bytes {
    let a1636 = f478(match v0.clone() {
        Adt23::C0(h1616, _, _, _, _, _, _, _) => {
            h1616
        }
    });
    let a1637 = {
        let a1634 = f478(match v0.clone() {
            Adt23::C0(_, h1617, _, _, _, _, _, _) => {
                h1617
            }
        });
        let a1635 = {
            let a1632 = f478(match v0.clone() {
                Adt23::C0(_, _, h1618, _, _, _, _, _) => {
                    h1618
                }
            });
            let a1633 = {
                let a1630 = f478(match v0.clone() {
                    Adt23::C0(_, _, _, h1619, _, _, _, _) => {
                        h1619
                    }
                });
                let a1631 = {
                    let a1628 = f478(match v0.clone() {
                        Adt23::C0(_, _, _, _, h1620, _, _, _) => {
                            h1620
                        }
                    });
                    let a1629 = {
                        let a1626 = f478(match v0.clone() {
                            Adt23::C0(_, _, _, _, _, h1621, _, _) => {
                                h1621
                            }
                        });
                        let a1627 = {
                            let a1624 = f478(match v0.clone() {
                                Adt23::C0(_, _, _, _, _, _, h1622, _) => {
                                    h1622
                                }
                            });
                            let a1625 = f478(match v0 {
                                Adt23::C0(_, _, _, _, _, _, _, h1623) => {
                                    h1623
                                }
                            });
                            append_bytes(a1624, a1625)
                        };
                        append_bytes(a1626, a1627)
                    };
                    append_bytes(a1628, a1629)
                };
                append_bytes(a1630, a1631)
            };
            append_bytes(a1632, a1633)
        };
        append_bytes(a1634, a1635)
    };
    append_bytes(a1636, a1637)
}

pub fn f390(v0: Bytes) -> R<Adt23> {
    let v1: u64 = {
        let a1638 = v0.clone();
        length_bytes(a1638)
    };
    let v2: u64 = {
        let a1644 = {
            let a1641 = {
                let a1639 = v1;
                let a1640 = 72u64;
                nat_add(a1639, a1640)?
            };
            let a1642 = 64u64;
            let a1643 = 0u64;
            nat_quot(a1641, a1642, a1643)
        };
        let a1645 = 64u64;
        nat_mul(a1644, a1645)?
    };
    let (_, h1646) = f479(Fn11::F480(v0, v1, v2), {
        let a1647 = v2;
        let a1648 = 64u64;
        let a1649 = 0u64;
        nat_quot(a1647, a1648, a1649)
    }, (0u64, f481()))?;
    Ok(h1646)
}

pub fn f391() -> Bytes {
    f8(Str::lit("target/release/verdant_readers"))
}

pub fn f392(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>) -> R<Adt12> {
    let m1650 = f91(v0.clone(), v2.clone())?;
    match m1650 {
        None => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
        }
        Some(v6) => {
            if f32(v4.clone(), f8(Str::lit("none"))) {
                f482(v0, v1, v2, v3, v4, v5, v6)
            } else {
                let v7: Bytes = {
                    let a1651 = f8(Str::lit("Catalog."));
                    let a1652 = v4.clone();
                    append_bytes(a1651, a1652)
                };
                if f51(f483(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, v7.clone(), v3.clone())) {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v3, List::cons(f8(Str::lit(" in model/admissions.json, so it cannot check the entries\u{a}")), List::<Bytes>::nil()))))))))
                } else {
                    let m1653 = f91(v0.clone(), f7(List::cons(f8(Str::lit("encodings/")), List::cons(v4.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))?;
                    match m1653 {
                        None => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: encodings/")), List::cons(v4, List::cons(f8(Str::lit(".schema.json is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v8) => {
                            let m1654 = f484(v8, v6.clone())?;
                            match m1654 {
                                None => {
                                    f482(v0, v1, v2, v3, {
                                        let a1655 = f8(Str::lit("Catalog."));
                                        let a1656 = v4;
                                        append_bytes(a1655, a1656)
                                    }, v5, v6)
                                }
                                Some(v9) => {
                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" does not conform to ")), List::cons(v7, List::cons(f8(Str::lit(": ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f393() -> Bytes {
    let a1657 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1657)
}

pub fn f394(v0: Adt2, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt2> = f93(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f292(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f292(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f292(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f292(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f292(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f292(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f292(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f292(Str::lit("url"), f464(v2.clone(), f8(Str::lit("url"))))?, List::cons(f292(Str::lit("media_type"), f464(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f292(Str::lit("license"), f464(v2.clone(), f8(Str::lit("license"))))?, List::cons(f292(Str::lit("redistribution"), f464(v2, f8(Str::lit("redistribution"))))?, List::cons(f292(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1658 = v1;
        match m1658 {
            None => {
                f7(List::cons(f292(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f292(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f292(Str::lit("artifact"), {
                    let (h1659, _) = v3;
                    h1659
                })?, List::cons(f292(Str::lit("checksum"), {
                    let (_, h1660) = v3;
                    h1660
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f485(f214(f187(f93(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f395(p1663: List<List<Bytes>>, v1: Bytes, p1664: List<Bytes>) -> R<List<Bytes>> {
    let m1662 = (p1663, p1664);
    {
        let mut m1662 = m1662;
        loop {
            let step = {
                let (v0, v2) = m1662;
                let m1661 = v0;
                match m1661.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f486(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f397(f487(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f318(v0)?))
}

pub fn f397(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1665 = v0;
    match m1665.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1666 = v3.clone();
            match m1666.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1669 = v2;
                    let a1670 = {
                        let a1667 = v1.clone();
                        let a1668 = f397(v3, v1);
                        append_bytes(a1667, a1668)
                    };
                    append_bytes(a1669, a1670)
                }
            }
        }
    }
}

pub fn f398(v0: Adt2, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1671 = f93(v0, v1);
    match m1671 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v3) => {
            f187(f93(v3, v2))
        }
    }
}

pub fn f399(v0: Bytes) -> bool {
    if f32(v0.clone(), f8(Str::lit("std"))) {
        true
    } else {
        if f32(v0.clone(), f8(Str::lit("req"))) {
            true
        } else {
            f32(v0, f8(Str::lit("der")))
        }
    }
}

pub fn f400(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a1672 = f8(Str::lit("Catalog."));
        let a1673 = f384(v1.clone())?;
        append_bytes(a1672, a1673)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f384(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f488(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f100(Adt2::C7(std::rc::Rc::new(f101(f489(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f100(Adt2::C7(std::rc::Rc::new(f101(f489(f96(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("path")), std::rc::Rc::new(Adt2::C6(v7.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("role")), std::rc::Rc::new(Adt2::C6(v4)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("stratum")), std::rc::Rc::new(Adt2::C3(false, 0u64)), std::rc::Rc::new(Adt2::C9)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a1677 = f490(f64(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v7.clone(), 4u64, {
            let a1675 = {
                let a1674 = v7;
                length_bytes(a1674)
            };
            let a1676 = 4u64;
            nat_sub(a1675, a1676)
        }))?;
        let a1678 = f8(Str::lit("\u{a}"));
        append_bytes(a1677, a1678)
    }), List::cons(f283(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f401(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f384(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f402(v0: Bytes) -> R<Bytes> {
    f491({
        let a1680 = {
            let a1679 = v0.clone();
            length_bytes(a1679)
        };
        let a1681 = 1u64;
        nat_add(a1680, a1681)?
    }, v0)
}

pub fn f403(v0: Adt2, v1: Bytes) -> bool {
    let m1682 = f93(v0, v1);
    match m1682 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f404(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1684 = {
                let a1683 = v0;
                length_bytes(a1683)
            };
            let a1685 = 1u64;
            nat_sub(a1684, a1685)
        })
    } else {
        v0
    }
}

pub fn f405(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1686 = v0;
    match m1686 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v8, r1687) => {
            let v9 = (*r1687).clone();
            Adt2::C1(v8, std::rc::Rc::new(v9))
        }
        Adt2::C2(v3) => {
            Adt2::C2(v3)
        }
        Adt2::C3(v4, v5) => {
            Adt2::C3(v4, v5)
        }
        Adt2::C4(v10, v11, v12) => {
            Adt2::C4(v10, v11, v12)
        }
        Adt2::C5(v13, v14) => {
            Adt2::C5(v13, v14)
        }
        Adt2::C6(v2) => {
            Adt2::C6(f492(f60(), v2, v1))
        }
        Adt2::C7(r1688) => {
            let v6 = (*r1688).clone();
            Adt2::C7(std::rc::Rc::new(v6))
        }
        Adt2::C8(r1689) => {
            let v7 = (*r1689).clone();
            Adt2::C8(std::rc::Rc::new(v7))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1690, r1691) => {
            let v15 = (*r1690).clone();
            let v16 = (*r1691).clone();
            Adt2::C10(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt2::C11(v17, r1692, r1693) => {
            let v18 = (*r1692).clone();
            let v19 = (*r1693).clone();
            Adt2::C11(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
    }
}

pub fn f406(v0: u64) -> R<Bytes> {
    f493(f494(), 10u64, v0)
}

pub fn f407(v0: bool) -> R<u64> {
    f409({
        let a1694 = f414(v0);
        let a1695 = 1u64;
        nat_sub(a1694, a1695)
    })
}

pub fn f408(v0: bool) -> R<u64> {
    f409(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f409(v0: u64) -> R<u64> {
    let m1696 = v0;
    if m1696 == 0 {
        Ok(1u64)
    } else {
        let v1 = m1696 - 1;
        let a1697 = 2u64;
        let a1698 = f409(v1)?;
        nat_mul(a1697, a1698)
    }
}

pub fn f410(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f411(v0: bool) -> R<u64> {
    let a1699 = f409(f410(v0))?;
    let a1700 = 1u64;
    Ok(nat_sub(a1699, a1700))
}

pub fn f412(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a1701 = Bytes::lit(&[0x2d]);
        let a1702 = v1;
        append_bytes(a1701, a1702)
    } else {
        v1
    }
}

pub fn f413(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f414(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f415(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        f495(f496(v2, f497(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f495(f496(v2, f497(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a1703 = v3.clone();
            length_bytes(a1703)
        };
        let v5: Bytes = {
            let a1704 = v2;
            let a1705 = v4;
            if nat_lt(a1704, a1705) {
                f46(v3.clone(), 0u64, {
                    let a1706 = v4;
                    let a1707 = v2;
                    nat_sub(a1706, a1707)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f498({
            let a1708 = v2;
            let a1709 = v4;
            if nat_lt(a1708, a1709) {
                f46(v3, {
                    let a1710 = v4;
                    let a1711 = v2;
                    nat_sub(a1710, a1711)
                }, v2)
            } else {
                let a1714 = f499({
                    let a1712 = v2;
                    let a1713 = v4;
                    nat_sub(a1712, a1713)
                });
                let a1715 = v3;
                append_bytes(a1714, a1715)
            }
        }, {
            let a1716 = v2;
            let a1717 = 1u64;
            nat_add(a1716, a1717)?
        })?;
        let a1719 = {
            let a1718 = v6.clone();
            length_bytes(a1718)
        };
        let a1720 = 0u64;
        if nat_eq(a1719, a1720) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f416(v0: Bytes, p1739: u64, p1740: u64, p1741: Bytes) -> R<Bytes> {
    let m1738 = (p1739, (p1740, p1741));
    {
        let mut m1738 = m1738;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1738;
                let m1721 = v2;
                if m1721 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1721 - 1;
                    let v5: u64 = f199(v0.clone(), v1, {
                        let a1725 = {
                            let a1723 = {
                                let a1722 = v0.clone();
                                length_bytes(a1722)
                            };
                            let a1724 = v1;
                            nat_sub(a1723, a1724)
                        };
                        let a1726 = 1u64;
                        nat_add(a1725, a1726)?
                    })?;
                    let v6: Bytes = {
                        let a1729 = v3;
                        let a1730 = f46(v0.clone(), v1, {
                            let a1727 = v5;
                            let a1728 = v1;
                            nat_sub(a1727, a1728)
                        });
                        append_bytes(a1729, a1730)
                    };
                    let m1733 = {
                        let a1731 = v0.clone();
                        let a1732 = v5;
                        index_bytes(a1731, a1732)
                    };
                    match m1733 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a1734 = v5;
                                let a1735 = 1u64;
                                nat_add(a1734, a1735)?
                            }, (v4, {
                                let a1736 = v6;
                                let a1737 = f500(v0.clone(), v5, v7)?;
                                append_bytes(a1736, a1737)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1738 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: Fn5, p1745: List<(Bytes, Bytes)>, p1746: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1744 = (p1745, p1746);
    {
        let mut m1744 = m1744;
        loop {
            let step = {
                let (v1, v2) = m1744;
                let m1742 = v2;
                match m1742.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1743: Fn5 = v0.clone();
                            c1743.apply(v1, v3)
                        }, v4))
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

pub fn f418(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f501(v0, v1)
}

pub fn f419(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f502(v0)
}

pub fn f420(v0: Fn6, p1750: List<Bytes>, p1751: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1749 = (p1750, p1751);
    {
        let mut m1749 = m1749;
        loop {
            let step = {
                let (v1, v2) = m1749;
                let m1747 = v2;
                match m1747.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1748: Fn6 = v0.clone();
                            c1748.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1749 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f503(v1)?, v0))
}

pub fn f422(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f504(v0)?, List::cons(f8(Str::lit(":")), List::cons(f504(v1)?, List::<Bytes>::nil())))))
}

pub fn f423(v0: Adt13) -> Bytes {
    let m1752 = v0;
    match m1752 {
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

pub fn f424(v0: Adt13) -> Bytes {
    let m1753 = v0;
    match m1753 {
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

pub fn f425(v0: Adt13) -> Bytes {
    let m1754 = v0;
    match m1754 {
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

pub fn f426(v0: Bytes, v1: u64) -> Option<u64> {
    let a1755 = 0u64;
    let a1756 = v1;
    if nat_lt(a1755, a1756) {
        if f505(v0, {
            let a1757 = v1;
            let a1758 = 1u64;
            nat_sub(a1757, a1758)
        }) {
            Some({
                let a1759 = v1;
                let a1760 = 1u64;
                nat_sub(a1759, a1760)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f427(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1761 = v2;
    let a1762 = v0;
    if nat_lt(a1761, a1762) {
        if f505(v1, v2) {
            Ok(Some({
                let a1763 = v2;
                let a1764 = 1u64;
                nat_add(a1763, a1764)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f428(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1767 = f46(v0.clone(), {
        let a1765 = v1;
        let a1766 = 2u64;
        nat_add(a1765, a1766)?
    }, 2u64);
    let a1768 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1767, a1768) {
        let m1771 = f506(f46(v0, {
            let a1769 = v1;
            let a1770 = 4u64;
            nat_add(a1769, a1770)?
        }, 2u64))?;
        match m1771 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f115(v2.clone(), 0u64)?;
                let a1772 = v3;
                let a1773 = 32u64;
                if nat_lt(a1772, a1773) {
                    if f507(v3) {
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

pub fn f429(v0: u64) -> Bytes {
    let a1774 = v0;
    let a1775 = 34u64;
    if nat_eq(a1774, a1775) {
        Bytes::lit(&[0x22])
    } else {
        let a1776 = v0;
        let a1777 = 92u64;
        if nat_eq(a1776, a1777) {
            Bytes::lit(&[0x5c])
        } else {
            let a1778 = v0;
            let a1779 = 98u64;
            if nat_eq(a1778, a1779) {
                Bytes::lit(&[0x08])
            } else {
                let a1780 = v0;
                let a1781 = 102u64;
                if nat_eq(a1780, a1781) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1782 = v0;
                    let a1783 = 110u64;
                    if nat_eq(a1782, a1783) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1784 = v0;
                        let a1785 = 114u64;
                        if nat_eq(a1784, a1785) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1786 = v0;
                            let a1787 = 116u64;
                            if nat_eq(a1786, a1787) {
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

pub fn f430(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f115(v0, v1)?;
    let a1788 = 47u64;
    let a1789 = v2;
    if if nat_lt(a1788, a1789) {
        let a1790 = v2;
        let a1791 = 58u64;
        nat_lt(a1790, a1791)
    } else {
        false
    } {
        Ok(Some({
            let a1792 = v1;
            let a1793 = 1u64;
            nat_add(a1792, a1793)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f431(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1794 = f115(v0, v1)?;
    let a1795 = 48u64;
    if nat_eq(a1794, a1795) {
        Ok(Some({
            let a1796 = v1;
            let a1797 = 1u64;
            nat_add(a1796, a1797)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f432(v0: Bytes) -> R<u64> {
    let (h1798, _) = f321(Fn3::F508(v0.clone()), {
        let a1799 = v0;
        length_bytes(a1799)
    }, 0u64)?;
    Ok(h1798)
}

pub fn f433(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1803 = {
        let a1801 = v0;
        let a1802 = {
            let (h1800, _) = v1;
            h1800
        };
        index_bytes(a1801, a1802)
    };
    match m1803 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1805 = {
                    let (h1804, _) = v1;
                    h1804
                };
                let a1806 = 1u64;
                nat_add(a1805, a1806)?
            }, {
                let a1810 = {
                    let a1808 = {
                        let (_, h1807) = v1;
                        h1807
                    };
                    let a1809 = 10u64;
                    nat_mul(a1808, a1809)?
                };
                let a1811 = f509(v2)?;
                nat_add(a1810, a1811)?
            })))
        }
    }
}

pub fn f434(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f510(v0, v1)
}

pub fn f435(v0: (Bytes, Bytes), p1814: List<(Bytes, Bytes)>) -> bool {
    let m1813 = p1814;
    {
        let mut m1813 = m1813;
        loop {
            let step = {
                let v1 = m1813;
                let m1812 = v1;
                match m1812.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f511(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1813 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1815 = v1;
    match m1815.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1816, _) = v2;
                h1816
            }, v0.clone()) {
                List::cons({
                    let (_, h1817) = v2;
                    h1817
                }, f436(v0, v3))
            } else {
                f436(v0, v3)
            }
        }
    }
}

pub fn f437() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f438(v0: Adt2, v1: Bytes, v2: Bytes) -> bool {
    let m1818 = f93(v0, v1);
    match m1818 {
        None => {
            false
        }
        Some(v3) => {
            f220(v3, v2)
        }
    }
}

pub fn f439() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f440(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f441(v0: Bytes) -> R<bool> {
    let v1: Bytes = f197(v0)?;
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
        let a1820 = {
            let a1819 = v1;
            length_bytes(a1819)
        };
        let a1821 = v2;
        nat_sub(a1820, a1821)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1822 = f340(v3.clone(), f8(Str::lit(")")))?;
        match m1822 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1823 = v4;
                    let a1824 = 1u64;
                    nat_add(a1823, a1824)?
                }, {
                    let a1828 = {
                        let a1825 = v3;
                        length_bytes(a1825)
                    };
                    let a1829 = {
                        let a1826 = v4;
                        let a1827 = 1u64;
                        nat_add(a1826, a1827)?
                    };
                    nat_sub(a1828, a1829)
                })
            }
        }
    } else {
        v3
    };
    let a1830 = 0u64;
    let a1831 = v2;
    if nat_lt(a1830, a1831) {
        f512(v5)
    } else {
        Ok(false)
    }
}

pub fn f442(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f443(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1834 = {
            let a1832 = v4;
            let a1833 = v1;
            nat_add(a1832, a1833)?
        };
        let a1835 = v3;
        if nat_lt(a1834, a1835) {
            Ok(Some({
                let a1836 = v4;
                let a1837 = 1u64;
                nat_add(a1836, a1837)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f443(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1842 = {
        let a1839 = v0;
        let a1840 = v2;
        let a1841 = {
            let a1838 = v1.clone();
            length_bytes(a1838)
        };
        slice_bytes(a1839, a1840, a1841)
    };
    match m1842 {
        None => {
            false
        }
        Some(v3) => {
            let a1843 = v3;
            let a1844 = v1;
            equal(a1843, a1844)
        }
    }
}

pub fn f444(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1845 = v0;
    match m1845.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f444(v3, {
                let a1846 = v1;
                let a1847 = 1u64;
                nat_add(a1846, a1847)?
            })?))
        }
    }
}

pub fn f445(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1848 = v2;
    match m1848 {
        Adt21::C0(v3, v4) => {
            if f451(v3, match v0.clone() {
                Adt18::C0(h1849, _, _, _) => {
                    h1849
                }
            }) {
                f513(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f451(v5, match v0 {
                Adt18::C0(_, _, h1850, _) => {
                    h1850
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f447(v7, v1) {
                f451(v8, match v0 {
                    Adt18::C0(_, _, _, h1851) => {
                        h1851
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f446(v0: Adt15, v1: Adt2) -> R<Adt21> {
    let m1852 = f93(v1.clone(), f8(Str::lit("std")));
    match m1852 {
        None => {
            let m1857 = f92(f93(v1.clone(), f8(Str::lit("req"))));
            match m1857 {
                None => {
                    let m1859 = f93(v1, f8(Str::lit("der")));
                    match m1859 {
                        None => {
                            Ok(f514(v0))
                        }
                        Some(v5) => {
                            let m1860 = f515(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1860 {
                                None => {
                                    Ok(f514(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f348(f214(f187(f93(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1861) => {
                                            h1861
                                        }
                                    })?, f449(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1862, _) => {
                                            h1862
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f449(v4, match v0 {
                        Adt15::C0(_, _, h1858, _, _) => {
                            h1858
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1853 = f92(f93(v2.clone(), f8(Str::lit("row"))));
            match m1853 {
                None => {
                    Ok(Adt21::C0(f449(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1854, _, _, _, _) => {
                            h1854
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f449(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1855, _, _, _, _) => {
                            h1855
                        }
                    })?, Some(f449(v3, match v0 {
                        Adt15::C0(_, h1856, _, _, _) => {
                            h1856
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f447(p1865: List<u64>, v1: List<u64>) -> bool {
    let m1864 = p1865;
    {
        let mut m1864 = m1864;
        loop {
            let step = {
                let v0 = m1864;
                let m1863 = v0;
                match m1863.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f451(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f448(p1870: List<u64>) -> bool {
    let m1869 = p1870;
    {
        let mut m1869 = m1869;
        loop {
            let step = {
                let v0 = m1869;
                let m1866 = v0;
                match m1866.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1867 = v1;
                        let a1868 = 7u64;
                        if nat_le(a1867, a1868) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1871 = f516(v0, v1.clone(), 0u64)?;
    match m1871 {
        None => {
            let a1872 = v1;
            Ok(length_list(a1872))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f450(v0: Adt2) -> u64 {
    let m1873 = f517(f93(v0, f8(Str::lit("stratum"))));
    match m1873 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f451(v0: u64, p1878: List<u64>) -> bool {
    let m1877 = p1878;
    {
        let mut m1877 = m1877;
        loop {
            let step = {
                let v1 = m1877;
                let m1874 = v1;
                match m1874.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1875 = v2;
                        let a1876 = v0;
                        if nat_eq(a1875, a1876) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1877 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(v0: Fn7, p1882: List<Bytes>, p1883: List<(Bytes, ())>) -> List<Bytes> {
    let m1881 = (p1882, p1883);
    {
        let mut m1881 = m1881;
        loop {
            let step = {
                let (v1, v2) = m1881;
                let m1879 = v2;
                match m1879.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1880: Fn7 = v0.clone();
                            c1880.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1884, _) = v1;
        h1884
    }, v0)
}

pub fn f454(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f518(Fn12::F519, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f455(v0: Fn8, p1888: List<List<(Bytes, ())>>, p1889: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1887 = (p1888, p1889);
    {
        let mut m1887 = m1887;
        loop {
            let step = {
                let (v1, v2) = m1887;
                let m1885 = v2;
                match m1885.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1886: Fn8 = v0.clone();
                            c1886.apply(v1, v3)
                        }, v4))
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

pub fn f456(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f457(v0: Fn9, p1894: u64, p1895: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1893 = (p1894, p1895);
    {
        let mut m1893 = m1893;
        loop {
            let step = {
                let (v1, v2) = m1893;
                let m1890 = v1;
                if m1890 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1890 - 1;
                    let m1892 = {
                        let c1891: Fn9 = v0.clone();
                        c1891.apply(v2.clone())?
                    };
                    match m1892 {
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
                Ok(next) => { m1893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f458(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f520(v0)
}

pub fn f459(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f521(Fn13::F522, List::<(Bytes, ())>::nil(), v0)
}

pub fn f460(v0: Fn10, p1899: List<(Bytes, ())>, p1900: List<Bytes>) -> List<(Bytes, ())> {
    let m1898 = (p1899, p1900);
    {
        let mut m1898 = m1898;
        loop {
            let step = {
                let (v1, v2) = m1898;
                let m1896 = v2;
                match m1896.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1897: Fn10 = v0.clone();
                            c1897.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1898 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f462(p1905: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1904 = p1905;
    {
        let mut m1904 = m1904;
        loop {
            let step = {
                let v0 = m1904;
                let m1901 = v0;
                match m1901.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1902, _) = v2;
                            h1902
                        }) {
                            let (_, h1903) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1903)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1904 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: Bytes, p1931: u64, p1932: u64, p1933: Bytes) -> R<Bytes> {
    let m1930 = (p1931, (p1932, p1933));
    {
        let mut m1930 = m1930;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1930;
                let m1906 = v2;
                if m1906 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1906 - 1;
                    let m1909 = {
                        let a1907 = v0.clone();
                        let a1908 = v1;
                        index_bytes(a1907, a1908)
                    };
                    match m1909 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f202({
                                let a1910 = v6;
                                let a1911 = 34u64;
                                nat_eq(a1910, a1911)
                            }, {
                                let a1912 = v6;
                                let a1913 = 92u64;
                                nat_eq(a1912, a1913)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1914 = v1;
                                    let a1915 = 1u64;
                                    nat_add(a1914, a1915)?
                                }, (v4, {
                                    let a1918 = v3;
                                    let a1919 = {
                                        let a1916 = f8(Str::lit("\\"));
                                        let a1917 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1916, a1917)
                                    };
                                    append_bytes(a1918, a1919)
                                })))
                            } else {
                                let a1920 = v6;
                                let a1921 = 10u64;
                                if nat_eq(a1920, a1921) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1922 = v1;
                                        let a1923 = 1u64;
                                        nat_add(a1922, a1923)?
                                    }, (v4, {
                                        let a1924 = v3;
                                        let a1925 = f8(Str::lit("\\n"));
                                        append_bytes(a1924, a1925)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1926 = v1;
                                        let a1927 = 1u64;
                                        nat_add(a1926, a1927)?
                                    }, (v4, {
                                        let a1928 = v3;
                                        let a1929 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1928, a1929)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1930 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f464(v0: Option<Adt2>, v1: Bytes) -> Bytes {
    let m1934 = v0;
    match m1934 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f465(v0: List<Adt2>) -> List<Bytes> {
    let m1935 = v0;
    match m1935.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f523(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f285(f58(v1, f8(Str::lit("authority")))), f465(v2)))
        }
    }
}

pub fn f466(v0: List<Bytes>) -> R<List<Adt2>> {
    let m1936 = f298(v0.clone())?;
    match m1936 {
        None => {
            Ok(f277(f276(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt2>::nil())
        }
    }
}

pub fn f467(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    let m1937 = f298(v3.clone())?;
    match m1937 {
        None => {
            let m1938 = f276(v3.clone())?;
            match m1938 {
                None => {
                    f526(v0, v1, v3)
                }
                Some(v5) => {
                    f525(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f524(v0, v1, v4)
        }
    }
}

pub fn f468(v0: List<Bytes>) -> List<Bytes> {
    let m1939 = v0;
    match m1939.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1941 = {
                    let a1940 = v1.clone();
                    length_bytes(a1940)
                };
                let a1942 = 0u64;
                if nat_eq(a1941, a1942) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f468(v2))
        }
    }
}

pub fn f469(v0: Bytes, v1: u64) -> R<u64> {
    let m1945 = {
        let a1943 = v0;
        let a1944 = v1;
        index_bytes(a1943, a1944)
    };
    match m1945 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f470(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1949 = {
        let a1946 = v1;
        let a1947 = v2;
        let a1948 = 1u64;
        slice_bytes(a1946, a1947, a1948)
    };
    match m1949 {
        None => {
            v0
        }
        Some(v3) => {
            let a1950 = v0;
            let a1951 = v3;
            append_bytes(a1950, a1951)
        }
    }
}

pub fn f471(p1954: List<Bytes>, p1955: List<Bytes>) -> List<Bytes> {
    let m1953 = (p1954, p1955);
    {
        let mut m1953 = m1953;
        loop {
            let step = {
                let (v0, v1) = m1953;
                let m1952 = v0;
                match m1952.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1953 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: List<Bytes>) -> Bytes {
    let m1956 = v0;
    match m1956.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f473(v0: Bytes) -> R<bool> {
    let m1958 = {
        let a1957 = v0.clone();
        length_bytes(a1957)
    };
    if m1958 == 0 {
        Ok(true)
    } else {
        let v1 = m1958 - 1;
        let m1961 = {
            let a1959 = v0;
            let a1960 = v1;
            index_bytes(a1959, a1960)
        };
        match m1961 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f527({
                    let v3: u8 = v2;
                    let a2006 = {
                        let a1982 = {
                            let a1970 = {
                                let a1964 = {
                                    let a1962 = v3;
                                    let a1963 = 128u8;
                                    fixed_u8::bit_and(a1962, a1963)
                                };
                                let a1965 = 128u8;
                                if equal(a1964, a1965) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1971 = {
                                let a1968 = {
                                    let a1966 = v3;
                                    let a1967 = 64u8;
                                    fixed_u8::bit_and(a1966, a1967)
                                };
                                let a1969 = 64u8;
                                if equal(a1968, a1969) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1970, a1971)?
                        };
                        let a1983 = {
                            let a1980 = {
                                let a1974 = {
                                    let a1972 = v3;
                                    let a1973 = 32u8;
                                    fixed_u8::bit_and(a1972, a1973)
                                };
                                let a1975 = 32u8;
                                if equal(a1974, a1975) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1981 = {
                                let a1978 = {
                                    let a1976 = v3;
                                    let a1977 = 16u8;
                                    fixed_u8::bit_and(a1976, a1977)
                                };
                                let a1979 = 16u8;
                                if equal(a1978, a1979) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1980, a1981)?
                        };
                        nat_add(a1982, a1983)?
                    };
                    let a2007 = {
                        let a2004 = {
                            let a1992 = {
                                let a1986 = {
                                    let a1984 = v3;
                                    let a1985 = 8u8;
                                    fixed_u8::bit_and(a1984, a1985)
                                };
                                let a1987 = 8u8;
                                if equal(a1986, a1987) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1993 = {
                                let a1990 = {
                                    let a1988 = v3;
                                    let a1989 = 4u8;
                                    fixed_u8::bit_and(a1988, a1989)
                                };
                                let a1991 = 4u8;
                                if equal(a1990, a1991) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1992, a1993)?
                        };
                        let a2005 = {
                            let a2002 = {
                                let a1996 = {
                                    let a1994 = v3;
                                    let a1995 = 2u8;
                                    fixed_u8::bit_and(a1994, a1995)
                                };
                                let a1997 = 2u8;
                                if equal(a1996, a1997) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2003 = {
                                let a2000 = {
                                    let a1998 = v3;
                                    let a1999 = 1u8;
                                    fixed_u8::bit_and(a1998, a1999)
                                };
                                let a2001 = 1u8;
                                if equal(a2000, a2001) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2002, a2003)?
                        };
                        nat_add(a2004, a2005)?
                    };
                    nat_add(a2006, a2007)?
                }))
            }
        }
    }
}

pub fn f474(v0: Adt2, v1: Bytes) -> bool {
    let a2009 = {
        let a2008 = f58(v0, v1);
        length_bytes(a2008)
    };
    let a2010 = 0u64;
    !nat_eq(a2009, a2010)
}

pub fn f475(v0: Adt2, v1: Bytes) -> bool {
    let m2011 = f93(v0, v1);
    match m2011 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f476(v0: Bytes, p2032: u64, p2033: u64, p2034: bool, p2035: Bytes) -> R<Bytes> {
    let m2031 = (p2032, (p2033, (p2034, p2035)));
    {
        let mut m2031 = m2031;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m2031;
                let m2012 = v2;
                if m2012 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m2012 - 1;
                    let m2015 = {
                        let a2013 = v0.clone();
                        let a2014 = v1;
                        index_bytes(a2013, a2014)
                    };
                    match m2015 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f528(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a2016 = v1;
                                    let a2017 = 1u64;
                                    nat_add(a2016, a2017)?
                                }, (v5, (true, v4))))
                            } else {
                                if f117(f529(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a2018 = v1;
                                        let a2019 = 1u64;
                                        nat_add(a2018, a2019)?
                                    }, (v5, (false, {
                                        let a2023 = v4;
                                        let a2024 = f46({
                                            let a2020 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a2020)
                                        }, {
                                            let a2021 = v7;
                                            let a2022 = 97u64;
                                            nat_sub(a2021, a2022)
                                        }, 1u64);
                                        append_bytes(a2023, a2024)
                                    }))))
                                } else {
                                    if f202(f529(v7), f202(f530(v7), f531(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2025 = v1;
                                            let a2026 = 1u64;
                                            nat_add(a2025, a2026)?
                                        }, (v5, (false, {
                                            let a2027 = v4;
                                            let a2028 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a2027, a2028)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2029 = v1;
                                            let a2030 = 1u64;
                                            nat_add(a2029, a2030)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2031 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f532(f533(f93(v0.clone(), f8(Str::lit("$defs")))), f533(f93(v0.clone(), f8(Str::lit("definitions")))));
    let m2036 = f534(4096u64, v3.clone(), List::<Bytes>::nil(), f535(v3)?)?;
    match m2036 {
        Ok(v5) => {
            let m2037 = f536(4096u64, f537(f538(v5)?, List::cons(Adt24::C0(v2, v0), List::<Adt24>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m2037 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f397(f242(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f478(v0: u32) -> Bytes {
    let a2066 = f539({
        let a2038 = f540(v0, 28u32);
        let a2039 = 15u32;
        fixed_u32::bit_and(a2038, a2039)
    });
    let a2067 = {
        let a2064 = f539({
            let a2040 = f540(v0, 24u32);
            let a2041 = 15u32;
            fixed_u32::bit_and(a2040, a2041)
        });
        let a2065 = {
            let a2062 = f539({
                let a2042 = f540(v0, 20u32);
                let a2043 = 15u32;
                fixed_u32::bit_and(a2042, a2043)
            });
            let a2063 = {
                let a2060 = f539({
                    let a2044 = f540(v0, 16u32);
                    let a2045 = 15u32;
                    fixed_u32::bit_and(a2044, a2045)
                });
                let a2061 = {
                    let a2058 = f539({
                        let a2046 = f540(v0, 12u32);
                        let a2047 = 15u32;
                        fixed_u32::bit_and(a2046, a2047)
                    });
                    let a2059 = {
                        let a2056 = f539({
                            let a2048 = f540(v0, 8u32);
                            let a2049 = 15u32;
                            fixed_u32::bit_and(a2048, a2049)
                        });
                        let a2057 = {
                            let a2054 = f539({
                                let a2050 = f540(v0, 4u32);
                                let a2051 = 15u32;
                                fixed_u32::bit_and(a2050, a2051)
                            });
                            let a2055 = f539({
                                let a2052 = v0;
                                let a2053 = 15u32;
                                fixed_u32::bit_and(a2052, a2053)
                            });
                            append_bytes(a2054, a2055)
                        };
                        append_bytes(a2056, a2057)
                    };
                    append_bytes(a2058, a2059)
                };
                append_bytes(a2060, a2061)
            };
            append_bytes(a2062, a2063)
        };
        append_bytes(a2064, a2065)
    };
    append_bytes(a2066, a2067)
}

pub fn f479(v0: Fn11, p2071: u64, p2072: (u64, Adt23)) -> R<(u64, Adt23)> {
    let m2070 = (p2071, p2072);
    {
        let mut m2070 = m2070;
        loop {
            let step = {
                let (v1, v2) = m2070;
                let m2068 = v1;
                if m2068 == 0 {
                    Err::<(u64, (u64, Adt23)), R<(u64, Adt23)>>(Ok(v2))
                } else {
                    let v3 = m2068 - 1;
                    Ok::<(u64, (u64, Adt23)), R<(u64, Adt23)>>((v3, {
                        let c2069: Fn11 = v0.clone();
                        c2069.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2070 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt23)) -> R<(u64, Adt23)> {
    Ok(({
        let a2074 = {
            let (h2073, _) = v3.clone();
            h2073
        };
        let a2075 = 64u64;
        nat_add(a2074, a2075)?
    }, f541({
        let (_, h2076) = v3;
        h2076
    }, f542(v0, v1, v2, {
        let (h2077, _) = v3;
        h2077
    })?)?))
}

pub fn f481() -> Adt23 {
    Adt23::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f482(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Adt2) -> R<Adt12> {
    let m2078 = f543(f544(v1.clone(), v3.clone(), v4, v6))?;
    match m2078 {
        Ok(v8) => {
            let v9: Bytes = f523(v1.clone());
            Ok(Adt12::C3(List::cons((v9.clone(), v8), List::cons((f8(Str::lit("model/admissions.json")), f100(Adt2::C7(std::rc::Rc::new(f101(f489(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v9.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("row")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f386(v5)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C6(f286(f64(v0, v2))?)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(Adt2::C9)))))))))))))?), List::<(Bytes, Bytes)>::nil())), f7(List::cons(f8(Str::lit("admitted ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        }
        Err(v7) => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(": ")), List::cons(v7, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
        }
    }
}

pub fn f483(p2082: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m2081 = p2082;
    {
        let mut m2081 = m2081;
        loop {
            let step = {
                let v0 = m2081;
                let m2079 = v0;
                match m2079.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f117(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2080 = f93(v3, f8(Str::lit("std")));
                            match m2080 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    f32(f58(v5, f8(Str::lit("row"))), v2.clone())
                                }
                            }
                        }) {
                            Err::<List<Adt2>, bool>(true)
                        } else {
                            Ok::<List<Adt2>, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2081 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: Adt2, v1: Adt2) -> R<Option<Bytes>> {
    f545({
        let a2089 = {
            let a2084 = {
                let a2083 = f546(v0.clone())?;
                length_bytes(a2083)
            };
            let a2085 = 2u64;
            nat_add(a2084, a2085)?
        };
        let a2090 = {
            let a2087 = {
                let a2086 = f546(v1.clone())?;
                length_bytes(a2086)
            };
            let a2088 = 2u64;
            nat_add(a2087, a2088)?
        };
        nat_mul(a2089, a2090)?
    }, v0.clone(), v0, v1, Bytes::lit(&[]))
}

pub fn f485(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f397(f547(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f486(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2091 = f44(v1, 34u8)?;
    match m2091.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2092 = v3;
            match m2092.uncons() {
                None => {
                    Ok(f548(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f487(v0: List<Bytes>) -> List<Bytes> {
    let m2093 = v0;
    match m2093.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f487(v2)
            } else {
                List::cons(v1, f487(v2))
            }
        }
    }
}

pub fn f488(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2094 = v0;
    match m2094.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f488(v3, v1))
        }
    }
}

pub fn f489(v0: List<Adt2>, v1: Bytes, v2: Bytes, v3: Adt2) -> List<Adt2> {
    let m2095 = v0;
    match m2095.uncons() {
        None => {
            List::cons(v3, List::<Adt2>::nil())
        }
        Some((v4, v5)) => {
            if f32(f58(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f549(v5, v1, v2))
            } else {
                List::cons(v4, f489(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f490(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f397(f550(f95(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f491(p2106: u64, p2107: Bytes) -> R<Bytes> {
    let m2105 = (p2106, p2107);
    {
        let mut m2105 = m2105;
        loop {
            let step = {
                let (v0, v1) = m2105;
                let m2096 = v0;
                if m2096 == 0 {
                    Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                } else {
                    let v2 = m2096 - 1;
                    if f85(v1.clone(), f8(Str::lit("-- "))) {
                        let m2097 = f340(v1.clone(), f8(Str::lit("\u{a}")))?;
                        match m2097 {
                            None => {
                                Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                            }
                            Some(v3) => {
                                Ok::<(u64, Bytes), R<Bytes>>((v2, f46(v1.clone(), {
                                    let a2098 = v3;
                                    let a2099 = 1u64;
                                    nat_add(a2098, a2099)?
                                }, {
                                    let a2103 = {
                                        let a2100 = v1;
                                        length_bytes(a2100)
                                    };
                                    let a2104 = {
                                        let a2101 = v3;
                                        let a2102 = 1u64;
                                        nat_add(a2101, a2102)?
                                    };
                                    nat_sub(a2103, a2104)
                                })))
                            }
                        }
                    } else {
                        Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m2105 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492(p2110: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2109 = p2110;
    {
        let mut m2109 = m2109;
        loop {
            let step = {
                let v0 = m2109;
                let m2108 = v0;
                match m2108.uncons() {
                    None => {
                        Err::<List<Adt13>, Bytes>(v1.clone())
                    }
                    Some((v3, v4)) => {
                        if f85(v1.clone(), f102(v3.clone())) {
                            Err::<List<Adt13>, Bytes>(f69(v3, v2.clone()))
                        } else {
                            Ok::<List<Adt13>, Bytes>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2109 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2111 = v2;
    let a2112 = 0u64;
    if nat_eq(a2111, a2112) {
        Ok(f551(v0, 0u64))
    } else {
        let (_, h2113) = {
            let (h2114, _) = f552(Fn14::F553(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2114
        };
        Ok(h2113)
    }
}

pub fn f494() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f495(v0: List<u64>) -> R<Bytes> {
    let m2115 = f554(v0, List::<u64>::nil());
    match m2115.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a2116 = f406(v1)?;
            let a2117 = f555(v2)?;
            Ok(append_bytes(a2116, a2117))
        }
    }
}

pub fn f496(p2120: u64, p2121: List<u64>, v2: u64) -> R<List<u64>> {
    let m2119 = (p2120, p2121);
    {
        let mut m2119 = m2119;
        loop {
            let step = {
                let (v0, v1) = m2119;
                let m2118 = v0;
                if m2118 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m2118 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f556(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m2119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: u64, v1: u64) -> List<u64> {
    let m2122 = v0;
    if m2122 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m2122 - 1;
        let a2123 = v1;
        let a2124 = 0u64;
        if nat_eq(a2123, a2124) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a2125 = v1;
                let a2126 = f557();
                let a2127 = 0u64;
                nat_rem(a2125, a2126, a2127)
            }, f497(v2, {
                let a2128 = v1;
                let a2129 = f557();
                let a2130 = 0u64;
                nat_quot(a2128, a2129, a2130)
            }))
        }
    }
}

pub fn f498(p2141: Bytes, p2142: u64) -> R<Bytes> {
    let m2140 = (p2141, p2142);
    {
        let mut m2140 = m2140;
        loop {
            let step = {
                let (v0, v1) = m2140;
                let m2131 = v1;
                if m2131 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m2131 - 1;
                    let a2135 = f115(v0.clone(), {
                        let a2133 = {
                            let a2132 = v0.clone();
                            length_bytes(a2132)
                        };
                        let a2134 = 1u64;
                        nat_sub(a2133, a2134)
                    })?;
                    let a2136 = 48u64;
                    if nat_eq(a2135, a2136) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a2138 = {
                                let a2137 = v0;
                                length_bytes(a2137)
                            };
                            let a2139 = 1u64;
                            nat_sub(a2138, a2139)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m2140 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f499(v0: u64) -> Bytes {
    let m2143 = v0;
    if m2143 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2143 - 1;
        let a2144 = Bytes::lit(&[0x30]);
        let a2145 = f499(v1);
        append_bytes(a2144, a2145)
    }
}

pub fn f500(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2190 = {
            let a2166 = {
                let a2154 = {
                    let a2148 = {
                        let a2146 = v3;
                        let a2147 = 128u8;
                        fixed_u8::bit_and(a2146, a2147)
                    };
                    let a2149 = 128u8;
                    if equal(a2148, a2149) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2155 = {
                    let a2152 = {
                        let a2150 = v3;
                        let a2151 = 64u8;
                        fixed_u8::bit_and(a2150, a2151)
                    };
                    let a2153 = 64u8;
                    if equal(a2152, a2153) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2154, a2155)?
            };
            let a2167 = {
                let a2164 = {
                    let a2158 = {
                        let a2156 = v3;
                        let a2157 = 32u8;
                        fixed_u8::bit_and(a2156, a2157)
                    };
                    let a2159 = 32u8;
                    if equal(a2158, a2159) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2165 = {
                    let a2162 = {
                        let a2160 = v3;
                        let a2161 = 16u8;
                        fixed_u8::bit_and(a2160, a2161)
                    };
                    let a2163 = 16u8;
                    if equal(a2162, a2163) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2164, a2165)?
            };
            nat_add(a2166, a2167)?
        };
        let a2191 = {
            let a2188 = {
                let a2176 = {
                    let a2170 = {
                        let a2168 = v3;
                        let a2169 = 8u8;
                        fixed_u8::bit_and(a2168, a2169)
                    };
                    let a2171 = 8u8;
                    if equal(a2170, a2171) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2177 = {
                    let a2174 = {
                        let a2172 = v3;
                        let a2173 = 4u8;
                        fixed_u8::bit_and(a2172, a2173)
                    };
                    let a2175 = 4u8;
                    if equal(a2174, a2175) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2176, a2177)?
            };
            let a2189 = {
                let a2186 = {
                    let a2180 = {
                        let a2178 = v3;
                        let a2179 = 2u8;
                        fixed_u8::bit_and(a2178, a2179)
                    };
                    let a2181 = 2u8;
                    if equal(a2180, a2181) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2187 = {
                    let a2184 = {
                        let a2182 = v3;
                        let a2183 = 1u8;
                        fixed_u8::bit_and(a2182, a2183)
                    };
                    let a2185 = 1u8;
                    if equal(a2184, a2185) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2186, a2187)?
            };
            nat_add(a2188, a2189)?
        };
        nat_add(a2190, a2191)?
    };
    let a2192 = v4;
    let a2193 = 34u64;
    if nat_eq(a2192, a2193) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2194 = v4;
        let a2195 = 92u64;
        if nat_eq(a2194, a2195) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2196 = v4;
            let a2197 = 8u64;
            if nat_eq(a2196, a2197) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2198 = v4;
                let a2199 = 12u64;
                if nat_eq(a2198, a2199) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2200 = v4;
                    let a2201 = 10u64;
                    if nat_eq(a2200, a2201) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2202 = v4;
                        let a2203 = 13u64;
                        if nat_eq(a2202, a2203) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2204 = v4;
                            let a2205 = 9u64;
                            if nat_eq(a2204, a2205) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2206 = v4;
                                let a2207 = 32u64;
                                if nat_lt(a2206, a2207) {
                                    let a2208 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2209 = f558(v2)?;
                                    Ok(append_bytes(a2208, a2209))
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

pub fn f501(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2210 = v0.clone();
    match m2210.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2213 = {
                let (h2211, _) = v2;
                h2211
            };
            let a2214 = {
                let (h2212, _) = v1.clone();
                h2212
            };
            if equal(a2213, a2214) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f502(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f559(f560(Fn15::F561, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2219 = {
        let (h2215, _) = f562(Fn16::F563, {
            let a2217 = {
                let a2216 = v0;
                length_list(a2216)
            };
            let a2218 = 1u64;
            nat_add(a2217, a2218)?
        }, v1)?;
        h2215
    };
    match m2219.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f503(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2224 = {
        let a2221 = f309({
            let (h2220, _) = v0;
            h2220
        })?;
        let a2222 = Bytes::lit(&[0x3a]);
        append_bytes(a2221, a2222)
    };
    let a2225 = {
        let (_, h2223) = v0;
        h2223
    };
    Ok(append_bytes(a2224, a2225))
}

pub fn f504(v0: Bytes) -> R<Bytes> {
    let a2231 = {
        let a2229 = Bytes::lit(&[0x22]);
        let a2230 = {
            let (_, h2226) = {
                let (h2227, _) = f552(Fn14::F564(v0.clone()), {
                    let a2228 = v0;
                    length_bytes(a2228)
                }, (0u64, Bytes::lit(&[])))?;
                h2227
            };
            h2226
        };
        append_bytes(a2229, a2230)
    };
    let a2232 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2231, a2232))
}

pub fn f505(v0: Bytes, v1: u64) -> bool {
    let m2233 = f207(v0, v1);
    match m2233 {
        None => {
            false
        }
        Some(v2) => {
            f565(v2)
        }
    }
}

pub fn f506(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2234 = v0.clone();
        length_bytes(a2234)
    };
    let a2238 = {
        let a2235 = v1;
        let a2236 = 2u64;
        let a2237 = 0u64;
        nat_rem(a2235, a2236, a2237)
    };
    let a2239 = 0u64;
    if nat_eq(a2238, a2239) {
        let (_, h2240) = {
            let (h2241, _) = f566(Fn17::F567(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2241
        };
        Ok(h2240)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f507(v0: u64) -> bool {
    let a2242 = v0;
    let a2243 = 8u64;
    if nat_eq(a2242, a2243) {
        true
    } else {
        let a2244 = v0;
        let a2245 = 9u64;
        if nat_eq(a2244, a2245) {
            true
        } else {
            let a2246 = v0;
            let a2247 = 10u64;
            if nat_eq(a2246, a2247) {
                true
            } else {
                let a2248 = v0;
                let a2249 = 12u64;
                if nat_eq(a2248, a2249) {
                    true
                } else {
                    let a2250 = v0;
                    let a2251 = 13u64;
                    nat_eq(a2250, a2251)
                }
            }
        }
    }
}

pub fn f508(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2254 = {
        let a2252 = v0;
        let a2253 = v1;
        index_bytes(a2252, a2253)
    };
    match m2254 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f568(v2)? {
                Ok(Some({
                    let a2255 = v1;
                    let a2256 = 1u64;
                    nat_add(a2255, a2256)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f509(v0: u8) -> R<u64> {
    let a2303 = {
        let v1: u8 = v0;
        let a2301 = {
            let a2277 = {
                let a2265 = {
                    let a2259 = {
                        let a2257 = v1;
                        let a2258 = 128u8;
                        fixed_u8::bit_and(a2257, a2258)
                    };
                    let a2260 = 128u8;
                    if equal(a2259, a2260) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2266 = {
                    let a2263 = {
                        let a2261 = v1;
                        let a2262 = 64u8;
                        fixed_u8::bit_and(a2261, a2262)
                    };
                    let a2264 = 64u8;
                    if equal(a2263, a2264) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2265, a2266)?
            };
            let a2278 = {
                let a2275 = {
                    let a2269 = {
                        let a2267 = v1;
                        let a2268 = 32u8;
                        fixed_u8::bit_and(a2267, a2268)
                    };
                    let a2270 = 32u8;
                    if equal(a2269, a2270) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2276 = {
                    let a2273 = {
                        let a2271 = v1;
                        let a2272 = 16u8;
                        fixed_u8::bit_and(a2271, a2272)
                    };
                    let a2274 = 16u8;
                    if equal(a2273, a2274) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2275, a2276)?
            };
            nat_add(a2277, a2278)?
        };
        let a2302 = {
            let a2299 = {
                let a2287 = {
                    let a2281 = {
                        let a2279 = v1;
                        let a2280 = 8u8;
                        fixed_u8::bit_and(a2279, a2280)
                    };
                    let a2282 = 8u8;
                    if equal(a2281, a2282) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2288 = {
                    let a2285 = {
                        let a2283 = v1;
                        let a2284 = 4u8;
                        fixed_u8::bit_and(a2283, a2284)
                    };
                    let a2286 = 4u8;
                    if equal(a2285, a2286) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2287, a2288)?
            };
            let a2300 = {
                let a2297 = {
                    let a2291 = {
                        let a2289 = v1;
                        let a2290 = 2u8;
                        fixed_u8::bit_and(a2289, a2290)
                    };
                    let a2292 = 2u8;
                    if equal(a2291, a2292) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2298 = {
                    let a2295 = {
                        let a2293 = v1;
                        let a2294 = 1u8;
                        fixed_u8::bit_and(a2293, a2294)
                    };
                    let a2296 = 1u8;
                    if equal(a2295, a2296) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2297, a2298)?
            };
            nat_add(a2299, a2300)?
        };
        nat_add(a2301, a2302)?
    };
    let a2304 = 48u64;
    Ok(nat_sub(a2303, a2304))
}

pub fn f510(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2305 = v0.clone();
        length_bytes(a2305)
    };
    let a2306 = v1;
    let a2307 = v2;
    if if nat_lt(a2306, a2307) {
        let a2312 = f115(v0, {
            let a2310 = v2;
            let a2311 = {
                let a2308 = v1;
                let a2309 = 1u64;
                nat_add(a2308, a2309)?
            };
            nat_sub(a2310, a2311)
        })?;
        let a2313 = 48u64;
        nat_eq(a2312, a2313)
    } else {
        false
    } {
        Ok(Some({
            let a2314 = v1;
            let a2315 = 1u64;
            nat_add(a2314, a2315)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f511(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f117(f32({
        let (h2316, _) = v0;
        h2316
    }, {
        let (h2317, _) = v1;
        h2317
    }), f32({
        let (_, h2318) = v0;
        h2318
    }, {
        let (_, h2319) = v1;
        h2319
    }))
}

pub fn f512(v0: Bytes) -> R<bool> {
    let m2320 = f340(v0.clone(), f8(Str::lit("&&")))?;
    if match m2320 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2321 = f340(v0, f8(Str::lit("||")))?;
        match m2321 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f513(v0: Adt18, v1: Option<u64>) -> bool {
    let m2322 = v1;
    match m2322 {
        None => {
            true
        }
        Some(v2) => {
            f451(v2, match v0 {
                Adt18::C0(_, h2323, _, _) => {
                    h2323
                }
            })
        }
    }
}

pub fn f514(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a2325 = match v0 {
            Adt15::C0(_, _, h2324, _, _) => {
                h2324
            }
        };
        length_list(a2325)
    })
}

pub fn f515(v0: Bytes) -> Option<Adt22> {
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

pub fn f516(v0: Bytes, p2330: List<Bytes>, p2331: u64) -> R<Option<u64>> {
    let m2329 = (p2330, p2331);
    {
        let mut m2329 = m2329;
        loop {
            let step = {
                let (v1, v2) = m2329;
                let m2326 = v1;
                match m2326.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2327 = v2;
                                let a2328 = 1u64;
                                nat_add(a2327, a2328)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: Option<Adt2>) -> Option<u64> {
    let m2332 = v0;
    match m2332 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2333 = v1;
            match m2333 {
                Adt2::C0 => {
                    None::<u64>
                }
                Adt2::C1(_, _) => {
                    None::<u64>
                }
                Adt2::C2(_) => {
                    None::<u64>
                }
                Adt2::C3(v7, v8) => {
                    if v7 {
                        None::<u64>
                    } else {
                        Some(v8)
                    }
                }
                Adt2::C4(_, _, _) => {
                    None::<u64>
                }
                Adt2::C5(_, _) => {
                    None::<u64>
                }
                Adt2::C6(_) => {
                    None::<u64>
                }
                Adt2::C7(_) => {
                    None::<u64>
                }
                Adt2::C8(_) => {
                    None::<u64>
                }
                Adt2::C9 => {
                    None::<u64>
                }
                Adt2::C10(_, _) => {
                    None::<u64>
                }
                Adt2::C11(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f518(v0: Fn12, p2337: List<List<(Bytes, ())>>, p2338: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2336 = (p2337, p2338);
    {
        let mut m2336 = m2336;
        loop {
            let step = {
                let (v1, v2) = m2336;
                let m2334 = v2;
                match m2334.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2335: Fn12 = v0.clone();
                            c2335.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2336 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f519(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f520(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2340 = {
        let a2339 = v0.clone();
        length_list(a2339)
    };
    let a2341 = 1u64;
    if nat_le(a2340, a2341) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f569(v0)?))
    }
}

pub fn f521(v0: Fn13, p2345: List<(Bytes, ())>, p2346: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2344 = (p2345, p2346);
    {
        let mut m2344 = m2344;
        loop {
            let step = {
                let (v1, v2) = m2344;
                let m2342 = v2;
                match m2342.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2343: Fn13 = v0.clone();
                            c2343.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f522(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f523(v0: Bytes) -> Bytes {
    f7(List::cons(f139(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f524(v0: Adt1, v1: Bytes, v2: List<Adt2>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f142(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f142(f96(v0, f153())?, f8(Str::lit("at"))));
    let a2348 = {
        let a2347 = v4.clone();
        length_list(a2347)
    };
    let a2349 = 0u64;
    if nat_eq(a2348, a2349) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2352 = {
            let a2350 = v4;
            length_list(a2350)
        };
        let a2353 = {
            let a2351 = v3;
            length_list(a2351)
        };
        if nat_eq(a2352, a2353) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f570(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f525(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt2>) -> R<(bool, Bytes)> {
    let m2354 = v3;
    match m2354.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f202(f67(v0.clone(), f523(v6.clone())), f52(v6.clone(), v2.clone())) {
                f525(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f285(v6.clone()))) {
                    let v9: (bool, Bytes) = f525(v0, v1.clone(), v2, v5)?;
                    let (h2355, _) = v9.clone();
                    if h2355 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2357 = f570(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2358 = {
                                let (_, h2356) = v9;
                                h2356
                            };
                            append_bytes(a2357, a2358)
                        }))
                    }
                } else {
                    if f117(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f483(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2359 = f8(Str::lit("Catalog."));
                        let a2360 = v8.clone();
                        append_bytes(a2359, a2360)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f525(v0, v1.clone(), v2, v5)?;
                        let (h2361, _) = v10.clone();
                        if h2361 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2363 = f570(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2364 = {
                                    let (_, h2362) = v10;
                                    h2362
                                };
                                append_bytes(a2363, a2364)
                            }))
                        }
                    } else {
                        Ok((true, f7(List::cons(f8(Str::lit("admit-registry ")), List::cons(v6, List::cons(f8(Str::lit(" ")), List::cons(v7, List::cons(f8(Str::lit(" ")), List::cons(v8, List::cons(f8(Str::lit(" ")), List::cons(f58(v4.clone(), f8(Str::lit("reader"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v4, f8(Str::lit("artifact"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))))))
                    }
                }
            }
        }
    }
}

pub fn f526(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt2> = f277(f279(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt2> = f277(f279(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f125(f142(v3.clone(), f8(Str::lit("id"))), f142(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f125(v6.clone(), f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2366 = {
        let a2365 = v7.clone();
        length_list(a2365)
    };
    let a2367 = 0u64;
    if nat_eq(a2366, a2367) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2370 = {
                let a2368 = v7;
                length_list(a2368)
            };
            let a2371 = {
                let a2369 = v5;
                length_list(a2369)
            };
            nat_eq(a2370, a2371)
        }) {
            Ok((false, f570(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f145(f125(f62(f571(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f572(v4), f125(v6, f142(v3, f8(Str::lit("id")))))))?;
            let a2373 = {
                let a2372 = v8.clone();
                length_list(a2372)
            };
            let a2374 = 0u64;
            if nat_eq(a2373, a2374) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f570(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f122(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f527(v0: u64) -> bool {
    let a2375 = v0;
    let a2376 = 47u64;
    nat_eq(a2375, a2376)
}

pub fn f528(v0: u64) -> bool {
    f202({
        let a2377 = v0;
        let a2378 = 95u64;
        nat_eq(a2377, a2378)
    }, f202({
        let a2379 = v0;
        let a2380 = 45u64;
        nat_eq(a2379, a2380)
    }, f202({
        let a2381 = v0;
        let a2382 = 32u64;
        nat_eq(a2381, a2382)
    }, f202({
        let a2383 = v0;
        let a2384 = 46u64;
        nat_eq(a2383, a2384)
    }, {
        let a2385 = v0;
        let a2386 = 47u64;
        nat_eq(a2385, a2386)
    }))))
}

pub fn f529(v0: u64) -> bool {
    f117({
        let a2387 = 97u64;
        let a2388 = v0;
        nat_le(a2387, a2388)
    }, {
        let a2389 = v0;
        let a2390 = 122u64;
        nat_le(a2389, a2390)
    })
}

pub fn f530(v0: u64) -> bool {
    f117({
        let a2391 = 65u64;
        let a2392 = v0;
        nat_le(a2391, a2392)
    }, {
        let a2393 = v0;
        let a2394 = 90u64;
        nat_le(a2393, a2394)
    })
}

pub fn f531(v0: u64) -> bool {
    f117({
        let a2395 = 48u64;
        let a2396 = v0;
        nat_le(a2395, a2396)
    }, {
        let a2397 = v0;
        let a2398 = 57u64;
        nat_le(a2397, a2398)
    })
}

pub fn f532(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m2399 = v0;
    match m2399.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f532(v3, v1))
        }
    }
}

pub fn f533(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m2400 = v0;
    match m2400 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f573(v1)
        }
    }
}

pub fn f534(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m2401 = v0;
    if m2401 == 0 {
        let m2402 = v1;
        match m2402.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2401 - 1;
        let m2403 = v1.clone();
        match m2403.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m2404 = f574(v1.clone(), v2.clone(), v3.clone())?;
                match m2404 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f397(f535(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2407 = f534(v6, f575(v1, {
                            let (h2405, _) = v9.clone();
                            h2405
                        }), List::cons(f576({
                            let (h2406, _) = v9.clone();
                            h2406
                        })?, v2), v3)?;
                        match m2407 {
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

pub fn f535(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m2408 = v0;
    match m2408.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f576({
                let (h2409, _) = v1;
                h2409
            })?, f535(v2)?))
        }
    }
}

pub fn f536(p2415: u64, p2416: List<Adt24>, p2417: List<Bytes>, p2418: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2414 = (p2415, (p2416, (p2417, p2418)));
    {
        let mut m2414 = m2414;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2414;
                let m2410 = v0;
                if m2410 == 0 {
                    Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2410 - 1;
                    let m2411 = v1;
                    match m2411.uncons() {
                        None => {
                            Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2412 = v5;
                            match m2412 {
                                Adt24::C0(v8, v9) => {
                                    if f32(v8.clone(), f577()) {
                                        Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f52(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2413 = f579(v8.clone(), v9)?;
                                            match m2413 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f537(f580(v11.clone()), List::cons(Adt24::C1(f581(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f52(f577(), v2.clone()) {
                                        Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f577(), v2), List::cons(f578(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f537(v0: List<Adt24>, v1: List<Adt24>) -> List<Adt24> {
    let m2419 = v0;
    match m2419.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f537(v3, v1))
        }
    }
}

pub fn f538(v0: List<(Bytes, Adt2)>) -> R<List<Adt24>> {
    let m2420 = v0;
    match m2420.uncons() {
        None => {
            Ok(List::<Adt24>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt24::C0(f576({
                let (h2421, _) = v1;
                h2421
            })?, {
                let (_, h2422) = v1;
                h2422
            }), f538(v2)?))
        }
    }
}

pub fn f539(v0: u32) -> Bytes {
    let a2425 = {
        let a2423 = v0;
        let a2424 = 8u32;
        fixed_u32::bit_and(a2423, a2424)
    };
    let a2426 = 0u32;
    if equal(a2425, a2426) {
        let a2429 = {
            let a2427 = v0;
            let a2428 = 4u32;
            fixed_u32::bit_and(a2427, a2428)
        };
        let a2430 = 0u32;
        if equal(a2429, a2430) {
            let a2433 = {
                let a2431 = v0;
                let a2432 = 2u32;
                fixed_u32::bit_and(a2431, a2432)
            };
            let a2434 = 0u32;
            if equal(a2433, a2434) {
                let a2437 = {
                    let a2435 = v0;
                    let a2436 = 1u32;
                    fixed_u32::bit_and(a2435, a2436)
                };
                let a2438 = 0u32;
                if equal(a2437, a2438) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2441 = {
                    let a2439 = v0;
                    let a2440 = 1u32;
                    fixed_u32::bit_and(a2439, a2440)
                };
                let a2442 = 0u32;
                if equal(a2441, a2442) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2445 = {
                let a2443 = v0;
                let a2444 = 2u32;
                fixed_u32::bit_and(a2443, a2444)
            };
            let a2446 = 0u32;
            if equal(a2445, a2446) {
                let a2449 = {
                    let a2447 = v0;
                    let a2448 = 1u32;
                    fixed_u32::bit_and(a2447, a2448)
                };
                let a2450 = 0u32;
                if equal(a2449, a2450) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2453 = {
                    let a2451 = v0;
                    let a2452 = 1u32;
                    fixed_u32::bit_and(a2451, a2452)
                };
                let a2454 = 0u32;
                if equal(a2453, a2454) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2457 = {
            let a2455 = v0;
            let a2456 = 4u32;
            fixed_u32::bit_and(a2455, a2456)
        };
        let a2458 = 0u32;
        if equal(a2457, a2458) {
            let a2461 = {
                let a2459 = v0;
                let a2460 = 2u32;
                fixed_u32::bit_and(a2459, a2460)
            };
            let a2462 = 0u32;
            if equal(a2461, a2462) {
                let a2465 = {
                    let a2463 = v0;
                    let a2464 = 1u32;
                    fixed_u32::bit_and(a2463, a2464)
                };
                let a2466 = 0u32;
                if equal(a2465, a2466) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2469 = {
                    let a2467 = v0;
                    let a2468 = 1u32;
                    fixed_u32::bit_and(a2467, a2468)
                };
                let a2470 = 0u32;
                if equal(a2469, a2470) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2473 = {
                let a2471 = v0;
                let a2472 = 2u32;
                fixed_u32::bit_and(a2471, a2472)
            };
            let a2474 = 0u32;
            if equal(a2473, a2474) {
                let a2477 = {
                    let a2475 = v0;
                    let a2476 = 1u32;
                    fixed_u32::bit_and(a2475, a2476)
                };
                let a2478 = 0u32;
                if equal(a2477, a2478) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2481 = {
                    let a2479 = v0;
                    let a2480 = 1u32;
                    fixed_u32::bit_and(a2479, a2480)
                };
                let a2482 = 0u32;
                if equal(a2481, a2482) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f540(v0: u32, v1: u32) -> u32 {
    let m2485 = {
        let a2483 = v0;
        let a2484 = v1;
        fixed_u32::shift_right(a2483, a2484)
    };
    match m2485 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f541(v0: Adt23, v1: Adt25) -> R<Adt23> {
    let v2: Adt26 = f582(Fn18::F583, 64u64, Adt26::C0(match v0.clone() {
        Adt23::C0(h2486, _, _, _, _, _, _, _) => {
            h2486
        }
    }, match v0.clone() {
        Adt23::C0(_, h2487, _, _, _, _, _, _) => {
            h2487
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h2488, _, _, _, _, _) => {
            h2488
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h2489, _, _, _, _) => {
            h2489
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, h2490, _, _, _) => {
            h2490
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2491, _, _) => {
            h2491
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2492, _) => {
            h2492
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h2493) => {
            h2493
        }
    }, v1, 0u64))?;
    Ok(Adt23::C0(f584(match v0.clone() {
        Adt23::C0(h2494, _, _, _, _, _, _, _) => {
            h2494
        }
    }, match v2.clone() {
        Adt26::C0(h2495, _, _, _, _, _, _, _, _, _) => {
            h2495
        }
    }), f584(match v0.clone() {
        Adt23::C0(_, h2496, _, _, _, _, _, _) => {
            h2496
        }
    }, match v2.clone() {
        Adt26::C0(_, h2497, _, _, _, _, _, _, _, _) => {
            h2497
        }
    }), f584(match v0.clone() {
        Adt23::C0(_, _, h2498, _, _, _, _, _) => {
            h2498
        }
    }, match v2.clone() {
        Adt26::C0(_, _, h2499, _, _, _, _, _, _, _) => {
            h2499
        }
    }), f584(match v0.clone() {
        Adt23::C0(_, _, _, h2500, _, _, _, _) => {
            h2500
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, h2501, _, _, _, _, _, _) => {
            h2501
        }
    }), f584(match v0.clone() {
        Adt23::C0(_, _, _, _, h2502, _, _, _) => {
            h2502
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, h2503, _, _, _, _, _) => {
            h2503
        }
    }), f584(match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2504, _, _) => {
            h2504
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, _, h2505, _, _, _, _) => {
            h2505
        }
    }), f584(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2506, _) => {
            h2506
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, _, _, h2507, _, _, _) => {
            h2507
        }
    }), f584(match v0 {
        Adt23::C0(_, _, _, _, _, _, _, h2508) => {
            h2508
        }
    }, match v2 {
        Adt26::C0(_, _, _, _, _, _, _, h2509, _, _) => {
            h2509
        }
    })))
}

pub fn f542(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt25> {
    Ok(Adt25::C0(f585(v0.clone(), v1, v2, v3)?, f585(v0.clone(), v1, v2, {
        let a2510 = v3;
        let a2511 = 4u64;
        nat_add(a2510, a2511)?
    })?, f585(v0.clone(), v1, v2, {
        let a2512 = v3;
        let a2513 = 8u64;
        nat_add(a2512, a2513)?
    })?, f585(v0.clone(), v1, v2, {
        let a2514 = v3;
        let a2515 = 12u64;
        nat_add(a2514, a2515)?
    })?, f585(v0.clone(), v1, v2, {
        let a2516 = v3;
        let a2517 = 16u64;
        nat_add(a2516, a2517)?
    })?, f585(v0.clone(), v1, v2, {
        let a2518 = v3;
        let a2519 = 20u64;
        nat_add(a2518, a2519)?
    })?, f585(v0.clone(), v1, v2, {
        let a2520 = v3;
        let a2521 = 24u64;
        nat_add(a2520, a2521)?
    })?, f585(v0.clone(), v1, v2, {
        let a2522 = v3;
        let a2523 = 28u64;
        nat_add(a2522, a2523)?
    })?, f585(v0.clone(), v1, v2, {
        let a2524 = v3;
        let a2525 = 32u64;
        nat_add(a2524, a2525)?
    })?, f585(v0.clone(), v1, v2, {
        let a2526 = v3;
        let a2527 = 36u64;
        nat_add(a2526, a2527)?
    })?, f585(v0.clone(), v1, v2, {
        let a2528 = v3;
        let a2529 = 40u64;
        nat_add(a2528, a2529)?
    })?, f585(v0.clone(), v1, v2, {
        let a2530 = v3;
        let a2531 = 44u64;
        nat_add(a2530, a2531)?
    })?, f585(v0.clone(), v1, v2, {
        let a2532 = v3;
        let a2533 = 48u64;
        nat_add(a2532, a2533)?
    })?, f585(v0.clone(), v1, v2, {
        let a2534 = v3;
        let a2535 = 52u64;
        nat_add(a2534, a2535)?
    })?, f585(v0.clone(), v1, v2, {
        let a2536 = v3;
        let a2537 = 56u64;
        nat_add(a2536, a2537)?
    })?, f585(v0, v1, v2, {
        let a2538 = v3;
        let a2539 = 60u64;
        nat_add(a2538, a2539)?
    })?))
}

pub fn f543(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m2540 = f586(v0.clone());
    match m2540 {
        None => {
            let m2541 = f587(v0.clone())?;
            match m2541 {
                None => {
                    Ok(Ok::<Bytes, Bytes>(f189(v0)?))
                }
                Some(v2) => {
                    Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the binary floating-point value ")), List::cons(v2, List::cons(f8(Str::lit(" has no JSON text: numeric values that cannot be represented as a sequence of digits (such as Infinity and NaN) are not permitted (RFC 8259 section 6)")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f544(v0: Bytes, v1: Bytes, v2: Bytes, v3: Adt2) -> Adt2 {
    Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("citation")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f588(List::cons(f8(Str::lit("std")), List::cons(v0, List::cons(v1, List::<Bytes>::nil()))))))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v2)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("pointer")), std::rc::Rc::new(Adt2::C6(Bytes::lit(&[]))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("value")), std::rc::Rc::new(v3), std::rc::Rc::new(Adt2::C9))))))))))
}

pub fn f545(v0: u64, v1: Adt2, v2: Adt2, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2542 = v0;
    if m2542 == 0 {
        Ok(Some(f7(List::cons(f8(Str::lit("the schema's references do not end before the instance at \"")), List::cons(v4, List::cons(f8(Str::lit("\" is checked (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
    } else {
        let v5 = m2542 - 1;
        let m2543 = v2.clone();
        match m2543 {
            Adt2::C0 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C1(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(None::<Bytes>)
                } else {
                    Ok(f589(v4, f8(Str::lit("is refused by the false schema (JSON Schema 2020-12 Core section 4.3.2)"))))
                }
            }
            Adt2::C3(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C4(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C5(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C6(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C7(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C8(r2557) => {
                let v7 = (*r2557).clone();
                let m2544 = f590(f573(v7));
                match m2544 {
                    None => {
                        Ok(f591(f592(v2.clone(), v3.clone(), v4.clone())?, f591({
                            let m2545 = f92(f93(v2.clone(), f8(Str::lit("$ref"))));
                            match m2545 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v9) => {
                                    let m2546 = f593(v1.clone(), v9)?;
                                    match m2546 {
                                        Ok(v11) => {
                                            f545(v5, v1.clone(), v11, v3.clone(), v4.clone())?
                                        }
                                        Err(v10) => {
                                            Some(v10)
                                        }
                                    }
                                }
                            }
                        }, f591(f594(v5, v1.clone(), f187(f93(v2.clone(), f8(Str::lit("allOf")))), v3.clone(), v4.clone())?, f591({
                            let m2547 = f93(v2.clone(), f8(Str::lit("anyOf")));
                            match m2547 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v12) => {
                                    let a2548 = 0u64;
                                    let a2549 = f595(v5, v1.clone(), f187(Some(v12)), v3.clone(), v4.clone())?;
                                    if nat_lt(a2548, a2549) {
                                        None::<Bytes>
                                    } else {
                                        f589(v4.clone(), f8(Str::lit("matches no anyOf branch (JSON Schema 2020-12 Core section 10.2.1.2)")))
                                    }
                                }
                            }
                        }, f591({
                            let m2550 = f93(v2.clone(), f8(Str::lit("oneOf")));
                            match m2550 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v13) => {
                                    let a2551 = f595(v5, v1.clone(), f187(Some(v13)), v3.clone(), v4.clone())?;
                                    let a2552 = 1u64;
                                    if nat_eq(a2551, a2552) {
                                        None::<Bytes>
                                    } else {
                                        f589(v4.clone(), f8(Str::lit("does not match exactly one oneOf branch (JSON Schema 2020-12 Core section 10.2.1.3)")))
                                    }
                                }
                            }
                        }, {
                            let m2553 = v3.clone();
                            match m2553 {
                                Adt2::C0 => {
                                    None::<Bytes>
                                }
                                Adt2::C1(_, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C2(_) => {
                                    None::<Bytes>
                                }
                                Adt2::C3(_, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C4(_, _, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C5(_, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C6(_) => {
                                    None::<Bytes>
                                }
                                Adt2::C7(r2555) => {
                                    let v15 = (*r2555).clone();
                                    let m2554 = f93(v2, f8(Str::lit("items")));
                                    match m2554 {
                                        None => {
                                            None::<Bytes>
                                        }
                                        Some(v16) => {
                                            f598(v5, v1, v16, f304(v15), v4, 0u64)?
                                        }
                                    }
                                }
                                Adt2::C8(r2556) => {
                                    let v14 = (*r2556).clone();
                                    f591(f596(v5, v1.clone(), f533(f93(v2.clone(), f8(Str::lit("properties")))), v3, v4.clone())?, f597(v5, v1, v2, f573(v14), v4)?)
                                }
                                Adt2::C9 => {
                                    None::<Bytes>
                                }
                                Adt2::C10(_, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C11(_, _, _) => {
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
            Adt2::C9 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C10(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C11(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
        }
    }
}

pub fn f546(v0: Adt2) -> R<Bytes> {
    f189(v0)
}

pub fn f547(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2558 = v0;
    match m2558.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f358(v1)?, f547(v2)?))
        }
    }
}

pub fn f548(p2561: List<Bytes>, v1: Bytes) -> bool {
    let m2560 = p2561;
    {
        let mut m2560 = m2560;
        loop {
            let step = {
                let v0 = m2560;
                let m2559 = v0;
                match m2559.uncons() {
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
                Ok(next) => { m2560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f549(v0: List<Adt2>, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m2562 = v0;
    match m2562.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v3, v4)) => {
            if f32(f58(v3.clone(), v1.clone()), v2.clone()) {
                f549(v4, v1, v2)
            } else {
                List::cons(v3, f549(v4, v1, v2))
            }
        }
    }
}

pub fn f550(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2563 = v0;
    match m2563.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f397(f547(f145(List::cons(v1, f599(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f550(v3, v1)?))
            }
        }
    }
}

pub fn f551(v0: Bytes, v1: u64) -> Bytes {
    let m2567 = {
        let a2564 = v0;
        let a2565 = v1;
        let a2566 = 1u64;
        slice_bytes(a2564, a2565, a2566)
    };
    match m2567 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f552(v0: Fn14, p2572: u64, p2573: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m2571 = (p2572, p2573);
    {
        let mut m2571 = m2571;
        loop {
            let step = {
                let (v1, v2) = m2571;
                let m2568 = v1;
                if m2568 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2568 - 1;
                    let m2570 = {
                        let c2569: Fn14 = v0.clone();
                        c2569.apply(v2.clone())?
                    };
                    match m2570 {
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
                Ok(next) => { m2571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f553(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f600(v1, v0, v2)
}

pub fn f554(p2576: List<u64>, p2577: List<u64>) -> List<u64> {
    let m2575 = (p2576, p2577);
    {
        let mut m2575 = m2575;
        loop {
            let step = {
                let (v0, v1) = m2575;
                let m2574 = v0;
                match m2574.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f555(v0: List<u64>) -> R<Bytes> {
    let m2578 = v0;
    match m2578.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a2579 = f601(v1)?;
            let a2580 = f555(v2)?;
            Ok(append_bytes(a2579, a2580))
        }
    }
}

pub fn f556(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m2581 = v0;
    match m2581.uncons() {
        None => {
            let a2582 = v2;
            let a2583 = 0u64;
            if nat_eq(a2582, a2583) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a2586 = {
                    let a2584 = v3;
                    let a2585 = v1;
                    nat_mul(a2584, a2585)?
                };
                let a2587 = v2;
                nat_add(a2586, a2587)?
            };
            Ok(List::cons({
                let a2588 = v5;
                let a2589 = f557();
                let a2590 = 0u64;
                nat_rem(a2588, a2589, a2590)
            }, f556(v4, v1, {
                let a2591 = v5;
                let a2592 = f557();
                let a2593 = 0u64;
                nat_quot(a2591, a2592, a2593)
            })?))
        }
    }
}

pub fn f557() -> u64 {
    1000000000u64
}

pub fn f558(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2638 = {
            let a2614 = {
                let a2602 = {
                    let a2596 = {
                        let a2594 = v1;
                        let a2595 = 128u8;
                        fixed_u8::bit_and(a2594, a2595)
                    };
                    let a2597 = 128u8;
                    if equal(a2596, a2597) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2603 = {
                    let a2600 = {
                        let a2598 = v1;
                        let a2599 = 64u8;
                        fixed_u8::bit_and(a2598, a2599)
                    };
                    let a2601 = 64u8;
                    if equal(a2600, a2601) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2602, a2603)?
            };
            let a2615 = {
                let a2612 = {
                    let a2606 = {
                        let a2604 = v1;
                        let a2605 = 32u8;
                        fixed_u8::bit_and(a2604, a2605)
                    };
                    let a2607 = 32u8;
                    if equal(a2606, a2607) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2613 = {
                    let a2610 = {
                        let a2608 = v1;
                        let a2609 = 16u8;
                        fixed_u8::bit_and(a2608, a2609)
                    };
                    let a2611 = 16u8;
                    if equal(a2610, a2611) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2612, a2613)?
            };
            nat_add(a2614, a2615)?
        };
        let a2639 = {
            let a2636 = {
                let a2624 = {
                    let a2618 = {
                        let a2616 = v1;
                        let a2617 = 8u8;
                        fixed_u8::bit_and(a2616, a2617)
                    };
                    let a2619 = 8u8;
                    if equal(a2618, a2619) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2625 = {
                    let a2622 = {
                        let a2620 = v1;
                        let a2621 = 4u8;
                        fixed_u8::bit_and(a2620, a2621)
                    };
                    let a2623 = 4u8;
                    if equal(a2622, a2623) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2624, a2625)?
            };
            let a2637 = {
                let a2634 = {
                    let a2628 = {
                        let a2626 = v1;
                        let a2627 = 2u8;
                        fixed_u8::bit_and(a2626, a2627)
                    };
                    let a2629 = 2u8;
                    if equal(a2628, a2629) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2635 = {
                    let a2632 = {
                        let a2630 = v1;
                        let a2631 = 1u8;
                        fixed_u8::bit_and(a2630, a2631)
                    };
                    let a2633 = 1u8;
                    if equal(a2632, a2633) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2634, a2635)?
            };
            nat_add(a2636, a2637)?
        };
        nat_add(a2638, a2639)?
    };
    let a2646 = f551(f602(), {
        let a2640 = v2;
        let a2641 = 16u64;
        let a2642 = 0u64;
        nat_quot(a2640, a2641, a2642)
    });
    let a2647 = f551(f602(), {
        let a2643 = v2;
        let a2644 = 16u64;
        let a2645 = 0u64;
        nat_rem(a2643, a2644, a2645)
    });
    Ok(append_bytes(a2646, a2647))
}

pub fn f559(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f603(Fn19::F604, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f560(v0: Fn15, p2651: List<List<(Bytes, Bytes)>>, p2652: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2650 = (p2651, p2652);
    {
        let mut m2650 = m2650;
        loop {
            let step = {
                let (v1, v2) = m2650;
                let m2648 = v2;
                match m2648.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2649: Fn15 = v0.clone();
                            c2649.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f562(v0: Fn16, p2657: u64, p2658: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2656 = (p2657, p2658);
    {
        let mut m2656 = m2656;
        loop {
            let step = {
                let (v1, v2) = m2656;
                let m2653 = v1;
                if m2653 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2653 - 1;
                    let m2655 = {
                        let c2654: Fn16 = v0.clone();
                        c2654.apply(v2.clone())?
                    };
                    match m2655 {
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
                Ok(next) => { m2656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f563(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f605(v0)
}

pub fn f564(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f606(v0, v1)
}

pub fn f565(v0: u8) -> bool {
    let a2659 = v0;
    let a2660 = 32u8;
    if if if equal(a2659, a2660) {
        true
    } else {
        let a2661 = v0;
        let a2662 = 9u8;
        equal(a2661, a2662)
    } {
        true
    } else {
        let a2663 = v0;
        let a2664 = 10u8;
        equal(a2663, a2664)
    } {
        true
    } else {
        let a2665 = v0;
        let a2666 = 12u8;
        if equal(a2665, a2666) {
            true
        } else {
            let a2667 = v0;
            let a2668 = 13u8;
            equal(a2667, a2668)
        }
    }
}

pub fn f566(v0: Fn17, p2673: u64, p2674: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2672 = (p2673, p2674);
    {
        let mut m2672 = m2672;
        loop {
            let step = {
                let (v1, v2) = m2672;
                let m2669 = v1;
                if m2669 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2669 - 1;
                    let m2671 = {
                        let c2670: Fn17 = v0.clone();
                        c2670.apply(v2.clone())?
                    };
                    match m2671 {
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
                Ok(next) => { m2672 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f567(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f607(v0, v1)
}

pub fn f568(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2719 = {
            let a2695 = {
                let a2683 = {
                    let a2677 = {
                        let a2675 = v1;
                        let a2676 = 128u8;
                        fixed_u8::bit_and(a2675, a2676)
                    };
                    let a2678 = 128u8;
                    if equal(a2677, a2678) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2684 = {
                    let a2681 = {
                        let a2679 = v1;
                        let a2680 = 64u8;
                        fixed_u8::bit_and(a2679, a2680)
                    };
                    let a2682 = 64u8;
                    if equal(a2681, a2682) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2683, a2684)?
            };
            let a2696 = {
                let a2693 = {
                    let a2687 = {
                        let a2685 = v1;
                        let a2686 = 32u8;
                        fixed_u8::bit_and(a2685, a2686)
                    };
                    let a2688 = 32u8;
                    if equal(a2687, a2688) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2694 = {
                    let a2691 = {
                        let a2689 = v1;
                        let a2690 = 16u8;
                        fixed_u8::bit_and(a2689, a2690)
                    };
                    let a2692 = 16u8;
                    if equal(a2691, a2692) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2693, a2694)?
            };
            nat_add(a2695, a2696)?
        };
        let a2720 = {
            let a2717 = {
                let a2705 = {
                    let a2699 = {
                        let a2697 = v1;
                        let a2698 = 8u8;
                        fixed_u8::bit_and(a2697, a2698)
                    };
                    let a2700 = 8u8;
                    if equal(a2699, a2700) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2706 = {
                    let a2703 = {
                        let a2701 = v1;
                        let a2702 = 4u8;
                        fixed_u8::bit_and(a2701, a2702)
                    };
                    let a2704 = 4u8;
                    if equal(a2703, a2704) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2705, a2706)?
            };
            let a2718 = {
                let a2715 = {
                    let a2709 = {
                        let a2707 = v1;
                        let a2708 = 2u8;
                        fixed_u8::bit_and(a2707, a2708)
                    };
                    let a2710 = 2u8;
                    if equal(a2709, a2710) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2716 = {
                    let a2713 = {
                        let a2711 = v1;
                        let a2712 = 1u8;
                        fixed_u8::bit_and(a2711, a2712)
                    };
                    let a2714 = 1u8;
                    if equal(a2713, a2714) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2715, a2716)?
            };
            nat_add(a2717, a2718)?
        };
        nat_add(a2719, a2720)?
    };
    let a2721 = 48u64;
    let a2722 = v2;
    if nat_le(a2721, a2722) {
        let a2723 = v2;
        let a2724 = 57u64;
        Ok(nat_le(a2723, a2724))
    } else {
        Ok(false)
    }
}

pub fn f569(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f454({
        let (_, h2725) = {
            let (h2726, _) = f608(Fn20::F609, {
                let a2728 = {
                    let a2727 = v0.clone();
                    length_list(a2727)
                };
                let a2729 = 1u64;
                nat_add(a2728, a2729)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2726
        };
        h2725
    }))
}

pub fn f570(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f571(v0: List<Adt2>) -> List<Bytes> {
    let m2730 = v0;
    match m2730.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2731 = f93(v1, f8(Str::lit("std")));
            match m2731 {
                None => {
                    f571(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f571(v2))
                }
            }
        }
    }
}

pub fn f572(v0: List<Adt2>) -> List<Bytes> {
    let m2732 = v0;
    match m2732.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2733 = f92(f93(v1, f8(Str::lit("req"))));
            match m2733 {
                None => {
                    f572(v2)
                }
                Some(v3) => {
                    List::cons(v3, f572(v2))
                }
            }
        }
    }
}

pub fn f573(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m2734 = v0;
    match m2734 {
        Adt2::C0 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C1(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C2(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C3(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C4(_, _, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C5(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C6(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C7(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C8(r2735) => {
            let v4 = (*r2735).clone();
            f573(v4)
        }
        Adt2::C9 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C10(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C11(v1, r2736, r2737) => {
            let v2 = (*r2736).clone();
            let v3 = (*r2737).clone();
            List::cons((v1, v2), f573(v3))
        }
    }
}

pub fn f574(p2742: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m2741 = p2742;
    {
        let mut m2741 = m2741;
        loop {
            let step = {
                let v0 = m2741;
                let m2738 = v0;
                match m2738.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f610(f611({
                            let (_, h2739) = v3.clone();
                            h2739
                        }, List::<Bytes>::nil())?, f576({
                            let (h2740, _) = v3.clone();
                            h2740
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f575(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m2743 = v0;
    match m2743.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h2744, _) = v2.clone();
                h2744
            }, v1.clone()) {
                f575(v3, v1)
            } else {
                List::cons(v2, f575(v3, v1))
            }
        }
    }
}

pub fn f576(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f384(v0)?;
    if f52(v1.clone(), f612()) {
        f613(v1)
    } else {
        Ok(v1)
    }
}

pub fn f577() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f578() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f579(v0: Bytes, v1: Adt2) -> R<Result<Adt27, Bytes>> {
    f614(v0, f615(v1))
}

pub fn f580(v0: Adt27) -> List<Adt24> {
    let m2745 = v0;
    match m2745 {
        Adt27::C0(_, v2) => {
            v2
        }
    }
}

pub fn f581(v0: Adt27) -> Bytes {
    let m2746 = v0;
    match m2746 {
        Adt27::C0(v1, _) => {
            v1
        }
    }
}

pub fn f582(v0: Fn18, p2750: u64, p2751: Adt26) -> R<Adt26> {
    let m2749 = (p2750, p2751);
    {
        let mut m2749 = m2749;
        loop {
            let step = {
                let (v1, v2) = m2749;
                let m2747 = v1;
                if m2747 == 0 {
                    Err::<(u64, Adt26), R<Adt26>>(Ok(v2))
                } else {
                    let v3 = m2747 - 1;
                    Ok::<(u64, Adt26), R<Adt26>>((v3, {
                        let c2748: Fn18 = v0.clone();
                        c2748.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2749 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: Adt26) -> R<Adt26> {
    f616(v0)
}

pub fn f584(v0: u32, v1: u32) -> u32 {
    let m2754 = {
        let a2752 = f617(v0);
        let a2753 = f617(v1);
        fixed_u64::checked_add(a2752, a2753)
    };
    match m2754 {
        None => {
            0u32
        }
        Some(v2) => {
            f618(v2)
        }
    }
}

pub fn f585(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2757 = {
        let a2755 = v3;
        let a2756 = 8u64;
        nat_add(a2755, a2756)?
    };
    let a2758 = v2;
    if nat_eq(a2757, a2758) {
        Ok(f619({
            let a2761 = {
                let a2759 = v1;
                let a2760 = 8u64;
                nat_mul(a2759, a2760)?
            };
            let a2762 = 4294967296u64;
            let a2763 = 0u64;
            nat_quot(a2761, a2762, a2763)
        }))
    } else {
        let a2766 = {
            let a2764 = v3;
            let a2765 = 4u64;
            nat_add(a2764, a2765)?
        };
        let a2767 = v2;
        if nat_eq(a2766, a2767) {
            Ok(f619({
                let a2770 = {
                    let a2768 = v1;
                    let a2769 = 8u64;
                    nat_mul(a2768, a2769)?
                };
                let a2771 = 4294967296u64;
                let a2772 = 0u64;
                nat_rem(a2770, a2771, a2772)
            }))
        } else {
            Ok(f620(f621(v0.clone(), v1, v3), f621(v0.clone(), v1, {
                let a2773 = v3;
                let a2774 = 1u64;
                nat_add(a2773, a2774)?
            }), f621(v0.clone(), v1, {
                let a2775 = v3;
                let a2776 = 2u64;
                nat_add(a2775, a2776)?
            }), f621(v0, v1, {
                let a2777 = v3;
                let a2778 = 3u64;
                nat_add(a2777, a2778)?
            })))
        }
    }
}

pub fn f586(v0: Adt2) -> Option<Bytes> {
    let m2779 = v0;
    match m2779 {
        Adt2::C0 => {
            None::<Bytes>
        }
        Adt2::C1(v1, _) => {
            Some(v1)
        }
        Adt2::C2(_) => {
            None::<Bytes>
        }
        Adt2::C3(_, _) => {
            None::<Bytes>
        }
        Adt2::C4(_, _, _) => {
            None::<Bytes>
        }
        Adt2::C5(_, _) => {
            None::<Bytes>
        }
        Adt2::C6(_) => {
            None::<Bytes>
        }
        Adt2::C7(r2782) => {
            let v3 = (*r2782).clone();
            f586(v3)
        }
        Adt2::C8(r2783) => {
            let v4 = (*r2783).clone();
            f586(v4)
        }
        Adt2::C9 => {
            None::<Bytes>
        }
        Adt2::C10(r2784, r2785) => {
            let v5 = (*r2784).clone();
            let v6 = (*r2785).clone();
            let m2780 = f586(v5);
            match m2780 {
                None => {
                    f586(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C11(_, r2786, r2787) => {
            let v9 = (*r2786).clone();
            let v10 = (*r2787).clone();
            let m2781 = f586(v9);
            match m2781 {
                None => {
                    f586(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f587(v0: Adt2) -> R<Option<Bytes>> {
    let m2788 = v0;
    match m2788 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, r2791) => {
            let v2 = (*r2791).clone();
            f587(v2)
        }
        Adt2::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C3(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C4(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C5(v18, v19) => {
            if f622(v18, v19)? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f308(v18, v19)?))
            }
        }
        Adt2::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(r2792) => {
            let v3 = (*r2792).clone();
            f587(v3)
        }
        Adt2::C8(r2793) => {
            let v4 = (*r2793).clone();
            f587(v4)
        }
        Adt2::C9 => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(r2794, r2795) => {
            let v5 = (*r2794).clone();
            let v6 = (*r2795).clone();
            let m2789 = f587(v5)?;
            match m2789 {
                None => {
                    f587(v6)
                }
                Some(v7) => {
                    Ok(Some(v7))
                }
            }
        }
        Adt2::C11(_, r2796, r2797) => {
            let v9 = (*r2796).clone();
            let v10 = (*r2797).clone();
            let m2790 = f587(v9)?;
            match m2790 {
                None => {
                    f587(v10)
                }
                Some(v11) => {
                    Ok(Some(v11))
                }
            }
        }
    }
}

pub fn f588(v0: List<Bytes>) -> Adt2 {
    let m2798 = v0;
    match m2798.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f588(v2)))
        }
    }
}

pub fn f589(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    Some(f7(List::cons(f8(Str::lit("the instance at \"")), List::cons(v0, List::cons(f8(Str::lit("\" ")), List::cons(v1, List::<Bytes>::nil()))))))
}

pub fn f590(p2803: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m2802 = p2803;
    {
        let mut m2802 = m2802;
        loop {
            let step = {
                let v0 = m2802;
                let m2799 = v0;
                match m2799.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h2800, _) = v1.clone();
                            h2800
                        }, f623()) {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h2801, _) = v1;
                                h2801
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2802 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f591(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2804 = v0;
    match m2804 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f592(v0: Adt2, v1: Adt2, v2: Bytes) -> R<Option<Bytes>> {
    let v3: Bytes = f624(v1.clone());
    let v7: Option<Bytes> = {
        let m2805 = f93(v0.clone(), f8(Str::lit("type")));
        match m2805 {
            None => {
                None::<Bytes>
            }
            Some(v4) => {
                let v6: List<Bytes> = {
                    let m2806 = f92(Some(v4.clone()));
                    match m2806 {
                        None => {
                            f214(f187(Some(v4)))
                        }
                        Some(v5) => {
                            List::cons(v5, List::<Bytes>::nil())
                        }
                    }
                };
                if f625(v6.clone(), v1.clone()) {
                    None::<Bytes>
                } else {
                    f589(v2.clone(), f7(List::cons(f8(Str::lit("is ")), List::cons(v3, List::cons(f8(Str::lit(", not of the type ")), List::cons(f397(v6, f8(Str::lit(" or "))), List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.1.1)")), List::<Bytes>::nil())))))))
                }
            }
        }
    };
    let v9: Option<Bytes> = {
        let m2807 = f93(v0.clone(), f8(Str::lit("enum")));
        match m2807 {
            None => {
                None::<Bytes>
            }
            Some(v8) => {
                if f626(f187(Some(v8)), v1.clone())? {
                    None::<Bytes>
                } else {
                    f589(v2.clone(), f8(Str::lit("is none of the enum values (JSON Schema 2020-12 Validation section 6.1.2)")))
                }
            }
        }
    };
    let v11: Option<Bytes> = {
        let m2808 = f93(v0.clone(), f8(Str::lit("const")));
        match m2808 {
            None => {
                None::<Bytes>
            }
            Some(v10) => {
                if f627(false, v10, v1.clone())? {
                    None::<Bytes>
                } else {
                    f589(v2.clone(), f8(Str::lit("is not the const value (JSON Schema 2020-12 Validation section 6.1.3)")))
                }
            }
        }
    };
    let v45: Option<Bytes> = {
        let m2809 = v1.clone();
        match m2809 {
            Adt2::C0 => {
                None::<Bytes>
            }
            Adt2::C1(v28, _) => {
                f589(v2.clone(), f7(List::cons(f8(Str::lit("is tagged ")), List::cons(v28, List::cons(f8(Str::lit(" and has no JSON type (RFC 8259 section 3)")), List::<Bytes>::nil())))))
            }
            Adt2::C2(_) => {
                None::<Bytes>
            }
            Adt2::C3(_, _) => {
                None::<Bytes>
            }
            Adt2::C4(_, _, _) => {
                None::<Bytes>
            }
            Adt2::C5(v36, v37) => {
                if f622(v36, v37)? {
                    let m2826 = f93(v0.clone(), f8(Str::lit("minimum")));
                    match m2826 {
                        None => {
                            let m2827 = f93(v0.clone(), f8(Str::lit("maximum")));
                            match m2827 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(_) => {
                                    f589(v2.clone(), f8(Str::lit("is a binary floating-point number, which maximum compares only as an exact decimal (JSON Schema 2020-12 Validation section 6.2.2)")))
                                }
                            }
                        }
                        Some(_) => {
                            f589(v2.clone(), f8(Str::lit("is a binary floating-point number, which minimum compares only as an exact decimal (JSON Schema 2020-12 Validation section 6.2.4)")))
                        }
                    }
                } else {
                    f589(v2.clone(), f8(Str::lit("is NaN or an infinity, which is not a JSON number (RFC 8259 section 6)")))
                }
            }
            Adt2::C6(v12) => {
                let m2810 = f628(v12.clone())?;
                match m2810 {
                    None => {
                        f589(v2.clone(), f8(Str::lit("is not well-formed UTF-8 (RFC 3629 section 4)")))
                    }
                    Some(v13) => {
                        let v14: u64 = f629(v13)?;
                        f591({
                            let m2811 = f517(f93(v0.clone(), f8(Str::lit("minLength"))));
                            match m2811 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v15) => {
                                    let a2812 = v14;
                                    let a2813 = v15;
                                    if nat_lt(a2812, a2813) {
                                        f589(v2.clone(), f8(Str::lit("is shorter than minLength (JSON Schema 2020-12 Validation section 6.3.2)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, f591({
                            let m2814 = f517(f93(v0.clone(), f8(Str::lit("maxLength"))));
                            match m2814 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v16) => {
                                    let a2815 = v16;
                                    let a2816 = v14;
                                    if nat_lt(a2815, a2816) {
                                        f589(v2.clone(), f8(Str::lit("is longer than maxLength (JSON Schema 2020-12 Validation section 6.3.1)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, {
                            let m2817 = f92(f93(v0.clone(), f8(Str::lit("pattern"))));
                            match m2817 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v17) => {
                                    let m2818 = f630(v17.clone(), v12)?;
                                    match m2818 {
                                        Ok(v19) => {
                                            if v19 {
                                                None::<Bytes>
                                            } else {
                                                f589(v2.clone(), f7(List::cons(f8(Str::lit("does not match the pattern ")), List::cons(v17, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.3.3)")), List::<Bytes>::nil())))))
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
            Adt2::C7(r2828) => {
                let v20 = (*r2828).clone();
                let v21: u64 = f631(v20)?;
                f591({
                    let m2819 = f517(f93(v0.clone(), f8(Str::lit("minItems"))));
                    match m2819 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v22) => {
                            let a2820 = v21;
                            let a2821 = v22;
                            if nat_lt(a2820, a2821) {
                                f589(v2.clone(), f8(Str::lit("has fewer items than minItems (JSON Schema 2020-12 Validation section 6.4.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2822 = f517(f93(v0.clone(), f8(Str::lit("maxItems"))));
                    match m2822 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v23) => {
                            let a2823 = v23;
                            let a2824 = v21;
                            if nat_lt(a2823, a2824) {
                                f589(v2.clone(), f8(Str::lit("has more items than maxItems (JSON Schema 2020-12 Validation section 6.4.1)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
            Adt2::C8(r2829) => {
                let v24 = (*r2829).clone();
                let v25: List<Bytes> = f632(f214(f187(f93(v0.clone(), f8(Str::lit("required"))))), f633(f573(v24)));
                let m2825 = v25;
                match m2825.uncons() {
                    None => {
                        None::<Bytes>
                    }
                    Some((v26, _)) => {
                        f589(v2.clone(), f7(List::cons(f8(Str::lit("lacks the required member ")), List::cons(v26, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.5.3)")), List::<Bytes>::nil())))))
                    }
                }
            }
            Adt2::C9 => {
                None::<Bytes>
            }
            Adt2::C10(_, _) => {
                None::<Bytes>
            }
            Adt2::C11(_, _, _) => {
                None::<Bytes>
            }
        }
    };
    let v51: Option<Bytes> = {
        let m2830 = f634(v1);
        match m2830 {
            None => {
                None::<Bytes>
            }
            Some(v46) => {
                f591({
                    let m2832 = {
                        let m2831 = f93(v0.clone(), f8(Str::lit("minimum")));
                        match m2831 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v47) => {
                                f634(v47)
                            }
                        }
                    };
                    match m2832 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v48) => {
                            if f635(v46, v48)? {
                                f589(v2.clone(), f8(Str::lit("is less than minimum (JSON Schema 2020-12 Validation section 6.2.4)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2834 = {
                        let m2833 = f93(v0, f8(Str::lit("maximum")));
                        match m2833 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v49) => {
                                f634(v49)
                            }
                        }
                    };
                    match m2834 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v50) => {
                            if f635(v50, v46)? {
                                f589(v2, f8(Str::lit("is greater than maximum (JSON Schema 2020-12 Validation section 6.2.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
        }
    };
    Ok(f591(v7, f591(v9, f591(v11, f591(v45, v51)))))
}

pub fn f593(v0: Adt2, v1: Bytes) -> R<Result<Adt2, Bytes>> {
    let v2: u64 = {
        let a2835 = v1.clone();
        length_bytes(a2835)
    };
    if f32(v1.clone(), f8(Str::lit("#"))) {
        Ok(Ok::<Adt2, Bytes>(v0))
    } else {
        let m2840 = if f85(v1.clone(), f8(Str::lit("#/$defs/"))) {
            Some((f8(Str::lit("$defs")), f46(v1.clone(), 8u64, {
                let a2836 = v2;
                let a2837 = 8u64;
                nat_sub(a2836, a2837)
            })))
        } else {
            if f85(v1.clone(), f8(Str::lit("#/definitions/"))) {
                Some((f8(Str::lit("definitions")), f46(v1.clone(), 14u64, {
                    let a2838 = v2;
                    let a2839 = 14u64;
                    nat_sub(a2838, a2839)
                })))
            } else {
                None::<(Bytes, Bytes)>
            }
        };
        match m2840 {
            None => {
                Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" is not resolved: only #, #/$defs/NAME and #/definitions/NAME in the same document are (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
            }
            Some(v3) => {
                let v4: Bytes = f636({
                    let a2843 = {
                        let a2842 = {
                            let (_, h2841) = v3.clone();
                            h2841
                        };
                        length_bytes(a2842)
                    };
                    let a2844 = 1u64;
                    nat_add(a2843, a2844)?
                }, {
                    let (_, h2845) = v3;
                    h2845
                }, 0u64, Bytes::lit(&[]))?;
                let m2848 = {
                    let m2847 = f93(v0, {
                        let (h2846, _) = v3;
                        h2846
                    });
                    match m2847 {
                        None => {
                            None::<Adt2>
                        }
                        Some(v5) => {
                            f93(v5, v4)
                        }
                    }
                };
                match m2848 {
                    None => {
                        Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" names no definition of this schema (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok(Ok::<Adt2, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f594(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2849 = v0;
    if m2849 == 0 {
        Ok(Some(f8(Str::lit("the allOf branches do not end (JSON Schema 2020-12 Core section 10.2.1.1)"))))
    } else {
        let v5 = m2849 - 1;
        let m2850 = v2;
        match m2850.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f591(f545(v5, v1.clone(), v6, v3.clone(), v4.clone())?, f594(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f595(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<u64> {
    let m2851 = v0;
    if m2851 == 0 {
        Ok(0u64)
    } else {
        let v5 = m2851 - 1;
        let m2852 = v2;
        match m2852.uncons() {
            None => {
                Ok(0u64)
            }
            Some((v6, v7)) => {
                let a2854 = {
                    let m2853 = f545(v5, v1.clone(), v6, v3.clone(), v4.clone())?;
                    match m2853 {
                        None => {
                            1u64
                        }
                        Some(_) => {
                            0u64
                        }
                    }
                };
                let a2855 = f595(v5, v1, v7, v3, v4)?;
                nat_add(a2854, a2855)
            }
        }
    }
}

pub fn f596(v0: u64, v1: Adt2, v2: List<(Bytes, Adt2)>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2856 = v0;
    if m2856 == 0 {
        Ok(Some(f8(Str::lit("the properties do not end (JSON Schema 2020-12 Core section 10.3.2.1)"))))
    } else {
        let v5 = m2856 - 1;
        let m2857 = v2;
        match m2857.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f591({
                    let m2859 = f93(v3.clone(), {
                        let (h2858, _) = v6.clone();
                        h2858
                    });
                    match m2859 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v8) => {
                            f545(v5, v1.clone(), {
                                let (_, h2860) = v6;
                                h2860
                            }, v8, f637(v4.clone(), {
                                let (h2861, _) = v6;
                                h2861
                            })?)?
                        }
                    }
                }, f596(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f597(v0: u64, v1: Adt2, v2: Adt2, v3: List<(Bytes, Adt2)>, v4: Bytes) -> R<Option<Bytes>> {
    let m2862 = v0;
    if m2862 == 0 {
        Ok(Some(f8(Str::lit("the members do not end (JSON Schema 2020-12 Core section 10.3.2.2)"))))
    } else {
        let v5 = m2862 - 1;
        let m2863 = v3;
        match m2863.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                let v8: Bytes = f637(v4.clone(), {
                    let (h2864, _) = v6.clone();
                    h2864
                })?;
                let m2866 = f638(f533(f93(v2.clone(), f8(Str::lit("patternProperties")))), {
                    let (h2865, _) = v6.clone();
                    h2865
                })?;
                match m2866 {
                    Ok(v10) => {
                        Ok(f591(f594(v5, v1.clone(), v10.clone(), {
                            let (_, h2867) = v6.clone();
                            h2867
                        }, v8.clone())?, f591({
                            let m2868 = v10;
                            match m2868.uncons() {
                                None => {
                                    if f52({
                                        let (h2869, _) = v6;
                                        h2869
                                    }, f633(f573({
                                        let m2870 = f93(v2.clone(), f8(Str::lit("properties")));
                                        match m2870 {
                                            None => {
                                                Adt2::C8(std::rc::Rc::new(Adt2::C9))
                                            }
                                            Some(v13) => {
                                                v13
                                            }
                                        }
                                    }))) {
                                        None::<Bytes>
                                    } else {
                                        let m2871 = f93(v2.clone(), f8(Str::lit("additionalProperties")));
                                        match m2871 {
                                            None => {
                                                None::<Bytes>
                                            }
                                            Some(v14) => {
                                                f545(v5, v1.clone(), v14, {
                                                    let (_, h2872) = v6;
                                                    h2872
                                                }, v8)?
                                            }
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    None::<Bytes>
                                }
                            }
                        }, f597(v5, v1, v2, v7, v4)?)))
                    }
                    Err(v9) => {
                        Ok(Some(v9))
                    }
                }
            }
        }
    }
}

pub fn f598(v0: u64, v1: Adt2, v2: Adt2, v3: List<Adt2>, v4: Bytes, v5: u64) -> R<Option<Bytes>> {
    let m2873 = v0;
    if m2873 == 0 {
        Ok(Some(f8(Str::lit("the items do not end (JSON Schema 2020-12 Core section 10.3.1.2)"))))
    } else {
        let v6 = m2873 - 1;
        let m2874 = v3;
        match m2874.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v7, v8)) => {
                Ok(f591(f545(v6, v1.clone(), v2.clone(), v7, f637(v4.clone(), f406(v5)?)?)?, f598(v6, v1, v2, v8, v4, {
                    let a2875 = v5;
                    let a2876 = 1u64;
                    nat_add(a2875, a2876)?
                })?))
            }
        }
    }
}

pub fn f599(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2877 = v0;
    match m2877.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f599(v3, false))
            } else {
                f599(v3, true)
            }
        }
    }
}

pub fn f600(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2878, _) = v2;
        h2878
    };
    let a2879 = v3;
    let a2880 = 0u64;
    if nat_eq(a2879, a2880) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2881 = v3;
            let a2882 = v1;
            let a2883 = 0u64;
            nat_quot(a2881, a2882, a2883)
        }, {
            let a2888 = f551(v0, {
                let a2884 = v3;
                let a2885 = v1;
                let a2886 = 0u64;
                nat_rem(a2884, a2885, a2886)
            });
            let a2889 = {
                let (_, h2887) = v2;
                h2887
            };
            append_bytes(a2888, a2889)
        }))
    }
}

pub fn f601(v0: u64) -> R<Bytes> {
    let v1: Bytes = f406(v0)?;
    let a2893 = f499({
        let a2891 = 9u64;
        let a2892 = {
            let a2890 = v1.clone();
            length_bytes(a2890)
        };
        nat_sub(a2891, a2892)
    });
    let a2894 = v1;
    Ok(append_bytes(a2893, a2894))
}

pub fn f602() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f603(v0: Fn19, p2898: List<List<(Bytes, Bytes)>>, p2899: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2897 = (p2898, p2899);
    {
        let mut m2897 = m2897;
        loop {
            let step = {
                let (v1, v2) = m2897;
                let m2895 = v2;
                match m2895.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2896: Fn19 = v0.clone();
                            c2896.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2897 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f605(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2901 = {
        let a2900 = v0.clone();
        length_list(a2900)
    };
    let a2902 = 1u64;
    if nat_le(a2901, a2902) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f639(v0)?))
    }
}

pub fn f606(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2906 = {
        let a2904 = v0.clone();
        let a2905 = {
            let (h2903, _) = v1.clone();
            h2903
        };
        index_bytes(a2904, a2905)
    };
    match m2906 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2908 = {
                    let (h2907, _) = v1.clone();
                    h2907
                };
                let a2909 = 1u64;
                nat_add(a2908, a2909)?
            }, {
                let a2912 = {
                    let (_, h2910) = v1;
                    h2910
                };
                let a2913 = f500(v0, {
                    let (h2911, _) = v1;
                    h2911
                }, v2)?;
                append_bytes(a2912, a2913)
            })))
        }
    }
}

pub fn f607(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2914, _) = v1;
        h2914
    };
    let m2916 = {
        let (_, h2915) = v1;
        h2915
    };
    match m2916 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2919 = {
                let a2917 = v0.clone();
                let a2918 = v2;
                index_bytes(a2917, a2918)
            };
            match m2919 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2924 = {
                        let a2922 = v0;
                        let a2923 = {
                            let a2920 = v2;
                            let a2921 = 1u64;
                            nat_add(a2920, a2921)?
                        };
                        index_bytes(a2922, a2923)
                    };
                    match m2924 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f640(v4)? {
                                f640(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2925 = v2;
                                    let a2926 = 2u64;
                                    nat_add(a2925, a2926)?
                                }, Some({
                                    let a2931 = v3;
                                    let a2932 = f551(f641(), {
                                        let a2929 = {
                                            let a2927 = f642(v4)?;
                                            let a2928 = 16u64;
                                            nat_mul(a2927, a2928)?
                                        };
                                        let a2930 = f642(v5)?;
                                        nat_add(a2929, a2930)?
                                    });
                                    append_bytes(a2931, a2932)
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

pub fn f608(v0: Fn20, p2937: u64, p2938: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2936 = (p2937, p2938);
    {
        let mut m2936 = m2936;
        loop {
            let step = {
                let (v1, v2) = m2936;
                let m2933 = v1;
                if m2933 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2933 - 1;
                    let m2935 = {
                        let c2934: Fn20 = v0.clone();
                        c2934.apply(v2.clone())?
                    };
                    match m2935 {
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
                Ok(next) => { m2936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f609(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f643(v0)
}

pub fn f610(p2941: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2940 = p2941;
    {
        let mut m2940 = m2940;
        loop {
            let step = {
                let v0 = m2940;
                let m2939 = v0;
                match m2939.uncons() {
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
                Ok(next) => { m2940 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f611(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2942 = v0;
    match m2942 {
        Adt2::C0 => {
            Ok(v1)
        }
        Adt2::C1(_, _) => {
            Ok(v1)
        }
        Adt2::C2(_) => {
            Ok(v1)
        }
        Adt2::C3(_, _) => {
            Ok(v1)
        }
        Adt2::C4(_, _, _) => {
            Ok(v1)
        }
        Adt2::C5(_, _) => {
            Ok(v1)
        }
        Adt2::C6(_) => {
            Ok(v1)
        }
        Adt2::C7(r2945) => {
            let v2 = (*r2945).clone();
            f611(v2, v1)
        }
        Adt2::C8(r2946) => {
            let v3 = (*r2946).clone();
            f611(v3, v1)
        }
        Adt2::C9 => {
            Ok(v1)
        }
        Adt2::C10(r2947, r2948) => {
            let v4 = (*r2947).clone();
            let v5 = (*r2948).clone();
            f611(v4, f611(v5, v1)?)
        }
        Adt2::C11(v6, r2949, r2950) => {
            let v7 = (*r2949).clone();
            let v8 = (*r2950).clone();
            let v9: List<Bytes> = f611(v7.clone(), f611(v8, v1)?)?;
            if f32(v6, f8(Str::lit("$ref"))) {
                let m2943 = f92(Some(v7));
                match m2943 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2944 = f644(v10)?;
                        match m2944 {
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

pub fn f612() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f613(v0: Bytes) -> R<Bytes> {
    let m2953 = {
        let a2951 = v0.clone();
        let a2952 = 0u64;
        index_bytes(a2951, a2952)
    };
    match m2953 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2959 = {
                let a2954 = f8(Str::lit("u"));
                let a2955 = f645(8u64, f72(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2954, a2955)
            };
            let a2960 = f46(v0.clone(), 1u64, {
                let a2957 = {
                    let a2956 = v0;
                    length_bytes(a2956)
                };
                let a2958 = 1u64;
                nat_sub(a2957, a2958)
            });
            Ok(append_bytes(a2959, a2960))
        }
    }
}

pub fn f614(v0: Bytes, v1: Adt2) -> R<Result<Adt27, Bytes>> {
    let m2961 = f93(v1.clone(), f8(Str::lit("enum")));
    match m2961 {
        None => {
            let m2963 = f93(v1.clone(), f8(Str::lit("oneOf")));
            match m2963 {
                None => {
                    if f650(v1.clone(), v0.clone())? {
                        let m2964 = f651(v0.clone(), f533(f93(v1.clone(), f8(Str::lit("properties")))), f214(f187(f93(v1, f8(Str::lit("required"))))), true)?;
                        match m2964 {
                            Ok(v7) => {
                                Ok(f648(f652(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f397(f653(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f654(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt27, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2965 = f533(f93(v1.clone(), f8(Str::lit("properties"))));
                        match m2965.uncons() {
                            None => {
                                if f655(v1.clone()) {
                                    Ok(f648(f656(v0), List::<Adt24>::nil()))
                                } else {
                                    let m2968 = f657(v1, {
                                        let a2966 = v0.clone();
                                        let a2967 = f8(Str::lit("Value"));
                                        append_bytes(a2966, a2967)
                                    })?;
                                    match m2968 {
                                        Ok(v9) => {
                                            Ok(f648(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f581(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f580(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt27, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2969 = f651(v0.clone(), f533(f93(v1.clone(), f8(Str::lit("properties")))), f214(f187(f93(v1, f8(Str::lit("required"))))), false)?;
                                match m2969 {
                                    Ok(v13) => {
                                        Ok(f648(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2970 = f653(v13.clone());
                                            match m2970.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f397(f653(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f654(v13)))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<Adt27, Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f649(v0, f304(v5))
                }
            }
        }
        Some(v2) => {
            let m2962 = f646(f304(v2), List::<Bytes>::nil())?;
            match m2962 {
                Ok(v4) => {
                    Ok(f648(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f397(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt24>::nil()))
                }
                Err(v3) => {
                    Ok(f647(v3, v0))
                }
            }
        }
    }
}

pub fn f615(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m2971 = f93(v0.clone(), f8(Str::lit("anyOf")));
        match m2971 {
            None => {
                v0
            }
            Some(v1) => {
                if f658(f304(v1.clone())) {
                    f659(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f660(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f661(f304(v1.clone()))
                    } {
                        v0
                    } else {
                        f662(f659(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m2972 = f93(v2.clone(), f8(Str::lit("oneOf")));
        match m2972 {
            None => {
                v2
            }
            Some(v3) => {
                if f658(f304(v3)) {
                    f659(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2973 = f93(v4.clone(), f8(Str::lit("allOf")));
    match m2973 {
        None => {
            v4
        }
        Some(v5) => {
            if f663(f304(v5)) {
                f659(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f616(v0: Adt26) -> R<Adt26> {
    let v1: u32 = f664(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2974) => {
            h2974
        }
    });
    let v2: u32 = f584(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h2975, _, _) => {
            h2975
        }
    }, f584(f665(match v0.clone() {
        Adt26::C0(_, _, _, _, h2976, _, _, _, _, _) => {
            h2976
        }
    }), f584(f666(match v0.clone() {
        Adt26::C0(_, _, _, _, h2977, _, _, _, _, _) => {
            h2977
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2978, _, _, _, _) => {
            h2978
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2979, _, _, _) => {
            h2979
        }
    }), f584(v1, {
        let m2982 = match v0.clone() {
            Adt26::C0(_, _, _, _, _, _, _, _, h2981, _) => {
                h2981
            }
        };
        match m2982 {
            Adt25::C0(h2980, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2980
            }
        }
    }))));
    let v3: u32 = f584(f667(match v0.clone() {
        Adt26::C0(h2983, _, _, _, _, _, _, _, _, _) => {
            h2983
        }
    }), f668(match v0.clone() {
        Adt26::C0(h2984, _, _, _, _, _, _, _, _, _) => {
            h2984
        }
    }, match v0.clone() {
        Adt26::C0(_, h2985, _, _, _, _, _, _, _, _) => {
            h2985
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2986, _, _, _, _, _, _, _) => {
            h2986
        }
    }));
    Ok(Adt26::C0(f584(v2, v3), match v0.clone() {
        Adt26::C0(h2987, _, _, _, _, _, _, _, _, _) => {
            h2987
        }
    }, match v0.clone() {
        Adt26::C0(_, h2988, _, _, _, _, _, _, _, _) => {
            h2988
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2989, _, _, _, _, _, _, _) => {
            h2989
        }
    }, f584(match v0.clone() {
        Adt26::C0(_, _, _, h2990, _, _, _, _, _, _) => {
            h2990
        }
    }, v2), match v0.clone() {
        Adt26::C0(_, _, _, _, h2991, _, _, _, _, _) => {
            h2991
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2992, _, _, _, _) => {
            h2992
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2993, _, _, _) => {
            h2993
        }
    }, f669(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h2994, _) => {
            h2994
        }
    }), {
        let a2996 = match v0 {
            Adt26::C0(_, _, _, _, _, _, _, _, _, h2995) => {
                h2995
            }
        };
        let a2997 = 1u64;
        nat_add(a2996, a2997)?
    }))
}

pub fn f617(v0: u32) -> u64 {
    let m2999 = {
        let a2998 = v0;
        fixed_u64::convert(i128::from(a2998))
    };
    match m2999 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f618(v0: u64) -> u32 {
    let m3003 = {
        let a3002 = {
            let a3000 = v0;
            let a3001 = 4294967295u64;
            fixed_u64::bit_and(a3000, a3001)
        };
        fixed_u32::convert(i128::from(a3002))
    };
    match m3003 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f619(v0: u64) -> u32 {
    let (_, h3004) = {
        let (_, h3005) = f670(Fn21::F671, 32u64, (v0, (1u32, 0u32)));
        h3005
    };
    h3004
}

pub fn f620(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3010 = f672(v0, 24u32);
    let a3011 = {
        let a3008 = f672(v1, 16u32);
        let a3009 = {
            let a3006 = f672(v2, 8u32);
            let a3007 = v3;
            fixed_u32::bit_or(a3006, a3007)
        };
        fixed_u32::bit_or(a3008, a3009)
    };
    fixed_u32::bit_or(a3010, a3011)
}

pub fn f621(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3012 = v2;
    let a3013 = v1;
    if nat_lt(a3012, a3013) {
        let m3016 = {
            let a3014 = v0;
            let a3015 = v2;
            index_bytes(a3014, a3015)
        };
        match m3016 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3018 = {
                    let a3017 = v3;
                    fixed_u32::convert(i128::from(a3017))
                };
                match m3018 {
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
        let a3019 = v2;
        let a3020 = v1;
        if nat_eq(a3019, a3020) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f622(v0: bool, v1: u64) -> R<bool> {
    let a3027 = {
        let a3024 = {
            let a3021 = v1;
            let a3022 = f407(v0)?;
            let a3023 = 0u64;
            nat_quot(a3021, a3022, a3023)
        };
        let a3025 = f409(f410(v0))?;
        let a3026 = 0u64;
        nat_rem(a3024, a3025, a3026)
    };
    let a3028 = f411(v0)?;
    Ok(!nat_eq(a3027, a3028))
}

pub fn f623() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("patternProperties")), List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("definitions")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("format")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))
}

pub fn f624(v0: Adt2) -> Bytes {
    let m3029 = v0;
    match m3029 {
        Adt2::C0 => {
            f8(Str::lit("null"))
        }
        Adt2::C1(_, _) => {
            f8(Str::lit("tagged"))
        }
        Adt2::C2(_) => {
            f8(Str::lit("boolean"))
        }
        Adt2::C3(_, _) => {
            f8(Str::lit("integer"))
        }
        Adt2::C4(_, _, _) => {
            f8(Str::lit("number"))
        }
        Adt2::C5(_, _) => {
            f8(Str::lit("number"))
        }
        Adt2::C6(_) => {
            f8(Str::lit("string"))
        }
        Adt2::C7(_) => {
            f8(Str::lit("array"))
        }
        Adt2::C8(_) => {
            f8(Str::lit("object"))
        }
        Adt2::C9 => {
            f8(Str::lit("spine"))
        }
        Adt2::C10(_, _) => {
            f8(Str::lit("spine"))
        }
        Adt2::C11(_, _, _) => {
            f8(Str::lit("spine"))
        }
    }
}

pub fn f625(p3032: List<Bytes>, v1: Adt2) -> bool {
    let m3031 = p3032;
    {
        let mut m3031 = m3031;
        loop {
            let step = {
                let v0 = m3031;
                let m3030 = v0;
                match m3030.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f673(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3031 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626(p3035: List<Adt2>, v1: Adt2) -> R<bool> {
    let m3034 = p3035;
    {
        let mut m3034 = m3034;
        loop {
            let step = {
                let v0 = m3034;
                let m3033 = v0;
                match m3033.uncons() {
                    None => {
                        Err::<List<Adt2>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f627(false, v2, v1.clone())? {
                            Err::<List<Adt2>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt2>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f627(v0: bool, v1: Adt2, v2: Adt2) -> R<bool> {
    let m3036 = v1;
    match m3036 {
        Adt2::C0 => {
            Ok(f32(f546(v2)?, f8(Str::lit("null"))))
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(v85) => {
            Ok(f32(f546(v2)?, f546(Adt2::C2(v85))?))
        }
        Adt2::C3(v86, v87) => {
            Ok(f32(f546(v2)?, f546(Adt2::C3(v86, v87))?))
        }
        Adt2::C4(v88, v89, v90) => {
            Ok(f32(f546(v2)?, f546(Adt2::C4(v88, v89, v90))?))
        }
        Adt2::C5(v91, v92) => {
            Ok(f32(f546(v2)?, f546(Adt2::C5(v91, v92))?))
        }
        Adt2::C6(v93) => {
            let m3048 = v2;
            match m3048 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(v94) => {
                    Ok(f32(v93, v94))
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8(_) => {
                    Ok(false)
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(_, _) => {
                    Ok(false)
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C7(r3049) => {
            let v64 = (*r3049).clone();
            let m3046 = v2;
            match m3046 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(r3047) => {
                    let v65 = (*r3047).clone();
                    f627(false, v64, v65)
                }
                Adt2::C8(_) => {
                    Ok(false)
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(_, _) => {
                    Ok(false)
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C8(r3050) => {
            let v45 = (*r3050).clone();
            let m3042 = v2;
            match m3042 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8(r3045) => {
                    let v46 = (*r3045).clone();
                    if f117({
                        let a3043 = f631(v45.clone())?;
                        let a3044 = f631(v46.clone())?;
                        nat_eq(a3043, a3044)
                    }, true) {
                        f627(true, v45, v46)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(_, _) => {
                    Ok(false)
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C9 => {
            if v0 {
                Ok(true)
            } else {
                let m3038 = v2;
                match m3038 {
                    Adt2::C0 => {
                        Ok(false)
                    }
                    Adt2::C1(_, _) => {
                        Ok(false)
                    }
                    Adt2::C2(_) => {
                        Ok(false)
                    }
                    Adt2::C3(_, _) => {
                        Ok(false)
                    }
                    Adt2::C4(_, _, _) => {
                        Ok(false)
                    }
                    Adt2::C5(_, _) => {
                        Ok(false)
                    }
                    Adt2::C6(_) => {
                        Ok(false)
                    }
                    Adt2::C7(_) => {
                        Ok(false)
                    }
                    Adt2::C8(_) => {
                        Ok(false)
                    }
                    Adt2::C9 => {
                        Ok(true)
                    }
                    Adt2::C10(_, _) => {
                        Ok(false)
                    }
                    Adt2::C11(_, _, _) => {
                        Ok(false)
                    }
                }
            }
        }
        Adt2::C10(r3051, r3052) => {
            let v25 = (*r3051).clone();
            let v26 = (*r3052).clone();
            let m3039 = v2;
            match m3039 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8(_) => {
                    Ok(false)
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(r3040, r3041) => {
                    let v27 = (*r3040).clone();
                    let v28 = (*r3041).clone();
                    if f627(false, v25, v27)? {
                        f627(false, v26, v28)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C11(v3, r3053, r3054) => {
            let v4 = (*r3053).clone();
            let v5 = (*r3054).clone();
            let m3037 = f93(v2.clone(), v3);
            if match m3037 {
                None => {
                    false
                }
                Some(v6) => {
                    f627(false, v4, v6)?
                }
            } {
                f627(true, v5, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f628(v0: Bytes) -> R<Option<List<u64>>> {
    f674({
        let a3056 = {
            let a3055 = v0.clone();
            length_bytes(a3055)
        };
        let a3057 = 1u64;
        nat_add(a3056, a3057)?
    }, v0, 0u64)
}

pub fn f629(v0: List<u64>) -> R<u64> {
    let m3058 = v0;
    match m3058.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a3059 = f629(v2)?;
            let a3060 = 1u64;
            nat_add(a3059, a3060)
        }
    }
}

pub fn f630(v0: Bytes, v1: Bytes) -> R<Result<bool, Bytes>> {
    let m3061 = f675(v0)?;
    match m3061 {
        Ok(v3) => {
            let m3062 = f628(v1)?;
            match m3062 {
                None => {
                    Ok(Err::<bool, Bytes>(f8(Str::lit("a string is not well-formed UTF-8 (RFC 3629 section 4)"))))
                }
                Some(v4) => {
                    Ok(Ok::<bool, Bytes>({
                        let (h3063, _) = {
                            let (h3064, _) = v3.clone();
                            h3064
                        };
                        if h3063 {
                            f676({
                                let a3068 = {
                                    let a3066 = f677({
                                        let (_, h3065) = v3.clone();
                                        h3065
                                    })?;
                                    let a3067 = f629(v4.clone())?;
                                    nat_add(a3066, a3067)?
                                };
                                let a3069 = 2u64;
                                nat_add(a3068, a3069)?
                            }, {
                                let (_, h3070) = v3;
                                h3070
                            }, v4, {
                                let (_, h3071) = {
                                    let (h3072, _) = v3;
                                    h3072
                                };
                                h3071
                            })?
                        } else {
                            f678({
                                let (_, h3073) = v3;
                                h3073
                            }, v4, {
                                let (_, h3074) = {
                                    let (h3075, _) = v3;
                                    h3075
                                };
                                h3074
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

pub fn f631(v0: Adt2) -> R<u64> {
    let m3076 = v0;
    match m3076 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1(_, _) => {
            Ok(0u64)
        }
        Adt2::C2(_) => {
            Ok(0u64)
        }
        Adt2::C3(_, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_, _) => {
            Ok(0u64)
        }
        Adt2::C6(_) => {
            Ok(0u64)
        }
        Adt2::C7(_) => {
            Ok(0u64)
        }
        Adt2::C8(_) => {
            Ok(0u64)
        }
        Adt2::C9 => {
            Ok(0u64)
        }
        Adt2::C10(_, r3081) => {
            let v5 = (*r3081).clone();
            let a3079 = f631(v5)?;
            let a3080 = 1u64;
            nat_add(a3079, a3080)
        }
        Adt2::C11(_, _, r3082) => {
            let v3 = (*r3082).clone();
            let a3077 = f631(v3)?;
            let a3078 = 1u64;
            nat_add(a3077, a3078)
        }
    }
}

pub fn f632(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3083 = v0;
    match m3083.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f632(v3, v1)
            } else {
                List::cons(v2, f632(v3, v1))
            }
        }
    }
}

pub fn f633(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m3084 = v0;
    match m3084.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h3085, _) = v1;
                h3085
            }, f633(v2))
        }
    }
}

pub fn f634(v0: Adt2) -> Option<(bool, (u64, u64))> {
    let m3086 = v0;
    match m3086 {
        Adt2::C0 => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C1(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C2(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C3(v1, v2) => {
            Some((v1, (v2, 0u64)))
        }
        Adt2::C4(v3, v4, v5) => {
            Some((v3, (v4, v5)))
        }
        Adt2::C5(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C6(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C7(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C8(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C9 => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C10(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C11(_, _, _) => {
            None::<(bool, (u64, u64))>
        }
    }
}

pub fn f635(v0: (bool, (u64, u64)), v1: (bool, (u64, u64))) -> R<bool> {
    let v2: u64 = {
        let a3091 = {
            let (h3087, _) = {
                let (_, h3088) = v0;
                h3088
            };
            h3087
        };
        let a3092 = f679({
            let (_, h3089) = {
                let (_, h3090) = v1;
                h3090
            };
            h3089
        })?;
        nat_mul(a3091, a3092)?
    };
    let v3: u64 = {
        let a3097 = {
            let (h3093, _) = {
                let (_, h3094) = v1;
                h3094
            };
            h3093
        };
        let a3098 = f679({
            let (_, h3095) = {
                let (_, h3096) = v0;
                h3096
            };
            h3095
        })?;
        nat_mul(a3097, a3098)?
    };
    let (h3099, _) = v0;
    if h3099 {
        let (h3100, _) = v1;
        if h3100 {
            let a3101 = v3;
            let a3102 = v2;
            Ok(nat_lt(a3101, a3102))
        } else {
            Ok(true)
        }
    } else {
        let (h3103, _) = v1;
        if h3103 {
            Ok(false)
        } else {
            let a3104 = v2;
            let a3105 = v3;
            Ok(nat_lt(a3104, a3105))
        }
    }
}

pub fn f636(p3135: u64, v1: Bytes, p3136: u64, p3137: Bytes) -> R<Bytes> {
    let m3134 = (p3135, (p3136, p3137));
    {
        let mut m3134 = m3134;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3134;
                let m3106 = v0;
                if m3106 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3106 - 1;
                    let a3108 = {
                        let a3107 = v1.clone();
                        length_bytes(a3107)
                    };
                    let a3109 = v2;
                    if nat_le(a3108, a3109) {
                        Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                    } else {
                        if f117({
                            let a3110 = f115(v1.clone(), v2)?;
                            let a3111 = 126u64;
                            nat_eq(a3110, a3111)
                        }, {
                            let a3114 = f115(v1.clone(), {
                                let a3112 = v2;
                                let a3113 = 1u64;
                                nat_add(a3112, a3113)?
                            })?;
                            let a3115 = 48u64;
                            nat_eq(a3114, a3115)
                        }) {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3116 = v2;
                                let a3117 = 2u64;
                                nat_add(a3116, a3117)?
                            }, {
                                let a3118 = v3;
                                let a3119 = f8(Str::lit("~"));
                                append_bytes(a3118, a3119)
                            })))
                        } else {
                            if f117({
                                let a3120 = f115(v1.clone(), v2)?;
                                let a3121 = 126u64;
                                nat_eq(a3120, a3121)
                            }, {
                                let a3124 = f115(v1.clone(), {
                                    let a3122 = v2;
                                    let a3123 = 1u64;
                                    nat_add(a3122, a3123)?
                                })?;
                                let a3125 = 49u64;
                                nat_eq(a3124, a3125)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a3126 = v2;
                                    let a3127 = 2u64;
                                    nat_add(a3126, a3127)?
                                }, {
                                    let a3128 = v3;
                                    let a3129 = f8(Str::lit("/"));
                                    append_bytes(a3128, a3129)
                                })))
                            } else {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a3130 = v2;
                                    let a3131 = 1u64;
                                    nat_add(a3130, a3131)?
                                }, {
                                    let a3132 = v3;
                                    let a3133 = f46(v1.clone(), v2, 1u64);
                                    append_bytes(a3132, a3133)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3134 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f637(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(f680({
        let a3139 = {
            let a3138 = v1.clone();
            length_bytes(a3138)
        };
        let a3140 = 1u64;
        nat_add(a3139, a3140)?
    }, v1, 0u64, Bytes::lit(&[]))?, List::<Bytes>::nil())))))
}

pub fn f638(v0: List<(Bytes, Adt2)>, v1: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let m3141 = v0;
    match m3141.uncons() {
        None => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let m3143 = f630({
                let (h3142, _) = v2;
                h3142
            }, v1.clone())?;
            match m3143 {
                Ok(v5) => {
                    let m3144 = f638(v3, v1)?;
                    match m3144 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt2>, Bytes>(if v5 {
                                List::cons({
                                    let (_, h3145) = v2;
                                    h3145
                                }, v7)
                            } else {
                                v7
                            }))
                        }
                        Err(v6) => {
                            Ok(Err::<List<Adt2>, Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt2>, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f639(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f559({
        let (_, h3146) = {
            let (h3147, _) = f681(Fn22::F682, {
                let a3149 = {
                    let a3148 = v0.clone();
                    length_list(a3148)
                };
                let a3150 = 1u64;
                nat_add(a3149, a3150)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3147
        };
        h3146
    }))
}

pub fn f640(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3195 = {
            let a3171 = {
                let a3159 = {
                    let a3153 = {
                        let a3151 = v1;
                        let a3152 = 128u8;
                        fixed_u8::bit_and(a3151, a3152)
                    };
                    let a3154 = 128u8;
                    if equal(a3153, a3154) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3160 = {
                    let a3157 = {
                        let a3155 = v1;
                        let a3156 = 64u8;
                        fixed_u8::bit_and(a3155, a3156)
                    };
                    let a3158 = 64u8;
                    if equal(a3157, a3158) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3159, a3160)?
            };
            let a3172 = {
                let a3169 = {
                    let a3163 = {
                        let a3161 = v1;
                        let a3162 = 32u8;
                        fixed_u8::bit_and(a3161, a3162)
                    };
                    let a3164 = 32u8;
                    if equal(a3163, a3164) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3170 = {
                    let a3167 = {
                        let a3165 = v1;
                        let a3166 = 16u8;
                        fixed_u8::bit_and(a3165, a3166)
                    };
                    let a3168 = 16u8;
                    if equal(a3167, a3168) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3169, a3170)?
            };
            nat_add(a3171, a3172)?
        };
        let a3196 = {
            let a3193 = {
                let a3181 = {
                    let a3175 = {
                        let a3173 = v1;
                        let a3174 = 8u8;
                        fixed_u8::bit_and(a3173, a3174)
                    };
                    let a3176 = 8u8;
                    if equal(a3175, a3176) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3182 = {
                    let a3179 = {
                        let a3177 = v1;
                        let a3178 = 4u8;
                        fixed_u8::bit_and(a3177, a3178)
                    };
                    let a3180 = 4u8;
                    if equal(a3179, a3180) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3181, a3182)?
            };
            let a3194 = {
                let a3191 = {
                    let a3185 = {
                        let a3183 = v1;
                        let a3184 = 2u8;
                        fixed_u8::bit_and(a3183, a3184)
                    };
                    let a3186 = 2u8;
                    if equal(a3185, a3186) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3192 = {
                    let a3189 = {
                        let a3187 = v1;
                        let a3188 = 1u8;
                        fixed_u8::bit_and(a3187, a3188)
                    };
                    let a3190 = 1u8;
                    if equal(a3189, a3190) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3191, a3192)?
            };
            nat_add(a3193, a3194)?
        };
        nat_add(a3195, a3196)?
    };
    let a3197 = 48u64;
    let a3198 = v2;
    if if nat_le(a3197, a3198) {
        let a3199 = v2;
        let a3200 = 57u64;
        nat_le(a3199, a3200)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3201 = 97u64;
        let a3202 = v2;
        if nat_le(a3201, a3202) {
            let a3203 = v2;
            let a3204 = 102u64;
            Ok(nat_le(a3203, a3204))
        } else {
            Ok(false)
        }
    }
}

pub fn f641() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f642(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3249 = {
            let a3225 = {
                let a3213 = {
                    let a3207 = {
                        let a3205 = v1;
                        let a3206 = 128u8;
                        fixed_u8::bit_and(a3205, a3206)
                    };
                    let a3208 = 128u8;
                    if equal(a3207, a3208) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3214 = {
                    let a3211 = {
                        let a3209 = v1;
                        let a3210 = 64u8;
                        fixed_u8::bit_and(a3209, a3210)
                    };
                    let a3212 = 64u8;
                    if equal(a3211, a3212) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3213, a3214)?
            };
            let a3226 = {
                let a3223 = {
                    let a3217 = {
                        let a3215 = v1;
                        let a3216 = 32u8;
                        fixed_u8::bit_and(a3215, a3216)
                    };
                    let a3218 = 32u8;
                    if equal(a3217, a3218) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3224 = {
                    let a3221 = {
                        let a3219 = v1;
                        let a3220 = 16u8;
                        fixed_u8::bit_and(a3219, a3220)
                    };
                    let a3222 = 16u8;
                    if equal(a3221, a3222) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3223, a3224)?
            };
            nat_add(a3225, a3226)?
        };
        let a3250 = {
            let a3247 = {
                let a3235 = {
                    let a3229 = {
                        let a3227 = v1;
                        let a3228 = 8u8;
                        fixed_u8::bit_and(a3227, a3228)
                    };
                    let a3230 = 8u8;
                    if equal(a3229, a3230) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3236 = {
                    let a3233 = {
                        let a3231 = v1;
                        let a3232 = 4u8;
                        fixed_u8::bit_and(a3231, a3232)
                    };
                    let a3234 = 4u8;
                    if equal(a3233, a3234) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3235, a3236)?
            };
            let a3248 = {
                let a3245 = {
                    let a3239 = {
                        let a3237 = v1;
                        let a3238 = 2u8;
                        fixed_u8::bit_and(a3237, a3238)
                    };
                    let a3240 = 2u8;
                    if equal(a3239, a3240) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3246 = {
                    let a3243 = {
                        let a3241 = v1;
                        let a3242 = 1u8;
                        fixed_u8::bit_and(a3241, a3242)
                    };
                    let a3244 = 1u8;
                    if equal(a3243, a3244) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3245, a3246)?
            };
            nat_add(a3247, a3248)?
        };
        nat_add(a3249, a3250)?
    };
    let a3251 = v2;
    let a3252 = 57u64;
    if nat_le(a3251, a3252) {
        let a3253 = v2;
        let a3254 = 48u64;
        Ok(nat_sub(a3253, a3254))
    } else {
        let a3255 = v2;
        let a3256 = 87u64;
        Ok(nat_sub(a3255, a3256))
    }
}

pub fn f643(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3257, _) = v0;
        h3257
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3258) = v0;
        h3258
    };
    let m3259 = v1;
    match m3259.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3260 = v4;
            match m3260.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f683(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f644(v0: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f576(f46(v0.clone(), 8u64, {
            let a3262 = {
                let a3261 = v0;
                length_bytes(a3261)
            };
            let a3263 = 8u64;
            nat_sub(a3262, a3263)
        }))?))
    } else {
        if f85(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f576(f46(v0.clone(), 14u64, {
                let a3265 = {
                    let a3264 = v0;
                    length_bytes(a3264)
                };
                let a3266 = 14u64;
                nat_sub(a3265, a3266)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f645(p3283: u64, p3284: u64, p3285: u64, p3286: Bytes) -> Bytes {
    let m3282 = (p3283, (p3284, (p3285, p3286)));
    {
        let mut m3282 = m3282;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3282;
                let m3267 = v0;
                if m3267 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3267 - 1;
                    if f117({
                        let a3268 = v1;
                        let a3269 = 0u64;
                        nat_eq(a3268, a3269)
                    }, {
                        let a3270 = v2;
                        let a3271 = 0u64;
                        nat_eq(a3270, a3271)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3272 = v1;
                            let a3273 = 16u64;
                            let a3274 = 0u64;
                            nat_quot(a3272, a3273, a3274)
                        }, ({
                            let a3275 = v2;
                            let a3276 = 1u64;
                            nat_sub(a3275, a3276)
                        }, {
                            let a3280 = f684({
                                let a3277 = v1;
                                let a3278 = 16u64;
                                let a3279 = 0u64;
                                nat_rem(a3277, a3278, a3279)
                            });
                            let a3281 = v3;
                            append_bytes(a3280, a3281)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f646(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f685(f686(f214(v0.clone()))?, v0, v1)
}

pub fn f647(v0: Bytes, v1: Bytes) -> Result<Adt27, Bytes> {
    Err::<Adt27, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f648(v0: Bytes, v1: List<Adt24>) -> Result<Adt27, Bytes> {
    Ok::<Adt27, Bytes>(Adt27::C0(v0, v1))
}

pub fn f649(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt27, Bytes>> {
    let m3287 = v1.clone();
    match m3287.uncons() {
        None => {
            Ok(f647(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3288 = f687(f533(f93(v2, f8(Str::lit("properties")))), v1.clone());
            match m3288.uncons() {
                None => {
                    let m3289 = f688(f686(f689(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3289 {
                        Ok(v5) => {
                            Ok(f648(f652(v0, f653(v5.clone())), f654(v5)))
                        }
                        Err(v4) => {
                            Ok(f647(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3290 = f690(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3290 {
                        Ok(v9) => {
                            Ok(f648(f652(v0, f653(v9.clone())), f654(v9)))
                        }
                        Err(v8) => {
                            Ok(f647(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f650(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3291 = v0;
    match m3291 {
        Adt2::C0 => {
            Ok(false)
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(_) => {
            Ok(false)
        }
        Adt2::C3(_, _) => {
            Ok(false)
        }
        Adt2::C4(_, _, _) => {
            Ok(false)
        }
        Adt2::C5(_, _) => {
            Ok(false)
        }
        Adt2::C6(_) => {
            Ok(false)
        }
        Adt2::C7(r3292) => {
            let v2 = (*r3292).clone();
            f650(v2, v1)
        }
        Adt2::C8(r3293) => {
            let v3 = (*r3293).clone();
            f650(v3, v1)
        }
        Adt2::C9 => {
            Ok(false)
        }
        Adt2::C10(r3294, r3295) => {
            let v4 = (*r3294).clone();
            let v5 = (*r3295).clone();
            if f650(v4, v1.clone())? {
                Ok(true)
            } else {
                f650(v5, v1)
            }
        }
        Adt2::C11(v6, r3296, r3297) => {
            let v7 = (*r3296).clone();
            let v8 = (*r3297).clone();
            if if f32(v6, f8(Str::lit("$ref"))) {
                f691(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f650(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f650(v8, v1)
                }
            }
        }
    }
}

pub fn f651(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt28, Bytes>> {
    f692(f686(f633(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f652(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f397(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f653(v0: Adt28) -> List<Bytes> {
    let m3298 = v0;
    match m3298 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f654(v0: Adt28) -> List<Adt24> {
    let m3299 = v0;
    match m3299 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f655(v0: Adt2) -> bool {
    let m3300 = f533(f93(v0.clone(), f8(Str::lit("properties"))));
    match m3300.uncons() {
        None => {
            let m3301 = f693(v0);
            match m3301 {
                None => {
                    false
                }
                Some(v3) => {
                    f694(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f656(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f657(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3302 = f695(v0.clone());
    match m3302 {
        None => {
            if f32(f58(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3303 = f93(v0, f8(Str::lit("items")));
                match m3303 {
                    None => {
                        Ok(f696(f8(Str::lit("List")), f698()))
                    }
                    Some(v3) => {
                        Ok(f696(f8(Str::lit("List")), f697(v3, {
                            let a3304 = v1;
                            let a3305 = f8(Str::lit("Item"));
                            append_bytes(a3304, a3305)
                        })?))
                    }
                }
            } else {
                f697(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f696(f8(Str::lit("Option")), f697(v2, v1)?))
        }
    }
}

pub fn f658(p3308: List<Adt2>) -> bool {
    let m3307 = p3308;
    {
        let mut m3307 = m3307;
        loop {
            let step = {
                let v0 = m3307;
                let m3306 = v0;
                match m3306.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f699(f573(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3307 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f659(v0: Adt2, v1: Bytes) -> Adt2 {
    let m3309 = v0;
    match m3309 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v6, r3310) => {
            let v7 = (*r3310).clone();
            Adt2::C1(v6, std::rc::Rc::new(v7))
        }
        Adt2::C2(v8) => {
            Adt2::C2(v8)
        }
        Adt2::C3(v14, v15) => {
            Adt2::C3(v14, v15)
        }
        Adt2::C4(v9, v10, v11) => {
            Adt2::C4(v9, v10, v11)
        }
        Adt2::C5(v12, v13) => {
            Adt2::C5(v12, v13)
        }
        Adt2::C6(v16) => {
            Adt2::C6(v16)
        }
        Adt2::C7(r3311) => {
            let v17 = (*r3311).clone();
            Adt2::C7(std::rc::Rc::new(v17))
        }
        Adt2::C8(r3312) => {
            let v5 = (*r3312).clone();
            Adt2::C8(std::rc::Rc::new(f659(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r3313, r3314) => {
            let v18 = (*r3313).clone();
            let v19 = (*r3314).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r3315, r3316) => {
            let v3 = (*r3315).clone();
            let v4 = (*r3316).clone();
            if f32(v2.clone(), v1.clone()) {
                f659(v4, v1)
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f659(v4, v1)))
            }
        }
    }
}

pub fn f660(v0: Adt2, v1: Bytes) -> bool {
    let m3317 = f93(v0, v1);
    match m3317 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f661(v0: List<Adt2>) -> bool {
    let m3318 = v0.clone();
    match m3318.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3319 = f687(f533(f93(v1, f8(Str::lit("properties")))), v0.clone());
            match m3319.uncons() {
                None => {
                    f701(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f700(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f662(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m3320 = v0;
    match m3320 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v7, r3321) => {
            let v8 = (*r3321).clone();
            Adt2::C1(v7, std::rc::Rc::new(v8))
        }
        Adt2::C2(v9) => {
            Adt2::C2(v9)
        }
        Adt2::C3(v15, v16) => {
            Adt2::C3(v15, v16)
        }
        Adt2::C4(v10, v11, v12) => {
            Adt2::C4(v10, v11, v12)
        }
        Adt2::C5(v13, v14) => {
            Adt2::C5(v13, v14)
        }
        Adt2::C6(v17) => {
            Adt2::C6(v17)
        }
        Adt2::C7(r3322) => {
            let v18 = (*r3322).clone();
            Adt2::C7(std::rc::Rc::new(v18))
        }
        Adt2::C8(r3323) => {
            let v3 = (*r3323).clone();
            Adt2::C8(std::rc::Rc::new(Adt2::C11(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r3324, r3325) => {
            let v19 = (*r3324).clone();
            let v20 = (*r3325).clone();
            Adt2::C10(std::rc::Rc::new(v19), std::rc::Rc::new(v20))
        }
        Adt2::C11(v4, r3326, r3327) => {
            let v5 = (*r3326).clone();
            let v6 = (*r3327).clone();
            Adt2::C11(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f663(p3330: List<Adt2>) -> bool {
    let m3329 = p3330;
    {
        let mut m3329 = m3329;
        loop {
            let step = {
                let v0 = m3329;
                let m3328 = v0;
                match m3328.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f702(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
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

pub fn f664(v0: u64) -> u32 {
    let a3331 = v0;
    let a3332 = 32u64;
    if nat_lt(a3331, a3332) {
        let a3333 = v0;
        let a3334 = 16u64;
        if nat_lt(a3333, a3334) {
            let a3335 = v0;
            let a3336 = 8u64;
            if nat_lt(a3335, a3336) {
                let a3337 = v0;
                let a3338 = 4u64;
                if nat_lt(a3337, a3338) {
                    let a3339 = v0;
                    let a3340 = 2u64;
                    if nat_lt(a3339, a3340) {
                        let a3341 = v0;
                        let a3342 = 1u64;
                        if nat_lt(a3341, a3342) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3343 = v0;
                        let a3344 = 3u64;
                        if nat_lt(a3343, a3344) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3345 = v0;
                    let a3346 = 6u64;
                    if nat_lt(a3345, a3346) {
                        let a3347 = v0;
                        let a3348 = 5u64;
                        if nat_lt(a3347, a3348) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3349 = v0;
                        let a3350 = 7u64;
                        if nat_lt(a3349, a3350) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3351 = v0;
                let a3352 = 12u64;
                if nat_lt(a3351, a3352) {
                    let a3353 = v0;
                    let a3354 = 10u64;
                    if nat_lt(a3353, a3354) {
                        let a3355 = v0;
                        let a3356 = 9u64;
                        if nat_lt(a3355, a3356) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3357 = v0;
                        let a3358 = 11u64;
                        if nat_lt(a3357, a3358) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3359 = v0;
                    let a3360 = 14u64;
                    if nat_lt(a3359, a3360) {
                        let a3361 = v0;
                        let a3362 = 13u64;
                        if nat_lt(a3361, a3362) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3363 = v0;
                        let a3364 = 15u64;
                        if nat_lt(a3363, a3364) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3365 = v0;
            let a3366 = 24u64;
            if nat_lt(a3365, a3366) {
                let a3367 = v0;
                let a3368 = 20u64;
                if nat_lt(a3367, a3368) {
                    let a3369 = v0;
                    let a3370 = 18u64;
                    if nat_lt(a3369, a3370) {
                        let a3371 = v0;
                        let a3372 = 17u64;
                        if nat_lt(a3371, a3372) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3373 = v0;
                        let a3374 = 19u64;
                        if nat_lt(a3373, a3374) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3375 = v0;
                    let a3376 = 22u64;
                    if nat_lt(a3375, a3376) {
                        let a3377 = v0;
                        let a3378 = 21u64;
                        if nat_lt(a3377, a3378) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3379 = v0;
                        let a3380 = 23u64;
                        if nat_lt(a3379, a3380) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3381 = v0;
                let a3382 = 28u64;
                if nat_lt(a3381, a3382) {
                    let a3383 = v0;
                    let a3384 = 26u64;
                    if nat_lt(a3383, a3384) {
                        let a3385 = v0;
                        let a3386 = 25u64;
                        if nat_lt(a3385, a3386) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3387 = v0;
                        let a3388 = 27u64;
                        if nat_lt(a3387, a3388) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3389 = v0;
                    let a3390 = 30u64;
                    if nat_lt(a3389, a3390) {
                        let a3391 = v0;
                        let a3392 = 29u64;
                        if nat_lt(a3391, a3392) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3393 = v0;
                        let a3394 = 31u64;
                        if nat_lt(a3393, a3394) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3395 = v0;
        let a3396 = 48u64;
        if nat_lt(a3395, a3396) {
            let a3397 = v0;
            let a3398 = 40u64;
            if nat_lt(a3397, a3398) {
                let a3399 = v0;
                let a3400 = 36u64;
                if nat_lt(a3399, a3400) {
                    let a3401 = v0;
                    let a3402 = 34u64;
                    if nat_lt(a3401, a3402) {
                        let a3403 = v0;
                        let a3404 = 33u64;
                        if nat_lt(a3403, a3404) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3405 = v0;
                        let a3406 = 35u64;
                        if nat_lt(a3405, a3406) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3407 = v0;
                    let a3408 = 38u64;
                    if nat_lt(a3407, a3408) {
                        let a3409 = v0;
                        let a3410 = 37u64;
                        if nat_lt(a3409, a3410) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3411 = v0;
                        let a3412 = 39u64;
                        if nat_lt(a3411, a3412) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3413 = v0;
                let a3414 = 44u64;
                if nat_lt(a3413, a3414) {
                    let a3415 = v0;
                    let a3416 = 42u64;
                    if nat_lt(a3415, a3416) {
                        let a3417 = v0;
                        let a3418 = 41u64;
                        if nat_lt(a3417, a3418) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3419 = v0;
                        let a3420 = 43u64;
                        if nat_lt(a3419, a3420) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3421 = v0;
                    let a3422 = 46u64;
                    if nat_lt(a3421, a3422) {
                        let a3423 = v0;
                        let a3424 = 45u64;
                        if nat_lt(a3423, a3424) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3425 = v0;
                        let a3426 = 47u64;
                        if nat_lt(a3425, a3426) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3427 = v0;
            let a3428 = 56u64;
            if nat_lt(a3427, a3428) {
                let a3429 = v0;
                let a3430 = 52u64;
                if nat_lt(a3429, a3430) {
                    let a3431 = v0;
                    let a3432 = 50u64;
                    if nat_lt(a3431, a3432) {
                        let a3433 = v0;
                        let a3434 = 49u64;
                        if nat_lt(a3433, a3434) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3435 = v0;
                        let a3436 = 51u64;
                        if nat_lt(a3435, a3436) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3437 = v0;
                    let a3438 = 54u64;
                    if nat_lt(a3437, a3438) {
                        let a3439 = v0;
                        let a3440 = 53u64;
                        if nat_lt(a3439, a3440) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3441 = v0;
                        let a3442 = 55u64;
                        if nat_lt(a3441, a3442) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3443 = v0;
                let a3444 = 60u64;
                if nat_lt(a3443, a3444) {
                    let a3445 = v0;
                    let a3446 = 58u64;
                    if nat_lt(a3445, a3446) {
                        let a3447 = v0;
                        let a3448 = 57u64;
                        if nat_lt(a3447, a3448) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3449 = v0;
                        let a3450 = 59u64;
                        if nat_lt(a3449, a3450) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3451 = v0;
                    let a3452 = 62u64;
                    if nat_lt(a3451, a3452) {
                        let a3453 = v0;
                        let a3454 = 61u64;
                        if nat_lt(a3453, a3454) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3455 = v0;
                        let a3456 = 63u64;
                        if nat_lt(a3455, a3456) {
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

pub fn f665(v0: u32) -> u32 {
    let a3459 = {
        let a3457 = f703(v0, 6u32, 26u32);
        let a3458 = f703(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3457, a3458)
    };
    let a3460 = f703(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3459, a3460)
}

pub fn f666(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3466 = {
        let a3461 = v0;
        let a3462 = v1;
        fixed_u32::bit_and(a3461, a3462)
    };
    let a3467 = {
        let a3464 = {
            let a3463 = v0;
            fixed_u32::bit_not(a3463)
        };
        let a3465 = v2;
        fixed_u32::bit_and(a3464, a3465)
    };
    fixed_u32::bit_xor(a3466, a3467)
}

pub fn f667(v0: u32) -> u32 {
    let a3470 = {
        let a3468 = f703(v0, 2u32, 30u32);
        let a3469 = f703(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3468, a3469)
    };
    let a3471 = f703(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3470, a3471)
}

pub fn f668(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3480 = {
        let a3476 = {
            let a3472 = v0;
            let a3473 = v1;
            fixed_u32::bit_and(a3472, a3473)
        };
        let a3477 = {
            let a3474 = v0;
            let a3475 = v2;
            fixed_u32::bit_and(a3474, a3475)
        };
        fixed_u32::bit_xor(a3476, a3477)
    };
    let a3481 = {
        let a3478 = v1;
        let a3479 = v2;
        fixed_u32::bit_and(a3478, a3479)
    };
    fixed_u32::bit_xor(a3480, a3481)
}

pub fn f669(v0: Adt25) -> Adt25 {
    Adt25::C0(match v0.clone() {
        Adt25::C0(_, h3482, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3482
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h3483, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3483
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, h3484, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3484
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, h3485, _, _, _, _, _, _, _, _, _, _, _) => {
            h3485
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h3486, _, _, _, _, _, _, _, _, _, _) => {
            h3486
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h3487, _, _, _, _, _, _, _, _, _) => {
            h3487
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h3488, _, _, _, _, _, _, _, _) => {
            h3488
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h3489, _, _, _, _, _, _, _) => {
            h3489
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h3490, _, _, _, _, _, _) => {
            h3490
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, h3491, _, _, _, _, _) => {
            h3491
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, h3492, _, _, _, _) => {
            h3492
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3493, _, _, _) => {
            h3493
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3494, _, _) => {
            h3494
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3495, _) => {
            h3495
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3496) => {
            h3496
        }
    }, f584(f704(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3497, _) => {
            h3497
        }
    }), f584(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h3498, _, _, _, _, _, _) => {
            h3498
        }
    }, f584(f705(match v0.clone() {
        Adt25::C0(_, h3499, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3499
        }
    }), match v0 {
        Adt25::C0(h3500, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3500
        }
    }))))
}

pub fn f670(v0: Fn21, p3504: u64, p3505: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3503 = (p3504, p3505);
    {
        let mut m3503 = m3503;
        loop {
            let step = {
                let (v1, v2) = m3503;
                let m3501 = v1;
                if m3501 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3501 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3502: Fn21 = v0.clone();
                        c3502.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3503 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f671(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3506, _) = v0;
        h3506
    };
    let v2: u32 = {
        let (h3507, _) = {
            let (_, h3508) = v0;
            h3508
        };
        h3507
    };
    let v3: u32 = {
        let (_, h3509) = {
            let (_, h3510) = v0;
            h3510
        };
        h3509
    };
    ({
        let a3511 = v1;
        let a3512 = 2u64;
        let a3513 = 0u64;
        nat_quot(a3511, a3512, a3513)
    }, (f672(v2, 1u32), {
        let a3517 = {
            let a3514 = v1;
            let a3515 = 2u64;
            let a3516 = 0u64;
            nat_rem(a3514, a3515, a3516)
        };
        let a3518 = 1u64;
        if nat_eq(a3517, a3518) {
            let a3519 = v3;
            let a3520 = v2;
            fixed_u32::bit_or(a3519, a3520)
        } else {
            v3
        }
    }))
}

pub fn f672(v0: u32, v1: u32) -> u32 {
    let m3523 = {
        let a3521 = v0;
        let a3522 = v1;
        fixed_u32::shift_left(a3521, a3522)
    };
    match m3523 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f673(v0: Bytes, v1: Adt2) -> bool {
    let v2: Bytes = f624(v1);
    if f32(v0.clone(), f8(Str::lit("number"))) {
        if f32(v2.clone(), f8(Str::lit("integer"))) {
            true
        } else {
            f32(v2, f8(Str::lit("number")))
        }
    } else {
        f32(v0, v2)
    }
}

pub fn f674(v0: u64, v1: Bytes, v2: u64) -> R<Option<List<u64>>> {
    let m3524 = v0;
    if m3524 == 0 {
        Ok(Some(List::<u64>::nil()))
    } else {
        let v3 = m3524 - 1;
        let a3526 = {
            let a3525 = v1.clone();
            length_bytes(a3525)
        };
        let a3527 = v2;
        if nat_le(a3526, a3527) {
            Ok(Some(List::<u64>::nil()))
        } else {
            let m3528 = f706(v1.clone(), v2)?;
            match m3528 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v4) => {
                    let m3532 = f674(v3, v1, {
                        let a3530 = v2;
                        let a3531 = {
                            let (_, h3529) = v4;
                            h3529
                        };
                        nat_add(a3530, a3531)?
                    })?;
                    match m3532 {
                        None => {
                            Ok(None::<List<u64>>)
                        }
                        Some(v5) => {
                            Ok(Some(List::cons({
                                let (h3533, _) = v4;
                                h3533
                            }, v5)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f675(v0: Bytes) -> R<Result<((bool, bool), List<Adt29>), Bytes>> {
    let v1: u64 = {
        let a3534 = v0.clone();
        length_bytes(a3534)
    };
    let v2: bool = {
        let a3535 = 0u64;
        let a3536 = v1;
        if nat_lt(a3535, a3536) {
            let a3537 = f115(v0.clone(), 0u64)?;
            let a3538 = 94u64;
            nat_eq(a3537, a3538)
        } else {
            false
        }
    };
    let v3: bool = {
        let a3539 = 0u64;
        let a3540 = v1;
        if nat_lt(a3539, a3540) {
            f117({
                let a3543 = f115(v0.clone(), {
                    let a3541 = v1;
                    let a3542 = 1u64;
                    nat_sub(a3541, a3542)
                })?;
                let a3544 = 36u64;
                nat_eq(a3543, a3544)
            }, f707({
                let a3547 = f115(v0.clone(), {
                    let a3545 = v1;
                    let a3546 = 2u64;
                    nat_sub(a3545, a3546)
                })?;
                let a3548 = 92u64;
                nat_eq(a3547, a3548)
            }))
        } else {
            false
        }
    };
    let m3553 = f708({
        let a3549 = v1;
        let a3550 = 1u64;
        nat_add(a3549, a3550)?
    }, v0, if v2 {
        1u64
    } else {
        0u64
    }, if v3 {
        let a3551 = v1;
        let a3552 = 1u64;
        nat_sub(a3551, a3552)
    } else {
        v1
    })?;
    match m3553 {
        Ok(v5) => {
            Ok(Ok::<((bool, bool), List<Adt29>), Bytes>(((v2, v3), v5)))
        }
        Err(v4) => {
            Ok(Err::<((bool, bool), List<Adt29>), Bytes>(v4))
        }
    }
}

pub fn f676(v0: u64, v1: List<Adt29>, v2: List<u64>, v3: bool) -> R<bool> {
    let m3554 = v0;
    if m3554 == 0 {
        Ok(false)
    } else {
        let v4 = m3554 - 1;
        let m3555 = v1.clone();
        match m3555.uncons() {
            None => {
                if v3 {
                    let a3556 = f629(v2)?;
                    let a3557 = 0u64;
                    Ok(nat_eq(a3556, a3557))
                } else {
                    Ok(true)
                }
            }
            Some((v5, v6)) => {
                let m3558 = v5.clone();
                match m3558 {
                    Adt29::C0(v7, v8, v9) => {
                        let a3559 = v9;
                        let a3560 = 1u64;
                        if nat_eq(a3559, a3560) {
                            if f676(v4, v6, v2.clone(), v3)? {
                                Ok(true)
                            } else {
                                let m3561 = v2;
                                match m3561.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v10, v11)) => {
                                        if f709(v5, v10) {
                                            f676(v4, v1, v11, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a3562 = v9;
                            let a3563 = 2u64;
                            if nat_eq(a3562, a3563) {
                                let m3564 = v2;
                                match m3564.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v12, v13)) => {
                                        if f709(v5, v12) {
                                            f676(v4, List::cons(Adt29::C0(v7, v8, 1u64), v6), v13, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            } else {
                                let a3565 = v9;
                                let a3566 = 3u64;
                                if nat_eq(a3565, a3566) {
                                    if f676(v4, v6.clone(), v2.clone(), v3)? {
                                        Ok(true)
                                    } else {
                                        let m3567 = v2;
                                        match m3567.uncons() {
                                            None => {
                                                Ok(false)
                                            }
                                            Some((v14, v15)) => {
                                                if f709(v5, v14) {
                                                    f676(v4, v6, v15, v3)
                                                } else {
                                                    Ok(false)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let m3568 = v2;
                                    match m3568.uncons() {
                                        None => {
                                            Ok(false)
                                        }
                                        Some((v16, v17)) => {
                                            if f709(v5, v16) {
                                                f676(v4, v6, v17, v3)
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

pub fn f677(v0: List<Adt29>) -> R<u64> {
    let m3569 = v0;
    match m3569.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a3570 = f677(v2)?;
            let a3571 = 1u64;
            nat_add(a3570, a3571)
        }
    }
}

pub fn f678(v0: List<Adt29>, v1: List<u64>, v2: bool) -> R<bool> {
    if f676({
        let a3574 = {
            let a3572 = f677(v0.clone())?;
            let a3573 = f629(v1.clone())?;
            nat_add(a3572, a3573)?
        };
        let a3575 = 2u64;
        nat_add(a3574, a3575)?
    }, v0.clone(), v1.clone(), v2)? {
        Ok(true)
    } else {
        f710(v0, v1, v2)
    }
}

pub fn f679(v0: u64) -> R<u64> {
    let m3576 = v0;
    if m3576 == 0 {
        Ok(1u64)
    } else {
        let v1 = m3576 - 1;
        let a3577 = f679(v1)?;
        let a3578 = 10u64;
        nat_mul(a3577, a3578)
    }
}

pub fn f680(p3592: u64, v1: Bytes, p3593: u64, p3594: Bytes) -> R<Bytes> {
    let m3591 = (p3592, (p3593, p3594));
    {
        let mut m3591 = m3591;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3591;
                let m3579 = v0;
                if m3579 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3579 - 1;
                    let m3582 = {
                        let a3580 = v1.clone();
                        let a3581 = v2;
                        index_bytes(a3580, a3581)
                    };
                    match m3582 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3583 = v2;
                                let a3584 = 1u64;
                                nat_add(a3583, a3584)?
                            }, {
                                let a3589 = v3;
                                let a3590 = {
                                    let a3585 = v6;
                                    let a3586 = 126u64;
                                    if nat_eq(a3585, a3586) {
                                        f8(Str::lit("~0"))
                                    } else {
                                        let a3587 = v6;
                                        let a3588 = 47u64;
                                        if nat_eq(a3587, a3588) {
                                            f8(Str::lit("~1"))
                                        } else {
                                            f46(v1.clone(), v2, 1u64)
                                        }
                                    }
                                };
                                append_bytes(a3589, a3590)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f681(v0: Fn22, p3599: u64, p3600: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3598 = (p3599, p3600);
    {
        let mut m3598 = m3598;
        loop {
            let step = {
                let (v1, v2) = m3598;
                let m3595 = v1;
                if m3595 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3595 - 1;
                    let m3597 = {
                        let c3596: Fn22 = v0.clone();
                        c3596.apply(v2.clone())?
                    };
                    match m3597 {
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
                Ok(next) => { m3598 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f682(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f711(v0)
}

pub fn f683(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3605 = {
            let a3603 = {
                let a3601 = v0.clone();
                length_list(a3601)
            };
            let a3604 = {
                let a3602 = v1.clone();
                length_list(a3602)
            };
            nat_add(a3603, a3604)?
        };
        let a3606 = 1u64;
        nat_add(a3605, a3606)?
    };
    Ok(f459({
        let (_, h3607) = {
            let (_, h3608) = {
                let (h3609, _) = f712(Fn23::F713, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3609
            };
            h3608
        };
        h3607
    }))
}

pub fn f684(v0: u64) -> Bytes {
    f46({
        let a3610 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3610)
    }, v0, 1u64)
}

pub fn f685(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3611 = v1;
    match m3611.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3612 = f714(v0, v3.clone())?;
            match m3612 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f546(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f52(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3613 = f685(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3613 {
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

pub fn f686(p3617: List<Bytes>) -> R<bool> {
    let m3616 = p3617;
    {
        let mut m3616 = m3616;
        loop {
            let step = {
                let v0 = m3616;
                let m3614 = v0;
                match m3614.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3615 = f715(v1)?;
                        match m3615 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f52(v3, f716(false, v2.clone())?) {
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
                Ok(next) => { m3616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f687(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m3618 = v0;
    match m3618.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f717(v1.clone(), {
                let (h3619, _) = v2.clone();
                h3619
            }) {
                List::cons({
                    let (h3620, _) = v2;
                    h3620
                }, f687(v3, v1))
            } else {
                f687(v3, v1)
            }
        }
    }
}

pub fn f688(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    let m3621 = v2;
    match m3621.uncons() {
        None => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v4, v5)) => {
            let m3622 = f718(v4.clone())?;
            match m3622 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3623 = f719(v0, v6.clone())?;
                    match m3623 {
                        None => {
                            Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f52(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3626 = f720(v4, {
                                    let a3624 = v1.clone();
                                    let a3625 = f384(v6)?;
                                    append_bytes(a3624, a3625)
                                })?;
                                match m3626 {
                                    Ok(v9) => {
                                        let m3627 = f688(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3627 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f581(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f653(v11.clone())), f537(f580(v9), f654(v11)))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<Adt28, Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<Adt28, Bytes>(v8))
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

pub fn f689(v0: List<Adt2>) -> R<List<Bytes>> {
    let m3628 = v0;
    match m3628.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3629 = f718(v1)?;
            match m3629 {
                None => {
                    f689(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f689(v2)?))
                }
            }
        }
    }
}

pub fn f690(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    f721(f686(f722(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f691(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3630 = f92(Some(v0));
    match m3630 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3631 = f644(v2)?;
            match m3631 {
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

pub fn f692(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt28, Bytes>> {
    let m3632 = v2;
    match m3632.uncons() {
        None => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v5, v6)) => {
            let m3634 = f723(v0, {
                let (h3633, _) = v5.clone();
                h3633
            })?;
            match m3634 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3635, _) = v5;
                        h3635
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f52(v7.clone(), f724(v0, f633(v6.clone()))?) {
                        Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3640 = f657({
                            let (_, h3636) = v5.clone();
                            h3636
                        }, {
                            let a3638 = v1.clone();
                            let a3639 = f384({
                                let (h3637, _) = v5.clone();
                                h3637
                            })?;
                            append_bytes(a3638, a3639)
                        })?;
                        match m3640 {
                            Ok(v9) => {
                                let v10: Bytes = if f52({
                                    let (h3641, _) = v5;
                                    h3641
                                }, v3.clone()) {
                                    f581(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f581(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3642 = f692(v0, v1, v6, v3, v4)?;
                                match m3642 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f653(v12.clone())), f537(f580(v9), f654(v12)))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt28, Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<Adt28, Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f693(v0: Adt2) -> Option<Adt2> {
    let m3643 = f93(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3643 {
        None => {
            f725(v0)
        }
        Some(v1) => {
            if f694(v1.clone()) {
                let m3644 = f725(v0);
                match m3644 {
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

pub fn f694(v0: Adt2) -> bool {
    let m3645 = v0;
    match m3645 {
        Adt2::C0 => {
            false
        }
        Adt2::C1(_, _) => {
            false
        }
        Adt2::C2(v3) => {
            !v3
        }
        Adt2::C3(_, _) => {
            false
        }
        Adt2::C4(_, _, _) => {
            false
        }
        Adt2::C5(_, _) => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            false
        }
        Adt2::C9 => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11(_, _, _) => {
            false
        }
    }
}

pub fn f695(v0: Adt2) -> Option<Adt2> {
    let m3646 = f187(f93(v0, f8(Str::lit("oneOf"))));
    match m3646.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3647 = v2;
            match m3647.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m3648 = v4;
                    match m3648.uncons() {
                        None => {
                            if f726(v3.clone()) {
                                Some(v1)
                            } else {
                                if f726(v1) {
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

pub fn f696(v0: Bytes, v1: Result<Adt27, Bytes>) -> Result<Adt27, Bytes> {
    let m3649 = v1;
    match m3649 {
        Ok(v3) => {
            f648(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f581(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f580(v3))
        }
        Err(v2) => {
            Err::<Adt27, Bytes>(v2)
        }
    }
}

pub fn f697(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3650 = f695(v0.clone());
    match m3650 {
        None => {
            if f32(f58(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3651 = f93(v0, f8(Str::lit("items")));
                match m3651 {
                    None => {
                        Ok(f696(f8(Str::lit("List")), f698()))
                    }
                    Some(v3) => {
                        Ok(f696(f8(Str::lit("List")), f720(v3, {
                            let a3652 = v1;
                            let a3653 = f8(Str::lit("Item"));
                            append_bytes(a3652, a3653)
                        })?))
                    }
                }
            } else {
                f720(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f696(f8(Str::lit("Option")), f720(v2, v1)?))
        }
    }
}

pub fn f698() -> Result<Adt27, Bytes> {
    Ok::<Adt27, Bytes>(Adt27::C0(f577(), List::cons(Adt24::C2, List::<Adt24>::nil())))
}

pub fn f699(p3657: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m3656 = p3657;
    {
        let mut m3656 = m3656;
        loop {
            let step = {
                let v0 = m3656;
                let m3654 = v0;
                match m3654.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f52({
                            let (h3655, _) = v2;
                            h3655
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f700(p3661: List<Adt2>, v1: Bytes, p3662: List<Bytes>) -> bool {
    let m3660 = (p3661, p3662);
    {
        let mut m3660 = m3660;
        loop {
            let step = {
                let (v0, v2) = m3660;
                let m3658 = v0;
                match m3658.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3659 = f727(v3, v1.clone());
                        match m3659 {
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
                Ok(next) => { m3660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f701(p3665: List<Adt2>, p3666: List<Bytes>) -> bool {
    let m3664 = (p3665, p3666);
    {
        let mut m3664 = m3664;
        loop {
            let step = {
                let (v0, v1) = m3664;
                let m3663 = v0;
                match m3663.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f58(v2, f8(Str::lit("type")));
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
                Ok(next) => { m3664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f702(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m3667 = f93(v0.clone(), f8(Str::lit("anyOf")));
        match m3667 {
            None => {
                Adt2::C9
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f699(f573(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f658(f304(v2))
    } else {
        false
    }
}

pub fn f703(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3668 = f540(v0, v1);
    let a3669 = f672(v0, v2);
    fixed_u32::bit_or(a3668, a3669)
}

pub fn f704(v0: u32) -> u32 {
    let a3672 = {
        let a3670 = f703(v0, 17u32, 15u32);
        let a3671 = f703(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3670, a3671)
    };
    let a3673 = f540(v0, 10u32);
    fixed_u32::bit_xor(a3672, a3673)
}

pub fn f705(v0: u32) -> u32 {
    let a3676 = {
        let a3674 = f703(v0, 7u32, 25u32);
        let a3675 = f703(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3674, a3675)
    };
    let a3677 = f540(v0, 3u32);
    fixed_u32::bit_xor(a3676, a3677)
}

pub fn f706(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3680 = {
        let a3678 = v0.clone();
        let a3679 = v1;
        index_bytes(a3678, a3679)
    };
    match m3680 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            let a3681 = v3;
            let a3682 = 128u64;
            if nat_lt(a3681, a3682) {
                Ok(Some((v3, 1u64)))
            } else {
                if f117({
                    let a3683 = 194u64;
                    let a3684 = v3;
                    nat_le(a3683, a3684)
                }, {
                    let a3685 = v3;
                    let a3686 = 223u64;
                    nat_le(a3685, a3686)
                }) {
                    let m3689 = f728(v0, {
                        let a3687 = v1;
                        let a3688 = 1u64;
                        nat_add(a3687, a3688)?
                    })?;
                    match m3689 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f729({
                                let a3694 = {
                                    let a3692 = {
                                        let a3690 = v3;
                                        let a3691 = 192u64;
                                        nat_sub(a3690, a3691)
                                    };
                                    let a3693 = 64u64;
                                    nat_mul(a3692, a3693)?
                                };
                                let a3695 = v4;
                                nat_add(a3694, a3695)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f117({
                        let a3696 = 224u64;
                        let a3697 = v3;
                        nat_le(a3696, a3697)
                    }, {
                        let a3698 = v3;
                        let a3699 = 239u64;
                        nat_le(a3698, a3699)
                    }) {
                        let m3702 = f728(v0.clone(), {
                            let a3700 = v1;
                            let a3701 = 1u64;
                            nat_add(a3700, a3701)?
                        })?;
                        match m3702 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3705 = f728(v0, {
                                    let a3703 = v1;
                                    let a3704 = 2u64;
                                    nat_add(a3703, a3704)?
                                })?;
                                match m3705 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f729({
                                            let a3714 = {
                                                let a3708 = {
                                                    let a3706 = v3;
                                                    let a3707 = 224u64;
                                                    nat_sub(a3706, a3707)
                                                };
                                                let a3709 = 4096u64;
                                                nat_mul(a3708, a3709)?
                                            };
                                            let a3715 = {
                                                let a3712 = {
                                                    let a3710 = v5;
                                                    let a3711 = 64u64;
                                                    nat_mul(a3710, a3711)?
                                                };
                                                let a3713 = v6;
                                                nat_add(a3712, a3713)?
                                            };
                                            nat_add(a3714, a3715)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f117({
                            let a3716 = 240u64;
                            let a3717 = v3;
                            nat_le(a3716, a3717)
                        }, {
                            let a3718 = v3;
                            let a3719 = 244u64;
                            nat_le(a3718, a3719)
                        }) {
                            let m3722 = f728(v0.clone(), {
                                let a3720 = v1;
                                let a3721 = 1u64;
                                nat_add(a3720, a3721)?
                            })?;
                            match m3722 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3725 = f728(v0.clone(), {
                                        let a3723 = v1;
                                        let a3724 = 2u64;
                                        nat_add(a3723, a3724)?
                                    })?;
                                    match m3725 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3728 = f728(v0, {
                                                let a3726 = v1;
                                                let a3727 = 3u64;
                                                nat_add(a3726, a3727)?
                                            })?;
                                            match m3728 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f729({
                                                        let a3741 = {
                                                            let a3731 = {
                                                                let a3729 = v3;
                                                                let a3730 = 240u64;
                                                                nat_sub(a3729, a3730)
                                                            };
                                                            let a3732 = 262144u64;
                                                            nat_mul(a3731, a3732)?
                                                        };
                                                        let a3742 = {
                                                            let a3739 = {
                                                                let a3733 = v7;
                                                                let a3734 = 4096u64;
                                                                nat_mul(a3733, a3734)?
                                                            };
                                                            let a3740 = {
                                                                let a3737 = {
                                                                    let a3735 = v8;
                                                                    let a3736 = 64u64;
                                                                    nat_mul(a3735, a3736)?
                                                                };
                                                                let a3738 = v9;
                                                                nat_add(a3737, a3738)?
                                                            };
                                                            nat_add(a3739, a3740)?
                                                        };
                                                        nat_add(a3741, a3742)?
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

pub fn f707(v0: bool) -> bool {
    !v0
}

pub fn f708(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Result<List<Adt29>, Bytes>> {
    let m3743 = v0;
    if m3743 == 0 {
        Ok(Ok::<List<Adt29>, Bytes>(List::<Adt29>::nil()))
    } else {
        let v4 = m3743 - 1;
        let a3744 = v3;
        let a3745 = v2;
        if nat_le(a3744, a3745) {
            Ok(Ok::<List<Adt29>, Bytes>(List::<Adt29>::nil()))
        } else {
            let v5: u64 = f115(v1.clone(), v2)?;
            let m3794 = {
                let a3746 = v5;
                let a3747 = 91u64;
                if nat_eq(a3746, a3747) {
                    let a3750 = f115(v1.clone(), {
                        let a3748 = v2;
                        let a3749 = 1u64;
                        nat_add(a3748, a3749)?
                    })?;
                    let a3751 = 94u64;
                    if nat_eq(a3750, a3751) {
                        let m3755 = f730({
                            let a3752 = v1.clone();
                            length_bytes(a3752)
                        }, v1.clone(), {
                            let a3753 = v2;
                            let a3754 = 2u64;
                            nat_add(a3753, a3754)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3755 {
                            Ok(v7) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, {
                                    let (h3756, _) = v7;
                                    h3756
                                }), {
                                    let (_, h3757) = v7;
                                    h3757
                                }))
                            }
                            Err(v6) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v6)
                            }
                        }
                    } else {
                        let m3761 = f730({
                            let a3758 = v1.clone();
                            length_bytes(a3758)
                        }, v1.clone(), {
                            let a3759 = v2;
                            let a3760 = 1u64;
                            nat_add(a3759, a3760)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3761 {
                            Ok(v9) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, {
                                    let (h3762, _) = v9;
                                    h3762
                                }), {
                                    let (_, h3763) = v9;
                                    h3763
                                }))
                            }
                            Err(v8) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v8)
                            }
                        }
                    }
                } else {
                    let a3764 = v5;
                    let a3765 = 46u64;
                    if nat_eq(a3764, a3765) {
                        Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, List::cons((10u64, 10u64), List::cons((13u64, 13u64), List::cons((8232u64, 8233u64), List::<(u64, u64)>::nil())))), {
                            let a3766 = v2;
                            let a3767 = 1u64;
                            nat_add(a3766, a3767)?
                        }))
                    } else {
                        let a3768 = v5;
                        let a3769 = 40u64;
                        if if if if if if nat_eq(a3768, a3769) {
                            true
                        } else {
                            let a3770 = v5;
                            let a3771 = 41u64;
                            nat_eq(a3770, a3771)
                        } {
                            true
                        } else {
                            let a3772 = v5;
                            let a3773 = 124u64;
                            if nat_eq(a3772, a3773) {
                                true
                            } else {
                                let a3774 = v5;
                                let a3775 = 123u64;
                                nat_eq(a3774, a3775)
                            }
                        } {
                            true
                        } else {
                            let a3776 = v5;
                            let a3777 = 125u64;
                            if if nat_eq(a3776, a3777) {
                                true
                            } else {
                                let a3778 = v5;
                                let a3779 = 92u64;
                                nat_eq(a3778, a3779)
                            } {
                                true
                            } else {
                                let a3780 = v5;
                                let a3781 = 94u64;
                                if nat_eq(a3780, a3781) {
                                    true
                                } else {
                                    let a3782 = v5;
                                    let a3783 = 36u64;
                                    nat_eq(a3782, a3783)
                                }
                            }
                        } {
                            true
                        } else {
                            let a3784 = v5;
                            let a3785 = 42u64;
                            if if nat_eq(a3784, a3785) {
                                true
                            } else {
                                let a3786 = v5;
                                let a3787 = 43u64;
                                nat_eq(a3786, a3787)
                            } {
                                true
                            } else {
                                let a3788 = v5;
                                let a3789 = 63u64;
                                nat_eq(a3788, a3789)
                            }
                        } {
                            true
                        } else {
                            let a3790 = 127u64;
                            let a3791 = v5;
                            nat_lt(a3790, a3791)
                        } {
                            Err::<((bool, List<(u64, u64)>), u64), Bytes>(f7(List::cons(f8(Str::lit("the pattern construct ")), List::cons(f46(v1.clone(), v2, 1u64), List::cons(f8(Str::lit(" is not evaluated: only ^, $, ., character classes, literal ASCII characters and the quantifiers *, + and ? are (ECMA-262 section 22.2.1)")), List::<Bytes>::nil())))))
                        } else {
                            Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, List::cons((v5, v5), List::<(u64, u64)>::nil())), {
                                let a3792 = v2;
                                let a3793 = 1u64;
                                nat_add(a3792, a3793)?
                            }))
                        }
                    }
                }
            };
            match m3794 {
                Ok(v11) => {
                    let v12: u64 = {
                        let (_, h3795) = v11.clone();
                        h3795
                    };
                    let v13: u64 = {
                        let a3796 = v12;
                        let a3797 = v3;
                        if nat_lt(a3796, a3797) {
                            f731(f115(v1.clone(), v12)?)
                        } else {
                            0u64
                        }
                    };
                    let a3798 = v12;
                    let a3799 = v3;
                    if if nat_lt(a3798, a3799) {
                        let a3800 = f115(v1.clone(), v12)?;
                        let a3801 = 123u64;
                        nat_eq(a3800, a3801)
                    } else {
                        false
                    } {
                        Ok(Err::<List<Adt29>, Bytes>(f8(Str::lit("a {n,m} quantifier in a pattern is not evaluated (ECMA-262 section 22.2.1)"))))
                    } else {
                        let m3806 = f708(v4, v1, {
                            let a3802 = 0u64;
                            let a3803 = v13;
                            if nat_lt(a3802, a3803) {
                                let a3804 = v12;
                                let a3805 = 1u64;
                                nat_add(a3804, a3805)?
                            } else {
                                v12
                            }
                        }, v3)?;
                        match m3806 {
                            Ok(v15) => {
                                Ok(Ok::<List<Adt29>, Bytes>(List::cons(Adt29::C0({
                                    let (h3807, _) = {
                                        let (h3808, _) = v11.clone();
                                        h3808
                                    };
                                    h3807
                                }, {
                                    let (_, h3809) = {
                                        let (h3810, _) = v11;
                                        h3810
                                    };
                                    h3809
                                }, v13), v15)))
                            }
                            Err(v14) => {
                                Ok(Err::<List<Adt29>, Bytes>(v14))
                            }
                        }
                    }
                }
                Err(v10) => {
                    Ok(Err::<List<Adt29>, Bytes>(v10))
                }
            }
        }
    }
}

pub fn f709(v0: Adt29, v1: u64) -> bool {
    let m3811 = v0;
    match m3811 {
        Adt29::C0(v2, v3, _) => {
            if v2 {
                f707(f732(v3, v1))
            } else {
                f732(v3, v1)
            }
        }
    }
}

pub fn f710(v0: List<Adt29>, p3818: List<u64>, v2: bool) -> R<bool> {
    let m3817 = p3818;
    {
        let mut m3817 = m3817;
        loop {
            let step = {
                let v1 = m3817;
                let m3812 = v1;
                match m3812.uncons() {
                    None => {
                        Err::<List<u64>, R<bool>>(Ok(false))
                    }
                    Some((_, v4)) => {
                        if f676({
                            let a3815 = {
                                let a3813 = f677(v0.clone())?;
                                let a3814 = f629(v4.clone())?;
                                nat_add(a3813, a3814)?
                            };
                            let a3816 = 2u64;
                            nat_add(a3815, a3816)?
                        }, v0.clone(), v4.clone(), v2)? {
                            Err::<List<u64>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<u64>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f711(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3819, _) = v0;
        h3819
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3820) = v0;
        h3820
    };
    let m3821 = v1;
    match m3821.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3822 = v4;
            match m3822.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f733(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f712(v0: Fn23, p3827: u64, p3828: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3826 = (p3827, p3828);
    {
        let mut m3826 = m3826;
        loop {
            let step = {
                let (v1, v2) = m3826;
                let m3823 = v1;
                if m3823 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3823 - 1;
                    let m3825 = {
                        let c3824: Fn23 = v0.clone();
                        c3824.apply(v2.clone())
                    };
                    match m3825 {
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
                Ok(next) => { m3826 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f713(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f734(v0)
}

pub fn f714(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3829 = v1;
    match m3829 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C3(v9, v10) => {
            Ok(Some({
                let a3840 = if f117(v9, {
                    let a3838 = 0u64;
                    let a3839 = v10;
                    nat_lt(a3838, a3839)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3841 = f306(false, v10)?;
                append_bytes(a3840, a3841)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3832 = f735({
                let a3830 = f8(Str::lit("n"));
                let a3831 = f307(false, v4, v5)?;
                append_bytes(a3830, a3831)
            })?;
            match m3832 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3836 = f8(Str::lit("nMinus"));
                        let a3837 = f46(v6.clone(), 1u64, {
                            let a3834 = {
                                let a3833 = v6;
                                length_bytes(a3833)
                            };
                            let a3835 = 1u64;
                            nat_sub(a3834, a3835)
                        });
                        append_bytes(a3836, a3837)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C6(v2) => {
            f719(v0, v2)
        }
        Adt2::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C8(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C9 => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C11(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f715(v0: Bytes) -> R<Option<Bytes>> {
    let m3842 = f736(v0)?;
    match m3842 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f737(v1)?))
        }
    }
}

pub fn f716(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3843 = v1;
    match m3843.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3844 = f719(v0, v2)?;
            match m3844 {
                None => {
                    f716(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f716(v0, v3)?))
                }
            }
        }
    }
}

pub fn f717(p3848: List<Adt2>, v1: Bytes) -> bool {
    let m3847 = p3848;
    {
        let mut m3847 = m3847;
        loop {
            let step = {
                let v0 = m3847;
                let m3845 = v0;
                match m3845.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3846 = f727(v2, v1.clone());
                        if match m3846 {
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
                Ok(next) => { m3847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f718(v0: Adt2) -> R<Option<Bytes>> {
    let m3849 = f92(f93(v0.clone(), f8(Str::lit("$ref"))));
    match m3849 {
        None => {
            Ok(f92(f93(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f644(v1)
        }
    }
}

pub fn f719(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f735(v1)
    } else {
        f715(v1)
    }
}

pub fn f720(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    f738(f615(v0), v1)
}

pub fn f721(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    let m3850 = v2;
    match m3850.uncons() {
        None => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v5, v6)) => {
            let m3851 = f727(v5.clone(), v3.clone());
            match m3851 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3852 = f719(v0, v7.clone())?;
                    match m3852 {
                        None => {
                            Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f52(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3855 = f651({
                                    let a3853 = v1.clone();
                                    let a3854 = f384(v7)?;
                                    append_bytes(a3853, a3854)
                                }, f575(f533(f93(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f214(f187(f93(v5, f8(Str::lit("required"))))), true)?;
                                match m3855 {
                                    Ok(v10) => {
                                        let m3856 = f721(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3856 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f397(f653(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f653(v12.clone())), f537(f654(v10), f654(v12)))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt28, Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<Adt28, Bytes>(v9))
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

pub fn f722(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3857 = v0;
    match m3857.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3858 = f727(v2, v1.clone());
            match m3858 {
                None => {
                    f722(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f722(v3, v1))
                }
            }
        }
    }
}

pub fn f723(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3859 = f719(v0, v1)?;
    match m3859 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f32(v2.clone(), f8(Str::lit("mk"))) {
                f613(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f724(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3860 = v1;
    match m3860.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3861 = f723(v0, v2)?;
            match m3861 {
                None => {
                    f724(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f724(v0, v3)?))
                }
            }
        }
    }
}

pub fn f725(v0: Adt2) -> Option<Adt2> {
    let m3862 = f533(f93(v0, f8(Str::lit("patternProperties"))));
    match m3862.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3863 = v2;
            match m3863.uncons() {
                None => {
                    Some({
                        let (_, h3864) = v1;
                        h3864
                    })
                }
                Some((_, _)) => {
                    None::<Adt2>
                }
            }
        }
    }
}

pub fn f726(v0: Adt2) -> bool {
    f32(f58(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f727(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f52(v1.clone(), f214(f187(f93(v0.clone(), f8(Str::lit("required")))))) {
        let m3865 = f93(v0, f8(Str::lit("properties")));
        match m3865 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3866 = f93(v2, v1);
                match m3866 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f92(f93(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f728(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3869 = {
        let a3867 = v0;
        let a3868 = v1;
        index_bytes(a3867, a3868)
    };
    match m3869 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f117({
                let a3870 = 128u64;
                let a3871 = v3;
                nat_le(a3870, a3871)
            }, {
                let a3872 = v3;
                let a3873 = 191u64;
                nat_le(a3872, a3873)
            }) {
                Ok(Some({
                    let a3874 = v3;
                    let a3875 = 128u64;
                    nat_sub(a3874, a3875)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f729(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f117({
        let a3876 = v1;
        let a3877 = v0;
        nat_le(a3876, a3877)
    }, f117({
        let a3878 = v0;
        let a3879 = v2;
        nat_le(a3878, a3879)
    }, f202({
        let a3880 = v0;
        let a3881 = 55296u64;
        nat_lt(a3880, a3881)
    }, {
        let a3882 = 57343u64;
        let a3883 = v0;
        nat_lt(a3882, a3883)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f730(p3920: u64, v1: Bytes, p3921: u64, p3922: List<(u64, u64)>) -> R<Result<(List<(u64, u64)>, u64), Bytes>> {
    let m3919 = (p3920, (p3921, p3922));
    {
        let mut m3919 = m3919;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3919;
                let m3884 = v0;
                if m3884 == 0 {
                    Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                } else {
                    let v4 = m3884 - 1;
                    let v5: u64 = f115(v1.clone(), v2)?;
                    let a3886 = {
                        let a3885 = v1.clone();
                        length_bytes(a3885)
                    };
                    let a3887 = v2;
                    if nat_le(a3886, a3887) {
                        Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                    } else {
                        let a3888 = v5;
                        let a3889 = 93u64;
                        if nat_eq(a3888, a3889) {
                            Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Ok::<(List<(u64, u64)>, u64), Bytes>((v3, {
                                let a3890 = v2;
                                let a3891 = 1u64;
                                nat_add(a3890, a3891)?
                            }))))
                        } else {
                            let a3892 = v5;
                            let a3893 = 92u64;
                            if if nat_eq(a3892, a3893) {
                                true
                            } else {
                                let a3894 = 127u64;
                                let a3895 = v5;
                                nat_lt(a3894, a3895)
                            } {
                                Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("an escape or a non-ASCII character in a pattern's character class is not evaluated (ECMA-262 section 22.2.1)")))))
                            } else {
                                let a3898 = f115(v1.clone(), {
                                    let a3896 = v2;
                                    let a3897 = 1u64;
                                    nat_add(a3896, a3897)?
                                })?;
                                let a3899 = 45u64;
                                if if nat_eq(a3898, a3899) {
                                    f117({
                                        let a3903 = {
                                            let a3900 = v2;
                                            let a3901 = 2u64;
                                            nat_add(a3900, a3901)?
                                        };
                                        let a3904 = {
                                            let a3902 = v1.clone();
                                            length_bytes(a3902)
                                        };
                                        nat_lt(a3903, a3904)
                                    }, f117(f707({
                                        let a3907 = f115(v1.clone(), {
                                            let a3905 = v2;
                                            let a3906 = 2u64;
                                            nat_add(a3905, a3906)?
                                        })?;
                                        let a3908 = 93u64;
                                        nat_eq(a3907, a3908)
                                    }), f707({
                                        let a3911 = f115(v1.clone(), {
                                            let a3909 = v2;
                                            let a3910 = 2u64;
                                            nat_add(a3909, a3910)?
                                        })?;
                                        let a3912 = 92u64;
                                        nat_eq(a3911, a3912)
                                    })))
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3913 = v2;
                                        let a3914 = 3u64;
                                        nat_add(a3913, a3914)?
                                    }, List::cons((v5, f115(v1.clone(), {
                                        let a3915 = v2;
                                        let a3916 = 2u64;
                                        nat_add(a3915, a3916)?
                                    })?), v3))))
                                } else {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3917 = v2;
                                        let a3918 = 1u64;
                                        nat_add(a3917, a3918)?
                                    }, List::cons((v5, v5), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3919 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f731(v0: u64) -> u64 {
    let a3923 = v0;
    let a3924 = 42u64;
    if nat_eq(a3923, a3924) {
        1u64
    } else {
        let a3925 = v0;
        let a3926 = 43u64;
        if nat_eq(a3925, a3926) {
            2u64
        } else {
            let a3927 = v0;
            let a3928 = 63u64;
            if nat_eq(a3927, a3928) {
                3u64
            } else {
                0u64
            }
        }
    }
}

pub fn f732(p3937: List<(u64, u64)>, v1: u64) -> bool {
    let m3936 = p3937;
    {
        let mut m3936 = m3936;
        loop {
            let step = {
                let v0 = m3936;
                let m3929 = v0;
                match m3929.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3931 = {
                            let (h3930, _) = v2;
                            h3930
                        };
                        let a3932 = v1;
                        if if nat_le(a3931, a3932) {
                            let a3934 = v1;
                            let a3935 = {
                                let (_, h3933) = v2;
                                h3933
                            };
                            nat_le(a3934, a3935)
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
                Ok(next) => { m3936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f733(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3942 = {
            let a3940 = {
                let a3938 = v0.clone();
                length_list(a3938)
            };
            let a3941 = {
                let a3939 = v1.clone();
                length_list(a3939)
            };
            nat_add(a3940, a3941)?
        };
        let a3943 = 1u64;
        nat_add(a3942, a3943)?
    };
    Ok(f739({
        let (_, h3944) = {
            let (_, h3945) = {
                let (h3946, _) = f740(Fn24::F741, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3946
            };
            h3945
        };
        h3944
    }))
}

pub fn f734(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3947, _) = v0.clone();
        h3947
    };
    let v2: List<(Bytes, ())> = {
        let (h3948, _) = {
            let (_, h3949) = v0.clone();
            h3949
        };
        h3948
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3950) = {
            let (_, h3951) = v0;
            h3951
        };
        h3950
    };
    let m3952 = v1.clone();
    match m3952.uncons() {
        None => {
            let m3953 = v2;
            match m3953.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3954 = v2.clone();
            match m3954.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f742({
                        let (h3955, _) = v8.clone();
                        h3955
                    }, {
                        let (h3956, _) = v6.clone();
                        h3956
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

pub fn f735(v0: Bytes) -> R<Option<Bytes>> {
    let m3958 = f743({
        let a3957 = v0.clone();
        length_bytes(a3957)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3958 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3960 = {
                let a3959 = v1.clone();
                length_bytes(a3959)
            };
            let a3961 = 0u64;
            if nat_eq(a3960, a3961) {
                let a3963 = {
                    let a3962 = v0;
                    length_bytes(a3962)
                };
                let a3964 = 0u64;
                if nat_eq(a3963, a3964) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f737(v1)?))
            }
        }
    }
}

pub fn f736(v0: Bytes) -> R<Option<Bytes>> {
    if f744({
        let a3965 = v0.clone();
        length_bytes(a3965)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f745(v0)?;
        if f746(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3968 = {
                let a3966 = v1.clone();
                let a3967 = 0u64;
                index_bytes(a3966, a3967)
            };
            match m3968 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f531(f72(v2)?) {
                        Ok(Some({
                            let a3969 = f8(Str::lit("n"));
                            let a3970 = v1;
                            append_bytes(a3969, a3970)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f747({
            let a3971 = v0.clone();
            length_bytes(a3971)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f737(v0: Bytes) -> R<Bytes> {
    if f52(v0.clone(), f748()) {
        f613(v0)
    } else {
        Ok(v0)
    }
}

pub fn f738(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3972 = v0.clone();
    match m3972 {
        Adt2::C0 => {
            Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C1(_, _) => {
            Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(v4) => {
            if v4 {
                Ok(f698())
            } else {
                Ok(f647(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C3(_, _) => {
            Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_, _, _) => {
            Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_, _) => {
            Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C6(_) => {
            Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C7(_) => {
            Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C8(_) => {
            let m3973 = f749(f573(v0.clone()));
            match m3973 {
                None => {
                    let m3974 = f92(f93(v0.clone(), f8(Str::lit("$ref"))));
                    match m3974 {
                        None => {
                            let m3976 = f93(v0.clone(), f8(Str::lit("allOf")));
                            match m3976 {
                                None => {
                                    let m3981 = f93(v0.clone(), f8(Str::lit("enum")));
                                    match m3981 {
                                        None => {
                                            let m3982 = f93(v0.clone(), f8(Str::lit("const")));
                                            match m3982 {
                                                None => {
                                                    let m3984 = f93(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3984 {
                                                        None => {
                                                            let v34: Bytes = f58(v0.clone(), f8(Str::lit("type")));
                                                            if if f32(v34.clone(), Bytes::lit(&[])) {
                                                                !f750(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f698())
                                                            } else {
                                                                if f32(v34.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f648(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                                                } else {
                                                                    if f32(v34.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f648(f751(v0), List::<Adt24>::nil()))
                                                                    } else {
                                                                        if f32(v34.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f648(f752(), List::<Adt24>::nil()))
                                                                        } else {
                                                                            if f32(v34.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f648(f8(Str::lit("Bool")), List::<Adt24>::nil()))
                                                                            } else {
                                                                                if f32(v34.clone(), f8(Str::lit("object"))) {
                                                                                    let m3985 = f533(f93(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3985.uncons() {
                                                                                        None => {
                                                                                            if f655(v0.clone()) {
                                                                                                Ok(f648(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                                                            } else {
                                                                                                let m3986 = f693(v0);
                                                                                                match m3986 {
                                                                                                    None => {
                                                                                                        Ok(f696(f8(Str::lit("List")), f753(f698())))
                                                                                                    }
                                                                                                    Some(v37) => {
                                                                                                        Ok(f696(f8(Str::lit("List")), f753(f754(v37, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f648(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f647(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v34, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f648(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v31) => {
                                                    let m3983 = f92(Some(v31));
                                                    match m3983 {
                                                        None => {
                                                            Ok(f647(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f648(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f648(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                        }
                                    }
                                }
                                Some(v23) => {
                                    let m3977 = f304(v23);
                                    match m3977.uncons() {
                                        None => {
                                            Ok(f647(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v24, v25)) => {
                                            let m3978 = v25;
                                            match m3978.uncons() {
                                                None => {
                                                    let m3979 = f92(f93(v24, f8(Str::lit("$ref"))));
                                                    match m3979 {
                                                        None => {
                                                            Ok(f647(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v28) => {
                                                            let m3980 = f644(v28)?;
                                                            match m3980 {
                                                                None => {
                                                                    Ok(f647(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v29) => {
                                                                    Ok(f648(v29, List::<Adt24>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f647(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v21) => {
                            let m3975 = f644(v21)?;
                            match m3975 {
                                None => {
                                    Ok(f647(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v22) => {
                                    Ok(f648(v22, List::<Adt24>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v20) => {
                    Ok(f647(if f32(v20.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v20, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C9 => {
            Ok(f698())
        }
        Adt2::C10(_, _) => {
            Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C11(_, _, _) => {
            Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f739(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f417(Fn5::F755, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f740(v0: Fn24, p3991: u64, p3992: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3990 = (p3991, p3992);
    {
        let mut m3990 = m3990;
        loop {
            let step = {
                let (v1, v2) = m3990;
                let m3987 = v1;
                if m3987 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3987 - 1;
                    let m3989 = {
                        let c3988: Fn24 = v0.clone();
                        c3988.apply(v2.clone())
                    };
                    match m3989 {
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
                Ok(next) => { m3990 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f741(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f756(v0)
}

pub fn f742(v0: Bytes, v1: Bytes) -> bool {
    let a3997 = {
        let a3993 = v0;
        let a3994 = v1;
        compare_bytes(a3993, a3994)
    };
    let a3998 = {
        let a3995 = Bytes::lit(&[]);
        let a3996 = Bytes::lit(&[0x01]);
        compare_bytes(a3995, a3996)
    };
    equal(a3997, a3998)
}

pub fn f743(p4011: u64, v1: Bytes, p4012: u64, p4013: Bytes) -> R<Option<Bytes>> {
    let m4010 = (p4011, (p4012, p4013));
    {
        let mut m4010 = m4010;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4010;
                let m3999 = v0;
                if m3999 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3999 - 1;
                    let a4001 = {
                        let a4000 = v1.clone();
                        length_bytes(a4000)
                    };
                    let a4002 = v2;
                    if nat_le(a4001, a4002) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4003 = f706(v1.clone(), v2)?;
                        match m4003 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4005 = v2;
                                    let a4006 = {
                                        let (_, h4004) = v5;
                                        h4004
                                    };
                                    nat_add(a4005, a4006)?
                                }, {
                                    let a4008 = v3;
                                    let a4009 = f757(v1.clone(), v2, {
                                        let (h4007, _) = v5;
                                        h4007
                                    });
                                    append_bytes(a4008, a4009)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f744(p4023: u64, v1: Bytes, p4024: u64) -> R<bool> {
    let m4022 = (p4023, p4024);
    {
        let mut m4022 = m4022;
        loop {
            let step = {
                let (v0, v2) = m4022;
                let m4014 = v0;
                if m4014 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m4014 - 1;
                    let m4017 = {
                        let a4015 = v1.clone();
                        let a4016 = v2;
                        index_bytes(a4015, a4016)
                    };
                    match m4017 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a4018 = f72(v4)?;
                            let a4019 = 128u64;
                            if nat_lt(a4018, a4019) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a4020 = v2;
                                    let a4021 = 1u64;
                                    nat_add(a4020, a4021)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4022 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f745(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f476(v0.clone(), 0u64, {
        let a4025 = v0;
        length_bytes(a4025)
    }, false, Bytes::lit(&[]))?;
    let m4028 = {
        let a4026 = v1.clone();
        let a4027 = 0u64;
        index_bytes(a4026, a4027)
    };
    match m4028 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f530(v3) {
                let a4035 = f46({
                    let a4029 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a4029)
                }, {
                    let a4030 = v3;
                    let a4031 = 65u64;
                    nat_sub(a4030, a4031)
                }, 1u64);
                let a4036 = f46(v1.clone(), 1u64, {
                    let a4033 = {
                        let a4032 = v1;
                        length_bytes(a4032)
                    };
                    let a4034 = 1u64;
                    nat_sub(a4033, a4034)
                });
                Ok(append_bytes(a4035, a4036))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f746(v0: Bytes) -> R<bool> {
    let m4039 = {
        let a4037 = v0;
        let a4038 = 0u64;
        index_bytes(a4037, a4038)
    };
    match m4039 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f202(f529(f72(v1)?), f530(f72(v1)?)))
        }
    }
}

pub fn f747(p4052: u64, v1: Bytes, p4053: u64, p4054: Bytes) -> R<Option<Bytes>> {
    let m4051 = (p4052, (p4053, p4054));
    {
        let mut m4051 = m4051;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4051;
                let m4040 = v0;
                if m4040 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4040 - 1;
                    let a4042 = {
                        let a4041 = v1.clone();
                        length_bytes(a4041)
                    };
                    let a4043 = v2;
                    if nat_le(a4042, a4043) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4044 = f706(v1.clone(), v2)?;
                        match m4044 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4046 = v2;
                                    let a4047 = {
                                        let (_, h4045) = v5;
                                        h4045
                                    };
                                    nat_add(a4046, a4047)?
                                }, {
                                    let a4049 = v3;
                                    let a4050 = f758({
                                        let (h4048, _) = v5;
                                        h4048
                                    });
                                    append_bytes(a4049, a4050)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4051 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f748() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f749(p4059: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m4058 = p4059;
    {
        let mut m4058 = m4058;
        loop {
            let step = {
                let v0 = m4058;
                let m4055 = v0;
                match m4055.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h4056, _) = v1.clone();
                            h4056
                        }, f759()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h4057, _) = v1;
                                h4057
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
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

pub fn f750(v0: Adt2) -> bool {
    if f660(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f660(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f660(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f660(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f660(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f660(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f751(v0: Adt2) -> Bytes {
    let m4060 = f517(f93(v0, f8(Str::lit("minimum"))));
    match m4060 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f752() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f753(v0: Result<Adt27, Bytes>) -> Result<Adt27, Bytes> {
    let m4061 = v0;
    match m4061 {
        Ok(v2) => {
            f648(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f581(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f580(v2))
        }
        Err(v1) => {
            Err::<Adt27, Bytes>(v1)
        }
    }
}

pub fn f754(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    f760(16u64, v0, v1)
}

pub fn f755(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f756(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4062, _) = v0.clone();
        h4062
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4063, _) = {
            let (_, h4064) = v0.clone();
            h4064
        };
        h4063
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4065) = {
            let (_, h4066) = v0;
            h4066
        };
        h4065
    };
    let m4067 = v1.clone();
    match m4067.uncons() {
        None => {
            let m4068 = v2;
            match m4068.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4069 = v2.clone();
            match m4069.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f742({
                        let (h4070, _) = v8.clone();
                        h4070
                    }, {
                        let (h4071, _) = v6.clone();
                        h4071
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

pub fn f757(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f202(f117(f202(f529(v2), f530(v2)), f117({
        let a4072 = v2;
        let a4073 = 117u64;
        !nat_eq(a4072, a4073)
    }, {
        let a4074 = v2;
        let a4075 = 85u64;
        !nat_eq(a4074, a4075)
    })), f117(f531(v2), {
        let a4076 = 0u64;
        let a4077 = v1;
        nat_lt(a4076, a4077)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a4078 = v2;
        let a4079 = 65536u64;
        if nat_lt(a4078, a4079) {
            let a4080 = f8(Str::lit("u"));
            let a4081 = f645(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4080, a4081)
        } else {
            let a4082 = f8(Str::lit("U"));
            let a4083 = f645(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4082, a4083)
        }
    }
}

pub fn f758(v0: u64) -> Bytes {
    let a4084 = f8(Str::lit("u"));
    let a4085 = f645(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4084, a4085)
}

pub fn f759() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f760(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt27, Bytes>> {
    let m4086 = v0;
    if m4086 == 0 {
        Ok(f647(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4086 - 1;
        let m4087 = v1.clone();
        match m4087 {
            Adt2::C0 => {
                Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C1(_, _) => {
                Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(f698())
                } else {
                    Ok(f647(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C3(_, _) => {
                Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_, _, _) => {
                Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_, _) => {
                Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C6(_) => {
                Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C7(_) => {
                Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C8(_) => {
                let m4088 = f92(f93(v1.clone(), f8(Str::lit("$ref"))));
                match m4088 {
                    None => {
                        if if f660(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f660(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f660(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f648(v2.clone(), List::cons(Adt24::C0(v2, v1), List::<Adt24>::nil())))
                        } else {
                            let v24: Bytes = f58(v1.clone(), f8(Str::lit("type")));
                            if if f32(v24.clone(), Bytes::lit(&[])) {
                                !f750(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f698())
                            } else {
                                if f32(v24.clone(), f8(Str::lit("string"))) {
                                    Ok(f648(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                } else {
                                    if f32(v24.clone(), f8(Str::lit("integer"))) {
                                        Ok(f648(f751(v1), List::<Adt24>::nil()))
                                    } else {
                                        if f32(v24.clone(), f8(Str::lit("number"))) {
                                            Ok(f648(f752(), List::<Adt24>::nil()))
                                        } else {
                                            if f32(v24.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f648(f8(Str::lit("Bool")), List::<Adt24>::nil()))
                                            } else {
                                                if f32(v24.clone(), f8(Str::lit("array"))) {
                                                    let m4090 = f93(v1, f8(Str::lit("items")));
                                                    match m4090 {
                                                        None => {
                                                            Ok(f696(f8(Str::lit("List")), f698()))
                                                        }
                                                        Some(v25) => {
                                                            Ok(f696(f8(Str::lit("List")), f760(v3, v25, {
                                                                let a4091 = v2;
                                                                let a4092 = f8(Str::lit("Item"));
                                                                append_bytes(a4091, a4092)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v24.clone(), f8(Str::lit("object"))) {
                                                        if f655(v1.clone()) {
                                                            Ok(f648(v2.clone(), List::cons(Adt24::C0(v2, v1), List::<Adt24>::nil())))
                                                        } else {
                                                            let m4093 = f693(v1);
                                                            match m4093 {
                                                                None => {
                                                                    Ok(f696(f8(Str::lit("List")), f753(f698())))
                                                                }
                                                                Some(v26) => {
                                                                    Ok(f696(f8(Str::lit("List")), f753(f760(v3, v26, {
                                                                        let a4094 = v2;
                                                                        let a4095 = f8(Str::lit("Value"));
                                                                        append_bytes(a4094, a4095)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f647(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v24, List::<Bytes>::nil()))), v2))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(v22) => {
                        let m4089 = f644(v22)?;
                        match m4089 {
                            None => {
                                Ok(f647(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v23) => {
                                Ok(f648(v23, List::<Adt24>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C9 => {
                Ok(f698())
            }
            Adt2::C10(_, _) => {
                Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C11(_, _, _) => {
                Ok(f647(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
