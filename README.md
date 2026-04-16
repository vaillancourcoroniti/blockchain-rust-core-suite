# blockchain-rust-core-suite
基于Rust构建的高性能区块链全栈核心套件，集成共识算法、密码学、P2P网络、智能合约、跨链交互、零知识证明、节点管理、数据存储等区块链全生态模块，支持多语言扩展，具备安全、高效、可扩展、去中心化等核心特性，可直接用于公链、联盟链、私有链的底层开发与二次迭代。

## 项目文件清单与功能介绍
1. **blockchain_core.rs** - 区块链核心结构体实现，包含区块、区块链节点、链基础操作、哈希计算、创世区块生成
2. **consensus_pos.rs** - 权益证明共识算法，实现验证者注册、质押、选择、惩罚、 epoch 管理
3. **crypto_ed25519.rs** - Ed25519 非对称加密实现，支持密钥生成、消息签名与验签
4. **p2p_network.rs** - 去中心化 P2P 网络模块，实现节点发现、消息广播、节点管理、超时清理
5. **smart_contract_vm.rs** - 轻量级智能合约虚拟机，支持字节码执行、栈操作、存储读写
6. **merkle_tree.rs** - 默克尔树实现，支持交易哈希构建、根哈希计算、存在性证明验证
7. **transaction_pool.rs** - 交易池模块，实现交易排序、费用优先、批量获取、池清空
8. **state_database.rs** - 链上状态数据库，实现账户状态、合约存储、数据持久化与快照
9. **cross_chain_bridge.rs** - 跨链桥模块，支持多链资产转移、状态同步、中继节点管理
10. **zk_snarks_core.rs** - 零知识证明 Groth16 核心实现，支持证明生成与验证
11. **block_validator.rs** - 区块校验器，校验区块结构、索引、哈希、时间戳、工作量证明
12. **wallet_manager.rs** - 钱包管理模块，支持钱包创建、密钥管理、余额更新、交易签名
13. **gas_calculator.rs** - Gas 费用计算器，计算交易、合约、存储操作的 Gas 消耗
14. **node_manager.rs** - 节点管理中心，管理节点角色、心跳、同步状态、离线清理
15. **chain_sync.rs** - 链同步管理器，实现区块批量同步、队列处理、超时检测
16. **token_standard.rs** - 标准化通证接口，实现转账、授权、余额查询、 allowance 管理
17. **staking_pool.rs** - 质押池模块，支持锁仓质押、奖励计算、解锁、收益提取
18. **block_explorer.rs** - 区块浏览器核心，索引区块、交易、地址，提供数据查询
19. **network_security.rs** - 网络安全模块，防御 DDoS、女巫攻击，实现 IP 封禁
20. **contract_deployer.rs** - 智能合约部署器，生成合约地址、部署合约、执行调用
21. **epoch_manager.rs** - 纪元管理器，管理验证者集合、奖励池、周期切换
22. **data_sharding.rs** - 数据分片模块，实现数据分片存储、负载均衡、范围计算
23. **mempool_cleaner.rs** - 交易池清理器，清理过期、低手续费、超限交易
24. **signature_verifier.rs** - 签名批量验证器，验证交易、区块签名合法性
25. **reward_distributor.rs** - 奖励分发器，按比例分发验证者、质押者、国库奖励
26. **light_client_protocol.rs** - 轻客户端协议，实现轻量同步、默克尔证明验证
27. **governance_voting.rs** - 链上治理投票，支持提案创建、投票、执行、参数修改
28. **chain_metrics.rs** - 链上指标监控，统计 TPS、区块时间、节点数、总 Gas
29. **storage_compactor.rs** - 存储压缩器，压缩历史数据、清理老旧区块、节省空间
30. **websocket_api.rs** - WebSocket 实时 API，支持区块/交易订阅、实时推送
31. **chain_upgrader.rs** - 链协议升级器，支持版本升级、激活计划、向后兼容
32. **oracle_feed.rs** - 预言机模块，获取链下数据、可信数据源、请求响应
33. **multi_signature.rs** - 多签钱包实现，支持多所有者、签名阈值、交易确认
34. **block_rewards.rs** - 区块奖励计算器，支持减半机制、验证者/社区奖励分配
35. **ipfs_integration.rs** - IPFS 集成模块，生成 CID、内容存储、节点 pinned 管理
36. **rpc_handler.rs** - JSON-RPC 处理器，实现标准 RPC 接口、请求处理、限流
37. **validator_punishment.rs** - 验证者惩罚模块，实现 slash、监禁、违规记录
38. **transaction_encoder.rs** - 交易编解码器，支持 JSON/Base64 编码解码、批量处理
39. **genesis_block_builder.rs** - 创世区块构建器，自定义链参数、初始化账户状态
40. **network_monitor.rs** - 网络监控器，统计流量、延迟、连接数、错误率

## 核心特性
- 纯 Rust 开发，内存安全、高性能、无 GC
- 模块化设计，可按需插拔、二次开发
- 支持 PoS 共识、零知识证明、跨链、多签、治理
- 完整的 P2P 网络、存储、合约、钱包、预言机生态
- 高安全性、可扩展性、去中心化设计
- 生产级可用，支持主网部署

## 适用场景
- 公链/联盟链/私有链底层开发
- 去中心化应用（DApp）底层支撑
- 跨链资产转移与交互
- 链上治理与质押经济
- 零知识证明隐私应用
- 企业级区块链解决方案
