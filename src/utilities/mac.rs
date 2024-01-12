use std::fmt::Write;

pub fn mac_str_to_u48(mac: &str) -> Option<u64> {
    let mut ret = 0;
    let mut splited = mac.split(|v| v == '-' || v == ':');
    for i in (0..48).step_by(8).rev() {
        ret += u64::from_str_radix(splited.next()?, 16).ok()? << i;
    }
    Some(ret)
}

pub fn mac_str_to_u64(mac: &str) -> Option<u64> {
    let mut ret = 0;
    let mut splited = mac.split(|v| v == '-' || v == ':');
    for i in (0..64).step_by(8).rev() {
        ret += u64::from_str_radix(splited.next()?, 16).ok()? << i;
    }
    Some(ret)
}

pub fn mac_str_to_u128(mac: &str) -> Option<u128> {
    let mut ret: u128 = 0;
    let mut splited = mac.split(|v| v == '-' || v == ':');
    for i in (0..120).rev().step_by(8) {
        ret += u128::from_str_radix(splited.next()?, 16).ok()? << i;
    }
    Some(ret)
}

pub fn mac_u64_to_str(mac: u64) -> String {
    let mut ret = String::with_capacity(32);
    for i in (0..64).step_by(8).rev() {
        let _ = ret.write_fmt(format_args!("{:02x}:", (mac >> i) & 0xff));
    }
    ret.pop();
    ret
}
pub fn mac_u128_to_str(mac: u128) -> String {
    let mut ret = String::with_capacity(32);
    let count = mac.leading_zeros();
    let count = if count < 32 {
        128
    } else if count < 80 {
        64
    } else {
        48
    };
    for i in (0..count).step_by(8).rev() {
        let _ = ret.write_fmt(format_args!("{:02x}:", (mac >> i) & 0xff));
    }
    ret.pop();
    ret
}

pub fn mac_u48_to_str(mac: u64) -> String {
    let mut ret = String::with_capacity(32);
    for i in (0..48).step_by(8).rev() {
        let _ = ret.write_fmt(format_args!("{:02x}:", (mac >> i) & 0xff));
    }
    ret.pop();
    ret
}

pub fn mac_str_to_u128_any(mac: &str) -> Option<u128> {
    let mut ret: u128 = 0;
    let splited = mac.split(|v| v == '-' || v == ':');
    let count = splited.count();
    if count != 6 && count != 8 && count != 16 {
        return None;
    }
    let mut splited = mac.split(|v| v == '-' || v == ':');

    for i in (0..count * 8).step_by(8).rev() {
        let nxt = splited.next()?;
        ret += u128::from_str_radix(nxt, 16).ok()? << i;
    }
    Some(ret)
}

#[test]
fn should_convert_mac() {
    let mac = mac_str_to_u48("fb:96:65:6d:17:96").unwrap();
    assert_eq!(276623365314454, mac);
    let mac = mac_str_to_u48("fb-96-65-6d-17-96").unwrap();
    assert_eq!(276623365314454, mac);
    let mac = mac_u48_to_str(mac);
    assert_eq!("fb:96:65:6d:17:96", mac);

    let mac = mac_str_to_u64("fb:96:65:00:00:6d:17:96").unwrap();
    assert_eq!(18128788400708065174, mac);
    let mac = mac_str_to_u64("fb-96-65-00-00-6d-17-96").unwrap();
    assert_eq!(18128788400708065174, mac);
    let mac = mac_u64_to_str(mac);
    assert_eq!("fb:96:65:00:00:6d:17:96", mac);

    let mac = mac_str_to_u128_any("fb:96:65:6d:17:96").unwrap();
    assert_eq!(276623365314454, mac);
    let mac = mac_u128_to_str(mac);
    assert_eq!("fb:96:65:6d:17:96", mac);
    let mac = mac_str_to_u128_any("00:96:65:6d:17:96").unwrap();
    assert_eq!(645946742678, mac);
    let mac = mac_u128_to_str(mac);
    assert_eq!("00:96:65:6d:17:96", mac);
    let mac = mac_str_to_u128_any("fb:96:65:00:00:6d:17:96").unwrap();
    assert_eq!(18128788400708065174, mac);
    let mac = mac_u128_to_str(mac);
    assert_eq!("fb:96:65:00:00:6d:17:96", mac);
    let mac = mac_str_to_u128_any("00:01:65:00:00:6d:17:96").unwrap();
    assert_eq!(392525658265494, mac);
    let mac = mac_u128_to_str(mac);
    assert_eq!("00:01:65:00:00:6d:17:96", mac);
}
