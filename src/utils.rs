use std::collections::HashMap;
use std::str::FromStr;
use getrandom::fill;

pub fn random_element<T>(vec: &[T]) -> Option<&T> {
    if vec.is_empty() { return None; }

    let mut buf = [0u8; 8]; // buffer per 64 bit casuali
    fill(&mut buf).expect("Failed to get randomness");

    let num = u64::from_le_bytes(buf);
    let index = (num % vec.len() as u64) as usize;

    vec.get(index)
}

pub fn random_range_f32(min: f32, max: f32) -> f32 {
    let mut byte = [0u8; 1];
    fill(&mut byte).expect("Failed to get randomness");

    let norm = byte[0] as f32 / 255.0;
    min + norm * (max - min)
}

pub fn random_range_u32(min: u32, max: u32) -> u32 {
    let mut buf = [0u8; 4];
    fill(&mut buf).expect("random error");
    let raw = u32::from_le_bytes(buf);

    let range = max - min + 1;
    min + (raw % range)
}

pub enum ParamValue<T> {
    Single(T),
    Range(T, T),
    List(Vec<T>),
}

pub fn get_param<T: FromStr + Clone>(query: &HashMap<String, String>, key: &str, default: ParamValue<T>) -> ParamValue<T> {
    match query.get(key) {
        Some(v) => {
            let parts: Vec<_> = v.split(',').map(|s| s.trim().parse::<T>()).collect();
            let ok_parts: Vec<T> = parts.into_iter().filter_map(Result::ok).collect();

            match ok_parts.as_slice() {
                [val] => ParamValue::Single(val.clone()),
                [start, end] => ParamValue::Range(start.clone(), end.clone()),
                _ if ok_parts.len() >= 1 => ParamValue::List(ok_parts),
                _ => default,
            }
        }
        None => default,
    }
}
