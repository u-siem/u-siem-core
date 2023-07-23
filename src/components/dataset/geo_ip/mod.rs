#[cfg(not(feature="slow_geoip"))]
mod fast;
#[cfg(feature="slow_geoip")]
mod slow;

use serde::{Serialize, Deserialize};
#[cfg(feature="slow_geoip")]
use sled::IVec;

use crate::prelude::types::LogString;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GeoIpInfo {
    pub country: LogString,
    pub country_iso: LogString,
    pub city: LogString,
    pub latitude: f32,
    pub longitude: f32,
    pub isp: LogString, // More important than country in my opinion because Geolocalization is very imprecise.
    pub asn: u32,
}
#[cfg(feature="slow_geoip")]
impl Into<IVec> for GeoIpInfo {
    fn into(self) -> IVec {
        IVec::from(any_as_u8_slice(&self))
    }
}
#[cfg(feature="slow_geoip")]
impl From<IVec> for GeoIpInfo {
    fn from(value: IVec) -> Self {
        let s: GeoIpInfo = unsafe { std::ptr::read(value.as_ptr() as *const _) };
        s
    }
}
#[cfg(feature="slow_geoip")]
fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::core::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::core::mem::size_of::<T>(),
        )
    }
}
#[cfg(not(feature="slow_geoip"))]
pub use fast::{GeoIpDataset, GeoIpSynDataset, UpdateGeoIp};

#[cfg(feature="slow_geoip")]
pub use slow::{SlowGeoIpDataset as GeoIpDataset, SlowGeoIpSynDataset as GeoIpSynDataset, UpdateSlowGeoIp as UpdateGeoIp};