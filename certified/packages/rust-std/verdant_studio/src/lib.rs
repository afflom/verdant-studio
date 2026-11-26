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
    C5(Bytes),
    C6(std::rc::Rc<Adt2>),
    C7(std::rc::Rc<Adt2>),
    C8,
    C9(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C10(Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
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
    F342,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F121 => f121(p0, p1),
            Fn1::F237 => f237(p0, p1),
            Fn1::F342 => f342(p0, p1),
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
    F312(Bytes),
    F313(Bytes),
    F406(Bytes),
    F407(u64, Bytes),
    F414(Bytes),
    F422(Bytes, u64, Bytes, u64),
    F483(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F312(k0) => f312(k0.clone(), p0),
            Fn3::F313(k0) => f313(k0.clone(), p0),
            Fn3::F406(k0) => Ok(f406(k0.clone(), p0)),
            Fn3::F407(k0, k1) => f407(*k0, k1.clone(), p0),
            Fn3::F414(k0) => f414(k0.clone(), p0),
            Fn3::F422(k0, k1, k2, k3) => f422(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F483(k0) => f483(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F316(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F316(k0) => f316(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F398,
    F723,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F398 => f398(p0, p1),
            Fn5::F723 => f723(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F401,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F401 => f401(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F433,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F433 => f433(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F436,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F436 => f436(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F438,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F438 => f438(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F441,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F441 => f441(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F460(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt23)) -> R<(u64, Adt23)> {
        match self {
            Fn11::F460(k0, k1, k2) => f460(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F494,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F494 => f494(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F497,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F497 => f497(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F528(u64, Bytes),
    F535(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn14::F528(k0, k1) => Ok(f528(*k0, k1.clone(), p0)),
            Fn14::F535(k0) => f535(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F532,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F532 => f532(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F534,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F534 => f534(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F538(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F538(k0) => f538(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F554,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt26) -> R<Adt26> {
        match self {
            Fn18::F554 => f554(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F573,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F573 => f573(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F578,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F578 => f578(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F639,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F639 => f639(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F650,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F650 => f650(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F681,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F681 => f681(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F709,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F709 => f709(p0),
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
        Ok(Adt12::C3(List::cons((f8(Str::lit("model/admissions.json")), f100(Adt2::C6(std::rc::Rc::new(f101(v3))))?), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("re-cited the admissions of the verdant-engine registers at ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                                Adt2::C8
                            }
                            Some(v4) => {
                                v4
                            }
                        }
                    }, f8(Str::lit("object")));
                    match m290 {
                        None => {
                            Adt2::C8
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
                                                Adt2::C8
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
                                Ok(Some((Adt2::C9(std::rc::Rc::new({
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
                            Ok(Some((Adt2::C9(std::rc::Rc::new({
                                let (h374, _) = v6;
                                h374
                            }), std::rc::Rc::new(Adt2::C8)), {
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
                                                Ok(Some((Adt2::C10({
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
                                            Ok(Some((Adt2::C10({
                                                let (_, h405) = v9;
                                                h405
                                            }, std::rc::Rc::new({
                                                let (h406, _) = v10;
                                                h406
                                            }), std::rc::Rc::new(Adt2::C8)), {
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
                            Ok(Some((Adt2::C5({
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
                                        Ok(Some((Adt2::C6(std::rc::Rc::new(Adt2::C8)), {
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
                                                Ok(Some((Adt2::C6(std::rc::Rc::new({
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
                                            Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C8)), {
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
                                                    Ok(Some((Adt2::C7(std::rc::Rc::new({
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
                Adt2::C5(v2) => {
                    Some(v2)
                }
                Adt2::C6(_) => {
                    None::<Bytes>
                }
                Adt2::C7(_) => {
                    None::<Bytes>
                }
                Adt2::C8 => {
                    None::<Bytes>
                }
                Adt2::C9(_, _) => {
                    None::<Bytes>
                }
                Adt2::C10(_, _, _) => {
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
                    Adt2::C5(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C6(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C7(r496) => {
                        let v5 = (*r496).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C8 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C9(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(v2, r497, r498) => {
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
            Adt2::C8
        }
        Some((v1, v2)) => {
            Adt2::C9(std::rc::Rc::new(v1), std::rc::Rc::new(f101(v2)))
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("check-yaml FILE")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))))))
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
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f158() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f248(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op > target/attempt.txt 2>&1; then\u{a}                tee -a target/operation.txt < target/attempt.txt\u{a}                last=$op\u{a}              else\u{a}                cat target/attempt.txt\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f159() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}# The exception is one whose auto-merge is set, the next change to merge: it stays ready and keeps its\u{a}# auto-merge, so it merges as soon as its regeneration passes the studio check. It is\u{a}# regenerated first, and only the oldest such one at a time, since two changes to the\u{a}# API conflict on their export (#309 and #315 were pushed behind main this way).\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}      auto: ${{ steps.stale.outputs.auto }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          auto=''\u{a}          # Oldest first, so the oldest one whose auto-merge is set is the one regenerated now.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = true ]; then\u{a}              # Drafting would drop its auto-merge, so it stays ready. A later one waits: it is\u{a}              # regenerated once this one merges, when it would conflict again anyway.\u{a}              if [ -z \"$auto\" ]; then auto=$pr; else echo \"#${pr} waits for #${auto} to merge\"; fi\u{a}            elif [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          if [ -n \"$auto\" ]; then stale=\" $auto$stale\"; fi\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"auto=$auto\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}          AUTO: ${{ needs.find.outputs.auto }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A regeneration of the one whose auto-merge is set that fails leaves it a draft, which drops\u{a}          # its auto-merge, so it is no longer the next change to merge and nothing is handed over red.\u{a}          trap 'rc=$?; if [ \"$rc\" != 0 ] && [ \"$STALE\" = \"$AUTO\" ]; then gh pr ready --undo \"$STALE\" || true; fi' EXIT\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            # The one whose auto-merge is set stays ready, so that it keeps its auto-merge.\u{a}            if [ \"$pr\" != \"$AUTO\" ]; then gh pr ready --undo \"$pr\" || true; fi\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f248(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
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
    let v4: List<List<Bytes>> = f249(f250(f64(v0, f212()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f251(v4.clone(), f252(v4), f217(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f133(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f163(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f253(f127(v0.clone())?, f135(v0.clone(), f136(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f164(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f254(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f255(f96(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f165() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f166(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m773 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m773.uncons() {
        None => {
            if f256(f95(f64(v0, v1))?)? {
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
                    f257(v5)?
                } else {
                    f258(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f239(f259(), f260(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f239(f259(), f260(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f261()
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
            let m776 = f262(f95(f64(v0.clone(), v1.clone()))?)?;
            match m776 {
                None => {
                    Ok(Adt12::C4(f263()))
                }
                Some(v4) => {
                    let m777 = f264(f95(f64(v0, v1))?)?;
                    match m777 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f265(v5)))
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
        if f117(f266(f267(v2.clone())?), f51({
            let a779 = {
                let a778 = f268(f267(v2)?);
                length_list(a778)
            };
            let a780 = 0u64;
            nat_eq(a779, a780)
        })) {
            Ok(Adt12::C3(List::cons((f239(f259(), f260(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f239(f259(), f260(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt12::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f170(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m781 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m781.uncons() {
        None => {
            let m782 = f269(f95(f64(v0, v1))?)?;
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
            }, f260(v1.clone())?);
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
            let m789 = f270(v4.clone(), f8(Str::lit("requirement")))?;
            match m789 {
                None => {
                    Ok(Adt12::C4(f261()))
                }
                Some(v5) => {
                    let m790 = f270(v4, f8(Str::lit("admission")))?;
                    match m790 {
                        None => {
                            Ok(Adt12::C4(f261()))
                        }
                        Some(v6) => {
                            let v7: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f125(f142(v5.clone(), f8(Str::lit("id"))), f142(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f145(f125(f142(v7.clone(), f8(Str::lit("id"))), f142(v8.clone(), f8(Str::lit("construct")))))?;
                            let m791 = f271(f131(v6.clone()), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m791 {
                                None => {
                                    let a793 = {
                                        let a792 = v9.clone();
                                        length_list(a792)
                                    };
                                    let a794 = 0u64;
                                    if nat_eq(a793, a794) {
                                        Ok(Adt12::C4(f261()))
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
                                            Ok(Adt12::C3(List::cons(f273(f8(Str::lit("model/requirements.json")), f223(v7, v5))?, List::cons(f273(f8(Str::lit("model/admissions.json")), f223(v8, v6))?, List::cons(f274(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f122(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f272(v11))
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
                            f275(v0, f100(v5.clone())?, v5, f58(v4.clone(), f8(Str::lit("name"))), f214(f187(f93(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
                                        let m806 = f56(v0.clone(), List::cons(f276(v1.clone()), List::cons(v9.clone(), List::cons(f8(Str::lit("model/admissions.json")), if f32(v5.clone(), f8(Str::lit("none"))) {
                                            List::<Bytes>::nil()
                                        } else {
                                            List::cons(f7(List::cons(f8(Str::lit("encodings/")), List::cons(v5.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), List::<Bytes>::nil())
                                        }))));
                                        match m806.uncons() {
                                            None => {
                                                if f51(f67(v0.clone(), f276(v1.clone()))) {
                                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v1.clone(), List::cons(f8(Str::lit(" is not a bound authority, because ")), List::cons(f276(v1), List::cons(f8(Str::lit(" is absent, and every admitted value cites a bound standard (REQ-ADMISSION)\u{a}")), List::<Bytes>::nil()))))))))
                                                } else {
                                                    let m807 = f57(v0.clone(), f276(v1.clone()))?;
                                                    match m807 {
                                                        None => {
                                                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(f276(v1), List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
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
                                                            let v16: Bytes = f277(f64(v0.clone(), v9.clone()))?;
                                                            if f51(f32(v16.clone(), v15.clone())) {
                                                                Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v9, List::cons(f8(Str::lit(" is not the artifact ")), List::cons(v1, List::cons(f8(Str::lit(" binds: its SHA-256 is ")), List::cons(v16, List::cons(f8(Str::lit(" and the binding's source.sha256 is ")), List::cons(v15, List::cons(f8(Str::lit(" (FIPS 180-4 section 6.2)\u{a}")), List::<Bytes>::nil()))))))))))))
                                                            } else {
                                                                f278(v0, v1, v3, v5, v6, v9)
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
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f279())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt2 = {
                            let m811 = f93(v4.clone(), f8(Str::lit("source")));
                            match m811 {
                                None => {
                                    Adt2::C8
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
                                f280(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f280(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f239(v9, f260(f58(v7.clone(), f8(Str::lit("url"))))?);
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
                                                if f32(f277(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f280(v0, v4, v5, v10, Some((v12, v11)))
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
            let v5: List<List<Bytes>> = f250(v4.clone())?;
            if f51(f98(f281(v5.clone()), f282())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f283(Str::lit("id"), v6.clone())?, List::cons(f283(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f283(Str::lit("artifact"), f212())?, List::cons(f283(Str::lit("checksum"), f277(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f212(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f284(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f274(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f122(f252(f249(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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
                    if f285(f187(Some(v4))) {
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
                    f275(v0.clone(), f64(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
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
            let m821 = f286(v3.clone());
            match m821.uncons() {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                }
                Some((v6, v7)) => {
                    if f85(v6.clone(), f8(Str::lit("read-"))) {
                        f287(v0.clone(), f64(v0, v1), v2, List::cons(v6, v7), f288(v3))
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
            let m823 = f289(f95(f64(v0.clone(), v1))?)?;
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
                        Ok(Adt12::C3(List::cons(f273(f153(), f223(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f122(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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
                            let m834 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f290(), List::<Bytes>::nil())));
                            match m834.uncons() {
                                None => {
                                    let v11: Bytes = f122(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt2> = f96(v0, f290())?;
                                    if f291(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f273(f290(), f223(v13, List::cons(f292(v11.clone(), v12, v5, v3), List::<Adt2>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
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
                            if f293({
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
    f294(v0, v1)
}

pub fn f187(v0: Option<Adt2>) -> List<Adt2> {
    let m901 = v0;
    match m901 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f295(v1)
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
        Adt2::C5(v9) => {
            Adt2::C5(v9)
        }
        Adt2::C6(r904) => {
            let v10 = (*r904).clone();
            Adt2::C6(std::rc::Rc::new(v10))
        }
        Adt2::C7(r905) => {
            let v5 = (*r905).clone();
            Adt2::C7(std::rc::Rc::new(f188(v5, v1)))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r906, r907) => {
            let v16 = (*r906).clone();
            let v17 = (*r907).clone();
            Adt2::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt2::C10(v2, r908, r909) => {
            let v3 = (*r908).clone();
            let v4 = (*r909).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt2::C10(v2, std::rc::Rc::new(f296(v3, v1.clone())), std::rc::Rc::new(f188(v4, v1)))
            } else {
                Adt2::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f188(v4, v1)))
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
            f297(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f298(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f299(v9)
        }
        Adt2::C6(r911) => {
            let v10 = (*r911).clone();
            Ok(f300(f301(v10)?))
        }
        Adt2::C7(r912) => {
            let v11 = (*r912).clone();
            f302(f303(v11)?)
        }
        Adt2::C8 => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C9(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C10(_, _, _) => {
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
    Ok(f239(f304(), f260(f305(v0))?))
}

pub fn f192(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f305(v0), List::<Bytes>::nil())))))
}

pub fn f193() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f194() -> Bytes {
    f306()
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
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f307(v5.clone(), v2.clone(), v3.clone(), f277(f64(v0.clone(), f191(v5)?))?, v4.clone())?), f196(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f197(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f308(v0.clone())?;
    let v2: u64 = f309(v0.clone(), v1)?;
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
                            let m955 = f310(v0.clone(), v5, v6)?;
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
    let (h967, _) = f311(Fn3::F312(v0.clone()), {
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
    let (h971, _) = f311(Fn3::F313(v0.clone()), {
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
    if f314(v0.clone())? {
        Ok(Some({
            let (_, h975) = {
                let (h976, _) = f315(Fn4::F316(v0.clone()), {
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
        let a980 = f317(v1)?;
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
            let v11: u64 = f317(v7.clone())?;
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
                    let a1015 = f318(v12);
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
    let m1034 = f250(f64(v0, f212()))?;
    match m1034.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f252(v2))
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
    let v3: List<(Bytes, Bytes)> = f319(v1);
    let v4: List<(Bytes, Bytes)> = f320(v0, v2);
    f321(v3.clone(), f154(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
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
                    let v4: List<Bytes> = f145(f125(v0.clone(), f322(v0.clone(), v1.clone())))?;
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
    if f323(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f323(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
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
                            f324(v5.clone())
                        } else {
                            f325(v5.clone())
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
                    f326(v0.clone(), v4, v2.clone(), v3.clone())?
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
            f125(f327({
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
    f328(f329(v0, f232())?)
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
                        let m1073 = f330(f64(v0.clone(), v2.clone()), f331())?;
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
    let v2: Adt18 = f332(v1.clone())?;
    let v3: List<Bytes> = f333(v1.clone());
    let v4: List<Bytes> = f142(match v0.clone() {
        Adt16::C0(_, _, h1077, _, _) => {
            h1077
        }
    }, f8(Str::lit("path")));
    let m1079 = f334(v2, List::<u64>::nil(), f335(v1.clone(), match v0.clone() {
        Adt16::C0(_, h1078, _, _, _) => {
            h1078
        }
    }, 0u64)?);
    match m1079 {
        None => {
            if f337(f338(match v0.clone() {
                Adt16::C0(_, _, _, _, h1081) => {
                    h1081
                }
            }, f125(match v0.clone() {
                Adt16::C0(_, _, _, _, h1082) => {
                    h1082
                }
            }, v4.clone()))?, f338(v4.clone(), f125(match v0.clone() {
                Adt16::C0(_, _, _, _, h1083) => {
                    h1083
                }
            }, v4.clone()))?, f339(match v0.clone() {
                Adt16::C0(_, _, h1084, _, _) => {
                    h1084
                }
            })) {
                let m1088 = f340(f338(match v0.clone() {
                    Adt16::C0(_, _, _, h1085, _) => {
                        h1085
                    }
                }, f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h1086, _) => {
                        h1086
                    }
                }))?, f338(v3.clone(), f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h1087, _) => {
                        h1087
                    }
                }))?);
                match m1088 {
                    None => {
                        let m1091 = f341(f142(match v0 {
                            Adt16::C0(_, h1090, _, _, _) => {
                                h1090
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m1091 {
                            None => {
                                let m1092 = f341(v4, List::<Bytes>::nil());
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f336(f125(v3, match v0 {
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
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f336(match v1 {
                Adt15::C0(_, _, _, _, h1080) => {
                    h1080
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f236(v0: List<Bytes>) -> List<Bytes> {
    f120(Fn1::F342, List::<Bytes>::nil(), v0)
}

pub fn f237(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f343(v0, v1)
}

pub fn f238(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f344(f345(f346(v0))?))
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
            let v4: List<Bytes> = f347(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
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
                                Adt2::C8
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
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f348(f349(v3, v8))?, List::<Bytes>::nil()))), v4)))
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
            if f350(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
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
            let v5: Adt2 = f351(v0.clone(), v3)?;
            if f52(f352(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f353(v5)?, f244(v0, v4, v2)?))
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
                        let v5: Adt2 = f351(v0.clone(), v3.clone())?;
                        if if f52(f352(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f100({
                                let m1109 = f93(v5.clone(), f8(Str::lit("schema")));
                                match m1109 {
                                    None => {
                                        Adt2::C8
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f353(v5)?))
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
    }, f259())) {
        Ok(Adt12::C1(List::cons(f259(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f354(f147(match v0.clone() {
            Adt1::C0(_, h1113, _, _) => {
                h1113
            }
        }, f259())))?, v1.clone());
        let m1114 = f56(v0.clone(), List::cons(f153(), List::cons(f212(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1114.uncons() {
            None => {
                let m1115 = f56(v0.clone(), f355(v0.clone(), v2.clone())?);
                match m1115.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f356(v0.clone(), v2, v1)?;
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
                                            }, List::cons(f7(f145(f357(f127(v0.clone())?, f135(v0.clone(), f136(v0.clone())?)?))?), List::cons(f358(v0)?, List::<Bytes>::nil()))))))
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
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f359(f96(v0, f153())?)?)))
}

pub fn f248() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f249(v0: List<List<Bytes>>) -> List<List<Bytes>> {
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

pub fn f250(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt19 = f360(v0.clone(), 0u64, {
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
            f361(v1)
        }
    };
    Ok(f362(match v4 {
        Adt19::C0(_, _, _, _, h1129) => {
            h1129
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f251(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt17>, v3: List<Adt2>, v4: List<Bytes>) -> List<Bytes> {
    let m1130 = v1;
    match m1130.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f125(f363(v5.clone(), v2.clone()), f364(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f365(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
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
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f251(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f252(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1134 = v0;
    match m1134.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1135 = v1;
            match m1135.uncons() {
                None => {
                    f252(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f252(v2))
                }
            }
        }
    }
}

pub fn f253(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1136 = v1;
    match m1136.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f220(v2.clone(), f8(Str::lit("domain"))) {
                f366(v0.clone(), v2.clone())
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
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f367(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
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
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f253(v0, v3))
            } else {
                f253(v0, v3)
            }
        }
    }
}

pub fn f254(v0: List<Adt2>) -> List<Bytes> {
    let m1140 = v0;
    match m1140.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f254(v2))
        }
    }
}

pub fn f255(v0: List<Adt2>) -> List<Bytes> {
    let m1141 = v0;
    match m1141.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f368(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f255(v2))
        }
    }
}

pub fn f256(p1147: List<Bytes>) -> R<bool> {
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

pub fn f257(v0: List<Bytes>) -> R<bool> {
    if f266(f289(v0.clone())?) {
        Ok(f51({
            let a1149 = {
                let a1148 = f268(f289(v0)?);
                length_list(a1148)
            };
            let a1150 = 0u64;
            nat_eq(a1149, a1150)
        }))
    } else {
        Ok(false)
    }
}

pub fn f258(v0: List<Bytes>) -> R<bool> {
    if f266(f270(v0.clone(), f8(Str::lit("requirement")))?) {
        if f266(f270(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1152 = {
                    let a1151 = f223(f268(f270(v0.clone(), f8(Str::lit("requirement")))?), f268(f270(v0, f8(Str::lit("admission")))?));
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

pub fn f259() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f260(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f261() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f262(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
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
                f262(v2)
            } else {
                let m1158 = f43(v3.clone())?;
                match m1158 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f369(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1159 = f262(v2)?;
                            match m1159 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f370(v5), {
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

pub fn f263() -> Bytes {
    let a1162 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1162)
}

pub fn f264(p1167: List<Bytes>) -> R<Option<Bytes>> {
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
                                        if f371(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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

pub fn f265(v0: Bytes) -> Bytes {
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

pub fn f266(v0: Option<List<Adt2>>) -> bool {
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

pub fn f267(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
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
                f267(v2)
            } else {
                let m1179 = f43(v3)?;
                match m1179 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if f117(f117(f220(v4.clone(), f8(Str::lit("authority"))), f220(v4.clone(), f8(Str::lit("row")))), f117(f117(f220(v4.clone(), f8(Str::lit("construct"))), f220(v4.clone(), f8(Str::lit("artifact")))), f85(f58(v4.clone(), f8(Str::lit("reader"))), f8(Str::lit("read-"))))) {
                            let m1180 = f267(v2)?;
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

pub fn f268(v0: Option<List<Adt2>>) -> List<Adt2> {
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

pub fn f269(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
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
                f269(v2)
            } else {
                let m1186 = f43(v3.clone())?;
                match m1186 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f372(v4.clone()) {
                            let m1187 = f269(v2)?;
                            match m1187 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f243(), List::cons(f8(Str::lit("/")), List::cons(f373(f58(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
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

pub fn f270(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt2>>> {
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
                f270(v3, v1)
            } else {
                let m1194 = f43(v4)?;
                match m1194 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f374(v5.clone());
                        let a1196 = {
                            let a1195 = v6.clone();
                            length_bytes(a1195)
                        };
                        let a1197 = 0u64;
                        if nat_eq(a1196, a1197) {
                            Ok(None::<List<Adt2>>)
                        } else {
                            let m1198 = f270(v3, v1.clone())?;
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

pub fn f271(p1201: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
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

pub fn f272(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f273(v0: Bytes, v1: List<Adt2>) -> R<(Bytes, Bytes)> {
    Ok((v0, f100(Adt2::C6(std::rc::Rc::new(f101(v1))))?))
}

pub fn f274(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f100(Adt2::C6(std::rc::Rc::new(f375(f145(f125(v1, f214(f96(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f275(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1202 = f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m1202.uncons() {
        None => {
            let m1204 = f271(f131(List::cons(Adt2::C7(std::rc::Rc::new({
                let m1203 = f376(v4.clone());
                match m1203 {
                    None => {
                        Adt2::C8
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt2>::nil())), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1204 {
                None => {
                    f377(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f272(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f276(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f277(v0: Bytes) -> R<Bytes> {
    Ok(f378(f379(v0)?))
}

pub fn f278(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: List<Bytes>, v5: Bytes) -> R<Adt12> {
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
        Ok(Adt12::C2(f380(), v4, v6))
    } else {
        let m1208 = f56(v0.clone(), List::cons(f306(), List::<Bytes>::nil()));
        match m1208.uncons() {
            None => {
                f381(v0, v1, f306(), v2, v3, v4)
            }
            Some((v7, v8)) => {
                Ok(Adt12::C0(List::cons(v7, v8)))
            }
        }
    }
}

pub fn f279() -> Bytes {
    f382()
}

pub fn f280(v0: Adt1, v1: Adt2, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f284(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f383(v1, v4)?)?), List::cons(f274(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f281(v0: List<List<Bytes>>) -> List<Bytes> {
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

pub fn f282() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f283(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f348(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f284(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f384(f150(f95(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f385(f386(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f285(p1212: List<Adt2>) -> bool {
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
                            f52(f8(Str::lit("studio")), f142(f387(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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

pub fn f286(v0: List<Bytes>) -> List<Bytes> {
    let m1213 = v0;
    match m1213.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f388(v1.clone()) {
                List::<Bytes>::nil()
            } else {
                List::cons(v1, f286(v2))
            }
        }
    }
}

pub fn f287(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>, v4: List<Bytes>) -> R<Adt12> {
    let m1214 = f376(v4);
    match m1214 {
        None => {
            Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m1215 = f271(f131(List::cons(Adt2::C7(std::rc::Rc::new(v5.clone())), List::<Adt2>::nil())), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1215 {
                None => {
                    f389(v0, v2.clone(), Adt2::C10(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f375(v3)))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C5(f277(v1.clone())?)), std::rc::Rc::new(Adt2::C8))))))), std::rc::Rc::new(v5)), f390(v2, f391(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f272(v6))
                }
            }
        }
    }
}

pub fn f288(p1218: List<Bytes>) -> List<Bytes> {
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
                        if f388(v1.clone()) {
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

pub fn f289(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
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
                f289(v2)
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
                                    f392(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1224 = f289(v2)?;
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

pub fn f290() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f291(p1227: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
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

pub fn f292(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt2 {
    Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C5(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt2::C5(v1)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt2::C5(v2)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt2::C5(v0)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt2::C5(v3)), std::rc::Rc::new(Adt2::C8))))))))))))
}

pub fn f293(v0: u64) -> bool {
    let a1228 = v0;
    let a1229 = 127u64;
    nat_le(a1228, a1229)
}

pub fn f294(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
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
                }, List::cons(f393(f46(v0, v3, {
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

pub fn f295(v0: Adt2) -> List<Adt2> {
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
        Adt2::C5(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C6(r1247) => {
            let v3 = (*r1247).clone();
            f295(v3)
        }
        Adt2::C7(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C8 => {
            List::<Adt2>::nil()
        }
        Adt2::C9(r1248, r1249) => {
            let v1 = (*r1248).clone();
            let v2 = (*r1249).clone();
            List::cons(v1, f295(v2))
        }
        Adt2::C10(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f296(v0: Adt2, v1: Bytes) -> Adt2 {
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
        Adt2::C5(v9) => {
            Adt2::C5(v9)
        }
        Adt2::C6(r1252) => {
            let v10 = (*r1252).clone();
            Adt2::C6(std::rc::Rc::new(v10))
        }
        Adt2::C7(r1253) => {
            let v5 = (*r1253).clone();
            Adt2::C7(std::rc::Rc::new(f296(v5, v1)))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r1254, r1255) => {
            let v16 = (*r1254).clone();
            let v17 = (*r1255).clone();
            Adt2::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt2::C10(v2, r1256, r1257) => {
            let v3 = (*r1256).clone();
            let v4 = (*r1257).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt2::C10(v2, std::rc::Rc::new(f394(v3, v1.clone())), std::rc::Rc::new(f296(v4, v1)))
            } else {
                Adt2::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f296(v4, v1)))
            }
        }
    }
}

pub fn f297(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1258 = 0u64;
        let a1259 = v1;
        nat_lt(a1258, a1259)
    } else {
        false
    } {
        let a1260 = Bytes::lit(&[0x2d]);
        let a1261 = f395(v1)?;
        Ok(append_bytes(a1260, a1261))
    } else {
        f395(v1)
    }
}

pub fn f298(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f395(v1)?;
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
            f7(List::cons(f8(Str::lit("0.")), List::cons(f318({
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

pub fn f299(v0: Bytes) -> R<Bytes> {
    let a1277 = f199(v0.clone(), 0u64, {
        let a1274 = {
            let a1273 = v0.clone();
            length_bytes(a1273)
        };
        let a1275 = 1u64;
        nat_add(a1274, a1275)?
    })?;
    let a1278 = {
        let a1276 = v0.clone();
        length_bytes(a1276)
    };
    if nat_eq(a1277, a1278) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1284 = {
            let a1282 = Bytes::lit(&[0x22]);
            let a1283 = f396(v0.clone(), 0u64, {
                let a1280 = {
                    let a1279 = v0;
                    length_bytes(a1279)
                };
                let a1281 = 1u64;
                nat_add(a1280, a1281)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1282, a1283)
        };
        let a1285 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1284, a1285))
    }
}

pub fn f300(v0: List<Bytes>) -> Bytes {
    let a1288 = {
        let a1286 = Bytes::lit(&[0x5b]);
        let a1287 = f122(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1286, a1287)
    };
    let a1289 = Bytes::lit(&[0x5d]);
    append_bytes(a1288, a1289)
}

pub fn f301(v0: Adt2) -> R<List<Bytes>> {
    let m1290 = v0;
    match m1290 {
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
        Adt2::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C8 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C9(r1291, r1292) => {
            let v12 = (*r1291).clone();
            let v13 = (*r1292).clone();
            Ok(List::cons(f189(v12)?, f301(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f302(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f397(Fn5::F398, List::<(Bytes, Bytes)>::nil(), f399(v0)?);
    let v2: List<Bytes> = f400(Fn6::F401, List::<Bytes>::nil(), v1)?;
    let a1295 = {
        let a1293 = Bytes::lit(&[0x7b]);
        let a1294 = f122(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1293, a1294)
    };
    let a1296 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1295, a1296))
}

pub fn f303(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1297 = v0;
    match m1297 {
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
        Adt2::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C8 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C9(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C10(v14, r1298, r1299) => {
            let v15 = (*r1298).clone();
            let v16 = (*r1299).clone();
            Ok(List::cons((v14, f189(v15)?), f303(v16)?))
        }
    }
}

pub fn f304() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f305(v0: Adt13) -> Bytes {
    let m1300 = v0;
    match m1300 {
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

pub fn f306() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f307(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f402(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f402(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f402(f8(Str::lit("name")), f403(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f402(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f402(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f402(f8(Str::lit("statement")), f404(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f402(f8(Str::lit("media_type")), f405(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f402(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f402(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f402(f8(Str::lit("url")), f192(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f308(v0: Bytes) -> R<u64> {
    let (h1301, _) = f311(Fn3::F406(v0.clone()), {
        let a1302 = v0.clone();
        length_bytes(a1302)
    }, {
        let a1303 = v0;
        length_bytes(a1303)
    })?;
    Ok(h1301)
}

pub fn f309(v0: Bytes, v1: u64) -> R<u64> {
    let (h1304, _) = f311(Fn3::F407(v1, v0), v1, 0u64)?;
    Ok(h1304)
}

pub fn f310(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f115(v0.clone(), {
        let a1305 = v1;
        let a1306 = 1u64;
        nat_add(a1305, a1306)?
    })?;
    let v4: Bytes = {
        let a1307 = v3;
        let a1308 = 117u64;
        if nat_eq(a1307, a1308) {
            f408(v0, v1)?
        } else {
            f409(v3)
        }
    };
    let a1309 = v4.clone();
    let a1310 = Bytes::lit(&[]);
    if equal(a1309, a1310) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1311 = v3;
            let a1312 = 117u64;
            if nat_eq(a1311, a1312) {
                let a1313 = v1;
                let a1314 = 6u64;
                nat_add(a1313, a1314)?
            } else {
                let a1315 = v1;
                let a1316 = 2u64;
                nat_add(a1315, a1316)?
            }
        }, ({
            let a1317 = v2;
            let a1318 = v4;
            append_bytes(a1317, a1318)
        }, 0u64))))
    }
}

pub fn f311(v0: Fn3, p1323: u64, p1324: u64) -> R<(u64, bool)> {
    let m1322 = (p1323, p1324);
    {
        let mut m1322 = m1322;
        loop {
            let step = {
                let (v1, v2) = m1322;
                let m1319 = v1;
                if m1319 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1319 - 1;
                    let m1321 = {
                        let c1320: Fn3 = v0.clone();
                        c1320.apply(v2)?
                    };
                    match m1321 {
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
                Ok(next) => { m1322 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f410(v0, v1)
}

pub fn f313(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f411(v0, v1)
}

pub fn f314(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1325 = v0.clone();
        length_bytes(a1325)
    };
    let a1326 = 0u64;
    let a1327 = v1;
    if nat_lt(a1326, a1327) {
        let a1328 = f412(v0.clone())?;
        let a1329 = v1;
        if nat_eq(a1328, a1329) {
            let a1330 = 1u64;
            let a1331 = v1;
            if nat_lt(a1330, a1331) {
                let m1334 = {
                    let a1332 = v0;
                    let a1333 = 0u64;
                    index_bytes(a1332, a1333)
                };
                match m1334 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1335 = v2;
                        let a1336 = 48u8;
                        Ok(!equal(a1335, a1336))
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

pub fn f315(v0: Fn4, p1341: u64, p1342: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1340 = (p1341, p1342);
    {
        let mut m1340 = m1340;
        loop {
            let step = {
                let (v1, v2) = m1340;
                let m1337 = v1;
                if m1337 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1337 - 1;
                    let m1339 = {
                        let c1338: Fn4 = v0.clone();
                        c1338.apply(v2)?
                    };
                    match m1339 {
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
                Ok(next) => { m1340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f413(v0, v1)
}

pub fn f317(v0: Bytes) -> R<u64> {
    let (h1343, _) = f311(Fn3::F414(v0.clone()), {
        let a1345 = {
            let a1344 = v0;
            length_bytes(a1344)
        };
        let a1346 = 1u64;
        nat_add(a1345, a1346)?
    }, 0u64)?;
    Ok(h1343)
}

pub fn f318(v0: u64) -> Bytes {
    let m1347 = v0;
    if m1347 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1347 - 1;
        let a1348 = f8(Str::lit("0"));
        let a1349 = f318(v1);
        append_bytes(a1348, a1349)
    }
}

pub fn f319(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1350 = v0;
    match m1350.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1351 = f93(v1, f8(Str::lit("std")));
            match m1351 {
                None => {
                    f319(v2)
                }
                Some(v3) => {
                    if f220(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f319(v2))
                    } else {
                        f319(v2)
                    }
                }
            }
        }
    }
}

pub fn f320(v0: Adt14, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1352 = v1;
    match m1352.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt14::C0(h1353, _, _) => {
                    h1353
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h1354, _) => {
                        h1354
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f320(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt14::C0(h1355, _, _) => {
                        h1355
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1356, _) => {
                            h1356
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f320(v0, v3))
                } else {
                    f320(v0, v3)
                }
            }
        }
    }
}

pub fn f321(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt17>> {
    let m1357 = v0;
    match m1357.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v4, v5)) => {
            if if f415(v4.clone(), v2.clone()) {
                true
            } else {
                f415(v4.clone(), v3.clone())
            } {
                f321(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt17::C0({
                    let (h1358, _) = v4.clone();
                    h1358
                }, {
                    let a1361 = {
                        let a1360 = f145(f416({
                            let (h1359, _) = v4.clone();
                            h1359
                        }, v1.clone()))?;
                        length_list(a1360)
                    };
                    let a1362 = 1u64;
                    if nat_eq(a1361, a1362) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1363) = v4.clone();
                    h1363
                }), f321(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f322(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1364 = v1;
    match m1364.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1365, _) = v2.clone();
                h1365
            }, v0.clone()) {
                List::cons({
                    let (_, h1366) = v2;
                    h1366
                }, f322(v0, v3))
            } else {
                if f52({
                    let (_, h1367) = v2;
                    h1367
                }, v0.clone()) {
                    List::cons({
                        let (h1368, _) = v2;
                        h1368
                    }, f322(v0, v3))
                } else {
                    f322(v0, v3)
                }
            }
        }
    }
}

pub fn f323(v0: Adt14, v1: Adt2, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt14::C0(h1369, _, _) => {
            h1369
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1370, _) => {
                h1370
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt14::C0(_, _, h1371) => {
                    h1371
                }
            }) {
                true
            } else {
                if f220(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1372, _) => {
                            h1372
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f324(v0: Adt2) -> bool {
    if f220(v0.clone(), f8(Str::lit("domain"))) {
        if f220(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f417()) {
                if f418(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f418(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f418(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f418(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f325(v0: Adt2) -> bool {
    if f220(v0.clone(), f8(Str::lit("subject"))) {
        if f220(v0.clone(), f8(Str::lit("object"))) {
            if f220(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f419())
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

pub fn f326(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1373 = f43(v1)?;
    match m1373 {
        None => {
            Ok(f226(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f214(f187(f93(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1374 = f92(f93(v4.clone(), f8(Str::lit("construct"))));
                match m1374 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1375 = v5;
            match m1375.uncons() {
                None => {
                    Ok(f226(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1376 = v9;
                    match m1376.uncons() {
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
                                        let m1377 = f92(f93(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1377 {
                                            None => {
                                                Ok(f226(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1378 = f93(v4, f8(Str::lit("value")));
                                                match m1378 {
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

pub fn f327(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1379 = v1;
    match m1379.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f420(v3.clone())
            } else {
                f51(f420(v3.clone()))
            } {
                List::cons(f239(v0.clone(), v3), f327(v0, v4, v2))
            } else {
                f327(v0, v4, v2)
            }
        }
    }
}

pub fn f328(p1382: List<Bytes>) -> R<Option<Bytes>> {
    let m1381 = p1382;
    {
        let mut m1381 = m1381;
        loop {
            let step = {
                let v0 = m1381;
                let m1380 = v0;
                match m1380.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f421(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f197(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f329(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1383 = v1;
    match m1383.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f95(f64(v0.clone(), v2))?, f329(v0, v3)?))
        }
    }
}

pub fn f330(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1384 = v0.clone();
        length_bytes(a1384)
    };
    let v3: u64 = {
        let a1385 = v1.clone();
        length_bytes(a1385)
    };
    let v4: u64 = {
        let (h1386, _) = f311(Fn3::F422(v1.clone(), v3, v0.clone(), v2), {
            let a1387 = v2;
            let a1388 = 1u64;
            nat_add(a1387, a1388)?
        }, 0u64)?;
        h1386
    };
    if f423(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f331() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f332(v0: Adt15) -> R<Adt18> {
    Ok(Adt18::C0(f424(match v0.clone() {
        Adt15::C0(h1389, _, _, _, _) => {
            h1389
        }
    }, 0u64)?, f424(match v0.clone() {
        Adt15::C0(_, h1390, _, _, _) => {
            h1390
        }
    }, 0u64)?, f424(match v0.clone() {
        Adt15::C0(_, _, h1391, _, _) => {
            h1391
        }
    }, 0u64)?, f424(match v0 {
        Adt15::C0(_, _, _, h1392, _) => {
            h1392
        }
    }, 0u64)?))
}

pub fn f333(v0: Adt15) -> List<Bytes> {
    f125(match v0.clone() {
        Adt15::C0(h1393, _, _, _, _) => {
            h1393
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, h1394, _, _) => {
            h1394
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, _, h1395, _) => {
            h1395
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1396) => {
            h1396
        }
    })))
}

pub fn f334(v0: Adt18, p1402: List<u64>, p1403: List<Adt20>) -> Option<u64> {
    let m1401 = (p1402, p1403);
    {
        let mut m1401 = m1401;
        loop {
            let step = {
                let (v1, v2) = m1401;
                let m1397 = v2;
                match m1397.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f425(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1398) => {
                                h1398
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1399, _) => {
                                    h1399
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1400, _) => {
                                    h1400
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1401 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Adt15, v1: List<Adt2>, v2: u64) -> R<List<Adt20>> {
    let m1404 = v1;
    match m1404.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f426(v0.clone(), v3)?), f335(v0, v4, {
                let a1405 = v2;
                let a1406 = 1u64;
                nat_add(a1405, a1406)?
            })?))
        }
    }
}

pub fn f336(p1410: List<Bytes>, p1411: u64) -> Bytes {
    let m1409 = (p1410, p1411);
    {
        let mut m1409 = m1409;
        loop {
            let step = {
                let (v0, v1) = m1409;
                let m1407 = v0;
                match m1407.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1408 = v1;
                        if m1408 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1408 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f427(v0.clone(), v1.clone()) {
        if f427(v1, v0) {
            f428(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f338(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1412 = v0;
    match m1412.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f429(v2, v1.clone())?, f338(v3, v1)?))
        }
    }
}

pub fn f339(v0: List<Adt2>) -> List<u64> {
    let m1413 = v0;
    match m1413.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f430(v1), f339(v2))
        }
    }
}

pub fn f340(p1416: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1415 = p1416;
    {
        let mut m1415 = m1415;
        loop {
            let step = {
                let v0 = m1415;
                let m1414 = v0;
                match m1414.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f431(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(p1419: List<Bytes>, p1420: List<Bytes>) -> Option<Bytes> {
    let m1418 = (p1419, p1420);
    {
        let mut m1418 = m1418;
        loop {
            let step = {
                let (v0, v1) = m1418;
                let m1417 = v0;
                match m1417.uncons() {
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
                Ok(next) => { m1418 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f342(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f343(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1421 = v0.clone();
    match m1421.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1422 = v2;
            let a1423 = v1.clone();
            if equal(a1422, a1423) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f344(v0: List<(Bytes, ())>) -> List<Bytes> {
    f236(f432(Fn7::F433, List::<Bytes>::nil(), v0))
}

pub fn f345(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f434(f435(Fn8::F436, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1428 = {
        let (h1424, _) = f437(Fn9::F438, {
            let a1426 = {
                let a1425 = v0;
                length_list(a1425)
            };
            let a1427 = 1u64;
            nat_add(a1426, a1427)?
        }, v1)?;
        h1424
    };
    match m1428.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f346(v0: List<Bytes>) -> List<(Bytes, ())> {
    f439(f440(Fn10::F441, List::<(Bytes, ())>::nil(), v0))
}

pub fn f347(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1429 = f44(v1, 34u8)?;
    match m1429.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1430 = v3;
            match m1430.uncons() {
                None => {
                    Ok(f442(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f348(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f443(v0.clone(), 0u64, {
        let a1431 = v0;
        length_bytes(a1431)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f349(v0: Adt2, v1: Bytes) -> Bytes {
    if f32(f444(f93(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f350(v0: Bytes, v1: Bytes, p1444: u64, p1445: u64) -> R<bool> {
    let m1443 = (p1444, p1445);
    {
        let mut m1443 = m1443;
        loop {
            let step = {
                let (v2, v3) = m1443;
                let m1432 = v3;
                if m1432 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1432 - 1;
                    let m1435 = {
                        let a1433 = v1.clone();
                        let a1434 = v2;
                        index_bytes(a1433, a1434)
                    };
                    match m1435 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1438 = {
                                let a1436 = v0.clone();
                                let a1437 = v2;
                                index_bytes(a1436, a1437)
                            };
                            match m1438 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1439 = v5;
                                    let a1440 = v6;
                                    if equal(a1439, a1440) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1441 = v2;
                                            let a1442 = 1u64;
                                            nat_add(a1441, a1442)?
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
                Ok(next) => { m1443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1446 = f91(v0, v1)?;
    match m1446 {
        None => {
            Ok(Adt2::C8)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f352(v0: Bytes) -> R<Bytes> {
    let a1447 = f8(Str::lit("Catalog."));
    let a1448 = f373(v0)?;
    Ok(append_bytes(a1447, a1448))
}

pub fn f353(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f373(f58(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f354(v0: List<Bytes>) -> List<Bytes> {
    let m1449 = v0;
    match m1449.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f239(f259(), v1), f354(v2))
            } else {
                f354(v2)
            }
        }
    }
}

pub fn f355(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1450 = v1;
    match m1450.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f445(f446(f95(f64(v0.clone(), v2))?)?), f355(v0, v3)?))
        }
    }
}

pub fn f356(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1451 = v1;
    match m1451.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f447(v0.clone(), v3, v2.clone())?;
            let (h1452, _) = v5.clone();
            if h1452 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f356(v0, v4, v2)?;
                let (h1453, _) = v6.clone();
                if h1453 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1456 = {
                            let (_, h1454) = v5;
                            h1454
                        };
                        let a1457 = {
                            let (_, h1455) = v6;
                            h1455
                        };
                        append_bytes(a1456, a1457)
                    }))
                }
            }
        }
    }
}

pub fn f357(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1458 = v1;
    match m1458.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f220(v2.clone(), f8(Str::lit("subject"))) {
                if f221(v0.clone(), v2.clone()) {
                    f357(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f357(v0, v3))
                }
            } else {
                if f366(v0.clone(), v2.clone()) {
                    f357(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f367(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f357(v0, v3))
                }
            }
        }
    }
}

pub fn f358(v0: Adt1) -> R<Bytes> {
    let a1460 = {
        let a1459 = f128(f129(f252(f249(f250(f64(v0.clone(), f212()))?)), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f131(f96(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1459)
    };
    let a1461 = 0u64;
    if nat_eq(a1460, a1461) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f359(v0: List<Adt2>) -> R<List<Bytes>> {
    let m1462 = v0;
    match m1462.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f122(f214(f187(f93(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f448(f95(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f359(v2)?))
        }
    }
}

pub fn f360(v0: Bytes, p1520: u64, p1521: u64, p1522: Adt19) -> R<Adt19> {
    let m1519 = (p1520, (p1521, p1522));
    {
        let mut m1519 = m1519;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1519;
                let m1463 = v2;
                if m1463 == 0 {
                    Err::<(u64, (u64, Adt19)), R<Adt19>>(Ok(v3))
                } else {
                    let v4 = m1463 - 1;
                    let v5: u64 = f449(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt19::C0(_, h1464, _, _, _) => {
                            h1464
                        }
                    } {
                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                            let a1465 = v1;
                            let a1466 = 1u64;
                            nat_add(a1465, a1466)?
                        }, (v4, Adt19::C0(true, false, match v3.clone() {
                            Adt19::C0(_, _, h1467, _, _) => {
                                h1467
                            }
                        }, match v3.clone() {
                            Adt19::C0(_, _, _, h1468, _) => {
                                h1468
                            }
                        }, match v3 {
                            Adt19::C0(_, _, _, _, h1469) => {
                                h1469
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt19::C0(h1470, _, _, _, _) => {
                                h1470
                            }
                        } {
                            let a1471 = v5;
                            let a1472 = 34u64;
                            if nat_eq(a1471, a1472) {
                                let a1475 = f449(v0.clone(), {
                                    let a1473 = v1;
                                    let a1474 = 1u64;
                                    nat_add(a1473, a1474)?
                                })?;
                                let a1476 = 34u64;
                                if nat_eq(a1475, a1476) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1477 = v1;
                                        let a1478 = 1u64;
                                        nat_add(a1477, a1478)?
                                    }, (v4, Adt19::C0(true, true, f450(match v3.clone() {
                                        Adt19::C0(_, _, h1479, _, _) => {
                                            h1479
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt19::C0(_, _, _, h1480, _) => {
                                            h1480
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1481) => {
                                            h1481
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1482 = v1;
                                        let a1483 = 1u64;
                                        nat_add(a1482, a1483)?
                                    }, (v4, Adt19::C0(false, false, match v3.clone() {
                                        Adt19::C0(_, _, h1484, _, _) => {
                                            h1484
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1485, _) => {
                                            h1485
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1486) => {
                                            h1486
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1487 = v1;
                                    let a1488 = 1u64;
                                    nat_add(a1487, a1488)?
                                }, (v4, Adt19::C0(true, false, f450(match v3.clone() {
                                    Adt19::C0(_, _, h1489, _, _) => {
                                        h1489
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt19::C0(_, _, _, h1490, _) => {
                                        h1490
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1491) => {
                                        h1491
                                    }
                                }))))
                            }
                        } else {
                            let a1492 = v5;
                            let a1493 = 34u64;
                            if nat_eq(a1492, a1493) {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1494 = v1;
                                    let a1495 = 1u64;
                                    nat_add(a1494, a1495)?
                                }, (v4, Adt19::C0(true, false, match v3.clone() {
                                    Adt19::C0(_, _, h1496, _, _) => {
                                        h1496
                                    }
                                }, match v3.clone() {
                                    Adt19::C0(_, _, _, h1497, _) => {
                                        h1497
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1498) => {
                                        h1498
                                    }
                                }))))
                            } else {
                                let a1499 = v5;
                                let a1500 = 44u64;
                                if nat_eq(a1499, a1500) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1501 = v1;
                                        let a1502 = 1u64;
                                        nat_add(a1501, a1502)?
                                    }, (v4, Adt19::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt19::C0(_, _, h1503, _, _) => {
                                            h1503
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1504, _) => {
                                            h1504
                                        }
                                    }), match v3 {
                                        Adt19::C0(_, _, _, _, h1505) => {
                                            h1505
                                        }
                                    }))))
                                } else {
                                    let a1506 = v5;
                                    let a1507 = 13u64;
                                    if nat_eq(a1506, a1507) {
                                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                            let a1508 = v1;
                                            let a1509 = 1u64;
                                            nat_add(a1508, a1509)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1510 = v5;
                                        let a1511 = 10u64;
                                        if nat_eq(a1510, a1511) {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1512 = v1;
                                                let a1513 = 1u64;
                                                nat_add(a1512, a1513)?
                                            }, (v4, f361(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1514 = v1;
                                                let a1515 = 1u64;
                                                nat_add(a1514, a1515)?
                                            }, (v4, Adt19::C0(false, false, f450(match v3.clone() {
                                                Adt19::C0(_, _, h1516, _, _) => {
                                                    h1516
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt19::C0(_, _, _, h1517, _) => {
                                                    h1517
                                                }
                                            }, match v3 {
                                                Adt19::C0(_, _, _, _, h1518) => {
                                                    h1518
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
                Ok(next) => { m1519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: Adt19) -> Adt19 {
    Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f451(List::cons(match v0.clone() {
        Adt19::C0(_, _, h1523, _, _) => {
            h1523
        }
    }, match v0.clone() {
        Adt19::C0(_, _, _, h1524, _) => {
            h1524
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt19::C0(_, _, _, _, h1525) => {
            h1525
        }
    }))
}

pub fn f362(p1528: List<List<Bytes>>, p1529: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1527 = (p1528, p1529);
    {
        let mut m1527 = m1527;
        loop {
            let step = {
                let (v0, v1) = m1527;
                let m1526 = v0;
                match m1526.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363(v0: Bytes, v1: List<Adt17>) -> List<Bytes> {
    let m1530 = v1;
    match m1530.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt17::C0(h1531, _, _) => {
                    h1531
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt17::C0(_, h1532, _) => {
                        h1532
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt17::C0(_, _, h1533) => {
                        h1533
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f363(v0, v3))
            } else {
                f363(v0, v3)
            }
        }
    }
}

pub fn f364(v0: Bytes, v1: List<Adt2>) -> List<Bytes> {
    let m1534 = v1;
    match m1534.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f364(v0, v3))
            } else {
                if f32(f58(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f58(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f364(v0, v3))
                } else {
                    f364(v0, v3)
                }
            }
        }
    }
}

pub fn f365(p1539: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1538 = p1539;
    {
        let mut m1538 = m1538;
        loop {
            let step = {
                let v0 = m1538;
                let m1535 = v0;
                match m1535.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1536 = v2;
                        match m1536.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1537 = v5;
                                    match m1537.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f452(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1538 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f366(v0: Adt14, v1: Adt2) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1540, _, _) => {
            h1540
        }
    }) {
        f52(f367(v1), match v0 {
            Adt14::C0(_, h1541, _) => {
                h1541
            }
        })
    } else {
        false
    }
}

pub fn f367(v0: Adt2) -> Bytes {
    let m1542 = f93(v0, f8(Str::lit("evidence")));
    match m1542 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f368(v0: Adt2) -> Bytes {
    if f220(v0.clone(), f8(Str::lit("req"))) {
        let a1543 = f8(Str::lit("req "));
        let a1544 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1543, a1544)
    } else {
        let m1545 = f93(v0.clone(), f8(Str::lit("std")));
        match m1545 {
            None => {
                let m1548 = f93(v0, f8(Str::lit("der")));
                match m1548 {
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
                    let a1546 = f8(Str::lit(" row "));
                    let a1547 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1546, a1547)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f369(v0: Bytes, v1: Bytes) -> bool {
    let a1549 = v0;
    let a1550 = f382();
    if equal(a1549, a1550) {
        let a1554 = {
            let a1552 = {
                let a1551 = v1;
                length_bytes(a1551)
            };
            let a1553 = 0u64;
            nat_eq(a1552, a1553)
        };
        bool_not(a1554)
    } else {
        false
    }
}

pub fn f370(v0: Bytes) -> Bytes {
    f7(List::cons(f144(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f371(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1556 = v0;
    let a1557 = {
        let a1555 = Str::lit("redistributable");
        utf8_encode(a1555)
    };
    if equal(a1556, a1557) {
        f453(v1)
    } else {
        Ok(false)
    }
}

pub fn f372(v0: Adt2) -> bool {
    if f454(v0.clone(), f8(Str::lit("name"))) {
        if f455(v0.clone(), f8(Str::lit("schema"))) {
            let m1558 = f376(f214(f187(f93(v0, f8(Str::lit("citation"))))));
            match m1558 {
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

pub fn f373(v0: Bytes) -> R<Bytes> {
    f456(v0.clone(), 0u64, {
        let a1559 = v0;
        length_bytes(a1559)
    }, true, Bytes::lit(&[]))
}

pub fn f374(v0: Adt2) -> Bytes {
    if f220(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f220(v0.clone(), f8(Str::lit("id"))) {
            if f220(v0.clone(), f8(Str::lit("statement"))) {
                if f220(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f392(v0, f8(Str::lit("evidence")))
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

pub fn f375(v0: List<Bytes>) -> Adt2 {
    let m1560 = v0;
    match m1560.uncons() {
        None => {
            Adt2::C8
        }
        Some((v1, v2)) => {
            Adt2::C9(std::rc::Rc::new(Adt2::C5(v1)), std::rc::Rc::new(f375(v2)))
        }
    }
}

pub fn f376(v0: List<Bytes>) -> Option<Adt2> {
    let m1561 = v0;
    match m1561.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1562 = v2;
                match m1562.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C10(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C5(v3)), std::rc::Rc::new(Adt2::C8)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1563 = v2;
                    match m1563.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C10(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C5(v5)), std::rc::Rc::new({
                                let m1564 = v6;
                                match m1564.uncons() {
                                    None => {
                                        Adt2::C8
                                    }
                                    Some((v7, _)) => {
                                        Adt2::C10(f8(Str::lit("row")), std::rc::Rc::new(Adt2::C5(v7)), std::rc::Rc::new(Adt2::C8))
                                    }
                                }
                            }))))), std::rc::Rc::new(Adt2::C8)))
                        }
                    }
                } else {
                    if f32(v1, f8(Str::lit("der"))) {
                        let m1565 = v2;
                        match m1565.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m1566 = v10;
                                match m1566.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C10(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C5(v11)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f375(v12)))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C5(v9)), std::rc::Rc::new(Adt2::C8))))))))), std::rc::Rc::new(Adt2::C8)))
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

pub fn f377(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1567 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1567.uncons() {
        None => {
            let m1568 = f376(v4);
            match m1568 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1569 = f8(Str::lit("Catalog."));
                        let a1570 = f373(v3.clone())?;
                        append_bytes(a1569, a1570)
                    };
                    let m1571 = f457(v2, v9, f373(v3.clone())?)?;
                    match m1571 {
                        Ok(v11) => {
                            f389(v0, v3.clone(), v8, v11, f8(Str::lit("Encoding entered through propose-encoding.")), List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f373(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::<(Bytes, Bytes)>::nil()))
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

pub fn f378(v0: Adt23) -> Bytes {
    let a1592 = f458(match v0.clone() {
        Adt23::C0(h1572, _, _, _, _, _, _, _) => {
            h1572
        }
    });
    let a1593 = {
        let a1590 = f458(match v0.clone() {
            Adt23::C0(_, h1573, _, _, _, _, _, _) => {
                h1573
            }
        });
        let a1591 = {
            let a1588 = f458(match v0.clone() {
                Adt23::C0(_, _, h1574, _, _, _, _, _) => {
                    h1574
                }
            });
            let a1589 = {
                let a1586 = f458(match v0.clone() {
                    Adt23::C0(_, _, _, h1575, _, _, _, _) => {
                        h1575
                    }
                });
                let a1587 = {
                    let a1584 = f458(match v0.clone() {
                        Adt23::C0(_, _, _, _, h1576, _, _, _) => {
                            h1576
                        }
                    });
                    let a1585 = {
                        let a1582 = f458(match v0.clone() {
                            Adt23::C0(_, _, _, _, _, h1577, _, _) => {
                                h1577
                            }
                        });
                        let a1583 = {
                            let a1580 = f458(match v0.clone() {
                                Adt23::C0(_, _, _, _, _, _, h1578, _) => {
                                    h1578
                                }
                            });
                            let a1581 = f458(match v0 {
                                Adt23::C0(_, _, _, _, _, _, _, h1579) => {
                                    h1579
                                }
                            });
                            append_bytes(a1580, a1581)
                        };
                        append_bytes(a1582, a1583)
                    };
                    append_bytes(a1584, a1585)
                };
                append_bytes(a1586, a1587)
            };
            append_bytes(a1588, a1589)
        };
        append_bytes(a1590, a1591)
    };
    append_bytes(a1592, a1593)
}

pub fn f379(v0: Bytes) -> R<Adt23> {
    let v1: u64 = {
        let a1594 = v0.clone();
        length_bytes(a1594)
    };
    let v2: u64 = {
        let a1600 = {
            let a1597 = {
                let a1595 = v1;
                let a1596 = 72u64;
                nat_add(a1595, a1596)?
            };
            let a1598 = 64u64;
            let a1599 = 0u64;
            nat_quot(a1597, a1598, a1599)
        };
        let a1601 = 64u64;
        nat_mul(a1600, a1601)?
    };
    let (_, h1602) = f459(Fn11::F460(v0, v1, v2), {
        let a1603 = v2;
        let a1604 = 64u64;
        let a1605 = 0u64;
        nat_quot(a1603, a1604, a1605)
    }, (0u64, f461()))?;
    Ok(h1602)
}

pub fn f380() -> Bytes {
    f8(Str::lit("target/release/verdant_readers"))
}

pub fn f381(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>) -> R<Adt12> {
    let m1606 = f91(v0.clone(), v2.clone())?;
    match m1606 {
        None => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
        }
        Some(v6) => {
            if f32(v4.clone(), f8(Str::lit("none"))) {
                f462(v0, v1, v2, v3, v4, v5, v6)
            } else {
                let v7: Bytes = {
                    let a1607 = f8(Str::lit("Catalog."));
                    let a1608 = v4.clone();
                    append_bytes(a1607, a1608)
                };
                if f51(f463(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, v7.clone(), v3.clone())) {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v3, List::cons(f8(Str::lit(" in model/admissions.json, so it cannot check the entries\u{a}")), List::<Bytes>::nil()))))))))
                } else {
                    let m1609 = f91(v0.clone(), f7(List::cons(f8(Str::lit("encodings/")), List::cons(v4.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))?;
                    match m1609 {
                        None => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: encodings/")), List::cons(v4, List::cons(f8(Str::lit(".schema.json is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v8) => {
                            let m1610 = f464(v8, v6.clone())?;
                            match m1610 {
                                None => {
                                    f462(v0, v1, v2, v3, {
                                        let a1611 = f8(Str::lit("Catalog."));
                                        let a1612 = v4;
                                        append_bytes(a1611, a1612)
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

pub fn f382() -> Bytes {
    let a1613 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1613)
}

pub fn f383(v0: Adt2, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt2> = f93(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f283(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f283(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f283(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f283(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f283(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f283(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f283(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f283(Str::lit("url"), f444(v2.clone(), f8(Str::lit("url"))))?, List::cons(f283(Str::lit("media_type"), f444(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f283(Str::lit("license"), f444(v2.clone(), f8(Str::lit("license"))))?, List::cons(f283(Str::lit("redistribution"), f444(v2, f8(Str::lit("redistribution"))))?, List::cons(f283(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1614 = v1;
        match m1614 {
            None => {
                f7(List::cons(f283(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f283(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f283(Str::lit("artifact"), {
                    let (h1615, _) = v3;
                    h1615
                })?, List::cons(f283(Str::lit("checksum"), {
                    let (_, h1616) = v3;
                    h1616
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f465(f214(f187(f93(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f384(p1619: List<List<Bytes>>, v1: Bytes, p1620: List<Bytes>) -> R<List<Bytes>> {
    let m1618 = (p1619, p1620);
    {
        let mut m1618 = m1618;
        loop {
            let step = {
                let (v0, v2) = m1618;
                let m1617 = v0;
                match m1617.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f466(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f386(f467(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1618 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f308(v0)?))
}

pub fn f386(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1621 = v0;
    match m1621.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1622 = v3.clone();
            match m1622.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1625 = v2;
                    let a1626 = {
                        let a1623 = v1.clone();
                        let a1624 = f386(v3, v1);
                        append_bytes(a1623, a1624)
                    };
                    append_bytes(a1625, a1626)
                }
            }
        }
    }
}

pub fn f387(v0: Adt2, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1627 = f93(v0, v1);
    match m1627 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v3) => {
            f187(f93(v3, v2))
        }
    }
}

pub fn f388(v0: Bytes) -> bool {
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

pub fn f389(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a1628 = f8(Str::lit("Catalog."));
        let a1629 = f373(v1.clone())?;
        append_bytes(a1628, a1629)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f373(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f468(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f100(Adt2::C6(std::rc::Rc::new(f101(f469(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C5(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f100(Adt2::C6(std::rc::Rc::new(f101(f469(f96(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("path")), std::rc::Rc::new(Adt2::C5(v7.clone())), std::rc::Rc::new(Adt2::C10(f8(Str::lit("role")), std::rc::Rc::new(Adt2::C5(v4)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("stratum")), std::rc::Rc::new(Adt2::C3(false, 0u64)), std::rc::Rc::new(Adt2::C8)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a1633 = f470(f64(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v7.clone(), 4u64, {
            let a1631 = {
                let a1630 = v7;
                length_bytes(a1630)
            };
            let a1632 = 4u64;
            nat_sub(a1631, a1632)
        }))?;
        let a1634 = f8(Str::lit("\u{a}"));
        append_bytes(a1633, a1634)
    }), List::cons(f274(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f390(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f373(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f391(v0: Bytes) -> R<Bytes> {
    f471({
        let a1636 = {
            let a1635 = v0.clone();
            length_bytes(a1635)
        };
        let a1637 = 1u64;
        nat_add(a1636, a1637)?
    }, v0)
}

pub fn f392(v0: Adt2, v1: Bytes) -> bool {
    let m1638 = f93(v0, v1);
    match m1638 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f393(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1640 = {
                let a1639 = v0;
                length_bytes(a1639)
            };
            let a1641 = 1u64;
            nat_sub(a1640, a1641)
        })
    } else {
        v0
    }
}

pub fn f394(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1642 = v0;
    match m1642 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v8, r1643) => {
            let v9 = (*r1643).clone();
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
        Adt2::C5(v2) => {
            Adt2::C5(f472(f60(), v2, v1))
        }
        Adt2::C6(r1644) => {
            let v6 = (*r1644).clone();
            Adt2::C6(std::rc::Rc::new(v6))
        }
        Adt2::C7(r1645) => {
            let v7 = (*r1645).clone();
            Adt2::C7(std::rc::Rc::new(v7))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r1646, r1647) => {
            let v13 = (*r1646).clone();
            let v14 = (*r1647).clone();
            Adt2::C9(std::rc::Rc::new(v13), std::rc::Rc::new(v14))
        }
        Adt2::C10(v15, r1648, r1649) => {
            let v16 = (*r1648).clone();
            let v17 = (*r1649).clone();
            Adt2::C10(v15, std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
    }
}

pub fn f395(v0: u64) -> R<Bytes> {
    f473(f474(), 10u64, v0)
}

pub fn f396(v0: Bytes, p1668: u64, p1669: u64, p1670: Bytes) -> R<Bytes> {
    let m1667 = (p1668, (p1669, p1670));
    {
        let mut m1667 = m1667;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1667;
                let m1650 = v2;
                if m1650 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1650 - 1;
                    let v5: u64 = f199(v0.clone(), v1, {
                        let a1654 = {
                            let a1652 = {
                                let a1651 = v0.clone();
                                length_bytes(a1651)
                            };
                            let a1653 = v1;
                            nat_sub(a1652, a1653)
                        };
                        let a1655 = 1u64;
                        nat_add(a1654, a1655)?
                    })?;
                    let v6: Bytes = {
                        let a1658 = v3;
                        let a1659 = f46(v0.clone(), v1, {
                            let a1656 = v5;
                            let a1657 = v1;
                            nat_sub(a1656, a1657)
                        });
                        append_bytes(a1658, a1659)
                    };
                    let m1662 = {
                        let a1660 = v0.clone();
                        let a1661 = v5;
                        index_bytes(a1660, a1661)
                    };
                    match m1662 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a1663 = v5;
                                let a1664 = 1u64;
                                nat_add(a1663, a1664)?
                            }, (v4, {
                                let a1665 = v6;
                                let a1666 = f475(v0.clone(), v5, v7)?;
                                append_bytes(a1665, a1666)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1667 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: Fn5, p1674: List<(Bytes, Bytes)>, p1675: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1673 = (p1674, p1675);
    {
        let mut m1673 = m1673;
        loop {
            let step = {
                let (v1, v2) = m1673;
                let m1671 = v2;
                match m1671.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1672: Fn5 = v0.clone();
                            c1672.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1673 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f476(v0, v1)
}

pub fn f399(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f477(v0)
}

pub fn f400(v0: Fn6, p1679: List<Bytes>, p1680: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1678 = (p1679, p1680);
    {
        let mut m1678 = m1678;
        loop {
            let step = {
                let (v1, v2) = m1678;
                let m1676 = v2;
                match m1676.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1677: Fn6 = v0.clone();
                            c1677.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f401(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f478(v1)?, v0))
}

pub fn f402(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f479(v0)?, List::cons(f8(Str::lit(":")), List::cons(f479(v1)?, List::<Bytes>::nil())))))
}

pub fn f403(v0: Adt13) -> Bytes {
    let m1681 = v0;
    match m1681 {
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

pub fn f404(v0: Adt13) -> Bytes {
    let m1682 = v0;
    match m1682 {
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

pub fn f405(v0: Adt13) -> Bytes {
    let m1683 = v0;
    match m1683 {
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

pub fn f406(v0: Bytes, v1: u64) -> Option<u64> {
    let a1684 = 0u64;
    let a1685 = v1;
    if nat_lt(a1684, a1685) {
        if f480(v0, {
            let a1686 = v1;
            let a1687 = 1u64;
            nat_sub(a1686, a1687)
        }) {
            Some({
                let a1688 = v1;
                let a1689 = 1u64;
                nat_sub(a1688, a1689)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f407(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1690 = v2;
    let a1691 = v0;
    if nat_lt(a1690, a1691) {
        if f480(v1, v2) {
            Ok(Some({
                let a1692 = v2;
                let a1693 = 1u64;
                nat_add(a1692, a1693)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f408(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1696 = f46(v0.clone(), {
        let a1694 = v1;
        let a1695 = 2u64;
        nat_add(a1694, a1695)?
    }, 2u64);
    let a1697 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1696, a1697) {
        let m1700 = f481(f46(v0, {
            let a1698 = v1;
            let a1699 = 4u64;
            nat_add(a1698, a1699)?
        }, 2u64))?;
        match m1700 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f115(v2.clone(), 0u64)?;
                let a1701 = v3;
                let a1702 = 32u64;
                if nat_lt(a1701, a1702) {
                    if f482(v3) {
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

pub fn f409(v0: u64) -> Bytes {
    let a1703 = v0;
    let a1704 = 34u64;
    if nat_eq(a1703, a1704) {
        Bytes::lit(&[0x22])
    } else {
        let a1705 = v0;
        let a1706 = 92u64;
        if nat_eq(a1705, a1706) {
            Bytes::lit(&[0x5c])
        } else {
            let a1707 = v0;
            let a1708 = 98u64;
            if nat_eq(a1707, a1708) {
                Bytes::lit(&[0x08])
            } else {
                let a1709 = v0;
                let a1710 = 102u64;
                if nat_eq(a1709, a1710) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1711 = v0;
                    let a1712 = 110u64;
                    if nat_eq(a1711, a1712) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1713 = v0;
                        let a1714 = 114u64;
                        if nat_eq(a1713, a1714) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1715 = v0;
                            let a1716 = 116u64;
                            if nat_eq(a1715, a1716) {
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

pub fn f410(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f115(v0, v1)?;
    let a1717 = 47u64;
    let a1718 = v2;
    if if nat_lt(a1717, a1718) {
        let a1719 = v2;
        let a1720 = 58u64;
        nat_lt(a1719, a1720)
    } else {
        false
    } {
        Ok(Some({
            let a1721 = v1;
            let a1722 = 1u64;
            nat_add(a1721, a1722)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f411(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1723 = f115(v0, v1)?;
    let a1724 = 48u64;
    if nat_eq(a1723, a1724) {
        Ok(Some({
            let a1725 = v1;
            let a1726 = 1u64;
            nat_add(a1725, a1726)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f412(v0: Bytes) -> R<u64> {
    let (h1727, _) = f311(Fn3::F483(v0.clone()), {
        let a1728 = v0;
        length_bytes(a1728)
    }, 0u64)?;
    Ok(h1727)
}

pub fn f413(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1732 = {
        let a1730 = v0;
        let a1731 = {
            let (h1729, _) = v1;
            h1729
        };
        index_bytes(a1730, a1731)
    };
    match m1732 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1734 = {
                    let (h1733, _) = v1;
                    h1733
                };
                let a1735 = 1u64;
                nat_add(a1734, a1735)?
            }, {
                let a1739 = {
                    let a1737 = {
                        let (_, h1736) = v1;
                        h1736
                    };
                    let a1738 = 10u64;
                    nat_mul(a1737, a1738)?
                };
                let a1740 = f484(v2)?;
                nat_add(a1739, a1740)?
            })))
        }
    }
}

pub fn f414(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f485(v0, v1)
}

pub fn f415(v0: (Bytes, Bytes), p1743: List<(Bytes, Bytes)>) -> bool {
    let m1742 = p1743;
    {
        let mut m1742 = m1742;
        loop {
            let step = {
                let v1 = m1742;
                let m1741 = v1;
                match m1741.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f486(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1742 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1744 = v1;
    match m1744.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1745, _) = v2;
                h1745
            }, v0.clone()) {
                List::cons({
                    let (_, h1746) = v2;
                    h1746
                }, f416(v0, v3))
            } else {
                f416(v0, v3)
            }
        }
    }
}

pub fn f417() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f418(v0: Adt2, v1: Bytes, v2: Bytes) -> bool {
    let m1747 = f93(v0, v1);
    match m1747 {
        None => {
            false
        }
        Some(v3) => {
            f220(v3, v2)
        }
    }
}

pub fn f419() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f420(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f421(v0: Bytes) -> R<bool> {
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
        let a1749 = {
            let a1748 = v1;
            length_bytes(a1748)
        };
        let a1750 = v2;
        nat_sub(a1749, a1750)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1751 = f330(v3.clone(), f8(Str::lit(")")))?;
        match m1751 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1752 = v4;
                    let a1753 = 1u64;
                    nat_add(a1752, a1753)?
                }, {
                    let a1757 = {
                        let a1754 = v3;
                        length_bytes(a1754)
                    };
                    let a1758 = {
                        let a1755 = v4;
                        let a1756 = 1u64;
                        nat_add(a1755, a1756)?
                    };
                    nat_sub(a1757, a1758)
                })
            }
        }
    } else {
        v3
    };
    let a1759 = 0u64;
    let a1760 = v2;
    if nat_lt(a1759, a1760) {
        f487(v5)
    } else {
        Ok(false)
    }
}

pub fn f422(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f423(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1763 = {
            let a1761 = v4;
            let a1762 = v1;
            nat_add(a1761, a1762)?
        };
        let a1764 = v3;
        if nat_lt(a1763, a1764) {
            Ok(Some({
                let a1765 = v4;
                let a1766 = 1u64;
                nat_add(a1765, a1766)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f423(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1771 = {
        let a1768 = v0;
        let a1769 = v2;
        let a1770 = {
            let a1767 = v1.clone();
            length_bytes(a1767)
        };
        slice_bytes(a1768, a1769, a1770)
    };
    match m1771 {
        None => {
            false
        }
        Some(v3) => {
            let a1772 = v3;
            let a1773 = v1;
            equal(a1772, a1773)
        }
    }
}

pub fn f424(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1774 = v0;
    match m1774.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f424(v3, {
                let a1775 = v1;
                let a1776 = 1u64;
                nat_add(a1775, a1776)?
            })?))
        }
    }
}

pub fn f425(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1777 = v2;
    match m1777 {
        Adt21::C0(v3, v4) => {
            if f431(v3, match v0.clone() {
                Adt18::C0(h1778, _, _, _) => {
                    h1778
                }
            }) {
                f488(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f431(v5, match v0 {
                Adt18::C0(_, _, h1779, _) => {
                    h1779
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f427(v7, v1) {
                f431(v8, match v0 {
                    Adt18::C0(_, _, _, h1780) => {
                        h1780
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f426(v0: Adt15, v1: Adt2) -> R<Adt21> {
    let m1781 = f93(v1.clone(), f8(Str::lit("std")));
    match m1781 {
        None => {
            let m1786 = f92(f93(v1.clone(), f8(Str::lit("req"))));
            match m1786 {
                None => {
                    let m1788 = f93(v1, f8(Str::lit("der")));
                    match m1788 {
                        None => {
                            Ok(f489(v0))
                        }
                        Some(v5) => {
                            let m1789 = f490(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1789 {
                                None => {
                                    Ok(f489(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f338(f214(f187(f93(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1790) => {
                                            h1790
                                        }
                                    })?, f429(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1791, _) => {
                                            h1791
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f429(v4, match v0 {
                        Adt15::C0(_, _, h1787, _, _) => {
                            h1787
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1782 = f92(f93(v2.clone(), f8(Str::lit("row"))));
            match m1782 {
                None => {
                    Ok(Adt21::C0(f429(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1783, _, _, _, _) => {
                            h1783
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f429(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1784, _, _, _, _) => {
                            h1784
                        }
                    })?, Some(f429(v3, match v0 {
                        Adt15::C0(_, h1785, _, _, _) => {
                            h1785
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f427(p1794: List<u64>, v1: List<u64>) -> bool {
    let m1793 = p1794;
    {
        let mut m1793 = m1793;
        loop {
            let step = {
                let v0 = m1793;
                let m1792 = v0;
                match m1792.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f431(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f428(p1799: List<u64>) -> bool {
    let m1798 = p1799;
    {
        let mut m1798 = m1798;
        loop {
            let step = {
                let v0 = m1798;
                let m1795 = v0;
                match m1795.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1796 = v1;
                        let a1797 = 7u64;
                        if nat_le(a1796, a1797) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1798 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f429(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1800 = f491(v0, v1.clone(), 0u64)?;
    match m1800 {
        None => {
            let a1801 = v1;
            Ok(length_list(a1801))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f430(v0: Adt2) -> u64 {
    let m1802 = f492(f93(v0, f8(Str::lit("stratum"))));
    match m1802 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f431(v0: u64, p1807: List<u64>) -> bool {
    let m1806 = p1807;
    {
        let mut m1806 = m1806;
        loop {
            let step = {
                let v1 = m1806;
                let m1803 = v1;
                match m1803.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1804 = v2;
                        let a1805 = v0;
                        if nat_eq(a1804, a1805) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(v0: Fn7, p1811: List<Bytes>, p1812: List<(Bytes, ())>) -> List<Bytes> {
    let m1810 = (p1811, p1812);
    {
        let mut m1810 = m1810;
        loop {
            let step = {
                let (v1, v2) = m1810;
                let m1808 = v2;
                match m1808.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1809: Fn7 = v0.clone();
                            c1809.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1810 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1813, _) = v1;
        h1813
    }, v0)
}

pub fn f434(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f493(Fn12::F494, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f435(v0: Fn8, p1817: List<List<(Bytes, ())>>, p1818: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1816 = (p1817, p1818);
    {
        let mut m1816 = m1816;
        loop {
            let step = {
                let (v1, v2) = m1816;
                let m1814 = v2;
                match m1814.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1815: Fn8 = v0.clone();
                            c1815.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1816 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f437(v0: Fn9, p1823: u64, p1824: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1822 = (p1823, p1824);
    {
        let mut m1822 = m1822;
        loop {
            let step = {
                let (v1, v2) = m1822;
                let m1819 = v1;
                if m1819 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1819 - 1;
                    let m1821 = {
                        let c1820: Fn9 = v0.clone();
                        c1820.apply(v2.clone())?
                    };
                    match m1821 {
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
                Ok(next) => { m1822 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f495(v0)
}

pub fn f439(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f496(Fn13::F497, List::<(Bytes, ())>::nil(), v0)
}

pub fn f440(v0: Fn10, p1828: List<(Bytes, ())>, p1829: List<Bytes>) -> List<(Bytes, ())> {
    let m1827 = (p1828, p1829);
    {
        let mut m1827 = m1827;
        loop {
            let step = {
                let (v1, v2) = m1827;
                let m1825 = v2;
                match m1825.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1826: Fn10 = v0.clone();
                            c1826.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f442(p1834: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1833 = p1834;
    {
        let mut m1833 = m1833;
        loop {
            let step = {
                let v0 = m1833;
                let m1830 = v0;
                match m1830.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1831, _) = v2;
                            h1831
                        }) {
                            let (_, h1832) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1832)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: Bytes, p1860: u64, p1861: u64, p1862: Bytes) -> R<Bytes> {
    let m1859 = (p1860, (p1861, p1862));
    {
        let mut m1859 = m1859;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1859;
                let m1835 = v2;
                if m1835 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1835 - 1;
                    let m1838 = {
                        let a1836 = v0.clone();
                        let a1837 = v1;
                        index_bytes(a1836, a1837)
                    };
                    match m1838 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f202({
                                let a1839 = v6;
                                let a1840 = 34u64;
                                nat_eq(a1839, a1840)
                            }, {
                                let a1841 = v6;
                                let a1842 = 92u64;
                                nat_eq(a1841, a1842)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1843 = v1;
                                    let a1844 = 1u64;
                                    nat_add(a1843, a1844)?
                                }, (v4, {
                                    let a1847 = v3;
                                    let a1848 = {
                                        let a1845 = f8(Str::lit("\\"));
                                        let a1846 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1845, a1846)
                                    };
                                    append_bytes(a1847, a1848)
                                })))
                            } else {
                                let a1849 = v6;
                                let a1850 = 10u64;
                                if nat_eq(a1849, a1850) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1851 = v1;
                                        let a1852 = 1u64;
                                        nat_add(a1851, a1852)?
                                    }, (v4, {
                                        let a1853 = v3;
                                        let a1854 = f8(Str::lit("\\n"));
                                        append_bytes(a1853, a1854)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1855 = v1;
                                        let a1856 = 1u64;
                                        nat_add(a1855, a1856)?
                                    }, (v4, {
                                        let a1857 = v3;
                                        let a1858 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1857, a1858)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1859 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f444(v0: Option<Adt2>, v1: Bytes) -> Bytes {
    let m1863 = v0;
    match m1863 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f445(v0: List<Adt2>) -> List<Bytes> {
    let m1864 = v0;
    match m1864.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f498(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f276(f58(v1, f8(Str::lit("authority")))), f445(v2)))
        }
    }
}

pub fn f446(v0: List<Bytes>) -> R<List<Adt2>> {
    let m1865 = f289(v0.clone())?;
    match m1865 {
        None => {
            Ok(f268(f267(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt2>::nil())
        }
    }
}

pub fn f447(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    let m1866 = f289(v3.clone())?;
    match m1866 {
        None => {
            let m1867 = f267(v3.clone())?;
            match m1867 {
                None => {
                    f501(v0, v1, v3)
                }
                Some(v5) => {
                    f500(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f499(v0, v1, v4)
        }
    }
}

pub fn f448(v0: List<Bytes>) -> List<Bytes> {
    let m1868 = v0;
    match m1868.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1870 = {
                    let a1869 = v1.clone();
                    length_bytes(a1869)
                };
                let a1871 = 0u64;
                if nat_eq(a1870, a1871) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f448(v2))
        }
    }
}

pub fn f449(v0: Bytes, v1: u64) -> R<u64> {
    let m1874 = {
        let a1872 = v0;
        let a1873 = v1;
        index_bytes(a1872, a1873)
    };
    match m1874 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f450(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1878 = {
        let a1875 = v1;
        let a1876 = v2;
        let a1877 = 1u64;
        slice_bytes(a1875, a1876, a1877)
    };
    match m1878 {
        None => {
            v0
        }
        Some(v3) => {
            let a1879 = v0;
            let a1880 = v3;
            append_bytes(a1879, a1880)
        }
    }
}

pub fn f451(p1883: List<Bytes>, p1884: List<Bytes>) -> List<Bytes> {
    let m1882 = (p1883, p1884);
    {
        let mut m1882 = m1882;
        loop {
            let step = {
                let (v0, v1) = m1882;
                let m1881 = v0;
                match m1881.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1882 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(v0: List<Bytes>) -> Bytes {
    let m1885 = v0;
    match m1885.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f453(v0: Bytes) -> R<bool> {
    let m1887 = {
        let a1886 = v0.clone();
        length_bytes(a1886)
    };
    if m1887 == 0 {
        Ok(true)
    } else {
        let v1 = m1887 - 1;
        let m1890 = {
            let a1888 = v0;
            let a1889 = v1;
            index_bytes(a1888, a1889)
        };
        match m1890 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f502({
                    let v3: u8 = v2;
                    let a1935 = {
                        let a1911 = {
                            let a1899 = {
                                let a1893 = {
                                    let a1891 = v3;
                                    let a1892 = 128u8;
                                    fixed_u8::bit_and(a1891, a1892)
                                };
                                let a1894 = 128u8;
                                if equal(a1893, a1894) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1900 = {
                                let a1897 = {
                                    let a1895 = v3;
                                    let a1896 = 64u8;
                                    fixed_u8::bit_and(a1895, a1896)
                                };
                                let a1898 = 64u8;
                                if equal(a1897, a1898) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1899, a1900)?
                        };
                        let a1912 = {
                            let a1909 = {
                                let a1903 = {
                                    let a1901 = v3;
                                    let a1902 = 32u8;
                                    fixed_u8::bit_and(a1901, a1902)
                                };
                                let a1904 = 32u8;
                                if equal(a1903, a1904) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1910 = {
                                let a1907 = {
                                    let a1905 = v3;
                                    let a1906 = 16u8;
                                    fixed_u8::bit_and(a1905, a1906)
                                };
                                let a1908 = 16u8;
                                if equal(a1907, a1908) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1909, a1910)?
                        };
                        nat_add(a1911, a1912)?
                    };
                    let a1936 = {
                        let a1933 = {
                            let a1921 = {
                                let a1915 = {
                                    let a1913 = v3;
                                    let a1914 = 8u8;
                                    fixed_u8::bit_and(a1913, a1914)
                                };
                                let a1916 = 8u8;
                                if equal(a1915, a1916) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1922 = {
                                let a1919 = {
                                    let a1917 = v3;
                                    let a1918 = 4u8;
                                    fixed_u8::bit_and(a1917, a1918)
                                };
                                let a1920 = 4u8;
                                if equal(a1919, a1920) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1921, a1922)?
                        };
                        let a1934 = {
                            let a1931 = {
                                let a1925 = {
                                    let a1923 = v3;
                                    let a1924 = 2u8;
                                    fixed_u8::bit_and(a1923, a1924)
                                };
                                let a1926 = 2u8;
                                if equal(a1925, a1926) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1932 = {
                                let a1929 = {
                                    let a1927 = v3;
                                    let a1928 = 1u8;
                                    fixed_u8::bit_and(a1927, a1928)
                                };
                                let a1930 = 1u8;
                                if equal(a1929, a1930) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1931, a1932)?
                        };
                        nat_add(a1933, a1934)?
                    };
                    nat_add(a1935, a1936)?
                }))
            }
        }
    }
}

pub fn f454(v0: Adt2, v1: Bytes) -> bool {
    let a1938 = {
        let a1937 = f58(v0, v1);
        length_bytes(a1937)
    };
    let a1939 = 0u64;
    !nat_eq(a1938, a1939)
}

pub fn f455(v0: Adt2, v1: Bytes) -> bool {
    let m1940 = f93(v0, v1);
    match m1940 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f456(v0: Bytes, p1961: u64, p1962: u64, p1963: bool, p1964: Bytes) -> R<Bytes> {
    let m1960 = (p1961, (p1962, (p1963, p1964)));
    {
        let mut m1960 = m1960;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1960;
                let m1941 = v2;
                if m1941 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1941 - 1;
                    let m1944 = {
                        let a1942 = v0.clone();
                        let a1943 = v1;
                        index_bytes(a1942, a1943)
                    };
                    match m1944 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f503(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1945 = v1;
                                    let a1946 = 1u64;
                                    nat_add(a1945, a1946)?
                                }, (v5, (true, v4))))
                            } else {
                                if f117(f504(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1947 = v1;
                                        let a1948 = 1u64;
                                        nat_add(a1947, a1948)?
                                    }, (v5, (false, {
                                        let a1952 = v4;
                                        let a1953 = f46({
                                            let a1949 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1949)
                                        }, {
                                            let a1950 = v7;
                                            let a1951 = 97u64;
                                            nat_sub(a1950, a1951)
                                        }, 1u64);
                                        append_bytes(a1952, a1953)
                                    }))))
                                } else {
                                    if f202(f504(v7), f202(f505(v7), f506(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1954 = v1;
                                            let a1955 = 1u64;
                                            nat_add(a1954, a1955)?
                                        }, (v5, (false, {
                                            let a1956 = v4;
                                            let a1957 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1956, a1957)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1958 = v1;
                                            let a1959 = 1u64;
                                            nat_add(a1958, a1959)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1960 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f507(f508(f93(v0.clone(), f8(Str::lit("$defs")))), f508(f93(v0.clone(), f8(Str::lit("definitions")))));
    let m1965 = f509(4096u64, v3.clone(), List::<Bytes>::nil(), f510(v3)?)?;
    match m1965 {
        Ok(v5) => {
            let m1966 = f511(4096u64, f512(f513(v5)?, List::cons(Adt24::C0(v2, v0), List::<Adt24>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1966 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f386(f242(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f458(v0: u32) -> Bytes {
    let a1995 = f514({
        let a1967 = f515(v0, 28u32);
        let a1968 = 15u32;
        fixed_u32::bit_and(a1967, a1968)
    });
    let a1996 = {
        let a1993 = f514({
            let a1969 = f515(v0, 24u32);
            let a1970 = 15u32;
            fixed_u32::bit_and(a1969, a1970)
        });
        let a1994 = {
            let a1991 = f514({
                let a1971 = f515(v0, 20u32);
                let a1972 = 15u32;
                fixed_u32::bit_and(a1971, a1972)
            });
            let a1992 = {
                let a1989 = f514({
                    let a1973 = f515(v0, 16u32);
                    let a1974 = 15u32;
                    fixed_u32::bit_and(a1973, a1974)
                });
                let a1990 = {
                    let a1987 = f514({
                        let a1975 = f515(v0, 12u32);
                        let a1976 = 15u32;
                        fixed_u32::bit_and(a1975, a1976)
                    });
                    let a1988 = {
                        let a1985 = f514({
                            let a1977 = f515(v0, 8u32);
                            let a1978 = 15u32;
                            fixed_u32::bit_and(a1977, a1978)
                        });
                        let a1986 = {
                            let a1983 = f514({
                                let a1979 = f515(v0, 4u32);
                                let a1980 = 15u32;
                                fixed_u32::bit_and(a1979, a1980)
                            });
                            let a1984 = f514({
                                let a1981 = v0;
                                let a1982 = 15u32;
                                fixed_u32::bit_and(a1981, a1982)
                            });
                            append_bytes(a1983, a1984)
                        };
                        append_bytes(a1985, a1986)
                    };
                    append_bytes(a1987, a1988)
                };
                append_bytes(a1989, a1990)
            };
            append_bytes(a1991, a1992)
        };
        append_bytes(a1993, a1994)
    };
    append_bytes(a1995, a1996)
}

pub fn f459(v0: Fn11, p2000: u64, p2001: (u64, Adt23)) -> R<(u64, Adt23)> {
    let m1999 = (p2000, p2001);
    {
        let mut m1999 = m1999;
        loop {
            let step = {
                let (v1, v2) = m1999;
                let m1997 = v1;
                if m1997 == 0 {
                    Err::<(u64, (u64, Adt23)), R<(u64, Adt23)>>(Ok(v2))
                } else {
                    let v3 = m1997 - 1;
                    Ok::<(u64, (u64, Adt23)), R<(u64, Adt23)>>((v3, {
                        let c1998: Fn11 = v0.clone();
                        c1998.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1999 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt23)) -> R<(u64, Adt23)> {
    Ok(({
        let a2003 = {
            let (h2002, _) = v3.clone();
            h2002
        };
        let a2004 = 64u64;
        nat_add(a2003, a2004)?
    }, f516({
        let (_, h2005) = v3;
        h2005
    }, f517(v0, v1, v2, {
        let (h2006, _) = v3;
        h2006
    })?)?))
}

pub fn f461() -> Adt23 {
    Adt23::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f462(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Adt2) -> R<Adt12> {
    let m2007 = f518(f519(v1.clone(), v3.clone(), v4, v6))?;
    match m2007 {
        Ok(v8) => {
            let v9: Bytes = f498(v1.clone());
            Ok(Adt12::C3(List::cons((v9.clone(), v8), List::cons((f8(Str::lit("model/admissions.json")), f100(Adt2::C6(std::rc::Rc::new(f101(f469(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C5(v9.clone())), std::rc::Rc::new(Adt2::C10(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C5(v1)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("row")), std::rc::Rc::new(Adt2::C5(v3)), std::rc::Rc::new(Adt2::C8))))))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f375(v5)))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C5(f277(f64(v0, v2))?)), std::rc::Rc::new(Adt2::C8))))))), std::rc::Rc::new(Adt2::C8)))))))))))))?), List::<(Bytes, Bytes)>::nil())), f7(List::cons(f8(Str::lit("admitted ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        }
        Err(v7) => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(": ")), List::cons(v7, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
        }
    }
}

pub fn f463(p2011: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m2010 = p2011;
    {
        let mut m2010 = m2010;
        loop {
            let step = {
                let v0 = m2010;
                let m2008 = v0;
                match m2008.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f117(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2009 = f93(v3, f8(Str::lit("std")));
                            match m2009 {
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
                Ok(next) => { m2010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f464(v0: Adt2, v1: Adt2) -> R<Option<Bytes>> {
    f520({
        let a2018 = {
            let a2013 = {
                let a2012 = f521(v0.clone())?;
                length_bytes(a2012)
            };
            let a2014 = 2u64;
            nat_add(a2013, a2014)?
        };
        let a2019 = {
            let a2016 = {
                let a2015 = f521(v1.clone())?;
                length_bytes(a2015)
            };
            let a2017 = 2u64;
            nat_add(a2016, a2017)?
        };
        nat_mul(a2018, a2019)?
    }, v0.clone(), v0, v1, Bytes::lit(&[]))
}

pub fn f465(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f386(f522(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f466(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2020 = f44(v1, 34u8)?;
    match m2020.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2021 = v3;
            match m2021.uncons() {
                None => {
                    Ok(f523(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f467(v0: List<Bytes>) -> List<Bytes> {
    let m2022 = v0;
    match m2022.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f467(v2)
            } else {
                List::cons(v1, f467(v2))
            }
        }
    }
}

pub fn f468(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2023 = v0;
    match m2023.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f468(v3, v1))
        }
    }
}

pub fn f469(v0: List<Adt2>, v1: Bytes, v2: Bytes, v3: Adt2) -> List<Adt2> {
    let m2024 = v0;
    match m2024.uncons() {
        None => {
            List::cons(v3, List::<Adt2>::nil())
        }
        Some((v4, v5)) => {
            if f32(f58(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f524(v5, v1, v2))
            } else {
                List::cons(v4, f469(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f470(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f386(f525(f95(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f471(p2035: u64, p2036: Bytes) -> R<Bytes> {
    let m2034 = (p2035, p2036);
    {
        let mut m2034 = m2034;
        loop {
            let step = {
                let (v0, v1) = m2034;
                let m2025 = v0;
                if m2025 == 0 {
                    Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                } else {
                    let v2 = m2025 - 1;
                    if f85(v1.clone(), f8(Str::lit("-- "))) {
                        let m2026 = f330(v1.clone(), f8(Str::lit("\u{a}")))?;
                        match m2026 {
                            None => {
                                Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                            }
                            Some(v3) => {
                                Ok::<(u64, Bytes), R<Bytes>>((v2, f46(v1.clone(), {
                                    let a2027 = v3;
                                    let a2028 = 1u64;
                                    nat_add(a2027, a2028)?
                                }, {
                                    let a2032 = {
                                        let a2029 = v1;
                                        length_bytes(a2029)
                                    };
                                    let a2033 = {
                                        let a2030 = v3;
                                        let a2031 = 1u64;
                                        nat_add(a2030, a2031)?
                                    };
                                    nat_sub(a2032, a2033)
                                })))
                            }
                        }
                    } else {
                        Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m2034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(p2039: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2038 = p2039;
    {
        let mut m2038 = m2038;
        loop {
            let step = {
                let v0 = m2038;
                let m2037 = v0;
                match m2037.uncons() {
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
                Ok(next) => { m2038 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2040 = v2;
    let a2041 = 0u64;
    if nat_eq(a2040, a2041) {
        Ok(f526(v0, 0u64))
    } else {
        let (_, h2042) = {
            let (h2043, _) = f527(Fn14::F528(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2043
        };
        Ok(h2042)
    }
}

pub fn f474() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f475(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2088 = {
            let a2064 = {
                let a2052 = {
                    let a2046 = {
                        let a2044 = v3;
                        let a2045 = 128u8;
                        fixed_u8::bit_and(a2044, a2045)
                    };
                    let a2047 = 128u8;
                    if equal(a2046, a2047) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2053 = {
                    let a2050 = {
                        let a2048 = v3;
                        let a2049 = 64u8;
                        fixed_u8::bit_and(a2048, a2049)
                    };
                    let a2051 = 64u8;
                    if equal(a2050, a2051) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2052, a2053)?
            };
            let a2065 = {
                let a2062 = {
                    let a2056 = {
                        let a2054 = v3;
                        let a2055 = 32u8;
                        fixed_u8::bit_and(a2054, a2055)
                    };
                    let a2057 = 32u8;
                    if equal(a2056, a2057) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2063 = {
                    let a2060 = {
                        let a2058 = v3;
                        let a2059 = 16u8;
                        fixed_u8::bit_and(a2058, a2059)
                    };
                    let a2061 = 16u8;
                    if equal(a2060, a2061) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2062, a2063)?
            };
            nat_add(a2064, a2065)?
        };
        let a2089 = {
            let a2086 = {
                let a2074 = {
                    let a2068 = {
                        let a2066 = v3;
                        let a2067 = 8u8;
                        fixed_u8::bit_and(a2066, a2067)
                    };
                    let a2069 = 8u8;
                    if equal(a2068, a2069) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2075 = {
                    let a2072 = {
                        let a2070 = v3;
                        let a2071 = 4u8;
                        fixed_u8::bit_and(a2070, a2071)
                    };
                    let a2073 = 4u8;
                    if equal(a2072, a2073) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2074, a2075)?
            };
            let a2087 = {
                let a2084 = {
                    let a2078 = {
                        let a2076 = v3;
                        let a2077 = 2u8;
                        fixed_u8::bit_and(a2076, a2077)
                    };
                    let a2079 = 2u8;
                    if equal(a2078, a2079) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2085 = {
                    let a2082 = {
                        let a2080 = v3;
                        let a2081 = 1u8;
                        fixed_u8::bit_and(a2080, a2081)
                    };
                    let a2083 = 1u8;
                    if equal(a2082, a2083) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2084, a2085)?
            };
            nat_add(a2086, a2087)?
        };
        nat_add(a2088, a2089)?
    };
    let a2090 = v4;
    let a2091 = 34u64;
    if nat_eq(a2090, a2091) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2092 = v4;
        let a2093 = 92u64;
        if nat_eq(a2092, a2093) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2094 = v4;
            let a2095 = 8u64;
            if nat_eq(a2094, a2095) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2096 = v4;
                let a2097 = 12u64;
                if nat_eq(a2096, a2097) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2098 = v4;
                    let a2099 = 10u64;
                    if nat_eq(a2098, a2099) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2100 = v4;
                        let a2101 = 13u64;
                        if nat_eq(a2100, a2101) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2102 = v4;
                            let a2103 = 9u64;
                            if nat_eq(a2102, a2103) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2104 = v4;
                                let a2105 = 32u64;
                                if nat_lt(a2104, a2105) {
                                    let a2106 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2107 = f529(v2)?;
                                    Ok(append_bytes(a2106, a2107))
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

pub fn f476(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2108 = v0.clone();
    match m2108.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2111 = {
                let (h2109, _) = v2;
                h2109
            };
            let a2112 = {
                let (h2110, _) = v1.clone();
                h2110
            };
            if equal(a2111, a2112) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f477(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f530(f531(Fn15::F532, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2117 = {
        let (h2113, _) = f533(Fn16::F534, {
            let a2115 = {
                let a2114 = v0;
                length_list(a2114)
            };
            let a2116 = 1u64;
            nat_add(a2115, a2116)?
        }, v1)?;
        h2113
    };
    match m2117.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f478(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2122 = {
        let a2119 = f299({
            let (h2118, _) = v0;
            h2118
        })?;
        let a2120 = Bytes::lit(&[0x3a]);
        append_bytes(a2119, a2120)
    };
    let a2123 = {
        let (_, h2121) = v0;
        h2121
    };
    Ok(append_bytes(a2122, a2123))
}

pub fn f479(v0: Bytes) -> R<Bytes> {
    let a2129 = {
        let a2127 = Bytes::lit(&[0x22]);
        let a2128 = {
            let (_, h2124) = {
                let (h2125, _) = f527(Fn14::F535(v0.clone()), {
                    let a2126 = v0;
                    length_bytes(a2126)
                }, (0u64, Bytes::lit(&[])))?;
                h2125
            };
            h2124
        };
        append_bytes(a2127, a2128)
    };
    let a2130 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2129, a2130))
}

pub fn f480(v0: Bytes, v1: u64) -> bool {
    let m2131 = f207(v0, v1);
    match m2131 {
        None => {
            false
        }
        Some(v2) => {
            f536(v2)
        }
    }
}

pub fn f481(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2132 = v0.clone();
        length_bytes(a2132)
    };
    let a2136 = {
        let a2133 = v1;
        let a2134 = 2u64;
        let a2135 = 0u64;
        nat_rem(a2133, a2134, a2135)
    };
    let a2137 = 0u64;
    if nat_eq(a2136, a2137) {
        let (_, h2138) = {
            let (h2139, _) = f537(Fn17::F538(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2139
        };
        Ok(h2138)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f482(v0: u64) -> bool {
    let a2140 = v0;
    let a2141 = 8u64;
    if nat_eq(a2140, a2141) {
        true
    } else {
        let a2142 = v0;
        let a2143 = 9u64;
        if nat_eq(a2142, a2143) {
            true
        } else {
            let a2144 = v0;
            let a2145 = 10u64;
            if nat_eq(a2144, a2145) {
                true
            } else {
                let a2146 = v0;
                let a2147 = 12u64;
                if nat_eq(a2146, a2147) {
                    true
                } else {
                    let a2148 = v0;
                    let a2149 = 13u64;
                    nat_eq(a2148, a2149)
                }
            }
        }
    }
}

pub fn f483(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2152 = {
        let a2150 = v0;
        let a2151 = v1;
        index_bytes(a2150, a2151)
    };
    match m2152 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f539(v2)? {
                Ok(Some({
                    let a2153 = v1;
                    let a2154 = 1u64;
                    nat_add(a2153, a2154)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f484(v0: u8) -> R<u64> {
    let a2201 = {
        let v1: u8 = v0;
        let a2199 = {
            let a2175 = {
                let a2163 = {
                    let a2157 = {
                        let a2155 = v1;
                        let a2156 = 128u8;
                        fixed_u8::bit_and(a2155, a2156)
                    };
                    let a2158 = 128u8;
                    if equal(a2157, a2158) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2164 = {
                    let a2161 = {
                        let a2159 = v1;
                        let a2160 = 64u8;
                        fixed_u8::bit_and(a2159, a2160)
                    };
                    let a2162 = 64u8;
                    if equal(a2161, a2162) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2163, a2164)?
            };
            let a2176 = {
                let a2173 = {
                    let a2167 = {
                        let a2165 = v1;
                        let a2166 = 32u8;
                        fixed_u8::bit_and(a2165, a2166)
                    };
                    let a2168 = 32u8;
                    if equal(a2167, a2168) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2174 = {
                    let a2171 = {
                        let a2169 = v1;
                        let a2170 = 16u8;
                        fixed_u8::bit_and(a2169, a2170)
                    };
                    let a2172 = 16u8;
                    if equal(a2171, a2172) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2173, a2174)?
            };
            nat_add(a2175, a2176)?
        };
        let a2200 = {
            let a2197 = {
                let a2185 = {
                    let a2179 = {
                        let a2177 = v1;
                        let a2178 = 8u8;
                        fixed_u8::bit_and(a2177, a2178)
                    };
                    let a2180 = 8u8;
                    if equal(a2179, a2180) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2186 = {
                    let a2183 = {
                        let a2181 = v1;
                        let a2182 = 4u8;
                        fixed_u8::bit_and(a2181, a2182)
                    };
                    let a2184 = 4u8;
                    if equal(a2183, a2184) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2185, a2186)?
            };
            let a2198 = {
                let a2195 = {
                    let a2189 = {
                        let a2187 = v1;
                        let a2188 = 2u8;
                        fixed_u8::bit_and(a2187, a2188)
                    };
                    let a2190 = 2u8;
                    if equal(a2189, a2190) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2196 = {
                    let a2193 = {
                        let a2191 = v1;
                        let a2192 = 1u8;
                        fixed_u8::bit_and(a2191, a2192)
                    };
                    let a2194 = 1u8;
                    if equal(a2193, a2194) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2195, a2196)?
            };
            nat_add(a2197, a2198)?
        };
        nat_add(a2199, a2200)?
    };
    let a2202 = 48u64;
    Ok(nat_sub(a2201, a2202))
}

pub fn f485(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2203 = v0.clone();
        length_bytes(a2203)
    };
    let a2204 = v1;
    let a2205 = v2;
    if if nat_lt(a2204, a2205) {
        let a2210 = f115(v0, {
            let a2208 = v2;
            let a2209 = {
                let a2206 = v1;
                let a2207 = 1u64;
                nat_add(a2206, a2207)?
            };
            nat_sub(a2208, a2209)
        })?;
        let a2211 = 48u64;
        nat_eq(a2210, a2211)
    } else {
        false
    } {
        Ok(Some({
            let a2212 = v1;
            let a2213 = 1u64;
            nat_add(a2212, a2213)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f486(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f117(f32({
        let (h2214, _) = v0;
        h2214
    }, {
        let (h2215, _) = v1;
        h2215
    }), f32({
        let (_, h2216) = v0;
        h2216
    }, {
        let (_, h2217) = v1;
        h2217
    }))
}

pub fn f487(v0: Bytes) -> R<bool> {
    let m2218 = f330(v0.clone(), f8(Str::lit("&&")))?;
    if match m2218 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2219 = f330(v0, f8(Str::lit("||")))?;
        match m2219 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f488(v0: Adt18, v1: Option<u64>) -> bool {
    let m2220 = v1;
    match m2220 {
        None => {
            true
        }
        Some(v2) => {
            f431(v2, match v0 {
                Adt18::C0(_, h2221, _, _) => {
                    h2221
                }
            })
        }
    }
}

pub fn f489(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a2223 = match v0 {
            Adt15::C0(_, _, h2222, _, _) => {
                h2222
            }
        };
        length_list(a2223)
    })
}

pub fn f490(v0: Bytes) -> Option<Adt22> {
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

pub fn f491(v0: Bytes, p2228: List<Bytes>, p2229: u64) -> R<Option<u64>> {
    let m2227 = (p2228, p2229);
    {
        let mut m2227 = m2227;
        loop {
            let step = {
                let (v1, v2) = m2227;
                let m2224 = v1;
                match m2224.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2225 = v2;
                                let a2226 = 1u64;
                                nat_add(a2225, a2226)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492(v0: Option<Adt2>) -> Option<u64> {
    let m2230 = v0;
    match m2230 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2231 = v1;
            match m2231 {
                Adt2::C0 => {
                    None::<u64>
                }
                Adt2::C1(_, _) => {
                    None::<u64>
                }
                Adt2::C2(_) => {
                    None::<u64>
                }
                Adt2::C3(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt2::C4(_, _, _) => {
                    None::<u64>
                }
                Adt2::C5(_) => {
                    None::<u64>
                }
                Adt2::C6(_) => {
                    None::<u64>
                }
                Adt2::C7(_) => {
                    None::<u64>
                }
                Adt2::C8 => {
                    None::<u64>
                }
                Adt2::C9(_, _) => {
                    None::<u64>
                }
                Adt2::C10(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f493(v0: Fn12, p2235: List<List<(Bytes, ())>>, p2236: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2234 = (p2235, p2236);
    {
        let mut m2234 = m2234;
        loop {
            let step = {
                let (v1, v2) = m2234;
                let m2232 = v2;
                match m2232.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2233: Fn12 = v0.clone();
                            c2233.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f495(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2238 = {
        let a2237 = v0.clone();
        length_list(a2237)
    };
    let a2239 = 1u64;
    if nat_le(a2238, a2239) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f540(v0)?))
    }
}

pub fn f496(v0: Fn13, p2243: List<(Bytes, ())>, p2244: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2242 = (p2243, p2244);
    {
        let mut m2242 = m2242;
        loop {
            let step = {
                let (v1, v2) = m2242;
                let m2240 = v2;
                match m2240.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2241: Fn13 = v0.clone();
                            c2241.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f498(v0: Bytes) -> Bytes {
    f7(List::cons(f139(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f499(v0: Adt1, v1: Bytes, v2: List<Adt2>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f142(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f142(f96(v0, f153())?, f8(Str::lit("at"))));
    let a2246 = {
        let a2245 = v4.clone();
        length_list(a2245)
    };
    let a2247 = 0u64;
    if nat_eq(a2246, a2247) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2250 = {
            let a2248 = v4;
            length_list(a2248)
        };
        let a2251 = {
            let a2249 = v3;
            length_list(a2249)
        };
        if nat_eq(a2250, a2251) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f541(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f500(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt2>) -> R<(bool, Bytes)> {
    let m2252 = v3;
    match m2252.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f202(f67(v0.clone(), f498(v6.clone())), f52(v6.clone(), v2.clone())) {
                f500(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f276(v6.clone()))) {
                    let v9: (bool, Bytes) = f500(v0, v1.clone(), v2, v5)?;
                    let (h2253, _) = v9.clone();
                    if h2253 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2255 = f541(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2256 = {
                                let (_, h2254) = v9;
                                h2254
                            };
                            append_bytes(a2255, a2256)
                        }))
                    }
                } else {
                    if f117(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f463(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2257 = f8(Str::lit("Catalog."));
                        let a2258 = v8.clone();
                        append_bytes(a2257, a2258)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f500(v0, v1.clone(), v2, v5)?;
                        let (h2259, _) = v10.clone();
                        if h2259 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2261 = f541(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2262 = {
                                    let (_, h2260) = v10;
                                    h2260
                                };
                                append_bytes(a2261, a2262)
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

pub fn f501(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt2> = f268(f270(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt2> = f268(f270(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f125(f142(v3.clone(), f8(Str::lit("id"))), f142(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f125(v6.clone(), f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2264 = {
        let a2263 = v7.clone();
        length_list(a2263)
    };
    let a2265 = 0u64;
    if nat_eq(a2264, a2265) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2268 = {
                let a2266 = v7;
                length_list(a2266)
            };
            let a2269 = {
                let a2267 = v5;
                length_list(a2267)
            };
            nat_eq(a2268, a2269)
        }) {
            Ok((false, f541(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f145(f125(f62(f542(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f543(v4), f125(v6, f142(v3, f8(Str::lit("id")))))))?;
            let a2271 = {
                let a2270 = v8.clone();
                length_list(a2270)
            };
            let a2272 = 0u64;
            if nat_eq(a2271, a2272) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f541(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f122(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f502(v0: u64) -> bool {
    let a2273 = v0;
    let a2274 = 47u64;
    nat_eq(a2273, a2274)
}

pub fn f503(v0: u64) -> bool {
    f202({
        let a2275 = v0;
        let a2276 = 95u64;
        nat_eq(a2275, a2276)
    }, f202({
        let a2277 = v0;
        let a2278 = 45u64;
        nat_eq(a2277, a2278)
    }, f202({
        let a2279 = v0;
        let a2280 = 32u64;
        nat_eq(a2279, a2280)
    }, f202({
        let a2281 = v0;
        let a2282 = 46u64;
        nat_eq(a2281, a2282)
    }, {
        let a2283 = v0;
        let a2284 = 47u64;
        nat_eq(a2283, a2284)
    }))))
}

pub fn f504(v0: u64) -> bool {
    f117({
        let a2285 = 97u64;
        let a2286 = v0;
        nat_le(a2285, a2286)
    }, {
        let a2287 = v0;
        let a2288 = 122u64;
        nat_le(a2287, a2288)
    })
}

pub fn f505(v0: u64) -> bool {
    f117({
        let a2289 = 65u64;
        let a2290 = v0;
        nat_le(a2289, a2290)
    }, {
        let a2291 = v0;
        let a2292 = 90u64;
        nat_le(a2291, a2292)
    })
}

pub fn f506(v0: u64) -> bool {
    f117({
        let a2293 = 48u64;
        let a2294 = v0;
        nat_le(a2293, a2294)
    }, {
        let a2295 = v0;
        let a2296 = 57u64;
        nat_le(a2295, a2296)
    })
}

pub fn f507(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m2297 = v0;
    match m2297.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f507(v3, v1))
        }
    }
}

pub fn f508(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m2298 = v0;
    match m2298 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f544(v1)
        }
    }
}

pub fn f509(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m2299 = v0;
    if m2299 == 0 {
        let m2300 = v1;
        match m2300.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2299 - 1;
        let m2301 = v1.clone();
        match m2301.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m2302 = f545(v1.clone(), v2.clone(), v3.clone())?;
                match m2302 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f386(f510(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2305 = f509(v6, f546(v1, {
                            let (h2303, _) = v9.clone();
                            h2303
                        }), List::cons(f547({
                            let (h2304, _) = v9.clone();
                            h2304
                        })?, v2), v3)?;
                        match m2305 {
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

pub fn f510(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m2306 = v0;
    match m2306.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f547({
                let (h2307, _) = v1;
                h2307
            })?, f510(v2)?))
        }
    }
}

pub fn f511(p2313: u64, p2314: List<Adt24>, p2315: List<Bytes>, p2316: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2312 = (p2313, (p2314, (p2315, p2316)));
    {
        let mut m2312 = m2312;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2312;
                let m2308 = v0;
                if m2308 == 0 {
                    Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2308 - 1;
                    let m2309 = v1;
                    match m2309.uncons() {
                        None => {
                            Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2310 = v5;
                            match m2310 {
                                Adt24::C0(v8, v9) => {
                                    if f32(v8.clone(), f548()) {
                                        Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f52(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2311 = f550(v8.clone(), v9)?;
                                            match m2311 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f512(f551(v11.clone()), List::cons(Adt24::C1(f552(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f52(f548(), v2.clone()) {
                                        Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f548(), v2), List::cons(f549(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(v0: List<Adt24>, v1: List<Adt24>) -> List<Adt24> {
    let m2317 = v0;
    match m2317.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f512(v3, v1))
        }
    }
}

pub fn f513(v0: List<(Bytes, Adt2)>) -> R<List<Adt24>> {
    let m2318 = v0;
    match m2318.uncons() {
        None => {
            Ok(List::<Adt24>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt24::C0(f547({
                let (h2319, _) = v1;
                h2319
            })?, {
                let (_, h2320) = v1;
                h2320
            }), f513(v2)?))
        }
    }
}

pub fn f514(v0: u32) -> Bytes {
    let a2323 = {
        let a2321 = v0;
        let a2322 = 8u32;
        fixed_u32::bit_and(a2321, a2322)
    };
    let a2324 = 0u32;
    if equal(a2323, a2324) {
        let a2327 = {
            let a2325 = v0;
            let a2326 = 4u32;
            fixed_u32::bit_and(a2325, a2326)
        };
        let a2328 = 0u32;
        if equal(a2327, a2328) {
            let a2331 = {
                let a2329 = v0;
                let a2330 = 2u32;
                fixed_u32::bit_and(a2329, a2330)
            };
            let a2332 = 0u32;
            if equal(a2331, a2332) {
                let a2335 = {
                    let a2333 = v0;
                    let a2334 = 1u32;
                    fixed_u32::bit_and(a2333, a2334)
                };
                let a2336 = 0u32;
                if equal(a2335, a2336) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2339 = {
                    let a2337 = v0;
                    let a2338 = 1u32;
                    fixed_u32::bit_and(a2337, a2338)
                };
                let a2340 = 0u32;
                if equal(a2339, a2340) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2343 = {
                let a2341 = v0;
                let a2342 = 2u32;
                fixed_u32::bit_and(a2341, a2342)
            };
            let a2344 = 0u32;
            if equal(a2343, a2344) {
                let a2347 = {
                    let a2345 = v0;
                    let a2346 = 1u32;
                    fixed_u32::bit_and(a2345, a2346)
                };
                let a2348 = 0u32;
                if equal(a2347, a2348) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2351 = {
                    let a2349 = v0;
                    let a2350 = 1u32;
                    fixed_u32::bit_and(a2349, a2350)
                };
                let a2352 = 0u32;
                if equal(a2351, a2352) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2355 = {
            let a2353 = v0;
            let a2354 = 4u32;
            fixed_u32::bit_and(a2353, a2354)
        };
        let a2356 = 0u32;
        if equal(a2355, a2356) {
            let a2359 = {
                let a2357 = v0;
                let a2358 = 2u32;
                fixed_u32::bit_and(a2357, a2358)
            };
            let a2360 = 0u32;
            if equal(a2359, a2360) {
                let a2363 = {
                    let a2361 = v0;
                    let a2362 = 1u32;
                    fixed_u32::bit_and(a2361, a2362)
                };
                let a2364 = 0u32;
                if equal(a2363, a2364) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2367 = {
                    let a2365 = v0;
                    let a2366 = 1u32;
                    fixed_u32::bit_and(a2365, a2366)
                };
                let a2368 = 0u32;
                if equal(a2367, a2368) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2371 = {
                let a2369 = v0;
                let a2370 = 2u32;
                fixed_u32::bit_and(a2369, a2370)
            };
            let a2372 = 0u32;
            if equal(a2371, a2372) {
                let a2375 = {
                    let a2373 = v0;
                    let a2374 = 1u32;
                    fixed_u32::bit_and(a2373, a2374)
                };
                let a2376 = 0u32;
                if equal(a2375, a2376) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2379 = {
                    let a2377 = v0;
                    let a2378 = 1u32;
                    fixed_u32::bit_and(a2377, a2378)
                };
                let a2380 = 0u32;
                if equal(a2379, a2380) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f515(v0: u32, v1: u32) -> u32 {
    let m2383 = {
        let a2381 = v0;
        let a2382 = v1;
        fixed_u32::shift_right(a2381, a2382)
    };
    match m2383 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f516(v0: Adt23, v1: Adt25) -> R<Adt23> {
    let v2: Adt26 = f553(Fn18::F554, 64u64, Adt26::C0(match v0.clone() {
        Adt23::C0(h2384, _, _, _, _, _, _, _) => {
            h2384
        }
    }, match v0.clone() {
        Adt23::C0(_, h2385, _, _, _, _, _, _) => {
            h2385
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h2386, _, _, _, _, _) => {
            h2386
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h2387, _, _, _, _) => {
            h2387
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, h2388, _, _, _) => {
            h2388
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2389, _, _) => {
            h2389
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2390, _) => {
            h2390
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h2391) => {
            h2391
        }
    }, v1, 0u64))?;
    Ok(Adt23::C0(f555(match v0.clone() {
        Adt23::C0(h2392, _, _, _, _, _, _, _) => {
            h2392
        }
    }, match v2.clone() {
        Adt26::C0(h2393, _, _, _, _, _, _, _, _, _) => {
            h2393
        }
    }), f555(match v0.clone() {
        Adt23::C0(_, h2394, _, _, _, _, _, _) => {
            h2394
        }
    }, match v2.clone() {
        Adt26::C0(_, h2395, _, _, _, _, _, _, _, _) => {
            h2395
        }
    }), f555(match v0.clone() {
        Adt23::C0(_, _, h2396, _, _, _, _, _) => {
            h2396
        }
    }, match v2.clone() {
        Adt26::C0(_, _, h2397, _, _, _, _, _, _, _) => {
            h2397
        }
    }), f555(match v0.clone() {
        Adt23::C0(_, _, _, h2398, _, _, _, _) => {
            h2398
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, h2399, _, _, _, _, _, _) => {
            h2399
        }
    }), f555(match v0.clone() {
        Adt23::C0(_, _, _, _, h2400, _, _, _) => {
            h2400
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, h2401, _, _, _, _, _) => {
            h2401
        }
    }), f555(match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2402, _, _) => {
            h2402
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, _, h2403, _, _, _, _) => {
            h2403
        }
    }), f555(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2404, _) => {
            h2404
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, _, _, h2405, _, _, _) => {
            h2405
        }
    }), f555(match v0 {
        Adt23::C0(_, _, _, _, _, _, _, h2406) => {
            h2406
        }
    }, match v2 {
        Adt26::C0(_, _, _, _, _, _, _, h2407, _, _) => {
            h2407
        }
    })))
}

pub fn f517(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt25> {
    Ok(Adt25::C0(f556(v0.clone(), v1, v2, v3)?, f556(v0.clone(), v1, v2, {
        let a2408 = v3;
        let a2409 = 4u64;
        nat_add(a2408, a2409)?
    })?, f556(v0.clone(), v1, v2, {
        let a2410 = v3;
        let a2411 = 8u64;
        nat_add(a2410, a2411)?
    })?, f556(v0.clone(), v1, v2, {
        let a2412 = v3;
        let a2413 = 12u64;
        nat_add(a2412, a2413)?
    })?, f556(v0.clone(), v1, v2, {
        let a2414 = v3;
        let a2415 = 16u64;
        nat_add(a2414, a2415)?
    })?, f556(v0.clone(), v1, v2, {
        let a2416 = v3;
        let a2417 = 20u64;
        nat_add(a2416, a2417)?
    })?, f556(v0.clone(), v1, v2, {
        let a2418 = v3;
        let a2419 = 24u64;
        nat_add(a2418, a2419)?
    })?, f556(v0.clone(), v1, v2, {
        let a2420 = v3;
        let a2421 = 28u64;
        nat_add(a2420, a2421)?
    })?, f556(v0.clone(), v1, v2, {
        let a2422 = v3;
        let a2423 = 32u64;
        nat_add(a2422, a2423)?
    })?, f556(v0.clone(), v1, v2, {
        let a2424 = v3;
        let a2425 = 36u64;
        nat_add(a2424, a2425)?
    })?, f556(v0.clone(), v1, v2, {
        let a2426 = v3;
        let a2427 = 40u64;
        nat_add(a2426, a2427)?
    })?, f556(v0.clone(), v1, v2, {
        let a2428 = v3;
        let a2429 = 44u64;
        nat_add(a2428, a2429)?
    })?, f556(v0.clone(), v1, v2, {
        let a2430 = v3;
        let a2431 = 48u64;
        nat_add(a2430, a2431)?
    })?, f556(v0.clone(), v1, v2, {
        let a2432 = v3;
        let a2433 = 52u64;
        nat_add(a2432, a2433)?
    })?, f556(v0.clone(), v1, v2, {
        let a2434 = v3;
        let a2435 = 56u64;
        nat_add(a2434, a2435)?
    })?, f556(v0, v1, v2, {
        let a2436 = v3;
        let a2437 = 60u64;
        nat_add(a2436, a2437)?
    })?))
}

pub fn f518(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m2438 = f557(v0.clone());
    match m2438 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f189(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f519(v0: Bytes, v1: Bytes, v2: Bytes, v3: Adt2) -> Adt2 {
    Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("citation")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f558(List::cons(f8(Str::lit("std")), List::cons(v0, List::cons(v1, List::<Bytes>::nil()))))))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C5(v2)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("pointer")), std::rc::Rc::new(Adt2::C5(Bytes::lit(&[]))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("value")), std::rc::Rc::new(v3), std::rc::Rc::new(Adt2::C8))))))))))
}

pub fn f520(v0: u64, v1: Adt2, v2: Adt2, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2439 = v0;
    if m2439 == 0 {
        Ok(Some(f7(List::cons(f8(Str::lit("the schema's references do not end before the instance at \"")), List::cons(v4, List::cons(f8(Str::lit("\" is checked (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
    } else {
        let v5 = m2439 - 1;
        let m2440 = v2.clone();
        match m2440 {
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
                    Ok(f559(v4, f8(Str::lit("is refused by the false schema (JSON Schema 2020-12 Core section 4.3.2)"))))
                }
            }
            Adt2::C3(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C4(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C5(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C6(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C7(r2454) => {
                let v7 = (*r2454).clone();
                let m2441 = f560(f544(v7));
                match m2441 {
                    None => {
                        Ok(f561(f562(v2.clone(), v3.clone(), v4.clone())?, f561({
                            let m2442 = f92(f93(v2.clone(), f8(Str::lit("$ref"))));
                            match m2442 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v9) => {
                                    let m2443 = f563(v1.clone(), v9)?;
                                    match m2443 {
                                        Ok(v11) => {
                                            f520(v5, v1.clone(), v11, v3.clone(), v4.clone())?
                                        }
                                        Err(v10) => {
                                            Some(v10)
                                        }
                                    }
                                }
                            }
                        }, f561(f564(v5, v1.clone(), f187(f93(v2.clone(), f8(Str::lit("allOf")))), v3.clone(), v4.clone())?, f561({
                            let m2444 = f93(v2.clone(), f8(Str::lit("anyOf")));
                            match m2444 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v12) => {
                                    let a2445 = 0u64;
                                    let a2446 = f565(v5, v1.clone(), f187(Some(v12)), v3.clone(), v4.clone())?;
                                    if nat_lt(a2445, a2446) {
                                        None::<Bytes>
                                    } else {
                                        f559(v4.clone(), f8(Str::lit("matches no anyOf branch (JSON Schema 2020-12 Core section 10.2.1.2)")))
                                    }
                                }
                            }
                        }, f561({
                            let m2447 = f93(v2.clone(), f8(Str::lit("oneOf")));
                            match m2447 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v13) => {
                                    let a2448 = f565(v5, v1.clone(), f187(Some(v13)), v3.clone(), v4.clone())?;
                                    let a2449 = 1u64;
                                    if nat_eq(a2448, a2449) {
                                        None::<Bytes>
                                    } else {
                                        f559(v4.clone(), f8(Str::lit("does not match exactly one oneOf branch (JSON Schema 2020-12 Core section 10.2.1.3)")))
                                    }
                                }
                            }
                        }, {
                            let m2450 = v3.clone();
                            match m2450 {
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
                                Adt2::C5(_) => {
                                    None::<Bytes>
                                }
                                Adt2::C6(r2452) => {
                                    let v15 = (*r2452).clone();
                                    let m2451 = f93(v2, f8(Str::lit("items")));
                                    match m2451 {
                                        None => {
                                            None::<Bytes>
                                        }
                                        Some(v16) => {
                                            f568(v5, v1, v16, f295(v15), v4, 0u64)?
                                        }
                                    }
                                }
                                Adt2::C7(r2453) => {
                                    let v14 = (*r2453).clone();
                                    f561(f566(v5, v1.clone(), f508(f93(v2.clone(), f8(Str::lit("properties")))), v3, v4.clone())?, f567(v5, v1, v2, f544(v14), v4)?)
                                }
                                Adt2::C8 => {
                                    None::<Bytes>
                                }
                                Adt2::C9(_, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C10(_, _, _) => {
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
            Adt2::C8 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C9(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C10(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
        }
    }
}

pub fn f521(v0: Adt2) -> R<Bytes> {
    f189(v0)
}

pub fn f522(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2455 = v0;
    match m2455.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f348(v1)?, f522(v2)?))
        }
    }
}

pub fn f523(p2458: List<Bytes>, v1: Bytes) -> bool {
    let m2457 = p2458;
    {
        let mut m2457 = m2457;
        loop {
            let step = {
                let v0 = m2457;
                let m2456 = v0;
                match m2456.uncons() {
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
                Ok(next) => { m2457 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f524(v0: List<Adt2>, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m2459 = v0;
    match m2459.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v3, v4)) => {
            if f32(f58(v3.clone(), v1.clone()), v2.clone()) {
                f524(v4, v1, v2)
            } else {
                List::cons(v3, f524(v4, v1, v2))
            }
        }
    }
}

pub fn f525(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2460 = v0;
    match m2460.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f386(f522(f145(List::cons(v1, f569(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f525(v3, v1)?))
            }
        }
    }
}

pub fn f526(v0: Bytes, v1: u64) -> Bytes {
    let m2464 = {
        let a2461 = v0;
        let a2462 = v1;
        let a2463 = 1u64;
        slice_bytes(a2461, a2462, a2463)
    };
    match m2464 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f527(v0: Fn14, p2469: u64, p2470: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m2468 = (p2469, p2470);
    {
        let mut m2468 = m2468;
        loop {
            let step = {
                let (v1, v2) = m2468;
                let m2465 = v1;
                if m2465 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2465 - 1;
                    let m2467 = {
                        let c2466: Fn14 = v0.clone();
                        c2466.apply(v2.clone())?
                    };
                    match m2467 {
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
                Ok(next) => { m2468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f570(v1, v0, v2)
}

pub fn f529(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2515 = {
            let a2491 = {
                let a2479 = {
                    let a2473 = {
                        let a2471 = v1;
                        let a2472 = 128u8;
                        fixed_u8::bit_and(a2471, a2472)
                    };
                    let a2474 = 128u8;
                    if equal(a2473, a2474) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2480 = {
                    let a2477 = {
                        let a2475 = v1;
                        let a2476 = 64u8;
                        fixed_u8::bit_and(a2475, a2476)
                    };
                    let a2478 = 64u8;
                    if equal(a2477, a2478) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2479, a2480)?
            };
            let a2492 = {
                let a2489 = {
                    let a2483 = {
                        let a2481 = v1;
                        let a2482 = 32u8;
                        fixed_u8::bit_and(a2481, a2482)
                    };
                    let a2484 = 32u8;
                    if equal(a2483, a2484) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2490 = {
                    let a2487 = {
                        let a2485 = v1;
                        let a2486 = 16u8;
                        fixed_u8::bit_and(a2485, a2486)
                    };
                    let a2488 = 16u8;
                    if equal(a2487, a2488) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2489, a2490)?
            };
            nat_add(a2491, a2492)?
        };
        let a2516 = {
            let a2513 = {
                let a2501 = {
                    let a2495 = {
                        let a2493 = v1;
                        let a2494 = 8u8;
                        fixed_u8::bit_and(a2493, a2494)
                    };
                    let a2496 = 8u8;
                    if equal(a2495, a2496) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2502 = {
                    let a2499 = {
                        let a2497 = v1;
                        let a2498 = 4u8;
                        fixed_u8::bit_and(a2497, a2498)
                    };
                    let a2500 = 4u8;
                    if equal(a2499, a2500) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2501, a2502)?
            };
            let a2514 = {
                let a2511 = {
                    let a2505 = {
                        let a2503 = v1;
                        let a2504 = 2u8;
                        fixed_u8::bit_and(a2503, a2504)
                    };
                    let a2506 = 2u8;
                    if equal(a2505, a2506) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2512 = {
                    let a2509 = {
                        let a2507 = v1;
                        let a2508 = 1u8;
                        fixed_u8::bit_and(a2507, a2508)
                    };
                    let a2510 = 1u8;
                    if equal(a2509, a2510) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2511, a2512)?
            };
            nat_add(a2513, a2514)?
        };
        nat_add(a2515, a2516)?
    };
    let a2523 = f526(f571(), {
        let a2517 = v2;
        let a2518 = 16u64;
        let a2519 = 0u64;
        nat_quot(a2517, a2518, a2519)
    });
    let a2524 = f526(f571(), {
        let a2520 = v2;
        let a2521 = 16u64;
        let a2522 = 0u64;
        nat_rem(a2520, a2521, a2522)
    });
    Ok(append_bytes(a2523, a2524))
}

pub fn f530(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f572(Fn19::F573, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f531(v0: Fn15, p2528: List<List<(Bytes, Bytes)>>, p2529: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2527 = (p2528, p2529);
    {
        let mut m2527 = m2527;
        loop {
            let step = {
                let (v1, v2) = m2527;
                let m2525 = v2;
                match m2525.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2526: Fn15 = v0.clone();
                            c2526.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f532(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f533(v0: Fn16, p2534: u64, p2535: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2533 = (p2534, p2535);
    {
        let mut m2533 = m2533;
        loop {
            let step = {
                let (v1, v2) = m2533;
                let m2530 = v1;
                if m2530 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2530 - 1;
                    let m2532 = {
                        let c2531: Fn16 = v0.clone();
                        c2531.apply(v2.clone())?
                    };
                    match m2532 {
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
                Ok(next) => { m2533 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f534(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f574(v0)
}

pub fn f535(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f575(v0, v1)
}

pub fn f536(v0: u8) -> bool {
    let a2536 = v0;
    let a2537 = 32u8;
    if if if equal(a2536, a2537) {
        true
    } else {
        let a2538 = v0;
        let a2539 = 9u8;
        equal(a2538, a2539)
    } {
        true
    } else {
        let a2540 = v0;
        let a2541 = 10u8;
        equal(a2540, a2541)
    } {
        true
    } else {
        let a2542 = v0;
        let a2543 = 12u8;
        if equal(a2542, a2543) {
            true
        } else {
            let a2544 = v0;
            let a2545 = 13u8;
            equal(a2544, a2545)
        }
    }
}

pub fn f537(v0: Fn17, p2550: u64, p2551: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2549 = (p2550, p2551);
    {
        let mut m2549 = m2549;
        loop {
            let step = {
                let (v1, v2) = m2549;
                let m2546 = v1;
                if m2546 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2546 - 1;
                    let m2548 = {
                        let c2547: Fn17 = v0.clone();
                        c2547.apply(v2.clone())?
                    };
                    match m2548 {
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
                Ok(next) => { m2549 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f576(v0, v1)
}

pub fn f539(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2596 = {
            let a2572 = {
                let a2560 = {
                    let a2554 = {
                        let a2552 = v1;
                        let a2553 = 128u8;
                        fixed_u8::bit_and(a2552, a2553)
                    };
                    let a2555 = 128u8;
                    if equal(a2554, a2555) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2561 = {
                    let a2558 = {
                        let a2556 = v1;
                        let a2557 = 64u8;
                        fixed_u8::bit_and(a2556, a2557)
                    };
                    let a2559 = 64u8;
                    if equal(a2558, a2559) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2560, a2561)?
            };
            let a2573 = {
                let a2570 = {
                    let a2564 = {
                        let a2562 = v1;
                        let a2563 = 32u8;
                        fixed_u8::bit_and(a2562, a2563)
                    };
                    let a2565 = 32u8;
                    if equal(a2564, a2565) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2571 = {
                    let a2568 = {
                        let a2566 = v1;
                        let a2567 = 16u8;
                        fixed_u8::bit_and(a2566, a2567)
                    };
                    let a2569 = 16u8;
                    if equal(a2568, a2569) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2570, a2571)?
            };
            nat_add(a2572, a2573)?
        };
        let a2597 = {
            let a2594 = {
                let a2582 = {
                    let a2576 = {
                        let a2574 = v1;
                        let a2575 = 8u8;
                        fixed_u8::bit_and(a2574, a2575)
                    };
                    let a2577 = 8u8;
                    if equal(a2576, a2577) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2583 = {
                    let a2580 = {
                        let a2578 = v1;
                        let a2579 = 4u8;
                        fixed_u8::bit_and(a2578, a2579)
                    };
                    let a2581 = 4u8;
                    if equal(a2580, a2581) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2582, a2583)?
            };
            let a2595 = {
                let a2592 = {
                    let a2586 = {
                        let a2584 = v1;
                        let a2585 = 2u8;
                        fixed_u8::bit_and(a2584, a2585)
                    };
                    let a2587 = 2u8;
                    if equal(a2586, a2587) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2593 = {
                    let a2590 = {
                        let a2588 = v1;
                        let a2589 = 1u8;
                        fixed_u8::bit_and(a2588, a2589)
                    };
                    let a2591 = 1u8;
                    if equal(a2590, a2591) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2592, a2593)?
            };
            nat_add(a2594, a2595)?
        };
        nat_add(a2596, a2597)?
    };
    let a2598 = 48u64;
    let a2599 = v2;
    if nat_le(a2598, a2599) {
        let a2600 = v2;
        let a2601 = 57u64;
        Ok(nat_le(a2600, a2601))
    } else {
        Ok(false)
    }
}

pub fn f540(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f434({
        let (_, h2602) = {
            let (h2603, _) = f577(Fn20::F578, {
                let a2605 = {
                    let a2604 = v0.clone();
                    length_list(a2604)
                };
                let a2606 = 1u64;
                nat_add(a2605, a2606)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2603
        };
        h2602
    }))
}

pub fn f541(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f542(v0: List<Adt2>) -> List<Bytes> {
    let m2607 = v0;
    match m2607.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2608 = f93(v1, f8(Str::lit("std")));
            match m2608 {
                None => {
                    f542(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f542(v2))
                }
            }
        }
    }
}

pub fn f543(v0: List<Adt2>) -> List<Bytes> {
    let m2609 = v0;
    match m2609.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2610 = f92(f93(v1, f8(Str::lit("req"))));
            match m2610 {
                None => {
                    f543(v2)
                }
                Some(v3) => {
                    List::cons(v3, f543(v2))
                }
            }
        }
    }
}

pub fn f544(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m2611 = v0;
    match m2611 {
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
        Adt2::C5(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C6(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C7(r2612) => {
            let v4 = (*r2612).clone();
            f544(v4)
        }
        Adt2::C8 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C9(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C10(v1, r2613, r2614) => {
            let v2 = (*r2613).clone();
            let v3 = (*r2614).clone();
            List::cons((v1, v2), f544(v3))
        }
    }
}

pub fn f545(p2619: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m2618 = p2619;
    {
        let mut m2618 = m2618;
        loop {
            let step = {
                let v0 = m2618;
                let m2615 = v0;
                match m2615.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f579(f580({
                            let (_, h2616) = v3.clone();
                            h2616
                        }, List::<Bytes>::nil())?, f547({
                            let (h2617, _) = v3.clone();
                            h2617
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
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

pub fn f546(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m2620 = v0;
    match m2620.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h2621, _) = v2.clone();
                h2621
            }, v1.clone()) {
                f546(v3, v1)
            } else {
                List::cons(v2, f546(v3, v1))
            }
        }
    }
}

pub fn f547(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f373(v0)?;
    if f52(v1.clone(), f581()) {
        f582(v1)
    } else {
        Ok(v1)
    }
}

pub fn f548() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f549() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f550(v0: Bytes, v1: Adt2) -> R<Result<Adt27, Bytes>> {
    f583(v0, f584(v1))
}

pub fn f551(v0: Adt27) -> List<Adt24> {
    let m2622 = v0;
    match m2622 {
        Adt27::C0(_, v2) => {
            v2
        }
    }
}

pub fn f552(v0: Adt27) -> Bytes {
    let m2623 = v0;
    match m2623 {
        Adt27::C0(v1, _) => {
            v1
        }
    }
}

pub fn f553(v0: Fn18, p2627: u64, p2628: Adt26) -> R<Adt26> {
    let m2626 = (p2627, p2628);
    {
        let mut m2626 = m2626;
        loop {
            let step = {
                let (v1, v2) = m2626;
                let m2624 = v1;
                if m2624 == 0 {
                    Err::<(u64, Adt26), R<Adt26>>(Ok(v2))
                } else {
                    let v3 = m2624 - 1;
                    Ok::<(u64, Adt26), R<Adt26>>((v3, {
                        let c2625: Fn18 = v0.clone();
                        c2625.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2626 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: Adt26) -> R<Adt26> {
    f585(v0)
}

pub fn f555(v0: u32, v1: u32) -> u32 {
    let m2631 = {
        let a2629 = f586(v0);
        let a2630 = f586(v1);
        fixed_u64::checked_add(a2629, a2630)
    };
    match m2631 {
        None => {
            0u32
        }
        Some(v2) => {
            f587(v2)
        }
    }
}

pub fn f556(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2634 = {
        let a2632 = v3;
        let a2633 = 8u64;
        nat_add(a2632, a2633)?
    };
    let a2635 = v2;
    if nat_eq(a2634, a2635) {
        Ok(f588({
            let a2638 = {
                let a2636 = v1;
                let a2637 = 8u64;
                nat_mul(a2636, a2637)?
            };
            let a2639 = 4294967296u64;
            let a2640 = 0u64;
            nat_quot(a2638, a2639, a2640)
        }))
    } else {
        let a2643 = {
            let a2641 = v3;
            let a2642 = 4u64;
            nat_add(a2641, a2642)?
        };
        let a2644 = v2;
        if nat_eq(a2643, a2644) {
            Ok(f588({
                let a2647 = {
                    let a2645 = v1;
                    let a2646 = 8u64;
                    nat_mul(a2645, a2646)?
                };
                let a2648 = 4294967296u64;
                let a2649 = 0u64;
                nat_rem(a2647, a2648, a2649)
            }))
        } else {
            Ok(f589(f590(v0.clone(), v1, v3), f590(v0.clone(), v1, {
                let a2650 = v3;
                let a2651 = 1u64;
                nat_add(a2650, a2651)?
            }), f590(v0.clone(), v1, {
                let a2652 = v3;
                let a2653 = 2u64;
                nat_add(a2652, a2653)?
            }), f590(v0, v1, {
                let a2654 = v3;
                let a2655 = 3u64;
                nat_add(a2654, a2655)?
            })))
        }
    }
}

pub fn f557(v0: Adt2) -> Option<Bytes> {
    let m2656 = v0;
    match m2656 {
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
        Adt2::C5(_) => {
            None::<Bytes>
        }
        Adt2::C6(r2659) => {
            let v3 = (*r2659).clone();
            f557(v3)
        }
        Adt2::C7(r2660) => {
            let v4 = (*r2660).clone();
            f557(v4)
        }
        Adt2::C8 => {
            None::<Bytes>
        }
        Adt2::C9(r2661, r2662) => {
            let v5 = (*r2661).clone();
            let v6 = (*r2662).clone();
            let m2657 = f557(v5);
            match m2657 {
                None => {
                    f557(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r2663, r2664) => {
            let v9 = (*r2663).clone();
            let v10 = (*r2664).clone();
            let m2658 = f557(v9);
            match m2658 {
                None => {
                    f557(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f558(v0: List<Bytes>) -> Adt2 {
    let m2665 = v0;
    match m2665.uncons() {
        None => {
            Adt2::C8
        }
        Some((v1, v2)) => {
            Adt2::C9(std::rc::Rc::new(Adt2::C5(v1)), std::rc::Rc::new(f558(v2)))
        }
    }
}

pub fn f559(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    Some(f7(List::cons(f8(Str::lit("the instance at \"")), List::cons(v0, List::cons(f8(Str::lit("\" ")), List::cons(v1, List::<Bytes>::nil()))))))
}

pub fn f560(p2670: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m2669 = p2670;
    {
        let mut m2669 = m2669;
        loop {
            let step = {
                let v0 = m2669;
                let m2666 = v0;
                match m2666.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h2667, _) = v1.clone();
                            h2667
                        }, f591()) {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h2668, _) = v1;
                                h2668
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2671 = v0;
    match m2671 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f562(v0: Adt2, v1: Adt2, v2: Bytes) -> R<Option<Bytes>> {
    let v3: Bytes = f592(v1.clone());
    let v7: Option<Bytes> = {
        let m2672 = f93(v0.clone(), f8(Str::lit("type")));
        match m2672 {
            None => {
                None::<Bytes>
            }
            Some(v4) => {
                let v6: List<Bytes> = {
                    let m2673 = f92(Some(v4.clone()));
                    match m2673 {
                        None => {
                            f214(f187(Some(v4)))
                        }
                        Some(v5) => {
                            List::cons(v5, List::<Bytes>::nil())
                        }
                    }
                };
                if f593(v6.clone(), v1.clone()) {
                    None::<Bytes>
                } else {
                    f559(v2.clone(), f7(List::cons(f8(Str::lit("is ")), List::cons(v3, List::cons(f8(Str::lit(", not of the type ")), List::cons(f386(v6, f8(Str::lit(" or "))), List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.1.1)")), List::<Bytes>::nil())))))))
                }
            }
        }
    };
    let v9: Option<Bytes> = {
        let m2674 = f93(v0.clone(), f8(Str::lit("enum")));
        match m2674 {
            None => {
                None::<Bytes>
            }
            Some(v8) => {
                if f594(f187(Some(v8)), v1.clone())? {
                    None::<Bytes>
                } else {
                    f559(v2.clone(), f8(Str::lit("is none of the enum values (JSON Schema 2020-12 Validation section 6.1.2)")))
                }
            }
        }
    };
    let v11: Option<Bytes> = {
        let m2675 = f93(v0.clone(), f8(Str::lit("const")));
        match m2675 {
            None => {
                None::<Bytes>
            }
            Some(v10) => {
                if f595(false, v10, v1.clone())? {
                    None::<Bytes>
                } else {
                    f559(v2.clone(), f8(Str::lit("is not the const value (JSON Schema 2020-12 Validation section 6.1.3)")))
                }
            }
        }
    };
    let v41: Option<Bytes> = {
        let m2676 = v1.clone();
        match m2676 {
            Adt2::C0 => {
                None::<Bytes>
            }
            Adt2::C1(v28, _) => {
                f559(v2.clone(), f7(List::cons(f8(Str::lit("is tagged ")), List::cons(v28, List::cons(f8(Str::lit(" and has no JSON type (RFC 8259 section 3)")), List::<Bytes>::nil())))))
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
            Adt2::C5(v12) => {
                let m2677 = f596(v12.clone())?;
                match m2677 {
                    None => {
                        f559(v2.clone(), f8(Str::lit("is not well-formed UTF-8 (RFC 3629 section 4)")))
                    }
                    Some(v13) => {
                        let v14: u64 = f597(v13)?;
                        f561({
                            let m2678 = f492(f93(v0.clone(), f8(Str::lit("minLength"))));
                            match m2678 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v15) => {
                                    let a2679 = v14;
                                    let a2680 = v15;
                                    if nat_lt(a2679, a2680) {
                                        f559(v2.clone(), f8(Str::lit("is shorter than minLength (JSON Schema 2020-12 Validation section 6.3.2)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, f561({
                            let m2681 = f492(f93(v0.clone(), f8(Str::lit("maxLength"))));
                            match m2681 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v16) => {
                                    let a2682 = v16;
                                    let a2683 = v14;
                                    if nat_lt(a2682, a2683) {
                                        f559(v2.clone(), f8(Str::lit("is longer than maxLength (JSON Schema 2020-12 Validation section 6.3.1)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, {
                            let m2684 = f92(f93(v0.clone(), f8(Str::lit("pattern"))));
                            match m2684 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v17) => {
                                    let m2685 = f598(v17.clone(), v12)?;
                                    match m2685 {
                                        Ok(v19) => {
                                            if v19 {
                                                None::<Bytes>
                                            } else {
                                                f559(v2.clone(), f7(List::cons(f8(Str::lit("does not match the pattern ")), List::cons(v17, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.3.3)")), List::<Bytes>::nil())))))
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
            Adt2::C6(r2693) => {
                let v20 = (*r2693).clone();
                let v21: u64 = f599(v20)?;
                f561({
                    let m2686 = f492(f93(v0.clone(), f8(Str::lit("minItems"))));
                    match m2686 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v22) => {
                            let a2687 = v21;
                            let a2688 = v22;
                            if nat_lt(a2687, a2688) {
                                f559(v2.clone(), f8(Str::lit("has fewer items than minItems (JSON Schema 2020-12 Validation section 6.4.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2689 = f492(f93(v0.clone(), f8(Str::lit("maxItems"))));
                    match m2689 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v23) => {
                            let a2690 = v23;
                            let a2691 = v21;
                            if nat_lt(a2690, a2691) {
                                f559(v2.clone(), f8(Str::lit("has more items than maxItems (JSON Schema 2020-12 Validation section 6.4.1)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
            Adt2::C7(r2694) => {
                let v24 = (*r2694).clone();
                let v25: List<Bytes> = f600(f214(f187(f93(v0.clone(), f8(Str::lit("required"))))), f601(f544(v24)));
                let m2692 = v25;
                match m2692.uncons() {
                    None => {
                        None::<Bytes>
                    }
                    Some((v26, _)) => {
                        f559(v2.clone(), f7(List::cons(f8(Str::lit("lacks the required member ")), List::cons(v26, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.5.3)")), List::<Bytes>::nil())))))
                    }
                }
            }
            Adt2::C8 => {
                None::<Bytes>
            }
            Adt2::C9(_, _) => {
                None::<Bytes>
            }
            Adt2::C10(_, _, _) => {
                None::<Bytes>
            }
        }
    };
    let v47: Option<Bytes> = {
        let m2695 = f602(v1);
        match m2695 {
            None => {
                None::<Bytes>
            }
            Some(v42) => {
                f561({
                    let m2697 = {
                        let m2696 = f93(v0.clone(), f8(Str::lit("minimum")));
                        match m2696 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v43) => {
                                f602(v43)
                            }
                        }
                    };
                    match m2697 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v44) => {
                            if f603(v42, v44)? {
                                f559(v2.clone(), f8(Str::lit("is less than minimum (JSON Schema 2020-12 Validation section 6.2.4)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2699 = {
                        let m2698 = f93(v0, f8(Str::lit("maximum")));
                        match m2698 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v45) => {
                                f602(v45)
                            }
                        }
                    };
                    match m2699 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v46) => {
                            if f603(v46, v42)? {
                                f559(v2, f8(Str::lit("is greater than maximum (JSON Schema 2020-12 Validation section 6.2.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
        }
    };
    Ok(f561(v7, f561(v9, f561(v11, f561(v41, v47)))))
}

pub fn f563(v0: Adt2, v1: Bytes) -> R<Result<Adt2, Bytes>> {
    let v2: u64 = {
        let a2700 = v1.clone();
        length_bytes(a2700)
    };
    if f32(v1.clone(), f8(Str::lit("#"))) {
        Ok(Ok::<Adt2, Bytes>(v0))
    } else {
        let m2705 = if f85(v1.clone(), f8(Str::lit("#/$defs/"))) {
            Some((f8(Str::lit("$defs")), f46(v1.clone(), 8u64, {
                let a2701 = v2;
                let a2702 = 8u64;
                nat_sub(a2701, a2702)
            })))
        } else {
            if f85(v1.clone(), f8(Str::lit("#/definitions/"))) {
                Some((f8(Str::lit("definitions")), f46(v1.clone(), 14u64, {
                    let a2703 = v2;
                    let a2704 = 14u64;
                    nat_sub(a2703, a2704)
                })))
            } else {
                None::<(Bytes, Bytes)>
            }
        };
        match m2705 {
            None => {
                Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" is not resolved: only #, #/$defs/NAME and #/definitions/NAME in the same document are (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
            }
            Some(v3) => {
                let v4: Bytes = f604({
                    let a2708 = {
                        let a2707 = {
                            let (_, h2706) = v3.clone();
                            h2706
                        };
                        length_bytes(a2707)
                    };
                    let a2709 = 1u64;
                    nat_add(a2708, a2709)?
                }, {
                    let (_, h2710) = v3;
                    h2710
                }, 0u64, Bytes::lit(&[]))?;
                let m2713 = {
                    let m2712 = f93(v0, {
                        let (h2711, _) = v3;
                        h2711
                    });
                    match m2712 {
                        None => {
                            None::<Adt2>
                        }
                        Some(v5) => {
                            f93(v5, v4)
                        }
                    }
                };
                match m2713 {
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

pub fn f564(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2714 = v0;
    if m2714 == 0 {
        Ok(Some(f8(Str::lit("the allOf branches do not end (JSON Schema 2020-12 Core section 10.2.1.1)"))))
    } else {
        let v5 = m2714 - 1;
        let m2715 = v2;
        match m2715.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f561(f520(v5, v1.clone(), v6, v3.clone(), v4.clone())?, f564(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f565(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<u64> {
    let m2716 = v0;
    if m2716 == 0 {
        Ok(0u64)
    } else {
        let v5 = m2716 - 1;
        let m2717 = v2;
        match m2717.uncons() {
            None => {
                Ok(0u64)
            }
            Some((v6, v7)) => {
                let a2719 = {
                    let m2718 = f520(v5, v1.clone(), v6, v3.clone(), v4.clone())?;
                    match m2718 {
                        None => {
                            1u64
                        }
                        Some(_) => {
                            0u64
                        }
                    }
                };
                let a2720 = f565(v5, v1, v7, v3, v4)?;
                nat_add(a2719, a2720)
            }
        }
    }
}

pub fn f566(v0: u64, v1: Adt2, v2: List<(Bytes, Adt2)>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2721 = v0;
    if m2721 == 0 {
        Ok(Some(f8(Str::lit("the properties do not end (JSON Schema 2020-12 Core section 10.3.2.1)"))))
    } else {
        let v5 = m2721 - 1;
        let m2722 = v2;
        match m2722.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f561({
                    let m2724 = f93(v3.clone(), {
                        let (h2723, _) = v6.clone();
                        h2723
                    });
                    match m2724 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v8) => {
                            f520(v5, v1.clone(), {
                                let (_, h2725) = v6;
                                h2725
                            }, v8, f605(v4.clone(), {
                                let (h2726, _) = v6;
                                h2726
                            })?)?
                        }
                    }
                }, f566(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f567(v0: u64, v1: Adt2, v2: Adt2, v3: List<(Bytes, Adt2)>, v4: Bytes) -> R<Option<Bytes>> {
    let m2727 = v0;
    if m2727 == 0 {
        Ok(Some(f8(Str::lit("the members do not end (JSON Schema 2020-12 Core section 10.3.2.2)"))))
    } else {
        let v5 = m2727 - 1;
        let m2728 = v3;
        match m2728.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                let v8: Bytes = f605(v4.clone(), {
                    let (h2729, _) = v6.clone();
                    h2729
                })?;
                let m2731 = f606(f508(f93(v2.clone(), f8(Str::lit("patternProperties")))), {
                    let (h2730, _) = v6.clone();
                    h2730
                })?;
                match m2731 {
                    Ok(v10) => {
                        Ok(f561(f564(v5, v1.clone(), v10.clone(), {
                            let (_, h2732) = v6.clone();
                            h2732
                        }, v8.clone())?, f561({
                            let m2733 = v10;
                            match m2733.uncons() {
                                None => {
                                    if f52({
                                        let (h2734, _) = v6;
                                        h2734
                                    }, f601(f544({
                                        let m2735 = f93(v2.clone(), f8(Str::lit("properties")));
                                        match m2735 {
                                            None => {
                                                Adt2::C7(std::rc::Rc::new(Adt2::C8))
                                            }
                                            Some(v13) => {
                                                v13
                                            }
                                        }
                                    }))) {
                                        None::<Bytes>
                                    } else {
                                        let m2736 = f93(v2.clone(), f8(Str::lit("additionalProperties")));
                                        match m2736 {
                                            None => {
                                                None::<Bytes>
                                            }
                                            Some(v14) => {
                                                f520(v5, v1.clone(), v14, {
                                                    let (_, h2737) = v6;
                                                    h2737
                                                }, v8)?
                                            }
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    None::<Bytes>
                                }
                            }
                        }, f567(v5, v1, v2, v7, v4)?)))
                    }
                    Err(v9) => {
                        Ok(Some(v9))
                    }
                }
            }
        }
    }
}

pub fn f568(v0: u64, v1: Adt2, v2: Adt2, v3: List<Adt2>, v4: Bytes, v5: u64) -> R<Option<Bytes>> {
    let m2738 = v0;
    if m2738 == 0 {
        Ok(Some(f8(Str::lit("the items do not end (JSON Schema 2020-12 Core section 10.3.1.2)"))))
    } else {
        let v6 = m2738 - 1;
        let m2739 = v3;
        match m2739.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v7, v8)) => {
                Ok(f561(f520(v6, v1.clone(), v2.clone(), v7, f605(v4.clone(), f395(v5)?)?)?, f568(v6, v1, v2, v8, v4, {
                    let a2740 = v5;
                    let a2741 = 1u64;
                    nat_add(a2740, a2741)?
                })?))
            }
        }
    }
}

pub fn f569(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2742 = v0;
    match m2742.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f569(v3, false))
            } else {
                f569(v3, true)
            }
        }
    }
}

pub fn f570(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2743, _) = v2;
        h2743
    };
    let a2744 = v3;
    let a2745 = 0u64;
    if nat_eq(a2744, a2745) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2746 = v3;
            let a2747 = v1;
            let a2748 = 0u64;
            nat_quot(a2746, a2747, a2748)
        }, {
            let a2753 = f526(v0, {
                let a2749 = v3;
                let a2750 = v1;
                let a2751 = 0u64;
                nat_rem(a2749, a2750, a2751)
            });
            let a2754 = {
                let (_, h2752) = v2;
                h2752
            };
            append_bytes(a2753, a2754)
        }))
    }
}

pub fn f571() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f572(v0: Fn19, p2758: List<List<(Bytes, Bytes)>>, p2759: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2757 = (p2758, p2759);
    {
        let mut m2757 = m2757;
        loop {
            let step = {
                let (v1, v2) = m2757;
                let m2755 = v2;
                match m2755.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2756: Fn19 = v0.clone();
                            c2756.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2757 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f574(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2761 = {
        let a2760 = v0.clone();
        length_list(a2760)
    };
    let a2762 = 1u64;
    if nat_le(a2761, a2762) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f607(v0)?))
    }
}

pub fn f575(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2766 = {
        let a2764 = v0.clone();
        let a2765 = {
            let (h2763, _) = v1.clone();
            h2763
        };
        index_bytes(a2764, a2765)
    };
    match m2766 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2768 = {
                    let (h2767, _) = v1.clone();
                    h2767
                };
                let a2769 = 1u64;
                nat_add(a2768, a2769)?
            }, {
                let a2772 = {
                    let (_, h2770) = v1;
                    h2770
                };
                let a2773 = f475(v0, {
                    let (h2771, _) = v1;
                    h2771
                }, v2)?;
                append_bytes(a2772, a2773)
            })))
        }
    }
}

pub fn f576(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2774, _) = v1;
        h2774
    };
    let m2776 = {
        let (_, h2775) = v1;
        h2775
    };
    match m2776 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2779 = {
                let a2777 = v0.clone();
                let a2778 = v2;
                index_bytes(a2777, a2778)
            };
            match m2779 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2784 = {
                        let a2782 = v0;
                        let a2783 = {
                            let a2780 = v2;
                            let a2781 = 1u64;
                            nat_add(a2780, a2781)?
                        };
                        index_bytes(a2782, a2783)
                    };
                    match m2784 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f608(v4)? {
                                f608(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2785 = v2;
                                    let a2786 = 2u64;
                                    nat_add(a2785, a2786)?
                                }, Some({
                                    let a2791 = v3;
                                    let a2792 = f526(f609(), {
                                        let a2789 = {
                                            let a2787 = f610(v4)?;
                                            let a2788 = 16u64;
                                            nat_mul(a2787, a2788)?
                                        };
                                        let a2790 = f610(v5)?;
                                        nat_add(a2789, a2790)?
                                    });
                                    append_bytes(a2791, a2792)
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

pub fn f577(v0: Fn20, p2797: u64, p2798: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2796 = (p2797, p2798);
    {
        let mut m2796 = m2796;
        loop {
            let step = {
                let (v1, v2) = m2796;
                let m2793 = v1;
                if m2793 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2793 - 1;
                    let m2795 = {
                        let c2794: Fn20 = v0.clone();
                        c2794.apply(v2.clone())?
                    };
                    match m2795 {
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
                Ok(next) => { m2796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f611(v0)
}

pub fn f579(p2801: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2800 = p2801;
    {
        let mut m2800 = m2800;
        loop {
            let step = {
                let v0 = m2800;
                let m2799 = v0;
                match m2799.uncons() {
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
                Ok(next) => { m2800 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f580(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2802 = v0;
    match m2802 {
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
        Adt2::C5(_) => {
            Ok(v1)
        }
        Adt2::C6(r2805) => {
            let v2 = (*r2805).clone();
            f580(v2, v1)
        }
        Adt2::C7(r2806) => {
            let v3 = (*r2806).clone();
            f580(v3, v1)
        }
        Adt2::C8 => {
            Ok(v1)
        }
        Adt2::C9(r2807, r2808) => {
            let v4 = (*r2807).clone();
            let v5 = (*r2808).clone();
            f580(v4, f580(v5, v1)?)
        }
        Adt2::C10(v6, r2809, r2810) => {
            let v7 = (*r2809).clone();
            let v8 = (*r2810).clone();
            let v9: List<Bytes> = f580(v7.clone(), f580(v8, v1)?)?;
            if f32(v6, f8(Str::lit("$ref"))) {
                let m2803 = f92(Some(v7));
                match m2803 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2804 = f612(v10)?;
                        match m2804 {
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

pub fn f581() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f582(v0: Bytes) -> R<Bytes> {
    let m2813 = {
        let a2811 = v0.clone();
        let a2812 = 0u64;
        index_bytes(a2811, a2812)
    };
    match m2813 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2819 = {
                let a2814 = f8(Str::lit("u"));
                let a2815 = f613(8u64, f72(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2814, a2815)
            };
            let a2820 = f46(v0.clone(), 1u64, {
                let a2817 = {
                    let a2816 = v0;
                    length_bytes(a2816)
                };
                let a2818 = 1u64;
                nat_sub(a2817, a2818)
            });
            Ok(append_bytes(a2819, a2820))
        }
    }
}

pub fn f583(v0: Bytes, v1: Adt2) -> R<Result<Adt27, Bytes>> {
    let m2821 = f93(v1.clone(), f8(Str::lit("enum")));
    match m2821 {
        None => {
            let m2823 = f93(v1.clone(), f8(Str::lit("oneOf")));
            match m2823 {
                None => {
                    if f618(v1.clone(), v0.clone())? {
                        let m2824 = f619(v0.clone(), f508(f93(v1.clone(), f8(Str::lit("properties")))), f214(f187(f93(v1, f8(Str::lit("required"))))), true)?;
                        match m2824 {
                            Ok(v7) => {
                                Ok(f616(f620(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f386(f621(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f622(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt27, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2825 = f508(f93(v1.clone(), f8(Str::lit("properties"))));
                        match m2825.uncons() {
                            None => {
                                if f623(v1.clone()) {
                                    Ok(f616(f624(v0), List::<Adt24>::nil()))
                                } else {
                                    let m2828 = f625(v1, {
                                        let a2826 = v0.clone();
                                        let a2827 = f8(Str::lit("Value"));
                                        append_bytes(a2826, a2827)
                                    })?;
                                    match m2828 {
                                        Ok(v9) => {
                                            Ok(f616(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f552(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f551(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt27, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2829 = f619(v0.clone(), f508(f93(v1.clone(), f8(Str::lit("properties")))), f214(f187(f93(v1, f8(Str::lit("required"))))), false)?;
                                match m2829 {
                                    Ok(v13) => {
                                        Ok(f616(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2830 = f621(v13.clone());
                                            match m2830.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f386(f621(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f622(v13)))
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
                    f617(v0, f295(v5))
                }
            }
        }
        Some(v2) => {
            let m2822 = f614(f295(v2), List::<Bytes>::nil())?;
            match m2822 {
                Ok(v4) => {
                    Ok(f616(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f386(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt24>::nil()))
                }
                Err(v3) => {
                    Ok(f615(v3, v0))
                }
            }
        }
    }
}

pub fn f584(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m2831 = f93(v0.clone(), f8(Str::lit("anyOf")));
        match m2831 {
            None => {
                v0
            }
            Some(v1) => {
                if f626(f295(v1.clone())) {
                    f627(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f628(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f629(f295(v1.clone()))
                    } {
                        v0
                    } else {
                        f630(f627(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m2832 = f93(v2.clone(), f8(Str::lit("oneOf")));
        match m2832 {
            None => {
                v2
            }
            Some(v3) => {
                if f626(f295(v3)) {
                    f627(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2833 = f93(v4.clone(), f8(Str::lit("allOf")));
    match m2833 {
        None => {
            v4
        }
        Some(v5) => {
            if f631(f295(v5)) {
                f627(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f585(v0: Adt26) -> R<Adt26> {
    let v1: u32 = f632(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2834) => {
            h2834
        }
    });
    let v2: u32 = f555(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h2835, _, _) => {
            h2835
        }
    }, f555(f633(match v0.clone() {
        Adt26::C0(_, _, _, _, h2836, _, _, _, _, _) => {
            h2836
        }
    }), f555(f634(match v0.clone() {
        Adt26::C0(_, _, _, _, h2837, _, _, _, _, _) => {
            h2837
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2838, _, _, _, _) => {
            h2838
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2839, _, _, _) => {
            h2839
        }
    }), f555(v1, {
        let m2842 = match v0.clone() {
            Adt26::C0(_, _, _, _, _, _, _, _, h2841, _) => {
                h2841
            }
        };
        match m2842 {
            Adt25::C0(h2840, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2840
            }
        }
    }))));
    let v3: u32 = f555(f635(match v0.clone() {
        Adt26::C0(h2843, _, _, _, _, _, _, _, _, _) => {
            h2843
        }
    }), f636(match v0.clone() {
        Adt26::C0(h2844, _, _, _, _, _, _, _, _, _) => {
            h2844
        }
    }, match v0.clone() {
        Adt26::C0(_, h2845, _, _, _, _, _, _, _, _) => {
            h2845
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2846, _, _, _, _, _, _, _) => {
            h2846
        }
    }));
    Ok(Adt26::C0(f555(v2, v3), match v0.clone() {
        Adt26::C0(h2847, _, _, _, _, _, _, _, _, _) => {
            h2847
        }
    }, match v0.clone() {
        Adt26::C0(_, h2848, _, _, _, _, _, _, _, _) => {
            h2848
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2849, _, _, _, _, _, _, _) => {
            h2849
        }
    }, f555(match v0.clone() {
        Adt26::C0(_, _, _, h2850, _, _, _, _, _, _) => {
            h2850
        }
    }, v2), match v0.clone() {
        Adt26::C0(_, _, _, _, h2851, _, _, _, _, _) => {
            h2851
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2852, _, _, _, _) => {
            h2852
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2853, _, _, _) => {
            h2853
        }
    }, f637(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h2854, _) => {
            h2854
        }
    }), {
        let a2856 = match v0 {
            Adt26::C0(_, _, _, _, _, _, _, _, _, h2855) => {
                h2855
            }
        };
        let a2857 = 1u64;
        nat_add(a2856, a2857)?
    }))
}

pub fn f586(v0: u32) -> u64 {
    let m2859 = {
        let a2858 = v0;
        fixed_u64::convert(i128::from(a2858))
    };
    match m2859 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f587(v0: u64) -> u32 {
    let m2863 = {
        let a2862 = {
            let a2860 = v0;
            let a2861 = 4294967295u64;
            fixed_u64::bit_and(a2860, a2861)
        };
        fixed_u32::convert(i128::from(a2862))
    };
    match m2863 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f588(v0: u64) -> u32 {
    let (_, h2864) = {
        let (_, h2865) = f638(Fn21::F639, 32u64, (v0, (1u32, 0u32)));
        h2865
    };
    h2864
}

pub fn f589(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2870 = f640(v0, 24u32);
    let a2871 = {
        let a2868 = f640(v1, 16u32);
        let a2869 = {
            let a2866 = f640(v2, 8u32);
            let a2867 = v3;
            fixed_u32::bit_or(a2866, a2867)
        };
        fixed_u32::bit_or(a2868, a2869)
    };
    fixed_u32::bit_or(a2870, a2871)
}

pub fn f590(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2872 = v2;
    let a2873 = v1;
    if nat_lt(a2872, a2873) {
        let m2876 = {
            let a2874 = v0;
            let a2875 = v2;
            index_bytes(a2874, a2875)
        };
        match m2876 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2878 = {
                    let a2877 = v3;
                    fixed_u32::convert(i128::from(a2877))
                };
                match m2878 {
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
        let a2879 = v2;
        let a2880 = v1;
        if nat_eq(a2879, a2880) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f591() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("patternProperties")), List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("definitions")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("format")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))
}

pub fn f592(v0: Adt2) -> Bytes {
    let m2881 = v0;
    match m2881 {
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
        Adt2::C5(_) => {
            f8(Str::lit("string"))
        }
        Adt2::C6(_) => {
            f8(Str::lit("array"))
        }
        Adt2::C7(_) => {
            f8(Str::lit("object"))
        }
        Adt2::C8 => {
            f8(Str::lit("spine"))
        }
        Adt2::C9(_, _) => {
            f8(Str::lit("spine"))
        }
        Adt2::C10(_, _, _) => {
            f8(Str::lit("spine"))
        }
    }
}

pub fn f593(p2884: List<Bytes>, v1: Adt2) -> bool {
    let m2883 = p2884;
    {
        let mut m2883 = m2883;
        loop {
            let step = {
                let v0 = m2883;
                let m2882 = v0;
                match m2882.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f641(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2883 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f594(p2887: List<Adt2>, v1: Adt2) -> R<bool> {
    let m2886 = p2887;
    {
        let mut m2886 = m2886;
        loop {
            let step = {
                let v0 = m2886;
                let m2885 = v0;
                match m2885.uncons() {
                    None => {
                        Err::<List<Adt2>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f595(false, v2, v1.clone())? {
                            Err::<List<Adt2>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt2>, R<bool>>(v3)
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

pub fn f595(v0: bool, v1: Adt2, v2: Adt2) -> R<bool> {
    let m2888 = v1;
    match m2888 {
        Adt2::C0 => {
            Ok(f32(f521(v2)?, f8(Str::lit("null"))))
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(v77) => {
            Ok(f32(f521(v2)?, f521(Adt2::C2(v77))?))
        }
        Adt2::C3(v78, v79) => {
            Ok(f32(f521(v2)?, f521(Adt2::C3(v78, v79))?))
        }
        Adt2::C4(v80, v81, v82) => {
            Ok(f32(f521(v2)?, f521(Adt2::C4(v80, v81, v82))?))
        }
        Adt2::C5(v83) => {
            let m2900 = v2;
            match m2900 {
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
                Adt2::C5(v84) => {
                    Ok(f32(v83, v84))
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8 => {
                    Ok(false)
                }
                Adt2::C9(_, _) => {
                    Ok(false)
                }
                Adt2::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C6(r2901) => {
            let v58 = (*r2901).clone();
            let m2898 = v2;
            match m2898 {
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
                Adt2::C5(_) => {
                    Ok(false)
                }
                Adt2::C6(r2899) => {
                    let v59 = (*r2899).clone();
                    f595(false, v58, v59)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8 => {
                    Ok(false)
                }
                Adt2::C9(_, _) => {
                    Ok(false)
                }
                Adt2::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C7(r2902) => {
            let v41 = (*r2902).clone();
            let m2894 = v2;
            match m2894 {
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
                Adt2::C5(_) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(r2897) => {
                    let v42 = (*r2897).clone();
                    if f117({
                        let a2895 = f599(v41.clone())?;
                        let a2896 = f599(v42.clone())?;
                        nat_eq(a2895, a2896)
                    }, true) {
                        f595(true, v41, v42)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C8 => {
                    Ok(false)
                }
                Adt2::C9(_, _) => {
                    Ok(false)
                }
                Adt2::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C8 => {
            if v0 {
                Ok(true)
            } else {
                let m2890 = v2;
                match m2890 {
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
                    Adt2::C5(_) => {
                        Ok(false)
                    }
                    Adt2::C6(_) => {
                        Ok(false)
                    }
                    Adt2::C7(_) => {
                        Ok(false)
                    }
                    Adt2::C8 => {
                        Ok(true)
                    }
                    Adt2::C9(_, _) => {
                        Ok(false)
                    }
                    Adt2::C10(_, _, _) => {
                        Ok(false)
                    }
                }
            }
        }
        Adt2::C9(r2903, r2904) => {
            let v23 = (*r2903).clone();
            let v24 = (*r2904).clone();
            let m2891 = v2;
            match m2891 {
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
                Adt2::C5(_) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8 => {
                    Ok(false)
                }
                Adt2::C9(r2892, r2893) => {
                    let v25 = (*r2892).clone();
                    let v26 = (*r2893).clone();
                    if f595(false, v23, v25)? {
                        f595(false, v24, v26)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C10(v3, r2905, r2906) => {
            let v4 = (*r2905).clone();
            let v5 = (*r2906).clone();
            let m2889 = f93(v2.clone(), v3);
            if match m2889 {
                None => {
                    false
                }
                Some(v6) => {
                    f595(false, v4, v6)?
                }
            } {
                f595(true, v5, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f596(v0: Bytes) -> R<Option<List<u64>>> {
    f642({
        let a2908 = {
            let a2907 = v0.clone();
            length_bytes(a2907)
        };
        let a2909 = 1u64;
        nat_add(a2908, a2909)?
    }, v0, 0u64)
}

pub fn f597(v0: List<u64>) -> R<u64> {
    let m2910 = v0;
    match m2910.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a2911 = f597(v2)?;
            let a2912 = 1u64;
            nat_add(a2911, a2912)
        }
    }
}

pub fn f598(v0: Bytes, v1: Bytes) -> R<Result<bool, Bytes>> {
    let m2913 = f643(v0)?;
    match m2913 {
        Ok(v3) => {
            let m2914 = f596(v1)?;
            match m2914 {
                None => {
                    Ok(Err::<bool, Bytes>(f8(Str::lit("a string is not well-formed UTF-8 (RFC 3629 section 4)"))))
                }
                Some(v4) => {
                    Ok(Ok::<bool, Bytes>({
                        let (h2915, _) = {
                            let (h2916, _) = v3.clone();
                            h2916
                        };
                        if h2915 {
                            f644({
                                let a2920 = {
                                    let a2918 = f645({
                                        let (_, h2917) = v3.clone();
                                        h2917
                                    })?;
                                    let a2919 = f597(v4.clone())?;
                                    nat_add(a2918, a2919)?
                                };
                                let a2921 = 2u64;
                                nat_add(a2920, a2921)?
                            }, {
                                let (_, h2922) = v3;
                                h2922
                            }, v4, {
                                let (_, h2923) = {
                                    let (h2924, _) = v3;
                                    h2924
                                };
                                h2923
                            })?
                        } else {
                            f646({
                                let (_, h2925) = v3;
                                h2925
                            }, v4, {
                                let (_, h2926) = {
                                    let (h2927, _) = v3;
                                    h2927
                                };
                                h2926
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

pub fn f599(v0: Adt2) -> R<u64> {
    let m2928 = v0;
    match m2928 {
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
        Adt2::C5(_) => {
            Ok(0u64)
        }
        Adt2::C6(_) => {
            Ok(0u64)
        }
        Adt2::C7(_) => {
            Ok(0u64)
        }
        Adt2::C8 => {
            Ok(0u64)
        }
        Adt2::C9(_, r2933) => {
            let v5 = (*r2933).clone();
            let a2931 = f599(v5)?;
            let a2932 = 1u64;
            nat_add(a2931, a2932)
        }
        Adt2::C10(_, _, r2934) => {
            let v3 = (*r2934).clone();
            let a2929 = f599(v3)?;
            let a2930 = 1u64;
            nat_add(a2929, a2930)
        }
    }
}

pub fn f600(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2935 = v0;
    match m2935.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f600(v3, v1)
            } else {
                List::cons(v2, f600(v3, v1))
            }
        }
    }
}

pub fn f601(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m2936 = v0;
    match m2936.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2937, _) = v1;
                h2937
            }, f601(v2))
        }
    }
}

pub fn f602(v0: Adt2) -> Option<(bool, (u64, u64))> {
    let m2938 = v0;
    match m2938 {
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
        Adt2::C5(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C6(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C7(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C8 => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C9(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C10(_, _, _) => {
            None::<(bool, (u64, u64))>
        }
    }
}

pub fn f603(v0: (bool, (u64, u64)), v1: (bool, (u64, u64))) -> R<bool> {
    let v2: u64 = {
        let a2943 = {
            let (h2939, _) = {
                let (_, h2940) = v0;
                h2940
            };
            h2939
        };
        let a2944 = f647({
            let (_, h2941) = {
                let (_, h2942) = v1;
                h2942
            };
            h2941
        })?;
        nat_mul(a2943, a2944)?
    };
    let v3: u64 = {
        let a2949 = {
            let (h2945, _) = {
                let (_, h2946) = v1;
                h2946
            };
            h2945
        };
        let a2950 = f647({
            let (_, h2947) = {
                let (_, h2948) = v0;
                h2948
            };
            h2947
        })?;
        nat_mul(a2949, a2950)?
    };
    let (h2951, _) = v0;
    if h2951 {
        let (h2952, _) = v1;
        if h2952 {
            let a2953 = v3;
            let a2954 = v2;
            Ok(nat_lt(a2953, a2954))
        } else {
            Ok(true)
        }
    } else {
        let (h2955, _) = v1;
        if h2955 {
            Ok(false)
        } else {
            let a2956 = v2;
            let a2957 = v3;
            Ok(nat_lt(a2956, a2957))
        }
    }
}

pub fn f604(p2987: u64, v1: Bytes, p2988: u64, p2989: Bytes) -> R<Bytes> {
    let m2986 = (p2987, (p2988, p2989));
    {
        let mut m2986 = m2986;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2986;
                let m2958 = v0;
                if m2958 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2958 - 1;
                    let a2960 = {
                        let a2959 = v1.clone();
                        length_bytes(a2959)
                    };
                    let a2961 = v2;
                    if nat_le(a2960, a2961) {
                        Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                    } else {
                        if f117({
                            let a2962 = f115(v1.clone(), v2)?;
                            let a2963 = 126u64;
                            nat_eq(a2962, a2963)
                        }, {
                            let a2966 = f115(v1.clone(), {
                                let a2964 = v2;
                                let a2965 = 1u64;
                                nat_add(a2964, a2965)?
                            })?;
                            let a2967 = 48u64;
                            nat_eq(a2966, a2967)
                        }) {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a2968 = v2;
                                let a2969 = 2u64;
                                nat_add(a2968, a2969)?
                            }, {
                                let a2970 = v3;
                                let a2971 = f8(Str::lit("~"));
                                append_bytes(a2970, a2971)
                            })))
                        } else {
                            if f117({
                                let a2972 = f115(v1.clone(), v2)?;
                                let a2973 = 126u64;
                                nat_eq(a2972, a2973)
                            }, {
                                let a2976 = f115(v1.clone(), {
                                    let a2974 = v2;
                                    let a2975 = 1u64;
                                    nat_add(a2974, a2975)?
                                })?;
                                let a2977 = 49u64;
                                nat_eq(a2976, a2977)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a2978 = v2;
                                    let a2979 = 2u64;
                                    nat_add(a2978, a2979)?
                                }, {
                                    let a2980 = v3;
                                    let a2981 = f8(Str::lit("/"));
                                    append_bytes(a2980, a2981)
                                })))
                            } else {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a2982 = v2;
                                    let a2983 = 1u64;
                                    nat_add(a2982, a2983)?
                                }, {
                                    let a2984 = v3;
                                    let a2985 = f46(v1.clone(), v2, 1u64);
                                    append_bytes(a2984, a2985)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2986 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f605(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(f648({
        let a2991 = {
            let a2990 = v1.clone();
            length_bytes(a2990)
        };
        let a2992 = 1u64;
        nat_add(a2991, a2992)?
    }, v1, 0u64, Bytes::lit(&[]))?, List::<Bytes>::nil())))))
}

pub fn f606(v0: List<(Bytes, Adt2)>, v1: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let m2993 = v0;
    match m2993.uncons() {
        None => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let m2995 = f598({
                let (h2994, _) = v2;
                h2994
            }, v1.clone())?;
            match m2995 {
                Ok(v5) => {
                    let m2996 = f606(v3, v1)?;
                    match m2996 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt2>, Bytes>(if v5 {
                                List::cons({
                                    let (_, h2997) = v2;
                                    h2997
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

pub fn f607(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f530({
        let (_, h2998) = {
            let (h2999, _) = f649(Fn22::F650, {
                let a3001 = {
                    let a3000 = v0.clone();
                    length_list(a3000)
                };
                let a3002 = 1u64;
                nat_add(a3001, a3002)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2999
        };
        h2998
    }))
}

pub fn f608(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3047 = {
            let a3023 = {
                let a3011 = {
                    let a3005 = {
                        let a3003 = v1;
                        let a3004 = 128u8;
                        fixed_u8::bit_and(a3003, a3004)
                    };
                    let a3006 = 128u8;
                    if equal(a3005, a3006) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3012 = {
                    let a3009 = {
                        let a3007 = v1;
                        let a3008 = 64u8;
                        fixed_u8::bit_and(a3007, a3008)
                    };
                    let a3010 = 64u8;
                    if equal(a3009, a3010) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3011, a3012)?
            };
            let a3024 = {
                let a3021 = {
                    let a3015 = {
                        let a3013 = v1;
                        let a3014 = 32u8;
                        fixed_u8::bit_and(a3013, a3014)
                    };
                    let a3016 = 32u8;
                    if equal(a3015, a3016) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3022 = {
                    let a3019 = {
                        let a3017 = v1;
                        let a3018 = 16u8;
                        fixed_u8::bit_and(a3017, a3018)
                    };
                    let a3020 = 16u8;
                    if equal(a3019, a3020) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3021, a3022)?
            };
            nat_add(a3023, a3024)?
        };
        let a3048 = {
            let a3045 = {
                let a3033 = {
                    let a3027 = {
                        let a3025 = v1;
                        let a3026 = 8u8;
                        fixed_u8::bit_and(a3025, a3026)
                    };
                    let a3028 = 8u8;
                    if equal(a3027, a3028) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3034 = {
                    let a3031 = {
                        let a3029 = v1;
                        let a3030 = 4u8;
                        fixed_u8::bit_and(a3029, a3030)
                    };
                    let a3032 = 4u8;
                    if equal(a3031, a3032) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3033, a3034)?
            };
            let a3046 = {
                let a3043 = {
                    let a3037 = {
                        let a3035 = v1;
                        let a3036 = 2u8;
                        fixed_u8::bit_and(a3035, a3036)
                    };
                    let a3038 = 2u8;
                    if equal(a3037, a3038) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3044 = {
                    let a3041 = {
                        let a3039 = v1;
                        let a3040 = 1u8;
                        fixed_u8::bit_and(a3039, a3040)
                    };
                    let a3042 = 1u8;
                    if equal(a3041, a3042) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3043, a3044)?
            };
            nat_add(a3045, a3046)?
        };
        nat_add(a3047, a3048)?
    };
    let a3049 = 48u64;
    let a3050 = v2;
    if if nat_le(a3049, a3050) {
        let a3051 = v2;
        let a3052 = 57u64;
        nat_le(a3051, a3052)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3053 = 97u64;
        let a3054 = v2;
        if nat_le(a3053, a3054) {
            let a3055 = v2;
            let a3056 = 102u64;
            Ok(nat_le(a3055, a3056))
        } else {
            Ok(false)
        }
    }
}

pub fn f609() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f610(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3101 = {
            let a3077 = {
                let a3065 = {
                    let a3059 = {
                        let a3057 = v1;
                        let a3058 = 128u8;
                        fixed_u8::bit_and(a3057, a3058)
                    };
                    let a3060 = 128u8;
                    if equal(a3059, a3060) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3066 = {
                    let a3063 = {
                        let a3061 = v1;
                        let a3062 = 64u8;
                        fixed_u8::bit_and(a3061, a3062)
                    };
                    let a3064 = 64u8;
                    if equal(a3063, a3064) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3065, a3066)?
            };
            let a3078 = {
                let a3075 = {
                    let a3069 = {
                        let a3067 = v1;
                        let a3068 = 32u8;
                        fixed_u8::bit_and(a3067, a3068)
                    };
                    let a3070 = 32u8;
                    if equal(a3069, a3070) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3076 = {
                    let a3073 = {
                        let a3071 = v1;
                        let a3072 = 16u8;
                        fixed_u8::bit_and(a3071, a3072)
                    };
                    let a3074 = 16u8;
                    if equal(a3073, a3074) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3075, a3076)?
            };
            nat_add(a3077, a3078)?
        };
        let a3102 = {
            let a3099 = {
                let a3087 = {
                    let a3081 = {
                        let a3079 = v1;
                        let a3080 = 8u8;
                        fixed_u8::bit_and(a3079, a3080)
                    };
                    let a3082 = 8u8;
                    if equal(a3081, a3082) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3088 = {
                    let a3085 = {
                        let a3083 = v1;
                        let a3084 = 4u8;
                        fixed_u8::bit_and(a3083, a3084)
                    };
                    let a3086 = 4u8;
                    if equal(a3085, a3086) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3087, a3088)?
            };
            let a3100 = {
                let a3097 = {
                    let a3091 = {
                        let a3089 = v1;
                        let a3090 = 2u8;
                        fixed_u8::bit_and(a3089, a3090)
                    };
                    let a3092 = 2u8;
                    if equal(a3091, a3092) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3098 = {
                    let a3095 = {
                        let a3093 = v1;
                        let a3094 = 1u8;
                        fixed_u8::bit_and(a3093, a3094)
                    };
                    let a3096 = 1u8;
                    if equal(a3095, a3096) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3097, a3098)?
            };
            nat_add(a3099, a3100)?
        };
        nat_add(a3101, a3102)?
    };
    let a3103 = v2;
    let a3104 = 57u64;
    if nat_le(a3103, a3104) {
        let a3105 = v2;
        let a3106 = 48u64;
        Ok(nat_sub(a3105, a3106))
    } else {
        let a3107 = v2;
        let a3108 = 87u64;
        Ok(nat_sub(a3107, a3108))
    }
}

pub fn f611(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3109, _) = v0;
        h3109
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3110) = v0;
        h3110
    };
    let m3111 = v1;
    match m3111.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3112 = v4;
            match m3112.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f651(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f612(v0: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f547(f46(v0.clone(), 8u64, {
            let a3114 = {
                let a3113 = v0;
                length_bytes(a3113)
            };
            let a3115 = 8u64;
            nat_sub(a3114, a3115)
        }))?))
    } else {
        if f85(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f547(f46(v0.clone(), 14u64, {
                let a3117 = {
                    let a3116 = v0;
                    length_bytes(a3116)
                };
                let a3118 = 14u64;
                nat_sub(a3117, a3118)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f613(p3135: u64, p3136: u64, p3137: u64, p3138: Bytes) -> Bytes {
    let m3134 = (p3135, (p3136, (p3137, p3138)));
    {
        let mut m3134 = m3134;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3134;
                let m3119 = v0;
                if m3119 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3119 - 1;
                    if f117({
                        let a3120 = v1;
                        let a3121 = 0u64;
                        nat_eq(a3120, a3121)
                    }, {
                        let a3122 = v2;
                        let a3123 = 0u64;
                        nat_eq(a3122, a3123)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3124 = v1;
                            let a3125 = 16u64;
                            let a3126 = 0u64;
                            nat_quot(a3124, a3125, a3126)
                        }, ({
                            let a3127 = v2;
                            let a3128 = 1u64;
                            nat_sub(a3127, a3128)
                        }, {
                            let a3132 = f652({
                                let a3129 = v1;
                                let a3130 = 16u64;
                                let a3131 = 0u64;
                                nat_rem(a3129, a3130, a3131)
                            });
                            let a3133 = v3;
                            append_bytes(a3132, a3133)
                        }))))
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

pub fn f614(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f653(f654(f214(v0.clone()))?, v0, v1)
}

pub fn f615(v0: Bytes, v1: Bytes) -> Result<Adt27, Bytes> {
    Err::<Adt27, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f616(v0: Bytes, v1: List<Adt24>) -> Result<Adt27, Bytes> {
    Ok::<Adt27, Bytes>(Adt27::C0(v0, v1))
}

pub fn f617(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt27, Bytes>> {
    let m3139 = v1.clone();
    match m3139.uncons() {
        None => {
            Ok(f615(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3140 = f655(f508(f93(v2, f8(Str::lit("properties")))), v1.clone());
            match m3140.uncons() {
                None => {
                    let m3141 = f656(f654(f657(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3141 {
                        Ok(v5) => {
                            Ok(f616(f620(v0, f621(v5.clone())), f622(v5)))
                        }
                        Err(v4) => {
                            Ok(f615(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3142 = f658(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3142 {
                        Ok(v9) => {
                            Ok(f616(f620(v0, f621(v9.clone())), f622(v9)))
                        }
                        Err(v8) => {
                            Ok(f615(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f618(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3143 = v0;
    match m3143 {
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
        Adt2::C5(_) => {
            Ok(false)
        }
        Adt2::C6(r3144) => {
            let v2 = (*r3144).clone();
            f618(v2, v1)
        }
        Adt2::C7(r3145) => {
            let v3 = (*r3145).clone();
            f618(v3, v1)
        }
        Adt2::C8 => {
            Ok(false)
        }
        Adt2::C9(r3146, r3147) => {
            let v4 = (*r3146).clone();
            let v5 = (*r3147).clone();
            if f618(v4, v1.clone())? {
                Ok(true)
            } else {
                f618(v5, v1)
            }
        }
        Adt2::C10(v6, r3148, r3149) => {
            let v7 = (*r3148).clone();
            let v8 = (*r3149).clone();
            if if f32(v6, f8(Str::lit("$ref"))) {
                f659(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f618(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f618(v8, v1)
                }
            }
        }
    }
}

pub fn f619(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt28, Bytes>> {
    f660(f654(f601(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f620(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f386(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f621(v0: Adt28) -> List<Bytes> {
    let m3150 = v0;
    match m3150 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f622(v0: Adt28) -> List<Adt24> {
    let m3151 = v0;
    match m3151 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f623(v0: Adt2) -> bool {
    let m3152 = f508(f93(v0.clone(), f8(Str::lit("properties"))));
    match m3152.uncons() {
        None => {
            let m3153 = f661(v0);
            match m3153 {
                None => {
                    false
                }
                Some(v3) => {
                    f662(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f624(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f625(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3154 = f663(v0.clone());
    match m3154 {
        None => {
            if f32(f58(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3155 = f93(v0, f8(Str::lit("items")));
                match m3155 {
                    None => {
                        Ok(f664(f8(Str::lit("List")), f666()))
                    }
                    Some(v3) => {
                        Ok(f664(f8(Str::lit("List")), f665(v3, {
                            let a3156 = v1;
                            let a3157 = f8(Str::lit("Item"));
                            append_bytes(a3156, a3157)
                        })?))
                    }
                }
            } else {
                f665(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f664(f8(Str::lit("Option")), f665(v2, v1)?))
        }
    }
}

pub fn f626(p3160: List<Adt2>) -> bool {
    let m3159 = p3160;
    {
        let mut m3159 = m3159;
        loop {
            let step = {
                let v0 = m3159;
                let m3158 = v0;
                match m3158.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f667(f544(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f627(v0: Adt2, v1: Bytes) -> Adt2 {
    let m3161 = v0;
    match m3161 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v6, r3162) => {
            let v7 = (*r3162).clone();
            Adt2::C1(v6, std::rc::Rc::new(v7))
        }
        Adt2::C2(v8) => {
            Adt2::C2(v8)
        }
        Adt2::C3(v12, v13) => {
            Adt2::C3(v12, v13)
        }
        Adt2::C4(v9, v10, v11) => {
            Adt2::C4(v9, v10, v11)
        }
        Adt2::C5(v14) => {
            Adt2::C5(v14)
        }
        Adt2::C6(r3163) => {
            let v15 = (*r3163).clone();
            Adt2::C6(std::rc::Rc::new(v15))
        }
        Adt2::C7(r3164) => {
            let v5 = (*r3164).clone();
            Adt2::C7(std::rc::Rc::new(f627(v5, v1)))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r3165, r3166) => {
            let v16 = (*r3165).clone();
            let v17 = (*r3166).clone();
            Adt2::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt2::C10(v2, r3167, r3168) => {
            let v3 = (*r3167).clone();
            let v4 = (*r3168).clone();
            if f32(v2.clone(), v1.clone()) {
                f627(v4, v1)
            } else {
                Adt2::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f627(v4, v1)))
            }
        }
    }
}

pub fn f628(v0: Adt2, v1: Bytes) -> bool {
    let m3169 = f93(v0, v1);
    match m3169 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f629(v0: List<Adt2>) -> bool {
    let m3170 = v0.clone();
    match m3170.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3171 = f655(f508(f93(v1, f8(Str::lit("properties")))), v0.clone());
            match m3171.uncons() {
                None => {
                    f669(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f668(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f630(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m3172 = v0;
    match m3172 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v7, r3173) => {
            let v8 = (*r3173).clone();
            Adt2::C1(v7, std::rc::Rc::new(v8))
        }
        Adt2::C2(v9) => {
            Adt2::C2(v9)
        }
        Adt2::C3(v13, v14) => {
            Adt2::C3(v13, v14)
        }
        Adt2::C4(v10, v11, v12) => {
            Adt2::C4(v10, v11, v12)
        }
        Adt2::C5(v15) => {
            Adt2::C5(v15)
        }
        Adt2::C6(r3174) => {
            let v16 = (*r3174).clone();
            Adt2::C6(std::rc::Rc::new(v16))
        }
        Adt2::C7(r3175) => {
            let v3 = (*r3175).clone();
            Adt2::C7(std::rc::Rc::new(Adt2::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r3176, r3177) => {
            let v17 = (*r3176).clone();
            let v18 = (*r3177).clone();
            Adt2::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt2::C10(v4, r3178, r3179) => {
            let v5 = (*r3178).clone();
            let v6 = (*r3179).clone();
            Adt2::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f631(p3182: List<Adt2>) -> bool {
    let m3181 = p3182;
    {
        let mut m3181 = m3181;
        loop {
            let step = {
                let v0 = m3181;
                let m3180 = v0;
                match m3180.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f670(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f632(v0: u64) -> u32 {
    let a3183 = v0;
    let a3184 = 32u64;
    if nat_lt(a3183, a3184) {
        let a3185 = v0;
        let a3186 = 16u64;
        if nat_lt(a3185, a3186) {
            let a3187 = v0;
            let a3188 = 8u64;
            if nat_lt(a3187, a3188) {
                let a3189 = v0;
                let a3190 = 4u64;
                if nat_lt(a3189, a3190) {
                    let a3191 = v0;
                    let a3192 = 2u64;
                    if nat_lt(a3191, a3192) {
                        let a3193 = v0;
                        let a3194 = 1u64;
                        if nat_lt(a3193, a3194) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3195 = v0;
                        let a3196 = 3u64;
                        if nat_lt(a3195, a3196) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3197 = v0;
                    let a3198 = 6u64;
                    if nat_lt(a3197, a3198) {
                        let a3199 = v0;
                        let a3200 = 5u64;
                        if nat_lt(a3199, a3200) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3201 = v0;
                        let a3202 = 7u64;
                        if nat_lt(a3201, a3202) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3203 = v0;
                let a3204 = 12u64;
                if nat_lt(a3203, a3204) {
                    let a3205 = v0;
                    let a3206 = 10u64;
                    if nat_lt(a3205, a3206) {
                        let a3207 = v0;
                        let a3208 = 9u64;
                        if nat_lt(a3207, a3208) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3209 = v0;
                        let a3210 = 11u64;
                        if nat_lt(a3209, a3210) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3211 = v0;
                    let a3212 = 14u64;
                    if nat_lt(a3211, a3212) {
                        let a3213 = v0;
                        let a3214 = 13u64;
                        if nat_lt(a3213, a3214) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3215 = v0;
                        let a3216 = 15u64;
                        if nat_lt(a3215, a3216) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3217 = v0;
            let a3218 = 24u64;
            if nat_lt(a3217, a3218) {
                let a3219 = v0;
                let a3220 = 20u64;
                if nat_lt(a3219, a3220) {
                    let a3221 = v0;
                    let a3222 = 18u64;
                    if nat_lt(a3221, a3222) {
                        let a3223 = v0;
                        let a3224 = 17u64;
                        if nat_lt(a3223, a3224) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3225 = v0;
                        let a3226 = 19u64;
                        if nat_lt(a3225, a3226) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3227 = v0;
                    let a3228 = 22u64;
                    if nat_lt(a3227, a3228) {
                        let a3229 = v0;
                        let a3230 = 21u64;
                        if nat_lt(a3229, a3230) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3231 = v0;
                        let a3232 = 23u64;
                        if nat_lt(a3231, a3232) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3233 = v0;
                let a3234 = 28u64;
                if nat_lt(a3233, a3234) {
                    let a3235 = v0;
                    let a3236 = 26u64;
                    if nat_lt(a3235, a3236) {
                        let a3237 = v0;
                        let a3238 = 25u64;
                        if nat_lt(a3237, a3238) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3239 = v0;
                        let a3240 = 27u64;
                        if nat_lt(a3239, a3240) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3241 = v0;
                    let a3242 = 30u64;
                    if nat_lt(a3241, a3242) {
                        let a3243 = v0;
                        let a3244 = 29u64;
                        if nat_lt(a3243, a3244) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3245 = v0;
                        let a3246 = 31u64;
                        if nat_lt(a3245, a3246) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3247 = v0;
        let a3248 = 48u64;
        if nat_lt(a3247, a3248) {
            let a3249 = v0;
            let a3250 = 40u64;
            if nat_lt(a3249, a3250) {
                let a3251 = v0;
                let a3252 = 36u64;
                if nat_lt(a3251, a3252) {
                    let a3253 = v0;
                    let a3254 = 34u64;
                    if nat_lt(a3253, a3254) {
                        let a3255 = v0;
                        let a3256 = 33u64;
                        if nat_lt(a3255, a3256) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3257 = v0;
                        let a3258 = 35u64;
                        if nat_lt(a3257, a3258) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3259 = v0;
                    let a3260 = 38u64;
                    if nat_lt(a3259, a3260) {
                        let a3261 = v0;
                        let a3262 = 37u64;
                        if nat_lt(a3261, a3262) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3263 = v0;
                        let a3264 = 39u64;
                        if nat_lt(a3263, a3264) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3265 = v0;
                let a3266 = 44u64;
                if nat_lt(a3265, a3266) {
                    let a3267 = v0;
                    let a3268 = 42u64;
                    if nat_lt(a3267, a3268) {
                        let a3269 = v0;
                        let a3270 = 41u64;
                        if nat_lt(a3269, a3270) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3271 = v0;
                        let a3272 = 43u64;
                        if nat_lt(a3271, a3272) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3273 = v0;
                    let a3274 = 46u64;
                    if nat_lt(a3273, a3274) {
                        let a3275 = v0;
                        let a3276 = 45u64;
                        if nat_lt(a3275, a3276) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3277 = v0;
                        let a3278 = 47u64;
                        if nat_lt(a3277, a3278) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3279 = v0;
            let a3280 = 56u64;
            if nat_lt(a3279, a3280) {
                let a3281 = v0;
                let a3282 = 52u64;
                if nat_lt(a3281, a3282) {
                    let a3283 = v0;
                    let a3284 = 50u64;
                    if nat_lt(a3283, a3284) {
                        let a3285 = v0;
                        let a3286 = 49u64;
                        if nat_lt(a3285, a3286) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3287 = v0;
                        let a3288 = 51u64;
                        if nat_lt(a3287, a3288) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3289 = v0;
                    let a3290 = 54u64;
                    if nat_lt(a3289, a3290) {
                        let a3291 = v0;
                        let a3292 = 53u64;
                        if nat_lt(a3291, a3292) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3293 = v0;
                        let a3294 = 55u64;
                        if nat_lt(a3293, a3294) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3295 = v0;
                let a3296 = 60u64;
                if nat_lt(a3295, a3296) {
                    let a3297 = v0;
                    let a3298 = 58u64;
                    if nat_lt(a3297, a3298) {
                        let a3299 = v0;
                        let a3300 = 57u64;
                        if nat_lt(a3299, a3300) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3301 = v0;
                        let a3302 = 59u64;
                        if nat_lt(a3301, a3302) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3303 = v0;
                    let a3304 = 62u64;
                    if nat_lt(a3303, a3304) {
                        let a3305 = v0;
                        let a3306 = 61u64;
                        if nat_lt(a3305, a3306) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3307 = v0;
                        let a3308 = 63u64;
                        if nat_lt(a3307, a3308) {
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

pub fn f633(v0: u32) -> u32 {
    let a3311 = {
        let a3309 = f671(v0, 6u32, 26u32);
        let a3310 = f671(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3309, a3310)
    };
    let a3312 = f671(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3311, a3312)
}

pub fn f634(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3318 = {
        let a3313 = v0;
        let a3314 = v1;
        fixed_u32::bit_and(a3313, a3314)
    };
    let a3319 = {
        let a3316 = {
            let a3315 = v0;
            fixed_u32::bit_not(a3315)
        };
        let a3317 = v2;
        fixed_u32::bit_and(a3316, a3317)
    };
    fixed_u32::bit_xor(a3318, a3319)
}

pub fn f635(v0: u32) -> u32 {
    let a3322 = {
        let a3320 = f671(v0, 2u32, 30u32);
        let a3321 = f671(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3320, a3321)
    };
    let a3323 = f671(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3322, a3323)
}

pub fn f636(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3332 = {
        let a3328 = {
            let a3324 = v0;
            let a3325 = v1;
            fixed_u32::bit_and(a3324, a3325)
        };
        let a3329 = {
            let a3326 = v0;
            let a3327 = v2;
            fixed_u32::bit_and(a3326, a3327)
        };
        fixed_u32::bit_xor(a3328, a3329)
    };
    let a3333 = {
        let a3330 = v1;
        let a3331 = v2;
        fixed_u32::bit_and(a3330, a3331)
    };
    fixed_u32::bit_xor(a3332, a3333)
}

pub fn f637(v0: Adt25) -> Adt25 {
    Adt25::C0(match v0.clone() {
        Adt25::C0(_, h3334, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3334
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h3335, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3335
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, h3336, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3336
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, h3337, _, _, _, _, _, _, _, _, _, _, _) => {
            h3337
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h3338, _, _, _, _, _, _, _, _, _, _) => {
            h3338
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h3339, _, _, _, _, _, _, _, _, _) => {
            h3339
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h3340, _, _, _, _, _, _, _, _) => {
            h3340
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h3341, _, _, _, _, _, _, _) => {
            h3341
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h3342, _, _, _, _, _, _) => {
            h3342
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, h3343, _, _, _, _, _) => {
            h3343
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, h3344, _, _, _, _) => {
            h3344
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3345, _, _, _) => {
            h3345
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3346, _, _) => {
            h3346
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3347, _) => {
            h3347
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3348) => {
            h3348
        }
    }, f555(f672(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3349, _) => {
            h3349
        }
    }), f555(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h3350, _, _, _, _, _, _) => {
            h3350
        }
    }, f555(f673(match v0.clone() {
        Adt25::C0(_, h3351, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3351
        }
    }), match v0 {
        Adt25::C0(h3352, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3352
        }
    }))))
}

pub fn f638(v0: Fn21, p3356: u64, p3357: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3355 = (p3356, p3357);
    {
        let mut m3355 = m3355;
        loop {
            let step = {
                let (v1, v2) = m3355;
                let m3353 = v1;
                if m3353 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3353 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3354: Fn21 = v0.clone();
                        c3354.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f639(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3358, _) = v0;
        h3358
    };
    let v2: u32 = {
        let (h3359, _) = {
            let (_, h3360) = v0;
            h3360
        };
        h3359
    };
    let v3: u32 = {
        let (_, h3361) = {
            let (_, h3362) = v0;
            h3362
        };
        h3361
    };
    ({
        let a3363 = v1;
        let a3364 = 2u64;
        let a3365 = 0u64;
        nat_quot(a3363, a3364, a3365)
    }, (f640(v2, 1u32), {
        let a3369 = {
            let a3366 = v1;
            let a3367 = 2u64;
            let a3368 = 0u64;
            nat_rem(a3366, a3367, a3368)
        };
        let a3370 = 1u64;
        if nat_eq(a3369, a3370) {
            let a3371 = v3;
            let a3372 = v2;
            fixed_u32::bit_or(a3371, a3372)
        } else {
            v3
        }
    }))
}

pub fn f640(v0: u32, v1: u32) -> u32 {
    let m3375 = {
        let a3373 = v0;
        let a3374 = v1;
        fixed_u32::shift_left(a3373, a3374)
    };
    match m3375 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f641(v0: Bytes, v1: Adt2) -> bool {
    let v2: Bytes = f592(v1);
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

pub fn f642(v0: u64, v1: Bytes, v2: u64) -> R<Option<List<u64>>> {
    let m3376 = v0;
    if m3376 == 0 {
        Ok(Some(List::<u64>::nil()))
    } else {
        let v3 = m3376 - 1;
        let a3378 = {
            let a3377 = v1.clone();
            length_bytes(a3377)
        };
        let a3379 = v2;
        if nat_le(a3378, a3379) {
            Ok(Some(List::<u64>::nil()))
        } else {
            let m3380 = f674(v1.clone(), v2)?;
            match m3380 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v4) => {
                    let m3384 = f642(v3, v1, {
                        let a3382 = v2;
                        let a3383 = {
                            let (_, h3381) = v4;
                            h3381
                        };
                        nat_add(a3382, a3383)?
                    })?;
                    match m3384 {
                        None => {
                            Ok(None::<List<u64>>)
                        }
                        Some(v5) => {
                            Ok(Some(List::cons({
                                let (h3385, _) = v4;
                                h3385
                            }, v5)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f643(v0: Bytes) -> R<Result<((bool, bool), List<Adt29>), Bytes>> {
    let v1: u64 = {
        let a3386 = v0.clone();
        length_bytes(a3386)
    };
    let v2: bool = {
        let a3387 = 0u64;
        let a3388 = v1;
        if nat_lt(a3387, a3388) {
            let a3389 = f115(v0.clone(), 0u64)?;
            let a3390 = 94u64;
            nat_eq(a3389, a3390)
        } else {
            false
        }
    };
    let v3: bool = {
        let a3391 = 0u64;
        let a3392 = v1;
        if nat_lt(a3391, a3392) {
            f117({
                let a3395 = f115(v0.clone(), {
                    let a3393 = v1;
                    let a3394 = 1u64;
                    nat_sub(a3393, a3394)
                })?;
                let a3396 = 36u64;
                nat_eq(a3395, a3396)
            }, f675({
                let a3399 = f115(v0.clone(), {
                    let a3397 = v1;
                    let a3398 = 2u64;
                    nat_sub(a3397, a3398)
                })?;
                let a3400 = 92u64;
                nat_eq(a3399, a3400)
            }))
        } else {
            false
        }
    };
    let m3405 = f676({
        let a3401 = v1;
        let a3402 = 1u64;
        nat_add(a3401, a3402)?
    }, v0, if v2 {
        1u64
    } else {
        0u64
    }, if v3 {
        let a3403 = v1;
        let a3404 = 1u64;
        nat_sub(a3403, a3404)
    } else {
        v1
    })?;
    match m3405 {
        Ok(v5) => {
            Ok(Ok::<((bool, bool), List<Adt29>), Bytes>(((v2, v3), v5)))
        }
        Err(v4) => {
            Ok(Err::<((bool, bool), List<Adt29>), Bytes>(v4))
        }
    }
}

pub fn f644(v0: u64, v1: List<Adt29>, v2: List<u64>, v3: bool) -> R<bool> {
    let m3406 = v0;
    if m3406 == 0 {
        Ok(false)
    } else {
        let v4 = m3406 - 1;
        let m3407 = v1.clone();
        match m3407.uncons() {
            None => {
                if v3 {
                    let a3408 = f597(v2)?;
                    let a3409 = 0u64;
                    Ok(nat_eq(a3408, a3409))
                } else {
                    Ok(true)
                }
            }
            Some((v5, v6)) => {
                let m3410 = v5.clone();
                match m3410 {
                    Adt29::C0(v7, v8, v9) => {
                        let a3411 = v9;
                        let a3412 = 1u64;
                        if nat_eq(a3411, a3412) {
                            if f644(v4, v6, v2.clone(), v3)? {
                                Ok(true)
                            } else {
                                let m3413 = v2;
                                match m3413.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v10, v11)) => {
                                        if f677(v5, v10) {
                                            f644(v4, v1, v11, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a3414 = v9;
                            let a3415 = 2u64;
                            if nat_eq(a3414, a3415) {
                                let m3416 = v2;
                                match m3416.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v12, v13)) => {
                                        if f677(v5, v12) {
                                            f644(v4, List::cons(Adt29::C0(v7, v8, 1u64), v6), v13, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            } else {
                                let a3417 = v9;
                                let a3418 = 3u64;
                                if nat_eq(a3417, a3418) {
                                    if f644(v4, v6.clone(), v2.clone(), v3)? {
                                        Ok(true)
                                    } else {
                                        let m3419 = v2;
                                        match m3419.uncons() {
                                            None => {
                                                Ok(false)
                                            }
                                            Some((v14, v15)) => {
                                                if f677(v5, v14) {
                                                    f644(v4, v6, v15, v3)
                                                } else {
                                                    Ok(false)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let m3420 = v2;
                                    match m3420.uncons() {
                                        None => {
                                            Ok(false)
                                        }
                                        Some((v16, v17)) => {
                                            if f677(v5, v16) {
                                                f644(v4, v6, v17, v3)
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

pub fn f645(v0: List<Adt29>) -> R<u64> {
    let m3421 = v0;
    match m3421.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a3422 = f645(v2)?;
            let a3423 = 1u64;
            nat_add(a3422, a3423)
        }
    }
}

pub fn f646(v0: List<Adt29>, v1: List<u64>, v2: bool) -> R<bool> {
    if f644({
        let a3426 = {
            let a3424 = f645(v0.clone())?;
            let a3425 = f597(v1.clone())?;
            nat_add(a3424, a3425)?
        };
        let a3427 = 2u64;
        nat_add(a3426, a3427)?
    }, v0.clone(), v1.clone(), v2)? {
        Ok(true)
    } else {
        f678(v0, v1, v2)
    }
}

pub fn f647(v0: u64) -> R<u64> {
    let m3428 = v0;
    if m3428 == 0 {
        Ok(1u64)
    } else {
        let v1 = m3428 - 1;
        let a3429 = f647(v1)?;
        let a3430 = 10u64;
        nat_mul(a3429, a3430)
    }
}

pub fn f648(p3444: u64, v1: Bytes, p3445: u64, p3446: Bytes) -> R<Bytes> {
    let m3443 = (p3444, (p3445, p3446));
    {
        let mut m3443 = m3443;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3443;
                let m3431 = v0;
                if m3431 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3431 - 1;
                    let m3434 = {
                        let a3432 = v1.clone();
                        let a3433 = v2;
                        index_bytes(a3432, a3433)
                    };
                    match m3434 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3435 = v2;
                                let a3436 = 1u64;
                                nat_add(a3435, a3436)?
                            }, {
                                let a3441 = v3;
                                let a3442 = {
                                    let a3437 = v6;
                                    let a3438 = 126u64;
                                    if nat_eq(a3437, a3438) {
                                        f8(Str::lit("~0"))
                                    } else {
                                        let a3439 = v6;
                                        let a3440 = 47u64;
                                        if nat_eq(a3439, a3440) {
                                            f8(Str::lit("~1"))
                                        } else {
                                            f46(v1.clone(), v2, 1u64)
                                        }
                                    }
                                };
                                append_bytes(a3441, a3442)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f649(v0: Fn22, p3451: u64, p3452: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3450 = (p3451, p3452);
    {
        let mut m3450 = m3450;
        loop {
            let step = {
                let (v1, v2) = m3450;
                let m3447 = v1;
                if m3447 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3447 - 1;
                    let m3449 = {
                        let c3448: Fn22 = v0.clone();
                        c3448.apply(v2.clone())?
                    };
                    match m3449 {
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
                Ok(next) => { m3450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f650(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f679(v0)
}

pub fn f651(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3457 = {
            let a3455 = {
                let a3453 = v0.clone();
                length_list(a3453)
            };
            let a3456 = {
                let a3454 = v1.clone();
                length_list(a3454)
            };
            nat_add(a3455, a3456)?
        };
        let a3458 = 1u64;
        nat_add(a3457, a3458)?
    };
    Ok(f439({
        let (_, h3459) = {
            let (_, h3460) = {
                let (h3461, _) = f680(Fn23::F681, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3461
            };
            h3460
        };
        h3459
    }))
}

pub fn f652(v0: u64) -> Bytes {
    f46({
        let a3462 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3462)
    }, v0, 1u64)
}

pub fn f653(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3463 = v1;
    match m3463.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3464 = f682(v0, v3.clone())?;
            match m3464 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f521(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f52(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3465 = f653(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3465 {
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

pub fn f654(p3469: List<Bytes>) -> R<bool> {
    let m3468 = p3469;
    {
        let mut m3468 = m3468;
        loop {
            let step = {
                let v0 = m3468;
                let m3466 = v0;
                match m3466.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3467 = f683(v1)?;
                        match m3467 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f52(v3, f684(false, v2.clone())?) {
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
                Ok(next) => { m3468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f655(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m3470 = v0;
    match m3470.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f685(v1.clone(), {
                let (h3471, _) = v2.clone();
                h3471
            }) {
                List::cons({
                    let (h3472, _) = v2;
                    h3472
                }, f655(v3, v1))
            } else {
                f655(v3, v1)
            }
        }
    }
}

pub fn f656(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    let m3473 = v2;
    match m3473.uncons() {
        None => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v4, v5)) => {
            let m3474 = f686(v4.clone())?;
            match m3474 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3475 = f687(v0, v6.clone())?;
                    match m3475 {
                        None => {
                            Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f52(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3478 = f688(v4, {
                                    let a3476 = v1.clone();
                                    let a3477 = f373(v6)?;
                                    append_bytes(a3476, a3477)
                                })?;
                                match m3478 {
                                    Ok(v9) => {
                                        let m3479 = f656(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3479 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f552(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f621(v11.clone())), f512(f551(v9), f622(v11)))))
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

pub fn f657(v0: List<Adt2>) -> R<List<Bytes>> {
    let m3480 = v0;
    match m3480.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3481 = f686(v1)?;
            match m3481 {
                None => {
                    f657(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f657(v2)?))
                }
            }
        }
    }
}

pub fn f658(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    f689(f654(f690(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f659(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3482 = f92(Some(v0));
    match m3482 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3483 = f612(v2)?;
            match m3483 {
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

pub fn f660(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt28, Bytes>> {
    let m3484 = v2;
    match m3484.uncons() {
        None => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v5, v6)) => {
            let m3486 = f691(v0, {
                let (h3485, _) = v5.clone();
                h3485
            })?;
            match m3486 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3487, _) = v5;
                        h3487
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f52(v7.clone(), f692(v0, f601(v6.clone()))?) {
                        Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3492 = f625({
                            let (_, h3488) = v5.clone();
                            h3488
                        }, {
                            let a3490 = v1.clone();
                            let a3491 = f373({
                                let (h3489, _) = v5.clone();
                                h3489
                            })?;
                            append_bytes(a3490, a3491)
                        })?;
                        match m3492 {
                            Ok(v9) => {
                                let v10: Bytes = if f52({
                                    let (h3493, _) = v5;
                                    h3493
                                }, v3.clone()) {
                                    f552(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f552(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3494 = f660(v0, v1, v6, v3, v4)?;
                                match m3494 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f621(v12.clone())), f512(f551(v9), f622(v12)))))
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

pub fn f661(v0: Adt2) -> Option<Adt2> {
    let m3495 = f93(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3495 {
        None => {
            f693(v0)
        }
        Some(v1) => {
            if f662(v1.clone()) {
                let m3496 = f693(v0);
                match m3496 {
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

pub fn f662(v0: Adt2) -> bool {
    let m3497 = v0;
    match m3497 {
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
        Adt2::C5(_) => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8 => {
            false
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _, _) => {
            false
        }
    }
}

pub fn f663(v0: Adt2) -> Option<Adt2> {
    let m3498 = f187(f93(v0, f8(Str::lit("oneOf"))));
    match m3498.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3499 = v2;
            match m3499.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m3500 = v4;
                    match m3500.uncons() {
                        None => {
                            if f694(v3.clone()) {
                                Some(v1)
                            } else {
                                if f694(v1) {
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

pub fn f664(v0: Bytes, v1: Result<Adt27, Bytes>) -> Result<Adt27, Bytes> {
    let m3501 = v1;
    match m3501 {
        Ok(v3) => {
            f616(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f552(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f551(v3))
        }
        Err(v2) => {
            Err::<Adt27, Bytes>(v2)
        }
    }
}

pub fn f665(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3502 = f663(v0.clone());
    match m3502 {
        None => {
            if f32(f58(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3503 = f93(v0, f8(Str::lit("items")));
                match m3503 {
                    None => {
                        Ok(f664(f8(Str::lit("List")), f666()))
                    }
                    Some(v3) => {
                        Ok(f664(f8(Str::lit("List")), f688(v3, {
                            let a3504 = v1;
                            let a3505 = f8(Str::lit("Item"));
                            append_bytes(a3504, a3505)
                        })?))
                    }
                }
            } else {
                f688(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f664(f8(Str::lit("Option")), f688(v2, v1)?))
        }
    }
}

pub fn f666() -> Result<Adt27, Bytes> {
    Ok::<Adt27, Bytes>(Adt27::C0(f548(), List::cons(Adt24::C2, List::<Adt24>::nil())))
}

pub fn f667(p3509: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m3508 = p3509;
    {
        let mut m3508 = m3508;
        loop {
            let step = {
                let v0 = m3508;
                let m3506 = v0;
                match m3506.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f52({
                            let (h3507, _) = v2;
                            h3507
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f668(p3513: List<Adt2>, v1: Bytes, p3514: List<Bytes>) -> bool {
    let m3512 = (p3513, p3514);
    {
        let mut m3512 = m3512;
        loop {
            let step = {
                let (v0, v2) = m3512;
                let m3510 = v0;
                match m3510.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3511 = f695(v3, v1.clone());
                        match m3511 {
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
                Ok(next) => { m3512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f669(p3517: List<Adt2>, p3518: List<Bytes>) -> bool {
    let m3516 = (p3517, p3518);
    {
        let mut m3516 = m3516;
        loop {
            let step = {
                let (v0, v1) = m3516;
                let m3515 = v0;
                match m3515.uncons() {
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
                Ok(next) => { m3516 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f670(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m3519 = f93(v0.clone(), f8(Str::lit("anyOf")));
        match m3519 {
            None => {
                Adt2::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f667(f544(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f626(f295(v2))
    } else {
        false
    }
}

pub fn f671(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3520 = f515(v0, v1);
    let a3521 = f640(v0, v2);
    fixed_u32::bit_or(a3520, a3521)
}

pub fn f672(v0: u32) -> u32 {
    let a3524 = {
        let a3522 = f671(v0, 17u32, 15u32);
        let a3523 = f671(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3522, a3523)
    };
    let a3525 = f515(v0, 10u32);
    fixed_u32::bit_xor(a3524, a3525)
}

pub fn f673(v0: u32) -> u32 {
    let a3528 = {
        let a3526 = f671(v0, 7u32, 25u32);
        let a3527 = f671(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3526, a3527)
    };
    let a3529 = f515(v0, 3u32);
    fixed_u32::bit_xor(a3528, a3529)
}

pub fn f674(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3532 = {
        let a3530 = v0.clone();
        let a3531 = v1;
        index_bytes(a3530, a3531)
    };
    match m3532 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            let a3533 = v3;
            let a3534 = 128u64;
            if nat_lt(a3533, a3534) {
                Ok(Some((v3, 1u64)))
            } else {
                if f117({
                    let a3535 = 194u64;
                    let a3536 = v3;
                    nat_le(a3535, a3536)
                }, {
                    let a3537 = v3;
                    let a3538 = 223u64;
                    nat_le(a3537, a3538)
                }) {
                    let m3541 = f696(v0, {
                        let a3539 = v1;
                        let a3540 = 1u64;
                        nat_add(a3539, a3540)?
                    })?;
                    match m3541 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f697({
                                let a3546 = {
                                    let a3544 = {
                                        let a3542 = v3;
                                        let a3543 = 192u64;
                                        nat_sub(a3542, a3543)
                                    };
                                    let a3545 = 64u64;
                                    nat_mul(a3544, a3545)?
                                };
                                let a3547 = v4;
                                nat_add(a3546, a3547)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f117({
                        let a3548 = 224u64;
                        let a3549 = v3;
                        nat_le(a3548, a3549)
                    }, {
                        let a3550 = v3;
                        let a3551 = 239u64;
                        nat_le(a3550, a3551)
                    }) {
                        let m3554 = f696(v0.clone(), {
                            let a3552 = v1;
                            let a3553 = 1u64;
                            nat_add(a3552, a3553)?
                        })?;
                        match m3554 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3557 = f696(v0, {
                                    let a3555 = v1;
                                    let a3556 = 2u64;
                                    nat_add(a3555, a3556)?
                                })?;
                                match m3557 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f697({
                                            let a3566 = {
                                                let a3560 = {
                                                    let a3558 = v3;
                                                    let a3559 = 224u64;
                                                    nat_sub(a3558, a3559)
                                                };
                                                let a3561 = 4096u64;
                                                nat_mul(a3560, a3561)?
                                            };
                                            let a3567 = {
                                                let a3564 = {
                                                    let a3562 = v5;
                                                    let a3563 = 64u64;
                                                    nat_mul(a3562, a3563)?
                                                };
                                                let a3565 = v6;
                                                nat_add(a3564, a3565)?
                                            };
                                            nat_add(a3566, a3567)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f117({
                            let a3568 = 240u64;
                            let a3569 = v3;
                            nat_le(a3568, a3569)
                        }, {
                            let a3570 = v3;
                            let a3571 = 244u64;
                            nat_le(a3570, a3571)
                        }) {
                            let m3574 = f696(v0.clone(), {
                                let a3572 = v1;
                                let a3573 = 1u64;
                                nat_add(a3572, a3573)?
                            })?;
                            match m3574 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3577 = f696(v0.clone(), {
                                        let a3575 = v1;
                                        let a3576 = 2u64;
                                        nat_add(a3575, a3576)?
                                    })?;
                                    match m3577 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3580 = f696(v0, {
                                                let a3578 = v1;
                                                let a3579 = 3u64;
                                                nat_add(a3578, a3579)?
                                            })?;
                                            match m3580 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f697({
                                                        let a3593 = {
                                                            let a3583 = {
                                                                let a3581 = v3;
                                                                let a3582 = 240u64;
                                                                nat_sub(a3581, a3582)
                                                            };
                                                            let a3584 = 262144u64;
                                                            nat_mul(a3583, a3584)?
                                                        };
                                                        let a3594 = {
                                                            let a3591 = {
                                                                let a3585 = v7;
                                                                let a3586 = 4096u64;
                                                                nat_mul(a3585, a3586)?
                                                            };
                                                            let a3592 = {
                                                                let a3589 = {
                                                                    let a3587 = v8;
                                                                    let a3588 = 64u64;
                                                                    nat_mul(a3587, a3588)?
                                                                };
                                                                let a3590 = v9;
                                                                nat_add(a3589, a3590)?
                                                            };
                                                            nat_add(a3591, a3592)?
                                                        };
                                                        nat_add(a3593, a3594)?
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

pub fn f675(v0: bool) -> bool {
    !v0
}

pub fn f676(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Result<List<Adt29>, Bytes>> {
    let m3595 = v0;
    if m3595 == 0 {
        Ok(Ok::<List<Adt29>, Bytes>(List::<Adt29>::nil()))
    } else {
        let v4 = m3595 - 1;
        let a3596 = v3;
        let a3597 = v2;
        if nat_le(a3596, a3597) {
            Ok(Ok::<List<Adt29>, Bytes>(List::<Adt29>::nil()))
        } else {
            let v5: u64 = f115(v1.clone(), v2)?;
            let m3646 = {
                let a3598 = v5;
                let a3599 = 91u64;
                if nat_eq(a3598, a3599) {
                    let a3602 = f115(v1.clone(), {
                        let a3600 = v2;
                        let a3601 = 1u64;
                        nat_add(a3600, a3601)?
                    })?;
                    let a3603 = 94u64;
                    if nat_eq(a3602, a3603) {
                        let m3607 = f698({
                            let a3604 = v1.clone();
                            length_bytes(a3604)
                        }, v1.clone(), {
                            let a3605 = v2;
                            let a3606 = 2u64;
                            nat_add(a3605, a3606)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3607 {
                            Ok(v7) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, {
                                    let (h3608, _) = v7;
                                    h3608
                                }), {
                                    let (_, h3609) = v7;
                                    h3609
                                }))
                            }
                            Err(v6) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v6)
                            }
                        }
                    } else {
                        let m3613 = f698({
                            let a3610 = v1.clone();
                            length_bytes(a3610)
                        }, v1.clone(), {
                            let a3611 = v2;
                            let a3612 = 1u64;
                            nat_add(a3611, a3612)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3613 {
                            Ok(v9) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, {
                                    let (h3614, _) = v9;
                                    h3614
                                }), {
                                    let (_, h3615) = v9;
                                    h3615
                                }))
                            }
                            Err(v8) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v8)
                            }
                        }
                    }
                } else {
                    let a3616 = v5;
                    let a3617 = 46u64;
                    if nat_eq(a3616, a3617) {
                        Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, List::cons((10u64, 10u64), List::cons((13u64, 13u64), List::cons((8232u64, 8233u64), List::<(u64, u64)>::nil())))), {
                            let a3618 = v2;
                            let a3619 = 1u64;
                            nat_add(a3618, a3619)?
                        }))
                    } else {
                        let a3620 = v5;
                        let a3621 = 40u64;
                        if if if if if if nat_eq(a3620, a3621) {
                            true
                        } else {
                            let a3622 = v5;
                            let a3623 = 41u64;
                            nat_eq(a3622, a3623)
                        } {
                            true
                        } else {
                            let a3624 = v5;
                            let a3625 = 124u64;
                            if nat_eq(a3624, a3625) {
                                true
                            } else {
                                let a3626 = v5;
                                let a3627 = 123u64;
                                nat_eq(a3626, a3627)
                            }
                        } {
                            true
                        } else {
                            let a3628 = v5;
                            let a3629 = 125u64;
                            if if nat_eq(a3628, a3629) {
                                true
                            } else {
                                let a3630 = v5;
                                let a3631 = 92u64;
                                nat_eq(a3630, a3631)
                            } {
                                true
                            } else {
                                let a3632 = v5;
                                let a3633 = 94u64;
                                if nat_eq(a3632, a3633) {
                                    true
                                } else {
                                    let a3634 = v5;
                                    let a3635 = 36u64;
                                    nat_eq(a3634, a3635)
                                }
                            }
                        } {
                            true
                        } else {
                            let a3636 = v5;
                            let a3637 = 42u64;
                            if if nat_eq(a3636, a3637) {
                                true
                            } else {
                                let a3638 = v5;
                                let a3639 = 43u64;
                                nat_eq(a3638, a3639)
                            } {
                                true
                            } else {
                                let a3640 = v5;
                                let a3641 = 63u64;
                                nat_eq(a3640, a3641)
                            }
                        } {
                            true
                        } else {
                            let a3642 = 127u64;
                            let a3643 = v5;
                            nat_lt(a3642, a3643)
                        } {
                            Err::<((bool, List<(u64, u64)>), u64), Bytes>(f7(List::cons(f8(Str::lit("the pattern construct ")), List::cons(f46(v1.clone(), v2, 1u64), List::cons(f8(Str::lit(" is not evaluated: only ^, $, ., character classes, literal ASCII characters and the quantifiers *, + and ? are (ECMA-262 section 22.2.1)")), List::<Bytes>::nil())))))
                        } else {
                            Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, List::cons((v5, v5), List::<(u64, u64)>::nil())), {
                                let a3644 = v2;
                                let a3645 = 1u64;
                                nat_add(a3644, a3645)?
                            }))
                        }
                    }
                }
            };
            match m3646 {
                Ok(v11) => {
                    let v12: u64 = {
                        let (_, h3647) = v11.clone();
                        h3647
                    };
                    let v13: u64 = {
                        let a3648 = v12;
                        let a3649 = v3;
                        if nat_lt(a3648, a3649) {
                            f699(f115(v1.clone(), v12)?)
                        } else {
                            0u64
                        }
                    };
                    let a3650 = v12;
                    let a3651 = v3;
                    if if nat_lt(a3650, a3651) {
                        let a3652 = f115(v1.clone(), v12)?;
                        let a3653 = 123u64;
                        nat_eq(a3652, a3653)
                    } else {
                        false
                    } {
                        Ok(Err::<List<Adt29>, Bytes>(f8(Str::lit("a {n,m} quantifier in a pattern is not evaluated (ECMA-262 section 22.2.1)"))))
                    } else {
                        let m3658 = f676(v4, v1, {
                            let a3654 = 0u64;
                            let a3655 = v13;
                            if nat_lt(a3654, a3655) {
                                let a3656 = v12;
                                let a3657 = 1u64;
                                nat_add(a3656, a3657)?
                            } else {
                                v12
                            }
                        }, v3)?;
                        match m3658 {
                            Ok(v15) => {
                                Ok(Ok::<List<Adt29>, Bytes>(List::cons(Adt29::C0({
                                    let (h3659, _) = {
                                        let (h3660, _) = v11.clone();
                                        h3660
                                    };
                                    h3659
                                }, {
                                    let (_, h3661) = {
                                        let (h3662, _) = v11;
                                        h3662
                                    };
                                    h3661
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

pub fn f677(v0: Adt29, v1: u64) -> bool {
    let m3663 = v0;
    match m3663 {
        Adt29::C0(v2, v3, _) => {
            if v2 {
                f675(f700(v3, v1))
            } else {
                f700(v3, v1)
            }
        }
    }
}

pub fn f678(v0: List<Adt29>, p3670: List<u64>, v2: bool) -> R<bool> {
    let m3669 = p3670;
    {
        let mut m3669 = m3669;
        loop {
            let step = {
                let v1 = m3669;
                let m3664 = v1;
                match m3664.uncons() {
                    None => {
                        Err::<List<u64>, R<bool>>(Ok(false))
                    }
                    Some((_, v4)) => {
                        if f644({
                            let a3667 = {
                                let a3665 = f645(v0.clone())?;
                                let a3666 = f597(v4.clone())?;
                                nat_add(a3665, a3666)?
                            };
                            let a3668 = 2u64;
                            nat_add(a3667, a3668)?
                        }, v0.clone(), v4.clone(), v2)? {
                            Err::<List<u64>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<u64>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f679(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3671, _) = v0;
        h3671
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3672) = v0;
        h3672
    };
    let m3673 = v1;
    match m3673.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3674 = v4;
            match m3674.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f701(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f680(v0: Fn23, p3679: u64, p3680: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3678 = (p3679, p3680);
    {
        let mut m3678 = m3678;
        loop {
            let step = {
                let (v1, v2) = m3678;
                let m3675 = v1;
                if m3675 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3675 - 1;
                    let m3677 = {
                        let c3676: Fn23 = v0.clone();
                        c3676.apply(v2.clone())
                    };
                    match m3677 {
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
                Ok(next) => { m3678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f681(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f702(v0)
}

pub fn f682(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3681 = v1;
    match m3681 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C3(v7, v8) => {
            Ok(Some({
                let a3692 = if f117(v7, {
                    let a3690 = 0u64;
                    let a3691 = v8;
                    nat_lt(a3690, a3691)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3693 = f297(false, v8)?;
                append_bytes(a3692, a3693)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3684 = f703({
                let a3682 = f8(Str::lit("n"));
                let a3683 = f298(false, v4, v5)?;
                append_bytes(a3682, a3683)
            })?;
            match m3684 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3688 = f8(Str::lit("nMinus"));
                        let a3689 = f46(v6.clone(), 1u64, {
                            let a3686 = {
                                let a3685 = v6;
                                length_bytes(a3685)
                            };
                            let a3687 = 1u64;
                            nat_sub(a3686, a3687)
                        });
                        append_bytes(a3688, a3689)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f687(v0, v2)
        }
        Adt2::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C8 => {
            Ok(None::<Bytes>)
        }
        Adt2::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f683(v0: Bytes) -> R<Option<Bytes>> {
    let m3694 = f704(v0)?;
    match m3694 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f705(v1)?))
        }
    }
}

pub fn f684(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3695 = v1;
    match m3695.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3696 = f687(v0, v2)?;
            match m3696 {
                None => {
                    f684(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f684(v0, v3)?))
                }
            }
        }
    }
}

pub fn f685(p3700: List<Adt2>, v1: Bytes) -> bool {
    let m3699 = p3700;
    {
        let mut m3699 = m3699;
        loop {
            let step = {
                let v0 = m3699;
                let m3697 = v0;
                match m3697.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3698 = f695(v2, v1.clone());
                        if match m3698 {
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
                Ok(next) => { m3699 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: Adt2) -> R<Option<Bytes>> {
    let m3701 = f92(f93(v0.clone(), f8(Str::lit("$ref"))));
    match m3701 {
        None => {
            Ok(f92(f93(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f612(v1)
        }
    }
}

pub fn f687(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f703(v1)
    } else {
        f683(v1)
    }
}

pub fn f688(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    f706(f584(v0), v1)
}

pub fn f689(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    let m3702 = v2;
    match m3702.uncons() {
        None => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v5, v6)) => {
            let m3703 = f695(v5.clone(), v3.clone());
            match m3703 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3704 = f687(v0, v7.clone())?;
                    match m3704 {
                        None => {
                            Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f52(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3707 = f619({
                                    let a3705 = v1.clone();
                                    let a3706 = f373(v7)?;
                                    append_bytes(a3705, a3706)
                                }, f546(f508(f93(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f214(f187(f93(v5, f8(Str::lit("required"))))), true)?;
                                match m3707 {
                                    Ok(v10) => {
                                        let m3708 = f689(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3708 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f386(f621(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f621(v12.clone())), f512(f622(v10), f622(v12)))))
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

pub fn f690(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3709 = v0;
    match m3709.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3710 = f695(v2, v1.clone());
            match m3710 {
                None => {
                    f690(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f690(v3, v1))
                }
            }
        }
    }
}

pub fn f691(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3711 = f687(v0, v1)?;
    match m3711 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f32(v2.clone(), f8(Str::lit("mk"))) {
                f582(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f692(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3712 = v1;
    match m3712.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3713 = f691(v0, v2)?;
            match m3713 {
                None => {
                    f692(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f692(v0, v3)?))
                }
            }
        }
    }
}

pub fn f693(v0: Adt2) -> Option<Adt2> {
    let m3714 = f508(f93(v0, f8(Str::lit("patternProperties"))));
    match m3714.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3715 = v2;
            match m3715.uncons() {
                None => {
                    Some({
                        let (_, h3716) = v1;
                        h3716
                    })
                }
                Some((_, _)) => {
                    None::<Adt2>
                }
            }
        }
    }
}

pub fn f694(v0: Adt2) -> bool {
    f32(f58(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f695(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f52(v1.clone(), f214(f187(f93(v0.clone(), f8(Str::lit("required")))))) {
        let m3717 = f93(v0, f8(Str::lit("properties")));
        match m3717 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3718 = f93(v2, v1);
                match m3718 {
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

pub fn f696(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3721 = {
        let a3719 = v0;
        let a3720 = v1;
        index_bytes(a3719, a3720)
    };
    match m3721 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f117({
                let a3722 = 128u64;
                let a3723 = v3;
                nat_le(a3722, a3723)
            }, {
                let a3724 = v3;
                let a3725 = 191u64;
                nat_le(a3724, a3725)
            }) {
                Ok(Some({
                    let a3726 = v3;
                    let a3727 = 128u64;
                    nat_sub(a3726, a3727)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f697(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f117({
        let a3728 = v1;
        let a3729 = v0;
        nat_le(a3728, a3729)
    }, f117({
        let a3730 = v0;
        let a3731 = v2;
        nat_le(a3730, a3731)
    }, f202({
        let a3732 = v0;
        let a3733 = 55296u64;
        nat_lt(a3732, a3733)
    }, {
        let a3734 = 57343u64;
        let a3735 = v0;
        nat_lt(a3734, a3735)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f698(p3772: u64, v1: Bytes, p3773: u64, p3774: List<(u64, u64)>) -> R<Result<(List<(u64, u64)>, u64), Bytes>> {
    let m3771 = (p3772, (p3773, p3774));
    {
        let mut m3771 = m3771;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3771;
                let m3736 = v0;
                if m3736 == 0 {
                    Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                } else {
                    let v4 = m3736 - 1;
                    let v5: u64 = f115(v1.clone(), v2)?;
                    let a3738 = {
                        let a3737 = v1.clone();
                        length_bytes(a3737)
                    };
                    let a3739 = v2;
                    if nat_le(a3738, a3739) {
                        Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                    } else {
                        let a3740 = v5;
                        let a3741 = 93u64;
                        if nat_eq(a3740, a3741) {
                            Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Ok::<(List<(u64, u64)>, u64), Bytes>((v3, {
                                let a3742 = v2;
                                let a3743 = 1u64;
                                nat_add(a3742, a3743)?
                            }))))
                        } else {
                            let a3744 = v5;
                            let a3745 = 92u64;
                            if if nat_eq(a3744, a3745) {
                                true
                            } else {
                                let a3746 = 127u64;
                                let a3747 = v5;
                                nat_lt(a3746, a3747)
                            } {
                                Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("an escape or a non-ASCII character in a pattern's character class is not evaluated (ECMA-262 section 22.2.1)")))))
                            } else {
                                let a3750 = f115(v1.clone(), {
                                    let a3748 = v2;
                                    let a3749 = 1u64;
                                    nat_add(a3748, a3749)?
                                })?;
                                let a3751 = 45u64;
                                if if nat_eq(a3750, a3751) {
                                    f117({
                                        let a3755 = {
                                            let a3752 = v2;
                                            let a3753 = 2u64;
                                            nat_add(a3752, a3753)?
                                        };
                                        let a3756 = {
                                            let a3754 = v1.clone();
                                            length_bytes(a3754)
                                        };
                                        nat_lt(a3755, a3756)
                                    }, f117(f675({
                                        let a3759 = f115(v1.clone(), {
                                            let a3757 = v2;
                                            let a3758 = 2u64;
                                            nat_add(a3757, a3758)?
                                        })?;
                                        let a3760 = 93u64;
                                        nat_eq(a3759, a3760)
                                    }), f675({
                                        let a3763 = f115(v1.clone(), {
                                            let a3761 = v2;
                                            let a3762 = 2u64;
                                            nat_add(a3761, a3762)?
                                        })?;
                                        let a3764 = 92u64;
                                        nat_eq(a3763, a3764)
                                    })))
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3765 = v2;
                                        let a3766 = 3u64;
                                        nat_add(a3765, a3766)?
                                    }, List::cons((v5, f115(v1.clone(), {
                                        let a3767 = v2;
                                        let a3768 = 2u64;
                                        nat_add(a3767, a3768)?
                                    })?), v3))))
                                } else {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3769 = v2;
                                        let a3770 = 1u64;
                                        nat_add(a3769, a3770)?
                                    }, List::cons((v5, v5), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f699(v0: u64) -> u64 {
    let a3775 = v0;
    let a3776 = 42u64;
    if nat_eq(a3775, a3776) {
        1u64
    } else {
        let a3777 = v0;
        let a3778 = 43u64;
        if nat_eq(a3777, a3778) {
            2u64
        } else {
            let a3779 = v0;
            let a3780 = 63u64;
            if nat_eq(a3779, a3780) {
                3u64
            } else {
                0u64
            }
        }
    }
}

pub fn f700(p3789: List<(u64, u64)>, v1: u64) -> bool {
    let m3788 = p3789;
    {
        let mut m3788 = m3788;
        loop {
            let step = {
                let v0 = m3788;
                let m3781 = v0;
                match m3781.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3783 = {
                            let (h3782, _) = v2;
                            h3782
                        };
                        let a3784 = v1;
                        if if nat_le(a3783, a3784) {
                            let a3786 = v1;
                            let a3787 = {
                                let (_, h3785) = v2;
                                h3785
                            };
                            nat_le(a3786, a3787)
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
                Ok(next) => { m3788 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f701(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3794 = {
            let a3792 = {
                let a3790 = v0.clone();
                length_list(a3790)
            };
            let a3793 = {
                let a3791 = v1.clone();
                length_list(a3791)
            };
            nat_add(a3792, a3793)?
        };
        let a3795 = 1u64;
        nat_add(a3794, a3795)?
    };
    Ok(f707({
        let (_, h3796) = {
            let (_, h3797) = {
                let (h3798, _) = f708(Fn24::F709, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3798
            };
            h3797
        };
        h3796
    }))
}

pub fn f702(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3799, _) = v0.clone();
        h3799
    };
    let v2: List<(Bytes, ())> = {
        let (h3800, _) = {
            let (_, h3801) = v0.clone();
            h3801
        };
        h3800
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3802) = {
            let (_, h3803) = v0;
            h3803
        };
        h3802
    };
    let m3804 = v1.clone();
    match m3804.uncons() {
        None => {
            let m3805 = v2;
            match m3805.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3806 = v2.clone();
            match m3806.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f710({
                        let (h3807, _) = v8.clone();
                        h3807
                    }, {
                        let (h3808, _) = v6.clone();
                        h3808
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

pub fn f703(v0: Bytes) -> R<Option<Bytes>> {
    let m3810 = f711({
        let a3809 = v0.clone();
        length_bytes(a3809)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3810 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3812 = {
                let a3811 = v1.clone();
                length_bytes(a3811)
            };
            let a3813 = 0u64;
            if nat_eq(a3812, a3813) {
                let a3815 = {
                    let a3814 = v0;
                    length_bytes(a3814)
                };
                let a3816 = 0u64;
                if nat_eq(a3815, a3816) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f705(v1)?))
            }
        }
    }
}

pub fn f704(v0: Bytes) -> R<Option<Bytes>> {
    if f712({
        let a3817 = v0.clone();
        length_bytes(a3817)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f713(v0)?;
        if f714(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3820 = {
                let a3818 = v1.clone();
                let a3819 = 0u64;
                index_bytes(a3818, a3819)
            };
            match m3820 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f506(f72(v2)?) {
                        Ok(Some({
                            let a3821 = f8(Str::lit("n"));
                            let a3822 = v1;
                            append_bytes(a3821, a3822)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f715({
            let a3823 = v0.clone();
            length_bytes(a3823)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f705(v0: Bytes) -> R<Bytes> {
    if f52(v0.clone(), f716()) {
        f582(v0)
    } else {
        Ok(v0)
    }
}

pub fn f706(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3824 = v0.clone();
    match m3824 {
        Adt2::C0 => {
            Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C1(_, _) => {
            Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(v4) => {
            if v4 {
                Ok(f666())
            } else {
                Ok(f615(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C3(_, _) => {
            Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_, _, _) => {
            Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_) => {
            Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C6(_) => {
            Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C7(_) => {
            let m3825 = f717(f544(v0.clone()));
            match m3825 {
                None => {
                    let m3826 = f92(f93(v0.clone(), f8(Str::lit("$ref"))));
                    match m3826 {
                        None => {
                            let m3828 = f93(v0.clone(), f8(Str::lit("allOf")));
                            match m3828 {
                                None => {
                                    let m3833 = f93(v0.clone(), f8(Str::lit("enum")));
                                    match m3833 {
                                        None => {
                                            let m3834 = f93(v0.clone(), f8(Str::lit("const")));
                                            match m3834 {
                                                None => {
                                                    let m3836 = f93(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3836 {
                                                        None => {
                                                            let v32: Bytes = f58(v0.clone(), f8(Str::lit("type")));
                                                            if if f32(v32.clone(), Bytes::lit(&[])) {
                                                                !f718(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f666())
                                                            } else {
                                                                if f32(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f616(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                                                } else {
                                                                    if f32(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f616(f719(v0), List::<Adt24>::nil()))
                                                                    } else {
                                                                        if f32(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f616(f720(), List::<Adt24>::nil()))
                                                                        } else {
                                                                            if f32(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f616(f8(Str::lit("Bool")), List::<Adt24>::nil()))
                                                                            } else {
                                                                                if f32(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3837 = f508(f93(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3837.uncons() {
                                                                                        None => {
                                                                                            if f623(v0.clone()) {
                                                                                                Ok(f616(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                                                            } else {
                                                                                                let m3838 = f661(v0);
                                                                                                match m3838 {
                                                                                                    None => {
                                                                                                        Ok(f664(f8(Str::lit("List")), f721(f666())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f664(f8(Str::lit("List")), f721(f722(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f616(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f615(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f616(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3835 = f92(Some(v29));
                                                    match m3835 {
                                                        None => {
                                                            Ok(f615(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f616(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f616(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3829 = f295(v21);
                                    match m3829.uncons() {
                                        None => {
                                            Ok(f615(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3830 = v23;
                                            match m3830.uncons() {
                                                None => {
                                                    let m3831 = f92(f93(v22, f8(Str::lit("$ref"))));
                                                    match m3831 {
                                                        None => {
                                                            Ok(f615(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3832 = f612(v26)?;
                                                            match m3832 {
                                                                None => {
                                                                    Ok(f615(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f616(v27, List::<Adt24>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f615(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3827 = f612(v19)?;
                            match m3827 {
                                None => {
                                    Ok(f615(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f616(v20, List::<Adt24>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f615(if f32(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C8 => {
            Ok(f666())
        }
        Adt2::C9(_, _) => {
            Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C10(_, _, _) => {
            Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f707(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f397(Fn5::F723, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f708(v0: Fn24, p3843: u64, p3844: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3842 = (p3843, p3844);
    {
        let mut m3842 = m3842;
        loop {
            let step = {
                let (v1, v2) = m3842;
                let m3839 = v1;
                if m3839 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3839 - 1;
                    let m3841 = {
                        let c3840: Fn24 = v0.clone();
                        c3840.apply(v2.clone())
                    };
                    match m3841 {
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
                Ok(next) => { m3842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f709(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f724(v0)
}

pub fn f710(v0: Bytes, v1: Bytes) -> bool {
    let a3849 = {
        let a3845 = v0;
        let a3846 = v1;
        compare_bytes(a3845, a3846)
    };
    let a3850 = {
        let a3847 = Bytes::lit(&[]);
        let a3848 = Bytes::lit(&[0x01]);
        compare_bytes(a3847, a3848)
    };
    equal(a3849, a3850)
}

pub fn f711(p3863: u64, v1: Bytes, p3864: u64, p3865: Bytes) -> R<Option<Bytes>> {
    let m3862 = (p3863, (p3864, p3865));
    {
        let mut m3862 = m3862;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3862;
                let m3851 = v0;
                if m3851 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3851 - 1;
                    let a3853 = {
                        let a3852 = v1.clone();
                        length_bytes(a3852)
                    };
                    let a3854 = v2;
                    if nat_le(a3853, a3854) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3855 = f674(v1.clone(), v2)?;
                        match m3855 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3857 = v2;
                                    let a3858 = {
                                        let (_, h3856) = v5;
                                        h3856
                                    };
                                    nat_add(a3857, a3858)?
                                }, {
                                    let a3860 = v3;
                                    let a3861 = f725(v1.clone(), v2, {
                                        let (h3859, _) = v5;
                                        h3859
                                    });
                                    append_bytes(a3860, a3861)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3862 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f712(p3875: u64, v1: Bytes, p3876: u64) -> R<bool> {
    let m3874 = (p3875, p3876);
    {
        let mut m3874 = m3874;
        loop {
            let step = {
                let (v0, v2) = m3874;
                let m3866 = v0;
                if m3866 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3866 - 1;
                    let m3869 = {
                        let a3867 = v1.clone();
                        let a3868 = v2;
                        index_bytes(a3867, a3868)
                    };
                    match m3869 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3870 = f72(v4)?;
                            let a3871 = 128u64;
                            if nat_lt(a3870, a3871) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3872 = v2;
                                    let a3873 = 1u64;
                                    nat_add(a3872, a3873)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3874 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f713(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f456(v0.clone(), 0u64, {
        let a3877 = v0;
        length_bytes(a3877)
    }, false, Bytes::lit(&[]))?;
    let m3880 = {
        let a3878 = v1.clone();
        let a3879 = 0u64;
        index_bytes(a3878, a3879)
    };
    match m3880 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f505(v3) {
                let a3887 = f46({
                    let a3881 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3881)
                }, {
                    let a3882 = v3;
                    let a3883 = 65u64;
                    nat_sub(a3882, a3883)
                }, 1u64);
                let a3888 = f46(v1.clone(), 1u64, {
                    let a3885 = {
                        let a3884 = v1;
                        length_bytes(a3884)
                    };
                    let a3886 = 1u64;
                    nat_sub(a3885, a3886)
                });
                Ok(append_bytes(a3887, a3888))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f714(v0: Bytes) -> R<bool> {
    let m3891 = {
        let a3889 = v0;
        let a3890 = 0u64;
        index_bytes(a3889, a3890)
    };
    match m3891 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f202(f504(f72(v1)?), f505(f72(v1)?)))
        }
    }
}

pub fn f715(p3904: u64, v1: Bytes, p3905: u64, p3906: Bytes) -> R<Option<Bytes>> {
    let m3903 = (p3904, (p3905, p3906));
    {
        let mut m3903 = m3903;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3903;
                let m3892 = v0;
                if m3892 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3892 - 1;
                    let a3894 = {
                        let a3893 = v1.clone();
                        length_bytes(a3893)
                    };
                    let a3895 = v2;
                    if nat_le(a3894, a3895) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3896 = f674(v1.clone(), v2)?;
                        match m3896 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3898 = v2;
                                    let a3899 = {
                                        let (_, h3897) = v5;
                                        h3897
                                    };
                                    nat_add(a3898, a3899)?
                                }, {
                                    let a3901 = v3;
                                    let a3902 = f726({
                                        let (h3900, _) = v5;
                                        h3900
                                    });
                                    append_bytes(a3901, a3902)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3903 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f716() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f717(p3911: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m3910 = p3911;
    {
        let mut m3910 = m3910;
        loop {
            let step = {
                let v0 = m3910;
                let m3907 = v0;
                match m3907.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h3908, _) = v1.clone();
                            h3908
                        }, f727()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h3909, _) = v1;
                                h3909
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3910 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f718(v0: Adt2) -> bool {
    if f628(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f628(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f628(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f628(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f628(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f628(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f719(v0: Adt2) -> Bytes {
    let m3912 = f492(f93(v0, f8(Str::lit("minimum"))));
    match m3912 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f720() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f721(v0: Result<Adt27, Bytes>) -> Result<Adt27, Bytes> {
    let m3913 = v0;
    match m3913 {
        Ok(v2) => {
            f616(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f552(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f551(v2))
        }
        Err(v1) => {
            Err::<Adt27, Bytes>(v1)
        }
    }
}

pub fn f722(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    f728(16u64, v0, v1)
}

pub fn f723(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f724(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3914, _) = v0.clone();
        h3914
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3915, _) = {
            let (_, h3916) = v0.clone();
            h3916
        };
        h3915
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3917) = {
            let (_, h3918) = v0;
            h3918
        };
        h3917
    };
    let m3919 = v1.clone();
    match m3919.uncons() {
        None => {
            let m3920 = v2;
            match m3920.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3921 = v2.clone();
            match m3921.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f710({
                        let (h3922, _) = v8.clone();
                        h3922
                    }, {
                        let (h3923, _) = v6.clone();
                        h3923
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

pub fn f725(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f202(f117(f202(f504(v2), f505(v2)), f117({
        let a3924 = v2;
        let a3925 = 117u64;
        !nat_eq(a3924, a3925)
    }, {
        let a3926 = v2;
        let a3927 = 85u64;
        !nat_eq(a3926, a3927)
    })), f117(f506(v2), {
        let a3928 = 0u64;
        let a3929 = v1;
        nat_lt(a3928, a3929)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a3930 = v2;
        let a3931 = 65536u64;
        if nat_lt(a3930, a3931) {
            let a3932 = f8(Str::lit("u"));
            let a3933 = f613(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3932, a3933)
        } else {
            let a3934 = f8(Str::lit("U"));
            let a3935 = f613(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3934, a3935)
        }
    }
}

pub fn f726(v0: u64) -> Bytes {
    let a3936 = f8(Str::lit("u"));
    let a3937 = f613(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3936, a3937)
}

pub fn f727() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f728(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3938 = v0;
    if m3938 == 0 {
        Ok(f615(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3938 - 1;
        let m3939 = v1.clone();
        match m3939 {
            Adt2::C0 => {
                Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C1(_, _) => {
                Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(f666())
                } else {
                    Ok(f615(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C3(_, _) => {
                Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_, _, _) => {
                Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_) => {
                Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C6(_) => {
                Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C7(_) => {
                let m3940 = f92(f93(v1.clone(), f8(Str::lit("$ref"))));
                match m3940 {
                    None => {
                        if if f628(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f628(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f628(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f616(v2.clone(), List::cons(Adt24::C0(v2, v1), List::<Adt24>::nil())))
                        } else {
                            let v22: Bytes = f58(v1.clone(), f8(Str::lit("type")));
                            if if f32(v22.clone(), Bytes::lit(&[])) {
                                !f718(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f666())
                            } else {
                                if f32(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f616(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                } else {
                                    if f32(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f616(f719(v1), List::<Adt24>::nil()))
                                    } else {
                                        if f32(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f616(f720(), List::<Adt24>::nil()))
                                        } else {
                                            if f32(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f616(f8(Str::lit("Bool")), List::<Adt24>::nil()))
                                            } else {
                                                if f32(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3942 = f93(v1, f8(Str::lit("items")));
                                                    match m3942 {
                                                        None => {
                                                            Ok(f664(f8(Str::lit("List")), f666()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f664(f8(Str::lit("List")), f728(v3, v23, {
                                                                let a3943 = v2;
                                                                let a3944 = f8(Str::lit("Item"));
                                                                append_bytes(a3943, a3944)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v22.clone(), f8(Str::lit("object"))) {
                                                        if f623(v1.clone()) {
                                                            Ok(f616(v2.clone(), List::cons(Adt24::C0(v2, v1), List::<Adt24>::nil())))
                                                        } else {
                                                            let m3945 = f661(v1);
                                                            match m3945 {
                                                                None => {
                                                                    Ok(f664(f8(Str::lit("List")), f721(f666())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f664(f8(Str::lit("List")), f721(f728(v3, v24, {
                                                                        let a3946 = v2;
                                                                        let a3947 = f8(Str::lit("Value"));
                                                                        append_bytes(a3946, a3947)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f615(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
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
                        let m3941 = f612(v20)?;
                        match m3941 {
                            None => {
                                Ok(f615(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f616(v21, List::<Adt24>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C8 => {
                Ok(f666())
            }
            Adt2::C9(_, _) => {
                Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C10(_, _, _) => {
                Ok(f615(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
