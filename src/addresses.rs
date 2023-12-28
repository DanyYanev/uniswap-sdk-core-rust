use super::chains::{ChainId, SUPPORTED_CHAINS};
use lazy_static::lazy_static;
use std::collections::HashMap;

type AddressMap = HashMap<u32, String>;
type ChainMap = HashMap<u32, ChainAddresses>;
type ChainAddress = HashMap<u32, String>;

#[derive(Clone)]
pub struct ChainAddresses {
    v3_core_factory_address: String,
    multicall_address: String,
    quoter_address: String,
    v3_migrator_address: Option<String>,
    nonfungible_position_manager_address: Option<String>,
    tick_lens_address: Option<String>,
    swap_router02_address: Option<String>,
    v1_mixed_route_quoter_address: Option<String>,
}

pub const DEFAULT_NETWORKS: [ChainId; 3] = [ChainId::MAINNET, ChainId::GOERLI, ChainId::SEPOLIA];

pub fn construct_same_address_map(address: &str, additional_networks: &[ChainId]) -> AddressMap {
    let mut networks = DEFAULT_NETWORKS.to_vec();
    networks.extend_from_slice(additional_networks);
    let mut map = AddressMap::new();
    for chain_id in networks {
        map.insert(chain_id as u32, String::from(address));
    }
    map
}

lazy_static! {
    #[derive(Copy, Clone)]
    pub static ref UNI_ADDRESSES: AddressMap = construct_same_address_map(
        "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
        &[
            ChainId::OPTIMISM,
            ChainId::ARBITRUMONE,
            ChainId::POLYGON,
            ChainId::POLYGONMUMBAI,
            ChainId::SEPOLIA,
        ]
    );
}

pub const UNISWAP_NFT_AIRDROP_CLAIM_ADDRESS: &str = "0x8B799381ac40b838BBA4131ffB26197C432AFe78";

pub const V2_FACTORY_ADDRESS: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";

lazy_static! {
    pub static ref V2_FACTORY_ADDRESSES: AddressMap = construct_same_address_map(
        V2_FACTORY_ADDRESS,
        &[
            ChainId::POLYGON,
            ChainId::OPTIMISM,
            ChainId::CELO,
            ChainId::ARBITRUMONE,
            ChainId::BNB,
            ChainId::AVALANCHE,
            ChainId::BASE,
        ]
    );
}

pub const V2_ROUTER_ADDRESS: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";

lazy_static! {
    pub static ref V2_ROUTER_ADDRESSES: AddressMap =
        construct_same_address_map(V2_ROUTER_ADDRESS, &[]);
}

impl Default for ChainAddresses {
    /// Networks that share most of the same addresses i.e. Mainnet, Goerli, Optimism, Arbitrum, Polygon
    fn default() -> Self {
        Self {
            v3_core_factory_address: "0x1F98431c8aD98523631AE4a59f267346ea31F984".into(),
            multicall_address: "0x1F98415757620B543A52E61c46B32eB19261F984".into(),
            quoter_address: "0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6".into(),
            v3_migrator_address: Some("0xA5644E29708357803b5A882D272c41cC0dF92B34".into()),
            nonfungible_position_manager_address: Some(
                "0xC36442b4a4522E871399CD717aBDD847Ab11FE88".into(),
            ),
            tick_lens_address: None,
            swap_router02_address: None,
            v1_mixed_route_quoter_address: None,
        }
    }
}

pub fn mainnet_address() -> ChainAddresses {
    ChainAddresses {
        v1_mixed_route_quoter_address: Some("0x84E44095eeBfEC7793Cd7d5b57B7e401D7f1cA2E".into()),
        ..Default::default()
    }
}

pub fn goerli_address() -> ChainAddresses {
    ChainAddresses {
        v1_mixed_route_quoter_address: Some("0xBa60b6e6fF25488308789E6e0A65D838be34194e".into()),
        ..Default::default()
    }
}

pub fn optimism_addresses() -> ChainAddresses {
    ChainAddresses::default()
}

pub fn arbitum_one_addresses() -> ChainAddresses {
    ChainAddresses {
        multicall_address: "0xadF885960B47eA2CD9B55E6DAc6B42b7Cb2806dB".into(),
        tick_lens_address: Some("0xbfd8137f7d1516D3ea5cA83523914859ec47F573".into()),
        ..Default::default()
    }
}

pub fn polygon_addresses() -> ChainAddresses {
    ChainAddresses::default()
}

/// celo v3 addresses
pub fn celo_addresses() -> ChainAddresses {
    ChainAddresses {
        v3_core_factory_address: "0xAfE208a311B21f13EF87E33A90049fC17A7acDEc".into(),
        multicall_address: "0x633987602DE5C4F337e3DbF265303A1080324204".into(),
        quoter_address: "0x82825d0554fA07f7FC52Ab63c961F330fdEFa8E8".into(),
        v3_migrator_address: Some("0x3cFd4d48EDfDCC53D3f173F596f621064614C582".into()),
        nonfungible_position_manager_address: Some(
            "0x3d79EdAaBC0EaB6F08ED885C05Fc0B014290D95A".into(),
        ),
        tick_lens_address: Some("0x5f115D9113F88e0a0Db1b5033D90D4a9690AcD3D".into()),
        swap_router02_address: None,
        v1_mixed_route_quoter_address: None,
    }
}

/// BNB v3 addresses
pub fn bnb_addresses() -> ChainAddresses {
    ChainAddresses {
        v3_core_factory_address: "0xdB1d10011AD0Ff90774D0C6Bb92e5C5c8b4461F7".into(),
        multicall_address: "0x963Df249eD09c358A4819E39d9Cd5736c3087184".into(),
        quoter_address: "0x78D78E420Da98ad378D7799bE8f4AF69033EB077".into(),
        v3_migrator_address: Some("0x32681814957e0C13117ddc0c2aba232b5c9e760f".into()),
        nonfungible_position_manager_address: Some(
            "0x7b8A01B39D58278b5DE7e48c8449c9f4F5170613".into(),
        ),
        tick_lens_address: Some("0xD9270014D396281579760619CCf4c3af0501A47C".into()),
        swap_router02_address: Some("0xB971eF87ede563556b2ED4b1C0b0019111Dd85d2".into()),
        v1_mixed_route_quoter_address: None,
    }
}

/// Optimism Goerli addresses
pub fn optimism_goerli_addresses() -> ChainAddresses {
    ChainAddresses {
        v3_core_factory_address: "0xB656dA17129e7EB733A557f4EBc57B76CFbB5d10".into(),
        multicall_address: "0x07F2D8a2a02251B62af965f22fC4744A5f96BCCd".into(),
        quoter_address: "0x9569CbA925c8ca2248772A9A4976A516743A246F".into(),
        v3_migrator_address: Some("0xf6c55fBe84B1C8c3283533c53F51bC32F5C7Aba8".into()),
        nonfungible_position_manager_address: Some(
            "0x39Ca85Af2F383190cBf7d7c41ED9202D27426EF6".into(),
        ),
        tick_lens_address: Some("0xe6140Bd164b63E8BfCfc40D5dF952f83e171758e".into()),
        swap_router02_address: None,
        v1_mixed_route_quoter_address: None,
    }
}

/// Arbitrum Goerli v3 addresses
pub fn arbitrum_goerli_addresses() -> ChainAddresses {
    ChainAddresses {
        v3_core_factory_address: "0x4893376342d5D7b3e31d4184c08b265e5aB2A3f6".into(),
        multicall_address: "0x8260CB40247290317a4c062F3542622367F206Ee".into(),
        quoter_address: "0x1dd92b83591781D0C6d98d07391eea4b9a6008FA".into(),
        v3_migrator_address: Some("0xA815919D2584Ac3F76ea9CB62E6Fd40a43BCe0C3".into()),
        nonfungible_position_manager_address: Some(
            "0x622e4726a167799826d1E1D150b076A7725f5D81".into(),
        ),
        tick_lens_address: Some("0xb52429333da969a0C79a60930a4Bf0020E5D1DE8".into()),
        swap_router02_address: None,
        v1_mixed_route_quoter_address: None,
    }
}

/// sepolia v3 addresses
pub fn sepolia_address() -> ChainAddresses {
    ChainAddresses {
        v3_core_factory_address: "0x0227628f3F023bb0B980b67D528571c95c6DaC1c".into(),
        multicall_address: "0xD7F33bCdb21b359c8ee6F0251d30E94832baAd07".into(),
        quoter_address: "0xEd1f6473345F45b75F8179591dd5bA1888cf2FB3".into(),
        v3_migrator_address: Some("0x729004182cF005CEC8Bd85df140094b6aCbe8b15".into()),
        nonfungible_position_manager_address: Some(
            "0x1238536071E1c677A632429e3655c799b22cDA52".into(),
        ),
        tick_lens_address: Some("0xd7f33bcdb21b359c8ee6f0251d30e94832baad07".into()),
        swap_router02_address: None,
        v1_mixed_route_quoter_address: None,
    }
}

/// Avalanche v3 addresses
pub fn avalanche_addresses() -> ChainAddresses {
    ChainAddresses {
        v3_core_factory_address: "0x740b1c1de25031C31FF4fC9A62f554A55cdC1baD".into(),
        multicall_address: "0x0139141Cd4Ee88dF3Cdb65881D411bAE271Ef0C2".into(),
        quoter_address: "0xbe0F5544EC67e9B3b2D979aaA43f18Fd87E6257F".into(),
        v3_migrator_address: Some("0x44f5f1f5E452ea8d29C890E8F6e893fC0f1f0f97".into()),
        nonfungible_position_manager_address: Some(
            "0x655C406EBFa14EE2006250925e54ec43AD184f8B".into(),
        ),
        tick_lens_address: Some("0xEB9fFC8bf81b4fFd11fb6A63a6B0f098c6e21950".into()),
        swap_router02_address: Some("0xbb00FF08d01D300023C629E8fFfFcb65A5a578cE".into()),
        v1_mixed_route_quoter_address: None,
    }
}

/// Base v3 addresses
pub fn base_addresses() -> ChainAddresses {
    ChainAddresses {
        v3_core_factory_address: "0x33128a8fC17869897dcE68Ed026d694621f6FDfD".into(),
        multicall_address: "0x091e99cb1C49331a94dD62755D168E941AbD0693".into(),
        quoter_address: "0x3d4e44Eb1374240CE5F1B871ab261CD16335B76a".into(),
        v3_migrator_address: Some("0x23cF10b1ee3AdfCA73B0eF17C07F7577e7ACd2d7".into()),
        nonfungible_position_manager_address: Some(
            "0x03a520b32C04BF3bEEf7BEb72E919cf822Ed34f1".into(),
        ),
        tick_lens_address: Some("0x0CdeE061c75D43c82520eD998C23ac2991c9ac6d".into()),
        swap_router02_address: Some("0x2626664c2603336E57B271c5C0b26F421741e481".into()),
        v1_mixed_route_quoter_address: None,
    }
}

/// Base Goerli v3 addresses
pub fn base_goerli_addresses() -> ChainAddresses {
    ChainAddresses {
        v3_core_factory_address: "0x9323c1d6D800ed51Bd7C6B216cfBec678B7d0BC2".into(),
        multicall_address: "0xB206027a9E0E13F05eBEFa5D2402Bab3eA716439".into(),
        quoter_address: "0xedf539058e28E5937dAef3f69cEd0b25fbE66Ae9".into(),
        v3_migrator_address: Some("0x3efe5d02a04b7351D671Db7008ec6eBA9AD9e3aE".into()),
        nonfungible_position_manager_address: Some(
            "0x3c61369ef0D1D2AFa70d8feC2F31C5D6Ce134F30".into(),
        ),
        tick_lens_address: Some("0x1acB873Ee909D0c98adB18e4474943249F931b92".into()),
        swap_router02_address: Some("0x8357227D4eDc78991Db6FDB9bD6ADE250536dE1d".into()),
        v1_mixed_route_quoter_address: None,
    }
}

lazy_static! {
    pub static ref CHAIN_TO_ADDRESSES_MAP: ChainMap = {
        let mut new_map = ChainMap::new();
        new_map.insert(ChainId::BNB as u32, bnb_addresses());

        new_map.insert(ChainId::AVALANCHE as u32, avalanche_addresses());
        new_map.insert(ChainId::MAINNET as u32, mainnet_address());
        new_map.insert(ChainId::SEPOLIA as u32, sepolia_address());
        new_map.insert(ChainId::GOERLI as u32, goerli_address());
        new_map.insert(ChainId::ARBITRUMONE as u32, arbitum_one_addresses());
        new_map.insert(ChainId::ARBITRUMGOERLI as u32, arbitrum_goerli_addresses());
        new_map.insert(ChainId::CELO as u32, celo_addresses());
        new_map.insert(ChainId::CELOALFAJORES as u32, celo_addresses());

        new_map.insert(ChainId::POLYGON as u32, polygon_addresses());
        new_map.insert(ChainId::POLYGONMUMBAI as u32, polygon_addresses());
        new_map.insert(ChainId::OPTIMISM as u32, optimism_addresses());
        new_map.insert(ChainId::OPTIMISMGOERLI as u32, optimism_goerli_addresses());

        new_map.insert(ChainId::BASEGOERLI as u32, base_goerli_addresses());
        new_map.insert(ChainId::BASE as u32, base_addresses());
        new_map
    };
}

/// V3 Contract Addresses
pub fn v3_factory_addresses() -> ChainAddress {
    let mut chain_add = ChainAddress::new();
    for memo in SUPPORTED_CHAINS {
        chain_add.insert(
            memo as u32,
            CHAIN_TO_ADDRESSES_MAP
                .get(&(memo as u32))
                .unwrap()
                .v3_core_factory_address
                .clone(),
        );
    }
    chain_add
}

/// V3 Contract Addresses
pub fn v3_migrator_addresses() -> ChainAddress {
    let mut chain_add = ChainAddress::new();
    for memo in SUPPORTED_CHAINS {
        chain_add.insert(
            memo as u32,
            CHAIN_TO_ADDRESSES_MAP
                .get(&(memo as u32))
                .unwrap()
                .v3_migrator_address
                .clone()
                .unwrap(),
        );
    }
    chain_add
}

/// V3 Contract Addresses
pub fn multicall_addresses() -> ChainAddress {
    let mut chain_add = ChainAddress::new();
    for memo in SUPPORTED_CHAINS {
        chain_add.insert(
            memo as u32,
            CHAIN_TO_ADDRESSES_MAP
                .get(&(memo as u32))
                .unwrap()
                .multicall_address
                .clone(),
        );
    }
    chain_add
}

pub fn governance_alpha_v0_addresses() -> AddressMap {
    construct_same_address_map("0x5e4be8Bc9637f0EAA1A755019e06A68ce081D58F", &[])
}

pub fn governance_alpha_v1_addresses() -> AddressMap {
    let mut new_map = AddressMap::new();
    new_map.insert(
        ChainId::MAINNET as u32,
        "0xC4e172459f1E7939D522503B81AFAaC1014CE6F6".into(),
    );
    new_map
}

pub fn governance_bravo_addresses() -> AddressMap {
    let mut new_map = AddressMap::new();
    new_map.insert(
        ChainId::MAINNET as u32,
        "0x408ED6354d4973f66138C91495F2f2FCbd8724C3".into(),
    );
    new_map
}

pub fn timelock_addresses() -> AddressMap {
    construct_same_address_map("0x1a9C8182C09F50C8318d769245beA5", &[])
}

pub fn merkle_distributor_address() -> AddressMap {
    let mut new_map = AddressMap::new();
    new_map.insert(
        ChainId::MAINNET as u32,
        "0x090D4613473dEE047c3f2706764f49E0821D256e".into(),
    );
    new_map
}

pub fn argent_wallet_detector_address() -> AddressMap {
    let mut new_map = AddressMap::new();
    new_map.insert(
        ChainId::MAINNET as u32,
        "0xeca4B0bDBf7c55E9b7925919d03CbF8Dc82537E8".into(),
    );
    new_map
}

pub fn quoter_address() -> ChainAddress {
    let mut chain_add = ChainAddress::new();
    for memo in SUPPORTED_CHAINS {
        chain_add.insert(
            memo as u32,
            CHAIN_TO_ADDRESSES_MAP
                .get(&(memo as u32))
                .unwrap()
                .quoter_address
                .clone(),
        );
    }
    chain_add
}

pub fn nonfungible_position_manager_address() -> ChainAddress {
    let mut chain_add = ChainAddress::new();
    for memo in SUPPORTED_CHAINS {
        if CHAIN_TO_ADDRESSES_MAP
            .get(&(memo as u32))
            .unwrap()
            .nonfungible_position_manager_address
            .is_some()
        {
            chain_add.insert(
                memo as u32,
                CHAIN_TO_ADDRESSES_MAP
                    .get(&(memo as u32))
                    .unwrap()
                    .nonfungible_position_manager_address
                    .clone()
                    .unwrap(),
            );
        }
    }
    chain_add
}

pub fn ens_resgister_address_map() -> AddressMap {
    construct_same_address_map("0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e", &[])
}

pub fn socks_controller_addresses() -> AddressMap {
    let mut new_map = AddressMap::new();
    new_map.insert(
        ChainId::MAINNET as u32,
        "0x65770b5283117639760beA3F867b69b3697a91dd".into(),
    );
    new_map
}

pub fn tick_lens_addresses() -> ChainAddress {
    let mut chain_add = ChainAddress::new();
    for memo in SUPPORTED_CHAINS {
        if CHAIN_TO_ADDRESSES_MAP
            .get(&(memo as u32))
            .unwrap()
            .tick_lens_address
            .is_some()
        {
            chain_add.insert(
                memo as u32,
                CHAIN_TO_ADDRESSES_MAP
                    .get(&(memo as u32))
                    .unwrap()
                    .tick_lens_address
                    .clone()
                    .unwrap(),
            );
        }
    }
    chain_add
}

pub fn v1_mixed_route_quoter_address() -> ChainAddress {
    let mut chain_add = ChainAddress::new();
    for memo in SUPPORTED_CHAINS {
        if CHAIN_TO_ADDRESSES_MAP
            .get(&(memo as u32))
            .unwrap()
            .v1_mixed_route_quoter_address
            .is_some()
        {
            chain_add.insert(
                memo as u32,
                CHAIN_TO_ADDRESSES_MAP
                    .get(&(memo as u32))
                    .unwrap()
                    .v1_mixed_route_quoter_address
                    .clone()
                    .unwrap(),
            );
        }
    }
    chain_add
}

pub fn swap_router02_address(chain_id: u32) -> String {
    if chain_id == ChainId::BNB as u32 {
        CHAIN_TO_ADDRESSES_MAP
            .get(&chain_id)
            .unwrap()
            .swap_router02_address
            .clone()
            .unwrap()
    } else {
        "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45".into()
    }
}
