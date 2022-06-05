initSidebarItems({"enum":[["EthereumTransaction",""]],"fn":[["err",""],["internal_err",""],["internal_err_with_data",""],["public_key",""]],"mod":[["frontier_backend_client",""]],"struct":[["Eth","Eth API implementation."],["EthBlockDataCacheTask","Manage LRU caches for block data and their transaction statuses. These are large and take a lot of time to fetch from the database. Storing them in an LRU cache will allow to reduce database accesses when many subsequent requests are related to the same blocks."],["EthDevSigner",""],["EthFilter",""],["EthPubSub","Eth pub-sub API implementation."],["EthTask",""],["EthereumSubIdProvider",""],["Net","Net API implementation."],["OverrideHandle",""],["RuntimeApiStorageOverride","A wrapper type for the Runtime API. This type implements `StorageOverride`, so it can be used when calling the runtime API is desired but a `dyn StorageOverride` is required."],["SchemaV1Override","An override for runtimes that use Schema V1"],["SchemaV2Override","An override for runtimes that use Schema V2"],["SchemaV3Override","An override for runtimes that use Schema V3"],["Web3","Web3 API implementation."]],"trait":[["EthApiServer","Server trait implementation for the `EthApi` RPC API."],["EthFilterApiServer","Server trait implementation for the `EthFilterApi` RPC API."],["EthPubSubApiServer","Server trait implementation for the `EthPubSubApi` RPC API."],["EthSigner","A generic Ethereum signer."],["NetApiServer","Server trait implementation for the `NetApi` RPC API."],["StorageOverride","Something that can fetch Ethereum-related data. This trait is quite similar to the runtime API, and indeed oe implementation of it uses the runtime API. Having this trait is useful because it allows optimized implementations that fetch data from a State Backend with some assumptions about pallet-ethereum’s storage schema. Using such an optimized implementation avoids spawning a runtime and the overhead associated with it."],["Web3ApiServer","Server trait implementation for the `Web3Api` RPC API."]]});