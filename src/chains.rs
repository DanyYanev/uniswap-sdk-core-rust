#[derive(Debug, Clone, Copy)]
pub enum ChainId {
    MAINNET = 1,
    GOERLI = 5,
    SEPOLIA = 11155111,
    OPTIMISM = 10,
    OPTIMISMGOERLI = 420,
    OPTIMISMSEPOLIA = 11155420,
    ARBITRUMONE = 42161,
    ARBITRUMGOERLI = 421613,
    ARBITRUMSEPOLIA = 421614,
    POLYGON = 137,
    POLYGONMUMBAI = 80001,
    CELO = 42220,
    CELOALFAJORES = 44787,
    GNOSIS = 100,
    MOONBEAM = 1284,
    BNB = 56,
    AVALANCHE = 43114,
    BASEGOERLI = 84531,
    BASE = 8453,
    ZORA = 7777777,
    ZORASEPOLIA = 999999999,
    ROOTSTOCK = 30
}

pub const SUPPORTED_CHAINS: [ChainId; 20] = [
    ChainId::MAINNET,
    ChainId::OPTIMISM,
    ChainId::OPTIMISMGOERLI,
    ChainId::OPTIMISMSEPOLIA,
    ChainId::ARBITRUMONE,
    ChainId::ARBITRUMGOERLI,
    ChainId::ARBITRUMSEPOLIA,
    ChainId::POLYGON,
    ChainId::POLYGONMUMBAI,
    ChainId::GOERLI,
    ChainId::SEPOLIA,
    ChainId::CELOALFAJORES,
    ChainId::CELO,
    ChainId::BNB,
    ChainId::AVALANCHE,
    ChainId::BASE,
    ChainId::BASEGOERLI,
    ChainId::ZORA,
    ChainId::ZORASEPOLIA,
    ChainId::ROOTSTOCK
];

pub enum NativeCurrencyName {
    ETHER,
    MATIC,
    CELO,
    GNOSIS,
    MOONBEAM,
    BNB,
    AVAX,
    ROOTSTOCK,
}
