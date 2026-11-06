#![cfg(test)]

pub(crate) const GET_DEVICE_INFO_COMMAND: &str = include_str!("get_device_info.json");
pub(crate) const GET_MINER_STATUS_SUMMARY_COMMAND: &str =
    include_str!("get_miner_status_summary.json");
pub(crate) const GET_MINER_STATUS_POOLS_COMMAND: &str = include_str!("get_miner_status_pools.json");
pub(crate) const GET_MINER_STATUS_EDEVS_COMMAND: &str = include_str!("get_miner_status_edevs.json");
pub(crate) const GET_DEVICE_INFO_WITH_ERRORS_COMMAND: &str =
    include_str!("get_device_info_with_errors.json");
pub(crate) const GET_DEVICE_INFO_HYDRO_COMMAND: &str = include_str!("get_device_info_hydro.json");
pub(crate) const GET_MINER_STATUS_SUMMARY_HYDRO_COMMAND: &str =
    include_str!("get_miner_status_summary_hydro.json");
pub(crate) const GET_MINER_STATUS_EDEVS_HYDRO_COMMAND: &str =
    include_str!("get_miner_status_edevs_hydro.json");
