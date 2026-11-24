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
    F396(Bytes),
    F397(Bytes),
    F431(Bytes),
    F458(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F229(k0, k1, k2, k3) => f229(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F322(k0) => Ok(f322(k0.clone(), p0)),
            Fn3::F323(k0, k1) => f323(*k0, k1.clone(), p0),
            Fn3::F396(k0) => f396(k0.clone(), p0),
            Fn3::F397(k0) => f397(k0.clone(), p0),
            Fn3::F431(k0) => f431(k0.clone(), p0),
            Fn3::F458(k0) => f458(k0.clone(), p0),
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
    F408(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F343(k0) => f343(k0.clone(), p0),
            Fn9::F408(k0, k1) => Ok(f408(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F345,
    F535,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F345 => f345(p0, p1),
            Fn10::F535 => f535(p0, p1),
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
    F400(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F400(k0) => f400(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F412,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F412 => f412(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F414,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F414 => f414(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F416,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F416 => f416(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F424,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F424 => f424(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F450,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F450 => f450(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F496,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F496 => f496(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F498,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F498 => f498(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F500(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F500(k0) => f500(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F533,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F533 => f533(p0),
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
    let v5: u64 = {
        let a1720 = v4;
        let a1721 = v3;
        nat_sub(a1720, a1721)
    };
    let v6: bool = {
        let a1722 = f325(v0.clone(), v4)?;
        let a1723 = 46u64;
        nat_eq(a1722, a1723)
    };
    let v7: u64 = if v6 {
        let a1724 = v4;
        let a1725 = 1u64;
        nat_add(a1724, a1725)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f365(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f325(v0.clone(), v8)?;
    let v10: bool = {
        let a1726 = v9;
        let a1727 = 101u64;
        if nat_eq(a1726, a1727) {
            true
        } else {
            let a1728 = v9;
            let a1729 = 69u64;
            nat_eq(a1728, a1729)
        }
    };
    let v11: u64 = f325(v0.clone(), {
        let a1730 = v8;
        let a1731 = 1u64;
        nat_add(a1730, a1731)?
    })?;
    let v12: bool = if v10 {
        let a1732 = v11;
        let a1733 = 43u64;
        if nat_eq(a1732, a1733) {
            true
        } else {
            let a1734 = v11;
            let a1735 = 45u64;
            nat_eq(a1734, a1735)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a1736 = v8;
            let a1737 = 2u64;
            nat_add(a1736, a1737)?
        } else {
            let a1738 = v8;
            let a1739 = 1u64;
            nat_add(a1738, a1739)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f365(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a1740 = v14;
        let a1741 = v13;
        nat_sub(a1740, a1741)
    });
    let v16: u64 = f366(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a1743 = {
            let a1742 = v15;
            length_bytes(a1742)
        };
        let a1744 = v16;
        nat_sub(a1743, a1744)
    });
    let a1745 = v5;
    let a1746 = 0u64;
    if if nat_eq(a1745, a1746) {
        true
    } else {
        let a1747 = 1u64;
        let a1748 = v5;
        if if nat_lt(a1747, a1748) {
            let a1749 = f325(v0.clone(), v3)?;
            let a1750 = 48u64;
            nat_eq(a1749, a1750)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a1751 = v8;
                let a1752 = v7;
                nat_eq(a1751, a1752)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a1753 = v14;
                    let a1754 = v13;
                    nat_eq(a1753, a1754)
                } else {
                    false
                } {
                    true
                } else {
                    let a1756 = 18u64;
                    let a1757 = {
                        let a1755 = v17.clone();
                        length_bytes(a1755)
                    };
                    nat_lt(a1756, a1757)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1761 = {
            let a1759 = {
                let a1758 = v17.clone();
                length_bytes(a1758)
            };
            let a1760 = 0u64;
            if nat_eq(a1759, a1760) {
                Some(0u64)
            } else {
                f367(v17)?
            }
        };
        match m1761 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m1766 = f368(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a1762 = v8;
                    let a1763 = v7;
                    nat_sub(a1762, a1763)
                }), if v12 {
                    let a1764 = v11;
                    let a1765 = 45u64;
                    nat_eq(a1764, a1765)
                } else {
                    false
                }, v18)?;
                match m1766 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a1768 = 19u64;
                        let a1769 = {
                            let a1767 = v19.clone();
                            length_bytes(a1767)
                        };
                        if nat_lt(a1768, a1769) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m1770 = f367(v19)?;
                            match m1770 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a1771 = 0u64;
                                        let a1772 = v20;
                                        nat_lt(a1771, a1772)
                                    } else {
                                        false
                                    };
                                    if f369(v21, v20) {
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

pub fn f328(v0: u64) -> bool {
    let a1773 = v0;
    let a1774 = 95u64;
    if nat_eq(a1773, a1774) {
        true
    } else {
        let a1775 = v0;
        let a1776 = 45u64;
        if nat_eq(a1775, a1776) {
            true
        } else {
            let a1777 = v0;
            let a1778 = 32u64;
            if nat_eq(a1777, a1778) {
                true
            } else {
                let a1779 = v0;
                let a1780 = 46u64;
                if nat_eq(a1779, a1780) {
                    true
                } else {
                    let a1781 = v0;
                    let a1782 = 47u64;
                    nat_eq(a1781, a1782)
                }
            }
        }
    }
}

pub fn f329(v0: u64) -> bool {
    let a1783 = 97u64;
    let a1784 = v0;
    if nat_le(a1783, a1784) {
        let a1785 = v0;
        let a1786 = 122u64;
        nat_le(a1785, a1786)
    } else {
        false
    }
}

pub fn f330(v0: u64) -> bool {
    let a1787 = 65u64;
    let a1788 = v0;
    if nat_le(a1787, a1788) {
        let a1789 = v0;
        let a1790 = 90u64;
        nat_le(a1789, a1790)
    } else {
        false
    }
}

pub fn f331(v0: u64) -> bool {
    let a1791 = 48u64;
    let a1792 = v0;
    if nat_le(a1791, a1792) {
        let a1793 = v0;
        let a1794 = 57u64;
        nat_le(a1793, a1794)
    } else {
        false
    }
}

pub fn f332(v0: List<(Bytes, Adt14)>, v1: List<(Bytes, Adt14)>) -> List<(Bytes, Adt14)> {
    let m1795 = v0;
    match m1795.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f332(v3, v1))
        }
    }
}

pub fn f333(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1796 = v0;
    match m1796 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f370(v1)
        }
    }
}

pub fn f334(v0: u64, v1: List<(Bytes, Adt14)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt14)>, Bytes>> {
    let m1797 = v0;
    if m1797 == 0 {
        let m1798 = v1;
        match m1798.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt14)>, Bytes>(List::<(Bytes, Adt14)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt14)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m1797 - 1;
        let m1799 = v1.clone();
        match m1799.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt14)>, Bytes>(List::<(Bytes, Adt14)>::nil()))
            }
            Some((_, _)) => {
                let m1800 = f371(v1.clone(), v2.clone(), v3.clone())?;
                match m1800 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt14)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f204(f335(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m1803 = f334(v6, f372(v1, {
                            let (h1801, _) = v9.clone();
                            h1801
                        }), List::cons(f218({
                            let (h1802, _) = v9.clone();
                            h1802
                        })?, v2), v3)?;
                        match m1803 {
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
    let m1804 = v0;
    match m1804.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f218({
                let (h1805, _) = v1;
                h1805
            })?, f335(v2)?))
        }
    }
}

pub fn f336(p1813: u64, p1814: List<Adt23>, p1815: List<Bytes>, p1816: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1812 = (p1813, (p1814, (p1815, p1816)));
    {
        let mut m1812 = m1812;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1812;
                let m1806 = v0;
                if m1806 == 0 {
                    Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1806 - 1;
                    let m1807 = v1;
                    match m1807.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1808 = v5;
                            match m1808 {
                                Adt23::C0(v8, v9) => {
                                    if f36(v8.clone(), f373()) {
                                        Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f85(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1809 = f375(v8.clone(), v9)?;
                                            match m1809 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f337({
                                                        let (_, h1810) = v11;
                                                        h1810
                                                    }, List::cons(Adt23::C1({
                                                        let (h1811, _) = v11;
                                                        h1811
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
                                    if f85(f373(), v2.clone()) {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f373(), v2), List::cons(f374(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: List<Adt23>, v1: List<Adt23>) -> List<Adt23> {
    let m1817 = v0;
    match m1817.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f337(v3, v1))
        }
    }
}

pub fn f338(v0: List<(Bytes, Adt14)>) -> R<List<Adt23>> {
    let m1818 = v0;
    match m1818.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt23::C0(f218({
                let (h1819, _) = v1;
                h1819
            })?, {
                let (_, h1820) = v1;
                h1820
            }), f338(v2)?))
        }
    }
}

pub fn f339(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1821 = v0;
    match m1821.uncons() {
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
    let m1822 = v0;
    match m1822.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f194(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f204(f349(f77(List::cons(v1, f376(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f340(v3, v1)?))
            }
        }
    }
}

pub fn f341(v0: u64) -> R<Bytes> {
    f377(f378(), 10u64, v0)
}

pub fn f342(v0: Fn9, p1827: u64, p1828: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1826 = (p1827, p1828);
    {
        let mut m1826 = m1826;
        loop {
            let step = {
                let (v1, v2) = m1826;
                let m1823 = v1;
                if m1823 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1823 - 1;
                    let m1825 = {
                        let c1824: Fn9 = v0.clone();
                        c1824.apply(v2.clone())?
                    };
                    match m1825 {
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
                Ok(next) => { m1826 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f379(v0, v1)
}

pub fn f344(v0: Fn10, p1832: List<(Bytes, Bytes)>, p1833: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1831 = (p1832, p1833);
    {
        let mut m1831 = m1831;
        loop {
            let step = {
                let (v1, v2) = m1831;
                let m1829 = v2;
                match m1829.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1830: Fn10 = v0.clone();
                            c1830.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1831 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f380(v0, v1)
}

pub fn f346(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f381(v0)
}

pub fn f347(v0: Fn11, p1837: List<Bytes>, p1838: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1836 = (p1837, p1838);
    {
        let mut m1836 = m1836;
        loop {
            let step = {
                let (v1, v2) = m1836;
                let m1834 = v2;
                match m1834.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1835: Fn11 = v0.clone();
                            c1835.apply(v1, v3)?
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

pub fn f348(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f382(v1)?, v0))
}

pub fn f349(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1839 = v0;
    match m1839.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f199(v1)?, f349(v2)?))
        }
    }
}

pub fn f350(v0: Fn12, p1843: List<List<(Bytes, ())>>, p1844: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1842 = (p1843, p1844);
    {
        let mut m1842 = m1842;
        loop {
            let step = {
                let (v1, v2) = m1842;
                let m1840 = v2;
                match m1840.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1841: Fn12 = v0.clone();
                            c1841.apply(v1, v3)
                        }, v4))
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

pub fn f351(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f352(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1846 = {
        let a1845 = v0.clone();
        length_list(a1845)
    };
    let a1847 = 1u64;
    if nat_le(a1846, a1847) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f383(v0)?))
    }
}

pub fn f353(v0: Fn13, p1851: List<(Bytes, ())>, p1852: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1850 = (p1851, p1852);
    {
        let mut m1850 = m1850;
        loop {
            let step = {
                let (v1, v2) = m1850;
                let m1848 = v2;
                match m1848.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1849: Fn13 = v0.clone();
                            c1849.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1850 = next; }
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
    let v4: List<Bytes> = f384(v3.clone(), f71(f72(v0, f88())?, f8(Str::lit("at"))));
    let a1854 = {
        let a1853 = v4.clone();
        length_list(a1853)
    };
    let a1855 = 0u64;
    if nat_eq(a1854, a1855) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1858 = {
            let a1856 = v4;
            length_list(a1856)
        };
        let a1859 = {
            let a1857 = v3;
            length_list(a1857)
        };
        if nat_eq(a1858, a1859) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f385(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f356(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f214(f135(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f214(f135(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f63(f71(v3.clone(), f8(Str::lit("id"))), f71(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f71(f72(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f384(v5.clone(), f63(v6.clone(), f71(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1861 = {
        let a1860 = v7.clone();
        length_list(a1860)
    };
    let a1862 = 0u64;
    if nat_eq(a1861, a1862) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f74({
            let a1865 = {
                let a1863 = v7;
                length_list(a1863)
            };
            let a1866 = {
                let a1864 = v5;
                length_list(a1864)
            };
            nat_eq(a1865, a1866)
        }) {
            Ok((false, f385(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f77(f63(f384(f386(v4.clone()), f68(f69(v0, f8(Str::lit("model/authorities.toml"))))?), f384(f387(v4), f63(v6, f71(v3, f8(Str::lit("id")))))))?;
            let a1868 = {
                let a1867 = v8.clone();
                length_list(a1867)
            };
            let a1869 = 0u64;
            if nat_eq(a1868, a1869) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f385(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f120(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f357(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h1870, _) = v0;
        h1870
    }, {
        let (h1871, _) = v1;
        h1871
    }) {
        f36({
            let (_, h1872) = v0;
            h1872
        }, {
            let (_, h1873) = v1;
            h1873
        })
    } else {
        false
    }
}

pub fn f358(v0: Fn14, p1877: u64, p1878: Adt25) -> R<Adt25> {
    let m1876 = (p1877, p1878);
    {
        let mut m1876 = m1876;
        loop {
            let step = {
                let (v1, v2) = m1876;
                let m1874 = v1;
                if m1874 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m1874 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c1875: Fn14 = v0.clone();
                        c1875.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1876 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: Adt25) -> R<Adt25> {
    f388(v0)
}

pub fn f360(v0: u32, v1: u32) -> u32 {
    let m1881 = {
        let a1879 = f389(v0);
        let a1880 = f389(v1);
        fixed_u64::checked_add(a1879, a1880)
    };
    match m1881 {
        None => {
            0u32
        }
        Some(v2) => {
            f390(v2)
        }
    }
}

pub fn f361(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1884 = {
        let a1882 = v3;
        let a1883 = 8u64;
        nat_add(a1882, a1883)?
    };
    let a1885 = v2;
    if nat_eq(a1884, a1885) {
        Ok(f391({
            let a1888 = {
                let a1886 = v1;
                let a1887 = 8u64;
                nat_mul(a1886, a1887)?
            };
            let a1889 = 4294967296u64;
            let a1890 = 0u64;
            nat_quot(a1888, a1889, a1890)
        }))
    } else {
        let a1893 = {
            let a1891 = v3;
            let a1892 = 4u64;
            nat_add(a1891, a1892)?
        };
        let a1894 = v2;
        if nat_eq(a1893, a1894) {
            Ok(f391({
                let a1897 = {
                    let a1895 = v1;
                    let a1896 = 8u64;
                    nat_mul(a1895, a1896)?
                };
                let a1898 = 4294967296u64;
                let a1899 = 0u64;
                nat_rem(a1897, a1898, a1899)
            }))
        } else {
            Ok(f392(f393(v0.clone(), v1, v3), f393(v0.clone(), v1, {
                let a1900 = v3;
                let a1901 = 1u64;
                nat_add(a1900, a1901)?
            }), f393(v0.clone(), v1, {
                let a1902 = v3;
                let a1903 = 2u64;
                nat_add(a1902, a1903)?
            }), f393(v0, v1, {
                let a1904 = v3;
                let a1905 = 3u64;
                nat_add(a1904, a1905)?
            })))
        }
    }
}

pub fn f362(v0: Bytes, v1: u64) -> bool {
    let m1906 = f226(v0, v1);
    match m1906 {
        None => {
            false
        }
        Some(v2) => {
            f394(v2)
        }
    }
}

pub fn f363(v0: Fn15, p1911: u64, p1912: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1910 = (p1911, p1912);
    {
        let mut m1910 = m1910;
        loop {
            let step = {
                let (v1, v2) = m1910;
                let m1907 = v1;
                if m1907 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1907 - 1;
                    let m1909 = {
                        let c1908: Fn15 = v0.clone();
                        c1908.apply(v2.clone())?
                    };
                    match m1909 {
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
                Ok(next) => { m1910 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f395(v0, v1)
}

pub fn f365(v0: Bytes, v1: u64) -> R<u64> {
    let (h1913, _) = f228(Fn3::F396(v0.clone()), {
        let a1915 = {
            let a1914 = v0;
            length_bytes(a1914)
        };
        let a1916 = 1u64;
        nat_add(a1915, a1916)?
    }, v1)?;
    Ok(h1913)
}

pub fn f366(v0: Bytes) -> R<u64> {
    let (h1917, _) = f228(Fn3::F397(v0.clone()), {
        let a1919 = {
            let a1918 = v0;
            length_bytes(a1918)
        };
        let a1920 = 1u64;
        nat_add(a1919, a1920)?
    }, 0u64)?;
    Ok(h1917)
}

pub fn f367(v0: Bytes) -> R<Option<u64>> {
    if f398(v0.clone())? {
        Ok(Some({
            let (_, h1921) = {
                let (h1922, _) = f399(Fn16::F400(v0.clone()), {
                    let a1923 = v0;
                    length_bytes(a1923)
                }, (0u64, 0u64))?;
                h1922
            };
            h1921
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f368(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<Bytes>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a1925 = {
            let a1924 = v1.clone();
            length_bytes(a1924)
        };
        let a1926 = f401(v1)?;
        nat_sub(a1925, a1926)
    });
    let v5: Bytes = {
        let a1927 = v0;
        let a1928 = v4.clone();
        append_bytes(a1927, a1928)
    };
    let v6: u64 = f366(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a1930 = {
            let a1929 = v5;
            length_bytes(a1929)
        };
        let a1931 = v6;
        nat_sub(a1930, a1931)
    });
    let v8: u64 = {
        let a1932 = v4;
        length_bytes(a1932)
    };
    let v9: u64 = {
        let a1933 = v7.clone();
        length_bytes(a1933)
    };
    let a1934 = v9;
    let a1935 = 0u64;
    if nat_eq(a1934, a1935) {
        Ok(Some(f8(Str::lit("0"))))
    } else {
        if if v2 {
            true
        } else {
            let a1936 = v3;
            let a1937 = v8;
            nat_lt(a1936, a1937)
        } {
            let v10: u64 = if v2 {
                let a1938 = v3;
                let a1939 = v8;
                nat_add(a1938, a1939)?
            } else {
                let a1940 = v8;
                let a1941 = v3;
                nat_sub(a1940, a1941)
            };
            let a1942 = v10;
            let a1943 = f401(v7.clone())?;
            if nat_le(a1942, a1943) {
                Ok(Some(f32(v7, 0u64, {
                    let a1944 = v9;
                    let a1945 = v10;
                    nat_sub(a1944, a1945)
                })))
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            let v11: u64 = {
                let a1946 = v3;
                let a1947 = v8;
                nat_sub(a1946, a1947)
            };
            let a1950 = {
                let a1948 = v9;
                let a1949 = v11;
                nat_add(a1948, a1949)?
            };
            let a1951 = 19u64;
            if nat_le(a1950, a1951) {
                Ok(Some({
                    let a1952 = v7;
                    let a1953 = f402(v11);
                    append_bytes(a1952, a1953)
                }))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f369(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1954 = 0u64;
        let a1955 = v1;
        if nat_lt(a1954, a1955) {
            let a1956 = v1;
            let a1957 = 9223372036854775808u64;
            nat_le(a1956, a1957)
        } else {
            false
        }
    } else {
        let a1958 = v1;
        let a1959 = 9223372036854775807u64;
        nat_le(a1958, a1959)
    }
}

pub fn f370(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1960 = v0;
    match m1960 {
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
        Adt14::C4(r1961) => {
            let v4 = (*r1961).clone();
            f370(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1962, r1963) => {
            let v2 = (*r1962).clone();
            let v3 = (*r1963).clone();
            List::cons((v1, v2), f370(v3))
        }
    }
}

pub fn f371(p1968: List<(Bytes, Adt14)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt14)>> {
    let m1967 = p1968;
    {
        let mut m1967 = m1967;
        loop {
            let step = {
                let v0 = m1967;
                let m1964 = v0;
                match m1964.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, R<Option<(Bytes, Adt14)>>>(Ok(None::<(Bytes, Adt14)>))
                    }
                    Some((v3, v4)) => {
                        if f403(f404({
                            let (_, h1965) = v3.clone();
                            h1965
                        }, List::<Bytes>::nil())?, f218({
                            let (h1966, _) = v3.clone();
                            h1966
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt14)>, R<Option<(Bytes, Adt14)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, R<Option<(Bytes, Adt14)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1967 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(v0: List<(Bytes, Adt14)>, v1: Bytes) -> List<(Bytes, Adt14)> {
    let m1969 = v0;
    match m1969.uncons() {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1970, _) = v2.clone();
                h1970
            }, v1.clone()) {
                f372(v3, v1)
            } else {
                List::cons(v2, f372(v3, v1))
            }
        }
    }
}

pub fn f373() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f374() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f375(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    f405(v0, f406(v1))
}

pub fn f376(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1971 = v0;
    match m1971.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f376(v3, false))
            } else {
                f376(v3, true)
            }
        }
    }
}

pub fn f377(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1972 = v2;
    let a1973 = 0u64;
    if nat_eq(a1972, a1973) {
        Ok(f407(v0, 0u64))
    } else {
        let (_, h1974) = {
            let (h1975, _) = f342(Fn9::F408(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1975
        };
        Ok(h1974)
    }
}

pub fn f378() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f379(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1979 = {
        let a1977 = v0.clone();
        let a1978 = {
            let (h1976, _) = v1.clone();
            h1976
        };
        index_bytes(a1977, a1978)
    };
    match m1979 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1981 = {
                    let (h1980, _) = v1.clone();
                    h1980
                };
                let a1982 = 1u64;
                nat_add(a1981, a1982)?
            }, {
                let a1985 = {
                    let (_, h1983) = v1;
                    h1983
                };
                let a1986 = f409(v0, {
                    let (h1984, _) = v1;
                    h1984
                }, v2)?;
                append_bytes(a1985, a1986)
            })))
        }
    }
}

pub fn f380(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1987 = v0.clone();
    match m1987.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1990 = {
                let (h1988, _) = v2;
                h1988
            };
            let a1991 = {
                let (h1989, _) = v1.clone();
                h1989
            };
            if equal(a1990, a1991) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f381(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f410(f411(Fn17::F412, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1996 = {
        let (h1992, _) = f413(Fn18::F414, {
            let a1994 = {
                let a1993 = v0;
                length_list(a1993)
            };
            let a1995 = 1u64;
            nat_add(a1994, a1995)?
        }, v1)?;
        h1992
    };
    match m1996.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f382(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2001 = {
        let a1998 = f289({
            let (h1997, _) = v0;
            h1997
        })?;
        let a1999 = Bytes::lit(&[0x3a]);
        append_bytes(a1998, a1999)
    };
    let a2002 = {
        let (_, h2000) = v0;
        h2000
    };
    Ok(append_bytes(a2001, a2002))
}

pub fn f383(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f303({
        let (_, h2003) = {
            let (h2004, _) = f415(Fn19::F416, {
                let a2006 = {
                    let a2005 = v0.clone();
                    length_list(a2005)
                };
                let a2007 = 1u64;
                nat_add(a2006, a2007)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2004
        };
        h2003
    }))
}

pub fn f384(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2008 = v0;
    match m2008.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f384(v3, v1)
            } else {
                List::cons(v2, f384(v3, v1))
            }
        }
    }
}

pub fn f385(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f386(v0: List<Adt14>) -> List<Bytes> {
    let m2009 = v0;
    match m2009.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2010 = f142(v1, f8(Str::lit("std")));
            match m2010 {
                None => {
                    f386(v2)
                }
                Some(v3) => {
                    List::cons(f145(v3, f8(Str::lit("authority"))), f386(v2))
                }
            }
        }
    }
}

pub fn f387(v0: List<Adt14>) -> List<Bytes> {
    let m2011 = v0;
    match m2011.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2012 = f150(f142(v1, f8(Str::lit("req"))));
            match m2012 {
                None => {
                    f387(v2)
                }
                Some(v3) => {
                    List::cons(v3, f387(v2))
                }
            }
        }
    }
}

pub fn f388(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f417(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h2013) => {
            h2013
        }
    });
    let v2: u32 = f360(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h2014, _, _) => {
            h2014
        }
    }, f360(f418(match v0.clone() {
        Adt25::C0(_, _, _, _, h2015, _, _, _, _, _) => {
            h2015
        }
    }), f360(f419(match v0.clone() {
        Adt25::C0(_, _, _, _, h2016, _, _, _, _, _) => {
            h2016
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2017, _, _, _, _) => {
            h2017
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2018, _, _, _) => {
            h2018
        }
    }), f360(v1, {
        let m2021 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h2020, _) => {
                h2020
            }
        };
        match m2021 {
            Adt24::C0(h2019, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2019
            }
        }
    }))));
    let v3: u32 = f360(f420(match v0.clone() {
        Adt25::C0(h2022, _, _, _, _, _, _, _, _, _) => {
            h2022
        }
    }), f421(match v0.clone() {
        Adt25::C0(h2023, _, _, _, _, _, _, _, _, _) => {
            h2023
        }
    }, match v0.clone() {
        Adt25::C0(_, h2024, _, _, _, _, _, _, _, _) => {
            h2024
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2025, _, _, _, _, _, _, _) => {
            h2025
        }
    }));
    Ok(Adt25::C0(f360(v2, v3), match v0.clone() {
        Adt25::C0(h2026, _, _, _, _, _, _, _, _, _) => {
            h2026
        }
    }, match v0.clone() {
        Adt25::C0(_, h2027, _, _, _, _, _, _, _, _) => {
            h2027
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2028, _, _, _, _, _, _, _) => {
            h2028
        }
    }, f360(match v0.clone() {
        Adt25::C0(_, _, _, h2029, _, _, _, _, _, _) => {
            h2029
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h2030, _, _, _, _, _) => {
            h2030
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2031, _, _, _, _) => {
            h2031
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2032, _, _, _) => {
            h2032
        }
    }, f422(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h2033, _) => {
            h2033
        }
    }), {
        let a2035 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h2034) => {
                h2034
            }
        };
        let a2036 = 1u64;
        nat_add(a2035, a2036)?
    }))
}

pub fn f389(v0: u32) -> u64 {
    let m2038 = {
        let a2037 = v0;
        fixed_u64::convert(i128::from(a2037))
    };
    match m2038 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f390(v0: u64) -> u32 {
    let m2042 = {
        let a2041 = {
            let a2039 = v0;
            let a2040 = 4294967295u64;
            fixed_u64::bit_and(a2039, a2040)
        };
        fixed_u32::convert(i128::from(a2041))
    };
    match m2042 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f391(v0: u64) -> u32 {
    let (_, h2043) = {
        let (_, h2044) = f423(Fn20::F424, 32u64, (v0, (1u32, 0u32)));
        h2044
    };
    h2043
}

pub fn f392(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2049 = f425(v0, 24u32);
    let a2050 = {
        let a2047 = f425(v1, 16u32);
        let a2048 = {
            let a2045 = f425(v2, 8u32);
            let a2046 = v3;
            fixed_u32::bit_or(a2045, a2046)
        };
        fixed_u32::bit_or(a2047, a2048)
    };
    fixed_u32::bit_or(a2049, a2050)
}

pub fn f393(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2051 = v2;
    let a2052 = v1;
    if nat_lt(a2051, a2052) {
        let m2055 = {
            let a2053 = v0;
            let a2054 = v2;
            index_bytes(a2053, a2054)
        };
        match m2055 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2057 = {
                    let a2056 = v3;
                    fixed_u32::convert(i128::from(a2056))
                };
                match m2057 {
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
        let a2058 = v2;
        let a2059 = v1;
        if nat_eq(a2058, a2059) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f394(v0: u8) -> bool {
    let a2060 = v0;
    let a2061 = 32u8;
    if if if equal(a2060, a2061) {
        true
    } else {
        let a2062 = v0;
        let a2063 = 9u8;
        equal(a2062, a2063)
    } {
        true
    } else {
        let a2064 = v0;
        let a2065 = 10u8;
        equal(a2064, a2065)
    } {
        true
    } else {
        let a2066 = v0;
        let a2067 = 12u8;
        if equal(a2066, a2067) {
            true
        } else {
            let a2068 = v0;
            let a2069 = 13u8;
            equal(a2068, a2069)
        }
    }
}

pub fn f395(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2070, _) = v1.clone();
        h2070
    };
    let v3: Bytes = {
        let (h2071, _) = {
            let (_, h2072) = v1.clone();
            h2072
        };
        h2071
    };
    let a2075 = {
        let (_, h2073) = {
            let (_, h2074) = v1;
            h2074
        };
        h2073
    };
    let a2076 = 0u64;
    if nat_eq(a2075, a2076) {
        let v4: u64 = f325(v0.clone(), v2)?;
        let a2077 = v4;
        let a2078 = 34u64;
        if nat_eq(a2077, a2078) {
            Ok(Some(({
                let a2079 = v2;
                let a2080 = 1u64;
                nat_add(a2079, a2080)?
            }, (v3, 1u64))))
        } else {
            let a2081 = v4;
            let a2082 = 92u64;
            if nat_eq(a2081, a2082) {
                f426(v0, v2, v3)
            } else {
                let a2083 = v4;
                let a2084 = 32u64;
                if if nat_lt(a2083, a2084) {
                    true
                } else {
                    let a2085 = v4;
                    let a2086 = 256u64;
                    nat_eq(a2085, a2086)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2087 = v2;
                        let a2088 = 1u64;
                        nat_add(a2087, a2088)?
                    }, ({
                        let a2089 = v3;
                        let a2090 = f32(v0, v2, 1u64);
                        append_bytes(a2089, a2090)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f396(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f427(v0, v1)
}

pub fn f397(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f428(v0, v1)
}

pub fn f398(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2091 = v0.clone();
        length_bytes(a2091)
    };
    let a2092 = 0u64;
    let a2093 = v1;
    if nat_lt(a2092, a2093) {
        let a2094 = f429(v0.clone())?;
        let a2095 = v1;
        if nat_eq(a2094, a2095) {
            let a2096 = 1u64;
            let a2097 = v1;
            if nat_lt(a2096, a2097) {
                let m2100 = {
                    let a2098 = v0;
                    let a2099 = 0u64;
                    index_bytes(a2098, a2099)
                };
                match m2100 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2101 = v2;
                        let a2102 = 48u8;
                        Ok(!equal(a2101, a2102))
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

pub fn f399(v0: Fn16, p2107: u64, p2108: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2106 = (p2107, p2108);
    {
        let mut m2106 = m2106;
        loop {
            let step = {
                let (v1, v2) = m2106;
                let m2103 = v1;
                if m2103 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2103 - 1;
                    let m2105 = {
                        let c2104: Fn16 = v0.clone();
                        c2104.apply(v2)?
                    };
                    match m2105 {
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
                Ok(next) => { m2106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f430(v0, v1)
}

pub fn f401(v0: Bytes) -> R<u64> {
    let (h2109, _) = f228(Fn3::F431(v0.clone()), {
        let a2111 = {
            let a2110 = v0;
            length_bytes(a2110)
        };
        let a2112 = 1u64;
        nat_add(a2111, a2112)?
    }, 0u64)?;
    Ok(h2109)
}

pub fn f402(v0: u64) -> Bytes {
    let m2113 = v0;
    if m2113 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2113 - 1;
        let a2114 = f8(Str::lit("0"));
        let a2115 = f402(v1);
        append_bytes(a2114, a2115)
    }
}

pub fn f403(p2118: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2117 = p2118;
    {
        let mut m2117 = m2117;
        loop {
            let step = {
                let v0 = m2117;
                let m2116 = v0;
                match m2116.uncons() {
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
                Ok(next) => { m2117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: Adt14, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2119 = v0;
    match m2119 {
        Adt14::C0(_) => {
            Ok(v1)
        }
        Adt14::C1(_, _) => {
            Ok(v1)
        }
        Adt14::C2(_) => {
            Ok(v1)
        }
        Adt14::C3(r2122) => {
            let v2 = (*r2122).clone();
            f404(v2, v1)
        }
        Adt14::C4(r2123) => {
            let v3 = (*r2123).clone();
            f404(v3, v1)
        }
        Adt14::C5 => {
            Ok(v1)
        }
        Adt14::C6(r2124, r2125) => {
            let v4 = (*r2124).clone();
            let v5 = (*r2125).clone();
            f404(v4, f404(v5, v1)?)
        }
        Adt14::C7(v6, r2126, r2127) => {
            let v7 = (*r2126).clone();
            let v8 = (*r2127).clone();
            let v9: List<Bytes> = f404(v7.clone(), f404(v8, v1)?)?;
            if f36(v6, f8(Str::lit("$ref"))) {
                let m2120 = f150(Some(v7));
                match m2120 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2121 = f432(v10)?;
                        match m2121 {
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

pub fn f405(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2128 = f142(v1.clone(), f8(Str::lit("enum")));
    match m2128 {
        None => {
            let m2130 = f142(v1.clone(), f8(Str::lit("oneOf")));
            match m2130 {
                None => {
                    if f437(v1.clone(), v0.clone())? {
                        let m2131 = f438(v0.clone(), f333(f142(v1.clone(), f8(Str::lit("properties")))), f73(f146(f142(v1, f8(Str::lit("required"))))), true)?;
                        match m2131 {
                            Ok(v7) => {
                                Ok(f435(f439(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f204({
                                    let (h2132, _) = v7;
                                    h2132
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h2133) = v7;
                                    h2133
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt23>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m2134 = f333(f142(v1.clone(), f8(Str::lit("properties"))));
                        match m2134.uncons() {
                            None => {
                                let m2137 = f440(v1, {
                                    let a2135 = v0.clone();
                                    let a2136 = f8(Str::lit("Value"));
                                    append_bytes(a2135, a2136)
                                })?;
                                match m2137 {
                                    Ok(v9) => {
                                        Ok(f435(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h2138, _) = v9;
                                            h2138
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h2139) = v9;
                                            h2139
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt23>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2140 = f438(v0.clone(), f333(f142(v1.clone(), f8(Str::lit("properties")))), f73(f146(f142(v1, f8(Str::lit("required"))))), false)?;
                                match m2140 {
                                    Ok(v13) => {
                                        Ok(f435(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2142 = {
                                                let (h2141, _) = v13.clone();
                                                h2141
                                            };
                                            match m2142.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f204({
                                                        let (h2143, _) = v13;
                                                        h2143
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h2144) = v13;
                                            h2144
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
                    f436(v0, f222(v5))
                }
            }
        }
        Some(v2) => {
            let m2129 = f433(f222(v2), List::<Bytes>::nil())?;
            match m2129 {
                Ok(v4) => {
                    Ok(f435(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f204(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt23>::nil()))
                }
                Err(v3) => {
                    Ok(f434(v3, v0))
                }
            }
        }
    }
}

pub fn f406(v0: Adt14) -> Adt14 {
    let v2: Adt14 = {
        let m2145 = f142(v0.clone(), f8(Str::lit("anyOf")));
        match m2145 {
            None => {
                v0
            }
            Some(v1) => {
                if f441(f222(v1.clone())) {
                    f442(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f443(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f444(f222(v1.clone()))
                    } {
                        v0
                    } else {
                        f445(f442(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt14 = {
        let m2146 = f142(v2.clone(), f8(Str::lit("oneOf")));
        match m2146 {
            None => {
                v2
            }
            Some(v3) => {
                if f441(f222(v3)) {
                    f442(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2147 = f142(v4.clone(), f8(Str::lit("allOf")));
    match m2147 {
        None => {
            v4
        }
        Some(v5) => {
            if f446(f222(v5)) {
                f442(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f407(v0: Bytes, v1: u64) -> Bytes {
    let m2151 = {
        let a2148 = v0;
        let a2149 = v1;
        let a2150 = 1u64;
        slice_bytes(a2148, a2149, a2150)
    };
    match m2151 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f408(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f447(v1, v0, v2)
}

pub fn f409(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2196 = {
            let a2172 = {
                let a2160 = {
                    let a2154 = {
                        let a2152 = v3;
                        let a2153 = 128u8;
                        fixed_u8::bit_and(a2152, a2153)
                    };
                    let a2155 = 128u8;
                    if equal(a2154, a2155) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2161 = {
                    let a2158 = {
                        let a2156 = v3;
                        let a2157 = 64u8;
                        fixed_u8::bit_and(a2156, a2157)
                    };
                    let a2159 = 64u8;
                    if equal(a2158, a2159) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2160, a2161)?
            };
            let a2173 = {
                let a2170 = {
                    let a2164 = {
                        let a2162 = v3;
                        let a2163 = 32u8;
                        fixed_u8::bit_and(a2162, a2163)
                    };
                    let a2165 = 32u8;
                    if equal(a2164, a2165) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2171 = {
                    let a2168 = {
                        let a2166 = v3;
                        let a2167 = 16u8;
                        fixed_u8::bit_and(a2166, a2167)
                    };
                    let a2169 = 16u8;
                    if equal(a2168, a2169) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2170, a2171)?
            };
            nat_add(a2172, a2173)?
        };
        let a2197 = {
            let a2194 = {
                let a2182 = {
                    let a2176 = {
                        let a2174 = v3;
                        let a2175 = 8u8;
                        fixed_u8::bit_and(a2174, a2175)
                    };
                    let a2177 = 8u8;
                    if equal(a2176, a2177) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2183 = {
                    let a2180 = {
                        let a2178 = v3;
                        let a2179 = 4u8;
                        fixed_u8::bit_and(a2178, a2179)
                    };
                    let a2181 = 4u8;
                    if equal(a2180, a2181) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2182, a2183)?
            };
            let a2195 = {
                let a2192 = {
                    let a2186 = {
                        let a2184 = v3;
                        let a2185 = 2u8;
                        fixed_u8::bit_and(a2184, a2185)
                    };
                    let a2187 = 2u8;
                    if equal(a2186, a2187) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2193 = {
                    let a2190 = {
                        let a2188 = v3;
                        let a2189 = 1u8;
                        fixed_u8::bit_and(a2188, a2189)
                    };
                    let a2191 = 1u8;
                    if equal(a2190, a2191) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2192, a2193)?
            };
            nat_add(a2194, a2195)?
        };
        nat_add(a2196, a2197)?
    };
    let a2198 = v4;
    let a2199 = 34u64;
    if nat_eq(a2198, a2199) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2200 = v4;
        let a2201 = 92u64;
        if nat_eq(a2200, a2201) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2202 = v4;
            let a2203 = 8u64;
            if nat_eq(a2202, a2203) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2204 = v4;
                let a2205 = 12u64;
                if nat_eq(a2204, a2205) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2206 = v4;
                    let a2207 = 10u64;
                    if nat_eq(a2206, a2207) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2208 = v4;
                        let a2209 = 13u64;
                        if nat_eq(a2208, a2209) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2210 = v4;
                            let a2211 = 9u64;
                            if nat_eq(a2210, a2211) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2212 = v4;
                                let a2213 = 32u64;
                                if nat_lt(a2212, a2213) {
                                    let a2214 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2215 = f448(v2)?;
                                    Ok(append_bytes(a2214, a2215))
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

pub fn f410(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f449(Fn21::F450, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f411(v0: Fn17, p2219: List<List<(Bytes, Bytes)>>, p2220: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2218 = (p2219, p2220);
    {
        let mut m2218 = m2218;
        loop {
            let step = {
                let (v1, v2) = m2218;
                let m2216 = v2;
                match m2216.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2217: Fn17 = v0.clone();
                            c2217.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f413(v0: Fn18, p2225: u64, p2226: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2224 = (p2225, p2226);
    {
        let mut m2224 = m2224;
        loop {
            let step = {
                let (v1, v2) = m2224;
                let m2221 = v1;
                if m2221 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2221 - 1;
                    let m2223 = {
                        let c2222: Fn18 = v0.clone();
                        c2222.apply(v2.clone())?
                    };
                    match m2223 {
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
                Ok(next) => { m2224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f414(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f451(v0)
}

pub fn f415(v0: Fn19, p2231: u64, p2232: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2230 = (p2231, p2232);
    {
        let mut m2230 = m2230;
        loop {
            let step = {
                let (v1, v2) = m2230;
                let m2227 = v1;
                if m2227 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2227 - 1;
                    let m2229 = {
                        let c2228: Fn19 = v0.clone();
                        c2228.apply(v2.clone())?
                    };
                    match m2229 {
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
                Ok(next) => { m2230 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f452(v0)
}

pub fn f417(v0: u64) -> u32 {
    let a2233 = v0;
    let a2234 = 32u64;
    if nat_lt(a2233, a2234) {
        let a2235 = v0;
        let a2236 = 16u64;
        if nat_lt(a2235, a2236) {
            let a2237 = v0;
            let a2238 = 8u64;
            if nat_lt(a2237, a2238) {
                let a2239 = v0;
                let a2240 = 4u64;
                if nat_lt(a2239, a2240) {
                    let a2241 = v0;
                    let a2242 = 2u64;
                    if nat_lt(a2241, a2242) {
                        let a2243 = v0;
                        let a2244 = 1u64;
                        if nat_lt(a2243, a2244) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2245 = v0;
                        let a2246 = 3u64;
                        if nat_lt(a2245, a2246) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2247 = v0;
                    let a2248 = 6u64;
                    if nat_lt(a2247, a2248) {
                        let a2249 = v0;
                        let a2250 = 5u64;
                        if nat_lt(a2249, a2250) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2251 = v0;
                        let a2252 = 7u64;
                        if nat_lt(a2251, a2252) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2253 = v0;
                let a2254 = 12u64;
                if nat_lt(a2253, a2254) {
                    let a2255 = v0;
                    let a2256 = 10u64;
                    if nat_lt(a2255, a2256) {
                        let a2257 = v0;
                        let a2258 = 9u64;
                        if nat_lt(a2257, a2258) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2259 = v0;
                        let a2260 = 11u64;
                        if nat_lt(a2259, a2260) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2261 = v0;
                    let a2262 = 14u64;
                    if nat_lt(a2261, a2262) {
                        let a2263 = v0;
                        let a2264 = 13u64;
                        if nat_lt(a2263, a2264) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2265 = v0;
                        let a2266 = 15u64;
                        if nat_lt(a2265, a2266) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2267 = v0;
            let a2268 = 24u64;
            if nat_lt(a2267, a2268) {
                let a2269 = v0;
                let a2270 = 20u64;
                if nat_lt(a2269, a2270) {
                    let a2271 = v0;
                    let a2272 = 18u64;
                    if nat_lt(a2271, a2272) {
                        let a2273 = v0;
                        let a2274 = 17u64;
                        if nat_lt(a2273, a2274) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2275 = v0;
                        let a2276 = 19u64;
                        if nat_lt(a2275, a2276) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2277 = v0;
                    let a2278 = 22u64;
                    if nat_lt(a2277, a2278) {
                        let a2279 = v0;
                        let a2280 = 21u64;
                        if nat_lt(a2279, a2280) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2281 = v0;
                        let a2282 = 23u64;
                        if nat_lt(a2281, a2282) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2283 = v0;
                let a2284 = 28u64;
                if nat_lt(a2283, a2284) {
                    let a2285 = v0;
                    let a2286 = 26u64;
                    if nat_lt(a2285, a2286) {
                        let a2287 = v0;
                        let a2288 = 25u64;
                        if nat_lt(a2287, a2288) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2289 = v0;
                        let a2290 = 27u64;
                        if nat_lt(a2289, a2290) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2291 = v0;
                    let a2292 = 30u64;
                    if nat_lt(a2291, a2292) {
                        let a2293 = v0;
                        let a2294 = 29u64;
                        if nat_lt(a2293, a2294) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2295 = v0;
                        let a2296 = 31u64;
                        if nat_lt(a2295, a2296) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2297 = v0;
        let a2298 = 48u64;
        if nat_lt(a2297, a2298) {
            let a2299 = v0;
            let a2300 = 40u64;
            if nat_lt(a2299, a2300) {
                let a2301 = v0;
                let a2302 = 36u64;
                if nat_lt(a2301, a2302) {
                    let a2303 = v0;
                    let a2304 = 34u64;
                    if nat_lt(a2303, a2304) {
                        let a2305 = v0;
                        let a2306 = 33u64;
                        if nat_lt(a2305, a2306) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2307 = v0;
                        let a2308 = 35u64;
                        if nat_lt(a2307, a2308) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2309 = v0;
                    let a2310 = 38u64;
                    if nat_lt(a2309, a2310) {
                        let a2311 = v0;
                        let a2312 = 37u64;
                        if nat_lt(a2311, a2312) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2313 = v0;
                        let a2314 = 39u64;
                        if nat_lt(a2313, a2314) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2315 = v0;
                let a2316 = 44u64;
                if nat_lt(a2315, a2316) {
                    let a2317 = v0;
                    let a2318 = 42u64;
                    if nat_lt(a2317, a2318) {
                        let a2319 = v0;
                        let a2320 = 41u64;
                        if nat_lt(a2319, a2320) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2321 = v0;
                        let a2322 = 43u64;
                        if nat_lt(a2321, a2322) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2323 = v0;
                    let a2324 = 46u64;
                    if nat_lt(a2323, a2324) {
                        let a2325 = v0;
                        let a2326 = 45u64;
                        if nat_lt(a2325, a2326) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2327 = v0;
                        let a2328 = 47u64;
                        if nat_lt(a2327, a2328) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2329 = v0;
            let a2330 = 56u64;
            if nat_lt(a2329, a2330) {
                let a2331 = v0;
                let a2332 = 52u64;
                if nat_lt(a2331, a2332) {
                    let a2333 = v0;
                    let a2334 = 50u64;
                    if nat_lt(a2333, a2334) {
                        let a2335 = v0;
                        let a2336 = 49u64;
                        if nat_lt(a2335, a2336) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2337 = v0;
                        let a2338 = 51u64;
                        if nat_lt(a2337, a2338) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2339 = v0;
                    let a2340 = 54u64;
                    if nat_lt(a2339, a2340) {
                        let a2341 = v0;
                        let a2342 = 53u64;
                        if nat_lt(a2341, a2342) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2343 = v0;
                        let a2344 = 55u64;
                        if nat_lt(a2343, a2344) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2345 = v0;
                let a2346 = 60u64;
                if nat_lt(a2345, a2346) {
                    let a2347 = v0;
                    let a2348 = 58u64;
                    if nat_lt(a2347, a2348) {
                        let a2349 = v0;
                        let a2350 = 57u64;
                        if nat_lt(a2349, a2350) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2351 = v0;
                        let a2352 = 59u64;
                        if nat_lt(a2351, a2352) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2353 = v0;
                    let a2354 = 62u64;
                    if nat_lt(a2353, a2354) {
                        let a2355 = v0;
                        let a2356 = 61u64;
                        if nat_lt(a2355, a2356) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2357 = v0;
                        let a2358 = 63u64;
                        if nat_lt(a2357, a2358) {
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

pub fn f418(v0: u32) -> u32 {
    let a2361 = {
        let a2359 = f453(v0, 6u32, 26u32);
        let a2360 = f453(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2359, a2360)
    };
    let a2362 = f453(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2361, a2362)
}

pub fn f419(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2368 = {
        let a2363 = v0;
        let a2364 = v1;
        fixed_u32::bit_and(a2363, a2364)
    };
    let a2369 = {
        let a2366 = {
            let a2365 = v0;
            fixed_u32::bit_not(a2365)
        };
        let a2367 = v2;
        fixed_u32::bit_and(a2366, a2367)
    };
    fixed_u32::bit_xor(a2368, a2369)
}

pub fn f420(v0: u32) -> u32 {
    let a2372 = {
        let a2370 = f453(v0, 2u32, 30u32);
        let a2371 = f453(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2370, a2371)
    };
    let a2373 = f453(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2372, a2373)
}

pub fn f421(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2382 = {
        let a2378 = {
            let a2374 = v0;
            let a2375 = v1;
            fixed_u32::bit_and(a2374, a2375)
        };
        let a2379 = {
            let a2376 = v0;
            let a2377 = v2;
            fixed_u32::bit_and(a2376, a2377)
        };
        fixed_u32::bit_xor(a2378, a2379)
    };
    let a2383 = {
        let a2380 = v1;
        let a2381 = v2;
        fixed_u32::bit_and(a2380, a2381)
    };
    fixed_u32::bit_xor(a2382, a2383)
}

pub fn f422(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h2384, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2384
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2385, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2385
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2386, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2386
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2387, _, _, _, _, _, _, _, _, _, _, _) => {
            h2387
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2388, _, _, _, _, _, _, _, _, _, _) => {
            h2388
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2389, _, _, _, _, _, _, _, _, _) => {
            h2389
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2390, _, _, _, _, _, _, _, _) => {
            h2390
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h2391, _, _, _, _, _, _, _) => {
            h2391
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2392, _, _, _, _, _, _) => {
            h2392
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h2393, _, _, _, _, _) => {
            h2393
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h2394, _, _, _, _) => {
            h2394
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2395, _, _, _) => {
            h2395
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2396, _, _) => {
            h2396
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2397, _) => {
            h2397
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2398) => {
            h2398
        }
    }, f360(f454(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2399, _) => {
            h2399
        }
    }), f360(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2400, _, _, _, _, _, _) => {
            h2400
        }
    }, f360(f455(match v0.clone() {
        Adt24::C0(_, h2401, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2401
        }
    }), match v0 {
        Adt24::C0(h2402, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2402
        }
    }))))
}

pub fn f423(v0: Fn20, p2406: u64, p2407: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2405 = (p2406, p2407);
    {
        let mut m2405 = m2405;
        loop {
            let step = {
                let (v1, v2) = m2405;
                let m2403 = v1;
                if m2403 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2403 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2404: Fn20 = v0.clone();
                        c2404.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2405 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f424(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2408, _) = v0;
        h2408
    };
    let v2: u32 = {
        let (h2409, _) = {
            let (_, h2410) = v0;
            h2410
        };
        h2409
    };
    let v3: u32 = {
        let (_, h2411) = {
            let (_, h2412) = v0;
            h2412
        };
        h2411
    };
    ({
        let a2413 = v1;
        let a2414 = 2u64;
        let a2415 = 0u64;
        nat_quot(a2413, a2414, a2415)
    }, (f425(v2, 1u32), {
        let a2419 = {
            let a2416 = v1;
            let a2417 = 2u64;
            let a2418 = 0u64;
            nat_rem(a2416, a2417, a2418)
        };
        let a2420 = 1u64;
        if nat_eq(a2419, a2420) {
            let a2421 = v3;
            let a2422 = v2;
            fixed_u32::bit_or(a2421, a2422)
        } else {
            v3
        }
    }))
}

pub fn f425(v0: u32, v1: u32) -> u32 {
    let m2425 = {
        let a2423 = v0;
        let a2424 = v1;
        fixed_u32::shift_left(a2423, a2424)
    };
    match m2425 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f426(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f325(v0.clone(), {
        let a2426 = v1;
        let a2427 = 1u64;
        nat_add(a2426, a2427)?
    })?;
    let v4: Bytes = {
        let a2428 = v3;
        let a2429 = 117u64;
        if nat_eq(a2428, a2429) {
            f456(v0, v1)?
        } else {
            f457(v3)
        }
    };
    let a2430 = v4.clone();
    let a2431 = Bytes::lit(&[]);
    if equal(a2430, a2431) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2432 = v3;
            let a2433 = 117u64;
            if nat_eq(a2432, a2433) {
                let a2434 = v1;
                let a2435 = 6u64;
                nat_add(a2434, a2435)?
            } else {
                let a2436 = v1;
                let a2437 = 2u64;
                nat_add(a2436, a2437)?
            }
        }, ({
            let a2438 = v2;
            let a2439 = v4;
            append_bytes(a2438, a2439)
        }, 0u64))))
    }
}

pub fn f427(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f325(v0, v1)?;
    let a2440 = 47u64;
    let a2441 = v2;
    if if nat_lt(a2440, a2441) {
        let a2442 = v2;
        let a2443 = 58u64;
        nat_lt(a2442, a2443)
    } else {
        false
    } {
        Ok(Some({
            let a2444 = v1;
            let a2445 = 1u64;
            nat_add(a2444, a2445)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f428(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2446 = f325(v0, v1)?;
    let a2447 = 48u64;
    if nat_eq(a2446, a2447) {
        Ok(Some({
            let a2448 = v1;
            let a2449 = 1u64;
            nat_add(a2448, a2449)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f429(v0: Bytes) -> R<u64> {
    let (h2450, _) = f228(Fn3::F458(v0.clone()), {
        let a2451 = v0;
        length_bytes(a2451)
    }, 0u64)?;
    Ok(h2450)
}

pub fn f430(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2455 = {
        let a2453 = v0;
        let a2454 = {
            let (h2452, _) = v1;
            h2452
        };
        index_bytes(a2453, a2454)
    };
    match m2455 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2457 = {
                    let (h2456, _) = v1;
                    h2456
                };
                let a2458 = 1u64;
                nat_add(a2457, a2458)?
            }, {
                let a2462 = {
                    let a2460 = {
                        let (_, h2459) = v1;
                        h2459
                    };
                    let a2461 = 10u64;
                    nat_mul(a2460, a2461)?
                };
                let a2463 = f459(v2)?;
                nat_add(a2462, a2463)?
            })))
        }
    }
}

pub fn f431(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f460(v0, v1)
}

pub fn f432(v0: Bytes) -> R<Option<Bytes>> {
    if f194(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f218(f32(v0.clone(), 8u64, {
            let a2465 = {
                let a2464 = v0;
                length_bytes(a2464)
            };
            let a2466 = 8u64;
            nat_sub(a2465, a2466)
        }))?))
    } else {
        if f194(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f218(f32(v0.clone(), 14u64, {
                let a2468 = {
                    let a2467 = v0;
                    length_bytes(a2467)
                };
                let a2469 = 14u64;
                nat_sub(a2468, a2469)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f433(v0: List<Adt14>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f461(f462(f73(v0.clone()))?, v0, v1)
}

pub fn f434(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt23>), Bytes> {
    Err::<(Bytes, List<Adt23>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f435(v0: Bytes, v1: List<Adt23>) -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((v0, v1))
}

pub fn f436(v0: Bytes, v1: List<Adt14>) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2470 = v1.clone();
    match m2470.uncons() {
        None => {
            Ok(f434(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2471 = f463(f333(f142(v2, f8(Str::lit("properties")))), v1.clone());
            match m2471.uncons() {
                None => {
                    let m2472 = f464(f462(f465(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2472 {
                        Ok(v5) => {
                            Ok(f435(f439(v0, {
                                let (h2473, _) = v5;
                                h2473
                            }), {
                                let (_, h2474) = v5;
                                h2474
                            }))
                        }
                        Err(v4) => {
                            Ok(f434(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2475 = f466(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2475 {
                        Ok(v9) => {
                            Ok(f435(f439(v0, {
                                let (h2476, _) = v9;
                                h2476
                            }), {
                                let (_, h2477) = v9;
                                h2477
                            }))
                        }
                        Err(v8) => {
                            Ok(f434(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f437(v0: Adt14, v1: Bytes) -> R<bool> {
    let m2478 = v0;
    match m2478 {
        Adt14::C0(_) => {
            Ok(false)
        }
        Adt14::C1(_, _) => {
            Ok(false)
        }
        Adt14::C2(_) => {
            Ok(false)
        }
        Adt14::C3(r2479) => {
            let v2 = (*r2479).clone();
            f437(v2, v1)
        }
        Adt14::C4(r2480) => {
            let v3 = (*r2480).clone();
            f437(v3, v1)
        }
        Adt14::C5 => {
            Ok(false)
        }
        Adt14::C6(r2481, r2482) => {
            let v4 = (*r2481).clone();
            let v5 = (*r2482).clone();
            if f437(v4, v1.clone())? {
                Ok(true)
            } else {
                f437(v5, v1)
            }
        }
        Adt14::C7(v6, r2483, r2484) => {
            let v7 = (*r2483).clone();
            let v8 = (*r2484).clone();
            if if f36(v6, f8(Str::lit("$ref"))) {
                f467(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f437(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f437(v8, v1)
                }
            }
        }
    }
}

pub fn f438(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    f468(f462(f469(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f439(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f204(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f440(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2485 = f470(v0.clone());
    match m2485 {
        None => {
            if f36(f145(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2486 = f142(v0, f8(Str::lit("items")));
                match m2486 {
                    None => {
                        Ok(f471(f8(Str::lit("List")), f473()))
                    }
                    Some(v3) => {
                        Ok(f471(f8(Str::lit("List")), f472(v3, {
                            let a2487 = v1;
                            let a2488 = f8(Str::lit("Item"));
                            append_bytes(a2487, a2488)
                        })?))
                    }
                }
            } else {
                f472(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f471(f8(Str::lit("Option")), f472(v2, v1)?))
        }
    }
}

pub fn f441(p2491: List<Adt14>) -> bool {
    let m2490 = p2491;
    {
        let mut m2490 = m2490;
        loop {
            let step = {
                let v0 = m2490;
                let m2489 = v0;
                match m2489.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f474(f370(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt14>, bool>(v2)
                        } else {
                            Err::<List<Adt14>, bool>(false)
                        }
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

pub fn f442(v0: Adt14, v1: Bytes) -> Adt14 {
    let m2492 = v0;
    match m2492 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r2493) => {
            let v10 = (*r2493).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r2494) => {
            let v5 = (*r2494).clone();
            Adt14::C4(std::rc::Rc::new(f442(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r2495, r2496) => {
            let v11 = (*r2495).clone();
            let v12 = (*r2496).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r2497, r2498) => {
            let v3 = (*r2497).clone();
            let v4 = (*r2498).clone();
            if f36(v2.clone(), v1.clone()) {
                f442(v4, v1)
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f442(v4, v1)))
            }
        }
    }
}

pub fn f443(v0: Adt14, v1: Bytes) -> bool {
    let m2499 = f142(v0, v1);
    match m2499 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f444(v0: List<Adt14>) -> bool {
    let m2500 = v0.clone();
    match m2500.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2501 = f463(f333(f142(v1, f8(Str::lit("properties")))), v0.clone());
            match m2501.uncons() {
                None => {
                    f476(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f475(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f445(v0: Adt14, v1: Bytes, v2: Adt14) -> Adt14 {
    let m2502 = v0;
    match m2502 {
        Adt14::C0(v7) => {
            Adt14::C0(v7)
        }
        Adt14::C1(v8, v9) => {
            Adt14::C1(v8, v9)
        }
        Adt14::C2(v10) => {
            Adt14::C2(v10)
        }
        Adt14::C3(r2503) => {
            let v11 = (*r2503).clone();
            Adt14::C3(std::rc::Rc::new(v11))
        }
        Adt14::C4(r2504) => {
            let v3 = (*r2504).clone();
            Adt14::C4(std::rc::Rc::new(Adt14::C7(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r2505, r2506) => {
            let v12 = (*r2505).clone();
            let v13 = (*r2506).clone();
            Adt14::C6(std::rc::Rc::new(v12), std::rc::Rc::new(v13))
        }
        Adt14::C7(v4, r2507, r2508) => {
            let v5 = (*r2507).clone();
            let v6 = (*r2508).clone();
            Adt14::C7(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f446(p2512: List<Adt14>) -> bool {
    let m2511 = p2512;
    {
        let mut m2511 = m2511;
        loop {
            let step = {
                let v0 = m2511;
                let m2509 = v0;
                match m2509.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f474(f370(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f441(f222({
                                let m2510 = f142(v1, f8(Str::lit("anyOf")));
                                match m2510 {
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
                Ok(next) => { m2511 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2513, _) = v2;
        h2513
    };
    let a2514 = v3;
    let a2515 = 0u64;
    if nat_eq(a2514, a2515) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2516 = v3;
            let a2517 = v1;
            let a2518 = 0u64;
            nat_quot(a2516, a2517, a2518)
        }, {
            let a2523 = f407(v0, {
                let a2519 = v3;
                let a2520 = v1;
                let a2521 = 0u64;
                nat_rem(a2519, a2520, a2521)
            });
            let a2524 = {
                let (_, h2522) = v2;
                h2522
            };
            append_bytes(a2523, a2524)
        }))
    }
}

pub fn f448(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2569 = {
            let a2545 = {
                let a2533 = {
                    let a2527 = {
                        let a2525 = v1;
                        let a2526 = 128u8;
                        fixed_u8::bit_and(a2525, a2526)
                    };
                    let a2528 = 128u8;
                    if equal(a2527, a2528) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2534 = {
                    let a2531 = {
                        let a2529 = v1;
                        let a2530 = 64u8;
                        fixed_u8::bit_and(a2529, a2530)
                    };
                    let a2532 = 64u8;
                    if equal(a2531, a2532) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2533, a2534)?
            };
            let a2546 = {
                let a2543 = {
                    let a2537 = {
                        let a2535 = v1;
                        let a2536 = 32u8;
                        fixed_u8::bit_and(a2535, a2536)
                    };
                    let a2538 = 32u8;
                    if equal(a2537, a2538) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2544 = {
                    let a2541 = {
                        let a2539 = v1;
                        let a2540 = 16u8;
                        fixed_u8::bit_and(a2539, a2540)
                    };
                    let a2542 = 16u8;
                    if equal(a2541, a2542) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2543, a2544)?
            };
            nat_add(a2545, a2546)?
        };
        let a2570 = {
            let a2567 = {
                let a2555 = {
                    let a2549 = {
                        let a2547 = v1;
                        let a2548 = 8u8;
                        fixed_u8::bit_and(a2547, a2548)
                    };
                    let a2550 = 8u8;
                    if equal(a2549, a2550) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2556 = {
                    let a2553 = {
                        let a2551 = v1;
                        let a2552 = 4u8;
                        fixed_u8::bit_and(a2551, a2552)
                    };
                    let a2554 = 4u8;
                    if equal(a2553, a2554) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2555, a2556)?
            };
            let a2568 = {
                let a2565 = {
                    let a2559 = {
                        let a2557 = v1;
                        let a2558 = 2u8;
                        fixed_u8::bit_and(a2557, a2558)
                    };
                    let a2560 = 2u8;
                    if equal(a2559, a2560) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2566 = {
                    let a2563 = {
                        let a2561 = v1;
                        let a2562 = 1u8;
                        fixed_u8::bit_and(a2561, a2562)
                    };
                    let a2564 = 1u8;
                    if equal(a2563, a2564) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2565, a2566)?
            };
            nat_add(a2567, a2568)?
        };
        nat_add(a2569, a2570)?
    };
    let a2577 = f407(f477(), {
        let a2571 = v2;
        let a2572 = 16u64;
        let a2573 = 0u64;
        nat_quot(a2571, a2572, a2573)
    });
    let a2578 = f407(f477(), {
        let a2574 = v2;
        let a2575 = 16u64;
        let a2576 = 0u64;
        nat_rem(a2574, a2575, a2576)
    });
    Ok(append_bytes(a2577, a2578))
}

pub fn f449(v0: Fn21, p2582: List<List<(Bytes, Bytes)>>, p2583: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2581 = (p2582, p2583);
    {
        let mut m2581 = m2581;
        loop {
            let step = {
                let (v1, v2) = m2581;
                let m2579 = v2;
                match m2579.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2580: Fn21 = v0.clone();
                            c2580.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2581 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f451(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2585 = {
        let a2584 = v0.clone();
        length_list(a2584)
    };
    let a2586 = 1u64;
    if nat_le(a2585, a2586) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f478(v0)?))
    }
}

pub fn f452(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2587, _) = v0;
        h2587
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2588) = v0;
        h2588
    };
    let m2589 = v1;
    match m2589.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2590 = v4;
            match m2590.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f479(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f453(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2591 = f318(v0, v1);
    let a2592 = f425(v0, v2);
    fixed_u32::bit_or(a2591, a2592)
}

pub fn f454(v0: u32) -> u32 {
    let a2595 = {
        let a2593 = f453(v0, 17u32, 15u32);
        let a2594 = f453(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2593, a2594)
    };
    let a2596 = f318(v0, 10u32);
    fixed_u32::bit_xor(a2595, a2596)
}

pub fn f455(v0: u32) -> u32 {
    let a2599 = {
        let a2597 = f453(v0, 7u32, 25u32);
        let a2598 = f453(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2597, a2598)
    };
    let a2600 = f318(v0, 3u32);
    fixed_u32::bit_xor(a2599, a2600)
}

pub fn f456(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2603 = f32(v0.clone(), {
        let a2601 = v1;
        let a2602 = 2u64;
        nat_add(a2601, a2602)?
    }, 2u64);
    let a2604 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2603, a2604) {
        let m2607 = f480(f32(v0, {
            let a2605 = v1;
            let a2606 = 4u64;
            nat_add(a2605, a2606)?
        }, 2u64))?;
        match m2607 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f325(v2.clone(), 0u64)?;
                let a2608 = v3;
                let a2609 = 32u64;
                if nat_lt(a2608, a2609) {
                    if f481(v3) {
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

pub fn f457(v0: u64) -> Bytes {
    let a2610 = v0;
    let a2611 = 34u64;
    if nat_eq(a2610, a2611) {
        Bytes::lit(&[0x22])
    } else {
        let a2612 = v0;
        let a2613 = 92u64;
        if nat_eq(a2612, a2613) {
            Bytes::lit(&[0x5c])
        } else {
            let a2614 = v0;
            let a2615 = 98u64;
            if nat_eq(a2614, a2615) {
                Bytes::lit(&[0x08])
            } else {
                let a2616 = v0;
                let a2617 = 102u64;
                if nat_eq(a2616, a2617) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2618 = v0;
                    let a2619 = 110u64;
                    if nat_eq(a2618, a2619) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2620 = v0;
                        let a2621 = 114u64;
                        if nat_eq(a2620, a2621) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2622 = v0;
                            let a2623 = 116u64;
                            if nat_eq(a2622, a2623) {
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

pub fn f458(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2626 = {
        let a2624 = v0;
        let a2625 = v1;
        index_bytes(a2624, a2625)
    };
    match m2626 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f482(v2)? {
                Ok(Some({
                    let a2627 = v1;
                    let a2628 = 1u64;
                    nat_add(a2627, a2628)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f459(v0: u8) -> R<u64> {
    let a2675 = {
        let v1: u8 = v0;
        let a2673 = {
            let a2649 = {
                let a2637 = {
                    let a2631 = {
                        let a2629 = v1;
                        let a2630 = 128u8;
                        fixed_u8::bit_and(a2629, a2630)
                    };
                    let a2632 = 128u8;
                    if equal(a2631, a2632) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2638 = {
                    let a2635 = {
                        let a2633 = v1;
                        let a2634 = 64u8;
                        fixed_u8::bit_and(a2633, a2634)
                    };
                    let a2636 = 64u8;
                    if equal(a2635, a2636) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2637, a2638)?
            };
            let a2650 = {
                let a2647 = {
                    let a2641 = {
                        let a2639 = v1;
                        let a2640 = 32u8;
                        fixed_u8::bit_and(a2639, a2640)
                    };
                    let a2642 = 32u8;
                    if equal(a2641, a2642) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2648 = {
                    let a2645 = {
                        let a2643 = v1;
                        let a2644 = 16u8;
                        fixed_u8::bit_and(a2643, a2644)
                    };
                    let a2646 = 16u8;
                    if equal(a2645, a2646) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2647, a2648)?
            };
            nat_add(a2649, a2650)?
        };
        let a2674 = {
            let a2671 = {
                let a2659 = {
                    let a2653 = {
                        let a2651 = v1;
                        let a2652 = 8u8;
                        fixed_u8::bit_and(a2651, a2652)
                    };
                    let a2654 = 8u8;
                    if equal(a2653, a2654) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2660 = {
                    let a2657 = {
                        let a2655 = v1;
                        let a2656 = 4u8;
                        fixed_u8::bit_and(a2655, a2656)
                    };
                    let a2658 = 4u8;
                    if equal(a2657, a2658) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2659, a2660)?
            };
            let a2672 = {
                let a2669 = {
                    let a2663 = {
                        let a2661 = v1;
                        let a2662 = 2u8;
                        fixed_u8::bit_and(a2661, a2662)
                    };
                    let a2664 = 2u8;
                    if equal(a2663, a2664) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2670 = {
                    let a2667 = {
                        let a2665 = v1;
                        let a2666 = 1u8;
                        fixed_u8::bit_and(a2665, a2666)
                    };
                    let a2668 = 1u8;
                    if equal(a2667, a2668) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2669, a2670)?
            };
            nat_add(a2671, a2672)?
        };
        nat_add(a2673, a2674)?
    };
    let a2676 = 48u64;
    Ok(nat_sub(a2675, a2676))
}

pub fn f460(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2677 = v0.clone();
        length_bytes(a2677)
    };
    let a2678 = v1;
    let a2679 = v2;
    if if nat_lt(a2678, a2679) {
        let a2684 = f325(v0, {
            let a2682 = v2;
            let a2683 = {
                let a2680 = v1;
                let a2681 = 1u64;
                nat_add(a2680, a2681)?
            };
            nat_sub(a2682, a2683)
        })?;
        let a2685 = 48u64;
        nat_eq(a2684, a2685)
    } else {
        false
    } {
        Ok(Some({
            let a2686 = v1;
            let a2687 = 1u64;
            nat_add(a2686, a2687)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f461(v0: bool, v1: List<Adt14>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2688 = v1;
    match m2688.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2689 = f483(v0, v3.clone())?;
            match m2689 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f484(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f85(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2690 = f461(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2690 {
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

pub fn f462(p2694: List<Bytes>) -> R<bool> {
    let m2693 = p2694;
    {
        let mut m2693 = m2693;
        loop {
            let step = {
                let v0 = m2693;
                let m2691 = v0;
                match m2691.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2692 = f485(v1)?;
                        match m2692 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f85(v3, f486(false, v2.clone())?) {
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
                Ok(next) => { m2693 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: List<(Bytes, Adt14)>, v1: List<Adt14>) -> List<Bytes> {
    let m2695 = v0;
    match m2695.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f487(v1.clone(), {
                let (h2696, _) = v2.clone();
                h2696
            }) {
                List::cons({
                    let (h2697, _) = v2;
                    h2697
                }, f463(v3, v1))
            } else {
                f463(v3, v1)
            }
        }
    }
}

pub fn f464(v0: bool, v1: Bytes, v2: List<Adt14>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2698 = v2;
    match m2698.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v4, v5)) => {
            let m2699 = f488(v4.clone())?;
            match m2699 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m2700 = f489(v0, v6.clone())?;
                    match m2700 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f85(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2703 = f490(v4, {
                                    let a2701 = v1.clone();
                                    let a2702 = f218(v6)?;
                                    append_bytes(a2701, a2702)
                                })?;
                                match m2703 {
                                    Ok(v9) => {
                                        let m2704 = f464(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m2704 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h2705, _) = v9;
                                                    h2705
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h2706, _) = v11;
                                                    h2706
                                                }), f337({
                                                    let (_, h2707) = v9;
                                                    h2707
                                                }, {
                                                    let (_, h2708) = v11;
                                                    h2708
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

pub fn f465(v0: List<Adt14>) -> R<List<Bytes>> {
    let m2709 = v0;
    match m2709.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m2710 = f488(v1)?;
            match m2710 {
                None => {
                    f465(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f465(v2)?))
                }
            }
        }
    }
}

pub fn f466(v0: Bytes, v1: List<Adt14>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    f491(f462(f492(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f467(v0: Adt14, v1: Bytes) -> R<bool> {
    let m2711 = f150(Some(v0));
    match m2711 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2712 = f432(v2)?;
            match m2712 {
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

pub fn f468(v0: bool, v1: Bytes, v2: List<(Bytes, Adt14)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2713 = v2;
    match m2713.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v5, v6)) => {
            let m2715 = f489(v0, {
                let (h2714, _) = v5.clone();
                h2714
            })?;
            match m2715 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h2716, _) = v5;
                        h2716
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f85(v7.clone(), f486(v0, f469(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2721 = f440({
                            let (_, h2717) = v5.clone();
                            h2717
                        }, {
                            let a2719 = v1.clone();
                            let a2720 = f218({
                                let (h2718, _) = v5.clone();
                                h2718
                            })?;
                            append_bytes(a2719, a2720)
                        })?;
                        match m2721 {
                            Ok(v9) => {
                                let v10: Bytes = if f85({
                                    let (h2722, _) = v5;
                                    h2722
                                }, v3.clone()) {
                                    let (h2723, _) = v9;
                                    h2723
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h2724, _) = v9;
                                        h2724
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m2725 = f468(v0, v1, v6, v3, v4)?;
                                match m2725 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h2726, _) = v12;
                                            h2726
                                        }), f337({
                                            let (_, h2727) = v9;
                                            h2727
                                        }, {
                                            let (_, h2728) = v12;
                                            h2728
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

pub fn f469(v0: List<(Bytes, Adt14)>) -> List<Bytes> {
    let m2729 = v0;
    match m2729.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2730, _) = v1;
                h2730
            }, f469(v2))
        }
    }
}

pub fn f470(v0: Adt14) -> Option<Adt14> {
    let m2731 = f146(f142(v0, f8(Str::lit("oneOf"))));
    match m2731.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2732 = v2;
            match m2732.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2733 = v4;
                    match m2733.uncons() {
                        None => {
                            if f493(v3.clone()) {
                                Some(v1)
                            } else {
                                if f493(v1) {
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

pub fn f471(v0: Bytes, v1: Result<(Bytes, List<Adt23>), Bytes>) -> Result<(Bytes, List<Adt23>), Bytes> {
    let m2734 = v1;
    match m2734 {
        Ok(v3) => {
            f435(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2735, _) = v3;
                h2735
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2736) = v3;
                h2736
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v2)
        }
    }
}

pub fn f472(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2737 = f470(v0.clone());
    match m2737 {
        None => {
            if f36(f145(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2738 = f142(v0, f8(Str::lit("items")));
                match m2738 {
                    None => {
                        Ok(f471(f8(Str::lit("List")), f473()))
                    }
                    Some(v3) => {
                        Ok(f471(f8(Str::lit("List")), f490(v3, {
                            let a2739 = v1;
                            let a2740 = f8(Str::lit("Item"));
                            append_bytes(a2739, a2740)
                        })?))
                    }
                }
            } else {
                f490(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f471(f8(Str::lit("Option")), f490(v2, v1)?))
        }
    }
}

pub fn f473() -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((f373(), List::cons(Adt23::C2, List::<Adt23>::nil())))
}

pub fn f474(p2744: List<(Bytes, Adt14)>, v1: List<Bytes>) -> bool {
    let m2743 = p2744;
    {
        let mut m2743 = m2743;
        loop {
            let step = {
                let v0 = m2743;
                let m2741 = v0;
                match m2741.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f85({
                            let (h2742, _) = v2;
                            h2742
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt14)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt14)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475(p2748: List<Adt14>, v1: Bytes, p2749: List<Bytes>) -> bool {
    let m2747 = (p2748, p2749);
    {
        let mut m2747 = m2747;
        loop {
            let step = {
                let (v0, v2) = m2747;
                let m2745 = v0;
                match m2745.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m2746 = f494(v3, v1.clone());
                        match m2746 {
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
                Ok(next) => { m2747 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(p2752: List<Adt14>, p2753: List<Bytes>) -> bool {
    let m2751 = (p2752, p2753);
    {
        let mut m2751 = m2751;
        loop {
            let step = {
                let (v0, v1) = m2751;
                let m2750 = v0;
                match m2750.uncons() {
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
                Ok(next) => { m2751 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f478(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f410({
        let (_, h2754) = {
            let (h2755, _) = f495(Fn22::F496, {
                let a2757 = {
                    let a2756 = v0.clone();
                    length_list(a2756)
                };
                let a2758 = 1u64;
                nat_add(a2757, a2758)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2755
        };
        h2754
    }))
}

pub fn f479(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2763 = {
            let a2761 = {
                let a2759 = v0.clone();
                length_list(a2759)
            };
            let a2762 = {
                let a2760 = v1.clone();
                length_list(a2760)
            };
            nat_add(a2761, a2762)?
        };
        let a2764 = 1u64;
        nat_add(a2763, a2764)?
    };
    Ok(f308({
        let (_, h2765) = {
            let (_, h2766) = {
                let (h2767, _) = f497(Fn23::F498, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2767
            };
            h2766
        };
        h2765
    }))
}

pub fn f480(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2768 = v0.clone();
        length_bytes(a2768)
    };
    let a2772 = {
        let a2769 = v1;
        let a2770 = 2u64;
        let a2771 = 0u64;
        nat_rem(a2769, a2770, a2771)
    };
    let a2773 = 0u64;
    if nat_eq(a2772, a2773) {
        let (_, h2774) = {
            let (h2775, _) = f499(Fn24::F500(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2775
        };
        Ok(h2774)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f481(v0: u64) -> bool {
    let a2776 = v0;
    let a2777 = 8u64;
    if nat_eq(a2776, a2777) {
        true
    } else {
        let a2778 = v0;
        let a2779 = 9u64;
        if nat_eq(a2778, a2779) {
            true
        } else {
            let a2780 = v0;
            let a2781 = 10u64;
            if nat_eq(a2780, a2781) {
                true
            } else {
                let a2782 = v0;
                let a2783 = 12u64;
                if nat_eq(a2782, a2783) {
                    true
                } else {
                    let a2784 = v0;
                    let a2785 = 13u64;
                    nat_eq(a2784, a2785)
                }
            }
        }
    }
}

pub fn f482(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2830 = {
            let a2806 = {
                let a2794 = {
                    let a2788 = {
                        let a2786 = v1;
                        let a2787 = 128u8;
                        fixed_u8::bit_and(a2786, a2787)
                    };
                    let a2789 = 128u8;
                    if equal(a2788, a2789) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2795 = {
                    let a2792 = {
                        let a2790 = v1;
                        let a2791 = 64u8;
                        fixed_u8::bit_and(a2790, a2791)
                    };
                    let a2793 = 64u8;
                    if equal(a2792, a2793) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2794, a2795)?
            };
            let a2807 = {
                let a2804 = {
                    let a2798 = {
                        let a2796 = v1;
                        let a2797 = 32u8;
                        fixed_u8::bit_and(a2796, a2797)
                    };
                    let a2799 = 32u8;
                    if equal(a2798, a2799) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2805 = {
                    let a2802 = {
                        let a2800 = v1;
                        let a2801 = 16u8;
                        fixed_u8::bit_and(a2800, a2801)
                    };
                    let a2803 = 16u8;
                    if equal(a2802, a2803) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2804, a2805)?
            };
            nat_add(a2806, a2807)?
        };
        let a2831 = {
            let a2828 = {
                let a2816 = {
                    let a2810 = {
                        let a2808 = v1;
                        let a2809 = 8u8;
                        fixed_u8::bit_and(a2808, a2809)
                    };
                    let a2811 = 8u8;
                    if equal(a2810, a2811) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2817 = {
                    let a2814 = {
                        let a2812 = v1;
                        let a2813 = 4u8;
                        fixed_u8::bit_and(a2812, a2813)
                    };
                    let a2815 = 4u8;
                    if equal(a2814, a2815) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2816, a2817)?
            };
            let a2829 = {
                let a2826 = {
                    let a2820 = {
                        let a2818 = v1;
                        let a2819 = 2u8;
                        fixed_u8::bit_and(a2818, a2819)
                    };
                    let a2821 = 2u8;
                    if equal(a2820, a2821) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2827 = {
                    let a2824 = {
                        let a2822 = v1;
                        let a2823 = 1u8;
                        fixed_u8::bit_and(a2822, a2823)
                    };
                    let a2825 = 1u8;
                    if equal(a2824, a2825) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2826, a2827)?
            };
            nat_add(a2828, a2829)?
        };
        nat_add(a2830, a2831)?
    };
    let a2832 = 48u64;
    let a2833 = v2;
    if nat_le(a2832, a2833) {
        let a2834 = v2;
        let a2835 = 57u64;
        Ok(nat_le(a2834, a2835))
    } else {
        Ok(false)
    }
}

pub fn f483(v0: bool, v1: Adt14) -> R<Option<Bytes>> {
    let m2836 = v1;
    match m2836 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v3, v4) => {
            Ok(Some({
                let a2839 = if if v3 {
                    let a2837 = 0u64;
                    let a2838 = v4;
                    nat_lt(a2837, a2838)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2840 = f288(false, v4)?;
                append_bytes(a2839, a2840)
            }))
        }
        Adt14::C2(v2) => {
            f489(v0, v2)
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

pub fn f484(v0: Adt14) -> R<Bytes> {
    f221(v0)
}

pub fn f485(v0: Bytes) -> R<Option<Bytes>> {
    if f501({
        let a2841 = v0.clone();
        length_bytes(a2841)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f502(v0)?;
        if f503(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2844 = {
                let a2842 = v1.clone();
                let a2843 = 0u64;
                index_bytes(a2842, a2843)
            };
            match m2844 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f331({
                        let v3: u8 = v2;
                        let a2889 = {
                            let a2865 = {
                                let a2853 = {
                                    let a2847 = {
                                        let a2845 = v3;
                                        let a2846 = 128u8;
                                        fixed_u8::bit_and(a2845, a2846)
                                    };
                                    let a2848 = 128u8;
                                    if equal(a2847, a2848) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2854 = {
                                    let a2851 = {
                                        let a2849 = v3;
                                        let a2850 = 64u8;
                                        fixed_u8::bit_and(a2849, a2850)
                                    };
                                    let a2852 = 64u8;
                                    if equal(a2851, a2852) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2853, a2854)?
                            };
                            let a2866 = {
                                let a2863 = {
                                    let a2857 = {
                                        let a2855 = v3;
                                        let a2856 = 32u8;
                                        fixed_u8::bit_and(a2855, a2856)
                                    };
                                    let a2858 = 32u8;
                                    if equal(a2857, a2858) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2864 = {
                                    let a2861 = {
                                        let a2859 = v3;
                                        let a2860 = 16u8;
                                        fixed_u8::bit_and(a2859, a2860)
                                    };
                                    let a2862 = 16u8;
                                    if equal(a2861, a2862) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2863, a2864)?
                            };
                            nat_add(a2865, a2866)?
                        };
                        let a2890 = {
                            let a2887 = {
                                let a2875 = {
                                    let a2869 = {
                                        let a2867 = v3;
                                        let a2868 = 8u8;
                                        fixed_u8::bit_and(a2867, a2868)
                                    };
                                    let a2870 = 8u8;
                                    if equal(a2869, a2870) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2876 = {
                                    let a2873 = {
                                        let a2871 = v3;
                                        let a2872 = 4u8;
                                        fixed_u8::bit_and(a2871, a2872)
                                    };
                                    let a2874 = 4u8;
                                    if equal(a2873, a2874) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2875, a2876)?
                            };
                            let a2888 = {
                                let a2885 = {
                                    let a2879 = {
                                        let a2877 = v3;
                                        let a2878 = 2u8;
                                        fixed_u8::bit_and(a2877, a2878)
                                    };
                                    let a2880 = 2u8;
                                    if equal(a2879, a2880) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2886 = {
                                    let a2883 = {
                                        let a2881 = v3;
                                        let a2882 = 1u8;
                                        fixed_u8::bit_and(a2881, a2882)
                                    };
                                    let a2884 = 1u8;
                                    if equal(a2883, a2884) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2885, a2886)?
                            };
                            nat_add(a2887, a2888)?
                        };
                        nat_add(a2889, a2890)?
                    }) {
                        Ok(Some({
                            let a2891 = f8(Str::lit("n"));
                            let a2892 = v1;
                            append_bytes(a2891, a2892)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f504({
            let a2893 = v0.clone();
            length_bytes(a2893)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f486(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2894 = v1;
    match m2894.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2895 = f489(v0, v2)?;
            match m2895 {
                None => {
                    f486(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f486(v0, v3)?))
                }
            }
        }
    }
}

pub fn f487(p2899: List<Adt14>, v1: Bytes) -> bool {
    let m2898 = p2899;
    {
        let mut m2898 = m2898;
        loop {
            let step = {
                let v0 = m2898;
                let m2896 = v0;
                match m2896.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m2897 = f494(v2, v1.clone());
                        if match m2897 {
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
                Ok(next) => { m2898 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(v0: Adt14) -> R<Option<Bytes>> {
    let m2900 = f150(f142(v0.clone(), f8(Str::lit("$ref"))));
    match m2900 {
        None => {
            Ok(f150(f142(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f432(v1)
        }
    }
}

pub fn f489(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f505(v1)
    } else {
        f485(v1)
    }
}

pub fn f490(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    f506(f406(v0), v1)
}

pub fn f491(v0: bool, v1: Bytes, v2: List<Adt14>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2901 = v2;
    match m2901.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v5, v6)) => {
            let m2902 = f494(v5.clone(), v3.clone());
            match m2902 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m2903 = f489(v0, v7.clone())?;
                    match m2903 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f85(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2906 = f438({
                                    let a2904 = v1.clone();
                                    let a2905 = f218(v7)?;
                                    append_bytes(a2904, a2905)
                                }, f372(f333(f142(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f73(f146(f142(v5, f8(Str::lit("required"))))), true)?;
                                match m2906 {
                                    Ok(v10) => {
                                        let m2907 = f491(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m2907 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f204({
                                                    let (h2908, _) = v10;
                                                    h2908
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h2909, _) = v12;
                                                    h2909
                                                }), f337({
                                                    let (_, h2910) = v10;
                                                    h2910
                                                }, {
                                                    let (_, h2911) = v12;
                                                    h2911
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

pub fn f492(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m2912 = v0;
    match m2912.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m2913 = f494(v2, v1.clone());
            match m2913 {
                None => {
                    f492(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f492(v3, v1))
                }
            }
        }
    }
}

pub fn f493(v0: Adt14) -> bool {
    f36(f145(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f494(v0: Adt14, v1: Bytes) -> Option<Bytes> {
    if f85(v1.clone(), f73(f146(f142(v0.clone(), f8(Str::lit("required")))))) {
        let m2914 = f142(v0, f8(Str::lit("properties")));
        match m2914 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m2915 = f142(v2, v1);
                match m2915 {
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

pub fn f495(v0: Fn22, p2920: u64, p2921: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2919 = (p2920, p2921);
    {
        let mut m2919 = m2919;
        loop {
            let step = {
                let (v1, v2) = m2919;
                let m2916 = v1;
                if m2916 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2916 - 1;
                    let m2918 = {
                        let c2917: Fn22 = v0.clone();
                        c2917.apply(v2.clone())?
                    };
                    match m2918 {
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
                Ok(next) => { m2919 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f496(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f507(v0)
}

pub fn f497(v0: Fn23, p2926: u64, p2927: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2925 = (p2926, p2927);
    {
        let mut m2925 = m2925;
        loop {
            let step = {
                let (v1, v2) = m2925;
                let m2922 = v1;
                if m2922 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2922 - 1;
                    let m2924 = {
                        let c2923: Fn23 = v0.clone();
                        c2923.apply(v2.clone())
                    };
                    match m2924 {
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
                Ok(next) => { m2925 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f508(v0)
}

pub fn f499(v0: Fn24, p2932: u64, p2933: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2931 = (p2932, p2933);
    {
        let mut m2931 = m2931;
        loop {
            let step = {
                let (v1, v2) = m2931;
                let m2928 = v1;
                if m2928 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2928 - 1;
                    let m2930 = {
                        let c2929: Fn24 = v0.clone();
                        c2929.apply(v2.clone())?
                    };
                    match m2930 {
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
                Ok(next) => { m2931 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f500(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f509(v0, v1)
}

pub fn f501(p2989: u64, v1: Bytes, p2990: u64) -> R<bool> {
    let m2988 = (p2989, p2990);
    {
        let mut m2988 = m2988;
        loop {
            let step = {
                let (v0, v2) = m2988;
                let m2934 = v0;
                if m2934 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2934 - 1;
                    let m2937 = {
                        let a2935 = v1.clone();
                        let a2936 = v2;
                        index_bytes(a2935, a2936)
                    };
                    match m2937 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2984 = {
                                let v5: u8 = v4;
                                let a2982 = {
                                    let a2958 = {
                                        let a2946 = {
                                            let a2940 = {
                                                let a2938 = v5;
                                                let a2939 = 128u8;
                                                fixed_u8::bit_and(a2938, a2939)
                                            };
                                            let a2941 = 128u8;
                                            if equal(a2940, a2941) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2947 = {
                                            let a2944 = {
                                                let a2942 = v5;
                                                let a2943 = 64u8;
                                                fixed_u8::bit_and(a2942, a2943)
                                            };
                                            let a2945 = 64u8;
                                            if equal(a2944, a2945) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2946, a2947)?
                                    };
                                    let a2959 = {
                                        let a2956 = {
                                            let a2950 = {
                                                let a2948 = v5;
                                                let a2949 = 32u8;
                                                fixed_u8::bit_and(a2948, a2949)
                                            };
                                            let a2951 = 32u8;
                                            if equal(a2950, a2951) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2957 = {
                                            let a2954 = {
                                                let a2952 = v5;
                                                let a2953 = 16u8;
                                                fixed_u8::bit_and(a2952, a2953)
                                            };
                                            let a2955 = 16u8;
                                            if equal(a2954, a2955) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2956, a2957)?
                                    };
                                    nat_add(a2958, a2959)?
                                };
                                let a2983 = {
                                    let a2980 = {
                                        let a2968 = {
                                            let a2962 = {
                                                let a2960 = v5;
                                                let a2961 = 8u8;
                                                fixed_u8::bit_and(a2960, a2961)
                                            };
                                            let a2963 = 8u8;
                                            if equal(a2962, a2963) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2969 = {
                                            let a2966 = {
                                                let a2964 = v5;
                                                let a2965 = 4u8;
                                                fixed_u8::bit_and(a2964, a2965)
                                            };
                                            let a2967 = 4u8;
                                            if equal(a2966, a2967) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2968, a2969)?
                                    };
                                    let a2981 = {
                                        let a2978 = {
                                            let a2972 = {
                                                let a2970 = v5;
                                                let a2971 = 2u8;
                                                fixed_u8::bit_and(a2970, a2971)
                                            };
                                            let a2973 = 2u8;
                                            if equal(a2972, a2973) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2979 = {
                                            let a2976 = {
                                                let a2974 = v5;
                                                let a2975 = 1u8;
                                                fixed_u8::bit_and(a2974, a2975)
                                            };
                                            let a2977 = 1u8;
                                            if equal(a2976, a2977) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2978, a2979)?
                                    };
                                    nat_add(a2980, a2981)?
                                };
                                nat_add(a2982, a2983)?
                            };
                            let a2985 = 128u64;
                            if nat_lt(a2984, a2985) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2986 = v2;
                                    let a2987 = 1u64;
                                    nat_add(a2986, a2987)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f284(v0.clone(), 0u64, {
        let a2991 = v0;
        length_bytes(a2991)
    }, false, Bytes::lit(&[]))?;
    let m2994 = {
        let a2992 = v1.clone();
        let a2993 = 0u64;
        index_bytes(a2992, a2993)
    };
    match m2994 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3039 = {
                    let a3015 = {
                        let a3003 = {
                            let a2997 = {
                                let a2995 = v3;
                                let a2996 = 128u8;
                                fixed_u8::bit_and(a2995, a2996)
                            };
                            let a2998 = 128u8;
                            if equal(a2997, a2998) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3004 = {
                            let a3001 = {
                                let a2999 = v3;
                                let a3000 = 64u8;
                                fixed_u8::bit_and(a2999, a3000)
                            };
                            let a3002 = 64u8;
                            if equal(a3001, a3002) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3003, a3004)?
                    };
                    let a3016 = {
                        let a3013 = {
                            let a3007 = {
                                let a3005 = v3;
                                let a3006 = 32u8;
                                fixed_u8::bit_and(a3005, a3006)
                            };
                            let a3008 = 32u8;
                            if equal(a3007, a3008) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3014 = {
                            let a3011 = {
                                let a3009 = v3;
                                let a3010 = 16u8;
                                fixed_u8::bit_and(a3009, a3010)
                            };
                            let a3012 = 16u8;
                            if equal(a3011, a3012) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3013, a3014)?
                    };
                    nat_add(a3015, a3016)?
                };
                let a3040 = {
                    let a3037 = {
                        let a3025 = {
                            let a3019 = {
                                let a3017 = v3;
                                let a3018 = 8u8;
                                fixed_u8::bit_and(a3017, a3018)
                            };
                            let a3020 = 8u8;
                            if equal(a3019, a3020) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3026 = {
                            let a3023 = {
                                let a3021 = v3;
                                let a3022 = 4u8;
                                fixed_u8::bit_and(a3021, a3022)
                            };
                            let a3024 = 4u8;
                            if equal(a3023, a3024) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3025, a3026)?
                    };
                    let a3038 = {
                        let a3035 = {
                            let a3029 = {
                                let a3027 = v3;
                                let a3028 = 2u8;
                                fixed_u8::bit_and(a3027, a3028)
                            };
                            let a3030 = 2u8;
                            if equal(a3029, a3030) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3036 = {
                            let a3033 = {
                                let a3031 = v3;
                                let a3032 = 1u8;
                                fixed_u8::bit_and(a3031, a3032)
                            };
                            let a3034 = 1u8;
                            if equal(a3033, a3034) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3035, a3036)?
                    };
                    nat_add(a3037, a3038)?
                };
                nat_add(a3039, a3040)?
            };
            if f330(v4) {
                let a3047 = f32({
                    let a3041 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3041)
                }, {
                    let a3042 = v4;
                    let a3043 = 65u64;
                    nat_sub(a3042, a3043)
                }, 1u64);
                let a3048 = f32(v1.clone(), 1u64, {
                    let a3045 = {
                        let a3044 = v1;
                        length_bytes(a3044)
                    };
                    let a3046 = 1u64;
                    nat_sub(a3045, a3046)
                });
                Ok(append_bytes(a3047, a3048))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f503(v0: Bytes) -> R<bool> {
    let m3051 = {
        let a3049 = v0;
        let a3050 = 0u64;
        index_bytes(a3049, a3050)
    };
    match m3051 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f329({
                let v2: u8 = v1;
                let a3096 = {
                    let a3072 = {
                        let a3060 = {
                            let a3054 = {
                                let a3052 = v2;
                                let a3053 = 128u8;
                                fixed_u8::bit_and(a3052, a3053)
                            };
                            let a3055 = 128u8;
                            if equal(a3054, a3055) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3061 = {
                            let a3058 = {
                                let a3056 = v2;
                                let a3057 = 64u8;
                                fixed_u8::bit_and(a3056, a3057)
                            };
                            let a3059 = 64u8;
                            if equal(a3058, a3059) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3060, a3061)?
                    };
                    let a3073 = {
                        let a3070 = {
                            let a3064 = {
                                let a3062 = v2;
                                let a3063 = 32u8;
                                fixed_u8::bit_and(a3062, a3063)
                            };
                            let a3065 = 32u8;
                            if equal(a3064, a3065) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3071 = {
                            let a3068 = {
                                let a3066 = v2;
                                let a3067 = 16u8;
                                fixed_u8::bit_and(a3066, a3067)
                            };
                            let a3069 = 16u8;
                            if equal(a3068, a3069) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3070, a3071)?
                    };
                    nat_add(a3072, a3073)?
                };
                let a3097 = {
                    let a3094 = {
                        let a3082 = {
                            let a3076 = {
                                let a3074 = v2;
                                let a3075 = 8u8;
                                fixed_u8::bit_and(a3074, a3075)
                            };
                            let a3077 = 8u8;
                            if equal(a3076, a3077) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3083 = {
                            let a3080 = {
                                let a3078 = v2;
                                let a3079 = 4u8;
                                fixed_u8::bit_and(a3078, a3079)
                            };
                            let a3081 = 4u8;
                            if equal(a3080, a3081) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3082, a3083)?
                    };
                    let a3095 = {
                        let a3092 = {
                            let a3086 = {
                                let a3084 = v2;
                                let a3085 = 2u8;
                                fixed_u8::bit_and(a3084, a3085)
                            };
                            let a3087 = 2u8;
                            if equal(a3086, a3087) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3093 = {
                            let a3090 = {
                                let a3088 = v2;
                                let a3089 = 1u8;
                                fixed_u8::bit_and(a3088, a3089)
                            };
                            let a3091 = 1u8;
                            if equal(a3090, a3091) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3092, a3093)?
                    };
                    nat_add(a3094, a3095)?
                };
                nat_add(a3096, a3097)?
            }) {
                Ok(true)
            } else {
                Ok(f330({
                    let v3: u8 = v1;
                    let a3142 = {
                        let a3118 = {
                            let a3106 = {
                                let a3100 = {
                                    let a3098 = v3;
                                    let a3099 = 128u8;
                                    fixed_u8::bit_and(a3098, a3099)
                                };
                                let a3101 = 128u8;
                                if equal(a3100, a3101) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a3107 = {
                                let a3104 = {
                                    let a3102 = v3;
                                    let a3103 = 64u8;
                                    fixed_u8::bit_and(a3102, a3103)
                                };
                                let a3105 = 64u8;
                                if equal(a3104, a3105) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3106, a3107)?
                        };
                        let a3119 = {
                            let a3116 = {
                                let a3110 = {
                                    let a3108 = v3;
                                    let a3109 = 32u8;
                                    fixed_u8::bit_and(a3108, a3109)
                                };
                                let a3111 = 32u8;
                                if equal(a3110, a3111) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a3117 = {
                                let a3114 = {
                                    let a3112 = v3;
                                    let a3113 = 16u8;
                                    fixed_u8::bit_and(a3112, a3113)
                                };
                                let a3115 = 16u8;
                                if equal(a3114, a3115) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3116, a3117)?
                        };
                        nat_add(a3118, a3119)?
                    };
                    let a3143 = {
                        let a3140 = {
                            let a3128 = {
                                let a3122 = {
                                    let a3120 = v3;
                                    let a3121 = 8u8;
                                    fixed_u8::bit_and(a3120, a3121)
                                };
                                let a3123 = 8u8;
                                if equal(a3122, a3123) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a3129 = {
                                let a3126 = {
                                    let a3124 = v3;
                                    let a3125 = 4u8;
                                    fixed_u8::bit_and(a3124, a3125)
                                };
                                let a3127 = 4u8;
                                if equal(a3126, a3127) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3128, a3129)?
                        };
                        let a3141 = {
                            let a3138 = {
                                let a3132 = {
                                    let a3130 = v3;
                                    let a3131 = 2u8;
                                    fixed_u8::bit_and(a3130, a3131)
                                };
                                let a3133 = 2u8;
                                if equal(a3132, a3133) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a3139 = {
                                let a3136 = {
                                    let a3134 = v3;
                                    let a3135 = 1u8;
                                    fixed_u8::bit_and(a3134, a3135)
                                };
                                let a3137 = 1u8;
                                if equal(a3136, a3137) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3138, a3139)?
                        };
                        nat_add(a3140, a3141)?
                    };
                    nat_add(a3142, a3143)?
                }))
            }
        }
    }
}

pub fn f504(p3156: u64, v1: Bytes, p3157: u64, p3158: Bytes) -> R<Option<Bytes>> {
    let m3155 = (p3156, (p3157, p3158));
    {
        let mut m3155 = m3155;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3155;
                let m3144 = v0;
                if m3144 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3144 - 1;
                    let a3146 = {
                        let a3145 = v1.clone();
                        length_bytes(a3145)
                    };
                    let a3147 = v2;
                    if nat_le(a3146, a3147) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3148 = f510(v1.clone(), v2)?;
                        match m3148 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3150 = v2;
                                    let a3151 = {
                                        let (_, h3149) = v5;
                                        h3149
                                    };
                                    nat_add(a3150, a3151)?
                                }, {
                                    let a3153 = v3;
                                    let a3154 = f511({
                                        let (h3152, _) = v5;
                                        h3152
                                    });
                                    append_bytes(a3153, a3154)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f505(v0: Bytes) -> R<Option<Bytes>> {
    let m3160 = f512({
        let a3159 = v0.clone();
        length_bytes(a3159)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m3160 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3162 = {
                let a3161 = v1.clone();
                length_bytes(a3161)
            };
            let a3163 = 0u64;
            if nat_eq(a3162, a3163) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f506(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m3164 = v0.clone();
    match m3164 {
        Adt14::C0(v2) => {
            if v2 {
                Ok(f473())
            } else {
                Ok(f434(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt14::C1(_, _) => {
            Ok(f434(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C2(_) => {
            Ok(f434(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C3(_) => {
            Ok(f434(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C4(_) => {
            let m3165 = f513(f370(v0.clone()));
            match m3165 {
                None => {
                    let m3166 = f150(f142(v0.clone(), f8(Str::lit("$ref"))));
                    match m3166 {
                        None => {
                            let m3168 = f142(v0.clone(), f8(Str::lit("allOf")));
                            match m3168 {
                                None => {
                                    let m3173 = f142(v0.clone(), f8(Str::lit("enum")));
                                    match m3173 {
                                        None => {
                                            let m3174 = f142(v0.clone(), f8(Str::lit("const")));
                                            match m3174 {
                                                None => {
                                                    let m3176 = f142(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3176 {
                                                        None => {
                                                            let v27: Bytes = f145(v0.clone(), f8(Str::lit("type")));
                                                            if if f36(v27.clone(), Bytes::lit(&[])) {
                                                                !f514(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f473())
                                                            } else {
                                                                if f36(v27.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f435(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                                                } else {
                                                                    if f36(v27.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f435(f515(v0), List::<Adt23>::nil()))
                                                                    } else {
                                                                        if f36(v27.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f435(f516(), List::<Adt23>::nil()))
                                                                        } else {
                                                                            if f36(v27.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f435(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                                                            } else {
                                                                                if f36(v27.clone(), f8(Str::lit("object"))) {
                                                                                    let m3177 = f333(f142(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3177.uncons() {
                                                                                        None => {
                                                                                            let m3178 = f517(v0);
                                                                                            match m3178 {
                                                                                                None => {
                                                                                                    Ok(f471(f8(Str::lit("List")), f518(f473())))
                                                                                                }
                                                                                                Some(v30) => {
                                                                                                    Ok(f471(f8(Str::lit("List")), f518(f519(v30, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f435(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f434(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v27, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f435(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v24) => {
                                                    let m3175 = f150(Some(v24));
                                                    match m3175 {
                                                        None => {
                                                            Ok(f434(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f435(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f435(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                        }
                                    }
                                }
                                Some(v16) => {
                                    let m3169 = f222(v16);
                                    match m3169.uncons() {
                                        None => {
                                            Ok(f434(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v17, v18)) => {
                                            let m3170 = v18;
                                            match m3170.uncons() {
                                                None => {
                                                    let m3171 = f150(f142(v17, f8(Str::lit("$ref"))));
                                                    match m3171 {
                                                        None => {
                                                            Ok(f434(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v21) => {
                                                            let m3172 = f432(v21)?;
                                                            match m3172 {
                                                                None => {
                                                                    Ok(f434(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v22) => {
                                                                    Ok(f435(v22, List::<Adt23>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f434(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v14) => {
                            let m3167 = f432(v14)?;
                            match m3167 {
                                None => {
                                    Ok(f434(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v15) => {
                                    Ok(f435(v15, List::<Adt23>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v13) => {
                    Ok(f434(if f36(v13.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v13, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt14::C5 => {
            Ok(f473())
        }
        Adt14::C6(_, _) => {
            Ok(f434(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C7(_, _, _) => {
            Ok(f434(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f507(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3179, _) = v0;
        h3179
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3180) = v0;
        h3180
    };
    let m3181 = v1;
    match m3181.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3182 = v4;
            match m3182.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f520(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f508(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3183, _) = v0.clone();
        h3183
    };
    let v2: List<(Bytes, ())> = {
        let (h3184, _) = {
            let (_, h3185) = v0.clone();
            h3185
        };
        h3184
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3186) = {
            let (_, h3187) = v0;
            h3187
        };
        h3186
    };
    let m3188 = v1.clone();
    match m3188.uncons() {
        None => {
            let m3189 = v2;
            match m3189.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3190 = v2.clone();
            match m3190.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f521({
                        let (h3191, _) = v8.clone();
                        h3191
                    }, {
                        let (h3192, _) = v6.clone();
                        h3192
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

pub fn f509(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3193, _) = v1;
        h3193
    };
    let m3195 = {
        let (_, h3194) = v1;
        h3194
    };
    match m3195 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3198 = {
                let a3196 = v0.clone();
                let a3197 = v2;
                index_bytes(a3196, a3197)
            };
            match m3198 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3203 = {
                        let a3201 = v0;
                        let a3202 = {
                            let a3199 = v2;
                            let a3200 = 1u64;
                            nat_add(a3199, a3200)?
                        };
                        index_bytes(a3201, a3202)
                    };
                    match m3203 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f522(v4)? {
                                f522(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3204 = v2;
                                    let a3205 = 2u64;
                                    nat_add(a3204, a3205)?
                                }, Some({
                                    let a3210 = v3;
                                    let a3211 = f407(f523(), {
                                        let a3208 = {
                                            let a3206 = f524(v4)?;
                                            let a3207 = 16u64;
                                            nat_mul(a3206, a3207)?
                                        };
                                        let a3209 = f524(v5)?;
                                        nat_add(a3208, a3209)?
                                    });
                                    append_bytes(a3210, a3211)
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

pub fn f510(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3214 = {
        let a3212 = v0.clone();
        let a3213 = v1;
        index_bytes(a3212, a3213)
    };
    match m3214 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3259 = {
                    let a3235 = {
                        let a3223 = {
                            let a3217 = {
                                let a3215 = v3;
                                let a3216 = 128u8;
                                fixed_u8::bit_and(a3215, a3216)
                            };
                            let a3218 = 128u8;
                            if equal(a3217, a3218) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3224 = {
                            let a3221 = {
                                let a3219 = v3;
                                let a3220 = 64u8;
                                fixed_u8::bit_and(a3219, a3220)
                            };
                            let a3222 = 64u8;
                            if equal(a3221, a3222) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3223, a3224)?
                    };
                    let a3236 = {
                        let a3233 = {
                            let a3227 = {
                                let a3225 = v3;
                                let a3226 = 32u8;
                                fixed_u8::bit_and(a3225, a3226)
                            };
                            let a3228 = 32u8;
                            if equal(a3227, a3228) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3234 = {
                            let a3231 = {
                                let a3229 = v3;
                                let a3230 = 16u8;
                                fixed_u8::bit_and(a3229, a3230)
                            };
                            let a3232 = 16u8;
                            if equal(a3231, a3232) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3233, a3234)?
                    };
                    nat_add(a3235, a3236)?
                };
                let a3260 = {
                    let a3257 = {
                        let a3245 = {
                            let a3239 = {
                                let a3237 = v3;
                                let a3238 = 8u8;
                                fixed_u8::bit_and(a3237, a3238)
                            };
                            let a3240 = 8u8;
                            if equal(a3239, a3240) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3246 = {
                            let a3243 = {
                                let a3241 = v3;
                                let a3242 = 4u8;
                                fixed_u8::bit_and(a3241, a3242)
                            };
                            let a3244 = 4u8;
                            if equal(a3243, a3244) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3245, a3246)?
                    };
                    let a3258 = {
                        let a3255 = {
                            let a3249 = {
                                let a3247 = v3;
                                let a3248 = 2u8;
                                fixed_u8::bit_and(a3247, a3248)
                            };
                            let a3250 = 2u8;
                            if equal(a3249, a3250) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3256 = {
                            let a3253 = {
                                let a3251 = v3;
                                let a3252 = 1u8;
                                fixed_u8::bit_and(a3251, a3252)
                            };
                            let a3254 = 1u8;
                            if equal(a3253, a3254) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3255, a3256)?
                    };
                    nat_add(a3257, a3258)?
                };
                nat_add(a3259, a3260)?
            };
            let a3261 = v4;
            let a3262 = 128u64;
            if nat_lt(a3261, a3262) {
                Ok(Some((v4, 1u64)))
            } else {
                let a3263 = 194u64;
                let a3264 = v4;
                if if nat_le(a3263, a3264) {
                    let a3265 = v4;
                    let a3266 = 223u64;
                    nat_le(a3265, a3266)
                } else {
                    false
                } {
                    let m3269 = f525(v0, {
                        let a3267 = v1;
                        let a3268 = 1u64;
                        nat_add(a3267, a3268)?
                    })?;
                    match m3269 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f526({
                                let a3274 = {
                                    let a3272 = {
                                        let a3270 = v4;
                                        let a3271 = 192u64;
                                        nat_sub(a3270, a3271)
                                    };
                                    let a3273 = 64u64;
                                    nat_mul(a3272, a3273)?
                                };
                                let a3275 = v5;
                                nat_add(a3274, a3275)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a3276 = 224u64;
                    let a3277 = v4;
                    if if nat_le(a3276, a3277) {
                        let a3278 = v4;
                        let a3279 = 239u64;
                        nat_le(a3278, a3279)
                    } else {
                        false
                    } {
                        let m3282 = f525(v0.clone(), {
                            let a3280 = v1;
                            let a3281 = 1u64;
                            nat_add(a3280, a3281)?
                        })?;
                        match m3282 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m3285 = f525(v0, {
                                    let a3283 = v1;
                                    let a3284 = 2u64;
                                    nat_add(a3283, a3284)?
                                })?;
                                match m3285 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f526({
                                            let a3294 = {
                                                let a3288 = {
                                                    let a3286 = v4;
                                                    let a3287 = 224u64;
                                                    nat_sub(a3286, a3287)
                                                };
                                                let a3289 = 4096u64;
                                                nat_mul(a3288, a3289)?
                                            };
                                            let a3295 = {
                                                let a3292 = {
                                                    let a3290 = v6;
                                                    let a3291 = 64u64;
                                                    nat_mul(a3290, a3291)?
                                                };
                                                let a3293 = v7;
                                                nat_add(a3292, a3293)?
                                            };
                                            nat_add(a3294, a3295)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a3296 = 240u64;
                        let a3297 = v4;
                        if if nat_le(a3296, a3297) {
                            let a3298 = v4;
                            let a3299 = 244u64;
                            nat_le(a3298, a3299)
                        } else {
                            false
                        } {
                            let m3302 = f525(v0.clone(), {
                                let a3300 = v1;
                                let a3301 = 1u64;
                                nat_add(a3300, a3301)?
                            })?;
                            match m3302 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m3305 = f525(v0.clone(), {
                                        let a3303 = v1;
                                        let a3304 = 2u64;
                                        nat_add(a3303, a3304)?
                                    })?;
                                    match m3305 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m3308 = f525(v0, {
                                                let a3306 = v1;
                                                let a3307 = 3u64;
                                                nat_add(a3306, a3307)?
                                            })?;
                                            match m3308 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f526({
                                                        let a3321 = {
                                                            let a3311 = {
                                                                let a3309 = v4;
                                                                let a3310 = 240u64;
                                                                nat_sub(a3309, a3310)
                                                            };
                                                            let a3312 = 262144u64;
                                                            nat_mul(a3311, a3312)?
                                                        };
                                                        let a3322 = {
                                                            let a3319 = {
                                                                let a3313 = v8;
                                                                let a3314 = 4096u64;
                                                                nat_mul(a3313, a3314)?
                                                            };
                                                            let a3320 = {
                                                                let a3317 = {
                                                                    let a3315 = v9;
                                                                    let a3316 = 64u64;
                                                                    nat_mul(a3315, a3316)?
                                                                };
                                                                let a3318 = v10;
                                                                nat_add(a3317, a3318)?
                                                            };
                                                            nat_add(a3319, a3320)?
                                                        };
                                                        nat_add(a3321, a3322)?
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

pub fn f511(v0: u64) -> Bytes {
    let a3323 = f8(Str::lit("u"));
    let a3324 = f527(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3323, a3324)
}

pub fn f512(p3337: u64, v1: Bytes, p3338: u64, p3339: Bytes) -> R<Option<Bytes>> {
    let m3336 = (p3337, (p3338, p3339));
    {
        let mut m3336 = m3336;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3336;
                let m3325 = v0;
                if m3325 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3325 - 1;
                    let a3327 = {
                        let a3326 = v1.clone();
                        length_bytes(a3326)
                    };
                    let a3328 = v2;
                    if nat_le(a3327, a3328) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3329 = f510(v1.clone(), v2)?;
                        match m3329 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3331 = v2;
                                    let a3332 = {
                                        let (_, h3330) = v5;
                                        h3330
                                    };
                                    nat_add(a3331, a3332)?
                                }, {
                                    let a3334 = v3;
                                    let a3335 = f528(v1.clone(), v2, {
                                        let (h3333, _) = v5;
                                        h3333
                                    });
                                    append_bytes(a3334, a3335)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3336 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(p3344: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m3343 = p3344;
    {
        let mut m3343 = m3343;
        loop {
            let step = {
                let v0 = m3343;
                let m3340 = v0;
                match m3340.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f85({
                            let (h3341, _) = v1.clone();
                            h3341
                        }, f529()) {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h3342, _) = v1;
                                h3342
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f514(v0: Adt14) -> bool {
    if f443(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f443(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f443(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f443(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f443(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f443(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f515(v0: Adt14) -> Bytes {
    let m3345 = f300(f142(v0, f8(Str::lit("minimum"))));
    match m3345 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f516() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f517(v0: Adt14) -> Option<Adt14> {
    let m3346 = f142(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3346 {
        None => {
            let m3347 = f333(f142(v0, f8(Str::lit("patternProperties"))));
            match m3347.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v2, v3)) => {
                    let m3348 = v3;
                    match m3348.uncons() {
                        None => {
                            Some({
                                let (_, h3349) = v2;
                                h3349
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

pub fn f518(v0: Result<(Bytes, List<Adt23>), Bytes>) -> Result<(Bytes, List<Adt23>), Bytes> {
    let m3350 = v0;
    match m3350 {
        Ok(v2) => {
            f435(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h3351, _) = v2;
                h3351
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h3352) = v2;
                h3352
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v1)
        }
    }
}

pub fn f519(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    f530(16u64, v0, v1)
}

pub fn f520(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3357 = {
            let a3355 = {
                let a3353 = v0.clone();
                length_list(a3353)
            };
            let a3356 = {
                let a3354 = v1.clone();
                length_list(a3354)
            };
            nat_add(a3355, a3356)?
        };
        let a3358 = 1u64;
        nat_add(a3357, a3358)?
    };
    Ok(f531({
        let (_, h3359) = {
            let (_, h3360) = {
                let (h3361, _) = f532(Fn25::F533, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3361
            };
            h3360
        };
        h3359
    }))
}

pub fn f521(v0: Bytes, v1: Bytes) -> bool {
    let a3366 = {
        let a3362 = v0;
        let a3363 = v1;
        compare_bytes(a3362, a3363)
    };
    let a3367 = {
        let a3364 = Bytes::lit(&[]);
        let a3365 = Bytes::lit(&[0x01]);
        compare_bytes(a3364, a3365)
    };
    equal(a3366, a3367)
}

pub fn f522(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3412 = {
            let a3388 = {
                let a3376 = {
                    let a3370 = {
                        let a3368 = v1;
                        let a3369 = 128u8;
                        fixed_u8::bit_and(a3368, a3369)
                    };
                    let a3371 = 128u8;
                    if equal(a3370, a3371) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3377 = {
                    let a3374 = {
                        let a3372 = v1;
                        let a3373 = 64u8;
                        fixed_u8::bit_and(a3372, a3373)
                    };
                    let a3375 = 64u8;
                    if equal(a3374, a3375) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3376, a3377)?
            };
            let a3389 = {
                let a3386 = {
                    let a3380 = {
                        let a3378 = v1;
                        let a3379 = 32u8;
                        fixed_u8::bit_and(a3378, a3379)
                    };
                    let a3381 = 32u8;
                    if equal(a3380, a3381) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3387 = {
                    let a3384 = {
                        let a3382 = v1;
                        let a3383 = 16u8;
                        fixed_u8::bit_and(a3382, a3383)
                    };
                    let a3385 = 16u8;
                    if equal(a3384, a3385) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3386, a3387)?
            };
            nat_add(a3388, a3389)?
        };
        let a3413 = {
            let a3410 = {
                let a3398 = {
                    let a3392 = {
                        let a3390 = v1;
                        let a3391 = 8u8;
                        fixed_u8::bit_and(a3390, a3391)
                    };
                    let a3393 = 8u8;
                    if equal(a3392, a3393) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3399 = {
                    let a3396 = {
                        let a3394 = v1;
                        let a3395 = 4u8;
                        fixed_u8::bit_and(a3394, a3395)
                    };
                    let a3397 = 4u8;
                    if equal(a3396, a3397) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3398, a3399)?
            };
            let a3411 = {
                let a3408 = {
                    let a3402 = {
                        let a3400 = v1;
                        let a3401 = 2u8;
                        fixed_u8::bit_and(a3400, a3401)
                    };
                    let a3403 = 2u8;
                    if equal(a3402, a3403) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3409 = {
                    let a3406 = {
                        let a3404 = v1;
                        let a3405 = 1u8;
                        fixed_u8::bit_and(a3404, a3405)
                    };
                    let a3407 = 1u8;
                    if equal(a3406, a3407) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3408, a3409)?
            };
            nat_add(a3410, a3411)?
        };
        nat_add(a3412, a3413)?
    };
    let a3414 = 48u64;
    let a3415 = v2;
    if if nat_le(a3414, a3415) {
        let a3416 = v2;
        let a3417 = 57u64;
        nat_le(a3416, a3417)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3418 = 97u64;
        let a3419 = v2;
        if nat_le(a3418, a3419) {
            let a3420 = v2;
            let a3421 = 102u64;
            Ok(nat_le(a3420, a3421))
        } else {
            Ok(false)
        }
    }
}

pub fn f523() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f524(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3466 = {
            let a3442 = {
                let a3430 = {
                    let a3424 = {
                        let a3422 = v1;
                        let a3423 = 128u8;
                        fixed_u8::bit_and(a3422, a3423)
                    };
                    let a3425 = 128u8;
                    if equal(a3424, a3425) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3431 = {
                    let a3428 = {
                        let a3426 = v1;
                        let a3427 = 64u8;
                        fixed_u8::bit_and(a3426, a3427)
                    };
                    let a3429 = 64u8;
                    if equal(a3428, a3429) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3430, a3431)?
            };
            let a3443 = {
                let a3440 = {
                    let a3434 = {
                        let a3432 = v1;
                        let a3433 = 32u8;
                        fixed_u8::bit_and(a3432, a3433)
                    };
                    let a3435 = 32u8;
                    if equal(a3434, a3435) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3441 = {
                    let a3438 = {
                        let a3436 = v1;
                        let a3437 = 16u8;
                        fixed_u8::bit_and(a3436, a3437)
                    };
                    let a3439 = 16u8;
                    if equal(a3438, a3439) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3440, a3441)?
            };
            nat_add(a3442, a3443)?
        };
        let a3467 = {
            let a3464 = {
                let a3452 = {
                    let a3446 = {
                        let a3444 = v1;
                        let a3445 = 8u8;
                        fixed_u8::bit_and(a3444, a3445)
                    };
                    let a3447 = 8u8;
                    if equal(a3446, a3447) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3453 = {
                    let a3450 = {
                        let a3448 = v1;
                        let a3449 = 4u8;
                        fixed_u8::bit_and(a3448, a3449)
                    };
                    let a3451 = 4u8;
                    if equal(a3450, a3451) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3452, a3453)?
            };
            let a3465 = {
                let a3462 = {
                    let a3456 = {
                        let a3454 = v1;
                        let a3455 = 2u8;
                        fixed_u8::bit_and(a3454, a3455)
                    };
                    let a3457 = 2u8;
                    if equal(a3456, a3457) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3463 = {
                    let a3460 = {
                        let a3458 = v1;
                        let a3459 = 1u8;
                        fixed_u8::bit_and(a3458, a3459)
                    };
                    let a3461 = 1u8;
                    if equal(a3460, a3461) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3462, a3463)?
            };
            nat_add(a3464, a3465)?
        };
        nat_add(a3466, a3467)?
    };
    let a3468 = v2;
    let a3469 = 57u64;
    if nat_le(a3468, a3469) {
        let a3470 = v2;
        let a3471 = 48u64;
        Ok(nat_sub(a3470, a3471))
    } else {
        let a3472 = v2;
        let a3473 = 87u64;
        Ok(nat_sub(a3472, a3473))
    }
}

pub fn f525(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3476 = {
        let a3474 = v0;
        let a3475 = v1;
        index_bytes(a3474, a3475)
    };
    match m3476 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3521 = {
                    let a3497 = {
                        let a3485 = {
                            let a3479 = {
                                let a3477 = v3;
                                let a3478 = 128u8;
                                fixed_u8::bit_and(a3477, a3478)
                            };
                            let a3480 = 128u8;
                            if equal(a3479, a3480) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3486 = {
                            let a3483 = {
                                let a3481 = v3;
                                let a3482 = 64u8;
                                fixed_u8::bit_and(a3481, a3482)
                            };
                            let a3484 = 64u8;
                            if equal(a3483, a3484) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3485, a3486)?
                    };
                    let a3498 = {
                        let a3495 = {
                            let a3489 = {
                                let a3487 = v3;
                                let a3488 = 32u8;
                                fixed_u8::bit_and(a3487, a3488)
                            };
                            let a3490 = 32u8;
                            if equal(a3489, a3490) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3496 = {
                            let a3493 = {
                                let a3491 = v3;
                                let a3492 = 16u8;
                                fixed_u8::bit_and(a3491, a3492)
                            };
                            let a3494 = 16u8;
                            if equal(a3493, a3494) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3495, a3496)?
                    };
                    nat_add(a3497, a3498)?
                };
                let a3522 = {
                    let a3519 = {
                        let a3507 = {
                            let a3501 = {
                                let a3499 = v3;
                                let a3500 = 8u8;
                                fixed_u8::bit_and(a3499, a3500)
                            };
                            let a3502 = 8u8;
                            if equal(a3501, a3502) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3508 = {
                            let a3505 = {
                                let a3503 = v3;
                                let a3504 = 4u8;
                                fixed_u8::bit_and(a3503, a3504)
                            };
                            let a3506 = 4u8;
                            if equal(a3505, a3506) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3507, a3508)?
                    };
                    let a3520 = {
                        let a3517 = {
                            let a3511 = {
                                let a3509 = v3;
                                let a3510 = 2u8;
                                fixed_u8::bit_and(a3509, a3510)
                            };
                            let a3512 = 2u8;
                            if equal(a3511, a3512) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3518 = {
                            let a3515 = {
                                let a3513 = v3;
                                let a3514 = 1u8;
                                fixed_u8::bit_and(a3513, a3514)
                            };
                            let a3516 = 1u8;
                            if equal(a3515, a3516) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3517, a3518)?
                    };
                    nat_add(a3519, a3520)?
                };
                nat_add(a3521, a3522)?
            };
            let a3523 = 128u64;
            let a3524 = v4;
            if if nat_le(a3523, a3524) {
                let a3525 = v4;
                let a3526 = 191u64;
                nat_le(a3525, a3526)
            } else {
                false
            } {
                Ok(Some({
                    let a3527 = v4;
                    let a3528 = 128u64;
                    nat_sub(a3527, a3528)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f526(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3529 = v1;
    let a3530 = v0;
    if if nat_le(a3529, a3530) {
        let a3531 = v0;
        let a3532 = v2;
        if nat_le(a3531, a3532) {
            let a3533 = v0;
            let a3534 = 55296u64;
            if nat_lt(a3533, a3534) {
                true
            } else {
                let a3535 = 57343u64;
                let a3536 = v0;
                nat_lt(a3535, a3536)
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

pub fn f527(p3553: u64, p3554: u64, p3555: u64, p3556: Bytes) -> Bytes {
    let m3552 = (p3553, (p3554, (p3555, p3556)));
    {
        let mut m3552 = m3552;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3552;
                let m3537 = v0;
                if m3537 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3537 - 1;
                    let a3538 = v1;
                    let a3539 = 0u64;
                    if if nat_eq(a3538, a3539) {
                        let a3540 = v2;
                        let a3541 = 0u64;
                        nat_eq(a3540, a3541)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3542 = v1;
                            let a3543 = 16u64;
                            let a3544 = 0u64;
                            nat_quot(a3542, a3543, a3544)
                        }, ({
                            let a3545 = v2;
                            let a3546 = 1u64;
                            nat_sub(a3545, a3546)
                        }, {
                            let a3550 = f534({
                                let a3547 = v1;
                                let a3548 = 16u64;
                                let a3549 = 0u64;
                                nat_rem(a3547, a3548, a3549)
                            });
                            let a3551 = v3;
                            append_bytes(a3550, a3551)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f329(v2) {
        true
    } else {
        f330(v2)
    } {
        let a3557 = v2;
        let a3558 = 117u64;
        if !nat_eq(a3557, a3558) {
            let a3559 = v2;
            let a3560 = 85u64;
            !nat_eq(a3559, a3560)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f331(v2) {
            let a3561 = 0u64;
            let a3562 = v1;
            nat_lt(a3561, a3562)
        } else {
            false
        }
    } {
        f32(v0, v1, 1u64)
    } else {
        let a3563 = v2;
        let a3564 = 65536u64;
        if nat_lt(a3563, a3564) {
            let a3565 = f8(Str::lit("u"));
            let a3566 = f527(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3565, a3566)
        } else {
            let a3567 = f8(Str::lit("U"));
            let a3568 = f527(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3567, a3568)
        }
    }
}

pub fn f529() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f530(v0: u64, v1: Adt14, v2: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m3569 = v0;
    if m3569 == 0 {
        Ok(f434(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3569 - 1;
        let m3570 = v1.clone();
        match m3570 {
            Adt14::C0(v4) => {
                if v4 {
                    Ok(f473())
                } else {
                    Ok(f434(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt14::C1(_, _) => {
                Ok(f434(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C2(_) => {
                Ok(f434(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C3(_) => {
                Ok(f434(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C4(_) => {
                let m3571 = f150(f142(v1.clone(), f8(Str::lit("$ref"))));
                match m3571 {
                    None => {
                        if if f443(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f443(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f443(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f435(v2.clone(), List::cons(Adt23::C0(v2, v1), List::<Adt23>::nil())))
                        } else {
                            let v17: Bytes = f145(v1.clone(), f8(Str::lit("type")));
                            if if f36(v17.clone(), Bytes::lit(&[])) {
                                !f514(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f473())
                            } else {
                                if f36(v17.clone(), f8(Str::lit("string"))) {
                                    Ok(f435(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                } else {
                                    if f36(v17.clone(), f8(Str::lit("integer"))) {
                                        Ok(f435(f515(v1), List::<Adt23>::nil()))
                                    } else {
                                        if f36(v17.clone(), f8(Str::lit("number"))) {
                                            Ok(f435(f516(), List::<Adt23>::nil()))
                                        } else {
                                            if f36(v17.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f435(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                            } else {
                                                if f36(v17.clone(), f8(Str::lit("array"))) {
                                                    let m3573 = f142(v1, f8(Str::lit("items")));
                                                    match m3573 {
                                                        None => {
                                                            Ok(f471(f8(Str::lit("List")), f473()))
                                                        }
                                                        Some(v18) => {
                                                            Ok(f471(f8(Str::lit("List")), f530(v3, v18, {
                                                                let a3574 = v2;
                                                                let a3575 = f8(Str::lit("Item"));
                                                                append_bytes(a3574, a3575)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f36(v17.clone(), f8(Str::lit("object"))) {
                                                        let m3576 = f517(v1);
                                                        match m3576 {
                                                            None => {
                                                                Ok(f471(f8(Str::lit("List")), f518(f473())))
                                                            }
                                                            Some(v19) => {
                                                                Ok(f471(f8(Str::lit("List")), f518(f530(v3, v19, {
                                                                    let a3577 = v2;
                                                                    let a3578 = f8(Str::lit("Value"));
                                                                    append_bytes(a3577, a3578)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f434(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v17, List::<Bytes>::nil()))), v2))
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
                        let m3572 = f432(v15)?;
                        match m3572 {
                            None => {
                                Ok(f434(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v16) => {
                                Ok(f435(v16, List::<Adt23>::nil()))
                            }
                        }
                    }
                }
            }
            Adt14::C5 => {
                Ok(f473())
            }
            Adt14::C6(_, _) => {
                Ok(f434(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C7(_, _, _) => {
                Ok(f434(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f531(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f344(Fn10::F535, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f532(v0: Fn25, p3583: u64, p3584: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3582 = (p3583, p3584);
    {
        let mut m3582 = m3582;
        loop {
            let step = {
                let (v1, v2) = m3582;
                let m3579 = v1;
                if m3579 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3579 - 1;
                    let m3581 = {
                        let c3580: Fn25 = v0.clone();
                        c3580.apply(v2.clone())
                    };
                    match m3581 {
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
                Ok(next) => { m3582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f533(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f536(v0)
}

pub fn f534(v0: u64) -> Bytes {
    f32({
        let a3585 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3585)
    }, v0, 1u64)
}

pub fn f535(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f536(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3586, _) = v0.clone();
        h3586
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3587, _) = {
            let (_, h3588) = v0.clone();
            h3588
        };
        h3587
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3589) = {
            let (_, h3590) = v0;
            h3590
        };
        h3589
    };
    let m3591 = v1.clone();
    match m3591.uncons() {
        None => {
            let m3592 = v2;
            match m3592.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3593 = v2.clone();
            match m3593.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f521({
                        let (h3594, _) = v8.clone();
                        h3594
                    }, {
                        let (h3595, _) = v6.clone();
                        h3595
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
