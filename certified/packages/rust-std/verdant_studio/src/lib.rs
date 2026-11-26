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
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1(Bytes, std::rc::Rc<Adt18>),
    C2(bool),
    C3(bool, u64),
    C4(bool, u64, u64),
    C5(Bytes),
    C6(std::rc::Rc<Adt18>),
    C7(std::rc::Rc<Adt18>),
    C8,
    C9(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C10(Bytes, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
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
    C0(Bytes, Adt18),
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
    F64(u8, Bytes),
    F162(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn0::F162(k0) => f162(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F94,
    F209,
    F300,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F94 => f94(p0, p1),
            Fn1::F209 => f209(p0, p1),
            Fn1::F300 => f300(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F176(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F176(k0) => f176(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F287(Bytes, u64, Bytes, u64),
    F368(Bytes),
    F369(u64, Bytes),
    F468(Bytes),
    F469(Bytes),
    F507(Bytes),
    F528(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F287(k0, k1, k2, k3) => f287(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F368(k0) => Ok(f368(k0.clone(), p0)),
            Fn3::F369(k0, k1) => f369(*k0, k1.clone(), p0),
            Fn3::F468(k0) => f468(k0.clone(), p0),
            Fn3::F469(k0) => f469(k0.clone(), p0),
            Fn3::F507(k0) => f507(k0.clone(), p0),
            Fn3::F528(k0) => f528(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F358(Bytes),
    F475(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F358(k0) => f358(k0.clone(), p0),
            Fn4::F475(k0, k1) => Ok(f475(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F360,
    F640,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F360 => f360(p0, p1),
            Fn5::F640 => f640(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F363,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F363 => f363(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F385,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F385 => f385(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F388,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F388 => f388(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F390,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F390 => f390(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F393,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F393 => f393(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F412(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn11::F412(k0, k1, k2) => f412(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F438,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F438 => f438(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F441,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F441 => f441(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F467(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn14::F467(k0) => f467(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F472(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn15::F472(k0) => f472(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F479,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F479 => f479(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F481,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F481 => f481(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F498,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F498 => f498(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F511,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F511 => f511(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F514,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F514 => f514(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F561,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F561 => f561(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F567,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F567 => f567(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F596,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F596 => f596(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F613(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F613(k0) => f613(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F623,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F623 => f623(p0),
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
    let m308 = f45(v0.clone(), f111());
    match m308.uncons() {
        None => {
            let m309 = f112(v0.clone(), f111())?;
            match m309.uncons() {
                None => {
                    let m312 = f113(match v0.clone() {
                        Adt1::C0(_, h310, _) => {
                            h310
                        }
                    }, f98(f114(), f115(match v0.clone() {
                        Adt1::C0(_, h311, _) => {
                            h311
                        }
                    }, false)));
                    match m312.uncons() {
                        None => {
                            let m314 = f45(v0.clone(), f115(match v0.clone() {
                                Adt1::C0(_, h313, _) => {
                                    h313
                                }
                            }, true));
                            match m314.uncons() {
                                None => {
                                    let m315 = f45(v0.clone(), f116());
                                    match m315.uncons() {
                                        None => {
                                            let m316 = f117(v0.clone())?;
                                            match m316 {
                                                None => {
                                                    let m318 = f118(v0.clone(), f115(match v0.clone() {
                                                        Adt1::C0(_, h317, _) => {
                                                            h317
                                                        }
                                                    }, true))?;
                                                    match m318 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f119(Adt16::C0(Adt15::C0(f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f120(v0.clone())?, f121(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f52(f53(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f122(f82(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f115(match v0 {
                                                                Adt1::C0(_, h319, _) => {
                                                                    h319
                                                                }
                                                            }, true)))?;
                                                            let (h320, _) = v13;
                                                            if h320 {
                                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h321) = v13;
                                                                    h321
                                                                }))
                                                            } else {
                                                                Ok(Adt11::C4({
                                                                    let (_, h322) = v13;
                                                                    h322
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

pub fn f70(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f123(match v0.clone() {
        Adt1::C0(_, h323, _) => {
            h323
        }
    }, f124())) {
        Ok(Adt11::C1(List::cons(f124(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f125(f126(f124(), f127(match v0.clone() {
            Adt1::C0(_, h324, _) => {
                h324
            }
        }, f124())))?;
        let m325 = f45(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m325.uncons() {
            None => {
                let m326 = f128(v0.clone(), f129(f81(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f130(v2, v1.clone()))?;
                match m326 {
                    None => {
                        f131(v0, v1)
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
    let m327 = f97(v0.clone());
    match m327.uncons() {
        None => {
            let m328 = f98(f45(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f132(), List::<Bytes>::nil())))), f99(v0.clone())?);
            match m328.uncons() {
                None => {
                    Ok(Adt11::C3(f133(f134(v0.clone())?, List::cons((f135(), f136()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f137()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f138()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f139()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f140()), List::cons((f8(Str::lit("docs/catalog.md")), f141(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f142(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f143(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f144()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m333 = {
        let a330 = v0;
        let a331 = 0u64;
        let a332 = {
            let a329 = v1.clone();
            length_bytes(a329)
        };
        slice_bytes(a330, a331, a332)
    };
    match m333 {
        None => {
            false
        }
        Some(v2) => {
            let a334 = v2;
            let a335 = v1;
            equal(a334, a335)
        }
    }
}

pub fn f73(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f145(v3.clone(), v1.clone(), f146(v3, v1, false)?)
    } else {
        if f28(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f145(v3.clone(), v1.clone(), f147(v3, v1)?)
        } else {
            if f28(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f145(v3.clone(), v1.clone(), f146(v3, v1, true)?)
            } else {
                if f28(v0.clone(), f8(Str::lit("stage-encodings"))) {
                    f145(v3.clone(), v1.clone(), f148(v3, v1)?)
                } else {
                    if f28(v0.clone(), f8(Str::lit("stage-mappings"))) {
                        f145(v3.clone(), v1.clone(), f149(v3, v1, false)?)
                    } else {
                        if f28(v0, f8(Str::lit("stage-matrix"))) {
                            f145(v3.clone(), v1.clone(), f149(v3, v1, true)?)
                        } else {
                            Ok(Adt11::C4(f66()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("admit"))) {
        f150(v3, v1)
    } else {
        if f28(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f151(v3, v1)
        } else {
            if f28(v0.clone(), f8(Str::lit("ingest-authority"))) {
                f152(v3, v1)
            } else {
                if f28(v0.clone(), f8(Str::lit("ingest-index"))) {
                    f153(v3, v1)
                } else {
                    if f28(v0.clone(), f8(Str::lit("merge-gate"))) {
                        f154(v3, v1)
                    } else {
                        if f28(v0.clone(), f8(Str::lit("propose-encoding"))) {
                            let m336 = v2;
                            match m336.uncons() {
                                None => {
                                    Ok(Adt11::C4(f66()))
                                }
                                Some((v4, v5)) => {
                                    f155(v3, v1, v4, v5)
                                }
                            }
                        } else {
                            if f28(v0.clone(), f8(Str::lit("admit-translated"))) {
                                let m337 = v2;
                                match m337.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f66()))
                                    }
                                    Some((v6, v7)) => {
                                        f156(v3, v1, v6, v7)
                                    }
                                }
                            } else {
                                if f28(v0, f8(Str::lit("record-decision"))) {
                                    f157(v3, v1)
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
    f158(v0.clone(), 0u64, {
        let a338 = v0;
        length_bytes(a338)
    })
}

pub fn f76(p343: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m342 = p343;
    {
        let mut m342 = m342;
        loop {
            let step = {
                let v0 = m342;
                let m339 = v0;
                match m339.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h340, _) = v2;
                            h340
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h341) = v2;
                                h341
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m342 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Bytes) -> R<Option<Adt13>> {
    let m344 = f159(v0)?;
    match m344 {
        None => {
            Ok(None::<Adt13>)
        }
        Some(v1) => {
            Ok(f160(v1))
        }
    }
}

pub fn f78(v0: Option<Adt13>) -> Option<Bytes> {
    let m345 = v0;
    match m345 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m346 = v1;
            match m346 {
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

pub fn f79(p352: Adt13, v1: Bytes) -> Option<Adt13> {
    let m351 = p352;
    {
        let mut m351 = m351;
        loop {
            let step = {
                let v0 = m351;
                let m347 = v0;
                match m347 {
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
                    Adt13::C4(r348) => {
                        let v5 = (*r348).clone();
                        Ok::<Adt13, Option<Adt13>>(v5)
                    }
                    Adt13::C5 => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C6(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C7(v2, r349, r350) => {
                        let v3 = (*r349).clone();
                        let v4 = (*r350).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt13, Option<Adt13>>(Some(v3))
                        } else {
                            Ok::<Adt13, Option<Adt13>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m353 = v0;
    match m353.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m354 = f161(v2, v1.clone())?;
            match m354 {
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
        let a355 = v0.clone();
        length_bytes(a355)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h356, _) = f63(Fn0::F162(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h356
    };
    let v3: u64 = {
        let (h357, _) = {
            let (_, h358) = v2.clone();
            h358
        };
        h357
    };
    let v4: List<Bytes> = {
        let (_, h359) = {
            let (_, h360) = v2;
            h360
        };
        h359
    };
    let a361 = v3;
    let a362 = v1;
    if nat_lt(a361, a362) {
        Ok(f65(List::cons(f35(v0, v3, {
            let a363 = v1;
            let a364 = v3;
            nat_sub(a363, a364)
        }), v4)))
    } else {
        Ok(f65(v4))
    }
}

pub fn f82(v0: Adt1, v1: Bytes) -> R<List<Adt13>> {
    Ok(f163(f46(v0, v1)?))
}

pub fn f83(v0: List<Adt13>, v1: Bytes) -> List<Adt13> {
    let m365 = v0;
    match m365.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f164(v2, v1.clone()), f83(v3, v1))
        }
    }
}

pub fn f84(p370: List<Bytes>, p371: List<Bytes>) -> bool {
    let m369 = (p370, p371);
    {
        let mut m369 = m369;
        loop {
            let step = {
                let (v0, v1) = m369;
                let m366 = v0;
                match m366.uncons() {
                    None => {
                        let m367 = v1;
                        match m367.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m368 = v1;
                        match m368.uncons() {
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
                Ok(next) => { m369 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<Adt13>) -> List<Bytes> {
    let m372 = v0;
    match m372.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m373 = f79(v1, f8(Str::lit("std")));
            match m373 {
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
    let a374 = f165(v0)?;
    let a375 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a374, a375))
}

pub fn f87(v0: List<Adt13>) -> Adt13 {
    let m376 = v0;
    match m376.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f87(v2)))
        }
    }
}

pub fn f88(v0: Adt12) -> Bytes {
    let m377 = v0;
    match m377 {
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
    let a379 = {
        let a378 = v2.clone();
        length_bytes(a378)
    };
    let a380 = 0u64;
    if nat_eq(a379, a380) {
        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m381 = f166(v0.clone(), f49())?;
        match m381 {
            None => {
                if f40(f41(f169(), match v0.clone() {
                    Adt1::C0(_, _, h382) => {
                        h382
                    }
                })) {
                    Ok(Adt11::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f169()))
                } else {
                    let m383 = f45(v0.clone(), List::cons(f170(), f171(f49())?));
                    match m383.uncons() {
                        None => {
                            Ok(Adt11::C3(f172(v0.clone(), f49(), v1.clone(), v2.clone(), f173(f53(v0, f170()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt11::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f43(f167(v3.clone())?, f168(v3, v2)))
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h384, _) = v2.clone();
        h384
    };
    let v4: u64 = {
        let (h385, _) = {
            let (_, h386) = v2.clone();
            h386
        };
        h385
    };
    let v5: List<Bytes> = {
        let (_, h387) = {
            let (_, h388) = v2;
            h388
        };
        h387
    };
    let m389 = f174(v0.clone(), v3);
    match m389 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a390 = v6;
            let a391 = v1;
            if equal(a390, a391) {
                Ok(Some(({
                    let a392 = v3;
                    let a393 = 1u64;
                    nat_add(a392, a393)?
                }, ({
                    let a394 = v3;
                    let a395 = 1u64;
                    nat_add(a394, a395)?
                }, List::cons(f35(v0, v4, {
                    let a396 = v3;
                    let a397 = v4;
                    nat_sub(a396, a397)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a398 = v3;
                    let a399 = 1u64;
                    nat_add(a398, a399)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f93(v0: Fn1, p403: List<Bytes>, p404: List<Bytes>) -> List<Bytes> {
    let m402 = (p403, p404);
    {
        let mut m402 = m402;
        loop {
            let step = {
                let (v1, v2) = m402;
                let m400 = v2;
                match m400.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c401: Fn1 = v0.clone();
                            c401.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m405 = f175(Fn2::F176(v1), None::<Bytes>, v0);
    match m405 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f96() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))
}

pub fn f97(v0: Adt1) -> List<Bytes> {
    if f123(match v0.clone() {
        Adt1::C0(_, h406, _) => {
            h406
        }
    }, f177()) {
        if f123(match v0 {
            Adt1::C0(_, h407, _) => {
                h407
            }
        }, f178()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f178(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f177(), List::<Bytes>::nil())
    }
}

pub fn f98(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m408 = v0;
    match m408.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f98(v3, v1))
        }
    }
}

pub fn f99(v0: Adt1) -> R<List<Bytes>> {
    Ok(f45(v0.clone(), List::cons(f179(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f109(v0)?)))))
}

pub fn f100(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f120(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f121(f82(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f101(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m409 = v0;
    match m409.uncons() {
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
    let m410 = v0;
    match m410.uncons() {
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
    let m411 = v0;
    match m411.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f98(f122(f163(f79(v1, f8(Str::lit("defers"))))), f103(v2))
        }
    }
}

pub fn f104(v0: List<Adt13>) -> List<Bytes> {
    let m412 = v0;
    match m412.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m413 = f79(v1, f8(Str::lit("std")));
            match m413 {
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
    let m414 = v0;
    match m414.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f133(f180(v2.clone()), f181(f182(v0, v1.clone(), v2)?));
    f183(f125(f184(v1))?, v3.clone(), {
        let a416 = {
            let a415 = v3;
            length_list(a415)
        };
        let a417 = 1u64;
        nat_add(a416, a417)?
    })
}

pub fn f107(v0: Adt14, v1: List<Adt13>) -> List<Adt13> {
    let m418 = v1;
    match m418.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            if if f185(v2.clone(), f8(Str::lit("subject"))) {
                f186(v0.clone(), v2.clone())
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
    let m419 = v1;
    match m419.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v2, v3)) => {
            let m420 = f187(f81(f53(v0.clone(), v2.clone()))?, f72(v2, f178()))?;
            match m420 {
                None => {
                    f108(v0, v3)
                }
                Some(v4) => {
                    Ok(f188(v4, f108(v0, v3)?))
                }
            }
        }
    }
}

pub fn f109(v0: Adt1) -> R<List<Bytes>> {
    Ok(f98(f125(f189(f177(), f127(match v0.clone() {
        Adt1::C0(_, h421, _) => {
            h421
        }
    }, f177())))?, f125(f189(f178(), f127(match v0 {
        Adt1::C0(_, h422, _) => {
            h422
        }
    }, f178())))?))
}

pub fn f110(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f111() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f179(), List::<Bytes>::nil())))))))
}

pub fn f112(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m423 = v1;
    match m423.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f56(v0.clone(), v2.clone()) {
                f190(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m424 = f46(v0.clone(), v2.clone())?;
                match m424 {
                    None => {
                        Ok(List::cons(v2, f112(v0, v3)?))
                    }
                    Some(_) => {
                        f112(v0, v3)
                    }
                }
            } else {
                f112(v0, v3)
            }
        }
    }
}

pub fn f113(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m425 = v1;
    match m425.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f123(v0.clone(), v2.clone()) {
                f113(v0, v3)
            } else {
                List::cons(v2, f113(v0, v3))
            }
        }
    }
}

pub fn f114() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f115(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m426 = v0;
    match m426.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f98(f191({
                let (h427, _) = v2;
                h427
            }, {
                let (_, h428) = v2;
                h428
            }, v1), f115(v3, v1))
        }
    }
}

pub fn f116() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f117(v0: Adt1) -> R<Option<Bytes>> {
    f192(f193(v0, f116())?)
}

pub fn f118(v0: Adt1, p432: List<Bytes>) -> R<Option<Bytes>> {
    let m431 = p432;
    {
        let mut m431 = m431;
        loop {
            let step = {
                let v1 = m431;
                let m429 = v1;
                match m429.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m430 = f194(f53(v0.clone(), v2.clone()), f195())?;
                        match m430 {
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
                Ok(next) => { m431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h433, _, _, _, _) => {
            h433
        }
    };
    let v2: Adt17 = f196(v1.clone())?;
    let v3: List<Bytes> = f197(v1.clone());
    let v4: List<Bytes> = f121(match v0.clone() {
        Adt16::C0(_, _, h434, _, _) => {
            h434
        }
    }, f8(Str::lit("path")));
    let m436 = f198(v2, List::<u64>::nil(), f199(v1.clone(), match v0.clone() {
        Adt16::C0(_, h435, _, _, _) => {
            h435
        }
    }, 0u64)?);
    match m436 {
        None => {
            if f201(f202(match v0.clone() {
                Adt16::C0(_, _, _, _, h438) => {
                    h438
                }
            }, f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h439) => {
                    h439
                }
            }, v4.clone()))?, f202(v4.clone(), f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h440) => {
                    h440
                }
            }, v4.clone()))?, f203(match v0.clone() {
                Adt16::C0(_, _, h441, _, _) => {
                    h441
                }
            })) {
                let m445 = f204(f202(match v0.clone() {
                    Adt16::C0(_, _, _, h442, _) => {
                        h442
                    }
                }, f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h443, _) => {
                        h443
                    }
                }))?, f202(v3.clone(), f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h444, _) => {
                        h444
                    }
                }))?);
                match m445 {
                    None => {
                        let m448 = f205(f121(match v0 {
                            Adt16::C0(_, h447, _, _, _) => {
                                h447
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m448 {
                            None => {
                                let m449 = f205(v4, List::<Bytes>::nil());
                                match m449 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f200(f98(v3, match v0 {
                            Adt16::C0(_, _, _, h446, _) => {
                                h446
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f200(match v1 {
                Adt15::C0(_, _, _, _, h437) => {
                    h437
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f120(v0: Adt1) -> R<List<Bytes>> {
    let m450 = f206(f53(v0, f179()))?;
    match m450.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f207(v2))
        }
    }
}

pub fn f121(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m451 = v0;
    match m451.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f47(v2, v1.clone()), f121(v3, v1))
        }
    }
}

pub fn f122(v0: List<Adt13>) -> List<Bytes> {
    let m452 = v0;
    match m452.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m453 = f78(Some(v1));
            match m453 {
                None => {
                    f122(v2)
                }
                Some(v3) => {
                    List::cons(v3, f122(v2))
                }
            }
        }
    }
}

pub fn f123(p457: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m456 = p457;
    {
        let mut m456 = m456;
        loop {
            let step = {
                let v0 = m456;
                let m454 = v0;
                match m454.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h455, _) = v2;
                            h455
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f125(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f208(f93(Fn1::F209, List::<Bytes>::nil(), f210(v0)?)))
}

pub fn f126(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m458 = v1;
    match m458.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f190(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f211(v0.clone(), v2), f126(v0, v3))
            } else {
                f126(v0, v3)
            }
        }
    }
}

pub fn f127(p463: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m462 = p463;
    {
        let mut m462 = m462;
        loop {
            let step = {
                let v0 = m462;
                let m459 = v0;
                match m459.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h460, _) = v2;
                            h460
                        }, v1.clone()) {
                            let (_, h461) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h461)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: Adt1, v1: List<List<Bytes>>, p466: List<Bytes>) -> R<Option<Bytes>> {
    let m465 = p466;
    {
        let mut m465 = m465;
        loop {
            let step = {
                let v2 = m465;
                let m464 = v2;
                match m464.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f212(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
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

pub fn f129(p469: List<Bytes>, p470: List<Bytes>, p471: List<List<Bytes>>) -> List<List<Bytes>> {
    let m468 = (p469, (p470, p471));
    {
        let mut m468 = m468;
        loop {
            let step = {
                let (v0, (v1, v2)) = m468;
                let m467 = v0;
                match m467.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f213(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f28(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f213(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m472 = v0;
    match m472.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f130(v3, v1)
            } else {
                List::cons(v2, f130(v3, v1))
            }
        }
    }
}

pub fn f131(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f123(match v0.clone() {
        Adt1::C0(_, h473, _) => {
            h473
        }
    }, f214())) {
        Ok(Adt11::C1(List::cons(f214(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f130(f125(f126(f214(), f127(match v0.clone() {
            Adt1::C0(_, h474, _) => {
                h474
            }
        }, f214())))?, v1.clone());
        let m475 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m475.uncons() {
            None => {
                let v5: List<Bytes> = f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m476 = f45(v0.clone(), f215(v0.clone(), v2.clone(), v5.clone())?);
                match m476.uncons() {
                    None => {
                        let m477 = f216(v0.clone(), v2, v5)?;
                        match m477 {
                            None => {
                                f217(v0, v1)
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

pub fn f132() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f133(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m478 = v0;
    match m478.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f133(v3, v1))
        }
    }
}

pub fn f134(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m479 = f82(v0.clone(), f132())?;
    match m479.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f218(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f135() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f136() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f137() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f219(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f138() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}      merges: ${{ steps.stale.outputs.merges }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          merges=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # Drafting drops a pull request's auto-merge, so the owner's merge method is kept\u{a}              # here and restored once the regenerated pull request is ready again.\u{a}              method=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge.merge_method // \"\"')\u{a}              if [ -n \"$method\" ]; then merges=\"$merges $pr:$method\"; fi\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"merges=$merges\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}          MERGES: ${{ needs.find.outputs.merges }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f219(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            # The owner's auto-merge comes back with the same method, and only where it was set.\u{a}            for entry in $MERGES; do\u{a}              if [ \"${entry/:*/}\" = \"$pr\" ] && ! gh pr merge \"$pr\" --auto \"--${entry#*:}\"; then\u{a}                echo \"::error::#${pr}: its auto-merge (${entry#*:}) could not be restored\"\u{a}                failed=\"$failed $pr\"\u{a}              fi\u{a}            done\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f139() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, and readers when it exists.\u{a}pids=\"\"\u{a}for project in studio readers; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.$project\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}"))
}

pub fn f140() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f141(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f100(v0.clone())?;
    let v2: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt13> = f107(v1.clone(), f108(v0.clone(), f109(v0.clone())?)?);
    let v4: List<List<Bytes>> = f220(f206(f53(v0, f179()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f221(v4.clone(), f207(v4), f182(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f106(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f142(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f222(f100(v0.clone())?, f108(v0.clone(), f109(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f143(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f223(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f224(f82(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f144() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f145(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m480 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m480.uncons() {
        None => {
            if f225(f81(f53(v0, v1))?)? {
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

pub fn f146(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m481 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m481.uncons() {
        None => {
            let v5: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
            if f40(f190(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f226(v5)?
                } else {
                    f227(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f211(f228(), f229(v1.clone())?), f53(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f211(f228(), f229(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f230()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f147(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m482 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m482.uncons() {
        None => {
            let m483 = f231(f81(f53(v0.clone(), v1.clone()))?)?;
            match m483 {
                None => {
                    Ok(Adt11::C4(f232()))
                }
                Some(v4) => {
                    let m484 = f233(f81(f53(v0, v1))?)?;
                    match m484 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f95(f38(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f234(v5)))
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
    let m485 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m485.uncons() {
        None => {
            let m486 = f235(f81(f53(v0, v1))?)?;
            match m486 {
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

pub fn f149(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m487 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m487.uncons() {
        None => {
            let v5: Bytes = f211(if v2 {
                f178()
            } else {
                f177()
            }, f229(v1.clone())?);
            if f40(f190(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m488 = f187(f81(f53(v0.clone(), v1.clone()))?, v2)?;
                match m488 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a490 = {
                            let a489 = v6;
                            length_list(a489)
                        };
                        let a491 = 0u64;
                        if nat_eq(a490, a491) {
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

pub fn f150(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m492 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m492.uncons() {
        None => {
            let v4: List<Bytes> = f81(f53(v0.clone(), v1))?;
            let m493 = f236(v4.clone(), f8(Str::lit("requirement")))?;
            match m493 {
                None => {
                    Ok(Adt11::C4(f230()))
                }
                Some(v5) => {
                    let m494 = f236(v4, f8(Str::lit("admission")))?;
                    match m494 {
                        None => {
                            Ok(Adt11::C4(f230()))
                        }
                        Some(v6) => {
                            let v7: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f98(f121(v5.clone(), f8(Str::lit("id"))), f121(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f125(f98(f121(v7.clone(), f8(Str::lit("id"))), f121(v8.clone(), f8(Str::lit("construct")))))?;
                            let m495 = f237(f104(v6.clone()), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m495 {
                                None => {
                                    let a497 = {
                                        let a496 = v9.clone();
                                        length_list(a496)
                                    };
                                    let a498 = 0u64;
                                    if nat_eq(a497, a498) {
                                        Ok(Adt11::C4(f230()))
                                    } else {
                                        if f40({
                                            let a501 = {
                                                let a499 = f125(f98(v9.clone(), v10.clone()))?;
                                                length_list(a499)
                                            };
                                            let a502 = {
                                                let a500 = f98(v9.clone(), v10);
                                                length_list(a500)
                                            };
                                            nat_eq(a501, a502)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f239(f8(Str::lit("model/requirements.json")), f188(v7, v5))?, List::cons(f239(f8(Str::lit("model/admissions.json")), f188(v8, v6))?, List::cons(f240(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f95(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f238(v11))
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

pub fn f151(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m503 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m503.uncons() {
        None => {
            let m504 = f241(v0.clone(), v1.clone())?;
            match m504 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m505 = f242(v4.clone(), f8(Str::lit("schema")));
                    match m505 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f243(v0, f244(v5.clone())?, v5, f245(v4.clone(), f8(Str::lit("name"))), f246(f247(f242(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f152(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m506 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m506.uncons() {
        None => {
            let m507 = f46(v0.clone(), v1.clone())?;
            match m507 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f40(f28(f47(v4.clone(), f8(Str::lit("schema"))), f248())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt13 = {
                            let m508 = f79(v4.clone(), f8(Str::lit("source")));
                            match m508 {
                                None => {
                                    Adt13::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f78(f79(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f211(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f211(v9.clone(), f8(Str::lit("binding.json"))), f53(v0.clone(), v1));
                        let m509 = v8;
                        match m509 {
                            None => {
                                f249(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f40(f28(f47(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f249(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f211(v9, f229(f47(v7.clone(), f8(Str::lit("url"))))?);
                                    if f40(f41(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h510) => {
                                            h510
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f47(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m511 = f45(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m511.uncons() {
                                            None => {
                                                if f28(f250(f53(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f249(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f153(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m512 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m512.uncons() {
        None => {
            let v4: Bytes = f53(v0.clone(), v1);
            let v5: List<List<Bytes>> = f206(v4.clone())?;
            if f40(f84(f251(v5.clone()), f252())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f253(Str::lit("id"), v6.clone())?, List::cons(f253(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f253(Str::lit("artifact"), f179())?, List::cons(f253(Str::lit("checksum"), f250(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f179(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f254(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f240(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f95(f207(f220(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f154(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m513 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m513.uncons() {
        None => {
            let m514 = f46(v0, v1)?;
            match m514 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f255(f163(Some(v4))) {
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

pub fn f155(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m515 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m515.uncons() {
        None => {
            let m516 = f241(v0.clone(), v1.clone())?;
            match m516 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f243(v0.clone(), f53(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f156(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m517 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil())))))));
    match m517.uncons() {
        None => {
            let m518 = f256(v3.clone());
            match m518.uncons() {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                }
                Some((v6, v7)) => {
                    if f72(v6.clone(), f8(Str::lit("read-"))) {
                        f257(v0.clone(), f53(v0, v1), v2, List::cons(v6, v7), f258(v3))
                    } else {
                        Ok(Adt11::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                    }
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f157(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m519 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f132(), List::<Bytes>::nil())));
    match m519.uncons() {
        None => {
            let m520 = f259(f81(f53(v0.clone(), v1))?)?;
            match m520 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt13> = f82(v0, f132())?;
                    let v6: List<Bytes> = f121(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f125(f121(v5.clone(), f8(Str::lit("at"))))?;
                    let a522 = {
                        let a521 = v6.clone();
                        length_list(a521)
                    };
                    let a523 = 0u64;
                    if if nat_eq(a522, a523) {
                        true
                    } else {
                        f40({
                            let a526 = {
                                let a524 = f125(f98(v6.clone(), v7.clone()))?;
                                length_list(a524)
                            };
                            let a527 = {
                                let a525 = f98(v6.clone(), v7);
                                length_list(a525)
                            };
                            nat_eq(a526, a527)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f239(f132(), f188(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f95(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f158(v0: Bytes, p581: u64, p582: u64) -> R<bool> {
    let m580 = (p581, p582);
    {
        let mut m580 = m580;
        loop {
            let step = {
                let (v1, v2) = m580;
                let m528 = v2;
                if m528 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m528 - 1;
                    let m531 = {
                        let a529 = v0.clone();
                        let a530 = v1;
                        index_bytes(a529, a530)
                    };
                    match m531 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f260({
                                let v5: u8 = v4;
                                let a576 = {
                                    let a552 = {
                                        let a540 = {
                                            let a534 = {
                                                let a532 = v5;
                                                let a533 = 128u8;
                                                fixed_u8::bit_and(a532, a533)
                                            };
                                            let a535 = 128u8;
                                            if equal(a534, a535) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a541 = {
                                            let a538 = {
                                                let a536 = v5;
                                                let a537 = 64u8;
                                                fixed_u8::bit_and(a536, a537)
                                            };
                                            let a539 = 64u8;
                                            if equal(a538, a539) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a540, a541)?
                                    };
                                    let a553 = {
                                        let a550 = {
                                            let a544 = {
                                                let a542 = v5;
                                                let a543 = 32u8;
                                                fixed_u8::bit_and(a542, a543)
                                            };
                                            let a545 = 32u8;
                                            if equal(a544, a545) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a551 = {
                                            let a548 = {
                                                let a546 = v5;
                                                let a547 = 16u8;
                                                fixed_u8::bit_and(a546, a547)
                                            };
                                            let a549 = 16u8;
                                            if equal(a548, a549) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a550, a551)?
                                    };
                                    nat_add(a552, a553)?
                                };
                                let a577 = {
                                    let a574 = {
                                        let a562 = {
                                            let a556 = {
                                                let a554 = v5;
                                                let a555 = 8u8;
                                                fixed_u8::bit_and(a554, a555)
                                            };
                                            let a557 = 8u8;
                                            if equal(a556, a557) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a563 = {
                                            let a560 = {
                                                let a558 = v5;
                                                let a559 = 4u8;
                                                fixed_u8::bit_and(a558, a559)
                                            };
                                            let a561 = 4u8;
                                            if equal(a560, a561) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a562, a563)?
                                    };
                                    let a575 = {
                                        let a572 = {
                                            let a566 = {
                                                let a564 = v5;
                                                let a565 = 2u8;
                                                fixed_u8::bit_and(a564, a565)
                                            };
                                            let a567 = 2u8;
                                            if equal(a566, a567) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a573 = {
                                            let a570 = {
                                                let a568 = v5;
                                                let a569 = 1u8;
                                                fixed_u8::bit_and(a568, a569)
                                            };
                                            let a571 = 1u8;
                                            if equal(a570, a571) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a572, a573)?
                                    };
                                    nat_add(a574, a575)?
                                };
                                nat_add(a576, a577)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a578 = v1;
                                    let a579 = 1u64;
                                    nat_add(a578, a579)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
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

pub fn f159(v0: Bytes) -> R<Option<Adt18>> {
    let v1: Bytes = f261(v0)?;
    let m586 = f262(v1.clone(), 0u64, {
        let a584 = {
            let a583 = v1.clone();
            length_bytes(a583)
        };
        let a585 = 2u64;
        nat_add(a584, a585)?
    }, 0u64)?;
    match m586 {
        None => {
            Ok(None::<Adt18>)
        }
        Some(v2) => {
            let a589 = {
                let (_, h587) = v2;
                h587
            };
            let a590 = {
                let a588 = v1;
                length_bytes(a588)
            };
            if nat_eq(a589, a590) {
                Ok(Some({
                    let (h591, _) = v2;
                    h591
                }))
            } else {
                Ok(None::<Adt18>)
            }
        }
    }
}

pub fn f160(v0: Adt18) -> Option<Adt13> {
    let m592 = v0;
    match m592 {
        Adt18::C0 => {
            Some(Adt13::C3(std::rc::Rc::new(Adt13::C5)))
        }
        Adt18::C1(_, _) => {
            None::<Adt13>
        }
        Adt18::C2(v3) => {
            Some(Adt13::C0(v3))
        }
        Adt18::C3(v4, v5) => {
            Some(Adt13::C1(v4, v5))
        }
        Adt18::C4(_, _, _) => {
            None::<Adt13>
        }
        Adt18::C5(v9) => {
            Some(Adt13::C2(v9))
        }
        Adt18::C6(r599) => {
            let v10 = (*r599).clone();
            let m593 = f160(v10);
            match m593 {
                None => {
                    None::<Adt13>
                }
                Some(v11) => {
                    Some(Adt13::C3(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt18::C7(r600) => {
            let v12 = (*r600).clone();
            let m594 = f160(v12);
            match m594 {
                None => {
                    None::<Adt13>
                }
                Some(v13) => {
                    Some(Adt13::C4(std::rc::Rc::new(v13)))
                }
            }
        }
        Adt18::C8 => {
            Some(Adt13::C5)
        }
        Adt18::C9(r601, r602) => {
            let v14 = (*r601).clone();
            let v15 = (*r602).clone();
            let m595 = f160(v14);
            match m595 {
                None => {
                    None::<Adt13>
                }
                Some(v16) => {
                    let m596 = f160(v15);
                    match m596 {
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
        Adt18::C10(v18, r603, r604) => {
            let v19 = (*r603).clone();
            let v20 = (*r604).clone();
            let m597 = f160(v19);
            match m597 {
                None => {
                    None::<Adt13>
                }
                Some(v21) => {
                    let m598 = f160(v20);
                    match m598 {
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

pub fn f161(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), v1) {
        let m605 = f33(v0, 34u8)?;
        match m605.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m606 = v3;
                match m606.uncons() {
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

pub fn f162(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f263(v0, v1)
}

pub fn f163(v0: Option<Adt13>) -> List<Adt13> {
    let m607 = v0;
    match m607 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            f264(v1)
        }
    }
}

pub fn f164(v0: Adt13, v1: Bytes) -> Adt13 {
    let m608 = v0;
    match m608 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r609) => {
            let v10 = (*r609).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r610) => {
            let v5 = (*r610).clone();
            Adt13::C4(std::rc::Rc::new(f164(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r611, r612) => {
            let v11 = (*r611).clone();
            let v12 = (*r612).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r613, r614) => {
            let v3 = (*r613).clone();
            let v4 = (*r614).clone();
            if f28(v2.clone(), f8(Str::lit("std"))) {
                Adt13::C7(v2, std::rc::Rc::new(f265(v3, v1.clone())), std::rc::Rc::new(f164(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f164(v4, v1)))
            }
        }
    }
}

pub fn f165(v0: Adt13) -> R<Bytes> {
    let m615 = v0;
    match m615 {
        Adt13::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt13::C1(v2, v3) => {
            f266(v2, v3)
        }
        Adt13::C2(v4) => {
            f267(v4)
        }
        Adt13::C3(r616) => {
            let v5 = (*r616).clone();
            Ok(f268(f269(v5)?))
        }
        Adt13::C4(r617) => {
            let v6 = (*r617).clone();
            f270(f271(v6)?)
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

pub fn f166(v0: Adt1, p621: List<Adt12>) -> R<Option<Adt12>> {
    let m620 = p621;
    {
        let mut m620 = m620;
        loop {
            let step = {
                let v1 = m620;
                let m618 = v1;
                match m618.uncons() {
                    None => {
                        Err::<List<Adt12>, R<Option<Adt12>>>(Ok(None::<Adt12>))
                    }
                    Some((v2, v3)) => {
                        if f41(f167(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h619) => {
                                h619
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
                Ok(next) => { m620 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Adt12) -> R<Bytes> {
    Ok(f211(f272(), f229(f273(v0))?))
}

pub fn f168(v0: Adt12, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f273(v0), List::<Bytes>::nil())))))
}

pub fn f169() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f170() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f171(v0: List<Adt12>) -> R<List<Bytes>> {
    let m622 = v0;
    match m622.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f167(v1)?, f171(v2)?))
        }
    }
}

pub fn f172(v0: Adt1, v1: List<Adt12>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m623 = v1;
    match m623.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f57(f58(v5.clone(), f50(v2.clone()))), f274(v5.clone(), v2.clone(), v3.clone(), f250(f53(v0.clone(), f167(v5)?))?, v4.clone())?), f172(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f173(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f275(v0.clone())?;
    let v2: u64 = f276(v0.clone(), v1)?;
    Ok(f35(v0, v2, {
        let a624 = v1;
        let a625 = v2;
        nat_sub(a624, a625)
    }))
}

pub fn f174(v0: Bytes, v1: u64) -> Option<u8> {
    let a626 = v0;
    let a627 = v1;
    index_bytes(a626, a627)
}

pub fn f175(v0: Fn2, p631: Option<Bytes>, p632: List<Bytes>) -> Option<Bytes> {
    let m630 = (p631, p632);
    {
        let mut m630 = m630;
        loop {
            let step = {
                let (v1, v2) = m630;
                let m628 = v2;
                match m628.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c629: Fn2 = v0.clone();
                            c629.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m630 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m633 = v1;
    match m633 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a636 = {
                    let a634 = v3;
                    let a635 = v0;
                    append_bytes(a634, a635)
                };
                let a637 = v2;
                append_bytes(a636, a637)
            })
        }
    }
}

pub fn f177() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f178() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f179() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f180(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m638 = v0;
    match m638.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f47(v1.clone(), f8(Str::lit("subject"))), f47(v1, f8(Str::lit("object")))), f180(v2))
        }
    }
}

pub fn f181(v0: List<Adt19>) -> List<(Bytes, Bytes)> {
    let m639 = v0;
    match m639.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt19::C0(h640, _, _) => {
                    h640
                }
            }, match v1 {
                Adt19::C0(_, _, h641) => {
                    h641
                }
            }), f181(v2))
        }
    }
}

pub fn f182(v0: Adt14, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Adt19>> {
    let v3: List<(Bytes, Bytes)> = f277(v1);
    let v4: List<(Bytes, Bytes)> = f278(v0, v2);
    f279(v3.clone(), f133(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f183(p648: List<Bytes>, v1: List<(Bytes, Bytes)>, p649: u64) -> R<List<Bytes>> {
    let m647 = (p648, p649);
    {
        let mut m647 = m647;
        loop {
            let step = {
                let (v0, v2) = m647;
                let m642 = v2;
                if m642 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m642 - 1;
                    let v4: List<Bytes> = f125(f98(v0.clone(), f280(v0.clone(), v1.clone())))?;
                    let a645 = {
                        let a643 = v4.clone();
                        length_list(a643)
                    };
                    let a646 = {
                        let a644 = v0.clone();
                        length_list(a644)
                    };
                    if nat_eq(a645, a646) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: List<Adt13>) -> List<Bytes> {
    let m650 = v0;
    match m650.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m651 = f79(v1, f8(Str::lit("std")));
            match m651 {
                None => {
                    f184(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3.clone(), f8(Str::lit("authority"))), if f185(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f47(v3, f8(Str::lit("row"))), f184(v2))
                    } else {
                        f184(v2)
                    })
                }
            }
        }
    }
}

pub fn f185(v0: Adt13, v1: Bytes) -> bool {
    f40({
        let a653 = {
            let a652 = f47(v0, v1);
            length_bytes(a652)
        };
        let a654 = 0u64;
        nat_eq(a653, a654)
    })
}

pub fn f186(v0: Adt14, v1: Adt13) -> bool {
    if f281(v0.clone(), v1.clone(), f47(v1.clone(), f8(Str::lit("subject")))) {
        f281(v0, v1.clone(), f47(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f187(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt13>>> {
    let m655 = v0;
    match m655.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f173(v2)?;
            let a657 = {
                let a656 = v4.clone();
                length_bytes(a656)
            };
            let a658 = 0u64;
            if nat_eq(a657, a658) {
                f187(v3, v1)
            } else {
                let m659 = f77(v4)?;
                match m659 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f282(v5.clone())
                        } else {
                            f283(v5.clone())
                        } {
                            let m660 = f187(v3, v1)?;
                            match m660 {
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

pub fn f188(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m661 = v0;
    match m661.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f188(v3, v1))
        }
    }
}

pub fn f189(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m662 = v1;
    match m662.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f190(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f211(v0.clone(), v2), f189(v0, v3))
            } else {
                f189(v0, v3)
            }
        }
    }
}

pub fn f190(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a663 = v0.clone();
        length_bytes(a663)
    };
    let v3: u64 = {
        let a664 = v1.clone();
        length_bytes(a664)
    };
    let a665 = v3;
    let a666 = v2;
    if nat_le(a665, a666) {
        let m672 = {
            let a669 = v0;
            let a670 = {
                let a667 = v2;
                let a668 = v3;
                nat_sub(a667, a668)
            };
            let a671 = v3;
            slice_bytes(a669, a670, a671)
        };
        match m672 {
            None => {
                false
            }
            Some(v4) => {
                let a673 = v4;
                let a674 = v1;
                equal(a673, a674)
            }
        }
    } else {
        false
    }
}

pub fn f191(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m675 = v1;
    match m675.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f284(v3.clone())
            } else {
                f40(f284(v3.clone()))
            } {
                List::cons(f211(v0.clone(), v3), f191(v0, v4, v2))
            } else {
                f191(v0, v4, v2)
            }
        }
    }
}

pub fn f192(p678: List<Bytes>) -> R<Option<Bytes>> {
    let m677 = p678;
    {
        let mut m677 = m677;
        loop {
            let step = {
                let v0 = m677;
                let m676 = v0;
                match m676.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f285(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f173(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m677 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m679 = v1;
    match m679.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f98(f81(f53(v0.clone(), v2))?, f193(v0, v3)?))
        }
    }
}

pub fn f194(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a680 = v0.clone();
        length_bytes(a680)
    };
    let v3: u64 = {
        let a681 = v1.clone();
        length_bytes(a681)
    };
    let v4: u64 = {
        let (h682, _) = f286(Fn3::F287(v1.clone(), v3, v0.clone(), v2), {
            let a683 = v2;
            let a684 = 1u64;
            nat_add(a683, a684)?
        }, 0u64)?;
        h682
    };
    if f288(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f195() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f196(v0: Adt15) -> R<Adt17> {
    Ok(Adt17::C0(f289(match v0.clone() {
        Adt15::C0(h685, _, _, _, _) => {
            h685
        }
    }, 0u64)?, f289(match v0.clone() {
        Adt15::C0(_, h686, _, _, _) => {
            h686
        }
    }, 0u64)?, f289(match v0.clone() {
        Adt15::C0(_, _, h687, _, _) => {
            h687
        }
    }, 0u64)?, f289(match v0 {
        Adt15::C0(_, _, _, h688, _) => {
            h688
        }
    }, 0u64)?))
}

pub fn f197(v0: Adt15) -> List<Bytes> {
    f98(match v0.clone() {
        Adt15::C0(h689, _, _, _, _) => {
            h689
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, h690, _, _) => {
            h690
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, _, h691, _) => {
            h691
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h692) => {
            h692
        }
    })))
}

pub fn f198(v0: Adt17, p698: List<u64>, p699: List<Adt20>) -> Option<u64> {
    let m697 = (p698, p699);
    {
        let mut m697 = m697;
        loop {
            let step = {
                let (v1, v2) = m697;
                let m693 = v2;
                match m693.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f290(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h694) => {
                                h694
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h695, _) => {
                                    h695
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h696, _) => {
                                    h696
                                }
                            }))
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

pub fn f199(v0: Adt15, v1: List<Adt13>, v2: u64) -> R<List<Adt20>> {
    let m700 = v1;
    match m700.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f291(v0.clone(), v3)?), f199(v0, v4, {
                let a701 = v2;
                let a702 = 1u64;
                nat_add(a701, a702)?
            })?))
        }
    }
}

pub fn f200(p706: List<Bytes>, p707: u64) -> Bytes {
    let m705 = (p706, p707);
    {
        let mut m705 = m705;
        loop {
            let step = {
                let (v0, v1) = m705;
                let m703 = v0;
                match m703.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m704 = v1;
                        if m704 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m704 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m705 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f292(v0.clone(), v1.clone()) {
        if f292(v1, v0) {
            f293(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f202(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m708 = v0;
    match m708.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f294(v2, v1.clone())?, f202(v3, v1)?))
        }
    }
}

pub fn f203(v0: List<Adt13>) -> List<u64> {
    let m709 = v0;
    match m709.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f295(v1), f203(v2))
        }
    }
}

pub fn f204(p712: List<u64>, v1: List<u64>) -> Option<u64> {
    let m711 = p712;
    {
        let mut m711 = m711;
        loop {
            let step = {
                let v0 = m711;
                let m710 = v0;
                match m710.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f296(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m711 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(p715: List<Bytes>, p716: List<Bytes>) -> Option<Bytes> {
    let m714 = (p715, p716);
    {
        let mut m714 = m714;
        loop {
            let step = {
                let (v0, v1) = m714;
                let m713 = v0;
                match m713.uncons() {
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
                Ok(next) => { m714 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt23 = f297(v0.clone(), 0u64, {
        let a717 = v0;
        length_bytes(a717)
    }, Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt23 = {
        let a720 = {
            let a719 = match v1.clone() {
                Adt23::C0(_, _, h718, _, _) => {
                    h718
                }
            };
            length_bytes(a719)
        };
        let a721 = 0u64;
        if if nat_eq(a720, a721) {
            let m723 = match v1.clone() {
                Adt23::C0(_, _, _, h722, _) => {
                    h722
                }
            };
            match m723.uncons() {
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
            f298(v1)
        }
    };
    Ok(f299(match v4 {
        Adt23::C0(_, _, _, _, h724) => {
            h724
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f207(v0: List<List<Bytes>>) -> List<Bytes> {
    let m725 = v0;
    match m725.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m726 = v1;
            match m726.uncons() {
                None => {
                    f207(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f207(v2))
                }
            }
        }
    }
}

pub fn f208(v0: List<Bytes>) -> List<Bytes> {
    f93(Fn1::F300, List::<Bytes>::nil(), v0)
}

pub fn f209(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f301(v0, v1)
}

pub fn f210(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f302(f303(f304(v0))?))
}

pub fn f211(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f212(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m727 = f46(v0, v2)?;
    match m727 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f305(v1, f47(v3.clone(), f8(Str::lit("id"))))?;
            let m728 = v4.clone();
            match m728.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m730 = f78(f79({
                        let m729 = f79(v3.clone(), f8(Str::lit("source")));
                        match m729 {
                            None => {
                                Adt13::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m730 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f40(f41(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f306(f307(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f213(p733: List<Bytes>, p734: List<Bytes>) -> List<Bytes> {
    let m732 = (p733, p734);
    {
        let mut m732 = m732;
        loop {
            let step = {
                let (v0, v1) = m732;
                let m731 = v0;
                match m731.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f215(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m735 = v1;
    match m735.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt18 = f308(v0.clone(), v3)?;
            if f41(f309(f245(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f310(v5)?, f215(v0, v4, v2)?))
            } else {
                f215(v0, v4, v2)
            }
        }
    }
}

pub fn f216(v0: Adt1, p739: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m738 = p739;
    {
        let mut m738 = m738;
        loop {
            let step = {
                let v1 = m738;
                let m736 = v1;
                match m736.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt18 = f308(v0.clone(), v3.clone())?;
                        if if f41(f309(f245(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f28(f244({
                                let m737 = f242(v5.clone(), f8(Str::lit("schema")));
                                match m737 {
                                    None => {
                                        Adt18::C8
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f53(v0.clone(), f310(v5)?))
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
                Ok(next) => { m738 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f123(match v0.clone() {
        Adt1::C0(_, h740, _) => {
            h740
        }
    }, f228())) {
        Ok(Adt11::C1(List::cons(f228(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f130(f125(f311(f127(match v0.clone() {
            Adt1::C0(_, h741, _) => {
                h741
            }
        }, f228())))?, v1);
        let m742 = f45(v0.clone(), List::cons(f132(), List::cons(f179(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m742.uncons() {
            None => {
                let v5: (bool, Bytes) = f312(v0.clone(), v2)?;
                let (h743, _) = v5;
                if h743 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h744) = v5;
                        h744
                    }))
                } else {
                    let m745 = f97(v0.clone());
                    match m745.uncons() {
                        None => {
                            let m746 = f99(v0.clone())?;
                            match m746.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h747) = v5;
                                        h747
                                    }, List::cons(f7(f125(f313(f100(v0.clone())?, f108(v0.clone(), f109(v0.clone())?)?))?), List::cons(f314(v0)?, List::<Bytes>::nil()))))))
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

pub fn f218(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f315(f82(v0, f132())?)?)))
}

pub fn f219() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f220(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m748 = v0;
    match m748.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f221(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt19>, v3: List<Adt13>, v4: List<Bytes>) -> List<Bytes> {
    let m749 = v1;
    match m749.uncons() {
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
                let a751 = {
                    let a750 = v7.clone();
                    length_list(a750)
                };
                let a752 = 0u64;
                if nat_eq(a751, a752) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f95(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f221(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f222(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m753 = v1;
    match m753.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f185(v2.clone(), f8(Str::lit("domain"))) {
                f319(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f47(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m754 = f79(v2.clone(), f8(Str::lit("role")));
                    match m754 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f47(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f47(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f320(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m755 = f79(v2.clone(), f8(Str::lit("origin")));
                    match m755 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f47(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m756 = f79(v2, f8(Str::lit("evidence")));
                    match m756 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f47(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f222(v0, v3))
            } else {
                f222(v0, v3)
            }
        }
    }
}

pub fn f223(v0: List<Adt13>) -> List<Bytes> {
    let m757 = v0;
    match m757.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f47(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f47(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f47(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f223(v2))
        }
    }
}

pub fn f224(v0: List<Adt13>) -> List<Bytes> {
    let m758 = v0;
    match m758.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f47(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f321(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f224(v2))
        }
    }
}

pub fn f225(p764: List<Bytes>) -> R<bool> {
    let m763 = p764;
    {
        let mut m763 = m763;
        loop {
            let step = {
                let v0 = m763;
                let m759 = v0;
                match m759.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a761 = {
                            let a760 = f173(v1)?;
                            length_bytes(a760)
                        };
                        let a762 = 0u64;
                        if nat_eq(a761, a762) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m763 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: List<Bytes>) -> R<bool> {
    if f322(f259(v0.clone())?) {
        Ok(f40({
            let a766 = {
                let a765 = f323(f259(v0)?);
                length_list(a765)
            };
            let a767 = 0u64;
            nat_eq(a766, a767)
        }))
    } else {
        Ok(false)
    }
}

pub fn f227(v0: List<Bytes>) -> R<bool> {
    if f322(f236(v0.clone(), f8(Str::lit("requirement")))?) {
        if f322(f236(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f40({
                let a769 = {
                    let a768 = f188(f323(f236(v0.clone(), f8(Str::lit("requirement")))?), f323(f236(v0, f8(Str::lit("admission")))?));
                    length_list(a768)
                };
                let a770 = 0u64;
                nat_eq(a769, a770)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f228() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f229(v0: Bytes) -> R<Bytes> {
    Ok(f34(f33(v0, 47u8)?))
}

pub fn f230() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f231(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m771 = v0;
    match m771.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f173(v1)?;
            let a773 = {
                let a772 = v3.clone();
                length_bytes(a772)
            };
            let a774 = 0u64;
            if nat_eq(a773, a774) {
                f231(v2)
            } else {
                let m775 = f77(v3.clone())?;
                match m775 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        if f324(f47(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m776 = f231(v2)?;
                            match m776 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f325(v5), {
                                        let a777 = v3;
                                        let a778 = f8(Str::lit("\u{a}"));
                                        append_bytes(a777, a778)
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

pub fn f232() -> Bytes {
    let a779 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a779)
}

pub fn f233(p784: List<Bytes>) -> R<Option<Bytes>> {
    let m783 = p784;
    {
        let mut m783 = m783;
        loop {
            let step = {
                let v0 = m783;
                let m780 = v0;
                match m780.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m781 = f77(f173(v1)?)?;
                        match m781 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m782 = f79(v3.clone(), f8(Str::lit("source")));
                                match m782 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f326(f47(v4.clone(), f8(Str::lit("redistribution"))), f47(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m783 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: Bytes) -> Bytes {
    let a789 = {
        let a785 = Str::lit("binding ");
        utf8_encode(a785)
    };
    let a790 = {
        let a787 = v0;
        let a788 = {
            let a786 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a786)
        };
        append_bytes(a787, a788)
    };
    append_bytes(a789, a790)
}

pub fn f235(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m791 = v0;
    match m791.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f173(v1)?;
            let a793 = {
                let a792 = v3.clone();
                length_bytes(a792)
            };
            let a794 = 0u64;
            if nat_eq(a793, a794) {
                f235(v2)
            } else {
                let m795 = f159(v3.clone())?;
                match m795 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f327(v4.clone()) {
                            let m796 = f235(v2)?;
                            match m796 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f214(), List::cons(f8(Str::lit("/")), List::cons(f328(f245(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a797 = v3;
                                        let a798 = f8(Str::lit("\u{a}"));
                                        append_bytes(a797, a798)
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

pub fn f236(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt13>>> {
    let m799 = v0;
    match m799.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f173(v2)?;
            let a801 = {
                let a800 = v4.clone();
                length_bytes(a800)
            };
            let a802 = 0u64;
            if nat_eq(a801, a802) {
                f236(v3, v1)
            } else {
                let m803 = f77(v4)?;
                match m803 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f329(v5.clone());
                        let a805 = {
                            let a804 = v6.clone();
                            length_bytes(a804)
                        };
                        let a806 = 0u64;
                        if nat_eq(a805, a806) {
                            Ok(None::<List<Adt13>>)
                        } else {
                            let m807 = f236(v3, v1.clone())?;
                            match m807 {
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

pub fn f237(p810: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m809 = p810;
    {
        let mut m809 = m809;
        loop {
            let step = {
                let v0 = m809;
                let m808 = v0;
                match m808.uncons() {
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
                Ok(next) => { m809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f239(v0: Bytes, v1: List<Adt13>) -> R<(Bytes, Bytes)> {
    Ok((v0, f86(Adt13::C3(std::rc::Rc::new(f87(v1))))?))
}

pub fn f240(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f86(Adt13::C3(std::rc::Rc::new(f330(f125(f98(v1, f122(f82(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f241(v0: Adt1, v1: Bytes) -> R<Option<Adt18>> {
    f159(f53(v0, v1))
}

pub fn f242(p816: Adt18, v1: Bytes) -> Option<Adt18> {
    let m815 = p816;
    {
        let mut m815 = m815;
        loop {
            let step = {
                let v0 = m815;
                let m811 = v0;
                match m811 {
                    Adt18::C0 => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C1(_, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C3(_, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C4(_, _, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C5(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C6(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C7(r812) => {
                        let v5 = (*r812).clone();
                        Ok::<Adt18, Option<Adt18>>(v5)
                    }
                    Adt18::C8 => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C9(_, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C10(v2, r813, r814) => {
                        let v3 = (*r813).clone();
                        let v4 = (*r814).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt18, Option<Adt18>>(Some(v3))
                        } else {
                            Ok::<Adt18, Option<Adt18>>(v4)
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

pub fn f243(v0: Adt1, v1: Bytes, v2: Adt18, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m817 = f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m817.uncons() {
        None => {
            let m819 = f237(f104(List::cons(Adt13::C4(std::rc::Rc::new({
                let m818 = f331(v4.clone());
                match m818 {
                    None => {
                        Adt13::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt13>::nil())), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m819 {
                None => {
                    f332(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f238(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f244(v0: Adt18) -> R<Bytes> {
    let a820 = f333(v0)?;
    let a821 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a820, a821))
}

pub fn f245(v0: Adt18, v1: Bytes) -> Bytes {
    let m822 = f334(f242(v0, v1));
    match m822 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f246(v0: List<Adt18>) -> List<Bytes> {
    let m823 = v0;
    match m823.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m824 = f334(Some(v1));
            match m824 {
                None => {
                    f246(v2)
                }
                Some(v3) => {
                    List::cons(v3, f246(v2))
                }
            }
        }
    }
}

pub fn f247(v0: Option<Adt18>) -> List<Adt18> {
    let m825 = v0;
    match m825 {
        None => {
            List::<Adt18>::nil()
        }
        Some(v1) => {
            f335(v1)
        }
    }
}

pub fn f248() -> Bytes {
    f336()
}

pub fn f249(v0: Adt1, v1: Adt13, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f254(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f337(v1, v4)?)?), List::cons(f240(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f250(v0: Bytes) -> R<Bytes> {
    Ok(f338(f339(v0)?))
}

pub fn f251(v0: List<List<Bytes>>) -> List<Bytes> {
    let m826 = v0;
    match m826.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f252() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f253(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f306(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f254(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f340(f129(f81(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f341(f342(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f255(p829: List<Adt13>) -> bool {
    let m828 = p829;
    {
        let mut m828 = m828;
        loop {
            let step = {
                let v0 = m828;
                let m827 = v0;
                match m827.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f28(f47(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f41(f8(Str::lit("studio")), f121(f343(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: List<Bytes>) -> List<Bytes> {
    let m830 = v0;
    match m830.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f344(v1.clone()) {
                List::<Bytes>::nil()
            } else {
                List::cons(v1, f256(v2))
            }
        }
    }
}

pub fn f257(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>, v4: List<Bytes>) -> R<Adt11> {
    let m831 = f331(v4);
    match m831 {
        None => {
            Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m832 = f237(f104(List::cons(Adt13::C4(std::rc::Rc::new(v5.clone())), List::<Adt13>::nil())), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m832 {
                None => {
                    f345(v0, v2.clone(), Adt13::C7(f8(Str::lit("translation")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("command")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f330(v3)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt13::C2(f250(v1.clone())?)), std::rc::Rc::new(Adt13::C5))))))), std::rc::Rc::new(v5)), f346(v2, f347(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f238(v6))
                }
            }
        }
    }
}

pub fn f258(p835: List<Bytes>) -> List<Bytes> {
    let m834 = p835;
    {
        let mut m834 = m834;
        loop {
            let step = {
                let v0 = m834;
                let m833 = v0;
                match m833.uncons() {
                    None => {
                        Err::<List<Bytes>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v1, v2)) => {
                        if f344(v1.clone()) {
                            Err::<List<Bytes>, List<Bytes>>(List::cons(v1, v2))
                        } else {
                            Ok::<List<Bytes>, List<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m834 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: List<Bytes>) -> R<Option<List<Adt13>>> {
    let m836 = v0;
    match m836.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f173(v1)?;
            let a838 = {
                let a837 = v3.clone();
                length_bytes(a837)
            };
            let a839 = 0u64;
            if nat_eq(a838, a839) {
                f259(v2)
            } else {
                let m840 = f77(v3)?;
                match m840 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v4) => {
                        if if f185(v4.clone(), f8(Str::lit("agent"))) {
                            if f185(v4.clone(), f8(Str::lit("at"))) {
                                if f185(v4.clone(), f8(Str::lit("quote"))) {
                                    f348(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m841 = f259(v2)?;
                            match m841 {
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

pub fn f260(v0: u64) -> bool {
    let a842 = v0;
    let a843 = 127u64;
    nat_le(a842, a843)
}

pub fn f261(v0: Bytes) -> R<Bytes> {
    f349(v0.clone(), 0u64, {
        let a844 = v0;
        length_bytes(a844)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f262(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt18, u64)>> {
    let m845 = v2;
    if m845 == 0 {
        Ok(None::<(Adt18, u64)>)
    } else {
        let v4 = m845 - 1;
        let v5: u64 = f350(v0.clone(), v1)?;
        let a846 = v3;
        let a847 = 1u64;
        if nat_eq(a846, a847) {
            let m848 = f262(v0.clone(), v1, v4, 0u64)?;
            match m848 {
                None => {
                    Ok(None::<(Adt18, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f350(v0.clone(), {
                        let (_, h849) = v6.clone();
                        h849
                    })?;
                    let a850 = v7;
                    let a851 = 44u64;
                    if nat_eq(a850, a851) {
                        let m855 = f262(v0, {
                            let a853 = {
                                let (_, h852) = v6;
                                h852
                            };
                            let a854 = 1u64;
                            nat_add(a853, a854)?
                        }, v4, 1u64)?;
                        match m855 {
                            None => {
                                Ok(None::<(Adt18, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt18::C9(std::rc::Rc::new({
                                    let (h856, _) = v6;
                                    h856
                                }), std::rc::Rc::new({
                                    let (h857, _) = v8;
                                    h857
                                })), {
                                    let (_, h858) = v8;
                                    h858
                                })))
                            }
                        }
                    } else {
                        let a859 = v7;
                        let a860 = 93u64;
                        if nat_eq(a859, a860) {
                            Ok(Some((Adt18::C9(std::rc::Rc::new({
                                let (h861, _) = v6;
                                h861
                            }), std::rc::Rc::new(Adt18::C8)), {
                                let a863 = {
                                    let (_, h862) = v6;
                                    h862
                                };
                                let a864 = 1u64;
                                nat_add(a863, a864)?
                            })))
                        } else {
                            Ok(None::<(Adt18, u64)>)
                        }
                    }
                }
            }
        } else {
            let a865 = v3;
            let a866 = 2u64;
            if nat_eq(a865, a866) {
                let m869 = f351(v0.clone(), {
                    let a867 = v1;
                    let a868 = 1u64;
                    nat_add(a867, a868)?
                })?;
                match m869 {
                    None => {
                        Ok(None::<(Adt18, u64)>)
                    }
                    Some(v9) => {
                        if f352({
                            let a870 = v5;
                            let a871 = 34u64;
                            nat_eq(a870, a871)
                        }, {
                            let a873 = f350(v0.clone(), {
                                let (h872, _) = v9.clone();
                                h872
                            })?;
                            let a874 = 58u64;
                            nat_eq(a873, a874)
                        }) {
                            let m878 = f262(v0.clone(), {
                                let a876 = {
                                    let (h875, _) = v9;
                                    h875
                                };
                                let a877 = 1u64;
                                nat_add(a876, a877)?
                            }, v4, 0u64)?;
                            match m878 {
                                None => {
                                    Ok(None::<(Adt18, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f350(v0.clone(), {
                                        let (_, h879) = v10.clone();
                                        h879
                                    })?;
                                    let a880 = v11;
                                    let a881 = 44u64;
                                    if nat_eq(a880, a881) {
                                        let m885 = f262(v0, {
                                            let a883 = {
                                                let (_, h882) = v10;
                                                h882
                                            };
                                            let a884 = 1u64;
                                            nat_add(a883, a884)?
                                        }, v4, 2u64)?;
                                        match m885 {
                                            None => {
                                                Ok(None::<(Adt18, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt18::C10({
                                                    let (_, h886) = v9;
                                                    h886
                                                }, std::rc::Rc::new({
                                                    let (h887, _) = v10;
                                                    h887
                                                }), std::rc::Rc::new({
                                                    let (h888, _) = v12;
                                                    h888
                                                })), {
                                                    let (_, h889) = v12;
                                                    h889
                                                })))
                                            }
                                        }
                                    } else {
                                        let a890 = v11;
                                        let a891 = 125u64;
                                        if nat_eq(a890, a891) {
                                            Ok(Some((Adt18::C10({
                                                let (_, h892) = v9;
                                                h892
                                            }, std::rc::Rc::new({
                                                let (h893, _) = v10;
                                                h893
                                            }), std::rc::Rc::new(Adt18::C8)), {
                                                let a895 = {
                                                    let (_, h894) = v10;
                                                    h894
                                                };
                                                let a896 = 1u64;
                                                nat_add(a895, a896)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt18, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt18, u64)>)
                        }
                    }
                }
            } else {
                let a897 = v5;
                let a898 = 34u64;
                if nat_eq(a897, a898) {
                    let m901 = f351(v0, {
                        let a899 = v1;
                        let a900 = 1u64;
                        nat_add(a899, a900)?
                    })?;
                    match m901 {
                        None => {
                            Ok(None::<(Adt18, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt18::C5({
                                let (_, h902) = v13;
                                h902
                            }), {
                                let (h903, _) = v13;
                                h903
                            })))
                        }
                    }
                } else {
                    if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt18::C0, {
                            let a904 = v1;
                            let a905 = 4u64;
                            nat_add(a904, a905)?
                        })))
                    } else {
                        if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt18::C2(true), {
                                let a906 = v1;
                                let a907 = 4u64;
                                nat_add(a906, a907)?
                            })))
                        } else {
                            if f28(f35(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt18::C2(false), {
                                    let a908 = v1;
                                    let a909 = 5u64;
                                    nat_add(a908, a909)?
                                })))
                            } else {
                                let a910 = v5;
                                let a911 = 91u64;
                                if nat_eq(a910, a911) {
                                    let a914 = f350(v0.clone(), {
                                        let a912 = v1;
                                        let a913 = 1u64;
                                        nat_add(a912, a913)?
                                    })?;
                                    let a915 = 93u64;
                                    if nat_eq(a914, a915) {
                                        Ok(Some((Adt18::C6(std::rc::Rc::new(Adt18::C8)), {
                                            let a916 = v1;
                                            let a917 = 2u64;
                                            nat_add(a916, a917)?
                                        })))
                                    } else {
                                        let m920 = f262(v0, {
                                            let a918 = v1;
                                            let a919 = 1u64;
                                            nat_add(a918, a919)?
                                        }, v4, 1u64)?;
                                        match m920 {
                                            None => {
                                                Ok(None::<(Adt18, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt18::C6(std::rc::Rc::new({
                                                    let (h921, _) = v14;
                                                    h921
                                                })), {
                                                    let (_, h922) = v14;
                                                    h922
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a923 = v5;
                                    let a924 = 123u64;
                                    if nat_eq(a923, a924) {
                                        let a927 = f350(v0.clone(), {
                                            let a925 = v1;
                                            let a926 = 1u64;
                                            nat_add(a925, a926)?
                                        })?;
                                        let a928 = 125u64;
                                        if nat_eq(a927, a928) {
                                            Ok(Some((Adt18::C7(std::rc::Rc::new(Adt18::C8)), {
                                                let a929 = v1;
                                                let a930 = 2u64;
                                                nat_add(a929, a930)?
                                            })))
                                        } else {
                                            let m933 = f262(v0, {
                                                let a931 = v1;
                                                let a932 = 1u64;
                                                nat_add(a931, a932)?
                                            }, v4, 2u64)?;
                                            match m933 {
                                                None => {
                                                    Ok(None::<(Adt18, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt18::C7(std::rc::Rc::new({
                                                        let (h934, _) = v15;
                                                        h934
                                                    })), {
                                                        let (_, h935) = v15;
                                                        h935
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m936 = f353(v0, v1)?;
                                        match m936 {
                                            None => {
                                                Ok(None::<(Adt18, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h937) = v16;
                                                    h937
                                                }, {
                                                    let (h938, _) = v16;
                                                    h938
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

pub fn f263(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h939, _) = v1.clone();
        h939
    };
    let v3: u64 = {
        let (h940, _) = {
            let (_, h941) = v1.clone();
            h941
        };
        h940
    };
    let v4: List<Bytes> = {
        let (_, h942) = {
            let (_, h943) = v1;
            h943
        };
        h942
    };
    let m944 = f174(v0.clone(), v2);
    match m944 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a945 = v5;
            let a946 = 10u8;
            if equal(a945, a946) {
                Ok(Some(({
                    let a947 = v2;
                    let a948 = 1u64;
                    nat_add(a947, a948)?
                }, ({
                    let a949 = v2;
                    let a950 = 1u64;
                    nat_add(a949, a950)?
                }, List::cons(f354(f35(v0, v3, {
                    let a951 = v2;
                    let a952 = v3;
                    nat_sub(a951, a952)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a953 = v2;
                    let a954 = 1u64;
                    nat_add(a953, a954)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f264(v0: Adt13) -> List<Adt13> {
    let m955 = v0;
    match m955 {
        Adt13::C0(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C1(_, _) => {
            List::<Adt13>::nil()
        }
        Adt13::C2(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C3(r956) => {
            let v3 = (*r956).clone();
            f264(v3)
        }
        Adt13::C4(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C5 => {
            List::<Adt13>::nil()
        }
        Adt13::C6(r957, r958) => {
            let v1 = (*r957).clone();
            let v2 = (*r958).clone();
            List::cons(v1, f264(v2))
        }
        Adt13::C7(_, _, _) => {
            List::<Adt13>::nil()
        }
    }
}

pub fn f265(v0: Adt13, v1: Bytes) -> Adt13 {
    let m959 = v0;
    match m959 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r960) => {
            let v10 = (*r960).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r961) => {
            let v5 = (*r961).clone();
            Adt13::C4(std::rc::Rc::new(f265(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r962, r963) => {
            let v11 = (*r962).clone();
            let v12 = (*r963).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r964, r965) => {
            let v3 = (*r964).clone();
            let v4 = (*r965).clone();
            if f28(v2.clone(), f8(Str::lit("authority"))) {
                Adt13::C7(v2, std::rc::Rc::new(f355(v3, v1.clone())), std::rc::Rc::new(f265(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f265(v4, v1)))
            }
        }
    }
}

pub fn f266(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a966 = 0u64;
        let a967 = v1;
        nat_lt(a966, a967)
    } else {
        false
    } {
        let a968 = Bytes::lit(&[0x2d]);
        let a969 = f356(v1)?;
        Ok(append_bytes(a968, a969))
    } else {
        f356(v1)
    }
}

pub fn f267(v0: Bytes) -> R<Bytes> {
    let a975 = {
        let a973 = Bytes::lit(&[0x22]);
        let a974 = {
            let (_, h970) = {
                let (h971, _) = f357(Fn4::F358(v0.clone()), {
                    let a972 = v0;
                    length_bytes(a972)
                }, (0u64, Bytes::lit(&[])))?;
                h971
            };
            h970
        };
        append_bytes(a973, a974)
    };
    let a976 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a975, a976))
}

pub fn f268(v0: List<Bytes>) -> Bytes {
    let a979 = {
        let a977 = Bytes::lit(&[0x5b]);
        let a978 = f95(v0, Bytes::lit(&[0x2c]));
        append_bytes(a977, a978)
    };
    let a980 = Bytes::lit(&[0x5d]);
    append_bytes(a979, a980)
}

pub fn f269(v0: Adt13) -> R<List<Bytes>> {
    let m981 = v0;
    match m981 {
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
        Adt13::C6(r982, r983) => {
            let v7 = (*r982).clone();
            let v8 = (*r983).clone();
            Ok(List::cons(f165(v7)?, f269(v8)?))
        }
        Adt13::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f270(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f359(Fn5::F360, List::<(Bytes, Bytes)>::nil(), f361(v0)?);
    let v2: List<Bytes> = f362(Fn6::F363, List::<Bytes>::nil(), v1)?;
    let a986 = {
        let a984 = Bytes::lit(&[0x7b]);
        let a985 = f95(v2, Bytes::lit(&[0x2c]));
        append_bytes(a984, a985)
    };
    let a987 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a986, a987))
}

pub fn f271(v0: Adt13) -> R<List<(Bytes, Bytes)>> {
    let m988 = v0;
    match m988 {
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
        Adt13::C7(v9, r989, r990) => {
            let v10 = (*r989).clone();
            let v11 = (*r990).clone();
            Ok(List::cons((v9, f165(v10)?), f271(v11)?))
        }
    }
}

pub fn f272() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f273(v0: Adt12) -> Bytes {
    let m991 = v0;
    match m991 {
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

pub fn f274(v0: Adt12, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f364(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f364(f8(Str::lit("id")), f58(v0.clone(), f50(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f364(f8(Str::lit("name")), f365(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f364(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f364(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f364(f8(Str::lit("statement")), f366(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f364(f8(Str::lit("media_type")), f367(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f364(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f364(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f364(f8(Str::lit("url")), f168(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f275(v0: Bytes) -> R<u64> {
    let (h992, _) = f286(Fn3::F368(v0.clone()), {
        let a993 = v0.clone();
        length_bytes(a993)
    }, {
        let a994 = v0;
        length_bytes(a994)
    })?;
    Ok(h992)
}

pub fn f276(v0: Bytes, v1: u64) -> R<u64> {
    let (h995, _) = f286(Fn3::F369(v1, v0), v1, 0u64)?;
    Ok(h995)
}

pub fn f277(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m996 = v0;
    match m996.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m997 = f79(v1, f8(Str::lit("std")));
            match m997 {
                None => {
                    f277(v2)
                }
                Some(v3) => {
                    if f185(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f47(v3.clone(), f8(Str::lit("row"))), f47(v3, f8(Str::lit("authority")))), f277(v2))
                    } else {
                        f277(v2)
                    }
                }
            }
        }
    }
}

pub fn f278(v0: Adt14, v1: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m998 = v1;
    match m998.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f47(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f47(v2, f8(Str::lit("object")));
            if if f41(v4.clone(), match v0.clone() {
                Adt14::C0(h999, _, _) => {
                    h999
                }
            }) {
                f41(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h1000, _) => {
                        h1000
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f278(v0, v3))
            } else {
                if if f41(v5.clone(), match v0.clone() {
                    Adt14::C0(h1001, _, _) => {
                        h1001
                    }
                }) {
                    f41(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1002, _) => {
                            h1002
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f278(v0, v3))
                } else {
                    f278(v0, v3)
                }
            }
        }
    }
}

pub fn f279(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt19>> {
    let m1003 = v0;
    match m1003.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v4, v5)) => {
            if if f370(v4.clone(), v2.clone()) {
                true
            } else {
                f370(v4.clone(), v3.clone())
            } {
                f279(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt19::C0({
                    let (h1004, _) = v4.clone();
                    h1004
                }, {
                    let a1007 = {
                        let a1006 = f125(f371({
                            let (h1005, _) = v4.clone();
                            h1005
                        }, v1.clone()))?;
                        length_list(a1006)
                    };
                    let a1008 = 1u64;
                    if nat_eq(a1007, a1008) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1009) = v4.clone();
                    h1009
                }), f279(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f280(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1010 = v1;
    match m1010.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41({
                let (h1011, _) = v2.clone();
                h1011
            }, v0.clone()) {
                List::cons({
                    let (_, h1012) = v2;
                    h1012
                }, f280(v0, v3))
            } else {
                if f41({
                    let (_, h1013) = v2;
                    h1013
                }, v0.clone()) {
                    List::cons({
                        let (h1014, _) = v2;
                        h1014
                    }, f280(v0, v3))
                } else {
                    f280(v0, v3)
                }
            }
        }
    }
}

pub fn f281(v0: Adt14, v1: Adt13, v2: Bytes) -> bool {
    if f41(v2.clone(), match v0.clone() {
        Adt14::C0(h1015, _, _) => {
            h1015
        }
    }) {
        true
    } else {
        if f41(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1016, _) => {
                h1016
            }
        }) {
            true
        } else {
            if f41(v2, match v0.clone() {
                Adt14::C0(_, _, h1017) => {
                    h1017
                }
            }) {
                true
            } else {
                if f185(v1.clone(), f8(Str::lit("scheme"))) {
                    f41(f47(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1018, _) => {
                            h1018
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f282(v0: Adt13) -> bool {
    if f185(v0.clone(), f8(Str::lit("domain"))) {
        if f185(v0.clone(), f8(Str::lit("row"))) {
            if f41(f47(v0.clone(), f8(Str::lit("relation"))), f372()) {
                if f373(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f373(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f373(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f373(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f283(v0: Adt13) -> bool {
    if f185(v0.clone(), f8(Str::lit("subject"))) {
        if f185(v0.clone(), f8(Str::lit("object"))) {
            if f185(v0.clone(), f8(Str::lit("evidence"))) {
                f41(f47(v0, f8(Str::lit("relation"))), f374())
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

pub fn f284(v0: Bytes) -> bool {
    f190(v0, f8(Str::lit(".lex.tex")))
}

pub fn f285(v0: Bytes) -> R<bool> {
    let v1: Bytes = f173(v0)?;
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
        let a1020 = {
            let a1019 = v1;
            length_bytes(a1019)
        };
        let a1021 = v2;
        nat_sub(a1020, a1021)
    });
    let v5: Bytes = if f72(v3.clone(), f8(Str::lit("("))) {
        let m1022 = f194(v3.clone(), f8(Str::lit(")")))?;
        match m1022 {
            None => {
                v3
            }
            Some(v4) => {
                f35(v3.clone(), {
                    let a1023 = v4;
                    let a1024 = 1u64;
                    nat_add(a1023, a1024)?
                }, {
                    let a1028 = {
                        let a1025 = v3;
                        length_bytes(a1025)
                    };
                    let a1029 = {
                        let a1026 = v4;
                        let a1027 = 1u64;
                        nat_add(a1026, a1027)?
                    };
                    nat_sub(a1028, a1029)
                })
            }
        }
    } else {
        v3
    };
    let a1030 = 0u64;
    let a1031 = v2;
    if nat_lt(a1030, a1031) {
        f375(v5)
    } else {
        Ok(false)
    }
}

pub fn f286(v0: Fn3, p1036: u64, p1037: u64) -> R<(u64, bool)> {
    let m1035 = (p1036, p1037);
    {
        let mut m1035 = m1035;
        loop {
            let step = {
                let (v1, v2) = m1035;
                let m1032 = v1;
                if m1032 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1032 - 1;
                    let m1034 = {
                        let c1033: Fn3 = v0.clone();
                        c1033.apply(v2)?
                    };
                    match m1034 {
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
                Ok(next) => { m1035 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f288(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1040 = {
            let a1038 = v4;
            let a1039 = v1;
            nat_add(a1038, a1039)?
        };
        let a1041 = v3;
        if nat_lt(a1040, a1041) {
            Ok(Some({
                let a1042 = v4;
                let a1043 = 1u64;
                nat_add(a1042, a1043)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f288(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1048 = {
        let a1045 = v0;
        let a1046 = v2;
        let a1047 = {
            let a1044 = v1.clone();
            length_bytes(a1044)
        };
        slice_bytes(a1045, a1046, a1047)
    };
    match m1048 {
        None => {
            false
        }
        Some(v3) => {
            let a1049 = v3;
            let a1050 = v1;
            equal(a1049, a1050)
        }
    }
}

pub fn f289(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1051 = v0;
    match m1051.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f289(v3, {
                let a1052 = v1;
                let a1053 = 1u64;
                nat_add(a1052, a1053)?
            })?))
        }
    }
}

pub fn f290(v0: Adt17, v1: List<u64>, v2: Adt21) -> bool {
    let m1054 = v2;
    match m1054 {
        Adt21::C0(v3, v4) => {
            if f296(v3, match v0.clone() {
                Adt17::C0(h1055, _, _, _) => {
                    h1055
                }
            }) {
                f376(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f296(v5, match v0 {
                Adt17::C0(_, _, h1056, _) => {
                    h1056
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f292(v7, v1) {
                f296(v8, match v0 {
                    Adt17::C0(_, _, _, h1057) => {
                        h1057
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f291(v0: Adt15, v1: Adt13) -> R<Adt21> {
    let m1058 = f79(v1.clone(), f8(Str::lit("std")));
    match m1058 {
        None => {
            let m1063 = f78(f79(v1.clone(), f8(Str::lit("req"))));
            match m1063 {
                None => {
                    let m1065 = f79(v1, f8(Str::lit("der")));
                    match m1065 {
                        None => {
                            Ok(f377(v0))
                        }
                        Some(v5) => {
                            let m1066 = f378(f47(v5.clone(), f8(Str::lit("rule"))));
                            match m1066 {
                                None => {
                                    Ok(f377(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f202(f122(f163(f79(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1067) => {
                                            h1067
                                        }
                                    })?, f294(f47(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1068, _) => {
                                            h1068
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f294(v4, match v0 {
                        Adt15::C0(_, _, h1064, _, _) => {
                            h1064
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1059 = f78(f79(v2.clone(), f8(Str::lit("row"))));
            match m1059 {
                None => {
                    Ok(Adt21::C0(f294(f47(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1060, _, _, _, _) => {
                            h1060
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f294(f47(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1061, _, _, _, _) => {
                            h1061
                        }
                    })?, Some(f294(v3, match v0 {
                        Adt15::C0(_, h1062, _, _, _) => {
                            h1062
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f292(p1071: List<u64>, v1: List<u64>) -> bool {
    let m1070 = p1071;
    {
        let mut m1070 = m1070;
        loop {
            let step = {
                let v0 = m1070;
                let m1069 = v0;
                match m1069.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f296(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f293(p1076: List<u64>) -> bool {
    let m1075 = p1076;
    {
        let mut m1075 = m1075;
        loop {
            let step = {
                let v0 = m1075;
                let m1072 = v0;
                match m1072.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1073 = v1;
                        let a1074 = 7u64;
                        if nat_le(a1073, a1074) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1075 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1077 = f379(v0, v1.clone(), 0u64)?;
    match m1077 {
        None => {
            let a1078 = v1;
            Ok(length_list(a1078))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f295(v0: Adt13) -> u64 {
    let m1079 = f380(f79(v0, f8(Str::lit("stratum"))));
    match m1079 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f296(v0: u64, p1084: List<u64>) -> bool {
    let m1083 = p1084;
    {
        let mut m1083 = m1083;
        loop {
            let step = {
                let v1 = m1083;
                let m1080 = v1;
                match m1080.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1081 = v2;
                        let a1082 = v0;
                        if nat_eq(a1081, a1082) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
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

pub fn f297(v0: Bytes, p1142: u64, p1143: u64, p1144: Adt23) -> R<Adt23> {
    let m1141 = (p1142, (p1143, p1144));
    {
        let mut m1141 = m1141;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1141;
                let m1085 = v2;
                if m1085 == 0 {
                    Err::<(u64, (u64, Adt23)), R<Adt23>>(Ok(v3))
                } else {
                    let v4 = m1085 - 1;
                    let v5: u64 = f381(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt23::C0(_, h1086, _, _, _) => {
                            h1086
                        }
                    } {
                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                            let a1087 = v1;
                            let a1088 = 1u64;
                            nat_add(a1087, a1088)?
                        }, (v4, Adt23::C0(true, false, match v3.clone() {
                            Adt23::C0(_, _, h1089, _, _) => {
                                h1089
                            }
                        }, match v3.clone() {
                            Adt23::C0(_, _, _, h1090, _) => {
                                h1090
                            }
                        }, match v3 {
                            Adt23::C0(_, _, _, _, h1091) => {
                                h1091
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt23::C0(h1092, _, _, _, _) => {
                                h1092
                            }
                        } {
                            let a1093 = v5;
                            let a1094 = 34u64;
                            if nat_eq(a1093, a1094) {
                                let a1097 = f381(v0.clone(), {
                                    let a1095 = v1;
                                    let a1096 = 1u64;
                                    nat_add(a1095, a1096)?
                                })?;
                                let a1098 = 34u64;
                                if nat_eq(a1097, a1098) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1099 = v1;
                                        let a1100 = 1u64;
                                        nat_add(a1099, a1100)?
                                    }, (v4, Adt23::C0(true, true, f382(match v3.clone() {
                                        Adt23::C0(_, _, h1101, _, _) => {
                                            h1101
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt23::C0(_, _, _, h1102, _) => {
                                            h1102
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1103) => {
                                            h1103
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1104 = v1;
                                        let a1105 = 1u64;
                                        nat_add(a1104, a1105)?
                                    }, (v4, Adt23::C0(false, false, match v3.clone() {
                                        Adt23::C0(_, _, h1106, _, _) => {
                                            h1106
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1107, _) => {
                                            h1107
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1108) => {
                                            h1108
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1109 = v1;
                                    let a1110 = 1u64;
                                    nat_add(a1109, a1110)?
                                }, (v4, Adt23::C0(true, false, f382(match v3.clone() {
                                    Adt23::C0(_, _, h1111, _, _) => {
                                        h1111
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt23::C0(_, _, _, h1112, _) => {
                                        h1112
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1113) => {
                                        h1113
                                    }
                                }))))
                            }
                        } else {
                            let a1114 = v5;
                            let a1115 = 34u64;
                            if nat_eq(a1114, a1115) {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1116 = v1;
                                    let a1117 = 1u64;
                                    nat_add(a1116, a1117)?
                                }, (v4, Adt23::C0(true, false, match v3.clone() {
                                    Adt23::C0(_, _, h1118, _, _) => {
                                        h1118
                                    }
                                }, match v3.clone() {
                                    Adt23::C0(_, _, _, h1119, _) => {
                                        h1119
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1120) => {
                                        h1120
                                    }
                                }))))
                            } else {
                                let a1121 = v5;
                                let a1122 = 44u64;
                                if nat_eq(a1121, a1122) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1123 = v1;
                                        let a1124 = 1u64;
                                        nat_add(a1123, a1124)?
                                    }, (v4, Adt23::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt23::C0(_, _, h1125, _, _) => {
                                            h1125
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1126, _) => {
                                            h1126
                                        }
                                    }), match v3 {
                                        Adt23::C0(_, _, _, _, h1127) => {
                                            h1127
                                        }
                                    }))))
                                } else {
                                    let a1128 = v5;
                                    let a1129 = 13u64;
                                    if nat_eq(a1128, a1129) {
                                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                            let a1130 = v1;
                                            let a1131 = 1u64;
                                            nat_add(a1130, a1131)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1132 = v5;
                                        let a1133 = 10u64;
                                        if nat_eq(a1132, a1133) {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1134 = v1;
                                                let a1135 = 1u64;
                                                nat_add(a1134, a1135)?
                                            }, (v4, f298(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1136 = v1;
                                                let a1137 = 1u64;
                                                nat_add(a1136, a1137)?
                                            }, (v4, Adt23::C0(false, false, f382(match v3.clone() {
                                                Adt23::C0(_, _, h1138, _, _) => {
                                                    h1138
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt23::C0(_, _, _, h1139, _) => {
                                                    h1139
                                                }
                                            }, match v3 {
                                                Adt23::C0(_, _, _, _, h1140) => {
                                                    h1140
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
                Ok(next) => { m1141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: Adt23) -> Adt23 {
    Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f383(List::cons(match v0.clone() {
        Adt23::C0(_, _, h1145, _, _) => {
            h1145
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h1146, _) => {
            h1146
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt23::C0(_, _, _, _, h1147) => {
            h1147
        }
    }))
}

pub fn f299(p1150: List<List<Bytes>>, p1151: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1149 = (p1150, p1151);
    {
        let mut m1149 = m1149;
        loop {
            let step = {
                let (v0, v1) = m1149;
                let m1148 = v0;
                match m1148.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
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

pub fn f300(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f301(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1152 = v0.clone();
    match m1152.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1153 = v2;
            let a1154 = v1.clone();
            if equal(a1153, a1154) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f302(v0: List<(Bytes, ())>) -> List<Bytes> {
    f208(f384(Fn7::F385, List::<Bytes>::nil(), v0))
}

pub fn f303(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f386(f387(Fn8::F388, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1159 = {
        let (h1155, _) = f389(Fn9::F390, {
            let a1157 = {
                let a1156 = v0;
                length_list(a1156)
            };
            let a1158 = 1u64;
            nat_add(a1157, a1158)?
        }, v1)?;
        h1155
    };
    match m1159.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f304(v0: List<Bytes>) -> List<(Bytes, ())> {
    f391(f392(Fn10::F393, List::<(Bytes, ())>::nil(), v0))
}

pub fn f305(v0: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1160 = f33(v1, 34u8)?;
    match m1160.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1161 = v3;
            match m1161.uncons() {
                None => {
                    f394(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f306(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f395(v0.clone(), 0u64, {
        let a1162 = v0;
        length_bytes(a1162)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f307(v0: Adt13, v1: Bytes) -> Bytes {
    if f28(f396(f79(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f308(v0: Adt1, v1: Bytes) -> R<Adt18> {
    let m1163 = f241(v0, v1)?;
    match m1163 {
        None => {
            Ok(Adt18::C8)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f309(v0: Bytes) -> R<Bytes> {
    let a1164 = f8(Str::lit("Catalog."));
    let a1165 = f328(v0)?;
    Ok(append_bytes(a1164, a1165))
}

pub fn f310(v0: Adt18) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f328(f245(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f311(v0: List<Bytes>) -> List<Bytes> {
    let m1166 = v0;
    match m1166.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f190(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f211(f228(), v1), f311(v2))
            } else {
                f311(v2)
            }
        }
    }
}

pub fn f312(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1167 = v1;
    match m1167.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f397(v0.clone(), v2)?;
            let (h1168, _) = v4.clone();
            if h1168 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f312(v0, v3)?;
                let (h1169, _) = v5.clone();
                if h1169 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1172 = {
                            let (_, h1170) = v4;
                            h1170
                        };
                        let a1173 = {
                            let (_, h1171) = v5;
                            h1171
                        };
                        append_bytes(a1172, a1173)
                    }))
                }
            }
        }
    }
}

pub fn f313(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m1174 = v1;
    match m1174.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f185(v2.clone(), f8(Str::lit("subject"))) {
                if f186(v0.clone(), v2.clone()) {
                    f313(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f47(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f313(v0, v3))
                }
            } else {
                if f319(v0.clone(), v2.clone()) {
                    f313(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f320(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f313(v0, v3))
                }
            }
        }
    }
}

pub fn f314(v0: Adt1) -> R<Bytes> {
    let a1176 = {
        let a1175 = f101(f102(f207(f220(f206(f53(v0.clone(), f179()))?)), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(f82(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1175)
    };
    let a1177 = 0u64;
    if nat_eq(a1176, a1177) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f315(v0: List<Adt13>) -> R<List<Bytes>> {
    let m1178 = v0;
    match m1178.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f95(f122(f163(f79(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f47(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f47(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f398(f81(f47(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f315(v2)?))
        }
    }
}

pub fn f316(v0: Bytes, v1: List<Adt19>) -> List<Bytes> {
    let m1179 = v1;
    match m1179.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(match v2.clone() {
                Adt19::C0(h1180, _, _) => {
                    h1180
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt19::C0(_, h1181, _) => {
                        h1181
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt19::C0(_, _, h1182) => {
                        h1182
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f316(v0, v3))
            } else {
                f316(v0, v3)
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: List<Adt13>) -> List<Bytes> {
    let m1183 = v1;
    match m1183.uncons() {
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

pub fn f318(p1188: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1187 = p1188;
    {
        let mut m1187 = m1187;
        loop {
            let step = {
                let v0 = m1187;
                let m1184 = v0;
                match m1184.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1185 = v2;
                        match m1185.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f28(v4, v1.clone()) {
                                    let m1186 = v5;
                                    match m1186.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f399(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: Adt14, v1: Adt13) -> bool {
    if f41(f47(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1189, _, _) => {
            h1189
        }
    }) {
        f41(f320(v1), match v0 {
            Adt14::C0(_, h1190, _) => {
                h1190
            }
        })
    } else {
        false
    }
}

pub fn f320(v0: Adt13) -> Bytes {
    let m1191 = f79(v0, f8(Str::lit("evidence")));
    match m1191 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f47(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f321(v0: Adt13) -> Bytes {
    if f185(v0.clone(), f8(Str::lit("req"))) {
        let a1192 = f8(Str::lit("req "));
        let a1193 = f47(v0, f8(Str::lit("req")));
        append_bytes(a1192, a1193)
    } else {
        let m1194 = f79(v0.clone(), f8(Str::lit("std")));
        match m1194 {
            None => {
                let m1197 = f79(v0, f8(Str::lit("der")));
                match m1197 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f47(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f95(f122(f163(f79(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f47(v1.clone(), f8(Str::lit("authority"))), List::cons(if f185(v1.clone(), f8(Str::lit("row"))) {
                    let a1195 = f8(Str::lit(" row "));
                    let a1196 = f47(v1, f8(Str::lit("row")));
                    append_bytes(a1195, a1196)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f322(v0: Option<List<Adt13>>) -> bool {
    let m1198 = v0;
    match m1198 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f323(v0: Option<List<Adt13>>) -> List<Adt13> {
    let m1199 = v0;
    match m1199 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f324(v0: Bytes, v1: Bytes) -> bool {
    let a1200 = v0;
    let a1201 = f336();
    if equal(a1200, a1201) {
        let a1205 = {
            let a1203 = {
                let a1202 = v1;
                length_bytes(a1202)
            };
            let a1204 = 0u64;
            nat_eq(a1203, a1204)
        };
        bool_not(a1205)
    } else {
        false
    }
}

pub fn f325(v0: Bytes) -> Bytes {
    f7(List::cons(f124(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f326(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1207 = v0;
    let a1208 = {
        let a1206 = Str::lit("redistributable");
        utf8_encode(a1206)
    };
    if equal(a1207, a1208) {
        f400(v1)
    } else {
        Ok(false)
    }
}

pub fn f327(v0: Adt18) -> bool {
    if f401(v0.clone(), f8(Str::lit("name"))) {
        if f402(v0.clone(), f8(Str::lit("schema"))) {
            let m1209 = f331(f246(f247(f242(v0, f8(Str::lit("citation"))))));
            match m1209 {
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

pub fn f328(v0: Bytes) -> R<Bytes> {
    f403(v0.clone(), 0u64, {
        let a1210 = v0;
        length_bytes(a1210)
    }, true, Bytes::lit(&[]))
}

pub fn f329(v0: Adt13) -> Bytes {
    if f185(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f185(v0.clone(), f8(Str::lit("id"))) {
            if f185(v0.clone(), f8(Str::lit("statement"))) {
                if f185(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f348(v0, f8(Str::lit("evidence")))
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

pub fn f330(v0: List<Bytes>) -> Adt13 {
    let m1211 = v0;
    match m1211.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(f330(v2)))
        }
    }
}

pub fn f331(v0: List<Bytes>) -> Option<Adt13> {
    let m1212 = v0;
    match m1212.uncons() {
        None => {
            None::<Adt13>
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("req"))) {
                let m1213 = v2;
                match m1213.uncons() {
                    None => {
                        None::<Adt13>
                    }
                    Some((v3, _)) => {
                        Some(Adt13::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5)))
                    }
                }
            } else {
                if f28(v1.clone(), f8(Str::lit("std"))) {
                    let m1214 = v2;
                    match m1214.uncons() {
                        None => {
                            None::<Adt13>
                        }
                        Some((v5, v6)) => {
                            Some(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v5)), std::rc::Rc::new({
                                let m1215 = v6;
                                match m1215.uncons() {
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
                        let m1216 = v2;
                        match m1216.uncons() {
                            None => {
                                None::<Adt13>
                            }
                            Some((v9, v10)) => {
                                let m1217 = v10;
                                match m1217.uncons() {
                                    None => {
                                        None::<Adt13>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt13::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt13::C2(v11)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f330(v12)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt13::C2(v9)), std::rc::Rc::new(Adt13::C5))))))))), std::rc::Rc::new(Adt13::C5)))
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

pub fn f332(v0: Adt1, v1: Bytes, v2: Adt18, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1218 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1218.uncons() {
        None => {
            let m1219 = f331(v4);
            match m1219 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1220 = f8(Str::lit("Catalog."));
                        let a1221 = f328(v3.clone())?;
                        append_bytes(a1220, a1221)
                    };
                    let m1222 = f404(v2, v9, f328(v3.clone())?)?;
                    match m1222 {
                        Ok(v11) => {
                            f345(v0, v3.clone(), v8, v11, f8(Str::lit("Encoding entered through propose-encoding.")), List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f328(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::<(Bytes, Bytes)>::nil()))
                        }
                        Err(v10) => {
                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v10, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f333(v0: Adt18) -> R<Bytes> {
    let m1223 = v0;
    match m1223 {
        Adt18::C0 => {
            Ok(Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
        }
        Adt18::C1(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt18::C2(v3) => {
            if v3 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt18::C3(v4, v5) => {
            f266(v4, v5)
        }
        Adt18::C4(v6, v7, v8) => {
            f405(v6, v7, v8)
        }
        Adt18::C5(v9) => {
            f406(v9)
        }
        Adt18::C6(r1224) => {
            let v10 = (*r1224).clone();
            Ok(f268(f407(v10)?))
        }
        Adt18::C7(r1225) => {
            let v11 = (*r1225).clone();
            f270(f408(v11)?)
        }
        Adt18::C8 => {
            Ok(Bytes::lit(&[]))
        }
        Adt18::C9(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt18::C10(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f334(v0: Option<Adt18>) -> Option<Bytes> {
    let m1226 = v0;
    match m1226 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1227 = v1;
            match m1227 {
                Adt18::C0 => {
                    None::<Bytes>
                }
                Adt18::C1(_, _) => {
                    None::<Bytes>
                }
                Adt18::C2(_) => {
                    None::<Bytes>
                }
                Adt18::C3(_, _) => {
                    None::<Bytes>
                }
                Adt18::C4(_, _, _) => {
                    None::<Bytes>
                }
                Adt18::C5(v2) => {
                    Some(v2)
                }
                Adt18::C6(_) => {
                    None::<Bytes>
                }
                Adt18::C7(_) => {
                    None::<Bytes>
                }
                Adt18::C8 => {
                    None::<Bytes>
                }
                Adt18::C9(_, _) => {
                    None::<Bytes>
                }
                Adt18::C10(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f335(v0: Adt18) -> List<Adt18> {
    let m1228 = v0;
    match m1228 {
        Adt18::C0 => {
            List::<Adt18>::nil()
        }
        Adt18::C1(_, _) => {
            List::<Adt18>::nil()
        }
        Adt18::C2(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C3(_, _) => {
            List::<Adt18>::nil()
        }
        Adt18::C4(_, _, _) => {
            List::<Adt18>::nil()
        }
        Adt18::C5(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C6(r1229) => {
            let v3 = (*r1229).clone();
            f335(v3)
        }
        Adt18::C7(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C8 => {
            List::<Adt18>::nil()
        }
        Adt18::C9(r1230, r1231) => {
            let v1 = (*r1230).clone();
            let v2 = (*r1231).clone();
            List::cons(v1, f335(v2))
        }
        Adt18::C10(_, _, _) => {
            List::<Adt18>::nil()
        }
    }
}

pub fn f336() -> Bytes {
    let a1232 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1232)
}

pub fn f337(v0: Adt13, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt13> = f79(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f253(Str::lit("id"), f47(v0.clone(), f8(Str::lit("id"))))?, List::cons(f253(Str::lit("name"), f47(v0.clone(), f8(Str::lit("name"))))?, List::cons(f253(Str::lit("issuer"), f47(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f253(Str::lit("edition"), f47(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f253(Str::lit("canonical_id"), f47(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f253(Str::lit("source_role"), f47(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f253(Str::lit("retrieval_date"), f47(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f253(Str::lit("url"), f396(v2.clone(), f8(Str::lit("url"))))?, List::cons(f253(Str::lit("media_type"), f396(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f253(Str::lit("license"), f396(v2.clone(), f8(Str::lit("license"))))?, List::cons(f253(Str::lit("redistribution"), f396(v2, f8(Str::lit("redistribution"))))?, List::cons(f253(Str::lit("statement"), f47(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1233 = v1;
        match m1233 {
            None => {
                f7(List::cons(f253(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f253(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f253(Str::lit("artifact"), {
                    let (h1234, _) = v3;
                    h1234
                })?, List::cons(f253(Str::lit("checksum"), {
                    let (_, h1235) = v3;
                    h1235
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f409(f122(f163(f79(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f338(v0: Adt24) -> Bytes {
    let a1256 = f410(match v0.clone() {
        Adt24::C0(h1236, _, _, _, _, _, _, _) => {
            h1236
        }
    });
    let a1257 = {
        let a1254 = f410(match v0.clone() {
            Adt24::C0(_, h1237, _, _, _, _, _, _) => {
                h1237
            }
        });
        let a1255 = {
            let a1252 = f410(match v0.clone() {
                Adt24::C0(_, _, h1238, _, _, _, _, _) => {
                    h1238
                }
            });
            let a1253 = {
                let a1250 = f410(match v0.clone() {
                    Adt24::C0(_, _, _, h1239, _, _, _, _) => {
                        h1239
                    }
                });
                let a1251 = {
                    let a1248 = f410(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1240, _, _, _) => {
                            h1240
                        }
                    });
                    let a1249 = {
                        let a1246 = f410(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1241, _, _) => {
                                h1241
                            }
                        });
                        let a1247 = {
                            let a1244 = f410(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1242, _) => {
                                    h1242
                                }
                            });
                            let a1245 = f410(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1243) => {
                                    h1243
                                }
                            });
                            append_bytes(a1244, a1245)
                        };
                        append_bytes(a1246, a1247)
                    };
                    append_bytes(a1248, a1249)
                };
                append_bytes(a1250, a1251)
            };
            append_bytes(a1252, a1253)
        };
        append_bytes(a1254, a1255)
    };
    append_bytes(a1256, a1257)
}

pub fn f339(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1258 = v0.clone();
        length_bytes(a1258)
    };
    let v2: u64 = {
        let a1264 = {
            let a1261 = {
                let a1259 = v1;
                let a1260 = 72u64;
                nat_add(a1259, a1260)?
            };
            let a1262 = 64u64;
            let a1263 = 0u64;
            nat_quot(a1261, a1262, a1263)
        };
        let a1265 = 64u64;
        nat_mul(a1264, a1265)?
    };
    let (_, h1266) = f411(Fn11::F412(v0, v1, v2), {
        let a1267 = v2;
        let a1268 = 64u64;
        let a1269 = 0u64;
        nat_quot(a1267, a1268, a1269)
    }, (0u64, f413()))?;
    Ok(h1266)
}

pub fn f340(p1272: List<List<Bytes>>, v1: Bytes, p1273: List<Bytes>) -> R<List<Bytes>> {
    let m1271 = (p1272, p1273);
    {
        let mut m1271 = m1271;
        loop {
            let step = {
                let (v0, v2) = m1271;
                let m1270 = v0;
                match m1270.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f414(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f342(f415(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1271 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: Bytes) -> R<Bytes> {
    Ok(f35(v0.clone(), 0u64, f275(v0)?))
}

pub fn f342(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1274 = v0;
    match m1274.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1275 = v3.clone();
            match m1275.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1278 = v2;
                    let a1279 = {
                        let a1276 = v1.clone();
                        let a1277 = f342(v3, v1);
                        append_bytes(a1276, a1277)
                    };
                    append_bytes(a1278, a1279)
                }
            }
        }
    }
}

pub fn f343(v0: Adt13, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1280 = f79(v0, v1);
    match m1280 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v3) => {
            f163(f79(v3, v2))
        }
    }
}

pub fn f344(v0: Bytes) -> bool {
    if f28(v0.clone(), f8(Str::lit("std"))) {
        true
    } else {
        if f28(v0.clone(), f8(Str::lit("req"))) {
            true
        } else {
            f28(v0, f8(Str::lit("der")))
        }
    }
}

pub fn f345(v0: Adt1, v1: Bytes, v2: Adt13, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt11> {
    let v6: Bytes = {
        let a1281 = f8(Str::lit("Catalog."));
        let a1282 = f328(v1.clone())?;
        append_bytes(a1281, a1282)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f328(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt11::C3(f416(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f417(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f417(f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt13::C2(v7.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt13::C2(v4)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt13::C1(false, 0u64)), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a1286 = f418(f53(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f35(v7.clone(), 4u64, {
            let a1284 = {
                let a1283 = v7;
                length_bytes(a1283)
            };
            let a1285 = 4u64;
            nat_sub(a1284, a1285)
        }))?;
        let a1287 = f8(Str::lit("\u{a}"));
        append_bytes(a1286, a1287)
    }), List::cons(f240(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f346(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f328(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f347(v0: Bytes) -> R<Bytes> {
    if f72(v0.clone(), f8(Str::lit("-- "))) {
        let m1288 = f194(v0.clone(), f8(Str::lit("\u{a}")))?;
        match m1288 {
            None => {
                Ok(v0)
            }
            Some(v1) => {
                Ok(f35(v0.clone(), {
                    let a1289 = v1;
                    let a1290 = 1u64;
                    nat_add(a1289, a1290)?
                }, {
                    let a1294 = {
                        let a1291 = v0;
                        length_bytes(a1291)
                    };
                    let a1295 = {
                        let a1292 = v1;
                        let a1293 = 1u64;
                        nat_add(a1292, a1293)?
                    };
                    nat_sub(a1294, a1295)
                }))
            }
        }
    } else {
        Ok(v0)
    }
}

pub fn f348(v0: Adt13, v1: Bytes) -> bool {
    let m1296 = f79(v0, v1);
    match m1296 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f349(v0: Bytes, p1324: u64, p1325: u64, p1326: bool, p1327: bool, p1328: u64, p1329: Bytes) -> R<Bytes> {
    let m1323 = (p1324, (p1325, (p1326, (p1327, (p1328, p1329)))));
    {
        let mut m1323 = m1323;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m1323;
                let m1297 = v2;
                if m1297 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f419(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m1297 - 1;
                    let m1300 = {
                        let a1298 = v0.clone();
                        let a1299 = v1;
                        index_bytes(a1298, a1299)
                    };
                    match m1300 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f419(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f62(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a1301 = v1;
                                    let a1302 = 1u64;
                                    nat_add(a1301, a1302)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a1303 = v9;
                                    let a1304 = 34u64;
                                    !nat_eq(a1303, a1304)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1305 = v9;
                                    let a1306 = 92u64;
                                    nat_eq(a1305, a1306)
                                }, (v5, v6))))))
                            } else {
                                let a1307 = v9;
                                let a1308 = 32u64;
                                if if if if nat_eq(a1307, a1308) {
                                    true
                                } else {
                                    let a1309 = v9;
                                    let a1310 = 9u64;
                                    nat_eq(a1309, a1310)
                                } {
                                    true
                                } else {
                                    let a1311 = v9;
                                    let a1312 = 10u64;
                                    nat_eq(a1311, a1312)
                                } {
                                    true
                                } else {
                                    let a1313 = v9;
                                    let a1314 = 13u64;
                                    nat_eq(a1313, a1314)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1315 = v1;
                                        let a1316 = 1u64;
                                        nat_add(a1315, a1316)?
                                    }, (v7, (false, (false, ({
                                        let a1317 = v1;
                                        let a1318 = 1u64;
                                        nat_add(a1317, a1318)?
                                    }, f419(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1319 = v1;
                                        let a1320 = 1u64;
                                        nat_add(a1319, a1320)?
                                    }, (v7, ({
                                        let a1321 = v9;
                                        let a1322 = 34u64;
                                        nat_eq(a1321, a1322)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
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

pub fn f350(v0: Bytes, v1: u64) -> R<u64> {
    let m1332 = {
        let a1330 = v0;
        let a1331 = v1;
        index_bytes(a1330, a1331)
    };
    match m1332 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1377 = {
                let a1353 = {
                    let a1341 = {
                        let a1335 = {
                            let a1333 = v3;
                            let a1334 = 128u8;
                            fixed_u8::bit_and(a1333, a1334)
                        };
                        let a1336 = 128u8;
                        if equal(a1335, a1336) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1342 = {
                        let a1339 = {
                            let a1337 = v3;
                            let a1338 = 64u8;
                            fixed_u8::bit_and(a1337, a1338)
                        };
                        let a1340 = 64u8;
                        if equal(a1339, a1340) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1341, a1342)?
                };
                let a1354 = {
                    let a1351 = {
                        let a1345 = {
                            let a1343 = v3;
                            let a1344 = 32u8;
                            fixed_u8::bit_and(a1343, a1344)
                        };
                        let a1346 = 32u8;
                        if equal(a1345, a1346) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1352 = {
                        let a1349 = {
                            let a1347 = v3;
                            let a1348 = 16u8;
                            fixed_u8::bit_and(a1347, a1348)
                        };
                        let a1350 = 16u8;
                        if equal(a1349, a1350) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1351, a1352)?
                };
                nat_add(a1353, a1354)?
            };
            let a1378 = {
                let a1375 = {
                    let a1363 = {
                        let a1357 = {
                            let a1355 = v3;
                            let a1356 = 8u8;
                            fixed_u8::bit_and(a1355, a1356)
                        };
                        let a1358 = 8u8;
                        if equal(a1357, a1358) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1364 = {
                        let a1361 = {
                            let a1359 = v3;
                            let a1360 = 4u8;
                            fixed_u8::bit_and(a1359, a1360)
                        };
                        let a1362 = 4u8;
                        if equal(a1361, a1362) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1363, a1364)?
                };
                let a1376 = {
                    let a1373 = {
                        let a1367 = {
                            let a1365 = v3;
                            let a1366 = 2u8;
                            fixed_u8::bit_and(a1365, a1366)
                        };
                        let a1368 = 2u8;
                        if equal(a1367, a1368) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1374 = {
                        let a1371 = {
                            let a1369 = v3;
                            let a1370 = 1u8;
                            fixed_u8::bit_and(a1369, a1370)
                        };
                        let a1372 = 1u8;
                        if equal(a1371, a1372) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1373, a1374)?
                };
                nat_add(a1375, a1376)?
            };
            nat_add(a1377, a1378)
        }
    }
}

pub fn f351(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f420(v0.clone(), v1, {
        let a1380 = {
            let a1379 = v0.clone();
            length_bytes(a1379)
        };
        let a1381 = 1u64;
        nat_add(a1380, a1381)?
    })?;
    let a1382 = f350(v0.clone(), v2)?;
    let a1383 = 34u64;
    if nat_eq(a1382, a1383) {
        Ok(Some(({
            let a1384 = v2;
            let a1385 = 1u64;
            nat_add(a1384, a1385)?
        }, f35(v0, v1, {
            let a1386 = v2;
            let a1387 = v1;
            nat_sub(a1386, a1387)
        }))))
    } else {
        f421(v0, v1)
    }
}

pub fn f352(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f353(v0: Bytes, v1: u64) -> R<Option<(u64, Adt18)>> {
    let v2: bool = {
        let a1388 = f350(v0.clone(), v1)?;
        let a1389 = 45u64;
        nat_eq(a1388, a1389)
    };
    let v3: u64 = if v2 {
        let a1390 = v1;
        let a1391 = 1u64;
        nat_add(a1390, a1391)?
    } else {
        v1
    };
    let v4: u64 = f422(v0.clone(), v3)?;
    let v5: u64 = {
        let a1392 = v4;
        let a1393 = v3;
        nat_sub(a1392, a1393)
    };
    let v6: bool = {
        let a1394 = f350(v0.clone(), v4)?;
        let a1395 = 46u64;
        nat_eq(a1394, a1395)
    };
    let v7: u64 = if v6 {
        let a1396 = v4;
        let a1397 = 1u64;
        nat_add(a1396, a1397)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f422(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f350(v0.clone(), v8)?;
    let v10: bool = {
        let a1398 = v9;
        let a1399 = 101u64;
        if nat_eq(a1398, a1399) {
            true
        } else {
            let a1400 = v9;
            let a1401 = 69u64;
            nat_eq(a1400, a1401)
        }
    };
    let v11: u64 = f350(v0.clone(), {
        let a1402 = v8;
        let a1403 = 1u64;
        nat_add(a1402, a1403)?
    })?;
    let v12: bool = f352(v10, f423({
        let a1404 = v11;
        let a1405 = 43u64;
        nat_eq(a1404, a1405)
    }, {
        let a1406 = v11;
        let a1407 = 45u64;
        nat_eq(a1406, a1407)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a1408 = v8;
            let a1409 = 2u64;
            nat_add(a1408, a1409)?
        } else {
            let a1410 = v8;
            let a1411 = 1u64;
            nat_add(a1410, a1411)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f422(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f35(v0.clone(), v13, {
        let a1412 = v14;
        let a1413 = v13;
        nat_sub(a1412, a1413)
    });
    let v16: u64 = f424(v15.clone())?;
    let v17: Bytes = f35(v15.clone(), v16, {
        let a1415 = {
            let a1414 = v15;
            length_bytes(a1414)
        };
        let a1416 = v16;
        nat_sub(a1415, a1416)
    });
    let a1417 = v5;
    let a1418 = 0u64;
    if if nat_eq(a1417, a1418) {
        true
    } else {
        if f352({
            let a1419 = 1u64;
            let a1420 = v5;
            nat_lt(a1419, a1420)
        }, {
            let a1421 = f350(v0.clone(), v3)?;
            let a1422 = 48u64;
            nat_eq(a1421, a1422)
        }) {
            true
        } else {
            if f352(v6, {
                let a1423 = v8;
                let a1424 = v7;
                nat_eq(a1423, a1424)
            }) {
                true
            } else {
                f423(if v10 {
                    let a1425 = v14;
                    let a1426 = v13;
                    nat_eq(a1425, a1426)
                } else {
                    false
                }, {
                    let a1428 = 18u64;
                    let a1429 = {
                        let a1427 = v17.clone();
                        length_bytes(a1427)
                    };
                    nat_lt(a1428, a1429)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt18)>)
    } else {
        let m1433 = {
            let a1431 = {
                let a1430 = v17.clone();
                length_bytes(a1430)
            };
            let a1432 = 0u64;
            if nat_eq(a1431, a1432) {
                Some(0u64)
            } else {
                f425(v17)?
            }
        };
        match m1433 {
            None => {
                Ok(None::<(u64, Adt18)>)
            }
            Some(v18) => {
                let m1438 = f426(f35(v0.clone(), v3, v5), f35(v0, v7, {
                    let a1434 = v8;
                    let a1435 = v7;
                    nat_sub(a1434, a1435)
                }), f352(v12, {
                    let a1436 = v11;
                    let a1437 = 45u64;
                    nat_eq(a1436, a1437)
                }), v18)?;
                match m1438 {
                    None => {
                        Ok(None::<(u64, Adt18)>)
                    }
                    Some(v19) => {
                        let a1441 = 19u64;
                        let a1442 = {
                            let a1440 = {
                                let (h1439, _) = v19.clone();
                                h1439
                            };
                            length_bytes(a1440)
                        };
                        if nat_lt(a1441, a1442) {
                            Ok(None::<(u64, Adt18)>)
                        } else {
                            let m1444 = f425({
                                let (h1443, _) = v19.clone();
                                h1443
                            })?;
                            match m1444 {
                                None => {
                                    Ok(None::<(u64, Adt18)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f352(v2, {
                                        let a1445 = 0u64;
                                        let a1446 = v20;
                                        nat_lt(a1445, a1446)
                                    });
                                    if f427(v21, v20) {
                                        Ok(Some((v14, {
                                            let a1448 = {
                                                let (_, h1447) = v19.clone();
                                                h1447
                                            };
                                            let a1449 = 0u64;
                                            if nat_eq(a1448, a1449) {
                                                Adt18::C3(v21, v20)
                                            } else {
                                                Adt18::C4(v21, v20, {
                                                    let (_, h1450) = v19;
                                                    h1450
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt18)>)
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

pub fn f354(v0: Bytes) -> Bytes {
    if f190(v0.clone(), Bytes::lit(&[0x0d])) {
        f35(v0.clone(), 0u64, {
            let a1452 = {
                let a1451 = v0;
                length_bytes(a1451)
            };
            let a1453 = 1u64;
            nat_sub(a1452, a1453)
        })
    } else {
        v0
    }
}

pub fn f355(v0: Adt13, v1: Bytes) -> Adt13 {
    let m1454 = v0;
    match m1454 {
        Adt13::C0(v3) => {
            Adt13::C0(v3)
        }
        Adt13::C1(v4, v5) => {
            Adt13::C1(v4, v5)
        }
        Adt13::C2(v2) => {
            Adt13::C2(f428(f49(), v2, v1))
        }
        Adt13::C3(r1455) => {
            let v6 = (*r1455).clone();
            Adt13::C3(std::rc::Rc::new(v6))
        }
        Adt13::C4(r1456) => {
            let v7 = (*r1456).clone();
            Adt13::C4(std::rc::Rc::new(v7))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r1457, r1458) => {
            let v8 = (*r1457).clone();
            let v9 = (*r1458).clone();
            Adt13::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt13::C7(v10, r1459, r1460) => {
            let v11 = (*r1459).clone();
            let v12 = (*r1460).clone();
            Adt13::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f356(v0: u64) -> R<Bytes> {
    f429(f430(), 10u64, v0)
}

pub fn f357(v0: Fn4, p1465: u64, p1466: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1464 = (p1465, p1466);
    {
        let mut m1464 = m1464;
        loop {
            let step = {
                let (v1, v2) = m1464;
                let m1461 = v1;
                if m1461 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1461 - 1;
                    let m1463 = {
                        let c1462: Fn4 = v0.clone();
                        c1462.apply(v2.clone())?
                    };
                    match m1463 {
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
                Ok(next) => { m1464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f431(v0, v1)
}

pub fn f359(v0: Fn5, p1470: List<(Bytes, Bytes)>, p1471: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1469 = (p1470, p1471);
    {
        let mut m1469 = m1469;
        loop {
            let step = {
                let (v1, v2) = m1469;
                let m1467 = v2;
                match m1467.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1468: Fn5 = v0.clone();
                            c1468.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1469 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f360(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f432(v0, v1)
}

pub fn f361(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f433(v0)
}

pub fn f362(v0: Fn6, p1475: List<Bytes>, p1476: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1474 = (p1475, p1476);
    {
        let mut m1474 = m1474;
        loop {
            let step = {
                let (v1, v2) = m1474;
                let m1472 = v2;
                match m1472.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1473: Fn6 = v0.clone();
                            c1473.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1474 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f434(v1)?, v0))
}

pub fn f364(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f267(v0)?, List::cons(f8(Str::lit(":")), List::cons(f267(v1)?, List::<Bytes>::nil())))))
}

pub fn f365(v0: Adt12) -> Bytes {
    let m1477 = v0;
    match m1477 {
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

pub fn f366(v0: Adt12) -> Bytes {
    let m1478 = v0;
    match m1478 {
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

pub fn f367(v0: Adt12) -> Bytes {
    let m1479 = v0;
    match m1479 {
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

pub fn f368(v0: Bytes, v1: u64) -> Option<u64> {
    let a1480 = 0u64;
    let a1481 = v1;
    if nat_lt(a1480, a1481) {
        if f435(v0, {
            let a1482 = v1;
            let a1483 = 1u64;
            nat_sub(a1482, a1483)
        }) {
            Some({
                let a1484 = v1;
                let a1485 = 1u64;
                nat_sub(a1484, a1485)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f369(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1486 = v2;
    let a1487 = v0;
    if nat_lt(a1486, a1487) {
        if f435(v1, v2) {
            Ok(Some({
                let a1488 = v2;
                let a1489 = 1u64;
                nat_add(a1488, a1489)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f370(v0: (Bytes, Bytes), p1492: List<(Bytes, Bytes)>) -> bool {
    let m1491 = p1492;
    {
        let mut m1491 = m1491;
        loop {
            let step = {
                let v1 = m1491;
                let m1490 = v1;
                match m1490.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f436(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1493 = v1;
    match m1493.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h1494, _) = v2;
                h1494
            }, v0.clone()) {
                List::cons({
                    let (_, h1495) = v2;
                    h1495
                }, f371(v0, v3))
            } else {
                f371(v0, v3)
            }
        }
    }
}

pub fn f372() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f373(v0: Adt13, v1: Bytes, v2: Bytes) -> bool {
    let m1496 = f79(v0, v1);
    match m1496 {
        None => {
            false
        }
        Some(v3) => {
            f185(v3, v2)
        }
    }
}

pub fn f374() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f375(v0: Bytes) -> R<bool> {
    let m1497 = f194(v0.clone(), f8(Str::lit("&&")))?;
    if match m1497 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1498 = f194(v0, f8(Str::lit("||")))?;
        match m1498 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f376(v0: Adt17, v1: Option<u64>) -> bool {
    let m1499 = v1;
    match m1499 {
        None => {
            true
        }
        Some(v2) => {
            f296(v2, match v0 {
                Adt17::C0(_, h1500, _, _) => {
                    h1500
                }
            })
        }
    }
}

pub fn f377(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a1502 = match v0 {
            Adt15::C0(_, _, h1501, _, _) => {
                h1501
            }
        };
        length_list(a1502)
    })
}

pub fn f378(v0: Bytes) -> Option<Adt22> {
    if f28(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt22::C0)
    } else {
        if f28(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt22::C1)
        } else {
            if f28(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt22::C2)
            } else {
                None::<Adt22>
            }
        }
    }
}

pub fn f379(v0: Bytes, p1507: List<Bytes>, p1508: u64) -> R<Option<u64>> {
    let m1506 = (p1507, p1508);
    {
        let mut m1506 = m1506;
        loop {
            let step = {
                let (v1, v2) = m1506;
                let m1503 = v1;
                match m1503.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f28(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1504 = v2;
                                let a1505 = 1u64;
                                nat_add(a1504, a1505)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: Option<Adt13>) -> Option<u64> {
    let m1509 = v0;
    match m1509 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1510 = v1;
            match m1510 {
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

pub fn f381(v0: Bytes, v1: u64) -> R<u64> {
    let m1513 = {
        let a1511 = v0;
        let a1512 = v1;
        index_bytes(a1511, a1512)
    };
    match m1513 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f62(v2)
        }
    }
}

pub fn f382(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1517 = {
        let a1514 = v1;
        let a1515 = v2;
        let a1516 = 1u64;
        slice_bytes(a1514, a1515, a1516)
    };
    match m1517 {
        None => {
            v0
        }
        Some(v3) => {
            let a1518 = v0;
            let a1519 = v3;
            append_bytes(a1518, a1519)
        }
    }
}

pub fn f383(p1522: List<Bytes>, p1523: List<Bytes>) -> List<Bytes> {
    let m1521 = (p1522, p1523);
    {
        let mut m1521 = m1521;
        loop {
            let step = {
                let (v0, v1) = m1521;
                let m1520 = v0;
                match m1520.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1521 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: Fn7, p1527: List<Bytes>, p1528: List<(Bytes, ())>) -> List<Bytes> {
    let m1526 = (p1527, p1528);
    {
        let mut m1526 = m1526;
        loop {
            let step = {
                let (v1, v2) = m1526;
                let m1524 = v2;
                match m1524.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1525: Fn7 = v0.clone();
                            c1525.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1529, _) = v1;
        h1529
    }, v0)
}

pub fn f386(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f437(Fn12::F438, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f387(v0: Fn8, p1533: List<List<(Bytes, ())>>, p1534: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1532 = (p1533, p1534);
    {
        let mut m1532 = m1532;
        loop {
            let step = {
                let (v1, v2) = m1532;
                let m1530 = v2;
                match m1530.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1531: Fn8 = v0.clone();
                            c1531.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1532 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f388(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f389(v0: Fn9, p1539: u64, p1540: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1538 = (p1539, p1540);
    {
        let mut m1538 = m1538;
        loop {
            let step = {
                let (v1, v2) = m1538;
                let m1535 = v1;
                if m1535 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1535 - 1;
                    let m1537 = {
                        let c1536: Fn9 = v0.clone();
                        c1536.apply(v2.clone())?
                    };
                    match m1537 {
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
                Ok(next) => { m1538 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f439(v0)
}

pub fn f391(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f440(Fn13::F441, List::<(Bytes, ())>::nil(), v0)
}

pub fn f392(v0: Fn10, p1544: List<(Bytes, ())>, p1545: List<Bytes>) -> List<(Bytes, ())> {
    let m1543 = (p1544, p1545);
    {
        let mut m1543 = m1543;
        loop {
            let step = {
                let (v1, v2) = m1543;
                let m1541 = v2;
                match m1541.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1542: Fn10 = v0.clone();
                            c1542.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1543 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f394(p1548: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1547 = p1548;
    {
        let mut m1547 = m1547;
        loop {
            let step = {
                let v0 = m1547;
                let m1546 = v0;
                match m1546.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f442(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
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

pub fn f395(v0: Bytes, p1574: u64, p1575: u64, p1576: Bytes) -> R<Bytes> {
    let m1573 = (p1574, (p1575, p1576));
    {
        let mut m1573 = m1573;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1573;
                let m1549 = v2;
                if m1549 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1549 - 1;
                    let m1552 = {
                        let a1550 = v0.clone();
                        let a1551 = v1;
                        index_bytes(a1550, a1551)
                    };
                    match m1552 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            if f423({
                                let a1553 = v6;
                                let a1554 = 34u64;
                                nat_eq(a1553, a1554)
                            }, {
                                let a1555 = v6;
                                let a1556 = 92u64;
                                nat_eq(a1555, a1556)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1557 = v1;
                                    let a1558 = 1u64;
                                    nat_add(a1557, a1558)?
                                }, (v4, {
                                    let a1561 = v3;
                                    let a1562 = {
                                        let a1559 = f8(Str::lit("\\"));
                                        let a1560 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1559, a1560)
                                    };
                                    append_bytes(a1561, a1562)
                                })))
                            } else {
                                let a1563 = v6;
                                let a1564 = 10u64;
                                if nat_eq(a1563, a1564) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1565 = v1;
                                        let a1566 = 1u64;
                                        nat_add(a1565, a1566)?
                                    }, (v4, {
                                        let a1567 = v3;
                                        let a1568 = f8(Str::lit("\\n"));
                                        append_bytes(a1567, a1568)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1569 = v1;
                                        let a1570 = 1u64;
                                        nat_add(a1569, a1570)?
                                    }, (v4, {
                                        let a1571 = v3;
                                        let a1572 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1571, a1572)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(v0: Option<Adt13>, v1: Bytes) -> Bytes {
    let m1577 = v0;
    match m1577 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f47(v2, v1)
        }
    }
}

pub fn f397(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
    let m1578 = f259(v2.clone())?;
    match m1578 {
        None => {
            f444(v0, v1, v2)
        }
        Some(v3) => {
            f443(v0, v1, v3)
        }
    }
}

pub fn f398(v0: List<Bytes>) -> List<Bytes> {
    let m1579 = v0;
    match m1579.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1581 = {
                    let a1580 = v1.clone();
                    length_bytes(a1580)
                };
                let a1582 = 0u64;
                if nat_eq(a1581, a1582) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f398(v2))
        }
    }
}

pub fn f399(v0: List<Bytes>) -> Bytes {
    let m1583 = v0;
    match m1583.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f400(v0: Bytes) -> R<bool> {
    let m1585 = {
        let a1584 = v0.clone();
        length_bytes(a1584)
    };
    if m1585 == 0 {
        Ok(true)
    } else {
        let v1 = m1585 - 1;
        let m1588 = {
            let a1586 = v0;
            let a1587 = v1;
            index_bytes(a1586, a1587)
        };
        match m1588 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f445({
                    let v3: u8 = v2;
                    let a1633 = {
                        let a1609 = {
                            let a1597 = {
                                let a1591 = {
                                    let a1589 = v3;
                                    let a1590 = 128u8;
                                    fixed_u8::bit_and(a1589, a1590)
                                };
                                let a1592 = 128u8;
                                if equal(a1591, a1592) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1598 = {
                                let a1595 = {
                                    let a1593 = v3;
                                    let a1594 = 64u8;
                                    fixed_u8::bit_and(a1593, a1594)
                                };
                                let a1596 = 64u8;
                                if equal(a1595, a1596) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1597, a1598)?
                        };
                        let a1610 = {
                            let a1607 = {
                                let a1601 = {
                                    let a1599 = v3;
                                    let a1600 = 32u8;
                                    fixed_u8::bit_and(a1599, a1600)
                                };
                                let a1602 = 32u8;
                                if equal(a1601, a1602) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1608 = {
                                let a1605 = {
                                    let a1603 = v3;
                                    let a1604 = 16u8;
                                    fixed_u8::bit_and(a1603, a1604)
                                };
                                let a1606 = 16u8;
                                if equal(a1605, a1606) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1607, a1608)?
                        };
                        nat_add(a1609, a1610)?
                    };
                    let a1634 = {
                        let a1631 = {
                            let a1619 = {
                                let a1613 = {
                                    let a1611 = v3;
                                    let a1612 = 8u8;
                                    fixed_u8::bit_and(a1611, a1612)
                                };
                                let a1614 = 8u8;
                                if equal(a1613, a1614) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1620 = {
                                let a1617 = {
                                    let a1615 = v3;
                                    let a1616 = 4u8;
                                    fixed_u8::bit_and(a1615, a1616)
                                };
                                let a1618 = 4u8;
                                if equal(a1617, a1618) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1619, a1620)?
                        };
                        let a1632 = {
                            let a1629 = {
                                let a1623 = {
                                    let a1621 = v3;
                                    let a1622 = 2u8;
                                    fixed_u8::bit_and(a1621, a1622)
                                };
                                let a1624 = 2u8;
                                if equal(a1623, a1624) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1630 = {
                                let a1627 = {
                                    let a1625 = v3;
                                    let a1626 = 1u8;
                                    fixed_u8::bit_and(a1625, a1626)
                                };
                                let a1628 = 1u8;
                                if equal(a1627, a1628) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1629, a1630)?
                        };
                        nat_add(a1631, a1632)?
                    };
                    nat_add(a1633, a1634)?
                }))
            }
        }
    }
}

pub fn f401(v0: Adt18, v1: Bytes) -> bool {
    let a1636 = {
        let a1635 = f245(v0, v1);
        length_bytes(a1635)
    };
    let a1637 = 0u64;
    !nat_eq(a1636, a1637)
}

pub fn f402(v0: Adt18, v1: Bytes) -> bool {
    let m1638 = f242(v0, v1);
    match m1638 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f403(v0: Bytes, p1659: u64, p1660: u64, p1661: bool, p1662: Bytes) -> R<Bytes> {
    let m1658 = (p1659, (p1660, (p1661, p1662)));
    {
        let mut m1658 = m1658;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1658;
                let m1639 = v2;
                if m1639 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1639 - 1;
                    let m1642 = {
                        let a1640 = v0.clone();
                        let a1641 = v1;
                        index_bytes(a1640, a1641)
                    };
                    match m1642 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f62(v6)?;
                            if f446(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1643 = v1;
                                    let a1644 = 1u64;
                                    nat_add(a1643, a1644)?
                                }, (v5, (true, v4))))
                            } else {
                                if f352(f447(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1645 = v1;
                                        let a1646 = 1u64;
                                        nat_add(a1645, a1646)?
                                    }, (v5, (false, {
                                        let a1650 = v4;
                                        let a1651 = f35({
                                            let a1647 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1647)
                                        }, {
                                            let a1648 = v7;
                                            let a1649 = 97u64;
                                            nat_sub(a1648, a1649)
                                        }, 1u64);
                                        append_bytes(a1650, a1651)
                                    }))))
                                } else {
                                    if f423(f447(v7), f423(f448(v7), f449(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1652 = v1;
                                            let a1653 = 1u64;
                                            nat_add(a1652, a1653)?
                                        }, (v5, (false, {
                                            let a1654 = v4;
                                            let a1655 = f35(v0.clone(), v1, 1u64);
                                            append_bytes(a1654, a1655)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1656 = v1;
                                            let a1657 = 1u64;
                                            nat_add(a1656, a1657)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: Adt18, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt18)> = f450(f451(f242(v0.clone(), f8(Str::lit("$defs")))), f451(f242(v0.clone(), f8(Str::lit("definitions")))));
    let m1663 = f452(4096u64, v3.clone(), List::<Bytes>::nil(), f453(v3)?)?;
    match m1663 {
        Ok(v5) => {
            let m1664 = f454(4096u64, f455(f456(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1664 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f342(f213(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f405(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f356(v1)?;
    let v4: u64 = {
        let a1665 = v3.clone();
        length_bytes(a1665)
    };
    let v5: Bytes = {
        let a1666 = v2;
        let a1667 = v4;
        if nat_lt(a1666, a1667) {
            f7(List::cons(f35(v3.clone(), 0u64, {
                let a1668 = v4;
                let a1669 = v2;
                nat_sub(a1668, a1669)
            }), List::cons(f8(Str::lit(".")), List::cons(f35(v3, {
                let a1670 = v4;
                let a1671 = v2;
                nat_sub(a1670, a1671)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f457({
                let a1672 = v2;
                let a1673 = v4;
                nat_sub(a1672, a1673)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1674 = f8(Str::lit("-"));
        let a1675 = v5;
        Ok(append_bytes(a1674, a1675))
    } else {
        Ok(v5)
    }
}

pub fn f406(v0: Bytes) -> R<Bytes> {
    let a1680 = f420(v0.clone(), 0u64, {
        let a1677 = {
            let a1676 = v0.clone();
            length_bytes(a1676)
        };
        let a1678 = 1u64;
        nat_add(a1677, a1678)?
    })?;
    let a1681 = {
        let a1679 = v0.clone();
        length_bytes(a1679)
    };
    if nat_eq(a1680, a1681) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f267(v0)
    }
}

pub fn f407(v0: Adt18) -> R<List<Bytes>> {
    let m1682 = v0;
    match m1682 {
        Adt18::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C3(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C4(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C8 => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C9(r1683, r1684) => {
            let v12 = (*r1683).clone();
            let v13 = (*r1684).clone();
            Ok(List::cons(f333(v12)?, f407(v13)?))
        }
        Adt18::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f408(v0: Adt18) -> R<List<(Bytes, Bytes)>> {
    let m1685 = v0;
    match m1685 {
        Adt18::C0 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C3(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C4(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C8 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C9(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C10(v14, r1686, r1687) => {
            let v15 = (*r1686).clone();
            let v16 = (*r1687).clone();
            Ok(List::cons((v14, f333(v15)?), f408(v16)?))
        }
    }
}

pub fn f409(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f342(f458(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f410(v0: u32) -> Bytes {
    let a1716 = f459({
        let a1688 = f460(v0, 28u32);
        let a1689 = 15u32;
        fixed_u32::bit_and(a1688, a1689)
    });
    let a1717 = {
        let a1714 = f459({
            let a1690 = f460(v0, 24u32);
            let a1691 = 15u32;
            fixed_u32::bit_and(a1690, a1691)
        });
        let a1715 = {
            let a1712 = f459({
                let a1692 = f460(v0, 20u32);
                let a1693 = 15u32;
                fixed_u32::bit_and(a1692, a1693)
            });
            let a1713 = {
                let a1710 = f459({
                    let a1694 = f460(v0, 16u32);
                    let a1695 = 15u32;
                    fixed_u32::bit_and(a1694, a1695)
                });
                let a1711 = {
                    let a1708 = f459({
                        let a1696 = f460(v0, 12u32);
                        let a1697 = 15u32;
                        fixed_u32::bit_and(a1696, a1697)
                    });
                    let a1709 = {
                        let a1706 = f459({
                            let a1698 = f460(v0, 8u32);
                            let a1699 = 15u32;
                            fixed_u32::bit_and(a1698, a1699)
                        });
                        let a1707 = {
                            let a1704 = f459({
                                let a1700 = f460(v0, 4u32);
                                let a1701 = 15u32;
                                fixed_u32::bit_and(a1700, a1701)
                            });
                            let a1705 = f459({
                                let a1702 = v0;
                                let a1703 = 15u32;
                                fixed_u32::bit_and(a1702, a1703)
                            });
                            append_bytes(a1704, a1705)
                        };
                        append_bytes(a1706, a1707)
                    };
                    append_bytes(a1708, a1709)
                };
                append_bytes(a1710, a1711)
            };
            append_bytes(a1712, a1713)
        };
        append_bytes(a1714, a1715)
    };
    append_bytes(a1716, a1717)
}

pub fn f411(v0: Fn11, p1721: u64, p1722: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1720 = (p1721, p1722);
    {
        let mut m1720 = m1720;
        loop {
            let step = {
                let (v1, v2) = m1720;
                let m1718 = v1;
                if m1718 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1718 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1719: Fn11 = v0.clone();
                        c1719.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1720 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1724 = {
            let (h1723, _) = v3.clone();
            h1723
        };
        let a1725 = 64u64;
        nat_add(a1724, a1725)?
    }, f461({
        let (_, h1726) = v3;
        h1726
    }, f462(v0, v1, v2, {
        let (h1727, _) = v3;
        h1727
    })?)?))
}

pub fn f413() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f414(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1728 = f33(v1, 34u8)?;
    match m1728.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1729 = v3;
            match m1729.uncons() {
                None => {
                    Ok(f463(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f415(v0: List<Bytes>) -> List<Bytes> {
    let m1730 = v0;
    match m1730.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("authority = []"))) {
                f415(v2)
            } else {
                List::cons(v1, f415(v2))
            }
        }
    }
}

pub fn f416(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1731 = v0;
    match m1731.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f416(v3, v1))
        }
    }
}

pub fn f417(v0: List<Adt13>, v1: Bytes, v2: Bytes, v3: Adt13) -> List<Adt13> {
    let m1732 = v0;
    match m1732.uncons() {
        None => {
            List::cons(v3, List::<Adt13>::nil())
        }
        Some((v4, v5)) => {
            if f28(f47(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f464(v5, v1, v2))
            } else {
                List::cons(v4, f417(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f418(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f342(f465(f81(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f419(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1735 = v3;
    let a1736 = f35(v0, v1, {
        let a1733 = v2;
        let a1734 = v1;
        nat_sub(a1733, a1734)
    });
    append_bytes(a1735, a1736)
}

pub fn f420(v0: Bytes, p1749: u64, p1750: u64) -> R<u64> {
    let m1748 = (p1749, p1750);
    {
        let mut m1748 = m1748;
        loop {
            let step = {
                let (v1, v2) = m1748;
                let m1737 = v2;
                if m1737 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1737 - 1;
                    let v4: u64 = f350(v0.clone(), v1)?;
                    let a1738 = v4;
                    let a1739 = 34u64;
                    if nat_eq(a1738, a1739) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1740 = v4;
                        let a1741 = 92u64;
                        if nat_eq(a1740, a1741) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1742 = v4;
                            let a1743 = 32u64;
                            if nat_lt(a1742, a1743) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1744 = v4;
                                let a1745 = 256u64;
                                if nat_eq(a1744, a1745) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1746 = v1;
                                        let a1747 = 1u64;
                                        nat_add(a1746, a1747)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f466(Fn14::F467(v0.clone()), {
        let a1752 = {
            let a1751 = v0;
            length_bytes(a1751)
        };
        let a1753 = 1u64;
        nat_add(a1752, a1753)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1757 = {
        let (_, h1754) = {
            let (_, h1755) = {
                let (h1756, _) = v2.clone();
                h1756
            };
            h1755
        };
        h1754
    };
    let a1758 = 1u64;
    if nat_eq(a1757, a1758) {
        Ok(Some(({
            let (h1759, _) = {
                let (h1760, _) = v2.clone();
                h1760
            };
            h1759
        }, {
            let (h1761, _) = {
                let (_, h1762) = {
                    let (h1763, _) = v2;
                    h1763
                };
                h1762
            };
            h1761
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f422(v0: Bytes, v1: u64) -> R<u64> {
    let (h1764, _) = f286(Fn3::F468(v0.clone()), {
        let a1766 = {
            let a1765 = v0;
            length_bytes(a1765)
        };
        let a1767 = 1u64;
        nat_add(a1766, a1767)?
    }, v1)?;
    Ok(h1764)
}

pub fn f423(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f424(v0: Bytes) -> R<u64> {
    let (h1768, _) = f286(Fn3::F469(v0.clone()), {
        let a1770 = {
            let a1769 = v0;
            length_bytes(a1769)
        };
        let a1771 = 1u64;
        nat_add(a1770, a1771)?
    }, 0u64)?;
    Ok(h1768)
}

pub fn f425(v0: Bytes) -> R<Option<u64>> {
    if f470(v0.clone())? {
        Ok(Some({
            let (_, h1772) = {
                let (h1773, _) = f471(Fn15::F472(v0.clone()), {
                    let a1774 = v0;
                    length_bytes(a1774)
                }, (0u64, 0u64))?;
                h1773
            };
            h1772
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f426(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f35(v1.clone(), 0u64, {
        let a1776 = {
            let a1775 = v1.clone();
            length_bytes(a1775)
        };
        let a1777 = f473(v1)?;
        nat_sub(a1776, a1777)
    });
    let v5: Bytes = {
        let a1778 = v0;
        let a1779 = v4.clone();
        append_bytes(a1778, a1779)
    };
    let v6: u64 = f424(v5.clone())?;
    let v7: Bytes = f35(v5.clone(), v6, {
        let a1781 = {
            let a1780 = v5;
            length_bytes(a1780)
        };
        let a1782 = v6;
        nat_sub(a1781, a1782)
    });
    let v8: u64 = {
        let a1783 = v4;
        length_bytes(a1783)
    };
    let v9: u64 = {
        let a1784 = v7.clone();
        length_bytes(a1784)
    };
    let a1785 = v9;
    let a1786 = 0u64;
    if nat_eq(a1785, a1786) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f423(v2, {
            let a1787 = v3;
            let a1788 = v8;
            nat_lt(a1787, a1788)
        }) {
            let v10: u64 = if v2 {
                let a1789 = v3;
                let a1790 = v8;
                nat_add(a1789, a1790)?
            } else {
                let a1791 = v8;
                let a1792 = v3;
                nat_sub(a1791, a1792)
            };
            let v11: u64 = f473(v7.clone())?;
            let a1793 = v10;
            let a1794 = v11;
            if nat_le(a1793, a1794) {
                Ok(Some((f35(v7, 0u64, {
                    let a1795 = v9;
                    let a1796 = v10;
                    nat_sub(a1795, a1796)
                }), 0u64)))
            } else {
                let a1799 = {
                    let a1797 = v10;
                    let a1798 = v11;
                    nat_sub(a1797, a1798)
                };
                let a1800 = 19u64;
                if nat_le(a1799, a1800) {
                    Ok(Some((f35(v7, 0u64, {
                        let a1801 = v9;
                        let a1802 = v11;
                        nat_sub(a1801, a1802)
                    }), {
                        let a1803 = v10;
                        let a1804 = v11;
                        nat_sub(a1803, a1804)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1805 = v3;
                let a1806 = v8;
                nat_sub(a1805, a1806)
            };
            let a1809 = {
                let a1807 = v9;
                let a1808 = v12;
                nat_add(a1807, a1808)?
            };
            let a1810 = 19u64;
            if nat_le(a1809, a1810) {
                Ok(Some(({
                    let a1811 = v7;
                    let a1812 = f457(v12);
                    append_bytes(a1811, a1812)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f427(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1813 = 0u64;
        let a1814 = v1;
        if nat_lt(a1813, a1814) {
            let a1815 = v1;
            let a1816 = 9223372036854775808u64;
            nat_le(a1815, a1816)
        } else {
            false
        }
    } else {
        let a1817 = v1;
        let a1818 = 9223372036854775807u64;
        nat_le(a1817, a1818)
    }
}

pub fn f428(p1821: List<Adt12>, v1: Bytes, v2: Bytes) -> Bytes {
    let m1820 = p1821;
    {
        let mut m1820 = m1820;
        loop {
            let step = {
                let v0 = m1820;
                let m1819 = v0;
                match m1819.uncons() {
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
                Ok(next) => { m1820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f429(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1822 = v2;
    let a1823 = 0u64;
    if nat_eq(a1822, a1823) {
        Ok(f474(v0, 0u64))
    } else {
        let (_, h1824) = {
            let (h1825, _) = f357(Fn4::F475(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1825
        };
        Ok(h1824)
    }
}

pub fn f430() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f431(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1829 = {
        let a1827 = v0.clone();
        let a1828 = {
            let (h1826, _) = v1.clone();
            h1826
        };
        index_bytes(a1827, a1828)
    };
    match m1829 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1831 = {
                    let (h1830, _) = v1.clone();
                    h1830
                };
                let a1832 = 1u64;
                nat_add(a1831, a1832)?
            }, {
                let a1835 = {
                    let (_, h1833) = v1;
                    h1833
                };
                let a1836 = f476(v0, {
                    let (h1834, _) = v1;
                    h1834
                }, v2)?;
                append_bytes(a1835, a1836)
            })))
        }
    }
}

pub fn f432(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1837 = v0.clone();
    match m1837.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1840 = {
                let (h1838, _) = v2;
                h1838
            };
            let a1841 = {
                let (h1839, _) = v1.clone();
                h1839
            };
            if equal(a1840, a1841) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f433(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f477(f478(Fn16::F479, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1846 = {
        let (h1842, _) = f480(Fn17::F481, {
            let a1844 = {
                let a1843 = v0;
                length_list(a1843)
            };
            let a1845 = 1u64;
            nat_add(a1844, a1845)?
        }, v1)?;
        h1842
    };
    match m1846.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f434(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1851 = {
        let a1848 = f267({
            let (h1847, _) = v0;
            h1847
        })?;
        let a1849 = Bytes::lit(&[0x3a]);
        append_bytes(a1848, a1849)
    };
    let a1852 = {
        let (_, h1850) = v0;
        h1850
    };
    Ok(append_bytes(a1851, a1852))
}

pub fn f435(v0: Bytes, v1: u64) -> bool {
    let m1853 = f174(v0, v1);
    match m1853 {
        None => {
            false
        }
        Some(v2) => {
            f482(v2)
        }
    }
}

pub fn f436(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f352(f28({
        let (h1854, _) = v0;
        h1854
    }, {
        let (h1855, _) = v1;
        h1855
    }), f28({
        let (_, h1856) = v0;
        h1856
    }, {
        let (_, h1857) = v1;
        h1857
    }))
}

pub fn f437(v0: Fn12, p1861: List<List<(Bytes, ())>>, p1862: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1860 = (p1861, p1862);
    {
        let mut m1860 = m1860;
        loop {
            let step = {
                let (v1, v2) = m1860;
                let m1858 = v2;
                match m1858.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1859: Fn12 = v0.clone();
                            c1859.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1860 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f439(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1864 = {
        let a1863 = v0.clone();
        length_list(a1863)
    };
    let a1865 = 1u64;
    if nat_le(a1864, a1865) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f483(v0)?))
    }
}

pub fn f440(v0: Fn13, p1869: List<(Bytes, ())>, p1870: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1868 = (p1869, p1870);
    {
        let mut m1868 = m1868;
        loop {
            let step = {
                let (v1, v2) = m1868;
                let m1866 = v2;
                match m1866.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1867: Fn13 = v0.clone();
                            c1867.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1868 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f442(p1874: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1873 = p1874;
    {
        let mut m1873 = m1873;
        loop {
            let step = {
                let v0 = m1873;
                let m1871 = v0;
                match m1871.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f484(v2, v1.clone(), 0u64, {
                            let a1872 = v1.clone();
                            length_bytes(a1872)
                        })? {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1873 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: Adt1, v1: Bytes, v2: List<Adt13>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f121(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f51(v3.clone(), f121(f82(v0, f132())?, f8(Str::lit("at"))));
    let a1876 = {
        let a1875 = v4.clone();
        length_list(a1875)
    };
    let a1877 = 0u64;
    if nat_eq(a1876, a1877) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1880 = {
            let a1878 = v4;
            length_list(a1878)
        };
        let a1881 = {
            let a1879 = v3;
            length_list(a1879)
        };
        if nat_eq(a1880, a1881) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f485(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f444(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt13> = f323(f236(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt13> = f323(f236(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f98(f121(v3.clone(), f8(Str::lit("id"))), f121(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f121(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f51(v5.clone(), f98(v6.clone(), f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1883 = {
        let a1882 = v7.clone();
        length_list(a1882)
    };
    let a1884 = 0u64;
    if nat_eq(a1883, a1884) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f40({
            let a1887 = {
                let a1885 = v7;
                length_list(a1885)
            };
            let a1888 = {
                let a1886 = v5;
                length_list(a1886)
            };
            nat_eq(a1887, a1888)
        }) {
            Ok((false, f485(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f125(f98(f51(f486(v4.clone()), f52(f53(v0, f8(Str::lit("model/authorities.toml"))))?), f51(f487(v4), f98(v6, f121(v3, f8(Str::lit("id")))))))?;
            let a1890 = {
                let a1889 = v8.clone();
                length_list(a1889)
            };
            let a1891 = 0u64;
            if nat_eq(a1890, a1891) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f485(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f95(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f445(v0: u64) -> bool {
    let a1892 = v0;
    let a1893 = 47u64;
    nat_eq(a1892, a1893)
}

pub fn f446(v0: u64) -> bool {
    f423({
        let a1894 = v0;
        let a1895 = 95u64;
        nat_eq(a1894, a1895)
    }, f423({
        let a1896 = v0;
        let a1897 = 45u64;
        nat_eq(a1896, a1897)
    }, f423({
        let a1898 = v0;
        let a1899 = 32u64;
        nat_eq(a1898, a1899)
    }, f423({
        let a1900 = v0;
        let a1901 = 46u64;
        nat_eq(a1900, a1901)
    }, {
        let a1902 = v0;
        let a1903 = 47u64;
        nat_eq(a1902, a1903)
    }))))
}

pub fn f447(v0: u64) -> bool {
    f352({
        let a1904 = 97u64;
        let a1905 = v0;
        nat_le(a1904, a1905)
    }, {
        let a1906 = v0;
        let a1907 = 122u64;
        nat_le(a1906, a1907)
    })
}

pub fn f448(v0: u64) -> bool {
    f352({
        let a1908 = 65u64;
        let a1909 = v0;
        nat_le(a1908, a1909)
    }, {
        let a1910 = v0;
        let a1911 = 90u64;
        nat_le(a1910, a1911)
    })
}

pub fn f449(v0: u64) -> bool {
    f352({
        let a1912 = 48u64;
        let a1913 = v0;
        nat_le(a1912, a1913)
    }, {
        let a1914 = v0;
        let a1915 = 57u64;
        nat_le(a1914, a1915)
    })
}

pub fn f450(v0: List<(Bytes, Adt18)>, v1: List<(Bytes, Adt18)>) -> List<(Bytes, Adt18)> {
    let m1916 = v0;
    match m1916.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f450(v3, v1))
        }
    }
}

pub fn f451(v0: Option<Adt18>) -> List<(Bytes, Adt18)> {
    let m1917 = v0;
    match m1917 {
        None => {
            List::<(Bytes, Adt18)>::nil()
        }
        Some(v1) => {
            f488(v1)
        }
    }
}

pub fn f452(v0: u64, v1: List<(Bytes, Adt18)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt18)>, Bytes>> {
    let m1918 = v0;
    if m1918 == 0 {
        let m1919 = v1;
        match m1919.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::<(Bytes, Adt18)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt18)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m1918 - 1;
        let m1920 = v1.clone();
        match m1920.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::<(Bytes, Adt18)>::nil()))
            }
            Some((_, _)) => {
                let m1921 = f489(v1.clone(), v2.clone(), v3.clone())?;
                match m1921 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt18)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f342(f453(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m1924 = f452(v6, f490(v1, {
                            let (h1922, _) = v9.clone();
                            h1922
                        }), List::cons(f491({
                            let (h1923, _) = v9.clone();
                            h1923
                        })?, v2), v3)?;
                        match m1924 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt18)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f453(v0: List<(Bytes, Adt18)>) -> R<List<Bytes>> {
    let m1925 = v0;
    match m1925.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f491({
                let (h1926, _) = v1;
                h1926
            })?, f453(v2)?))
        }
    }
}

pub fn f454(p1932: u64, p1933: List<Adt25>, p1934: List<Bytes>, p1935: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1931 = (p1932, (p1933, (p1934, p1935)));
    {
        let mut m1931 = m1931;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1931;
                let m1927 = v0;
                if m1927 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1927 - 1;
                    let m1928 = v1;
                    match m1928.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1929 = v5;
                            match m1929 {
                                Adt25::C0(v8, v9) => {
                                    if f28(v8.clone(), f492()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f41(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1930 = f494(v8.clone(), v9)?;
                                            match m1930 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f455(f495(v11.clone()), List::cons(Adt25::C1(f496(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f41(f492(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f492(), v2), List::cons(f493(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1931 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m1936 = v0;
    match m1936.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f455(v3, v1))
        }
    }
}

pub fn f456(v0: List<(Bytes, Adt18)>) -> R<List<Adt25>> {
    let m1937 = v0;
    match m1937.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f491({
                let (h1938, _) = v1;
                h1938
            })?, {
                let (_, h1939) = v1;
                h1939
            }), f456(v2)?))
        }
    }
}

pub fn f457(v0: u64) -> Bytes {
    let m1940 = v0;
    if m1940 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1940 - 1;
        let a1941 = f8(Str::lit("0"));
        let a1942 = f457(v1);
        append_bytes(a1941, a1942)
    }
}

pub fn f458(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1943 = v0;
    match m1943.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f306(v1)?, f458(v2)?))
        }
    }
}

pub fn f459(v0: u32) -> Bytes {
    let a1946 = {
        let a1944 = v0;
        let a1945 = 8u32;
        fixed_u32::bit_and(a1944, a1945)
    };
    let a1947 = 0u32;
    if equal(a1946, a1947) {
        let a1950 = {
            let a1948 = v0;
            let a1949 = 4u32;
            fixed_u32::bit_and(a1948, a1949)
        };
        let a1951 = 0u32;
        if equal(a1950, a1951) {
            let a1954 = {
                let a1952 = v0;
                let a1953 = 2u32;
                fixed_u32::bit_and(a1952, a1953)
            };
            let a1955 = 0u32;
            if equal(a1954, a1955) {
                let a1958 = {
                    let a1956 = v0;
                    let a1957 = 1u32;
                    fixed_u32::bit_and(a1956, a1957)
                };
                let a1959 = 0u32;
                if equal(a1958, a1959) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1962 = {
                    let a1960 = v0;
                    let a1961 = 1u32;
                    fixed_u32::bit_and(a1960, a1961)
                };
                let a1963 = 0u32;
                if equal(a1962, a1963) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1966 = {
                let a1964 = v0;
                let a1965 = 2u32;
                fixed_u32::bit_and(a1964, a1965)
            };
            let a1967 = 0u32;
            if equal(a1966, a1967) {
                let a1970 = {
                    let a1968 = v0;
                    let a1969 = 1u32;
                    fixed_u32::bit_and(a1968, a1969)
                };
                let a1971 = 0u32;
                if equal(a1970, a1971) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1974 = {
                    let a1972 = v0;
                    let a1973 = 1u32;
                    fixed_u32::bit_and(a1972, a1973)
                };
                let a1975 = 0u32;
                if equal(a1974, a1975) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1978 = {
            let a1976 = v0;
            let a1977 = 4u32;
            fixed_u32::bit_and(a1976, a1977)
        };
        let a1979 = 0u32;
        if equal(a1978, a1979) {
            let a1982 = {
                let a1980 = v0;
                let a1981 = 2u32;
                fixed_u32::bit_and(a1980, a1981)
            };
            let a1983 = 0u32;
            if equal(a1982, a1983) {
                let a1986 = {
                    let a1984 = v0;
                    let a1985 = 1u32;
                    fixed_u32::bit_and(a1984, a1985)
                };
                let a1987 = 0u32;
                if equal(a1986, a1987) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1990 = {
                    let a1988 = v0;
                    let a1989 = 1u32;
                    fixed_u32::bit_and(a1988, a1989)
                };
                let a1991 = 0u32;
                if equal(a1990, a1991) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1994 = {
                let a1992 = v0;
                let a1993 = 2u32;
                fixed_u32::bit_and(a1992, a1993)
            };
            let a1995 = 0u32;
            if equal(a1994, a1995) {
                let a1998 = {
                    let a1996 = v0;
                    let a1997 = 1u32;
                    fixed_u32::bit_and(a1996, a1997)
                };
                let a1999 = 0u32;
                if equal(a1998, a1999) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2002 = {
                    let a2000 = v0;
                    let a2001 = 1u32;
                    fixed_u32::bit_and(a2000, a2001)
                };
                let a2003 = 0u32;
                if equal(a2002, a2003) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f460(v0: u32, v1: u32) -> u32 {
    let m2006 = {
        let a2004 = v0;
        let a2005 = v1;
        fixed_u32::shift_right(a2004, a2005)
    };
    match m2006 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f461(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f497(Fn18::F498, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2007, _, _, _, _, _, _, _) => {
            h2007
        }
    }, match v0.clone() {
        Adt24::C0(_, h2008, _, _, _, _, _, _) => {
            h2008
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2009, _, _, _, _, _) => {
            h2009
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2010, _, _, _, _) => {
            h2010
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2011, _, _, _) => {
            h2011
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2012, _, _) => {
            h2012
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2013, _) => {
            h2013
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2014) => {
            h2014
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f499(match v0.clone() {
        Adt24::C0(h2015, _, _, _, _, _, _, _) => {
            h2015
        }
    }, match v2.clone() {
        Adt27::C0(h2016, _, _, _, _, _, _, _, _, _) => {
            h2016
        }
    }), f499(match v0.clone() {
        Adt24::C0(_, h2017, _, _, _, _, _, _) => {
            h2017
        }
    }, match v2.clone() {
        Adt27::C0(_, h2018, _, _, _, _, _, _, _, _) => {
            h2018
        }
    }), f499(match v0.clone() {
        Adt24::C0(_, _, h2019, _, _, _, _, _) => {
            h2019
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2020, _, _, _, _, _, _, _) => {
            h2020
        }
    }), f499(match v0.clone() {
        Adt24::C0(_, _, _, h2021, _, _, _, _) => {
            h2021
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2022, _, _, _, _, _, _) => {
            h2022
        }
    }), f499(match v0.clone() {
        Adt24::C0(_, _, _, _, h2023, _, _, _) => {
            h2023
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2024, _, _, _, _, _) => {
            h2024
        }
    }), f499(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2025, _, _) => {
            h2025
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2026, _, _, _, _) => {
            h2026
        }
    }), f499(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2027, _) => {
            h2027
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2028, _, _, _) => {
            h2028
        }
    }), f499(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2029) => {
            h2029
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2030, _, _) => {
            h2030
        }
    })))
}

pub fn f462(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f500(v0.clone(), v1, v2, v3)?, f500(v0.clone(), v1, v2, {
        let a2031 = v3;
        let a2032 = 4u64;
        nat_add(a2031, a2032)?
    })?, f500(v0.clone(), v1, v2, {
        let a2033 = v3;
        let a2034 = 8u64;
        nat_add(a2033, a2034)?
    })?, f500(v0.clone(), v1, v2, {
        let a2035 = v3;
        let a2036 = 12u64;
        nat_add(a2035, a2036)?
    })?, f500(v0.clone(), v1, v2, {
        let a2037 = v3;
        let a2038 = 16u64;
        nat_add(a2037, a2038)?
    })?, f500(v0.clone(), v1, v2, {
        let a2039 = v3;
        let a2040 = 20u64;
        nat_add(a2039, a2040)?
    })?, f500(v0.clone(), v1, v2, {
        let a2041 = v3;
        let a2042 = 24u64;
        nat_add(a2041, a2042)?
    })?, f500(v0.clone(), v1, v2, {
        let a2043 = v3;
        let a2044 = 28u64;
        nat_add(a2043, a2044)?
    })?, f500(v0.clone(), v1, v2, {
        let a2045 = v3;
        let a2046 = 32u64;
        nat_add(a2045, a2046)?
    })?, f500(v0.clone(), v1, v2, {
        let a2047 = v3;
        let a2048 = 36u64;
        nat_add(a2047, a2048)?
    })?, f500(v0.clone(), v1, v2, {
        let a2049 = v3;
        let a2050 = 40u64;
        nat_add(a2049, a2050)?
    })?, f500(v0.clone(), v1, v2, {
        let a2051 = v3;
        let a2052 = 44u64;
        nat_add(a2051, a2052)?
    })?, f500(v0.clone(), v1, v2, {
        let a2053 = v3;
        let a2054 = 48u64;
        nat_add(a2053, a2054)?
    })?, f500(v0.clone(), v1, v2, {
        let a2055 = v3;
        let a2056 = 52u64;
        nat_add(a2055, a2056)?
    })?, f500(v0.clone(), v1, v2, {
        let a2057 = v3;
        let a2058 = 56u64;
        nat_add(a2057, a2058)?
    })?, f500(v0, v1, v2, {
        let a2059 = v3;
        let a2060 = 60u64;
        nat_add(a2059, a2060)?
    })?))
}

pub fn f463(p2063: List<Bytes>, v1: Bytes) -> bool {
    let m2062 = p2063;
    {
        let mut m2062 = m2062;
        loop {
            let step = {
                let v0 = m2062;
                let m2061 = v0;
                match m2061.uncons() {
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
                Ok(next) => { m2062 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f464(v0: List<Adt13>, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m2064 = v0;
    match m2064.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            if f28(f47(v3.clone(), v1.clone()), v2.clone()) {
                f464(v4, v1, v2)
            } else {
                List::cons(v3, f464(v4, v1, v2))
            }
        }
    }
}

pub fn f465(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2065 = v0;
    match m2065.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f72(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f342(f458(f125(List::cons(v1, f501(f33(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f465(v3, v1)?))
            }
        }
    }
}

pub fn f466(v0: Fn14, p2070: u64, p2071: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2069 = (p2070, p2071);
    {
        let mut m2069 = m2069;
        loop {
            let step = {
                let (v1, v2) = m2069;
                let m2066 = v1;
                if m2066 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2066 - 1;
                    let m2068 = {
                        let c2067: Fn14 = v0.clone();
                        c2067.apply(v2.clone())?
                    };
                    match m2068 {
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
                Ok(next) => { m2069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f502(v0, v1)
}

pub fn f468(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f503(v0, v1)
}

pub fn f469(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f504(v0, v1)
}

pub fn f470(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2072 = v0.clone();
        length_bytes(a2072)
    };
    let a2073 = 0u64;
    let a2074 = v1;
    if nat_lt(a2073, a2074) {
        let a2075 = f505(v0.clone())?;
        let a2076 = v1;
        if nat_eq(a2075, a2076) {
            let a2077 = 1u64;
            let a2078 = v1;
            if nat_lt(a2077, a2078) {
                let m2081 = {
                    let a2079 = v0;
                    let a2080 = 0u64;
                    index_bytes(a2079, a2080)
                };
                match m2081 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2082 = v2;
                        let a2083 = 48u8;
                        Ok(!equal(a2082, a2083))
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

pub fn f471(v0: Fn15, p2088: u64, p2089: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2087 = (p2088, p2089);
    {
        let mut m2087 = m2087;
        loop {
            let step = {
                let (v1, v2) = m2087;
                let m2084 = v1;
                if m2084 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2084 - 1;
                    let m2086 = {
                        let c2085: Fn15 = v0.clone();
                        c2085.apply(v2)?
                    };
                    match m2086 {
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
                Ok(next) => { m2087 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f506(v0, v1)
}

pub fn f473(v0: Bytes) -> R<u64> {
    let (h2090, _) = f286(Fn3::F507(v0.clone()), {
        let a2092 = {
            let a2091 = v0;
            length_bytes(a2091)
        };
        let a2093 = 1u64;
        nat_add(a2092, a2093)?
    }, 0u64)?;
    Ok(h2090)
}

pub fn f474(v0: Bytes, v1: u64) -> Bytes {
    let m2097 = {
        let a2094 = v0;
        let a2095 = v1;
        let a2096 = 1u64;
        slice_bytes(a2094, a2095, a2096)
    };
    match m2097 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f475(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f508(v1, v0, v2)
}

pub fn f476(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2142 = {
            let a2118 = {
                let a2106 = {
                    let a2100 = {
                        let a2098 = v3;
                        let a2099 = 128u8;
                        fixed_u8::bit_and(a2098, a2099)
                    };
                    let a2101 = 128u8;
                    if equal(a2100, a2101) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2107 = {
                    let a2104 = {
                        let a2102 = v3;
                        let a2103 = 64u8;
                        fixed_u8::bit_and(a2102, a2103)
                    };
                    let a2105 = 64u8;
                    if equal(a2104, a2105) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2106, a2107)?
            };
            let a2119 = {
                let a2116 = {
                    let a2110 = {
                        let a2108 = v3;
                        let a2109 = 32u8;
                        fixed_u8::bit_and(a2108, a2109)
                    };
                    let a2111 = 32u8;
                    if equal(a2110, a2111) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2117 = {
                    let a2114 = {
                        let a2112 = v3;
                        let a2113 = 16u8;
                        fixed_u8::bit_and(a2112, a2113)
                    };
                    let a2115 = 16u8;
                    if equal(a2114, a2115) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2116, a2117)?
            };
            nat_add(a2118, a2119)?
        };
        let a2143 = {
            let a2140 = {
                let a2128 = {
                    let a2122 = {
                        let a2120 = v3;
                        let a2121 = 8u8;
                        fixed_u8::bit_and(a2120, a2121)
                    };
                    let a2123 = 8u8;
                    if equal(a2122, a2123) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2129 = {
                    let a2126 = {
                        let a2124 = v3;
                        let a2125 = 4u8;
                        fixed_u8::bit_and(a2124, a2125)
                    };
                    let a2127 = 4u8;
                    if equal(a2126, a2127) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2128, a2129)?
            };
            let a2141 = {
                let a2138 = {
                    let a2132 = {
                        let a2130 = v3;
                        let a2131 = 2u8;
                        fixed_u8::bit_and(a2130, a2131)
                    };
                    let a2133 = 2u8;
                    if equal(a2132, a2133) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2139 = {
                    let a2136 = {
                        let a2134 = v3;
                        let a2135 = 1u8;
                        fixed_u8::bit_and(a2134, a2135)
                    };
                    let a2137 = 1u8;
                    if equal(a2136, a2137) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2138, a2139)?
            };
            nat_add(a2140, a2141)?
        };
        nat_add(a2142, a2143)?
    };
    let a2144 = v4;
    let a2145 = 34u64;
    if nat_eq(a2144, a2145) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2146 = v4;
        let a2147 = 92u64;
        if nat_eq(a2146, a2147) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2148 = v4;
            let a2149 = 8u64;
            if nat_eq(a2148, a2149) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2150 = v4;
                let a2151 = 12u64;
                if nat_eq(a2150, a2151) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2152 = v4;
                    let a2153 = 10u64;
                    if nat_eq(a2152, a2153) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2154 = v4;
                        let a2155 = 13u64;
                        if nat_eq(a2154, a2155) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2156 = v4;
                            let a2157 = 9u64;
                            if nat_eq(a2156, a2157) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2158 = v4;
                                let a2159 = 32u64;
                                if nat_lt(a2158, a2159) {
                                    let a2160 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2161 = f509(v2)?;
                                    Ok(append_bytes(a2160, a2161))
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

pub fn f477(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f510(Fn19::F511, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f478(v0: Fn16, p2165: List<List<(Bytes, Bytes)>>, p2166: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2164 = (p2165, p2166);
    {
        let mut m2164 = m2164;
        loop {
            let step = {
                let (v1, v2) = m2164;
                let m2162 = v2;
                match m2162.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2163: Fn16 = v0.clone();
                            c2163.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2164 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f480(v0: Fn17, p2171: u64, p2172: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2170 = (p2171, p2172);
    {
        let mut m2170 = m2170;
        loop {
            let step = {
                let (v1, v2) = m2170;
                let m2167 = v1;
                if m2167 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2167 - 1;
                    let m2169 = {
                        let c2168: Fn17 = v0.clone();
                        c2168.apply(v2.clone())?
                    };
                    match m2169 {
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
                Ok(next) => { m2170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f512(v0)
}

pub fn f482(v0: u8) -> bool {
    let a2173 = v0;
    let a2174 = 32u8;
    if if if equal(a2173, a2174) {
        true
    } else {
        let a2175 = v0;
        let a2176 = 9u8;
        equal(a2175, a2176)
    } {
        true
    } else {
        let a2177 = v0;
        let a2178 = 10u8;
        equal(a2177, a2178)
    } {
        true
    } else {
        let a2179 = v0;
        let a2180 = 12u8;
        if equal(a2179, a2180) {
            true
        } else {
            let a2181 = v0;
            let a2182 = 13u8;
            equal(a2181, a2182)
        }
    }
}

pub fn f483(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f386({
        let (_, h2183) = {
            let (h2184, _) = f513(Fn20::F514, {
                let a2186 = {
                    let a2185 = v0.clone();
                    length_list(a2185)
                };
                let a2187 = 1u64;
                nat_add(a2186, a2187)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2184
        };
        h2183
    }))
}

pub fn f484(v0: Bytes, v1: Bytes, p2200: u64, p2201: u64) -> R<bool> {
    let m2199 = (p2200, p2201);
    {
        let mut m2199 = m2199;
        loop {
            let step = {
                let (v2, v3) = m2199;
                let m2188 = v3;
                if m2188 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m2188 - 1;
                    let m2191 = {
                        let a2189 = v1.clone();
                        let a2190 = v2;
                        index_bytes(a2189, a2190)
                    };
                    match m2191 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m2194 = {
                                let a2192 = v0.clone();
                                let a2193 = v2;
                                index_bytes(a2192, a2193)
                            };
                            match m2194 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a2195 = v5;
                                    let a2196 = v6;
                                    if equal(a2195, a2196) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a2197 = v2;
                                            let a2198 = 1u64;
                                            nat_add(a2197, a2198)?
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
                Ok(next) => { m2199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f486(v0: List<Adt13>) -> List<Bytes> {
    let m2202 = v0;
    match m2202.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2203 = f79(v1, f8(Str::lit("std")));
            match m2203 {
                None => {
                    f486(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f486(v2))
                }
            }
        }
    }
}

pub fn f487(v0: List<Adt13>) -> List<Bytes> {
    let m2204 = v0;
    match m2204.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2205 = f78(f79(v1, f8(Str::lit("req"))));
            match m2205 {
                None => {
                    f487(v2)
                }
                Some(v3) => {
                    List::cons(v3, f487(v2))
                }
            }
        }
    }
}

pub fn f488(v0: Adt18) -> List<(Bytes, Adt18)> {
    let m2206 = v0;
    match m2206 {
        Adt18::C0 => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C1(_, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C2(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C3(_, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C4(_, _, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C5(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C6(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C7(r2207) => {
            let v4 = (*r2207).clone();
            f488(v4)
        }
        Adt18::C8 => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C9(_, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C10(v1, r2208, r2209) => {
            let v2 = (*r2208).clone();
            let v3 = (*r2209).clone();
            List::cons((v1, v2), f488(v3))
        }
    }
}

pub fn f489(p2214: List<(Bytes, Adt18)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt18)>> {
    let m2213 = p2214;
    {
        let mut m2213 = m2213;
        loop {
            let step = {
                let v0 = m2213;
                let m2210 = v0;
                match m2210.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(Ok(None::<(Bytes, Adt18)>))
                    }
                    Some((v3, v4)) => {
                        if f515(f516({
                            let (_, h2211) = v3.clone();
                            h2211
                        }, List::<Bytes>::nil())?, f491({
                            let (h2212, _) = v3.clone();
                            h2212
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(v0: List<(Bytes, Adt18)>, v1: Bytes) -> List<(Bytes, Adt18)> {
    let m2215 = v0;
    match m2215.uncons() {
        None => {
            List::<(Bytes, Adt18)>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h2216, _) = v2.clone();
                h2216
            }, v1.clone()) {
                f490(v3, v1)
            } else {
                List::cons(v2, f490(v3, v1))
            }
        }
    }
}

pub fn f491(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f328(v0)?;
    if f41(v1.clone(), f517()) {
        f518(v1)
    } else {
        Ok(v1)
    }
}

pub fn f492() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f493() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f494(v0: Bytes, v1: Adt18) -> R<Result<Adt28, Bytes>> {
    f519(v0, f520(v1))
}

pub fn f495(v0: Adt28) -> List<Adt25> {
    let m2217 = v0;
    match m2217 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f496(v0: Adt28) -> Bytes {
    let m2218 = v0;
    match m2218 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f497(v0: Fn18, p2222: u64, p2223: Adt27) -> R<Adt27> {
    let m2221 = (p2222, p2223);
    {
        let mut m2221 = m2221;
        loop {
            let step = {
                let (v1, v2) = m2221;
                let m2219 = v1;
                if m2219 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2219 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2220: Fn18 = v0.clone();
                        c2220.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2221 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(v0: Adt27) -> R<Adt27> {
    f521(v0)
}

pub fn f499(v0: u32, v1: u32) -> u32 {
    let m2226 = {
        let a2224 = f522(v0);
        let a2225 = f522(v1);
        fixed_u64::checked_add(a2224, a2225)
    };
    match m2226 {
        None => {
            0u32
        }
        Some(v2) => {
            f523(v2)
        }
    }
}

pub fn f500(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2229 = {
        let a2227 = v3;
        let a2228 = 8u64;
        nat_add(a2227, a2228)?
    };
    let a2230 = v2;
    if nat_eq(a2229, a2230) {
        Ok(f524({
            let a2233 = {
                let a2231 = v1;
                let a2232 = 8u64;
                nat_mul(a2231, a2232)?
            };
            let a2234 = 4294967296u64;
            let a2235 = 0u64;
            nat_quot(a2233, a2234, a2235)
        }))
    } else {
        let a2238 = {
            let a2236 = v3;
            let a2237 = 4u64;
            nat_add(a2236, a2237)?
        };
        let a2239 = v2;
        if nat_eq(a2238, a2239) {
            Ok(f524({
                let a2242 = {
                    let a2240 = v1;
                    let a2241 = 8u64;
                    nat_mul(a2240, a2241)?
                };
                let a2243 = 4294967296u64;
                let a2244 = 0u64;
                nat_rem(a2242, a2243, a2244)
            }))
        } else {
            Ok(f525(f526(v0.clone(), v1, v3), f526(v0.clone(), v1, {
                let a2245 = v3;
                let a2246 = 1u64;
                nat_add(a2245, a2246)?
            }), f526(v0.clone(), v1, {
                let a2247 = v3;
                let a2248 = 2u64;
                nat_add(a2247, a2248)?
            }), f526(v0, v1, {
                let a2249 = v3;
                let a2250 = 3u64;
                nat_add(a2249, a2250)?
            })))
        }
    }
}

pub fn f501(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2251 = v0;
    match m2251.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f501(v3, false))
            } else {
                f501(v3, true)
            }
        }
    }
}

pub fn f502(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2252, _) = v1.clone();
        h2252
    };
    let v3: Bytes = {
        let (h2253, _) = {
            let (_, h2254) = v1.clone();
            h2254
        };
        h2253
    };
    let a2257 = {
        let (_, h2255) = {
            let (_, h2256) = v1;
            h2256
        };
        h2255
    };
    let a2258 = 0u64;
    if nat_eq(a2257, a2258) {
        let v4: u64 = f350(v0.clone(), v2)?;
        let a2259 = v4;
        let a2260 = 34u64;
        if nat_eq(a2259, a2260) {
            Ok(Some(({
                let a2261 = v2;
                let a2262 = 1u64;
                nat_add(a2261, a2262)?
            }, (v3, 1u64))))
        } else {
            let a2263 = v4;
            let a2264 = 92u64;
            if nat_eq(a2263, a2264) {
                f527(v0, v2, v3)
            } else {
                let a2265 = v4;
                let a2266 = 32u64;
                if if nat_lt(a2265, a2266) {
                    true
                } else {
                    let a2267 = v4;
                    let a2268 = 256u64;
                    nat_eq(a2267, a2268)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2269 = v2;
                        let a2270 = 1u64;
                        nat_add(a2269, a2270)?
                    }, ({
                        let a2271 = v3;
                        let a2272 = f35(v0, v2, 1u64);
                        append_bytes(a2271, a2272)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f503(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f350(v0, v1)?;
    let a2273 = 47u64;
    let a2274 = v2;
    if if nat_lt(a2273, a2274) {
        let a2275 = v2;
        let a2276 = 58u64;
        nat_lt(a2275, a2276)
    } else {
        false
    } {
        Ok(Some({
            let a2277 = v1;
            let a2278 = 1u64;
            nat_add(a2277, a2278)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f504(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2279 = f350(v0, v1)?;
    let a2280 = 48u64;
    if nat_eq(a2279, a2280) {
        Ok(Some({
            let a2281 = v1;
            let a2282 = 1u64;
            nat_add(a2281, a2282)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f505(v0: Bytes) -> R<u64> {
    let (h2283, _) = f286(Fn3::F528(v0.clone()), {
        let a2284 = v0;
        length_bytes(a2284)
    }, 0u64)?;
    Ok(h2283)
}

pub fn f506(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2288 = {
        let a2286 = v0;
        let a2287 = {
            let (h2285, _) = v1;
            h2285
        };
        index_bytes(a2286, a2287)
    };
    match m2288 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2290 = {
                    let (h2289, _) = v1;
                    h2289
                };
                let a2291 = 1u64;
                nat_add(a2290, a2291)?
            }, {
                let a2295 = {
                    let a2293 = {
                        let (_, h2292) = v1;
                        h2292
                    };
                    let a2294 = 10u64;
                    nat_mul(a2293, a2294)?
                };
                let a2296 = f529(v2)?;
                nat_add(a2295, a2296)?
            })))
        }
    }
}

pub fn f507(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f530(v0, v1)
}

pub fn f508(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2297, _) = v2;
        h2297
    };
    let a2298 = v3;
    let a2299 = 0u64;
    if nat_eq(a2298, a2299) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2300 = v3;
            let a2301 = v1;
            let a2302 = 0u64;
            nat_quot(a2300, a2301, a2302)
        }, {
            let a2307 = f474(v0, {
                let a2303 = v3;
                let a2304 = v1;
                let a2305 = 0u64;
                nat_rem(a2303, a2304, a2305)
            });
            let a2308 = {
                let (_, h2306) = v2;
                h2306
            };
            append_bytes(a2307, a2308)
        }))
    }
}

pub fn f509(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2353 = {
            let a2329 = {
                let a2317 = {
                    let a2311 = {
                        let a2309 = v1;
                        let a2310 = 128u8;
                        fixed_u8::bit_and(a2309, a2310)
                    };
                    let a2312 = 128u8;
                    if equal(a2311, a2312) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2318 = {
                    let a2315 = {
                        let a2313 = v1;
                        let a2314 = 64u8;
                        fixed_u8::bit_and(a2313, a2314)
                    };
                    let a2316 = 64u8;
                    if equal(a2315, a2316) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2317, a2318)?
            };
            let a2330 = {
                let a2327 = {
                    let a2321 = {
                        let a2319 = v1;
                        let a2320 = 32u8;
                        fixed_u8::bit_and(a2319, a2320)
                    };
                    let a2322 = 32u8;
                    if equal(a2321, a2322) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2328 = {
                    let a2325 = {
                        let a2323 = v1;
                        let a2324 = 16u8;
                        fixed_u8::bit_and(a2323, a2324)
                    };
                    let a2326 = 16u8;
                    if equal(a2325, a2326) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2327, a2328)?
            };
            nat_add(a2329, a2330)?
        };
        let a2354 = {
            let a2351 = {
                let a2339 = {
                    let a2333 = {
                        let a2331 = v1;
                        let a2332 = 8u8;
                        fixed_u8::bit_and(a2331, a2332)
                    };
                    let a2334 = 8u8;
                    if equal(a2333, a2334) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2340 = {
                    let a2337 = {
                        let a2335 = v1;
                        let a2336 = 4u8;
                        fixed_u8::bit_and(a2335, a2336)
                    };
                    let a2338 = 4u8;
                    if equal(a2337, a2338) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2339, a2340)?
            };
            let a2352 = {
                let a2349 = {
                    let a2343 = {
                        let a2341 = v1;
                        let a2342 = 2u8;
                        fixed_u8::bit_and(a2341, a2342)
                    };
                    let a2344 = 2u8;
                    if equal(a2343, a2344) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2350 = {
                    let a2347 = {
                        let a2345 = v1;
                        let a2346 = 1u8;
                        fixed_u8::bit_and(a2345, a2346)
                    };
                    let a2348 = 1u8;
                    if equal(a2347, a2348) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2349, a2350)?
            };
            nat_add(a2351, a2352)?
        };
        nat_add(a2353, a2354)?
    };
    let a2361 = f474(f531(), {
        let a2355 = v2;
        let a2356 = 16u64;
        let a2357 = 0u64;
        nat_quot(a2355, a2356, a2357)
    });
    let a2362 = f474(f531(), {
        let a2358 = v2;
        let a2359 = 16u64;
        let a2360 = 0u64;
        nat_rem(a2358, a2359, a2360)
    });
    Ok(append_bytes(a2361, a2362))
}

pub fn f510(v0: Fn19, p2366: List<List<(Bytes, Bytes)>>, p2367: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2365 = (p2366, p2367);
    {
        let mut m2365 = m2365;
        loop {
            let step = {
                let (v1, v2) = m2365;
                let m2363 = v2;
                match m2363.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2364: Fn19 = v0.clone();
                            c2364.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f512(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2369 = {
        let a2368 = v0.clone();
        length_list(a2368)
    };
    let a2370 = 1u64;
    if nat_le(a2369, a2370) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f532(v0)?))
    }
}

pub fn f513(v0: Fn20, p2375: u64, p2376: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2374 = (p2375, p2376);
    {
        let mut m2374 = m2374;
        loop {
            let step = {
                let (v1, v2) = m2374;
                let m2371 = v1;
                if m2371 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2371 - 1;
                    let m2373 = {
                        let c2372: Fn20 = v0.clone();
                        c2372.apply(v2.clone())?
                    };
                    match m2373 {
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
                Ok(next) => { m2374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f514(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f533(v0)
}

pub fn f515(p2379: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2378 = p2379;
    {
        let mut m2378 = m2378;
        loop {
            let step = {
                let v0 = m2378;
                let m2377 = v0;
                match m2377.uncons() {
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
                Ok(next) => { m2378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: Adt18, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2380 = v0;
    match m2380 {
        Adt18::C0 => {
            Ok(v1)
        }
        Adt18::C1(_, _) => {
            Ok(v1)
        }
        Adt18::C2(_) => {
            Ok(v1)
        }
        Adt18::C3(_, _) => {
            Ok(v1)
        }
        Adt18::C4(_, _, _) => {
            Ok(v1)
        }
        Adt18::C5(_) => {
            Ok(v1)
        }
        Adt18::C6(r2383) => {
            let v2 = (*r2383).clone();
            f516(v2, v1)
        }
        Adt18::C7(r2384) => {
            let v3 = (*r2384).clone();
            f516(v3, v1)
        }
        Adt18::C8 => {
            Ok(v1)
        }
        Adt18::C9(r2385, r2386) => {
            let v4 = (*r2385).clone();
            let v5 = (*r2386).clone();
            f516(v4, f516(v5, v1)?)
        }
        Adt18::C10(v6, r2387, r2388) => {
            let v7 = (*r2387).clone();
            let v8 = (*r2388).clone();
            let v9: List<Bytes> = f516(v7.clone(), f516(v8, v1)?)?;
            if f28(v6, f8(Str::lit("$ref"))) {
                let m2381 = f334(Some(v7));
                match m2381 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2382 = f534(v10)?;
                        match m2382 {
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

pub fn f517() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f518(v0: Bytes) -> R<Bytes> {
    let m2391 = {
        let a2389 = v0.clone();
        let a2390 = 0u64;
        index_bytes(a2389, a2390)
    };
    match m2391 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2397 = {
                let a2392 = f8(Str::lit("u"));
                let a2393 = f535(8u64, f62(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2392, a2393)
            };
            let a2398 = f35(v0.clone(), 1u64, {
                let a2395 = {
                    let a2394 = v0;
                    length_bytes(a2394)
                };
                let a2396 = 1u64;
                nat_sub(a2395, a2396)
            });
            Ok(append_bytes(a2397, a2398))
        }
    }
}

pub fn f519(v0: Bytes, v1: Adt18) -> R<Result<Adt28, Bytes>> {
    let m2399 = f242(v1.clone(), f8(Str::lit("enum")));
    match m2399 {
        None => {
            let m2401 = f242(v1.clone(), f8(Str::lit("oneOf")));
            match m2401 {
                None => {
                    if f540(v1.clone(), v0.clone())? {
                        let m2402 = f541(v0.clone(), f451(f242(v1.clone(), f8(Str::lit("properties")))), f246(f247(f242(v1, f8(Str::lit("required"))))), true)?;
                        match m2402 {
                            Ok(v7) => {
                                Ok(f538(f542(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f342(f543(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f544(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2403 = f451(f242(v1.clone(), f8(Str::lit("properties"))));
                        match m2403.uncons() {
                            None => {
                                if f545(v1.clone()) {
                                    Ok(f538(f546(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2406 = f547(v1, {
                                        let a2404 = v0.clone();
                                        let a2405 = f8(Str::lit("Value"));
                                        append_bytes(a2404, a2405)
                                    })?;
                                    match m2406 {
                                        Ok(v9) => {
                                            Ok(f538(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f496(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f495(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2407 = f541(v0.clone(), f451(f242(v1.clone(), f8(Str::lit("properties")))), f246(f247(f242(v1, f8(Str::lit("required"))))), false)?;
                                match m2407 {
                                    Ok(v13) => {
                                        Ok(f538(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2408 = f543(v13.clone());
                                            match m2408.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f342(f543(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f544(v13)))
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
                    f539(v0, f335(v5))
                }
            }
        }
        Some(v2) => {
            let m2400 = f536(f335(v2), List::<Bytes>::nil())?;
            match m2400 {
                Ok(v4) => {
                    Ok(f538(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f342(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f537(v3, v0))
                }
            }
        }
    }
}

pub fn f520(v0: Adt18) -> Adt18 {
    let v2: Adt18 = {
        let m2409 = f242(v0.clone(), f8(Str::lit("anyOf")));
        match m2409 {
            None => {
                v0
            }
            Some(v1) => {
                if f548(f335(v1.clone())) {
                    f549(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f550(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f551(f335(v1.clone()))
                    } {
                        v0
                    } else {
                        f552(f549(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt18 = {
        let m2410 = f242(v2.clone(), f8(Str::lit("oneOf")));
        match m2410 {
            None => {
                v2
            }
            Some(v3) => {
                if f548(f335(v3)) {
                    f549(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2411 = f242(v4.clone(), f8(Str::lit("allOf")));
    match m2411 {
        None => {
            v4
        }
        Some(v5) => {
            if f553(f335(v5)) {
                f549(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f521(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f554(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2412) => {
            h2412
        }
    });
    let v2: u32 = f499(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2413, _, _) => {
            h2413
        }
    }, f499(f555(match v0.clone() {
        Adt27::C0(_, _, _, _, h2414, _, _, _, _, _) => {
            h2414
        }
    }), f499(f556(match v0.clone() {
        Adt27::C0(_, _, _, _, h2415, _, _, _, _, _) => {
            h2415
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2416, _, _, _, _) => {
            h2416
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2417, _, _, _) => {
            h2417
        }
    }), f499(v1, {
        let m2420 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2419, _) => {
                h2419
            }
        };
        match m2420 {
            Adt26::C0(h2418, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2418
            }
        }
    }))));
    let v3: u32 = f499(f557(match v0.clone() {
        Adt27::C0(h2421, _, _, _, _, _, _, _, _, _) => {
            h2421
        }
    }), f558(match v0.clone() {
        Adt27::C0(h2422, _, _, _, _, _, _, _, _, _) => {
            h2422
        }
    }, match v0.clone() {
        Adt27::C0(_, h2423, _, _, _, _, _, _, _, _) => {
            h2423
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2424, _, _, _, _, _, _, _) => {
            h2424
        }
    }));
    Ok(Adt27::C0(f499(v2, v3), match v0.clone() {
        Adt27::C0(h2425, _, _, _, _, _, _, _, _, _) => {
            h2425
        }
    }, match v0.clone() {
        Adt27::C0(_, h2426, _, _, _, _, _, _, _, _) => {
            h2426
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2427, _, _, _, _, _, _, _) => {
            h2427
        }
    }, f499(match v0.clone() {
        Adt27::C0(_, _, _, h2428, _, _, _, _, _, _) => {
            h2428
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2429, _, _, _, _, _) => {
            h2429
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2430, _, _, _, _) => {
            h2430
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2431, _, _, _) => {
            h2431
        }
    }, f559(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2432, _) => {
            h2432
        }
    }), {
        let a2434 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2433) => {
                h2433
            }
        };
        let a2435 = 1u64;
        nat_add(a2434, a2435)?
    }))
}

pub fn f522(v0: u32) -> u64 {
    let m2437 = {
        let a2436 = v0;
        fixed_u64::convert(i128::from(a2436))
    };
    match m2437 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f523(v0: u64) -> u32 {
    let m2441 = {
        let a2440 = {
            let a2438 = v0;
            let a2439 = 4294967295u64;
            fixed_u64::bit_and(a2438, a2439)
        };
        fixed_u32::convert(i128::from(a2440))
    };
    match m2441 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f524(v0: u64) -> u32 {
    let (_, h2442) = {
        let (_, h2443) = f560(Fn21::F561, 32u64, (v0, (1u32, 0u32)));
        h2443
    };
    h2442
}

pub fn f525(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2448 = f562(v0, 24u32);
    let a2449 = {
        let a2446 = f562(v1, 16u32);
        let a2447 = {
            let a2444 = f562(v2, 8u32);
            let a2445 = v3;
            fixed_u32::bit_or(a2444, a2445)
        };
        fixed_u32::bit_or(a2446, a2447)
    };
    fixed_u32::bit_or(a2448, a2449)
}

pub fn f526(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2450 = v2;
    let a2451 = v1;
    if nat_lt(a2450, a2451) {
        let m2454 = {
            let a2452 = v0;
            let a2453 = v2;
            index_bytes(a2452, a2453)
        };
        match m2454 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2456 = {
                    let a2455 = v3;
                    fixed_u32::convert(i128::from(a2455))
                };
                match m2456 {
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
        let a2457 = v2;
        let a2458 = v1;
        if nat_eq(a2457, a2458) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f527(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f350(v0.clone(), {
        let a2459 = v1;
        let a2460 = 1u64;
        nat_add(a2459, a2460)?
    })?;
    let v4: Bytes = {
        let a2461 = v3;
        let a2462 = 117u64;
        if nat_eq(a2461, a2462) {
            f563(v0, v1)?
        } else {
            f564(v3)
        }
    };
    let a2463 = v4.clone();
    let a2464 = Bytes::lit(&[]);
    if equal(a2463, a2464) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2465 = v3;
            let a2466 = 117u64;
            if nat_eq(a2465, a2466) {
                let a2467 = v1;
                let a2468 = 6u64;
                nat_add(a2467, a2468)?
            } else {
                let a2469 = v1;
                let a2470 = 2u64;
                nat_add(a2469, a2470)?
            }
        }, ({
            let a2471 = v2;
            let a2472 = v4;
            append_bytes(a2471, a2472)
        }, 0u64))))
    }
}

pub fn f528(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2475 = {
        let a2473 = v0;
        let a2474 = v1;
        index_bytes(a2473, a2474)
    };
    match m2475 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f565(v2)? {
                Ok(Some({
                    let a2476 = v1;
                    let a2477 = 1u64;
                    nat_add(a2476, a2477)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f529(v0: u8) -> R<u64> {
    let a2524 = {
        let v1: u8 = v0;
        let a2522 = {
            let a2498 = {
                let a2486 = {
                    let a2480 = {
                        let a2478 = v1;
                        let a2479 = 128u8;
                        fixed_u8::bit_and(a2478, a2479)
                    };
                    let a2481 = 128u8;
                    if equal(a2480, a2481) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2487 = {
                    let a2484 = {
                        let a2482 = v1;
                        let a2483 = 64u8;
                        fixed_u8::bit_and(a2482, a2483)
                    };
                    let a2485 = 64u8;
                    if equal(a2484, a2485) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2486, a2487)?
            };
            let a2499 = {
                let a2496 = {
                    let a2490 = {
                        let a2488 = v1;
                        let a2489 = 32u8;
                        fixed_u8::bit_and(a2488, a2489)
                    };
                    let a2491 = 32u8;
                    if equal(a2490, a2491) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2497 = {
                    let a2494 = {
                        let a2492 = v1;
                        let a2493 = 16u8;
                        fixed_u8::bit_and(a2492, a2493)
                    };
                    let a2495 = 16u8;
                    if equal(a2494, a2495) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2496, a2497)?
            };
            nat_add(a2498, a2499)?
        };
        let a2523 = {
            let a2520 = {
                let a2508 = {
                    let a2502 = {
                        let a2500 = v1;
                        let a2501 = 8u8;
                        fixed_u8::bit_and(a2500, a2501)
                    };
                    let a2503 = 8u8;
                    if equal(a2502, a2503) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2509 = {
                    let a2506 = {
                        let a2504 = v1;
                        let a2505 = 4u8;
                        fixed_u8::bit_and(a2504, a2505)
                    };
                    let a2507 = 4u8;
                    if equal(a2506, a2507) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2508, a2509)?
            };
            let a2521 = {
                let a2518 = {
                    let a2512 = {
                        let a2510 = v1;
                        let a2511 = 2u8;
                        fixed_u8::bit_and(a2510, a2511)
                    };
                    let a2513 = 2u8;
                    if equal(a2512, a2513) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2519 = {
                    let a2516 = {
                        let a2514 = v1;
                        let a2515 = 1u8;
                        fixed_u8::bit_and(a2514, a2515)
                    };
                    let a2517 = 1u8;
                    if equal(a2516, a2517) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2518, a2519)?
            };
            nat_add(a2520, a2521)?
        };
        nat_add(a2522, a2523)?
    };
    let a2525 = 48u64;
    Ok(nat_sub(a2524, a2525))
}

pub fn f530(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2526 = v0.clone();
        length_bytes(a2526)
    };
    let a2527 = v1;
    let a2528 = v2;
    if if nat_lt(a2527, a2528) {
        let a2533 = f350(v0, {
            let a2531 = v2;
            let a2532 = {
                let a2529 = v1;
                let a2530 = 1u64;
                nat_add(a2529, a2530)?
            };
            nat_sub(a2531, a2532)
        })?;
        let a2534 = 48u64;
        nat_eq(a2533, a2534)
    } else {
        false
    } {
        Ok(Some({
            let a2535 = v1;
            let a2536 = 1u64;
            nat_add(a2535, a2536)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f531() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f532(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f477({
        let (_, h2537) = {
            let (h2538, _) = f566(Fn22::F567, {
                let a2540 = {
                    let a2539 = v0.clone();
                    length_list(a2539)
                };
                let a2541 = 1u64;
                nat_add(a2540, a2541)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2538
        };
        h2537
    }))
}

pub fn f533(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2542, _) = v0;
        h2542
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2543) = v0;
        h2543
    };
    let m2544 = v1;
    match m2544.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2545 = v4;
            match m2545.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f568(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f534(v0: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f491(f35(v0.clone(), 8u64, {
            let a2547 = {
                let a2546 = v0;
                length_bytes(a2546)
            };
            let a2548 = 8u64;
            nat_sub(a2547, a2548)
        }))?))
    } else {
        if f72(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f491(f35(v0.clone(), 14u64, {
                let a2550 = {
                    let a2549 = v0;
                    length_bytes(a2549)
                };
                let a2551 = 14u64;
                nat_sub(a2550, a2551)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f535(p2568: u64, p2569: u64, p2570: u64, p2571: Bytes) -> Bytes {
    let m2567 = (p2568, (p2569, (p2570, p2571)));
    {
        let mut m2567 = m2567;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2567;
                let m2552 = v0;
                if m2552 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2552 - 1;
                    if f352({
                        let a2553 = v1;
                        let a2554 = 0u64;
                        nat_eq(a2553, a2554)
                    }, {
                        let a2555 = v2;
                        let a2556 = 0u64;
                        nat_eq(a2555, a2556)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2557 = v1;
                            let a2558 = 16u64;
                            let a2559 = 0u64;
                            nat_quot(a2557, a2558, a2559)
                        }, ({
                            let a2560 = v2;
                            let a2561 = 1u64;
                            nat_sub(a2560, a2561)
                        }, {
                            let a2565 = f569({
                                let a2562 = v1;
                                let a2563 = 16u64;
                                let a2564 = 0u64;
                                nat_rem(a2562, a2563, a2564)
                            });
                            let a2566 = v3;
                            append_bytes(a2565, a2566)
                        }))))
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

pub fn f536(v0: List<Adt18>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f570(f571(f246(v0.clone()))?, v0, v1)
}

pub fn f537(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f538(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f539(v0: Bytes, v1: List<Adt18>) -> R<Result<Adt28, Bytes>> {
    let m2572 = v1.clone();
    match m2572.uncons() {
        None => {
            Ok(f537(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2573 = f572(f451(f242(v2, f8(Str::lit("properties")))), v1.clone());
            match m2573.uncons() {
                None => {
                    let m2574 = f573(f571(f574(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2574 {
                        Ok(v5) => {
                            Ok(f538(f542(v0, f543(v5.clone())), f544(v5)))
                        }
                        Err(v4) => {
                            Ok(f537(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2575 = f575(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2575 {
                        Ok(v9) => {
                            Ok(f538(f542(v0, f543(v9.clone())), f544(v9)))
                        }
                        Err(v8) => {
                            Ok(f537(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f540(v0: Adt18, v1: Bytes) -> R<bool> {
    let m2576 = v0;
    match m2576 {
        Adt18::C0 => {
            Ok(false)
        }
        Adt18::C1(_, _) => {
            Ok(false)
        }
        Adt18::C2(_) => {
            Ok(false)
        }
        Adt18::C3(_, _) => {
            Ok(false)
        }
        Adt18::C4(_, _, _) => {
            Ok(false)
        }
        Adt18::C5(_) => {
            Ok(false)
        }
        Adt18::C6(r2577) => {
            let v2 = (*r2577).clone();
            f540(v2, v1)
        }
        Adt18::C7(r2578) => {
            let v3 = (*r2578).clone();
            f540(v3, v1)
        }
        Adt18::C8 => {
            Ok(false)
        }
        Adt18::C9(r2579, r2580) => {
            let v4 = (*r2579).clone();
            let v5 = (*r2580).clone();
            if f540(v4, v1.clone())? {
                Ok(true)
            } else {
                f540(v5, v1)
            }
        }
        Adt18::C10(v6, r2581, r2582) => {
            let v7 = (*r2581).clone();
            let v8 = (*r2582).clone();
            if if f28(v6, f8(Str::lit("$ref"))) {
                f576(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f540(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f540(v8, v1)
                }
            }
        }
    }
}

pub fn f541(v0: Bytes, v1: List<(Bytes, Adt18)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f577(f571(f578(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f542(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f342(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f543(v0: Adt29) -> List<Bytes> {
    let m2583 = v0;
    match m2583 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f544(v0: Adt29) -> List<Adt25> {
    let m2584 = v0;
    match m2584 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f545(v0: Adt18) -> bool {
    let m2585 = f451(f242(v0.clone(), f8(Str::lit("properties"))));
    match m2585.uncons() {
        None => {
            let m2586 = f579(v0);
            match m2586 {
                None => {
                    false
                }
                Some(v3) => {
                    f580(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f546(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f547(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2587 = f581(v0.clone());
    match m2587 {
        None => {
            if f28(f245(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2588 = f242(v0, f8(Str::lit("items")));
                match m2588 {
                    None => {
                        Ok(f582(f8(Str::lit("List")), f584()))
                    }
                    Some(v3) => {
                        Ok(f582(f8(Str::lit("List")), f583(v3, {
                            let a2589 = v1;
                            let a2590 = f8(Str::lit("Item"));
                            append_bytes(a2589, a2590)
                        })?))
                    }
                }
            } else {
                f583(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f582(f8(Str::lit("Option")), f583(v2, v1)?))
        }
    }
}

pub fn f548(p2593: List<Adt18>) -> bool {
    let m2592 = p2593;
    {
        let mut m2592 = m2592;
        loop {
            let step = {
                let v0 = m2592;
                let m2591 = v0;
                match m2591.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f585(f488(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt18>, bool>(v2)
                        } else {
                            Err::<List<Adt18>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f549(v0: Adt18, v1: Bytes) -> Adt18 {
    let m2594 = v0;
    match m2594 {
        Adt18::C0 => {
            Adt18::C0
        }
        Adt18::C1(v6, r2595) => {
            let v7 = (*r2595).clone();
            Adt18::C1(v6, std::rc::Rc::new(v7))
        }
        Adt18::C2(v8) => {
            Adt18::C2(v8)
        }
        Adt18::C3(v12, v13) => {
            Adt18::C3(v12, v13)
        }
        Adt18::C4(v9, v10, v11) => {
            Adt18::C4(v9, v10, v11)
        }
        Adt18::C5(v14) => {
            Adt18::C5(v14)
        }
        Adt18::C6(r2596) => {
            let v15 = (*r2596).clone();
            Adt18::C6(std::rc::Rc::new(v15))
        }
        Adt18::C7(r2597) => {
            let v5 = (*r2597).clone();
            Adt18::C7(std::rc::Rc::new(f549(v5, v1)))
        }
        Adt18::C8 => {
            Adt18::C8
        }
        Adt18::C9(r2598, r2599) => {
            let v16 = (*r2598).clone();
            let v17 = (*r2599).clone();
            Adt18::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt18::C10(v2, r2600, r2601) => {
            let v3 = (*r2600).clone();
            let v4 = (*r2601).clone();
            if f28(v2.clone(), v1.clone()) {
                f549(v4, v1)
            } else {
                Adt18::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f549(v4, v1)))
            }
        }
    }
}

pub fn f550(v0: Adt18, v1: Bytes) -> bool {
    let m2602 = f242(v0, v1);
    match m2602 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f551(v0: List<Adt18>) -> bool {
    let m2603 = v0.clone();
    match m2603.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2604 = f572(f451(f242(v1, f8(Str::lit("properties")))), v0.clone());
            match m2604.uncons() {
                None => {
                    f587(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f586(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f552(v0: Adt18, v1: Bytes, v2: Adt18) -> Adt18 {
    let m2605 = v0;
    match m2605 {
        Adt18::C0 => {
            Adt18::C0
        }
        Adt18::C1(v7, r2606) => {
            let v8 = (*r2606).clone();
            Adt18::C1(v7, std::rc::Rc::new(v8))
        }
        Adt18::C2(v9) => {
            Adt18::C2(v9)
        }
        Adt18::C3(v13, v14) => {
            Adt18::C3(v13, v14)
        }
        Adt18::C4(v10, v11, v12) => {
            Adt18::C4(v10, v11, v12)
        }
        Adt18::C5(v15) => {
            Adt18::C5(v15)
        }
        Adt18::C6(r2607) => {
            let v16 = (*r2607).clone();
            Adt18::C6(std::rc::Rc::new(v16))
        }
        Adt18::C7(r2608) => {
            let v3 = (*r2608).clone();
            Adt18::C7(std::rc::Rc::new(Adt18::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt18::C8 => {
            Adt18::C8
        }
        Adt18::C9(r2609, r2610) => {
            let v17 = (*r2609).clone();
            let v18 = (*r2610).clone();
            Adt18::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt18::C10(v4, r2611, r2612) => {
            let v5 = (*r2611).clone();
            let v6 = (*r2612).clone();
            Adt18::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f553(p2615: List<Adt18>) -> bool {
    let m2614 = p2615;
    {
        let mut m2614 = m2614;
        loop {
            let step = {
                let v0 = m2614;
                let m2613 = v0;
                match m2613.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f588(v1) {
                            Ok::<List<Adt18>, bool>(v2)
                        } else {
                            Err::<List<Adt18>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: u64) -> u32 {
    let a2616 = v0;
    let a2617 = 32u64;
    if nat_lt(a2616, a2617) {
        let a2618 = v0;
        let a2619 = 16u64;
        if nat_lt(a2618, a2619) {
            let a2620 = v0;
            let a2621 = 8u64;
            if nat_lt(a2620, a2621) {
                let a2622 = v0;
                let a2623 = 4u64;
                if nat_lt(a2622, a2623) {
                    let a2624 = v0;
                    let a2625 = 2u64;
                    if nat_lt(a2624, a2625) {
                        let a2626 = v0;
                        let a2627 = 1u64;
                        if nat_lt(a2626, a2627) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2628 = v0;
                        let a2629 = 3u64;
                        if nat_lt(a2628, a2629) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2630 = v0;
                    let a2631 = 6u64;
                    if nat_lt(a2630, a2631) {
                        let a2632 = v0;
                        let a2633 = 5u64;
                        if nat_lt(a2632, a2633) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2634 = v0;
                        let a2635 = 7u64;
                        if nat_lt(a2634, a2635) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2636 = v0;
                let a2637 = 12u64;
                if nat_lt(a2636, a2637) {
                    let a2638 = v0;
                    let a2639 = 10u64;
                    if nat_lt(a2638, a2639) {
                        let a2640 = v0;
                        let a2641 = 9u64;
                        if nat_lt(a2640, a2641) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2642 = v0;
                        let a2643 = 11u64;
                        if nat_lt(a2642, a2643) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2644 = v0;
                    let a2645 = 14u64;
                    if nat_lt(a2644, a2645) {
                        let a2646 = v0;
                        let a2647 = 13u64;
                        if nat_lt(a2646, a2647) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2648 = v0;
                        let a2649 = 15u64;
                        if nat_lt(a2648, a2649) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2650 = v0;
            let a2651 = 24u64;
            if nat_lt(a2650, a2651) {
                let a2652 = v0;
                let a2653 = 20u64;
                if nat_lt(a2652, a2653) {
                    let a2654 = v0;
                    let a2655 = 18u64;
                    if nat_lt(a2654, a2655) {
                        let a2656 = v0;
                        let a2657 = 17u64;
                        if nat_lt(a2656, a2657) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2658 = v0;
                        let a2659 = 19u64;
                        if nat_lt(a2658, a2659) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2660 = v0;
                    let a2661 = 22u64;
                    if nat_lt(a2660, a2661) {
                        let a2662 = v0;
                        let a2663 = 21u64;
                        if nat_lt(a2662, a2663) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2664 = v0;
                        let a2665 = 23u64;
                        if nat_lt(a2664, a2665) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2666 = v0;
                let a2667 = 28u64;
                if nat_lt(a2666, a2667) {
                    let a2668 = v0;
                    let a2669 = 26u64;
                    if nat_lt(a2668, a2669) {
                        let a2670 = v0;
                        let a2671 = 25u64;
                        if nat_lt(a2670, a2671) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2672 = v0;
                        let a2673 = 27u64;
                        if nat_lt(a2672, a2673) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2674 = v0;
                    let a2675 = 30u64;
                    if nat_lt(a2674, a2675) {
                        let a2676 = v0;
                        let a2677 = 29u64;
                        if nat_lt(a2676, a2677) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2678 = v0;
                        let a2679 = 31u64;
                        if nat_lt(a2678, a2679) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2680 = v0;
        let a2681 = 48u64;
        if nat_lt(a2680, a2681) {
            let a2682 = v0;
            let a2683 = 40u64;
            if nat_lt(a2682, a2683) {
                let a2684 = v0;
                let a2685 = 36u64;
                if nat_lt(a2684, a2685) {
                    let a2686 = v0;
                    let a2687 = 34u64;
                    if nat_lt(a2686, a2687) {
                        let a2688 = v0;
                        let a2689 = 33u64;
                        if nat_lt(a2688, a2689) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2690 = v0;
                        let a2691 = 35u64;
                        if nat_lt(a2690, a2691) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2692 = v0;
                    let a2693 = 38u64;
                    if nat_lt(a2692, a2693) {
                        let a2694 = v0;
                        let a2695 = 37u64;
                        if nat_lt(a2694, a2695) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2696 = v0;
                        let a2697 = 39u64;
                        if nat_lt(a2696, a2697) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2698 = v0;
                let a2699 = 44u64;
                if nat_lt(a2698, a2699) {
                    let a2700 = v0;
                    let a2701 = 42u64;
                    if nat_lt(a2700, a2701) {
                        let a2702 = v0;
                        let a2703 = 41u64;
                        if nat_lt(a2702, a2703) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2704 = v0;
                        let a2705 = 43u64;
                        if nat_lt(a2704, a2705) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2706 = v0;
                    let a2707 = 46u64;
                    if nat_lt(a2706, a2707) {
                        let a2708 = v0;
                        let a2709 = 45u64;
                        if nat_lt(a2708, a2709) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2710 = v0;
                        let a2711 = 47u64;
                        if nat_lt(a2710, a2711) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2712 = v0;
            let a2713 = 56u64;
            if nat_lt(a2712, a2713) {
                let a2714 = v0;
                let a2715 = 52u64;
                if nat_lt(a2714, a2715) {
                    let a2716 = v0;
                    let a2717 = 50u64;
                    if nat_lt(a2716, a2717) {
                        let a2718 = v0;
                        let a2719 = 49u64;
                        if nat_lt(a2718, a2719) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2720 = v0;
                        let a2721 = 51u64;
                        if nat_lt(a2720, a2721) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2722 = v0;
                    let a2723 = 54u64;
                    if nat_lt(a2722, a2723) {
                        let a2724 = v0;
                        let a2725 = 53u64;
                        if nat_lt(a2724, a2725) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2726 = v0;
                        let a2727 = 55u64;
                        if nat_lt(a2726, a2727) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2728 = v0;
                let a2729 = 60u64;
                if nat_lt(a2728, a2729) {
                    let a2730 = v0;
                    let a2731 = 58u64;
                    if nat_lt(a2730, a2731) {
                        let a2732 = v0;
                        let a2733 = 57u64;
                        if nat_lt(a2732, a2733) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2734 = v0;
                        let a2735 = 59u64;
                        if nat_lt(a2734, a2735) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2736 = v0;
                    let a2737 = 62u64;
                    if nat_lt(a2736, a2737) {
                        let a2738 = v0;
                        let a2739 = 61u64;
                        if nat_lt(a2738, a2739) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2740 = v0;
                        let a2741 = 63u64;
                        if nat_lt(a2740, a2741) {
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

pub fn f555(v0: u32) -> u32 {
    let a2744 = {
        let a2742 = f589(v0, 6u32, 26u32);
        let a2743 = f589(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2742, a2743)
    };
    let a2745 = f589(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2744, a2745)
}

pub fn f556(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2751 = {
        let a2746 = v0;
        let a2747 = v1;
        fixed_u32::bit_and(a2746, a2747)
    };
    let a2752 = {
        let a2749 = {
            let a2748 = v0;
            fixed_u32::bit_not(a2748)
        };
        let a2750 = v2;
        fixed_u32::bit_and(a2749, a2750)
    };
    fixed_u32::bit_xor(a2751, a2752)
}

pub fn f557(v0: u32) -> u32 {
    let a2755 = {
        let a2753 = f589(v0, 2u32, 30u32);
        let a2754 = f589(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2753, a2754)
    };
    let a2756 = f589(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2755, a2756)
}

pub fn f558(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2765 = {
        let a2761 = {
            let a2757 = v0;
            let a2758 = v1;
            fixed_u32::bit_and(a2757, a2758)
        };
        let a2762 = {
            let a2759 = v0;
            let a2760 = v2;
            fixed_u32::bit_and(a2759, a2760)
        };
        fixed_u32::bit_xor(a2761, a2762)
    };
    let a2766 = {
        let a2763 = v1;
        let a2764 = v2;
        fixed_u32::bit_and(a2763, a2764)
    };
    fixed_u32::bit_xor(a2765, a2766)
}

pub fn f559(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h2767, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2767
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2768, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2768
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h2769, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2769
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h2770, _, _, _, _, _, _, _, _, _, _, _) => {
            h2770
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2771, _, _, _, _, _, _, _, _, _, _) => {
            h2771
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2772, _, _, _, _, _, _, _, _, _) => {
            h2772
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h2773, _, _, _, _, _, _, _, _) => {
            h2773
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h2774, _, _, _, _, _, _, _) => {
            h2774
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2775, _, _, _, _, _, _) => {
            h2775
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h2776, _, _, _, _, _) => {
            h2776
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h2777, _, _, _, _) => {
            h2777
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2778, _, _, _) => {
            h2778
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2779, _, _) => {
            h2779
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2780, _) => {
            h2780
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2781) => {
            h2781
        }
    }, f499(f590(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2782, _) => {
            h2782
        }
    }), f499(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2783, _, _, _, _, _, _) => {
            h2783
        }
    }, f499(f591(match v0.clone() {
        Adt26::C0(_, h2784, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2784
        }
    }), match v0 {
        Adt26::C0(h2785, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2785
        }
    }))))
}

pub fn f560(v0: Fn21, p2789: u64, p2790: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2788 = (p2789, p2790);
    {
        let mut m2788 = m2788;
        loop {
            let step = {
                let (v1, v2) = m2788;
                let m2786 = v1;
                if m2786 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2786 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2787: Fn21 = v0.clone();
                        c2787.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2788 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2791, _) = v0;
        h2791
    };
    let v2: u32 = {
        let (h2792, _) = {
            let (_, h2793) = v0;
            h2793
        };
        h2792
    };
    let v3: u32 = {
        let (_, h2794) = {
            let (_, h2795) = v0;
            h2795
        };
        h2794
    };
    ({
        let a2796 = v1;
        let a2797 = 2u64;
        let a2798 = 0u64;
        nat_quot(a2796, a2797, a2798)
    }, (f562(v2, 1u32), {
        let a2802 = {
            let a2799 = v1;
            let a2800 = 2u64;
            let a2801 = 0u64;
            nat_rem(a2799, a2800, a2801)
        };
        let a2803 = 1u64;
        if nat_eq(a2802, a2803) {
            let a2804 = v3;
            let a2805 = v2;
            fixed_u32::bit_or(a2804, a2805)
        } else {
            v3
        }
    }))
}

pub fn f562(v0: u32, v1: u32) -> u32 {
    let m2808 = {
        let a2806 = v0;
        let a2807 = v1;
        fixed_u32::shift_left(a2806, a2807)
    };
    match m2808 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f563(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2811 = f35(v0.clone(), {
        let a2809 = v1;
        let a2810 = 2u64;
        nat_add(a2809, a2810)?
    }, 2u64);
    let a2812 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2811, a2812) {
        let m2815 = f592(f35(v0, {
            let a2813 = v1;
            let a2814 = 4u64;
            nat_add(a2813, a2814)?
        }, 2u64))?;
        match m2815 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f350(v2.clone(), 0u64)?;
                let a2816 = v3;
                let a2817 = 32u64;
                if nat_lt(a2816, a2817) {
                    if f593(v3) {
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

pub fn f564(v0: u64) -> Bytes {
    let a2818 = v0;
    let a2819 = 34u64;
    if nat_eq(a2818, a2819) {
        Bytes::lit(&[0x22])
    } else {
        let a2820 = v0;
        let a2821 = 92u64;
        if nat_eq(a2820, a2821) {
            Bytes::lit(&[0x5c])
        } else {
            let a2822 = v0;
            let a2823 = 98u64;
            if nat_eq(a2822, a2823) {
                Bytes::lit(&[0x08])
            } else {
                let a2824 = v0;
                let a2825 = 102u64;
                if nat_eq(a2824, a2825) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2826 = v0;
                    let a2827 = 110u64;
                    if nat_eq(a2826, a2827) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2828 = v0;
                        let a2829 = 114u64;
                        if nat_eq(a2828, a2829) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2830 = v0;
                            let a2831 = 116u64;
                            if nat_eq(a2830, a2831) {
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

pub fn f565(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2876 = {
            let a2852 = {
                let a2840 = {
                    let a2834 = {
                        let a2832 = v1;
                        let a2833 = 128u8;
                        fixed_u8::bit_and(a2832, a2833)
                    };
                    let a2835 = 128u8;
                    if equal(a2834, a2835) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2841 = {
                    let a2838 = {
                        let a2836 = v1;
                        let a2837 = 64u8;
                        fixed_u8::bit_and(a2836, a2837)
                    };
                    let a2839 = 64u8;
                    if equal(a2838, a2839) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2840, a2841)?
            };
            let a2853 = {
                let a2850 = {
                    let a2844 = {
                        let a2842 = v1;
                        let a2843 = 32u8;
                        fixed_u8::bit_and(a2842, a2843)
                    };
                    let a2845 = 32u8;
                    if equal(a2844, a2845) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2851 = {
                    let a2848 = {
                        let a2846 = v1;
                        let a2847 = 16u8;
                        fixed_u8::bit_and(a2846, a2847)
                    };
                    let a2849 = 16u8;
                    if equal(a2848, a2849) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2850, a2851)?
            };
            nat_add(a2852, a2853)?
        };
        let a2877 = {
            let a2874 = {
                let a2862 = {
                    let a2856 = {
                        let a2854 = v1;
                        let a2855 = 8u8;
                        fixed_u8::bit_and(a2854, a2855)
                    };
                    let a2857 = 8u8;
                    if equal(a2856, a2857) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2863 = {
                    let a2860 = {
                        let a2858 = v1;
                        let a2859 = 4u8;
                        fixed_u8::bit_and(a2858, a2859)
                    };
                    let a2861 = 4u8;
                    if equal(a2860, a2861) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2862, a2863)?
            };
            let a2875 = {
                let a2872 = {
                    let a2866 = {
                        let a2864 = v1;
                        let a2865 = 2u8;
                        fixed_u8::bit_and(a2864, a2865)
                    };
                    let a2867 = 2u8;
                    if equal(a2866, a2867) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2873 = {
                    let a2870 = {
                        let a2868 = v1;
                        let a2869 = 1u8;
                        fixed_u8::bit_and(a2868, a2869)
                    };
                    let a2871 = 1u8;
                    if equal(a2870, a2871) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2872, a2873)?
            };
            nat_add(a2874, a2875)?
        };
        nat_add(a2876, a2877)?
    };
    let a2878 = 48u64;
    let a2879 = v2;
    if nat_le(a2878, a2879) {
        let a2880 = v2;
        let a2881 = 57u64;
        Ok(nat_le(a2880, a2881))
    } else {
        Ok(false)
    }
}

pub fn f566(v0: Fn22, p2886: u64, p2887: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2885 = (p2886, p2887);
    {
        let mut m2885 = m2885;
        loop {
            let step = {
                let (v1, v2) = m2885;
                let m2882 = v1;
                if m2882 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2882 - 1;
                    let m2884 = {
                        let c2883: Fn22 = v0.clone();
                        c2883.apply(v2.clone())?
                    };
                    match m2884 {
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
                Ok(next) => { m2885 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f567(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f594(v0)
}

pub fn f568(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2892 = {
            let a2890 = {
                let a2888 = v0.clone();
                length_list(a2888)
            };
            let a2891 = {
                let a2889 = v1.clone();
                length_list(a2889)
            };
            nat_add(a2890, a2891)?
        };
        let a2893 = 1u64;
        nat_add(a2892, a2893)?
    };
    Ok(f391({
        let (_, h2894) = {
            let (_, h2895) = {
                let (h2896, _) = f595(Fn23::F596, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2896
            };
            h2895
        };
        h2894
    }))
}

pub fn f569(v0: u64) -> Bytes {
    f35({
        let a2897 = Str::lit("0123456789ABCDEF");
        utf8_encode(a2897)
    }, v0, 1u64)
}

pub fn f570(v0: bool, v1: List<Adt18>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2898 = v1;
    match m2898.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2899 = f597(v0, v3.clone())?;
            match m2899 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f598(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f41(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2900 = f570(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2900 {
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

pub fn f571(p2904: List<Bytes>) -> R<bool> {
    let m2903 = p2904;
    {
        let mut m2903 = m2903;
        loop {
            let step = {
                let v0 = m2903;
                let m2901 = v0;
                match m2901.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2902 = f599(v1)?;
                        match m2902 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f41(v3, f600(false, v2.clone())?) {
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
                Ok(next) => { m2903 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(v0: List<(Bytes, Adt18)>, v1: List<Adt18>) -> List<Bytes> {
    let m2905 = v0;
    match m2905.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f601(v1.clone(), {
                let (h2906, _) = v2.clone();
                h2906
            }) {
                List::cons({
                    let (h2907, _) = v2;
                    h2907
                }, f572(v3, v1))
            } else {
                f572(v3, v1)
            }
        }
    }
}

pub fn f573(v0: bool, v1: Bytes, v2: List<Adt18>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m2908 = v2;
    match m2908.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m2909 = f602(v4.clone())?;
            match m2909 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m2910 = f603(v0, v6.clone())?;
                    match m2910 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f41(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2913 = f604(v4, {
                                    let a2911 = v1.clone();
                                    let a2912 = f328(v6)?;
                                    append_bytes(a2911, a2912)
                                })?;
                                match m2913 {
                                    Ok(v9) => {
                                        let m2914 = f573(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m2914 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f496(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f543(v11.clone())), f455(f495(v9), f544(v11)))))
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

pub fn f574(v0: List<Adt18>) -> R<List<Bytes>> {
    let m2915 = v0;
    match m2915.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m2916 = f602(v1)?;
            match m2916 {
                None => {
                    f574(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f574(v2)?))
                }
            }
        }
    }
}

pub fn f575(v0: Bytes, v1: List<Adt18>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f605(f571(f606(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f576(v0: Adt18, v1: Bytes) -> R<bool> {
    let m2917 = f334(Some(v0));
    match m2917 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2918 = f534(v2)?;
            match m2918 {
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

pub fn f577(v0: bool, v1: Bytes, v2: List<(Bytes, Adt18)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m2919 = v2;
    match m2919.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m2921 = f607(v0, {
                let (h2920, _) = v5.clone();
                h2920
            })?;
            match m2921 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h2922, _) = v5;
                        h2922
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f41(v7.clone(), f608(v0, f578(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2927 = f547({
                            let (_, h2923) = v5.clone();
                            h2923
                        }, {
                            let a2925 = v1.clone();
                            let a2926 = f328({
                                let (h2924, _) = v5.clone();
                                h2924
                            })?;
                            append_bytes(a2925, a2926)
                        })?;
                        match m2927 {
                            Ok(v9) => {
                                let v10: Bytes = if f41({
                                    let (h2928, _) = v5;
                                    h2928
                                }, v3.clone()) {
                                    f496(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f496(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m2929 = f577(v0, v1, v6, v3, v4)?;
                                match m2929 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f543(v12.clone())), f455(f495(v9), f544(v12)))))
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

pub fn f578(v0: List<(Bytes, Adt18)>) -> List<Bytes> {
    let m2930 = v0;
    match m2930.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2931, _) = v1;
                h2931
            }, f578(v2))
        }
    }
}

pub fn f579(v0: Adt18) -> Option<Adt18> {
    let m2932 = f242(v0.clone(), f8(Str::lit("additionalProperties")));
    match m2932 {
        None => {
            f609(v0)
        }
        Some(v1) => {
            if f580(v1.clone()) {
                let m2933 = f609(v0);
                match m2933 {
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

pub fn f580(v0: Adt18) -> bool {
    let m2934 = v0;
    match m2934 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_, _) => {
            false
        }
        Adt18::C2(v3) => {
            !v3
        }
        Adt18::C3(_, _) => {
            false
        }
        Adt18::C4(_, _, _) => {
            false
        }
        Adt18::C5(_) => {
            false
        }
        Adt18::C6(_) => {
            false
        }
        Adt18::C7(_) => {
            false
        }
        Adt18::C8 => {
            false
        }
        Adt18::C9(_, _) => {
            false
        }
        Adt18::C10(_, _, _) => {
            false
        }
    }
}

pub fn f581(v0: Adt18) -> Option<Adt18> {
    let m2935 = f247(f242(v0, f8(Str::lit("oneOf"))));
    match m2935.uncons() {
        None => {
            None::<Adt18>
        }
        Some((v1, v2)) => {
            let m2936 = v2;
            match m2936.uncons() {
                None => {
                    None::<Adt18>
                }
                Some((v3, v4)) => {
                    let m2937 = v4;
                    match m2937.uncons() {
                        None => {
                            if f610(v3.clone()) {
                                Some(v1)
                            } else {
                                if f610(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt18>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt18>
                        }
                    }
                }
            }
        }
    }
}

pub fn f582(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m2938 = v1;
    match m2938 {
        Ok(v3) => {
            f538(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f496(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f495(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f583(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2939 = f581(v0.clone());
    match m2939 {
        None => {
            if f28(f245(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2940 = f242(v0, f8(Str::lit("items")));
                match m2940 {
                    None => {
                        Ok(f582(f8(Str::lit("List")), f584()))
                    }
                    Some(v3) => {
                        Ok(f582(f8(Str::lit("List")), f604(v3, {
                            let a2941 = v1;
                            let a2942 = f8(Str::lit("Item"));
                            append_bytes(a2941, a2942)
                        })?))
                    }
                }
            } else {
                f604(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f582(f8(Str::lit("Option")), f604(v2, v1)?))
        }
    }
}

pub fn f584() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f492(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f585(p2946: List<(Bytes, Adt18)>, v1: List<Bytes>) -> bool {
    let m2945 = p2946;
    {
        let mut m2945 = m2945;
        loop {
            let step = {
                let v0 = m2945;
                let m2943 = v0;
                match m2943.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h2944, _) = v2;
                            h2944
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt18)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt18)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2945 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586(p2950: List<Adt18>, v1: Bytes, p2951: List<Bytes>) -> bool {
    let m2949 = (p2950, p2951);
    {
        let mut m2949 = m2949;
        loop {
            let step = {
                let (v0, v2) = m2949;
                let m2947 = v0;
                match m2947.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m2948 = f611(v3, v1.clone());
                        match m2948 {
                            None => {
                                Err::<(List<Adt18>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f41(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt18>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt18>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2949 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(p2954: List<Adt18>, p2955: List<Bytes>) -> bool {
    let m2953 = (p2954, p2955);
    {
        let mut m2953 = m2953;
        loop {
            let step = {
                let (v0, v1) = m2953;
                let m2952 = v0;
                match m2952.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f245(v2, f8(Str::lit("type")));
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
                                    Ok::<(List<Adt18>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt18>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt18>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt18>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2953 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: Adt18) -> bool {
    let v2: Adt18 = {
        let m2956 = f242(v0.clone(), f8(Str::lit("anyOf")));
        match m2956 {
            None => {
                Adt18::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f585(f488(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f548(f335(v2))
    } else {
        false
    }
}

pub fn f589(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2957 = f460(v0, v1);
    let a2958 = f562(v0, v2);
    fixed_u32::bit_or(a2957, a2958)
}

pub fn f590(v0: u32) -> u32 {
    let a2961 = {
        let a2959 = f589(v0, 17u32, 15u32);
        let a2960 = f589(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2959, a2960)
    };
    let a2962 = f460(v0, 10u32);
    fixed_u32::bit_xor(a2961, a2962)
}

pub fn f591(v0: u32) -> u32 {
    let a2965 = {
        let a2963 = f589(v0, 7u32, 25u32);
        let a2964 = f589(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2963, a2964)
    };
    let a2966 = f460(v0, 3u32);
    fixed_u32::bit_xor(a2965, a2966)
}

pub fn f592(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2967 = v0.clone();
        length_bytes(a2967)
    };
    let a2971 = {
        let a2968 = v1;
        let a2969 = 2u64;
        let a2970 = 0u64;
        nat_rem(a2968, a2969, a2970)
    };
    let a2972 = 0u64;
    if nat_eq(a2971, a2972) {
        let (_, h2973) = {
            let (h2974, _) = f612(Fn24::F613(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2974
        };
        Ok(h2973)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f593(v0: u64) -> bool {
    let a2975 = v0;
    let a2976 = 8u64;
    if nat_eq(a2975, a2976) {
        true
    } else {
        let a2977 = v0;
        let a2978 = 9u64;
        if nat_eq(a2977, a2978) {
            true
        } else {
            let a2979 = v0;
            let a2980 = 10u64;
            if nat_eq(a2979, a2980) {
                true
            } else {
                let a2981 = v0;
                let a2982 = 12u64;
                if nat_eq(a2981, a2982) {
                    true
                } else {
                    let a2983 = v0;
                    let a2984 = 13u64;
                    nat_eq(a2983, a2984)
                }
            }
        }
    }
}

pub fn f594(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2985, _) = v0;
        h2985
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2986) = v0;
        h2986
    };
    let m2987 = v1;
    match m2987.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2988 = v4;
            match m2988.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f614(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f595(v0: Fn23, p2993: u64, p2994: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2992 = (p2993, p2994);
    {
        let mut m2992 = m2992;
        loop {
            let step = {
                let (v1, v2) = m2992;
                let m2989 = v1;
                if m2989 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2989 - 1;
                    let m2991 = {
                        let c2990: Fn23 = v0.clone();
                        c2990.apply(v2.clone())
                    };
                    match m2991 {
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
                Ok(next) => { m2992 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f615(v0)
}

pub fn f597(v0: bool, v1: Adt18) -> R<Option<Bytes>> {
    let m2995 = v1;
    match m2995 {
        Adt18::C0 => {
            Ok(None::<Bytes>)
        }
        Adt18::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt18::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C3(v7, v8) => {
            Ok(Some({
                let a3006 = if f352(v7, {
                    let a3004 = 0u64;
                    let a3005 = v8;
                    nat_lt(a3004, a3005)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3007 = f266(false, v8)?;
                append_bytes(a3006, a3007)
            }))
        }
        Adt18::C4(v3, v4, v5) => {
            let m2998 = f616({
                let a2996 = f8(Str::lit("n"));
                let a2997 = f405(false, v4, v5)?;
                append_bytes(a2996, a2997)
            })?;
            match m2998 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3002 = f8(Str::lit("nMinus"));
                        let a3003 = f35(v6.clone(), 1u64, {
                            let a3000 = {
                                let a2999 = v6;
                                length_bytes(a2999)
                            };
                            let a3001 = 1u64;
                            nat_sub(a3000, a3001)
                        });
                        append_bytes(a3002, a3003)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt18::C5(v2) => {
            f603(v0, v2)
        }
        Adt18::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C8 => {
            Ok(None::<Bytes>)
        }
        Adt18::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt18::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f598(v0: Adt18) -> R<Bytes> {
    f333(v0)
}

pub fn f599(v0: Bytes) -> R<Option<Bytes>> {
    let m3008 = f617(v0)?;
    match m3008 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f618(v1)?))
        }
    }
}

pub fn f600(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3009 = v1;
    match m3009.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3010 = f603(v0, v2)?;
            match m3010 {
                None => {
                    f600(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f600(v0, v3)?))
                }
            }
        }
    }
}

pub fn f601(p3014: List<Adt18>, v1: Bytes) -> bool {
    let m3013 = p3014;
    {
        let mut m3013 = m3013;
        loop {
            let step = {
                let v0 = m3013;
                let m3011 = v0;
                match m3011.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3012 = f611(v2, v1.clone());
                        if match m3012 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt18>, bool>(v3)
                        } else {
                            Err::<List<Adt18>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3013 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f602(v0: Adt18) -> R<Option<Bytes>> {
    let m3015 = f334(f242(v0.clone(), f8(Str::lit("$ref"))));
    match m3015 {
        None => {
            Ok(f334(f242(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f534(v1)
        }
    }
}

pub fn f603(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f616(v1)
    } else {
        f599(v1)
    }
}

pub fn f604(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f619(f520(v0), v1)
}

pub fn f605(v0: bool, v1: Bytes, v2: List<Adt18>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3016 = v2;
    match m3016.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3017 = f611(v5.clone(), v3.clone());
            match m3017 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3018 = f603(v0, v7.clone())?;
                    match m3018 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f41(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3021 = f541({
                                    let a3019 = v1.clone();
                                    let a3020 = f328(v7)?;
                                    append_bytes(a3019, a3020)
                                }, f490(f451(f242(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f246(f247(f242(v5, f8(Str::lit("required"))))), true)?;
                                match m3021 {
                                    Ok(v10) => {
                                        let m3022 = f605(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3022 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f342(f543(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f543(v12.clone())), f455(f544(v10), f544(v12)))))
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

pub fn f606(v0: List<Adt18>, v1: Bytes) -> List<Bytes> {
    let m3023 = v0;
    match m3023.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3024 = f611(v2, v1.clone());
            match m3024 {
                None => {
                    f606(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f606(v3, v1))
                }
            }
        }
    }
}

pub fn f607(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3025 = f603(v0, v1)?;
    match m3025 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f28(v2.clone(), f8(Str::lit("mk"))) {
                f518(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f608(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3026 = v1;
    match m3026.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3027 = f607(v0, v2)?;
            match m3027 {
                None => {
                    f608(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f608(v0, v3)?))
                }
            }
        }
    }
}

pub fn f609(v0: Adt18) -> Option<Adt18> {
    let m3028 = f451(f242(v0, f8(Str::lit("patternProperties"))));
    match m3028.uncons() {
        None => {
            None::<Adt18>
        }
        Some((v1, v2)) => {
            let m3029 = v2;
            match m3029.uncons() {
                None => {
                    Some({
                        let (_, h3030) = v1;
                        h3030
                    })
                }
                Some((_, _)) => {
                    None::<Adt18>
                }
            }
        }
    }
}

pub fn f610(v0: Adt18) -> bool {
    f28(f245(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f611(v0: Adt18, v1: Bytes) -> Option<Bytes> {
    if f41(v1.clone(), f246(f247(f242(v0.clone(), f8(Str::lit("required")))))) {
        let m3031 = f242(v0, f8(Str::lit("properties")));
        match m3031 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3032 = f242(v2, v1);
                match m3032 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f334(f242(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f612(v0: Fn24, p3037: u64, p3038: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3036 = (p3037, p3038);
    {
        let mut m3036 = m3036;
        loop {
            let step = {
                let (v1, v2) = m3036;
                let m3033 = v1;
                if m3033 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3033 - 1;
                    let m3035 = {
                        let c3034: Fn24 = v0.clone();
                        c3034.apply(v2.clone())?
                    };
                    match m3035 {
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
                Ok(next) => { m3036 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f613(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f620(v0, v1)
}

pub fn f614(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3043 = {
            let a3041 = {
                let a3039 = v0.clone();
                length_list(a3039)
            };
            let a3042 = {
                let a3040 = v1.clone();
                length_list(a3040)
            };
            nat_add(a3041, a3042)?
        };
        let a3044 = 1u64;
        nat_add(a3043, a3044)?
    };
    Ok(f621({
        let (_, h3045) = {
            let (_, h3046) = {
                let (h3047, _) = f622(Fn25::F623, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3047
            };
            h3046
        };
        h3045
    }))
}

pub fn f615(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3048, _) = v0.clone();
        h3048
    };
    let v2: List<(Bytes, ())> = {
        let (h3049, _) = {
            let (_, h3050) = v0.clone();
            h3050
        };
        h3049
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3051) = {
            let (_, h3052) = v0;
            h3052
        };
        h3051
    };
    let m3053 = v1.clone();
    match m3053.uncons() {
        None => {
            let m3054 = v2;
            match m3054.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3055 = v2.clone();
            match m3055.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f624({
                        let (h3056, _) = v8.clone();
                        h3056
                    }, {
                        let (h3057, _) = v6.clone();
                        h3057
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

pub fn f616(v0: Bytes) -> R<Option<Bytes>> {
    let m3059 = f625({
        let a3058 = v0.clone();
        length_bytes(a3058)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3059 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3061 = {
                let a3060 = v1.clone();
                length_bytes(a3060)
            };
            let a3062 = 0u64;
            if nat_eq(a3061, a3062) {
                let a3064 = {
                    let a3063 = v0;
                    length_bytes(a3063)
                };
                let a3065 = 0u64;
                if nat_eq(a3064, a3065) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f618(v1)?))
            }
        }
    }
}

pub fn f617(v0: Bytes) -> R<Option<Bytes>> {
    if f626({
        let a3066 = v0.clone();
        length_bytes(a3066)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f627(v0)?;
        if f628(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3069 = {
                let a3067 = v1.clone();
                let a3068 = 0u64;
                index_bytes(a3067, a3068)
            };
            match m3069 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f449(f62(v2)?) {
                        Ok(Some({
                            let a3070 = f8(Str::lit("n"));
                            let a3071 = v1;
                            append_bytes(a3070, a3071)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f629({
            let a3072 = v0.clone();
            length_bytes(a3072)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f618(v0: Bytes) -> R<Bytes> {
    if f41(v0.clone(), f630()) {
        f518(v0)
    } else {
        Ok(v0)
    }
}

pub fn f619(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3073 = v0.clone();
    match m3073 {
        Adt18::C0 => {
            Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C1(_, _) => {
            Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C2(v4) => {
            if v4 {
                Ok(f584())
            } else {
                Ok(f537(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt18::C3(_, _) => {
            Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C4(_, _, _) => {
            Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C5(_) => {
            Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C6(_) => {
            Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C7(_) => {
            let m3074 = f631(f488(v0.clone()));
            match m3074 {
                None => {
                    let m3075 = f334(f242(v0.clone(), f8(Str::lit("$ref"))));
                    match m3075 {
                        None => {
                            let m3077 = f242(v0.clone(), f8(Str::lit("allOf")));
                            match m3077 {
                                None => {
                                    let m3082 = f242(v0.clone(), f8(Str::lit("enum")));
                                    match m3082 {
                                        None => {
                                            let m3083 = f242(v0.clone(), f8(Str::lit("const")));
                                            match m3083 {
                                                None => {
                                                    let m3085 = f242(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3085 {
                                                        None => {
                                                            let v32: Bytes = f245(v0.clone(), f8(Str::lit("type")));
                                                            if if f28(v32.clone(), Bytes::lit(&[])) {
                                                                !f632(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f584())
                                                            } else {
                                                                if f28(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f538(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f28(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f538(f633(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f28(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f538(f634(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f28(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f538(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f28(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3086 = f451(f242(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3086.uncons() {
                                                                                        None => {
                                                                                            if f545(v0.clone()) {
                                                                                                Ok(f538(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3087 = f579(v0);
                                                                                                match m3087 {
                                                                                                    None => {
                                                                                                        Ok(f582(f8(Str::lit("List")), f635(f584())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f582(f8(Str::lit("List")), f635(f636(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f538(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f537(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f538(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3084 = f334(Some(v29));
                                                    match m3084 {
                                                        None => {
                                                            Ok(f537(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f538(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f538(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3078 = f335(v21);
                                    match m3078.uncons() {
                                        None => {
                                            Ok(f537(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3079 = v23;
                                            match m3079.uncons() {
                                                None => {
                                                    let m3080 = f334(f242(v22, f8(Str::lit("$ref"))));
                                                    match m3080 {
                                                        None => {
                                                            Ok(f537(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3081 = f534(v26)?;
                                                            match m3081 {
                                                                None => {
                                                                    Ok(f537(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f538(v27, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f537(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3076 = f534(v19)?;
                            match m3076 {
                                None => {
                                    Ok(f537(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f538(v20, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f537(if f28(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt18::C8 => {
            Ok(f584())
        }
        Adt18::C9(_, _) => {
            Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C10(_, _, _) => {
            Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f620(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3088, _) = v1;
        h3088
    };
    let m3090 = {
        let (_, h3089) = v1;
        h3089
    };
    match m3090 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3093 = {
                let a3091 = v0.clone();
                let a3092 = v2;
                index_bytes(a3091, a3092)
            };
            match m3093 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3098 = {
                        let a3096 = v0;
                        let a3097 = {
                            let a3094 = v2;
                            let a3095 = 1u64;
                            nat_add(a3094, a3095)?
                        };
                        index_bytes(a3096, a3097)
                    };
                    match m3098 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f637(v4)? {
                                f637(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3099 = v2;
                                    let a3100 = 2u64;
                                    nat_add(a3099, a3100)?
                                }, Some({
                                    let a3105 = v3;
                                    let a3106 = f474(f638(), {
                                        let a3103 = {
                                            let a3101 = f639(v4)?;
                                            let a3102 = 16u64;
                                            nat_mul(a3101, a3102)?
                                        };
                                        let a3104 = f639(v5)?;
                                        nat_add(a3103, a3104)?
                                    });
                                    append_bytes(a3105, a3106)
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

pub fn f621(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f359(Fn5::F640, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f622(v0: Fn25, p3111: u64, p3112: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3110 = (p3111, p3112);
    {
        let mut m3110 = m3110;
        loop {
            let step = {
                let (v1, v2) = m3110;
                let m3107 = v1;
                if m3107 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3107 - 1;
                    let m3109 = {
                        let c3108: Fn25 = v0.clone();
                        c3108.apply(v2.clone())
                    };
                    match m3109 {
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
                Ok(next) => { m3110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f623(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f641(v0)
}

pub fn f624(v0: Bytes, v1: Bytes) -> bool {
    let a3117 = {
        let a3113 = v0;
        let a3114 = v1;
        compare_bytes(a3113, a3114)
    };
    let a3118 = {
        let a3115 = Bytes::lit(&[]);
        let a3116 = Bytes::lit(&[0x01]);
        compare_bytes(a3115, a3116)
    };
    equal(a3117, a3118)
}

pub fn f625(p3131: u64, v1: Bytes, p3132: u64, p3133: Bytes) -> R<Option<Bytes>> {
    let m3130 = (p3131, (p3132, p3133));
    {
        let mut m3130 = m3130;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3130;
                let m3119 = v0;
                if m3119 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3119 - 1;
                    let a3121 = {
                        let a3120 = v1.clone();
                        length_bytes(a3120)
                    };
                    let a3122 = v2;
                    if nat_le(a3121, a3122) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3123 = f642(v1.clone(), v2)?;
                        match m3123 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3125 = v2;
                                    let a3126 = {
                                        let (_, h3124) = v5;
                                        h3124
                                    };
                                    nat_add(a3125, a3126)?
                                }, {
                                    let a3128 = v3;
                                    let a3129 = f643(v1.clone(), v2, {
                                        let (h3127, _) = v5;
                                        h3127
                                    });
                                    append_bytes(a3128, a3129)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626(p3143: u64, v1: Bytes, p3144: u64) -> R<bool> {
    let m3142 = (p3143, p3144);
    {
        let mut m3142 = m3142;
        loop {
            let step = {
                let (v0, v2) = m3142;
                let m3134 = v0;
                if m3134 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3134 - 1;
                    let m3137 = {
                        let a3135 = v1.clone();
                        let a3136 = v2;
                        index_bytes(a3135, a3136)
                    };
                    match m3137 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3138 = f62(v4)?;
                            let a3139 = 128u64;
                            if nat_lt(a3138, a3139) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3140 = v2;
                                    let a3141 = 1u64;
                                    nat_add(a3140, a3141)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f627(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f403(v0.clone(), 0u64, {
        let a3145 = v0;
        length_bytes(a3145)
    }, false, Bytes::lit(&[]))?;
    let m3148 = {
        let a3146 = v1.clone();
        let a3147 = 0u64;
        index_bytes(a3146, a3147)
    };
    match m3148 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f448(v3) {
                let a3155 = f35({
                    let a3149 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3149)
                }, {
                    let a3150 = v3;
                    let a3151 = 65u64;
                    nat_sub(a3150, a3151)
                }, 1u64);
                let a3156 = f35(v1.clone(), 1u64, {
                    let a3153 = {
                        let a3152 = v1;
                        length_bytes(a3152)
                    };
                    let a3154 = 1u64;
                    nat_sub(a3153, a3154)
                });
                Ok(append_bytes(a3155, a3156))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f628(v0: Bytes) -> R<bool> {
    let m3159 = {
        let a3157 = v0;
        let a3158 = 0u64;
        index_bytes(a3157, a3158)
    };
    match m3159 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f423(f447(f62(v1)?), f448(f62(v1)?)))
        }
    }
}

pub fn f629(p3172: u64, v1: Bytes, p3173: u64, p3174: Bytes) -> R<Option<Bytes>> {
    let m3171 = (p3172, (p3173, p3174));
    {
        let mut m3171 = m3171;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3171;
                let m3160 = v0;
                if m3160 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3160 - 1;
                    let a3162 = {
                        let a3161 = v1.clone();
                        length_bytes(a3161)
                    };
                    let a3163 = v2;
                    if nat_le(a3162, a3163) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3164 = f642(v1.clone(), v2)?;
                        match m3164 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3166 = v2;
                                    let a3167 = {
                                        let (_, h3165) = v5;
                                        h3165
                                    };
                                    nat_add(a3166, a3167)?
                                }, {
                                    let a3169 = v3;
                                    let a3170 = f644({
                                        let (h3168, _) = v5;
                                        h3168
                                    });
                                    append_bytes(a3169, a3170)
                                })))
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

pub fn f630() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f631(p3179: List<(Bytes, Adt18)>) -> Option<Bytes> {
    let m3178 = p3179;
    {
        let mut m3178 = m3178;
        loop {
            let step = {
                let v0 = m3178;
                let m3175 = v0;
                match m3175.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h3176, _) = v1.clone();
                            h3176
                        }, f645()) {
                            Err::<List<(Bytes, Adt18)>, Option<Bytes>>(Some({
                                let (h3177, _) = v1;
                                h3177
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt18)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f632(v0: Adt18) -> bool {
    if f550(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f550(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f550(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f550(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f550(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f550(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f633(v0: Adt18) -> Bytes {
    let m3180 = f646(f242(v0, f8(Str::lit("minimum"))));
    match m3180 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f634() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f635(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3181 = v0;
    match m3181 {
        Ok(v2) => {
            f538(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f496(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f495(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f636(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f647(16u64, v0, v1)
}

pub fn f637(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3226 = {
            let a3202 = {
                let a3190 = {
                    let a3184 = {
                        let a3182 = v1;
                        let a3183 = 128u8;
                        fixed_u8::bit_and(a3182, a3183)
                    };
                    let a3185 = 128u8;
                    if equal(a3184, a3185) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3191 = {
                    let a3188 = {
                        let a3186 = v1;
                        let a3187 = 64u8;
                        fixed_u8::bit_and(a3186, a3187)
                    };
                    let a3189 = 64u8;
                    if equal(a3188, a3189) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3190, a3191)?
            };
            let a3203 = {
                let a3200 = {
                    let a3194 = {
                        let a3192 = v1;
                        let a3193 = 32u8;
                        fixed_u8::bit_and(a3192, a3193)
                    };
                    let a3195 = 32u8;
                    if equal(a3194, a3195) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3201 = {
                    let a3198 = {
                        let a3196 = v1;
                        let a3197 = 16u8;
                        fixed_u8::bit_and(a3196, a3197)
                    };
                    let a3199 = 16u8;
                    if equal(a3198, a3199) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3200, a3201)?
            };
            nat_add(a3202, a3203)?
        };
        let a3227 = {
            let a3224 = {
                let a3212 = {
                    let a3206 = {
                        let a3204 = v1;
                        let a3205 = 8u8;
                        fixed_u8::bit_and(a3204, a3205)
                    };
                    let a3207 = 8u8;
                    if equal(a3206, a3207) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3213 = {
                    let a3210 = {
                        let a3208 = v1;
                        let a3209 = 4u8;
                        fixed_u8::bit_and(a3208, a3209)
                    };
                    let a3211 = 4u8;
                    if equal(a3210, a3211) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3212, a3213)?
            };
            let a3225 = {
                let a3222 = {
                    let a3216 = {
                        let a3214 = v1;
                        let a3215 = 2u8;
                        fixed_u8::bit_and(a3214, a3215)
                    };
                    let a3217 = 2u8;
                    if equal(a3216, a3217) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3223 = {
                    let a3220 = {
                        let a3218 = v1;
                        let a3219 = 1u8;
                        fixed_u8::bit_and(a3218, a3219)
                    };
                    let a3221 = 1u8;
                    if equal(a3220, a3221) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3222, a3223)?
            };
            nat_add(a3224, a3225)?
        };
        nat_add(a3226, a3227)?
    };
    let a3228 = 48u64;
    let a3229 = v2;
    if if nat_le(a3228, a3229) {
        let a3230 = v2;
        let a3231 = 57u64;
        nat_le(a3230, a3231)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3232 = 97u64;
        let a3233 = v2;
        if nat_le(a3232, a3233) {
            let a3234 = v2;
            let a3235 = 102u64;
            Ok(nat_le(a3234, a3235))
        } else {
            Ok(false)
        }
    }
}

pub fn f638() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f639(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3280 = {
            let a3256 = {
                let a3244 = {
                    let a3238 = {
                        let a3236 = v1;
                        let a3237 = 128u8;
                        fixed_u8::bit_and(a3236, a3237)
                    };
                    let a3239 = 128u8;
                    if equal(a3238, a3239) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3245 = {
                    let a3242 = {
                        let a3240 = v1;
                        let a3241 = 64u8;
                        fixed_u8::bit_and(a3240, a3241)
                    };
                    let a3243 = 64u8;
                    if equal(a3242, a3243) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3244, a3245)?
            };
            let a3257 = {
                let a3254 = {
                    let a3248 = {
                        let a3246 = v1;
                        let a3247 = 32u8;
                        fixed_u8::bit_and(a3246, a3247)
                    };
                    let a3249 = 32u8;
                    if equal(a3248, a3249) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3255 = {
                    let a3252 = {
                        let a3250 = v1;
                        let a3251 = 16u8;
                        fixed_u8::bit_and(a3250, a3251)
                    };
                    let a3253 = 16u8;
                    if equal(a3252, a3253) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3254, a3255)?
            };
            nat_add(a3256, a3257)?
        };
        let a3281 = {
            let a3278 = {
                let a3266 = {
                    let a3260 = {
                        let a3258 = v1;
                        let a3259 = 8u8;
                        fixed_u8::bit_and(a3258, a3259)
                    };
                    let a3261 = 8u8;
                    if equal(a3260, a3261) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3267 = {
                    let a3264 = {
                        let a3262 = v1;
                        let a3263 = 4u8;
                        fixed_u8::bit_and(a3262, a3263)
                    };
                    let a3265 = 4u8;
                    if equal(a3264, a3265) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3266, a3267)?
            };
            let a3279 = {
                let a3276 = {
                    let a3270 = {
                        let a3268 = v1;
                        let a3269 = 2u8;
                        fixed_u8::bit_and(a3268, a3269)
                    };
                    let a3271 = 2u8;
                    if equal(a3270, a3271) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3277 = {
                    let a3274 = {
                        let a3272 = v1;
                        let a3273 = 1u8;
                        fixed_u8::bit_and(a3272, a3273)
                    };
                    let a3275 = 1u8;
                    if equal(a3274, a3275) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3276, a3277)?
            };
            nat_add(a3278, a3279)?
        };
        nat_add(a3280, a3281)?
    };
    let a3282 = v2;
    let a3283 = 57u64;
    if nat_le(a3282, a3283) {
        let a3284 = v2;
        let a3285 = 48u64;
        Ok(nat_sub(a3284, a3285))
    } else {
        let a3286 = v2;
        let a3287 = 87u64;
        Ok(nat_sub(a3286, a3287))
    }
}

pub fn f640(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f641(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3288, _) = v0.clone();
        h3288
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3289, _) = {
            let (_, h3290) = v0.clone();
            h3290
        };
        h3289
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3291) = {
            let (_, h3292) = v0;
            h3292
        };
        h3291
    };
    let m3293 = v1.clone();
    match m3293.uncons() {
        None => {
            let m3294 = v2;
            match m3294.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3295 = v2.clone();
            match m3295.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f624({
                        let (h3296, _) = v8.clone();
                        h3296
                    }, {
                        let (h3297, _) = v6.clone();
                        h3297
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

pub fn f642(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3300 = {
        let a3298 = v0.clone();
        let a3299 = v1;
        index_bytes(a3298, a3299)
    };
    match m3300 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            let a3301 = v3;
            let a3302 = 128u64;
            if nat_lt(a3301, a3302) {
                Ok(Some((v3, 1u64)))
            } else {
                if f352({
                    let a3303 = 194u64;
                    let a3304 = v3;
                    nat_le(a3303, a3304)
                }, {
                    let a3305 = v3;
                    let a3306 = 223u64;
                    nat_le(a3305, a3306)
                }) {
                    let m3309 = f648(v0, {
                        let a3307 = v1;
                        let a3308 = 1u64;
                        nat_add(a3307, a3308)?
                    })?;
                    match m3309 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f649({
                                let a3314 = {
                                    let a3312 = {
                                        let a3310 = v3;
                                        let a3311 = 192u64;
                                        nat_sub(a3310, a3311)
                                    };
                                    let a3313 = 64u64;
                                    nat_mul(a3312, a3313)?
                                };
                                let a3315 = v4;
                                nat_add(a3314, a3315)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f352({
                        let a3316 = 224u64;
                        let a3317 = v3;
                        nat_le(a3316, a3317)
                    }, {
                        let a3318 = v3;
                        let a3319 = 239u64;
                        nat_le(a3318, a3319)
                    }) {
                        let m3322 = f648(v0.clone(), {
                            let a3320 = v1;
                            let a3321 = 1u64;
                            nat_add(a3320, a3321)?
                        })?;
                        match m3322 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3325 = f648(v0, {
                                    let a3323 = v1;
                                    let a3324 = 2u64;
                                    nat_add(a3323, a3324)?
                                })?;
                                match m3325 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f649({
                                            let a3334 = {
                                                let a3328 = {
                                                    let a3326 = v3;
                                                    let a3327 = 224u64;
                                                    nat_sub(a3326, a3327)
                                                };
                                                let a3329 = 4096u64;
                                                nat_mul(a3328, a3329)?
                                            };
                                            let a3335 = {
                                                let a3332 = {
                                                    let a3330 = v5;
                                                    let a3331 = 64u64;
                                                    nat_mul(a3330, a3331)?
                                                };
                                                let a3333 = v6;
                                                nat_add(a3332, a3333)?
                                            };
                                            nat_add(a3334, a3335)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f352({
                            let a3336 = 240u64;
                            let a3337 = v3;
                            nat_le(a3336, a3337)
                        }, {
                            let a3338 = v3;
                            let a3339 = 244u64;
                            nat_le(a3338, a3339)
                        }) {
                            let m3342 = f648(v0.clone(), {
                                let a3340 = v1;
                                let a3341 = 1u64;
                                nat_add(a3340, a3341)?
                            })?;
                            match m3342 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3345 = f648(v0.clone(), {
                                        let a3343 = v1;
                                        let a3344 = 2u64;
                                        nat_add(a3343, a3344)?
                                    })?;
                                    match m3345 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3348 = f648(v0, {
                                                let a3346 = v1;
                                                let a3347 = 3u64;
                                                nat_add(a3346, a3347)?
                                            })?;
                                            match m3348 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f649({
                                                        let a3361 = {
                                                            let a3351 = {
                                                                let a3349 = v3;
                                                                let a3350 = 240u64;
                                                                nat_sub(a3349, a3350)
                                                            };
                                                            let a3352 = 262144u64;
                                                            nat_mul(a3351, a3352)?
                                                        };
                                                        let a3362 = {
                                                            let a3359 = {
                                                                let a3353 = v7;
                                                                let a3354 = 4096u64;
                                                                nat_mul(a3353, a3354)?
                                                            };
                                                            let a3360 = {
                                                                let a3357 = {
                                                                    let a3355 = v8;
                                                                    let a3356 = 64u64;
                                                                    nat_mul(a3355, a3356)?
                                                                };
                                                                let a3358 = v9;
                                                                nat_add(a3357, a3358)?
                                                            };
                                                            nat_add(a3359, a3360)?
                                                        };
                                                        nat_add(a3361, a3362)?
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

pub fn f643(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f423(f352(f423(f447(v2), f448(v2)), f352({
        let a3363 = v2;
        let a3364 = 117u64;
        !nat_eq(a3363, a3364)
    }, {
        let a3365 = v2;
        let a3366 = 85u64;
        !nat_eq(a3365, a3366)
    })), f352(f449(v2), {
        let a3367 = 0u64;
        let a3368 = v1;
        nat_lt(a3367, a3368)
    })) {
        f35(v0, v1, 1u64)
    } else {
        let a3369 = v2;
        let a3370 = 65536u64;
        if nat_lt(a3369, a3370) {
            let a3371 = f8(Str::lit("u"));
            let a3372 = f535(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3371, a3372)
        } else {
            let a3373 = f8(Str::lit("U"));
            let a3374 = f535(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3373, a3374)
        }
    }
}

pub fn f644(v0: u64) -> Bytes {
    let a3375 = f8(Str::lit("u"));
    let a3376 = f535(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3375, a3376)
}

pub fn f645() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f646(v0: Option<Adt18>) -> Option<u64> {
    let m3377 = v0;
    match m3377 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m3378 = v1;
            match m3378 {
                Adt18::C0 => {
                    None::<u64>
                }
                Adt18::C1(_, _) => {
                    None::<u64>
                }
                Adt18::C2(_) => {
                    None::<u64>
                }
                Adt18::C3(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt18::C4(_, _, _) => {
                    None::<u64>
                }
                Adt18::C5(_) => {
                    None::<u64>
                }
                Adt18::C6(_) => {
                    None::<u64>
                }
                Adt18::C7(_) => {
                    None::<u64>
                }
                Adt18::C8 => {
                    None::<u64>
                }
                Adt18::C9(_, _) => {
                    None::<u64>
                }
                Adt18::C10(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f647(v0: u64, v1: Adt18, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3379 = v0;
    if m3379 == 0 {
        Ok(f537(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3379 - 1;
        let m3380 = v1.clone();
        match m3380 {
            Adt18::C0 => {
                Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C1(_, _) => {
                Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C2(v6) => {
                if v6 {
                    Ok(f584())
                } else {
                    Ok(f537(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt18::C3(_, _) => {
                Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C4(_, _, _) => {
                Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C5(_) => {
                Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C6(_) => {
                Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C7(_) => {
                let m3381 = f334(f242(v1.clone(), f8(Str::lit("$ref"))));
                match m3381 {
                    None => {
                        if if f550(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f550(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f550(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f538(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v22: Bytes = f245(v1.clone(), f8(Str::lit("type")));
                            if if f28(v22.clone(), Bytes::lit(&[])) {
                                !f632(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f584())
                            } else {
                                if f28(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f538(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f28(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f538(f633(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f28(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f538(f634(), List::<Adt25>::nil()))
                                        } else {
                                            if f28(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f538(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f28(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3383 = f242(v1, f8(Str::lit("items")));
                                                    match m3383 {
                                                        None => {
                                                            Ok(f582(f8(Str::lit("List")), f584()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f582(f8(Str::lit("List")), f647(v3, v23, {
                                                                let a3384 = v2;
                                                                let a3385 = f8(Str::lit("Item"));
                                                                append_bytes(a3384, a3385)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f28(v22.clone(), f8(Str::lit("object"))) {
                                                        if f545(v1.clone()) {
                                                            Ok(f538(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3386 = f579(v1);
                                                            match m3386 {
                                                                None => {
                                                                    Ok(f582(f8(Str::lit("List")), f635(f584())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f582(f8(Str::lit("List")), f635(f647(v3, v24, {
                                                                        let a3387 = v2;
                                                                        let a3388 = f8(Str::lit("Value"));
                                                                        append_bytes(a3387, a3388)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f537(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
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
                        let m3382 = f534(v20)?;
                        match m3382 {
                            None => {
                                Ok(f537(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f538(v21, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt18::C8 => {
                Ok(f584())
            }
            Adt18::C9(_, _) => {
                Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C10(_, _, _) => {
                Ok(f537(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f648(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3391 = {
        let a3389 = v0;
        let a3390 = v1;
        index_bytes(a3389, a3390)
    };
    match m3391 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f352({
                let a3392 = 128u64;
                let a3393 = v3;
                nat_le(a3392, a3393)
            }, {
                let a3394 = v3;
                let a3395 = 191u64;
                nat_le(a3394, a3395)
            }) {
                Ok(Some({
                    let a3396 = v3;
                    let a3397 = 128u64;
                    nat_sub(a3396, a3397)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f649(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f352({
        let a3398 = v1;
        let a3399 = v0;
        nat_le(a3398, a3399)
    }, f352({
        let a3400 = v0;
        let a3401 = v2;
        nat_le(a3400, a3401)
    }, f423({
        let a3402 = v0;
        let a3403 = 55296u64;
        nat_lt(a3402, a3403)
    }, {
        let a3404 = 57343u64;
        let a3405 = v0;
        nat_lt(a3404, a3405)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
