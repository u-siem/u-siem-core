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
        IVec::from(serde_json::to_string(&self).unwrap().as_bytes())
    }
}
#[cfg(feature="slow_geoip")]
impl From<IVec> for GeoIpInfo {
    fn from(value: IVec) -> Self {
        let s: GeoIpInfo = serde_json::from_slice(&value).unwrap();
        s
    }
}
#[cfg(not(feature="slow_geoip"))]
pub use fast::{GeoIpDataset, GeoIpSynDataset, UpdateGeoIp};

#[cfg(feature="slow_geoip")]
pub use slow::{SlowGeoIpDataset as GeoIpDataset, SlowGeoIpSynDataset as GeoIpSynDataset, UpdateSlowGeoIp as UpdateGeoIp};