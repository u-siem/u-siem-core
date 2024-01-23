use super::{SiemDataset, SiemDatasetType, text_map::TextMapSynDataset, geo_ip::GeoIpSynDataset, i18n::I18nSynDataset, ip_map::IpMapSynDataset, ip_map_list::IpMapListSynDataset, ip_set::IpSetSynDataset, text_set::TextSetSynDataset, text_map_list::TextMapListSynDataset, ip_net::IpNetSynDataset};
use std::collections::BTreeMap;

/// The dataset holder allows access to the latest version of a dataset almost instantly without the need to check if there is an update of a dataset using a channel as was done previously.
#[derive(Clone, Default)]
pub struct DatasetHolder {
    datasets: BTreeMap<SiemDatasetType, SiemDataset>,
}

impl DatasetHolder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_datasets(list: Vec<SiemDataset>) -> Self {
        let mut datasets = BTreeMap::new();
        for dataset in list {
            datasets.insert(dataset.dataset_type(), dataset);
        }

        Self { datasets }
    }

    pub fn insert(&mut self, dataset: SiemDataset) {
        self.datasets.insert(dataset.dataset_type(), dataset);
    }

    pub fn get(&self, key: &SiemDatasetType) -> Option<&SiemDataset> {
        self.datasets.get(key)
    }
    pub fn subset(&self, list: Vec<SiemDatasetType>) -> Self {
        let mut datasets = BTreeMap::new();
        for typ in list {
            let dataset = match self.datasets.get(&typ) {
                Some(d) => d,
                None => continue,
            };
            datasets.insert(typ, dataset.clone());
        }

        Self { datasets }
    }

    pub fn geoip(&self) -> Option<&GeoIpSynDataset> {
        self.datasets.get(&SiemDatasetType::GeoIp)?.try_into().ok()
    }
    pub fn i18n(&self) -> Option<&I18nSynDataset> {
        self.datasets.get(&SiemDatasetType::I18n)?.try_into().ok()
    }
    pub fn ip_mac(&self) -> Option<&IpMapSynDataset> {
        self.datasets.get(&SiemDatasetType::IpMac)?.try_into().ok()
    }
    pub fn ip_dns(&self) -> Option<&IpMapListSynDataset> {
        self.datasets.get(&SiemDatasetType::IpDNS)?.try_into().ok()
    }
    pub fn mac_host(&self) -> Option<&TextMapSynDataset> {
        self.datasets.get(&SiemDatasetType::MacHost)?.try_into().ok()
    }
    pub fn host_user(&self) -> Option<&TextMapSynDataset> {
        self.datasets.get(&SiemDatasetType::HostUser)?.try_into().ok()
    }
    pub fn block_ip(&self) -> Option<&IpSetSynDataset> {
        self.datasets.get(&SiemDatasetType::BlockIp)?.try_into().ok()
    }
    pub fn block_domain(&self) -> Option<&TextSetSynDataset> {
        self.datasets.get(&SiemDatasetType::BlockDomain)?.try_into().ok()
    }
    pub fn block_email(&self) -> Option<&TextSetSynDataset> {
        self.datasets.get(&SiemDatasetType::BlockEmailSender)?.try_into().ok()
    }
    pub fn block_country(&self) -> Option<&TextSetSynDataset> {
        self.datasets.get(&SiemDatasetType::BlockCountry)?.try_into().ok()
    }
    pub fn host_vulnerable(&self) -> Option<&TextMapListSynDataset> {
        self.datasets.get(&SiemDatasetType::HostVulnerable)?.try_into().ok()
    }
    pub fn user_tag(&self) -> Option<&TextMapListSynDataset> {
        self.datasets.get(&SiemDatasetType::UserTag)?.try_into().ok()
    }
    pub fn asset_tag(&self) -> Option<&TextMapListSynDataset> {
        self.datasets.get(&SiemDatasetType::AssetTag)?.try_into().ok()
    }
    pub fn cloud_service(&self) -> Option<&IpNetSynDataset> {
        self.datasets.get(&SiemDatasetType::IpCloudService)?.try_into().ok()
    }
    pub fn cloud_provider(&self) -> Option<&IpNetSynDataset> {
        self.datasets.get(&SiemDatasetType::IpCloudProvider)?.try_into().ok()
    }
    pub fn user_headquarters(&self) -> Option<&TextMapSynDataset> {
        self.datasets.get(&SiemDatasetType::UserHeadquarters)?.try_into().ok()
    }
    pub fn ip_headquarters(&self) -> Option<&IpNetSynDataset> {
        self.datasets.get(&SiemDatasetType::IpHeadquarters)?.try_into().ok()
    }
    pub fn configuration(&self) -> Option<&TextMapSynDataset> {
        self.datasets.get(&SiemDatasetType::Configuration)?.try_into().ok()
    }
}
