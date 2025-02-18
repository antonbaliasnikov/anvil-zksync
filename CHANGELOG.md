# Changelog

## [0.2.5](https://github.com/antonbaliasnikov/anvil-zksync/compare/v0.2.4...v0.2.5) (2025-01-17)


### Features

* Adding (experimental) zkos support ([#534](https://github.com/antonbaliasnikov/anvil-zksync/issues/534)) ([35c3009](https://github.com/antonbaliasnikov/anvil-zksync/commit/35c30095d361f184bbb4ad356cccb05bfd4081e2))


### Bug Fixes

* anvil-zksync able to start even if the port is busy ([#542](https://github.com/antonbaliasnikov/anvil-zksync/issues/542)) ([bb90831](https://github.com/antonbaliasnikov/anvil-zksync/commit/bb908317cd1c28ff23aebdd764c108f16a1fe370))
* replaying transactions fixed ([#544](https://github.com/antonbaliasnikov/anvil-zksync/issues/544)) ([3afd716](https://github.com/antonbaliasnikov/anvil-zksync/commit/3afd71614c10cee6715f3def1f9627ecfc9392b7))

## [0.3.0](https://github.com/antonbaliasnikov/anvil-zksync/compare/v0.2.5...v0.3.0) (2025-01-17)


### Features

* adapting Era test node to the new VM (and updating system contracts) ([#111](https://github.com/antonbaliasnikov/anvil-zksync/issues/111)) ([915b0c0](https://github.com/antonbaliasnikov/anvil-zksync/commit/915b0c0086173668a628d851e1a22430f74a2487))
* add --show-outputs flag ([#269](https://github.com/antonbaliasnikov/anvil-zksync/issues/269)) ([b3ed0ff](https://github.com/antonbaliasnikov/anvil-zksync/commit/b3ed0ff7dc52ccec33ed929c2b04cb434feeafb4))
* add `anvil_{get,set}Automine`/`anvil_setIntervalMining` ([#446](https://github.com/antonbaliasnikov/anvil-zksync/issues/446)) ([5c0d204](https://github.com/antonbaliasnikov/anvil-zksync/commit/5c0d204f809476538ea94e3dab69912e68e7ada9))
* add `anvil_{set,remove}BlockTimestampInterval` ([#442](https://github.com/antonbaliasnikov/anvil-zksync/issues/442)) ([2e9c7c8](https://github.com/antonbaliasnikov/anvil-zksync/commit/2e9c7c83d6901e269bbaa1f680a4d9b4f9f4ae83))
* add `anvil_` pool manipulation methods ([#447](https://github.com/antonbaliasnikov/anvil-zksync/issues/447)) ([5aa6e15](https://github.com/antonbaliasnikov/anvil-zksync/commit/5aa6e1584973845214c344b74148f3cbf68d8567))
* add `anvil_autoImpersonateAccount` ([#416](https://github.com/antonbaliasnikov/anvil-zksync/issues/416)) ([ebc7781](https://github.com/antonbaliasnikov/anvil-zksync/commit/ebc778194c84b7044b51aa786eedbb203d1ba181))
* add `anvil_dumpState`/`anvil_loadState` ([#476](https://github.com/antonbaliasnikov/anvil-zksync/issues/476)) ([7d0b689](https://github.com/antonbaliasnikov/anvil-zksync/commit/7d0b689092729b06f4a077b5c94e8e56cac9bb82))
* add `anvil_setLoggingEnabled` ([#437](https://github.com/antonbaliasnikov/anvil-zksync/issues/437)) ([7f5b663](https://github.com/antonbaliasnikov/anvil-zksync/commit/7f5b6631c4dc6b7835e0229e2cdf90ebde74f02d))
* add `anvil_setMinGasPrice` as an unimplemented method ([#438](https://github.com/antonbaliasnikov/anvil-zksync/issues/438)) ([45b9fd4](https://github.com/antonbaliasnikov/anvil-zksync/commit/45b9fd436425ffa6c197ac8fa93b946cde23b529))
* add `anvil_setNextBlockBaseFeePerGas` ([#450](https://github.com/antonbaliasnikov/anvil-zksync/issues/450)) ([6e6dba2](https://github.com/antonbaliasnikov/anvil-zksync/commit/6e6dba254627858d2631c596f5a421155869856c))
* add `anvil_setTime`, `anvil_increaseTime`, `anvil_setNextBlockTimestamp` ([#421](https://github.com/antonbaliasnikov/anvil-zksync/issues/421)) ([9818ee5](https://github.com/antonbaliasnikov/anvil-zksync/commit/9818ee5943a9457e45bda2ca9fd745e48d2c6404))
* add `anvil_snapshot`/`anvil_revert` ([#432](https://github.com/antonbaliasnikov/anvil-zksync/issues/432)) ([ede79a3](https://github.com/antonbaliasnikov/anvil-zksync/commit/ede79a320fe24ac02dfec758ef96dc8a6bd46093))
* Add binary attestation ([#511](https://github.com/antonbaliasnikov/anvil-zksync/issues/511)) ([43e544f](https://github.com/antonbaliasnikov/anvil-zksync/commit/43e544f60ea835808d8fe764b99f047e2493e0e5))
* Add caching to HttpForkSource ([#114](https://github.com/antonbaliasnikov/anvil-zksync/issues/114)) ([f3e1528](https://github.com/antonbaliasnikov/anvil-zksync/commit/f3e1528d8952d1d2328aca1659e0f3897f0a2d92))
* add cli options for server configuration ([#487](https://github.com/antonbaliasnikov/anvil-zksync/issues/487)) ([07ebfbc](https://github.com/antonbaliasnikov/anvil-zksync/commit/07ebfbc84017a2b4945abe3db735a27869744765))
* Add debug-mode flag ([#343](https://github.com/antonbaliasnikov/anvil-zksync/issues/343)) ([8978a21](https://github.com/antonbaliasnikov/anvil-zksync/commit/8978a212ced3098c74683dcd2423f1f54ee186bb))
* add docker build for amd64 / arm64, e2e tests, publishing ghcr ([#503](https://github.com/antonbaliasnikov/anvil-zksync/issues/503)) ([133121d](https://github.com/antonbaliasnikov/anvil-zksync/commit/133121dac20b2311c31fd03ea1cc641943896834))
* add eth_feeHistory ([#121](https://github.com/antonbaliasnikov/anvil-zksync/issues/121)) ([270e65a](https://github.com/antonbaliasnikov/anvil-zksync/commit/270e65aa167c984913e387d44828a91a6c1b9222))
* Add eth_getBlockTransactionCountByHash and eth_getBlockTransactionCountByNumber ([#117](https://github.com/antonbaliasnikov/anvil-zksync/issues/117)) ([35ac2c7](https://github.com/antonbaliasnikov/anvil-zksync/commit/35ac2c70fb850a1a9e1406f05e2cee5cfe9a9941))
* add eth_getLogs and eth_getFilterLogs ([#130](https://github.com/antonbaliasnikov/anvil-zksync/issues/130)) ([8b2f8c4](https://github.com/antonbaliasnikov/anvil-zksync/commit/8b2f8c46fc7b0c644ab0fa8cf99fecfbcf31d40b))
* add eth_getStorageAt ([#134](https://github.com/antonbaliasnikov/anvil-zksync/issues/134)) ([b790006](https://github.com/antonbaliasnikov/anvil-zksync/commit/b790006734aba5b2c8c6787610ebde4f24ef2a18))
* add eth_getTransactionByBlockHashAndIndex and eth_getTransactionByBlockNumberAndIndex ([#159](https://github.com/antonbaliasnikov/anvil-zksync/issues/159)) ([79f3dae](https://github.com/antonbaliasnikov/anvil-zksync/commit/79f3dae2858fd91e9ae145e7b9141d126356c395))
* add eth_protocolVersion ([#161](https://github.com/antonbaliasnikov/anvil-zksync/issues/161)) ([f691f8c](https://github.com/antonbaliasnikov/anvil-zksync/commit/f691f8ca38f2a50f6cfb9e2a5622563061155d17))
* add evm_snapshot/revert ([#158](https://github.com/antonbaliasnikov/anvil-zksync/issues/158)) ([166247d](https://github.com/antonbaliasnikov/anvil-zksync/commit/166247dff700ef08ba82d5063bc2a18c3a436e89))
* add filter support ([#124](https://github.com/antonbaliasnikov/anvil-zksync/issues/124)) ([ee82bc3](https://github.com/antonbaliasnikov/anvil-zksync/commit/ee82bc3d2226e23d5f70174ef20a7d25839c7137))
* add flags to control console logs ([#443](https://github.com/antonbaliasnikov/anvil-zksync/issues/443)) ([6863404](https://github.com/antonbaliasnikov/anvil-zksync/commit/68634048eb197abb472d16c98e308523c4957d48))
* add hardhat_setCode ([#171](https://github.com/antonbaliasnikov/anvil-zksync/issues/171)) ([85f238b](https://github.com/antonbaliasnikov/anvil-zksync/commit/85f238b6247beafa4bf1128fa4b0c6c0280e1369))
* add logs for incoming API requests ([#118](https://github.com/antonbaliasnikov/anvil-zksync/issues/118)) ([19b7cca](https://github.com/antonbaliasnikov/anvil-zksync/commit/19b7cca214fb27e3677110a606c7e9397668753d))
* add parent hash linking for blocks ([#209](https://github.com/antonbaliasnikov/anvil-zksync/issues/209)) ([96632f9](https://github.com/antonbaliasnikov/anvil-zksync/commit/96632f9a3c6b717ca1f619b042407f462252cc70))
* Add persistent cache to OpenChain hash resolving requests ([#348](https://github.com/antonbaliasnikov/anvil-zksync/issues/348)) ([986054e](https://github.com/antonbaliasnikov/anvil-zksync/commit/986054e13653878a35ffa0b087a2f416fd1dc52c))
* Add Pull Request template ([#11](https://github.com/antonbaliasnikov/anvil-zksync/issues/11)) ([f13f0cb](https://github.com/antonbaliasnikov/anvil-zksync/commit/f13f0cbcb732f65793b72b037c04f07d3e30b351))
* add rustbook ([#163](https://github.com/antonbaliasnikov/anvil-zksync/issues/163)) ([124691a](https://github.com/antonbaliasnikov/anvil-zksync/commit/124691a8c690244541cb0535d07d21e622c5ad8a))
* Add simplelog ([#103](https://github.com/antonbaliasnikov/anvil-zksync/issues/103)) ([0cac26b](https://github.com/antonbaliasnikov/anvil-zksync/commit/0cac26b9784d7fe218aec1522b256d72a092c4cc))
* Add support for health check endpoint ([#410](https://github.com/antonbaliasnikov/anvil-zksync/issues/410)) ([2b7b45c](https://github.com/antonbaliasnikov/anvil-zksync/commit/2b7b45c214fe113de4046e1f6578202650f0b90a))
* Add support for sepolia testnet ([#250](https://github.com/antonbaliasnikov/anvil-zksync/issues/250)) ([578cdbd](https://github.com/antonbaliasnikov/anvil-zksync/commit/578cdbd97ef5f2bf3ea3351f0cc68ebce03457b0))
* add support for timestamp asserter ([#383](https://github.com/antonbaliasnikov/anvil-zksync/issues/383)) ([b6bf74e](https://github.com/antonbaliasnikov/anvil-zksync/commit/b6bf74eba4d3b4dea5ecba228c614fac4f4f069f))
* Add support for zks_getBaseTokenL1Address. Add gas used per call in the stack. ([#350](https://github.com/antonbaliasnikov/anvil-zksync/issues/350)) ([3a6451d](https://github.com/antonbaliasnikov/anvil-zksync/commit/3a6451d908538f71b65ca6520d2671f20e2f4e45))
* Add well-known log selectors to console output ([#162](https://github.com/antonbaliasnikov/anvil-zksync/issues/162)) ([98b8420](https://github.com/antonbaliasnikov/anvil-zksync/commit/98b8420b46eadcde236d897b531b3f369846f865))
* add zks_getBridgeContracts ([#184](https://github.com/antonbaliasnikov/anvil-zksync/issues/184)) ([46d1b4d](https://github.com/antonbaliasnikov/anvil-zksync/commit/46d1b4da8d0add813e8ca33b8d45e11c833d6d8f))
* add zks_getBytecodeByHash ([#180](https://github.com/antonbaliasnikov/anvil-zksync/issues/180)) ([ee35740](https://github.com/antonbaliasnikov/anvil-zksync/commit/ee35740b997905a2c9315aa2a0c3bbd8eafe9b8a))
* Added detailed information on amount of pubdata per write ([#248](https://github.com/antonbaliasnikov/anvil-zksync/issues/248)) ([e91949f](https://github.com/antonbaliasnikov/anvil-zksync/commit/e91949fcc7f44884ca6098419be7bb11d155aad6))
* added hardhat_setStorageAt ([#298](https://github.com/antonbaliasnikov/anvil-zksync/issues/298)) ([5862c6a](https://github.com/antonbaliasnikov/anvil-zksync/commit/5862c6af10b9c8c9f91447e56a6bff78a2752b69))
* Adding (experimental) zkos support ([#534](https://github.com/antonbaliasnikov/anvil-zksync/issues/534)) ([35c3009](https://github.com/antonbaliasnikov/anvil-zksync/commit/35c30095d361f184bbb4ad356cccb05bfd4081e2))
* adds `--auto-impersonate` cli cmd ([#430](https://github.com/antonbaliasnikov/anvil-zksync/issues/430)) ([76bdf23](https://github.com/antonbaliasnikov/anvil-zksync/commit/76bdf23523774b64c830752cf94f8cecccd8675d))
* adds `eth_accounts` and updates `get_transaction_by_hash` to retrieve from fork if needed ([#139](https://github.com/antonbaliasnikov/anvil-zksync/issues/139)) ([4ffd847](https://github.com/antonbaliasnikov/anvil-zksync/commit/4ffd847f85686400b36882720808bdfbbe986a26))
* adds `fork-transaction-hash` and rename `network` to `fork-url` ([#431](https://github.com/antonbaliasnikov/anvil-zksync/issues/431)) ([4a99b85](https://github.com/antonbaliasnikov/anvil-zksync/commit/4a99b8522f19b1a52facddfa5fc2ee77d61f89bf))
* adds account configuration options ([#388](https://github.com/antonbaliasnikov/anvil-zksync/issues/388)) ([febc5e2](https://github.com/antonbaliasnikov/anvil-zksync/commit/febc5e2e762a20e781f8a46909f256e2fe656087))
* adds config-out cli option ([#394](https://github.com/antonbaliasnikov/anvil-zksync/issues/394)) ([5e17d02](https://github.com/antonbaliasnikov/anvil-zksync/commit/5e17d02456c7d49c29c1bc6f3a5da5f3b09bb89c))
* adds hardhat_getAutomine for evm emulator and hardhat ignition usage ([#357](https://github.com/antonbaliasnikov/anvil-zksync/issues/357)) ([28cad46](https://github.com/antonbaliasnikov/anvil-zksync/commit/28cad46223d3c653952095ec5ec4f67d38177dbe))
* adds init and timestamp genesis cli params ([#441](https://github.com/antonbaliasnikov/anvil-zksync/issues/441)) ([e5cd460](https://github.com/antonbaliasnikov/anvil-zksync/commit/e5cd460e2b29b833ad912b3038ead610bec9362d))
* adds-host-cli-option ([#395](https://github.com/antonbaliasnikov/anvil-zksync/issues/395)) ([2b24554](https://github.com/antonbaliasnikov/anvil-zksync/commit/2b24554217d46c643da176d225df3738a6235228))
* Allow bytecode replacement ([#336](https://github.com/antonbaliasnikov/anvil-zksync/issues/336)) ([adb900f](https://github.com/antonbaliasnikov/anvil-zksync/commit/adb900f96e1493da63a722da0cb361e4cbe96229))
* Allow overriding the chain id when starting era-test-node locally ([#361](https://github.com/antonbaliasnikov/anvil-zksync/issues/361)) ([6f4cc22](https://github.com/antonbaliasnikov/anvil-zksync/commit/6f4cc22c622297e9e0bfb6dc378a192701d7b6ce))
* allow returning data from transactions ([#232](https://github.com/antonbaliasnikov/anvil-zksync/issues/232)) ([21b48af](https://github.com/antonbaliasnikov/anvil-zksync/commit/21b48af90a9f9d98ec38cb93a32c119a3266f401))
* Allow to pass custom tracers to run_raw_tx ([#231](https://github.com/antonbaliasnikov/anvil-zksync/issues/231)) ([64835d2](https://github.com/antonbaliasnikov/anvil-zksync/commit/64835d25a6e4b1540a091ff0d805b01227213969))
* anvil API ([#313](https://github.com/antonbaliasnikov/anvil-zksync/issues/313)) ([706d726](https://github.com/antonbaliasnikov/anvil-zksync/commit/706d72605c55bc9b373e4f64fc190edcef5b5156))
* anvil set chain id ([#486](https://github.com/antonbaliasnikov/anvil-zksync/issues/486)) ([3ecf953](https://github.com/antonbaliasnikov/anvil-zksync/commit/3ecf95330bea7431231d8cf2caf534fc1739799b))
* asynchronous and configurable block sealing ([#392](https://github.com/antonbaliasnikov/anvil-zksync/issues/392)) ([47b3bf2](https://github.com/antonbaliasnikov/anvil-zksync/commit/47b3bf201b75d72f49daa7f0177da224be0d478a))
* boojum integration and eth_sendTransaction support ([#235](https://github.com/antonbaliasnikov/anvil-zksync/issues/235)) ([104da8e](https://github.com/antonbaliasnikov/anvil-zksync/commit/104da8e15a9074c97a878bc126f68dcbd2498810))
* **cli:** adds `--dump-state` cli cmd  ([#496](https://github.com/antonbaliasnikov/anvil-zksync/issues/496)) ([1f0a4b6](https://github.com/antonbaliasnikov/anvil-zksync/commit/1f0a4b64756bfcda44827e478f243048a4df7e1e))
* **cli:** adds load and state cli options ([#502](https://github.com/antonbaliasnikov/anvil-zksync/issues/502)) ([070c569](https://github.com/antonbaliasnikov/anvil-zksync/commit/070c569a332436a17836fb5a548e323ea1ef878c))
* Conditionally enable EVM emulation ([#355](https://github.com/antonbaliasnikov/anvil-zksync/issues/355)) ([56c4e92](https://github.com/antonbaliasnikov/anvil-zksync/commit/56c4e92693b5dd5ab166e368b066d9169b438855))
* config file ([#308](https://github.com/antonbaliasnikov/anvil-zksync/issues/308)) ([a0827eb](https://github.com/antonbaliasnikov/anvil-zksync/commit/a0827eb6fff6253c3214aa980a5d5ac56a262f9e))
* custom l1 price ([#294](https://github.com/antonbaliasnikov/anvil-zksync/issues/294)) ([788dc0a](https://github.com/antonbaliasnikov/anvil-zksync/commit/788dc0a88446f3cea492d286c8f024fdc2882c7e))
* different scaling gas constants based on forked network ([#295](https://github.com/antonbaliasnikov/anvil-zksync/issues/295)) ([9c66ac3](https://github.com/antonbaliasnikov/anvil-zksync/commit/9c66ac371df8fe6352fc53991bd846178f76f710))
* display more details about the gas fees ([#86](https://github.com/antonbaliasnikov/anvil-zksync/issues/86)) ([dd88a7f](https://github.com/antonbaliasnikov/anvil-zksync/commit/dd88a7f536e435ff3abd1130137af91ff2f0d184))
* display the revert reason when calls or gas estimations fail ([#98](https://github.com/antonbaliasnikov/anvil-zksync/issues/98)) ([cbac9a1](https://github.com/antonbaliasnikov/anvil-zksync/commit/cbac9a1830ac8d1212036cdf97ee0a125ef922fa))
* dynamic fee model config ([#296](https://github.com/antonbaliasnikov/anvil-zksync/issues/296)) ([e0439f5](https://github.com/antonbaliasnikov/anvil-zksync/commit/e0439f5ee1c2357f06ef94ad47f9c239068d7f6b))
* eth spec conformation test suite ([#373](https://github.com/antonbaliasnikov/anvil-zksync/issues/373)) ([43fe551](https://github.com/antonbaliasnikov/anvil-zksync/commit/43fe551077145c075ce5665df73e62e51c91108d))
* fees transactions order in mempool ([#492](https://github.com/antonbaliasnikov/anvil-zksync/issues/492)) ([4f35268](https://github.com/antonbaliasnikov/anvil-zksync/commit/4f3526899378a453f1fae6701496f40b44db3ad7))
* forbid ".only" from e2e-tests ([#179](https://github.com/antonbaliasnikov/anvil-zksync/issues/179)) ([d802216](https://github.com/antonbaliasnikov/anvil-zksync/commit/d802216a39ba07c6240f3e3a15192158e04c2bd3))
* get block returns null for non existing blocks ([#218](https://github.com/antonbaliasnikov/anvil-zksync/issues/218)) ([668a2cf](https://github.com/antonbaliasnikov/anvil-zksync/commit/668a2cfd39d82dd00fb90f914bfce0b97a563572))
* hardhat_reset ([#302](https://github.com/antonbaliasnikov/anvil-zksync/issues/302)) ([1afa81a](https://github.com/antonbaliasnikov/anvil-zksync/commit/1afa81aee9301039ff0dbdf8415c5f99290491e3))
* impl `debug_traceBlockByHash` and `debug_traceBlockByNumber` ([#168](https://github.com/antonbaliasnikov/anvil-zksync/issues/168)) ([3ff215f](https://github.com/antonbaliasnikov/anvil-zksync/commit/3ff215f88cf55c0b6fc3fb5669bc0befd7957190))
* impl `eth_syncing` ([#104](https://github.com/antonbaliasnikov/anvil-zksync/issues/104)) ([fee3152](https://github.com/antonbaliasnikov/anvil-zksync/commit/fee3152511d79cc53ffd7f9b71c01f09287242f8))
* impl `evm_mine` and `hardhat_mine` ([#116](https://github.com/antonbaliasnikov/anvil-zksync/issues/116)) ([0b73bd3](https://github.com/antonbaliasnikov/anvil-zksync/commit/0b73bd3211d9b83403f52c5c6c8d81fccec6ae88))
* impl `hardhat_impersonateAccount` and `hardhat_stopImpersonationAccount` ([#125](https://github.com/antonbaliasnikov/anvil-zksync/issues/125)) ([e2f07b2](https://github.com/antonbaliasnikov/anvil-zksync/commit/e2f07b2eed8515748b6182b590f9f0f0231be57b))
* impl `zks_getAllAccountBalances` and `zks_getConfirmedTokens` ([#198](https://github.com/antonbaliasnikov/anvil-zksync/issues/198)) ([c8fa0b1](https://github.com/antonbaliasnikov/anvil-zksync/commit/c8fa0b1b0bebf70e8e97e90d2d3e0ae40b2cff8a))
* impl `zks_getTransactionDetails` ([#176](https://github.com/antonbaliasnikov/anvil-zksync/issues/176)) ([81340b1](https://github.com/antonbaliasnikov/anvil-zksync/commit/81340b1989478df264f76092bd64212bb98e43a6))
* impl debug_traceCall ([#151](https://github.com/antonbaliasnikov/anvil-zksync/issues/151)) ([4e35082](https://github.com/antonbaliasnikov/anvil-zksync/commit/4e3508286a1d739912b4a8578565f20d8cdf4188))
* impl debug_traceTransaction ([#165](https://github.com/antonbaliasnikov/anvil-zksync/issues/165)) ([f806bb9](https://github.com/antonbaliasnikov/anvil-zksync/commit/f806bb97720d74e356727d031fac5f28850f2b94))
* impl zks_getBlockDetails ([#182](https://github.com/antonbaliasnikov/anvil-zksync/issues/182)) ([b13fa9d](https://github.com/antonbaliasnikov/anvil-zksync/commit/b13fa9d33e61e8b288dd5011739a78b6a1b6e61b))
* impl zks_getRawBlockTransactions ([#185](https://github.com/antonbaliasnikov/anvil-zksync/issues/185)) ([512484e](https://github.com/antonbaliasnikov/anvil-zksync/commit/512484ede8597490eb9fe44461a579a4849cc294))
* implement eth_estimateGas and zks_estimateFee ([#31](https://github.com/antonbaliasnikov/anvil-zksync/issues/31)) ([4ef5a96](https://github.com/antonbaliasnikov/anvil-zksync/commit/4ef5a96ec1525040f920f0a892d238e76529af23))
* Implement eth_getBlockByHash and eth_getBlockByNumber with correct behavior ([#102](https://github.com/antonbaliasnikov/anvil-zksync/issues/102)) ([f263687](https://github.com/antonbaliasnikov/anvil-zksync/commit/f263687055c6290430d1a04e866036c38f22c1d2))
* Implement evm_increaseTime, evm_setNextBlockTimestamp, and evm_setTime ([#93](https://github.com/antonbaliasnikov/anvil-zksync/issues/93)) ([40cf025](https://github.com/antonbaliasnikov/anvil-zksync/commit/40cf025d48e46d246e278680b55f2051e608a69a))
* implement hardhat_setNonce ([#101](https://github.com/antonbaliasnikov/anvil-zksync/issues/101)) ([14ef9bb](https://github.com/antonbaliasnikov/anvil-zksync/commit/14ef9bb49ef90303c92c4457acc2ac2443337c26))
* Implement web3_clientVersion ([#223](https://github.com/antonbaliasnikov/anvil-zksync/issues/223)) ([7f68fc4](https://github.com/antonbaliasnikov/anvil-zksync/commit/7f68fc498fe618834a890cbd56a5d35b4c9bc7f7))
* implement zks_getTokenPrice endpoint ([#58](https://github.com/antonbaliasnikov/anvil-zksync/issues/58)) ([c860c9b](https://github.com/antonbaliasnikov/anvil-zksync/commit/c860c9bf3c0a134e65b868f01f3e4a6f5b66575e))
* implements required api methods for deploying contracts with `hardhat-zksync-deploy` plugin ([#9](https://github.com/antonbaliasnikov/anvil-zksync/issues/9)) ([e18a9dc](https://github.com/antonbaliasnikov/anvil-zksync/commit/e18a9dcbab886a306b766b3ca840394d122569ec))
* installer script ([#301](https://github.com/antonbaliasnikov/anvil-zksync/issues/301)) ([8882bd0](https://github.com/antonbaliasnikov/anvil-zksync/commit/8882bd010bfacbe984e9c3fb889b51183c0391a1))
* integrate v24 into the test node ([#282](https://github.com/antonbaliasnikov/anvil-zksync/issues/282)) ([ae81a43](https://github.com/antonbaliasnikov/anvil-zksync/commit/ae81a43a7479007a4654ff022e5076b470e1c419))
* l2 gas cli argument ([#287](https://github.com/antonbaliasnikov/anvil-zksync/issues/287)) ([e009070](https://github.com/antonbaliasnikov/anvil-zksync/commit/e0090706173c27f42f0179e2eec1d00760f534d4))
* move storage view, allow ignoring bootloader execution during tx execution ([#251](https://github.com/antonbaliasnikov/anvil-zksync/issues/251)) ([6ee7d29](https://github.com/antonbaliasnikov/anvil-zksync/commit/6ee7d29e876b75506f58355218e1ea755a315d17))
* populate block fields ([#360](https://github.com/antonbaliasnikov/anvil-zksync/issues/360)) ([3cd918b](https://github.com/antonbaliasnikov/anvil-zksync/commit/3cd918b7fd5c660dec71389d45f4abb4de400a2c))
* Port test-node to most recent core revision ([#284](https://github.com/antonbaliasnikov/anvil-zksync/issues/284)) ([7ed3e5d](https://github.com/antonbaliasnikov/anvil-zksync/commit/7ed3e5d4e0371d08a61dc9aa1855bdbaad46f743))
* procure l2 gas price when forking ([#290](https://github.com/antonbaliasnikov/anvil-zksync/issues/290)) ([93946cc](https://github.com/antonbaliasnikov/anvil-zksync/commit/93946ccf84091d18111fd4fc2f670a856e0564ae))
* protocol 20(vm 1.4.1 + fee model) integration ([#258](https://github.com/antonbaliasnikov/anvil-zksync/issues/258)) ([76608a2](https://github.com/antonbaliasnikov/anvil-zksync/commit/76608a231cc6770ed492fe7b0f69a168d4eeca6c))
* protocol(22) support and upstream changes ([#266](https://github.com/antonbaliasnikov/anvil-zksync/issues/266)) ([dc0b89d](https://github.com/antonbaliasnikov/anvil-zksync/commit/dc0b89d53773e7b2af94bf4e614911fcf8cbf1bd))
* refactor logging to use tracing crate and make it dynamic ([#187](https://github.com/antonbaliasnikov/anvil-zksync/issues/187)) ([1dde85d](https://github.com/antonbaliasnikov/anvil-zksync/commit/1dde85d4b16fb1d450b39538883a27a05a053970))
* showing low-level VM errors ([#311](https://github.com/antonbaliasnikov/anvil-zksync/issues/311)) ([d35eb77](https://github.com/antonbaliasnikov/anvil-zksync/commit/d35eb774c70b4b8287a1feb2c5f0a52661705206))
* **spec-tests:** add execution-apis submodule ([#381](https://github.com/antonbaliasnikov/anvil-zksync/issues/381)) ([2fe746d](https://github.com/antonbaliasnikov/anvil-zksync/commit/2fe746d0aa59e0daa0eb8129401e1c45034cb145))
* **spec-tests:** enforce spec-tests in CI ([#400](https://github.com/antonbaliasnikov/anvil-zksync/issues/400)) ([de9ce0f](https://github.com/antonbaliasnikov/anvil-zksync/commit/de9ce0f0a1ecf37e44f6a371227a8e2c8c2d00bf))
* support builtInWithoutSecurity option ([#186](https://github.com/antonbaliasnikov/anvil-zksync/issues/186)) ([29bb65d](https://github.com/antonbaliasnikov/anvil-zksync/commit/29bb65db5005361376ecdf06f612bb44e89764da))
* Support stable Rust ([#510](https://github.com/antonbaliasnikov/anvil-zksync/issues/510)) ([49b627d](https://github.com/antonbaliasnikov/anvil-zksync/commit/49b627d7c016cde705cec58d3e5d398a7f2143a8))
* support zks_L1ChainId ([#262](https://github.com/antonbaliasnikov/anvil-zksync/issues/262)) ([207729d](https://github.com/antonbaliasnikov/anvil-zksync/commit/207729dfee4043c4f19557c0b2227ba478a51049))
* Treat 'run' as default command ([#305](https://github.com/antonbaliasnikov/anvil-zksync/issues/305)) ([f8e2b97](https://github.com/antonbaliasnikov/anvil-zksync/commit/f8e2b97f884b1f3514ecbd4349307399410ecf77))
* update `multivm::vm_virtual_blocks` dependency to `multivim::vm_latest` ([#220](https://github.com/antonbaliasnikov/anvil-zksync/issues/220)) ([7efa571](https://github.com/antonbaliasnikov/anvil-zksync/commit/7efa571fd9eaca3a28b3dbf2c49621348d96c020))
* Update event formatter and observability formatter ([#226](https://github.com/antonbaliasnikov/anvil-zksync/issues/226)) ([5307734](https://github.com/antonbaliasnikov/anvil-zksync/commit/53077346551ea7f45aa1f7b44e85192be7ddbfe9))
* **vm:** Adapt code for the latest zksync era changes ([#230](https://github.com/antonbaliasnikov/anvil-zksync/issues/230)) ([79f27e9](https://github.com/antonbaliasnikov/anvil-zksync/commit/79f27e9b9339f8555e923eeb57a216dd388fbfbc))


### Bug Fixes

* add back call stacks, console logs, and correct call stack count ([#155](https://github.com/antonbaliasnikov/anvil-zksync/issues/155)) ([14a50c7](https://github.com/antonbaliasnikov/anvil-zksync/commit/14a50c73da75803d32a0183a47796cf88c2f54cf))
* add missing version support and increase version in toml ([#283](https://github.com/antonbaliasnikov/anvil-zksync/issues/283)) ([c62ac0c](https://github.com/antonbaliasnikov/anvil-zksync/commit/c62ac0cd5ec189a351ceee9483166b4e4fce89d9))
* added errors to internal implementation of ReadStorage ([#292](https://github.com/antonbaliasnikov/anvil-zksync/issues/292)) ([2832bef](https://github.com/antonbaliasnikov/anvil-zksync/commit/2832beff1ae5bd96cc18dea3f60c8501ef2e8541))
* adjust system contracts directories ([#270](https://github.com/antonbaliasnikov/anvil-zksync/issues/270)) ([574c0d8](https://github.com/antonbaliasnikov/anvil-zksync/commit/574c0d8e29c0a7e046dc523b0c699e5cfba30983))
* anvil-zksync able to start even if the port is busy ([#542](https://github.com/antonbaliasnikov/anvil-zksync/issues/542)) ([bb90831](https://github.com/antonbaliasnikov/anvil-zksync/commit/bb908317cd1c28ff23aebdd764c108f16a1fe370))
* bind server to 0.0.0.0 for external access ([#115](https://github.com/antonbaliasnikov/anvil-zksync/issues/115)) ([d6529ab](https://github.com/antonbaliasnikov/anvil-zksync/commit/d6529ab32190997867a45b2bde85b389572db5f5))
* Bump hardhat deploy and solc versions ([#110](https://github.com/antonbaliasnikov/anvil-zksync/issues/110)) ([be53aec](https://github.com/antonbaliasnikov/anvil-zksync/commit/be53aec0b0ba2574ccfd6c4eddf10fda7c51d0c6))
* changed evm_setNextBlockTimestamp ([#321](https://github.com/antonbaliasnikov/anvil-zksync/issues/321)) ([b50fcd1](https://github.com/antonbaliasnikov/anvil-zksync/commit/b50fcd116ed4020512e654fb5f77465502660420))
* changed hardhat_setCode code type ([#327](https://github.com/antonbaliasnikov/anvil-zksync/issues/327)) ([2041018](https://github.com/antonbaliasnikov/anvil-zksync/commit/2041018903aa7e00e74c450f8c0baf799c056d86))
* default host to 0.0.0.0 ([#478](https://github.com/antonbaliasnikov/anvil-zksync/issues/478)) ([1485a57](https://github.com/antonbaliasnikov/anvil-zksync/commit/1485a57fc39d676eb9ba2838c118d31623a638b3))
* don't print console logs title when no console logs printed ([#500](https://github.com/antonbaliasnikov/anvil-zksync/issues/500)) ([e279c21](https://github.com/antonbaliasnikov/anvil-zksync/commit/e279c2100c7ed96685380c05d795be62581fc1c8))
* double gas limit, bootloader memory layout ([#289](https://github.com/antonbaliasnikov/anvil-zksync/issues/289)) ([1d72eae](https://github.com/antonbaliasnikov/anvil-zksync/commit/1d72eae6e9400b5b6caed5e60093e7c32eee8abb))
* fix forking for zksync-era@16.0.2 ([#194](https://github.com/antonbaliasnikov/anvil-zksync/issues/194)) ([5364ad0](https://github.com/antonbaliasnikov/anvil-zksync/commit/5364ad09a33c1bab768cb472844631464183c619))
* fix issue with balance usage ([#393](https://github.com/antonbaliasnikov/anvil-zksync/issues/393)) ([1fe4b40](https://github.com/antonbaliasnikov/anvil-zksync/commit/1fe4b40d5dddaca394063ba1f8f1891d7ddf648c))
* fix issue with gas params ([#471](https://github.com/antonbaliasnikov/anvil-zksync/issues/471)) ([def668a](https://github.com/antonbaliasnikov/anvil-zksync/commit/def668a35fa12c34738f236ddbaf5e70d2ba775d))
* Fix issue with incorrect recognition of initial writes ([#247](https://github.com/antonbaliasnikov/anvil-zksync/issues/247)) ([b4a7c06](https://github.com/antonbaliasnikov/anvil-zksync/commit/b4a7c065d60afa35f7a2713c01685b8d91f9598b))
* fixes anvil-zksync failing on ubuntu-latest ([#527](https://github.com/antonbaliasnikov/anvil-zksync/issues/527)) ([2c79e6b](https://github.com/antonbaliasnikov/anvil-zksync/commit/2c79e6ba4ed8c0be35a547cc7c5991f2f1e22a58))
* fixes broken logging and adds structure to logs ([#411](https://github.com/antonbaliasnikov/anvil-zksync/issues/411)) ([f3e58cd](https://github.com/antonbaliasnikov/anvil-zksync/commit/f3e58cd44ed276b600cb86fd530c9de37e1b5036))
* fixes gas params usage and add alias  ([#440](https://github.com/antonbaliasnikov/anvil-zksync/issues/440)) ([d3e51cc](https://github.com/antonbaliasnikov/anvil-zksync/commit/d3e51cc8c234f05dbee971233c9e538ae6100747))
* fixes install script for apple x86 ([#523](https://github.com/antonbaliasnikov/anvil-zksync/issues/523)) ([064efc4](https://github.com/antonbaliasnikov/anvil-zksync/commit/064efc401a99999ef341cb9d273e4a07e44e4221))
* fixes release pipeline due bump in rust version ([#351](https://github.com/antonbaliasnikov/anvil-zksync/issues/351)) ([09c7ed8](https://github.com/antonbaliasnikov/anvil-zksync/commit/09c7ed82c742ee7b28d398ec864357397bf2dbc9))
* hardhat_setCode error handling ([#293](https://github.com/antonbaliasnikov/anvil-zksync/issues/293)) ([df2125e](https://github.com/antonbaliasnikov/anvil-zksync/commit/df2125ee79252ba2f70cf39a7025777afa5108e0))
* hide multivm log warnings and traces ([#310](https://github.com/antonbaliasnikov/anvil-zksync/issues/310)) ([b591b36](https://github.com/antonbaliasnikov/anvil-zksync/commit/b591b362fe2c532f2f3b3be2135f3ad7afaa6878))
* Include transaction type in the transaction receipt ([#304](https://github.com/antonbaliasnikov/anvil-zksync/issues/304)) ([dd6d2f4](https://github.com/antonbaliasnikov/anvil-zksync/commit/dd6d2f463eb9697dc2365899a72ae12dae3ec809))
* install mdbook for ci ([#169](https://github.com/antonbaliasnikov/anvil-zksync/issues/169)) ([9012529](https://github.com/antonbaliasnikov/anvil-zksync/commit/901252914e87a1b4117acba031c5c09a1d2db3b5))
* L2 gas price change ([#285](https://github.com/antonbaliasnikov/anvil-zksync/issues/285)) ([d396b76](https://github.com/antonbaliasnikov/anvil-zksync/commit/d396b769c75372c737f38f1e5f70b3e35c812ebc))
* l2 gas price was decreased ([#271](https://github.com/antonbaliasnikov/anvil-zksync/issues/271)) ([2442c4a](https://github.com/antonbaliasnikov/anvil-zksync/commit/2442c4a503bfbcabf3eaae313b92b25611fb0f6c))
* make manual mining produce non-empty blocks ([#459](https://github.com/antonbaliasnikov/anvil-zksync/issues/459)) ([7587a7a](https://github.com/antonbaliasnikov/anvil-zksync/commit/7587a7a5e7ed9763190bfd5dfd0cf21b00adb60b))
* Print correct port for spawned server ([#513](https://github.com/antonbaliasnikov/anvil-zksync/issues/513)) ([b21c489](https://github.com/antonbaliasnikov/anvil-zksync/commit/b21c4898d44e3b775423b8d698d425fc43a85981))
* proper handling of halted txs ([#463](https://github.com/antonbaliasnikov/anvil-zksync/issues/463)) ([1578ee4](https://github.com/antonbaliasnikov/anvil-zksync/commit/1578ee4357987abcc33ff53ba7a21ec2e5b5cf90))
* properly set transaction index ([#499](https://github.com/antonbaliasnikov/anvil-zksync/issues/499)) ([100e3f3](https://github.com/antonbaliasnikov/anvil-zksync/commit/100e3f337c33a95a4b73016664dfae2aee326e88))
* release cross dep ([#260](https://github.com/antonbaliasnikov/anvil-zksync/issues/260)) ([a796760](https://github.com/antonbaliasnikov/anvil-zksync/commit/a796760cb831f472003b4643be95fa916296e751))
* Release drafts now attach files correctly ([#196](https://github.com/antonbaliasnikov/anvil-zksync/issues/196)) ([1ed74ea](https://github.com/antonbaliasnikov/anvil-zksync/commit/1ed74eaba01690db9cb1936d4d9e27f03c5ac9de))
* rename run_l2_tx_inner, fix panics for lib users ([#214](https://github.com/antonbaliasnikov/anvil-zksync/issues/214)) ([a74c39c](https://github.com/antonbaliasnikov/anvil-zksync/commit/a74c39c0d89aec29086524f9a8dd13bf2e4e03f6))
* replaying transactions fixed ([#544](https://github.com/antonbaliasnikov/anvil-zksync/issues/544)) ([3afd716](https://github.com/antonbaliasnikov/anvil-zksync/commit/3afd71614c10cee6715f3def1f9627ecfc9392b7))
* Resolve hashes now uses the new openchain.xyz endpoint ([#342](https://github.com/antonbaliasnikov/anvil-zksync/issues/342)) ([25b9eed](https://github.com/antonbaliasnikov/anvil-zksync/commit/25b9eede6e24ac721f7ae46c03456b67eac7af7e))
* resolves [#90](https://github.com/antonbaliasnikov/anvil-zksync/issues/90) | fixes release executable and headers issue ([#95](https://github.com/antonbaliasnikov/anvil-zksync/issues/95)) ([3041b7c](https://github.com/antonbaliasnikov/anvil-zksync/commit/3041b7ccf407e5e2e75c8e314d1c654a25d58722))
* returning errors from fork handling ([#335](https://github.com/antonbaliasnikov/anvil-zksync/issues/335)) ([c797d5f](https://github.com/antonbaliasnikov/anvil-zksync/commit/c797d5f8c29986f716ad2596d809c0f45e3a0615))
* returning errors from from_network and from_network_tx ([#316](https://github.com/antonbaliasnikov/anvil-zksync/issues/316)) ([fc4bea3](https://github.com/antonbaliasnikov/anvil-zksync/commit/fc4bea35252b0c48dbcedff342686c1859c6bdac))
* send transaction fees ([#297](https://github.com/antonbaliasnikov/anvil-zksync/issues/297)) ([c1f64c9](https://github.com/antonbaliasnikov/anvil-zksync/commit/c1f64c9f771ab4c69fb4155f7753b04385f0082b))
* Set transaction index to 0 in tx receipt ([#307](https://github.com/antonbaliasnikov/anvil-zksync/issues/307)) ([0cf7adc](https://github.com/antonbaliasnikov/anvil-zksync/commit/0cf7adc2e14d27335dc185b1f659ac5edc3746cb))
* try to change config ([9832297](https://github.com/antonbaliasnikov/anvil-zksync/commit/983229793ca001c51eb45694cc09226aa90e2edb))
* unit-tests fixed on main ([#193](https://github.com/antonbaliasnikov/anvil-zksync/issues/193)) ([f88cefb](https://github.com/antonbaliasnikov/anvil-zksync/commit/f88cefb1e3052c852151efb51fc07186e467f781))
* update `run` with dynamic fee params ([#377](https://github.com/antonbaliasnikov/anvil-zksync/issues/377)) ([2cece10](https://github.com/antonbaliasnikov/anvil-zksync/commit/2cece107848e8fa7130744012ad71ee11f6da730))
* update cargo.toml to reflect release ([#259](https://github.com/antonbaliasnikov/anvil-zksync/issues/259)) ([77f6358](https://github.com/antonbaliasnikov/anvil-zksync/commit/77f6358a4fe278a17b370c2836c5ad34e8c2b950))
* update compiled smart contracts to latest ([#157](https://github.com/antonbaliasnikov/anvil-zksync/issues/157)) ([4f627b5](https://github.com/antonbaliasnikov/anvil-zksync/commit/4f627b55cc90a89a7f439ca743208eefda093200))
* update config api reference ([#412](https://github.com/antonbaliasnikov/anvil-zksync/issues/412)) ([427f814](https://github.com/antonbaliasnikov/anvil-zksync/commit/427f8140734cfd1dbbdb4728bb7a8639041ce88f))
* Update gas estimation logic for forks ([#339](https://github.com/antonbaliasnikov/anvil-zksync/issues/339)) ([9e8e0e9](https://github.com/antonbaliasnikov/anvil-zksync/commit/9e8e0e91a0a44f8eca3af06b9d16f41ca07831b3))
* update hardhat and hardhat-zksync-solc in e2e-tests package ([#489](https://github.com/antonbaliasnikov/anvil-zksync/issues/489)) ([1c42504](https://github.com/antonbaliasnikov/anvil-zksync/commit/1c42504f0d834dac108fb3246bbee8201356d314))
* update trace logging to include bootloader ([#309](https://github.com/antonbaliasnikov/anvil-zksync/issues/309)) ([6c5d552](https://github.com/antonbaliasnikov/anvil-zksync/commit/6c5d5524bcc3bd48e471261fa81ad0073e3b5851))
* Update transaction type for gas estimation if one is not provided for EIP712 transactions, this fixes paymasters for era-test-node. ([#195](https://github.com/antonbaliasnikov/anvil-zksync/issues/195)) ([0b4a200](https://github.com/antonbaliasnikov/anvil-zksync/commit/0b4a200944da2b58e91c8acfaded704bd75ef22b))
* update typo in link to dev discussion ([#243](https://github.com/antonbaliasnikov/anvil-zksync/issues/243)) ([cc2ab1b](https://github.com/antonbaliasnikov/anvil-zksync/commit/cc2ab1bb031bbd72b2bef2f7b615c4044261f2bf))
* update zksync-era deps to v16.0.0 ([#173](https://github.com/antonbaliasnikov/anvil-zksync/issues/173)) ([3bad022](https://github.com/antonbaliasnikov/anvil-zksync/commit/3bad022e3d4942cf02cad36f8fdb2c275af4f4f7))
* Use actual replayed tx's  gas and pubdata prices to replay tx. ([#319](https://github.com/antonbaliasnikov/anvil-zksync/issues/319)) ([2a6ea11](https://github.com/antonbaliasnikov/anvil-zksync/commit/2a6ea11d56cb21c504c44dca521ec56e602e1bc4))
* use specific zksolc version for e2e tests ([#488](https://github.com/antonbaliasnikov/anvil-zksync/issues/488)) ([f88daea](https://github.com/antonbaliasnikov/anvil-zksync/commit/f88daea9243e5399ae7b64901c576ce3448b654a))
* validate gas_limit and max_fee_per_gas before transaction execution ([#207](https://github.com/antonbaliasnikov/anvil-zksync/issues/207)) ([c5350bf](https://github.com/antonbaliasnikov/anvil-zksync/commit/c5350bf89a25abc97304a86189fc983881308b8f))

## [0.2.5](https://github.com/antonbaliasnikov/anvil-zksync/compare/v0.2.4...v0.2.5) (2025-01-17)


### Features

* Adding (experimental) zkos support ([#534](https://github.com/antonbaliasnikov/anvil-zksync/issues/534)) ([35c3009](https://github.com/antonbaliasnikov/anvil-zksync/commit/35c30095d361f184bbb4ad356cccb05bfd4081e2))


### Bug Fixes

* anvil-zksync able to start even if the port is busy ([#542](https://github.com/antonbaliasnikov/anvil-zksync/issues/542)) ([bb90831](https://github.com/antonbaliasnikov/anvil-zksync/commit/bb908317cd1c28ff23aebdd764c108f16a1fe370))
* replaying transactions fixed ([#544](https://github.com/antonbaliasnikov/anvil-zksync/issues/544)) ([3afd716](https://github.com/antonbaliasnikov/anvil-zksync/commit/3afd71614c10cee6715f3def1f9627ecfc9392b7))
