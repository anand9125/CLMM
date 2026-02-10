# CLMM - Concentrated Liquidity Market Maker

A sophisticated decentralized exchange (DEX) protocol implementing concentrated liquidity market making, enabling liquidity providers to allocate capital within custom price ranges for improved capital efficiency.

## 🌟 Overview

CLMM (Concentrated Liquidity Market Maker) is an advanced automated market maker (AMM) that allows liquidity providers (LPs) to concentrate their liquidity within specific price ranges rather than across the entire price curve. This approach significantly improves capital efficiency compared to traditional constant product AMMs.

### Key Features

- **Concentrated Liquidity**: LPs can provide liquidity within custom price ranges
- **Multiple Fee Tiers**: Support for different fee structures (0.01%, 0.05%, 0.3%, 1%)
- **Non-Fungible Positions**: Each liquidity position is represented as an NFT
- **Flexible Position Management**: Add, remove, or adjust liquidity positions dynamically
- **Capital Efficiency**: Up to 4000x more efficient than traditional AMMs
- **Price Oracle**: Built-in time-weighted average price (TWAP) oracle
- **Flash Swaps**: Support for atomic swap and callback operations

## 📋 Table of Contents

- [Architecture](#architecture)
- [Getting Started](#getting-started)
- [Installation](#installation)
- [Usage](#usage)
- [Smart Contracts](#smart-contracts)
- [Core Concepts](#core-concepts)
- [Examples](#examples)
- [Testing](#testing)
- [Security](#security)
- [Contributing](#contributing)
- [License](#license)

## 🏗️ Architecture

The CLMM protocol consists of several core components:

```
┌─────────────────────────────────────────────────┐
│              CLMM Factory                        │
│  - Creates new liquidity pools                   │
│  - Manages fee tiers                             │
└─────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────┐
│              Liquidity Pool                      │
│  - Token swap logic                              │
│  - Liquidity management                          │
│  - Fee collection                                │
│  - Price oracle                                  │
└─────────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────┐
│         Position Manager (NFT)                   │
│  - Mint/burn liquidity positions                │
│  - Position tracking                             │
│  - Fee claiming                                  │
└─────────────────────────────────────────────────┘
```

## 🚀 Getting Started

### Prerequisites

- Node.js >= 16.x
- npm or yarn
- Hardhat or Foundry
- MetaMask or similar Web3 wallet

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/clmm.git
cd clmm

# Install dependencies
npm install

# Compile contracts
npm run compile
```

### Environment Setup

Create a `.env` file in the root directory:

```env
PRIVATE_KEY=your_private_key_here
INFURA_API_KEY=your_infura_key_here
ETHERSCAN_API_KEY=your_etherscan_key_here
```

## 💻 Usage

### Deploying Contracts

```bash
# Deploy to local network
npm run deploy:local

# Deploy to testnet
npm run deploy:testnet

# Deploy to mainnet
npm run deploy:mainnet
```

### Creating a Pool

```javascript
const factory = await ethers.getContractAt("CLMMFactory", FACTORY_ADDRESS);

// Create a new pool with 0.3% fee tier
const tx = await factory.createPool(
  TOKEN_A_ADDRESS,
  TOKEN_B_ADDRESS,
  3000, // 0.3% fee = 3000 basis points
  initialSqrtPrice
);

await tx.wait();
```

### Adding Liquidity

```javascript
const positionManager = await ethers.getContractAt("PositionManager", POSITION_MANAGER_ADDRESS);

const params = {
  token0: TOKEN_A_ADDRESS,
  token1: TOKEN_B_ADDRESS,
  fee: 3000,
  tickLower: -887200, // Lower price boundary
  tickUpper: 887200,  // Upper price boundary
  amount0Desired: ethers.utils.parseEther("10"),
  amount1Desired: ethers.utils.parseEther("10"),
  amount0Min: 0,
  amount1Min: 0,
  recipient: YOUR_ADDRESS,
  deadline: Math.floor(Date.now() / 1000) + 3600
};

const tx = await positionManager.mint(params);
await tx.wait();
```

### Swapping Tokens

```javascript
const pool = await ethers.getContractAt("CLMMPool", POOL_ADDRESS);

const swapParams = {
  recipient: YOUR_ADDRESS,
  zeroForOne: true, // Swapping token0 for token1
  amountSpecified: ethers.utils.parseEther("1"),
  sqrtPriceLimitX96: 0, // No price limit
  data: "0x" // Additional callback data
};

const tx = await pool.swap(swapParams);
await tx.wait();
```

## 📜 Smart Contracts

### Core Contracts

- **CLMMFactory.sol**: Factory contract for creating new pools
- **CLMMPool.sol**: Main pool contract implementing swap and liquidity logic
- **PositionManager.sol**: NFT-based position management
- **TickMath.sol**: Mathematical operations for tick calculations
- **SqrtPriceMath.sol**: Square root price calculations
- **SwapMath.sol**: Swap computation logic
- **LiquidityMath.sol**: Liquidity amount calculations

### Libraries

- **FullMath.sol**: Full precision mathematical operations
- **FixedPoint96.sol**: Fixed point number representations
- **SafeCast.sol**: Safe type casting utilities
- **TickBitmap.sol**: Efficient tick bitmap management

## 🎓 Core Concepts

### Concentrated Liquidity

Unlike traditional AMMs where liquidity is distributed across the entire 0 to ∞ price range, CLMM allows LPs to concentrate their capital within specific price ranges. This means:

- Higher capital efficiency
- More fees earned per unit of liquidity
- Better prices for traders in active ranges

### Ticks and Ranges

Prices are represented using "ticks" - discrete price points on a logarithmic scale:

```
Price = 1.0001^tick
```

LPs specify their position using tick ranges:
- **tickLower**: Lower bound of the range
- **tickUpper**: Upper bound of the range

### Fee Tiers

Multiple fee tiers accommodate different token pairs:

- **0.01%**: Stable pairs (USDC/USDT)
- **0.05%**: Low volatility pairs
- **0.3%**: Standard pairs (most common)
- **1%**: Exotic or high volatility pairs

### Position NFTs

Each liquidity position is represented as an ERC-721 NFT, which:

- Makes positions transferable
- Allows for position composition
- Enables use in DeFi protocols (lending, derivatives)

## 📚 Examples

### Example 1: ETH/USDC Pool

```javascript
// Create pool at current market price (~$2000/ETH)
const sqrtPriceX96 = encodePriceSqrt(2000, 1);
await factory.createPool(WETH, USDC, 3000, sqrtPriceX96);

// Add liquidity around current price ($1900 - $2100)
const params = {
  token0: USDC,
  token1: WETH,
  fee: 3000,
  tickLower: getTickAtPrice(1900),
  tickUpper: getTickAtPrice(2100),
  amount0Desired: parseUnits("10000", 6), // 10k USDC
  amount1Desired: parseEther("5"), // 5 ETH
  // ... other params
};
```

### Example 2: Stablecoin Pool

```javascript
// Tight range for stablecoin pair
const params = {
  token0: USDC,
  token1: USDT,
  fee: 100, // 0.01% fee
  tickLower: getTickAtPrice(0.998),
  tickUpper: getTickAtPrice(1.002),
  amount0Desired: parseUnits("100000", 6),
  amount1Desired: parseUnits("100000", 6),
  // ... other params
};
```

## 🧪 Testing

```bash
# Run all tests
npm test

# Run specific test suite
npm test test/CLMMPool.test.js

# Run with coverage
npm run coverage

# Run gas report
npm run gas-report
```

### Test Coverage

The project maintains >95% test coverage across:
- Unit tests for individual contracts
- Integration tests for contract interactions
- Fuzz testing for edge cases
- Gas optimization tests

## 🔒 Security

### Audits

- ✅ [Audit Firm Name] - Date
- ✅ [Audit Firm Name] - Date

### Bug Bounty

We operate an active bug bounty program. Please report security issues to security@yourproject.com

### Best Practices

- All contracts are upgradeable using proxy patterns
- Reentrancy guards on all external functions
- Comprehensive input validation
- Safe math operations throughout

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Solidity style guide
- Use meaningful variable names
- Add comprehensive comments
- Include NatSpec documentation

## 📊 Gas Optimization

Our implementation focuses on gas efficiency:

- Optimized storage layout
- Batch operations where possible
- Efficient bitmap operations
- Minimal external calls

Typical gas costs:
- Pool creation: ~4M gas
- Add liquidity: ~150k gas
- Remove liquidity: ~120k gas
- Swap: ~100-150k gas

## 🗺️ Roadmap

- [x] Core CLMM implementation
- [x] Position NFT manager
- [x] Multi-fee tier support
- [ ] Layer 2 deployment
- [ ] Governance token
- [ ] Liquidity mining
- [ ] Advanced oracle features
- [ ] Cross-chain support

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Contact & Support

- Website: https://yourproject.com
- Documentation: https://docs.yourproject.com
- Discord: https://discord.gg/yourproject
- Twitter: @yourproject
- Email: support@yourproject.com

## 🙏 Acknowledgments

- Uniswap V3 for concentrated liquidity innovation
- OpenZeppelin for secure contract libraries
- The Ethereum community

---

**Disclaimer**: This software is provided "as is", without warranty of any kind. Use at your own risk.