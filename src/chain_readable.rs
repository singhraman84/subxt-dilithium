#[allow(
    dead_code,
    missing_docs,
    unused_imports,
    non_camel_case_types,
    unreachable_patterns
)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 8usize] = [
        "System",
        "Timestamp",
        "Aura",
        "Grandpa",
        "Balances",
        "TransactionPayment",
        "Sudo",
        "Template",
    ];
    pub static RUNTIME_APIS: [&str; 13usize] = [
        "Core",
        "Metadata",
        "RuntimeViewFunction",
        "BlockBuilder",
        "TaggedTransactionQueue",
        "OffchainWorkerApi",
        "AuraApi",
        "SessionKeys",
        "GrandpaApi",
        "AccountNonceApi",
        "TransactionPaymentApi",
        "TransactionPaymentCallApi",
        "GenesisBuilder",
    ];
    #[doc = r" The error type that is returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::solochain_template_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::solochain_template_runtime::RuntimeCall;
    #[doc = r" The outer error enum represents the DispatchError's Module variant."]
    pub type Error = runtime_types::solochain_template_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    #[doc = r" This is an alias to [`Self::transactions()`]."]
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn transactions() -> TransactionApi {
        TransactionApi
    }
    pub fn runtime_apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use subxt::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {
            pub fn core(&self) -> core::Core {
                core::Core
            }
            pub fn metadata(&self) -> metadata::Metadata {
                metadata::Metadata
            }
            pub fn runtime_view_function(&self) -> runtime_view_function::RuntimeViewFunction {
                runtime_view_function::RuntimeViewFunction
            }
            pub fn block_builder(&self) -> block_builder::BlockBuilder {
                block_builder::BlockBuilder
            }
            pub fn tagged_transaction_queue(
                &self,
            ) -> tagged_transaction_queue::TaggedTransactionQueue {
                tagged_transaction_queue::TaggedTransactionQueue
            }
            pub fn offchain_worker_api(&self) -> offchain_worker_api::OffchainWorkerApi {
                offchain_worker_api::OffchainWorkerApi
            }
            pub fn aura_api(&self) -> aura_api::AuraApi {
                aura_api::AuraApi
            }
            pub fn session_keys(&self) -> session_keys::SessionKeys {
                session_keys::SessionKeys
            }
            pub fn grandpa_api(&self) -> grandpa_api::GrandpaApi {
                grandpa_api::GrandpaApi
            }
            pub fn account_nonce_api(&self) -> account_nonce_api::AccountNonceApi {
                account_nonce_api::AccountNonceApi
            }
            pub fn transaction_payment_api(
                &self,
            ) -> transaction_payment_api::TransactionPaymentApi {
                transaction_payment_api::TransactionPaymentApi
            }
            pub fn transaction_payment_call_api(
                &self,
            ) -> transaction_payment_call_api::TransactionPaymentCallApi {
                transaction_payment_call_api::TransactionPaymentCallApi
            }
            pub fn genesis_builder(&self) -> genesis_builder::GenesisBuilder {
                genesis_builder::GenesisBuilder
            }
        }
        pub mod core {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `Core` runtime api that every Substrate runtime needs to implement."]
            pub struct Core;
            impl Core {
                #[doc = " Returns the version of the runtime."]
                pub fn version(
                    &self,
                ) -> ::subxt::runtime_apis::StaticPayload<(), version::output::Output>
                {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "Core",
                        "version",
                        (),
                        [
                            79u8, 22u8, 137u8, 4u8, 40u8, 64u8, 30u8, 180u8, 49u8, 222u8, 114u8,
                            125u8, 44u8, 25u8, 33u8, 152u8, 98u8, 42u8, 72u8, 178u8, 240u8, 103u8,
                            34u8, 187u8, 81u8, 161u8, 183u8, 6u8, 120u8, 2u8, 146u8, 0u8,
                        ],
                    )
                }
                #[doc = " Execute the given block."]
                pub fn execute_block(
                    &self,
                    block: execute_block::Block,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (execute_block::Block,),
                    execute_block::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "Core",
                        "execute_block",
                        (block,),
                        [
                            133u8, 135u8, 228u8, 65u8, 106u8, 27u8, 85u8, 158u8, 112u8, 254u8,
                            93u8, 26u8, 102u8, 201u8, 118u8, 216u8, 249u8, 247u8, 91u8, 74u8, 56u8,
                            208u8, 231u8, 115u8, 131u8, 29u8, 209u8, 6u8, 65u8, 57u8, 214u8, 125u8,
                        ],
                    )
                }
                #[doc = " Initialize a block with the given header and return the runtime executive mode."]
                pub fn initialize_block(
                    &self,
                    header: initialize_block::Header,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (initialize_block::Header,),
                    initialize_block::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "Core",
                        "initialize_block",
                        (header,),
                        [
                            132u8, 169u8, 113u8, 112u8, 80u8, 139u8, 113u8, 35u8, 41u8, 81u8, 36u8,
                            35u8, 37u8, 202u8, 29u8, 207u8, 205u8, 229u8, 145u8, 7u8, 133u8, 94u8,
                            25u8, 108u8, 233u8, 86u8, 234u8, 29u8, 236u8, 57u8, 56u8, 186u8,
                        ],
                    )
                }
            }
            pub mod version {
                use super::root_mod;
                use super::runtime_types;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = runtime_types::sp_version::RuntimeVersion;
                }
            }
            pub mod execute_block {
                use super::root_mod;
                use super::runtime_types;
                pub type Block = runtime_types :: sp_runtime :: generic :: block :: LazyBlock < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: solochain_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: frame_metadata_hash_extension :: CheckMetadataHash , runtime_types :: frame_system :: extensions :: weight_reclaim :: WeightReclaim ,) > > ;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ();
                }
            }
            pub mod initialize_block {
                use super::root_mod;
                use super::runtime_types;
                pub type Header =
                    runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = runtime_types::sp_runtime::ExtrinsicInclusionMode;
                }
            }
        }
        pub mod metadata {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `Metadata` api trait that returns metadata for the runtime."]
            pub struct Metadata;
            impl Metadata {
                #[doc = " Returns the metadata of a runtime."]
                pub fn metadata(
                    &self,
                ) -> ::subxt::runtime_apis::StaticPayload<(), metadata::output::Output>
                {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "Metadata",
                        "metadata",
                        (),
                        [
                            231u8, 24u8, 67u8, 152u8, 23u8, 26u8, 188u8, 82u8, 229u8, 6u8, 185u8,
                            27u8, 175u8, 68u8, 83u8, 122u8, 69u8, 89u8, 185u8, 74u8, 248u8, 87u8,
                            217u8, 124u8, 193u8, 252u8, 199u8, 186u8, 196u8, 179u8, 179u8, 96u8,
                        ],
                    )
                }
                #[doc = " Returns the metadata at a given version."]
                #[doc = ""]
                #[doc = " If the given `version` isn't supported, this will return `None`."]
                #[doc = " Use [`Self::metadata_versions`] to find out about supported metadata version of the runtime."]
                pub fn metadata_at_version(
                    &self,
                    version: metadata_at_version::Version,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (metadata_at_version::Version,),
                    metadata_at_version::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "Metadata",
                        "metadata_at_version",
                        (version,),
                        [
                            131u8, 53u8, 212u8, 234u8, 16u8, 25u8, 120u8, 252u8, 153u8, 153u8,
                            216u8, 28u8, 54u8, 113u8, 52u8, 236u8, 146u8, 68u8, 142u8, 8u8, 10u8,
                            169u8, 131u8, 142u8, 204u8, 38u8, 48u8, 108u8, 134u8, 86u8, 226u8,
                            61u8,
                        ],
                    )
                }
                #[doc = " Returns the supported metadata versions."]
                #[doc = ""]
                #[doc = " This can be used to call `metadata_at_version`."]
                pub fn metadata_versions(
                    &self,
                ) -> ::subxt::runtime_apis::StaticPayload<(), metadata_versions::output::Output>
                {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "Metadata",
                        "metadata_versions",
                        (),
                        [
                            23u8, 144u8, 137u8, 91u8, 188u8, 39u8, 231u8, 208u8, 252u8, 218u8,
                            224u8, 176u8, 77u8, 32u8, 130u8, 212u8, 223u8, 76u8, 100u8, 190u8,
                            82u8, 94u8, 190u8, 8u8, 82u8, 244u8, 225u8, 179u8, 85u8, 176u8, 56u8,
                            16u8,
                        ],
                    )
                }
            }
            pub mod metadata {
                use super::root_mod;
                use super::runtime_types;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = runtime_types::sp_core::OpaqueMetadata;
                }
            }
            pub mod metadata_at_version {
                use super::root_mod;
                use super::runtime_types;
                pub type Version = ::core::primitive::u32;
                pub mod output {
                    use super::runtime_types;
                    pub type Output =
                        ::core::option::Option<runtime_types::sp_core::OpaqueMetadata>;
                }
            }
            pub mod metadata_versions {
                use super::root_mod;
                use super::runtime_types;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::subxt::alloc::vec::Vec<::core::primitive::u32>;
                }
            }
        }
        pub mod runtime_view_function {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " Runtime API for executing view functions"]
            pub struct RuntimeViewFunction;
            impl RuntimeViewFunction {
                #[doc = " Execute a view function query."]
                pub fn execute_view_function(
                    &self,
                    query_id: execute_view_function::QueryId,
                    input: execute_view_function::Input,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (execute_view_function::QueryId, execute_view_function::Input),
                    execute_view_function::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "RuntimeViewFunction",
                        "execute_view_function",
                        (query_id, input),
                        [
                            104u8, 132u8, 108u8, 136u8, 93u8, 251u8, 187u8, 218u8, 108u8, 238u8,
                            87u8, 89u8, 86u8, 176u8, 142u8, 19u8, 211u8, 249u8, 75u8, 165u8, 142u8,
                            174u8, 163u8, 245u8, 67u8, 57u8, 189u8, 72u8, 15u8, 232u8, 57u8, 81u8,
                        ],
                    )
                }
            }
            pub mod execute_view_function {
                use super::root_mod;
                use super::runtime_types;
                pub type QueryId = runtime_types::frame_support::view_functions::ViewFunctionId;
                pub type Input = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::result::Result<
                        ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                        runtime_types::frame_support::view_functions::ViewFunctionDispatchError,
                    >;
                }
            }
        }
        pub mod block_builder {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `BlockBuilder` api trait that provides the required functionality for building a block."]
            pub struct BlockBuilder;
            impl BlockBuilder {
                #[doc = " Apply the given extrinsic."]
                #[doc = ""]
                #[doc = " Returns an inclusion outcome which specifies if this extrinsic is included in"]
                #[doc = " this block or not."]
                pub fn apply_extrinsic(
                    &self,
                    extrinsic: apply_extrinsic::Extrinsic,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (apply_extrinsic::Extrinsic,),
                    apply_extrinsic::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "BlockBuilder",
                        "apply_extrinsic",
                        (extrinsic,),
                        [
                            192u8, 184u8, 199u8, 4u8, 85u8, 136u8, 214u8, 205u8, 29u8, 29u8, 98u8,
                            145u8, 172u8, 92u8, 168u8, 161u8, 150u8, 133u8, 100u8, 243u8, 100u8,
                            100u8, 118u8, 28u8, 104u8, 82u8, 93u8, 63u8, 79u8, 36u8, 149u8, 144u8,
                        ],
                    )
                }
                #[doc = " Finish the current block."]
                pub fn finalize_block(
                    &self,
                ) -> ::subxt::runtime_apis::StaticPayload<(), finalize_block::output::Output>
                {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "BlockBuilder",
                        "finalize_block",
                        (),
                        [
                            244u8, 207u8, 24u8, 33u8, 13u8, 69u8, 9u8, 249u8, 145u8, 143u8, 122u8,
                            96u8, 197u8, 55u8, 64u8, 111u8, 238u8, 224u8, 34u8, 201u8, 27u8, 146u8,
                            232u8, 99u8, 191u8, 30u8, 114u8, 16u8, 32u8, 220u8, 58u8, 62u8,
                        ],
                    )
                }
                #[doc = " Generate inherent extrinsics. The inherent data will vary from chain to chain."]
                pub fn inherent_extrinsics(
                    &self,
                    inherent: inherent_extrinsics::Inherent,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (inherent_extrinsics::Inherent,),
                    inherent_extrinsics::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "BlockBuilder",
                        "inherent_extrinsics",
                        (inherent,),
                        [
                            254u8, 110u8, 245u8, 201u8, 250u8, 192u8, 27u8, 228u8, 151u8, 213u8,
                            166u8, 89u8, 94u8, 81u8, 189u8, 234u8, 64u8, 18u8, 245u8, 80u8, 29u8,
                            18u8, 140u8, 129u8, 113u8, 236u8, 135u8, 55u8, 79u8, 159u8, 175u8,
                            183u8,
                        ],
                    )
                }
                #[doc = " Check that the inherents are valid. The inherent data will vary from chain to chain."]
                pub fn check_inherents(
                    &self,
                    block: check_inherents::Block,
                    data: check_inherents::Data,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (check_inherents::Block, check_inherents::Data),
                    check_inherents::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "BlockBuilder",
                        "check_inherents",
                        (block, data),
                        [
                            153u8, 134u8, 1u8, 215u8, 139u8, 11u8, 53u8, 51u8, 210u8, 175u8, 197u8,
                            28u8, 38u8, 209u8, 175u8, 247u8, 142u8, 157u8, 50u8, 151u8, 164u8,
                            191u8, 181u8, 118u8, 80u8, 97u8, 160u8, 248u8, 110u8, 217u8, 181u8,
                            234u8,
                        ],
                    )
                }
            }
            pub mod apply_extrinsic {
                use super::root_mod;
                use super::runtime_types;
                pub type Extrinsic = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: solochain_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: frame_metadata_hash_extension :: CheckMetadataHash , runtime_types :: frame_system :: extensions :: weight_reclaim :: WeightReclaim ,) > ;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::result::Result<
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                        runtime_types::sp_runtime::transaction_validity::TransactionValidityError,
                    >;
                }
            }
            pub mod finalize_block {
                use super::root_mod;
                use super::runtime_types;
                pub mod output {
                    use super::runtime_types;
                    pub type Output =
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                }
            }
            pub mod inherent_extrinsics {
                use super::root_mod;
                use super::runtime_types;
                pub type Inherent = runtime_types::sp_inherents::InherentData;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = :: subxt :: alloc :: vec :: Vec < :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: solochain_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: frame_metadata_hash_extension :: CheckMetadataHash , runtime_types :: frame_system :: extensions :: weight_reclaim :: WeightReclaim ,) > > ;
                }
            }
            pub mod check_inherents {
                use super::root_mod;
                use super::runtime_types;
                pub type Block = runtime_types :: sp_runtime :: generic :: block :: LazyBlock < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: solochain_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: frame_metadata_hash_extension :: CheckMetadataHash , runtime_types :: frame_system :: extensions :: weight_reclaim :: WeightReclaim ,) > > ;
                pub type Data = runtime_types::sp_inherents::InherentData;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = runtime_types::sp_inherents::CheckInherentsResult;
                }
            }
        }
        pub mod tagged_transaction_queue {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `TaggedTransactionQueue` api trait for interfering with the transaction queue."]
            pub struct TaggedTransactionQueue;
            impl TaggedTransactionQueue {
                #[doc = " Validate the transaction."]
                #[doc = ""]
                #[doc = " This method is invoked by the transaction pool to learn details about given transaction."]
                #[doc = " The implementation should make sure to verify the correctness of the transaction"]
                #[doc = " against current state. The given `block_hash` corresponds to the hash of the block"]
                #[doc = " that is used as current state."]
                #[doc = ""]
                #[doc = " Note that this call may be performed by the pool multiple times and transactions"]
                #[doc = " might be verified in any possible order."]
                pub fn validate_transaction(
                    &self,
                    source: validate_transaction::Source,
                    tx: validate_transaction::Tx,
                    block_hash: validate_transaction::BlockHash,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (
                        validate_transaction::Source,
                        validate_transaction::Tx,
                        validate_transaction::BlockHash,
                    ),
                    validate_transaction::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "TaggedTransactionQueue",
                        "validate_transaction",
                        (source, tx, block_hash),
                        [
                            19u8, 53u8, 170u8, 115u8, 75u8, 121u8, 231u8, 50u8, 199u8, 181u8,
                            243u8, 170u8, 163u8, 224u8, 213u8, 134u8, 206u8, 207u8, 88u8, 242u8,
                            80u8, 139u8, 233u8, 87u8, 175u8, 249u8, 178u8, 169u8, 255u8, 171u8,
                            4u8, 125u8,
                        ],
                    )
                }
            }
            pub mod validate_transaction {
                use super::root_mod;
                use super::runtime_types;
                pub type Source =
                    runtime_types::sp_runtime::transaction_validity::TransactionSource;
                pub type Tx = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: solochain_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: frame_metadata_hash_extension :: CheckMetadataHash , runtime_types :: frame_system :: extensions :: weight_reclaim :: WeightReclaim ,) > ;
                pub type BlockHash = ::subxt::utils::H256;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::result::Result<
                        runtime_types::sp_runtime::transaction_validity::ValidTransaction,
                        runtime_types::sp_runtime::transaction_validity::TransactionValidityError,
                    >;
                }
            }
        }
        pub mod offchain_worker_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The offchain worker api."]
            pub struct OffchainWorkerApi;
            impl OffchainWorkerApi {
                #[doc = " Starts the off-chain task for given block header."]
                pub fn offchain_worker(
                    &self,
                    header: offchain_worker::Header,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (offchain_worker::Header,),
                    offchain_worker::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "OffchainWorkerApi",
                        "offchain_worker",
                        (header,),
                        [
                            10u8, 135u8, 19u8, 153u8, 33u8, 216u8, 18u8, 242u8, 33u8, 140u8, 4u8,
                            223u8, 200u8, 130u8, 103u8, 118u8, 137u8, 24u8, 19u8, 127u8, 161u8,
                            29u8, 184u8, 111u8, 222u8, 111u8, 253u8, 73u8, 45u8, 31u8, 79u8, 60u8,
                        ],
                    )
                }
            }
            pub mod offchain_worker {
                use super::root_mod;
                use super::runtime_types;
                pub type Header =
                    runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ();
                }
            }
        }
        pub mod aura_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " API necessary for block authorship with aura."]
            pub struct AuraApi;
            impl AuraApi {
                #[doc = " Returns the slot duration for Aura."]
                #[doc = ""]
                #[doc = " Currently, only the value provided by this type at genesis will be used."]
                pub fn slot_duration(
                    &self,
                ) -> ::subxt::runtime_apis::StaticPayload<(), slot_duration::output::Output>
                {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "AuraApi",
                        "slot_duration",
                        (),
                        [
                            233u8, 210u8, 132u8, 172u8, 100u8, 125u8, 239u8, 92u8, 114u8, 82u8,
                            7u8, 110u8, 179u8, 196u8, 10u8, 19u8, 211u8, 15u8, 174u8, 2u8, 91u8,
                            73u8, 133u8, 100u8, 205u8, 201u8, 191u8, 60u8, 163u8, 122u8, 215u8,
                            10u8,
                        ],
                    )
                }
                #[doc = " Return the current set of authorities."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::runtime_apis::StaticPayload<(), authorities::output::Output>
                {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "AuraApi",
                        "authorities",
                        (),
                        [
                            35u8, 244u8, 24u8, 155u8, 95u8, 1u8, 221u8, 159u8, 33u8, 144u8, 213u8,
                            26u8, 13u8, 21u8, 136u8, 72u8, 45u8, 47u8, 15u8, 51u8, 235u8, 10u8,
                            6u8, 219u8, 9u8, 246u8, 50u8, 252u8, 49u8, 77u8, 64u8, 182u8,
                        ],
                    )
                }
            }
            pub mod slot_duration {
                use super::root_mod;
                use super::runtime_types;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = runtime_types::sp_consensus_slots::SlotDuration;
                }
            }
            pub mod authorities {
                use super::root_mod;
                use super::runtime_types;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::subxt::alloc::vec::Vec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >;
                }
            }
        }
        pub mod session_keys {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " Session keys runtime api."]
            pub struct SessionKeys;
            impl SessionKeys {
                #[doc = " Generate a set of session keys with optionally using the given seed."]
                #[doc = " The keys should be stored within the keystore exposed via runtime"]
                #[doc = " externalities."]
                #[doc = ""]
                #[doc = " The seed needs to be a valid `utf8` string."]
                #[doc = ""]
                #[doc = " Returns the concatenated SCALE encoded public keys."]
                pub fn generate_session_keys(
                    &self,
                    owner: generate_session_keys::Owner,
                    seed: generate_session_keys::Seed,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (generate_session_keys::Owner, generate_session_keys::Seed),
                    generate_session_keys::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "SessionKeys",
                        "generate_session_keys",
                        (owner, seed),
                        [
                            94u8, 230u8, 217u8, 119u8, 217u8, 37u8, 67u8, 190u8, 118u8, 204u8,
                            72u8, 95u8, 58u8, 138u8, 153u8, 164u8, 95u8, 31u8, 85u8, 83u8, 199u8,
                            12u8, 119u8, 135u8, 248u8, 96u8, 85u8, 142u8, 84u8, 238u8, 111u8,
                            254u8,
                        ],
                    )
                }
                #[doc = " Decode the given public session keys."]
                #[doc = ""]
                #[doc = " Returns the list of public raw public keys + key type."]
                pub fn decode_session_keys(
                    &self,
                    encoded: decode_session_keys::Encoded,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (decode_session_keys::Encoded,),
                    decode_session_keys::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "SessionKeys",
                        "decode_session_keys",
                        (encoded,),
                        [
                            57u8, 242u8, 18u8, 51u8, 132u8, 110u8, 238u8, 255u8, 39u8, 194u8, 8u8,
                            54u8, 198u8, 178u8, 75u8, 151u8, 148u8, 176u8, 144u8, 197u8, 87u8,
                            29u8, 179u8, 235u8, 176u8, 78u8, 252u8, 103u8, 72u8, 203u8, 151u8,
                            248u8,
                        ],
                    )
                }
            }
            pub mod generate_session_keys {
                use super::root_mod;
                use super::runtime_types;
                pub type Owner = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
                pub type Seed =
                    ::core::option::Option<::subxt::alloc::vec::Vec<::core::primitive::u8>>;
                pub mod output {
                    use super::runtime_types;
                    pub type Output =
                        runtime_types::sp_session::runtime_api::OpaqueGeneratedSessionKeys;
                }
            }
            pub mod decode_session_keys {
                use super::root_mod;
                use super::runtime_types;
                pub type Encoded = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::option::Option<
                        ::subxt::alloc::vec::Vec<(
                            ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                            runtime_types::sp_core::crypto::KeyTypeId,
                        )>,
                    >;
                }
            }
        }
        pub mod grandpa_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " APIs for integrating the GRANDPA finality gadget into runtimes."]
            #[doc = " This should be implemented on the runtime side."]
            #[doc = ""]
            #[doc = " This is primarily used for negotiating authority-set changes for the"]
            #[doc = " gadget. GRANDPA uses a signaling model of changing authority sets:"]
            #[doc = " changes should be signaled with a delay of N blocks, and then automatically"]
            #[doc = " applied in the runtime after those N blocks have passed."]
            #[doc = ""]
            #[doc = " The consensus protocol will coordinate the handoff externally."]
            pub struct GrandpaApi;
            impl GrandpaApi {
                #[doc = " Get the current GRANDPA authorities and weights. This should not change except"]
                #[doc = " for when changes are scheduled and the corresponding delay has passed."]
                #[doc = ""]
                #[doc = " When called at block B, it will return the set of authorities that should be"]
                #[doc = " used to finalize descendants of this block (B+1, B+2, ...). The block B itself"]
                #[doc = " is finalized by the authorities from block B-1."]
                pub fn grandpa_authorities(
                    &self,
                ) -> ::subxt::runtime_apis::StaticPayload<(), grandpa_authorities::output::Output>
                {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "GrandpaApi",
                        "grandpa_authorities",
                        (),
                        [
                            8u8, 1u8, 99u8, 227u8, 52u8, 95u8, 230u8, 139u8, 198u8, 90u8, 159u8,
                            146u8, 193u8, 81u8, 37u8, 27u8, 216u8, 227u8, 108u8, 126u8, 12u8, 94u8,
                            125u8, 183u8, 143u8, 231u8, 87u8, 101u8, 114u8, 190u8, 193u8, 180u8,
                        ],
                    )
                }
                #[doc = " Submits an unsigned extrinsic to report an equivocation. The caller"]
                #[doc = " must provide the equivocation proof and a key ownership proof"]
                #[doc = " (should be obtained using `generate_key_ownership_proof`). The"]
                #[doc = " extrinsic will be unsigned and should only be accepted for local"]
                #[doc = " authorship (not to be broadcast to the network). This method returns"]
                #[doc = " `None` when creation of the extrinsic fails, e.g. if equivocation"]
                #[doc = " reporting is disabled for the given runtime (i.e. this method is"]
                #[doc = " hardcoded to return `None`). Only useful in an offchain context."]
                pub fn submit_report_equivocation_unsigned_extrinsic(
                    &self,
                    equivocation_proof : submit_report_equivocation_unsigned_extrinsic :: EquivocationProof,
                    key_owner_proof: submit_report_equivocation_unsigned_extrinsic::KeyOwnerProof,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (
                        submit_report_equivocation_unsigned_extrinsic::EquivocationProof,
                        submit_report_equivocation_unsigned_extrinsic::KeyOwnerProof,
                    ),
                    submit_report_equivocation_unsigned_extrinsic::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "GrandpaApi",
                        "submit_report_equivocation_unsigned_extrinsic",
                        (equivocation_proof, key_owner_proof),
                        [
                            27u8, 32u8, 16u8, 79u8, 172u8, 124u8, 44u8, 13u8, 176u8, 89u8, 69u8,
                            60u8, 45u8, 176u8, 72u8, 151u8, 252u8, 5u8, 243u8, 82u8, 170u8, 51u8,
                            179u8, 197u8, 117u8, 177u8, 110u8, 111u8, 97u8, 15u8, 109u8, 169u8,
                        ],
                    )
                }
                #[doc = " Generates a proof of key ownership for the given authority in the"]
                #[doc = " given set. An example usage of this module is coupled with the"]
                #[doc = " session historical module to prove that a given authority key is"]
                #[doc = " tied to a given staking identity during a specific session. Proofs"]
                #[doc = " of key ownership are necessary for submitting equivocation reports."]
                #[doc = " NOTE: even though the API takes a `set_id` as parameter the current"]
                #[doc = " implementations ignore this parameter and instead rely on this"]
                #[doc = " method being called at the correct block height, i.e. any point at"]
                #[doc = " which the given set id is live on-chain. Future implementations will"]
                #[doc = " instead use indexed data through an offchain worker, not requiring"]
                #[doc = " older states to be available."]
                pub fn generate_key_ownership_proof(
                    &self,
                    set_id: generate_key_ownership_proof::SetId,
                    authority_id: generate_key_ownership_proof::AuthorityId,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (
                        generate_key_ownership_proof::SetId,
                        generate_key_ownership_proof::AuthorityId,
                    ),
                    generate_key_ownership_proof::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "GrandpaApi",
                        "generate_key_ownership_proof",
                        (set_id, authority_id),
                        [
                            13u8, 144u8, 66u8, 235u8, 24u8, 190u8, 39u8, 75u8, 29u8, 157u8, 215u8,
                            181u8, 173u8, 145u8, 224u8, 244u8, 189u8, 79u8, 6u8, 116u8, 139u8,
                            196u8, 54u8, 16u8, 89u8, 190u8, 121u8, 43u8, 137u8, 150u8, 117u8, 68u8,
                        ],
                    )
                }
                #[doc = " Get current GRANDPA authority set id."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::runtime_apis::StaticPayload<(), current_set_id::output::Output>
                {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "GrandpaApi",
                        "current_set_id",
                        (),
                        [
                            42u8, 230u8, 120u8, 211u8, 156u8, 245u8, 109u8, 86u8, 100u8, 146u8,
                            234u8, 205u8, 41u8, 183u8, 109u8, 42u8, 17u8, 33u8, 156u8, 25u8, 139u8,
                            84u8, 101u8, 75u8, 232u8, 198u8, 87u8, 136u8, 218u8, 233u8, 103u8,
                            156u8,
                        ],
                    )
                }
            }
            pub mod grandpa_authorities {
                use super::root_mod;
                use super::runtime_types;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::subxt::alloc::vec::Vec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>;
                }
            }
            pub mod submit_report_equivocation_unsigned_extrinsic {
                use super::root_mod;
                use super::runtime_types;
                pub type EquivocationProof = runtime_types::sp_consensus_grandpa::EquivocationProof<
                    ::subxt::utils::H256,
                    ::core::primitive::u32,
                >;
                pub type KeyOwnerProof = runtime_types::sp_runtime::OpaqueValue;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::option::Option<()>;
                }
            }
            pub mod generate_key_ownership_proof {
                use super::root_mod;
                use super::runtime_types;
                pub type SetId = ::core::primitive::u64;
                pub type AuthorityId = runtime_types::sp_consensus_grandpa::app::Public;
                pub mod output {
                    use super::runtime_types;
                    pub type Output =
                        ::core::option::Option<runtime_types::sp_runtime::OpaqueValue>;
                }
            }
            pub mod current_set_id {
                use super::root_mod;
                use super::runtime_types;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::primitive::u64;
                }
            }
        }
        pub mod account_nonce_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The API to query account nonce."]
            pub struct AccountNonceApi;
            impl AccountNonceApi {
                #[doc = " Get current account nonce of given `AccountId`."]
                pub fn account_nonce(
                    &self,
                    account: account_nonce::Account,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (account_nonce::Account,),
                    account_nonce::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "AccountNonceApi",
                        "account_nonce",
                        (account,),
                        [
                            231u8, 82u8, 7u8, 227u8, 131u8, 2u8, 215u8, 252u8, 173u8, 82u8, 11u8,
                            103u8, 200u8, 25u8, 114u8, 116u8, 79u8, 229u8, 152u8, 150u8, 236u8,
                            37u8, 101u8, 26u8, 220u8, 146u8, 182u8, 101u8, 73u8, 55u8, 191u8,
                            171u8,
                        ],
                    )
                }
            }
            pub mod account_nonce {
                use super::root_mod;
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::primitive::u32;
                }
            }
        }
        pub mod transaction_payment_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentApi;
            impl TransactionPaymentApi {
                pub fn query_info(
                    &self,
                    uxt: query_info::Uxt,
                    len: query_info::Len,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (query_info::Uxt, query_info::Len),
                    query_info::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_info",
                        (uxt, len),
                        [
                            56u8, 30u8, 174u8, 34u8, 202u8, 24u8, 177u8, 189u8, 145u8, 36u8, 1u8,
                            156u8, 98u8, 209u8, 178u8, 49u8, 198u8, 23u8, 150u8, 173u8, 35u8,
                            205u8, 147u8, 129u8, 42u8, 22u8, 69u8, 3u8, 129u8, 8u8, 196u8, 139u8,
                        ],
                    )
                }
                pub fn query_fee_details(
                    &self,
                    uxt: query_fee_details::Uxt,
                    len: query_fee_details::Len,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (query_fee_details::Uxt, query_fee_details::Len),
                    query_fee_details::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_fee_details",
                        (uxt, len),
                        [
                            117u8, 60u8, 137u8, 159u8, 237u8, 252u8, 216u8, 238u8, 232u8, 1u8,
                            100u8, 152u8, 26u8, 185u8, 145u8, 125u8, 68u8, 189u8, 4u8, 30u8, 125u8,
                            7u8, 196u8, 153u8, 235u8, 51u8, 219u8, 108u8, 185u8, 254u8, 100u8,
                            201u8,
                        ],
                    )
                }
                pub fn query_weight_to_fee(
                    &self,
                    weight: query_weight_to_fee::Weight,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (query_weight_to_fee::Weight,),
                    query_weight_to_fee::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_weight_to_fee",
                        (weight,),
                        [
                            206u8, 243u8, 189u8, 83u8, 231u8, 244u8, 247u8, 52u8, 126u8, 208u8,
                            224u8, 5u8, 163u8, 108u8, 254u8, 114u8, 214u8, 156u8, 227u8, 217u8,
                            211u8, 198u8, 121u8, 164u8, 110u8, 54u8, 181u8, 146u8, 50u8, 146u8,
                            146u8, 23u8,
                        ],
                    )
                }
                pub fn query_length_to_fee(
                    &self,
                    length: query_length_to_fee::Length,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (query_length_to_fee::Length,),
                    query_length_to_fee::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_length_to_fee",
                        (length,),
                        [
                            92u8, 132u8, 29u8, 119u8, 66u8, 11u8, 196u8, 224u8, 129u8, 23u8, 249u8,
                            12u8, 32u8, 28u8, 92u8, 50u8, 188u8, 101u8, 203u8, 229u8, 248u8, 216u8,
                            130u8, 150u8, 212u8, 161u8, 81u8, 254u8, 116u8, 89u8, 162u8, 48u8,
                        ],
                    )
                }
            }
            pub mod query_info {
                use super::root_mod;
                use super::runtime_types;
                pub type Uxt = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: solochain_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: frame_metadata_hash_extension :: CheckMetadataHash , runtime_types :: frame_system :: extensions :: weight_reclaim :: WeightReclaim ,) > ;
                pub type Len = ::core::primitive::u32;
                pub mod output {
                    use super::runtime_types;
                    pub type Output =
                        runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                            ::core::primitive::u128,
                            runtime_types::sp_weights::weight_v2::Weight,
                        >;
                }
            }
            pub mod query_fee_details {
                use super::root_mod;
                use super::runtime_types;
                pub type Uxt = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: solochain_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: frame_metadata_hash_extension :: CheckMetadataHash , runtime_types :: frame_system :: extensions :: weight_reclaim :: WeightReclaim ,) > ;
                pub type Len = ::core::primitive::u32;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = runtime_types::pallet_transaction_payment::types::FeeDetails<
                        ::core::primitive::u128,
                    >;
                }
            }
            pub mod query_weight_to_fee {
                use super::root_mod;
                use super::runtime_types;
                pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::primitive::u128;
                }
            }
            pub mod query_length_to_fee {
                use super::root_mod;
                use super::runtime_types;
                pub type Length = ::core::primitive::u32;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::primitive::u128;
                }
            }
        }
        pub mod transaction_payment_call_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentCallApi;
            impl TransactionPaymentCallApi {
                #[doc = " Query information of a dispatch class, weight, and fee of a given encoded `Call`."]
                pub fn query_call_info(
                    &self,
                    call: query_call_info::Call,
                    len: query_call_info::Len,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (query_call_info::Call, query_call_info::Len),
                    query_call_info::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "TransactionPaymentCallApi",
                        "query_call_info",
                        (call, len),
                        [
                            79u8, 137u8, 174u8, 43u8, 94u8, 68u8, 167u8, 53u8, 52u8, 47u8, 44u8,
                            70u8, 199u8, 116u8, 49u8, 140u8, 78u8, 51u8, 214u8, 81u8, 140u8, 167u8,
                            79u8, 104u8, 152u8, 122u8, 59u8, 13u8, 105u8, 169u8, 231u8, 171u8,
                        ],
                    )
                }
                #[doc = " Query fee details of a given encoded `Call`."]
                pub fn query_call_fee_details(
                    &self,
                    call: query_call_fee_details::Call,
                    len: query_call_fee_details::Len,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (query_call_fee_details::Call, query_call_fee_details::Len),
                    query_call_fee_details::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "TransactionPaymentCallApi",
                        "query_call_fee_details",
                        (call, len),
                        [
                            172u8, 119u8, 146u8, 223u8, 105u8, 34u8, 213u8, 121u8, 87u8, 231u8,
                            56u8, 218u8, 131u8, 55u8, 44u8, 84u8, 196u8, 119u8, 19u8, 45u8, 34u8,
                            144u8, 173u8, 14u8, 149u8, 45u8, 147u8, 43u8, 63u8, 232u8, 108u8, 91u8,
                        ],
                    )
                }
                #[doc = " Query the output of the current `WeightToFee` given some input."]
                pub fn query_weight_to_fee(
                    &self,
                    weight: query_weight_to_fee::Weight,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (query_weight_to_fee::Weight,),
                    query_weight_to_fee::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "TransactionPaymentCallApi",
                        "query_weight_to_fee",
                        (weight,),
                        [
                            117u8, 91u8, 94u8, 22u8, 248u8, 212u8, 15u8, 23u8, 97u8, 116u8, 64u8,
                            228u8, 83u8, 123u8, 87u8, 77u8, 97u8, 7u8, 98u8, 181u8, 6u8, 165u8,
                            114u8, 141u8, 164u8, 113u8, 126u8, 88u8, 174u8, 171u8, 224u8, 35u8,
                        ],
                    )
                }
                #[doc = " Query the output of the current `LengthToFee` given some input."]
                pub fn query_length_to_fee(
                    &self,
                    length: query_length_to_fee::Length,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (query_length_to_fee::Length,),
                    query_length_to_fee::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "TransactionPaymentCallApi",
                        "query_length_to_fee",
                        (length,),
                        [
                            246u8, 40u8, 4u8, 160u8, 152u8, 94u8, 170u8, 53u8, 205u8, 122u8, 5u8,
                            69u8, 70u8, 25u8, 128u8, 156u8, 119u8, 134u8, 116u8, 147u8, 14u8,
                            164u8, 65u8, 140u8, 86u8, 13u8, 250u8, 218u8, 89u8, 95u8, 234u8, 228u8,
                        ],
                    )
                }
            }
            pub mod query_call_info {
                use super::root_mod;
                use super::runtime_types;
                pub type Call = runtime_types::solochain_template_runtime::RuntimeCall;
                pub type Len = ::core::primitive::u32;
                pub mod output {
                    use super::runtime_types;
                    pub type Output =
                        runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                            ::core::primitive::u128,
                            runtime_types::sp_weights::weight_v2::Weight,
                        >;
                }
            }
            pub mod query_call_fee_details {
                use super::root_mod;
                use super::runtime_types;
                pub type Call = runtime_types::solochain_template_runtime::RuntimeCall;
                pub type Len = ::core::primitive::u32;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = runtime_types::pallet_transaction_payment::types::FeeDetails<
                        ::core::primitive::u128,
                    >;
                }
            }
            pub mod query_weight_to_fee {
                use super::root_mod;
                use super::runtime_types;
                pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::primitive::u128;
                }
            }
            pub mod query_length_to_fee {
                use super::root_mod;
                use super::runtime_types;
                pub type Length = ::core::primitive::u32;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::primitive::u128;
                }
            }
        }
        pub mod genesis_builder {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " API to interact with `RuntimeGenesisConfig` for the runtime"]
            pub struct GenesisBuilder;
            impl GenesisBuilder {
                #[doc = " Build `RuntimeGenesisConfig` from a JSON blob not using any defaults and store it in the"]
                #[doc = " storage."]
                #[doc = ""]
                #[doc = " In the case of a FRAME-based runtime, this function deserializes the full"]
                #[doc = " `RuntimeGenesisConfig` from the given JSON blob and puts it into the storage. If the"]
                #[doc = " provided JSON blob is incorrect or incomplete or the deserialization fails, an error"]
                #[doc = " is returned."]
                #[doc = ""]
                #[doc = " Please note that provided JSON blob must contain all `RuntimeGenesisConfig` fields, no"]
                #[doc = " defaults will be used."]
                pub fn build_state(
                    &self,
                    json: build_state::Json,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (build_state::Json,),
                    build_state::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "GenesisBuilder",
                        "build_state",
                        (json,),
                        [
                            203u8, 233u8, 104u8, 116u8, 111u8, 131u8, 201u8, 235u8, 117u8, 116u8,
                            140u8, 185u8, 93u8, 25u8, 155u8, 210u8, 56u8, 49u8, 23u8, 32u8, 253u8,
                            92u8, 149u8, 241u8, 85u8, 245u8, 137u8, 45u8, 209u8, 189u8, 81u8, 2u8,
                        ],
                    )
                }
                #[doc = " Returns a JSON blob representation of the built-in `RuntimeGenesisConfig` identified by"]
                #[doc = " `id`."]
                #[doc = ""]
                #[doc = " If `id` is `None` the function should return JSON blob representation of the default"]
                #[doc = " `RuntimeGenesisConfig` struct of the runtime. Implementation must provide default"]
                #[doc = " `RuntimeGenesisConfig`."]
                #[doc = ""]
                #[doc = " Otherwise function returns a JSON representation of the built-in, named"]
                #[doc = " `RuntimeGenesisConfig` preset identified by `id`, or `None` if such preset does not"]
                #[doc = " exist. Returned `Vec<u8>` contains bytes of JSON blob (patch) which comprises a list of"]
                #[doc = " (potentially nested) key-value pairs that are intended for customizing the default"]
                #[doc = " runtime genesis config. The patch shall be merged (rfc7386) with the JSON representation"]
                #[doc = " of the default `RuntimeGenesisConfig` to create a comprehensive genesis config that can"]
                #[doc = " be used in `build_state` method."]
                pub fn get_preset(
                    &self,
                    id: get_preset::Id,
                ) -> ::subxt::runtime_apis::StaticPayload<
                    (get_preset::Id,),
                    get_preset::output::Output,
                > {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "GenesisBuilder",
                        "get_preset",
                        (id,),
                        [
                            43u8, 153u8, 23u8, 52u8, 113u8, 161u8, 227u8, 122u8, 169u8, 135u8,
                            119u8, 8u8, 128u8, 33u8, 143u8, 235u8, 13u8, 173u8, 58u8, 121u8, 178u8,
                            223u8, 66u8, 217u8, 22u8, 244u8, 168u8, 113u8, 202u8, 186u8, 241u8,
                            124u8,
                        ],
                    )
                }
                #[doc = " Returns a list of identifiers for available builtin `RuntimeGenesisConfig` presets."]
                #[doc = ""]
                #[doc = " The presets from the list can be queried with [`GenesisBuilder::get_preset`] method. If"]
                #[doc = " no named presets are provided by the runtime the list is empty."]
                pub fn preset_names(
                    &self,
                ) -> ::subxt::runtime_apis::StaticPayload<(), preset_names::output::Output>
                {
                    ::subxt::runtime_apis::StaticPayload::new_static(
                        "GenesisBuilder",
                        "preset_names",
                        (),
                        [
                            150u8, 117u8, 54u8, 129u8, 221u8, 130u8, 186u8, 71u8, 13u8, 140u8,
                            77u8, 180u8, 141u8, 37u8, 22u8, 219u8, 149u8, 218u8, 186u8, 206u8,
                            80u8, 42u8, 165u8, 41u8, 99u8, 184u8, 73u8, 37u8, 125u8, 188u8, 167u8,
                            122u8,
                        ],
                    )
                }
            }
            pub mod build_state {
                use super::root_mod;
                use super::runtime_types;
                pub type Json = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::result::Result<(), ::subxt::alloc::string::String>;
                }
            }
            pub mod get_preset {
                use super::root_mod;
                use super::runtime_types;
                pub type Id = ::core::option::Option<::subxt::alloc::string::String>;
                pub mod output {
                    use super::runtime_types;
                    pub type Output =
                        ::core::option::Option<::subxt::alloc::vec::Vec<::core::primitive::u8>>;
                }
            }
            pub mod preset_names {
                use super::root_mod;
                use super::runtime_types;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::subxt::alloc::vec::Vec<::subxt::alloc::string::String>;
                }
            }
        }
    }
    pub fn view_functions() -> ViewFunctionsApi {
        ViewFunctionsApi
    }
    pub fn custom_values() -> CustomValuesApi {
        CustomValuesApi
    }
    pub struct CustomValuesApi;
    impl CustomValuesApi {}
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn aura(&self) -> aura::constants::ConstantsApi {
            aura::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn aura(&self) -> aura::storage::StorageApi {
            aura::storage::StorageApi
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn template(&self) -> template::storage::StorageApi {
            template::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::api::TransactionApi {
            system::calls::api::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::api::TransactionApi {
            timestamp::calls::api::TransactionApi
        }
        pub fn grandpa(&self) -> grandpa::calls::api::TransactionApi {
            grandpa::calls::api::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::api::TransactionApi {
            balances::calls::api::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::api::TransactionApi {
            sudo::calls::api::TransactionApi
        }
        pub fn template(&self) -> template::calls::api::TransactionApi {
            template::calls::api::TransactionApi
        }
    }
    pub struct ViewFunctionsApi;
    impl ViewFunctionsApi {}
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                154u8, 152u8, 229u8, 185u8, 48u8, 126u8, 87u8, 176u8, 207u8, 176u8, 177u8, 51u8,
                1u8, 99u8, 30u8, 157u8, 187u8, 75u8, 8u8, 61u8, 107u8, 171u8, 214u8, 41u8, 120u8,
                144u8, 76u8, 170u8, 54u8, 87u8, 81u8, 83u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the System pallet"]
        pub type Error = runtime_types::frame_system::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Make some on-chain remark."]
            #[doc = ""]
            #[doc = "Can be executed by every `origin`."]
            pub struct Remark {
                pub remark: remark::Remark,
            }
            pub mod remark {
                use super::runtime_types;
                pub type Remark = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl Remark {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "remark";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for Remark {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Set the number of pages in the WebAssembly environment's heap."]
            pub struct SetHeapPages {
                pub pages: set_heap_pages::Pages,
            }
            pub mod set_heap_pages {
                use super::runtime_types;
                pub type Pages = ::core::primitive::u64;
            }
            impl SetHeapPages {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "set_heap_pages";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for SetHeapPages {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Set the new runtime code."]
            pub struct SetCode {
                pub code: set_code::Code,
            }
            pub mod set_code {
                use super::runtime_types;
                pub type Code = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl SetCode {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "set_code";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for SetCode {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Set the new runtime code without doing any checks of the given `code`."]
            #[doc = ""]
            #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
            #[doc = "version!"]
            pub struct SetCodeWithoutChecks {
                pub code: set_code_without_checks::Code,
            }
            pub mod set_code_without_checks {
                use super::runtime_types;
                pub type Code = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl SetCodeWithoutChecks {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "set_code_without_checks";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for SetCodeWithoutChecks {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Set some items of storage."]
            pub struct SetStorage {
                pub items: set_storage::Items,
            }
            pub mod set_storage {
                use super::runtime_types;
                pub type Items = ::subxt::alloc::vec::Vec<(
                    ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                    ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                )>;
            }
            impl SetStorage {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "set_storage";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for SetStorage {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Kill some items from storage."]
            pub struct KillStorage {
                pub keys: kill_storage::Keys,
            }
            pub mod kill_storage {
                use super::runtime_types;
                pub type Keys =
                    ::subxt::alloc::vec::Vec<::subxt::alloc::vec::Vec<::core::primitive::u8>>;
            }
            impl KillStorage {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "kill_storage";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for KillStorage {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Kill all storage items with a key that starts with the given prefix."]
            #[doc = ""]
            #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
            #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
            pub struct KillPrefix {
                pub prefix: kill_prefix::Prefix,
                pub subkeys: kill_prefix::Subkeys,
            }
            pub mod kill_prefix {
                use super::runtime_types;
                pub type Prefix = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
                pub type Subkeys = ::core::primitive::u32;
            }
            impl KillPrefix {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "kill_prefix";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for KillPrefix {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Make some on-chain remark and emit event."]
            pub struct RemarkWithEvent {
                pub remark: remark_with_event::Remark,
            }
            pub mod remark_with_event {
                use super::runtime_types;
                pub type Remark = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl RemarkWithEvent {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "remark_with_event";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for RemarkWithEvent {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
            #[doc = "later."]
            #[doc = ""]
            #[doc = "This call requires Root origin."]
            pub struct AuthorizeUpgrade {
                pub code_hash: authorize_upgrade::CodeHash,
            }
            pub mod authorize_upgrade {
                use super::runtime_types;
                pub type CodeHash = ::subxt::utils::H256;
            }
            impl AuthorizeUpgrade {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "authorize_upgrade";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for AuthorizeUpgrade {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
            #[doc = "later."]
            #[doc = ""]
            #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
            #[doc = "example that the spec name remains the same and that the version number increases. Not"]
            #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
            #[doc = ""]
            #[doc = "This call requires Root origin."]
            pub struct AuthorizeUpgradeWithoutChecks {
                pub code_hash: authorize_upgrade_without_checks::CodeHash,
            }
            pub mod authorize_upgrade_without_checks {
                use super::runtime_types;
                pub type CodeHash = ::subxt::utils::H256;
            }
            impl AuthorizeUpgradeWithoutChecks {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "authorize_upgrade_without_checks";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for AuthorizeUpgradeWithoutChecks {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
            #[doc = ""]
            #[doc = "If the authorization required a version check, this call will ensure the spec name"]
            #[doc = "remains unchanged and that the spec version has increased."]
            #[doc = ""]
            #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
            #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
            #[doc = ""]
            #[doc = "All origins are allowed."]
            pub struct ApplyAuthorizedUpgrade {
                pub code: apply_authorized_upgrade::Code,
            }
            pub mod apply_authorized_upgrade {
                use super::runtime_types;
                pub type Code = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl ApplyAuthorizedUpgrade {
                const PALLET_NAME: &'static str = "System";
                const CALL_NAME: &'static str = "apply_authorized_upgrade";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for ApplyAuthorizedUpgrade {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            pub mod api {
                pub struct TransactionApi;
                impl TransactionApi {
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "Can be executed by every `origin`."]
                    pub fn remark(
                        &self,
                        remark: super::remark::Remark,
                    ) -> ::subxt::transactions::StaticPayload<super::Remark> {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "remark",
                            super::Remark { remark },
                            [
                                43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                                216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8,
                                33u8, 250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8,
                                207u8, 13u8,
                            ],
                        )
                    }
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    pub fn set_heap_pages(
                        &self,
                        pages: super::set_heap_pages::Pages,
                    ) -> ::subxt::transactions::StaticPayload<super::SetHeapPages>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "set_heap_pages",
                            super::SetHeapPages { pages },
                            [
                                188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                                215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                                134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                                57u8, 147u8,
                            ],
                        )
                    }
                    #[doc = "Set the new runtime code."]
                    pub fn set_code(
                        &self,
                        code: super::set_code::Code,
                    ) -> ::subxt::transactions::StaticPayload<super::SetCode> {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "set_code",
                            super::SetCode { code },
                            [
                                233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8,
                                19u8, 203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8,
                                244u8, 188u8, 27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8,
                                182u8, 22u8,
                            ],
                        )
                    }
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                    #[doc = "version!"]
                    pub fn set_code_without_checks(
                        &self,
                        code: super::set_code_without_checks::Code,
                    ) -> ::subxt::transactions::StaticPayload<super::SetCodeWithoutChecks>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "set_code_without_checks",
                            super::SetCodeWithoutChecks { code },
                            [
                                82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8,
                                107u8, 157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8,
                                10u8, 147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8,
                                119u8, 115u8,
                            ],
                        )
                    }
                    #[doc = "Set some items of storage."]
                    pub fn set_storage(
                        &self,
                        items: super::set_storage::Items,
                    ) -> ::subxt::transactions::StaticPayload<super::SetStorage>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "set_storage",
                            super::SetStorage { items },
                            [
                                141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                                163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                                150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                                234u8, 43u8,
                            ],
                        )
                    }
                    #[doc = "Kill some items from storage."]
                    pub fn kill_storage(
                        &self,
                        keys: super::kill_storage::Keys,
                    ) -> ::subxt::transactions::StaticPayload<super::KillStorage>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "kill_storage",
                            super::KillStorage { keys },
                            [
                                73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8,
                                209u8, 234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8,
                                58u8, 156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8,
                                41u8, 35u8,
                            ],
                        )
                    }
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    pub fn kill_prefix(
                        &self,
                        prefix: super::kill_prefix::Prefix,
                        subkeys: super::kill_prefix::Subkeys,
                    ) -> ::subxt::transactions::StaticPayload<super::KillPrefix>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "kill_prefix",
                            super::KillPrefix { prefix, subkeys },
                            [
                                184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                                175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                                67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8,
                                214u8, 85u8,
                            ],
                        )
                    }
                    #[doc = "Make some on-chain remark and emit event."]
                    pub fn remark_with_event(
                        &self,
                        remark: super::remark_with_event::Remark,
                    ) -> ::subxt::transactions::StaticPayload<super::RemarkWithEvent>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "remark_with_event",
                            super::RemarkWithEvent { remark },
                            [
                                120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8,
                                105u8, 228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8,
                                4u8, 49u8, 147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8,
                                8u8, 154u8,
                            ],
                        )
                    }
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    pub fn authorize_upgrade(
                        &self,
                        code_hash: super::authorize_upgrade::CodeHash,
                    ) -> ::subxt::transactions::StaticPayload<super::AuthorizeUpgrade>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "authorize_upgrade",
                            super::AuthorizeUpgrade { code_hash },
                            [
                                4u8, 14u8, 76u8, 107u8, 209u8, 129u8, 9u8, 39u8, 193u8, 17u8, 84u8,
                                254u8, 170u8, 214u8, 24u8, 155u8, 29u8, 184u8, 249u8, 241u8, 109u8,
                                58u8, 145u8, 131u8, 109u8, 63u8, 38u8, 165u8, 107u8, 215u8, 217u8,
                                172u8,
                            ],
                        )
                    }
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                    #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                    #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    pub fn authorize_upgrade_without_checks(
                        &self,
                        code_hash: super::authorize_upgrade_without_checks::CodeHash,
                    ) -> ::subxt::transactions::StaticPayload<super::AuthorizeUpgradeWithoutChecks>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "authorize_upgrade_without_checks",
                            super::AuthorizeUpgradeWithoutChecks { code_hash },
                            [
                                126u8, 126u8, 55u8, 26u8, 47u8, 55u8, 66u8, 8u8, 167u8, 18u8, 29u8,
                                136u8, 146u8, 14u8, 189u8, 117u8, 16u8, 227u8, 162u8, 61u8, 149u8,
                                197u8, 104u8, 184u8, 185u8, 161u8, 99u8, 154u8, 80u8, 125u8, 181u8,
                                233u8,
                            ],
                        )
                    }
                    #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                    #[doc = ""]
                    #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                    #[doc = "remains unchanged and that the spec version has increased."]
                    #[doc = ""]
                    #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                    #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                    #[doc = ""]
                    #[doc = "All origins are allowed."]
                    pub fn apply_authorized_upgrade(
                        &self,
                        code: super::apply_authorized_upgrade::Code,
                    ) -> ::subxt::transactions::StaticPayload<super::ApplyAuthorizedUpgrade>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "System",
                            "apply_authorized_upgrade",
                            super::ApplyAuthorizedUpgrade { code },
                            [
                                232u8, 107u8, 127u8, 38u8, 230u8, 29u8, 97u8, 4u8, 160u8, 191u8,
                                222u8, 156u8, 245u8, 102u8, 196u8, 141u8, 44u8, 163u8, 98u8, 68u8,
                                125u8, 32u8, 124u8, 101u8, 108u8, 93u8, 211u8, 52u8, 0u8, 231u8,
                                33u8, 227u8,
                            ],
                        )
                    }
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: extrinsic_success::DispatchInfo,
            }
            pub mod extrinsic_success {
                use super::runtime_types;
                pub type DispatchInfo = runtime_types::frame_system::DispatchEventInfo;
            }
            impl ExtrinsicSuccess {
                const PALLET_NAME: &'static str = "System";
                const EVENT_NAME: &'static str = "ExtrinsicSuccess";
            }
            impl ::subxt::events::DecodeAsEvent for ExtrinsicSuccess {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: extrinsic_failed::DispatchError,
                pub dispatch_info: extrinsic_failed::DispatchInfo,
            }
            pub mod extrinsic_failed {
                use super::runtime_types;
                pub type DispatchError = runtime_types::sp_runtime::DispatchError;
                pub type DispatchInfo = runtime_types::frame_system::DispatchEventInfo;
            }
            impl ExtrinsicFailed {
                const PALLET_NAME: &'static str = "System";
                const EVENT_NAME: &'static str = "ExtrinsicFailed";
            }
            impl ::subxt::events::DecodeAsEvent for ExtrinsicFailed {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl CodeUpdated {
                const PALLET_NAME: &'static str = "System";
                const EVENT_NAME: &'static str = "CodeUpdated";
            }
            impl ::subxt::events::DecodeAsEvent for CodeUpdated {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: new_account::Account,
            }
            pub mod new_account {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
            }
            impl NewAccount {
                const PALLET_NAME: &'static str = "System";
                const EVENT_NAME: &'static str = "NewAccount";
            }
            impl ::subxt::events::DecodeAsEvent for NewAccount {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: killed_account::Account,
            }
            pub mod killed_account {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
            }
            impl KilledAccount {
                const PALLET_NAME: &'static str = "System";
                const EVENT_NAME: &'static str = "KilledAccount";
            }
            impl ::subxt::events::DecodeAsEvent for KilledAccount {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: remarked::Sender,
                pub hash: remarked::Hash,
            }
            pub mod remarked {
                use super::runtime_types;
                pub type Sender = ::subxt::utils::AccountId32;
                pub type Hash = ::subxt::utils::H256;
            }
            impl Remarked {
                const PALLET_NAME: &'static str = "System";
                const EVENT_NAME: &'static str = "Remarked";
            }
            impl ::subxt::events::DecodeAsEvent for Remarked {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An upgrade was authorized."]
            pub struct UpgradeAuthorized {
                pub code_hash: upgrade_authorized::CodeHash,
                pub check_version: upgrade_authorized::CheckVersion,
            }
            pub mod upgrade_authorized {
                use super::runtime_types;
                pub type CodeHash = ::subxt::utils::H256;
                pub type CheckVersion = ::core::primitive::bool;
            }
            impl UpgradeAuthorized {
                const PALLET_NAME: &'static str = "System";
                const EVENT_NAME: &'static str = "UpgradeAuthorized";
            }
            impl ::subxt::events::DecodeAsEvent for UpgradeAuthorized {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An invalid authorized upgrade was rejected while trying to apply it."]
            pub struct RejectedInvalidAuthorizedUpgrade {
                pub code_hash: rejected_invalid_authorized_upgrade::CodeHash,
                pub error: rejected_invalid_authorized_upgrade::Error,
            }
            pub mod rejected_invalid_authorized_upgrade {
                use super::runtime_types;
                pub type CodeHash = ::subxt::utils::H256;
                pub type Error = runtime_types::sp_runtime::DispatchError;
            }
            impl RejectedInvalidAuthorizedUpgrade {
                const PALLET_NAME: &'static str = "System";
                const EVENT_NAME: &'static str = "RejectedInvalidAuthorizedUpgrade";
            }
            impl ::subxt::events::DecodeAsEvent for RejectedInvalidAuthorizedUpgrade {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
        }
        pub mod storage {
            use super::root_mod;
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (account::input::Param0,),
                    account::Output,
                    ::subxt::utils::Maybe,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "Account",
                        [
                            181u8, 49u8, 172u8, 169u8, 233u8, 186u8, 227u8, 180u8, 188u8, 130u8,
                            4u8, 70u8, 12u8, 226u8, 233u8, 72u8, 145u8, 59u8, 210u8, 78u8, 48u8,
                            177u8, 203u8, 27u8, 216u8, 196u8, 244u8, 208u8, 26u8, 34u8, 13u8, 50u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), extrinsic_count::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "ExtrinsicCount",
                        [
                            217u8, 77u8, 146u8, 117u8, 157u8, 10u8, 137u8, 158u8, 27u8, 206u8,
                            129u8, 195u8, 192u8, 141u8, 178u8, 6u8, 39u8, 199u8, 156u8, 101u8,
                            60u8, 4u8, 166u8, 244u8, 193u8, 255u8, 148u8, 199u8, 83u8, 157u8, 67u8,
                            193u8,
                        ],
                    )
                }
                #[doc = " Whether all inherents have been applied."]
                pub fn inherents_applied(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (),
                    inherents_applied::Output,
                    ::subxt::utils::Yes,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "InherentsApplied",
                        [
                            18u8, 210u8, 88u8, 91u8, 207u8, 11u8, 44u8, 234u8, 226u8, 71u8, 52u8,
                            99u8, 125u8, 73u8, 149u8, 37u8, 57u8, 70u8, 39u8, 156u8, 159u8, 16u8,
                            174u8, 10u8, 101u8, 172u8, 44u8, 61u8, 160u8, 139u8, 148u8, 113u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), block_weight::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "BlockWeight",
                        [
                            30u8, 69u8, 207u8, 199u8, 27u8, 245u8, 128u8, 231u8, 49u8, 94u8, 194u8,
                            254u8, 18u8, 97u8, 20u8, 94u8, 12u8, 245u8, 93u8, 39u8, 34u8, 216u8,
                            49u8, 39u8, 128u8, 139u8, 230u8, 83u8, 10u8, 42u8, 195u8, 115u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (),
                    all_extrinsics_len::Output,
                    ::subxt::utils::Yes,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        [
                            120u8, 200u8, 84u8, 67u8, 97u8, 25u8, 119u8, 71u8, 170u8, 217u8, 12u8,
                            157u8, 63u8, 100u8, 245u8, 118u8, 99u8, 65u8, 148u8, 110u8, 252u8,
                            86u8, 172u8, 45u8, 235u8, 90u8, 58u8, 123u8, 59u8, 191u8, 1u8, 215u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (block_hash::input::Param0,),
                    block_hash::Output,
                    ::subxt::utils::Maybe,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        [
                            251u8, 175u8, 179u8, 11u8, 47u8, 25u8, 43u8, 165u8, 168u8, 224u8, 35u8,
                            57u8, 49u8, 93u8, 29u8, 47u8, 145u8, 113u8, 84u8, 200u8, 186u8, 21u8,
                            22u8, 102u8, 126u8, 233u8, 10u8, 131u8, 47u8, 32u8, 165u8, 194u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (extrinsic_data::input::Param0,),
                    extrinsic_data::Output,
                    ::subxt::utils::Maybe,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        [
                            223u8, 197u8, 229u8, 38u8, 179u8, 46u8, 153u8, 107u8, 35u8, 131u8,
                            71u8, 231u8, 213u8, 198u8, 141u8, 55u8, 2u8, 75u8, 114u8, 159u8, 0u8,
                            16u8, 128u8, 190u8, 177u8, 92u8, 225u8, 213u8, 48u8, 167u8, 29u8,
                            121u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), number::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "Number",
                        [
                            93u8, 185u8, 195u8, 173u8, 19u8, 1u8, 39u8, 245u8, 243u8, 67u8, 228u8,
                            232u8, 25u8, 15u8, 14u8, 109u8, 225u8, 34u8, 17u8, 110u8, 25u8, 154u8,
                            149u8, 46u8, 184u8, 208u8, 79u8, 254u8, 166u8, 168u8, 33u8, 173u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), parent_hash::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "ParentHash",
                        [
                            252u8, 127u8, 135u8, 108u8, 14u8, 75u8, 71u8, 121u8, 36u8, 231u8, 44u8,
                            64u8, 49u8, 246u8, 24u8, 49u8, 202u8, 229u8, 242u8, 74u8, 206u8, 65u8,
                            78u8, 207u8, 12u8, 118u8, 33u8, 42u8, 130u8, 233u8, 33u8, 136u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), digest::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "Digest",
                        [
                            137u8, 44u8, 198u8, 131u8, 117u8, 17u8, 114u8, 93u8, 213u8, 123u8,
                            212u8, 55u8, 43u8, 34u8, 114u8, 86u8, 39u8, 18u8, 189u8, 157u8, 27u8,
                            157u8, 155u8, 159u8, 147u8, 41u8, 138u8, 195u8, 20u8, 204u8, 110u8,
                            53u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), events::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "Events",
                        [
                            250u8, 241u8, 86u8, 36u8, 19u8, 187u8, 14u8, 242u8, 233u8, 111u8,
                            238u8, 107u8, 227u8, 204u8, 31u8, 247u8, 46u8, 40u8, 146u8, 200u8,
                            208u8, 55u8, 175u8, 223u8, 242u8, 107u8, 220u8, 118u8, 33u8, 100u8,
                            202u8, 182u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), event_count::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "EventCount",
                        [
                            32u8, 54u8, 196u8, 23u8, 224u8, 204u8, 158u8, 79u8, 151u8, 46u8, 107u8,
                            24u8, 120u8, 90u8, 137u8, 234u8, 64u8, 92u8, 174u8, 25u8, 152u8, 22u8,
                            24u8, 245u8, 243u8, 212u8, 148u8, 149u8, 122u8, 171u8, 92u8, 140u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (event_topics::input::Param0,),
                    event_topics::Output,
                    ::subxt::utils::Maybe,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        [
                            91u8, 29u8, 70u8, 62u8, 102u8, 127u8, 50u8, 42u8, 122u8, 136u8, 211u8,
                            187u8, 165u8, 1u8, 82u8, 213u8, 58u8, 154u8, 239u8, 26u8, 213u8, 120u8,
                            8u8, 179u8, 2u8, 134u8, 90u8, 241u8, 163u8, 199u8, 98u8, 94u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (),
                    last_runtime_upgrade::Output,
                    ::subxt::utils::Yes,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        [
                            239u8, 183u8, 167u8, 75u8, 149u8, 166u8, 239u8, 31u8, 200u8, 140u8,
                            61u8, 169u8, 227u8, 186u8, 101u8, 14u8, 78u8, 101u8, 19u8, 86u8, 128u8,
                            203u8, 250u8, 97u8, 210u8, 179u8, 96u8, 191u8, 226u8, 225u8, 32u8,
                            212u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (),
                    upgraded_to_u32_ref_count::Output,
                    ::subxt::utils::Yes,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        [
                            121u8, 56u8, 110u8, 113u8, 59u8, 171u8, 213u8, 125u8, 149u8, 111u8,
                            171u8, 66u8, 48u8, 0u8, 129u8, 158u8, 118u8, 33u8, 255u8, 236u8, 109u8,
                            47u8, 123u8, 153u8, 40u8, 25u8, 16u8, 60u8, 248u8, 5u8, 94u8, 235u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (),
                    upgraded_to_triple_ref_count::Output,
                    ::subxt::utils::Yes,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        [
                            21u8, 68u8, 180u8, 14u8, 122u8, 62u8, 230u8, 35u8, 163u8, 50u8, 98u8,
                            110u8, 27u8, 46u8, 205u8, 112u8, 29u8, 175u8, 250u8, 160u8, 76u8,
                            139u8, 10u8, 64u8, 158u8, 114u8, 176u8, 180u8, 252u8, 66u8, 6u8, 103u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), execution_phase::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "ExecutionPhase",
                        [
                            73u8, 148u8, 64u8, 200u8, 68u8, 224u8, 56u8, 2u8, 224u8, 156u8, 197u8,
                            124u8, 8u8, 173u8, 3u8, 36u8, 146u8, 33u8, 219u8, 205u8, 36u8, 89u8,
                            99u8, 231u8, 208u8, 2u8, 236u8, 254u8, 254u8, 108u8, 65u8, 68u8,
                        ],
                    )
                }
                #[doc = " `Some` if a code upgrade has been authorized."]
                pub fn authorized_upgrade(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (),
                    authorized_upgrade::Output,
                    ::subxt::utils::Yes,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "AuthorizedUpgrade",
                        [
                            227u8, 212u8, 35u8, 221u8, 172u8, 87u8, 76u8, 244u8, 15u8, 213u8, 25u8,
                            209u8, 213u8, 3u8, 230u8, 224u8, 81u8, 7u8, 62u8, 238u8, 51u8, 158u8,
                            221u8, 35u8, 1u8, 5u8, 213u8, 142u8, 140u8, 206u8, 216u8, 214u8,
                        ],
                    )
                }
                #[doc = " The weight reclaimed for the extrinsic."]
                #[doc = ""]
                #[doc = " This information is available until the end of the extrinsic execution."]
                #[doc = " More precisely this information is removed in `note_applied_extrinsic`."]
                #[doc = ""]
                #[doc = " Logic doing some post dispatch weight reduction must update this storage to avoid duplicate"]
                #[doc = " reduction."]
                pub fn extrinsic_weight_reclaimed(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (),
                    extrinsic_weight_reclaimed::Output,
                    ::subxt::utils::Yes,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "System",
                        "ExtrinsicWeightReclaimed",
                        [
                            205u8, 30u8, 170u8, 39u8, 212u8, 71u8, 90u8, 173u8, 142u8, 127u8,
                            164u8, 223u8, 113u8, 224u8, 161u8, 109u8, 102u8, 241u8, 4u8, 225u8,
                            105u8, 163u8, 161u8, 96u8, 69u8, 178u8, 77u8, 154u8, 222u8, 83u8,
                            106u8, 175u8,
                        ],
                    )
                }
            }
            pub mod account {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub type Output = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                >;
            }
            pub mod extrinsic_count {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::u32;
            }
            pub mod inherents_applied {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::bool;
            }
            pub mod block_weight {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::frame_support::dispatch::PerDispatchClass<
                    runtime_types::sp_weights::weight_v2::Weight,
                >;
            }
            pub mod all_extrinsics_len {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::u32;
            }
            pub mod block_hash {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub type Output = ::subxt::utils::H256;
            }
            pub mod extrinsic_data {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub type Output = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
            }
            pub mod number {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::u32;
            }
            pub mod parent_hash {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::subxt::utils::H256;
            }
            pub mod digest {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::sp_runtime::generic::digest::Digest;
            }
            pub mod events {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::subxt::alloc::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::solochain_template_runtime::RuntimeEvent,
                        ::subxt::utils::H256,
                    >,
                >;
            }
            pub mod event_count {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::u32;
            }
            pub mod event_topics {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                    pub type Param0 = ::subxt::utils::H256;
                }
                pub type Output =
                    ::subxt::alloc::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
            }
            pub mod last_runtime_upgrade {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::frame_system::LastRuntimeUpgradeInfo;
            }
            pub mod upgraded_to_u32_ref_count {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::bool;
            }
            pub mod upgraded_to_triple_ref_count {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::bool;
            }
            pub mod execution_phase {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::frame_system::Phase;
            }
            pub mod authorized_upgrade {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::frame_system::CodeUpgradeAuthorization;
            }
            pub mod extrinsic_weight_reclaimed {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::sp_weights::weight_v2::Weight;
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::StaticAddress<
                    runtime_types::frame_system::limits::BlockWeights,
                > {
                    ::subxt::constants::StaticAddress::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::StaticAddress<
                    runtime_types::frame_system::limits::BlockLength,
                > {
                    ::subxt::constants::StaticAddress::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u32> {
                    ::subxt::constants::StaticAddress::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::StaticAddress<runtime_types::sp_weights::RuntimeDbWeight>
                {
                    ::subxt::constants::StaticAddress::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }
                #[doc = " Get the chain's in-code version."]
                pub fn version(
                    &self,
                ) -> ::subxt::constants::StaticAddress<runtime_types::sp_version::RuntimeVersion>
                {
                    ::subxt::constants::StaticAddress::new_static(
                        "System",
                        "Version",
                        [
                            214u8, 43u8, 96u8, 193u8, 96u8, 213u8, 63u8, 124u8, 22u8, 111u8, 41u8,
                            78u8, 146u8, 77u8, 34u8, 163u8, 117u8, 100u8, 6u8, 216u8, 238u8, 54u8,
                            80u8, 185u8, 219u8, 11u8, 192u8, 200u8, 129u8, 88u8, 161u8, 250u8,
                        ],
                    )
                }
                #[doc = " The designated SS58 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u16> {
                    ::subxt::constants::StaticAddress::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Set the current time."]
            #[doc = ""]
            #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
            #[doc = "phase, if this call hasn't been invoked by that time."]
            #[doc = ""]
            #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
            #[doc = "[`Config::MinimumPeriod`]."]
            #[doc = ""]
            #[doc = "The dispatch origin for this call must be _None_."]
            #[doc = ""]
            #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
            #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
            #[doc = "block to execute any other calls."]
            #[doc = ""]
            #[doc = "## Complexity"]
            #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
            #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
            #[doc = "  `on_finalize`)"]
            #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
            pub struct Set {
                #[codec(compact)]
                pub now: set::Now,
            }
            pub mod set {
                use super::runtime_types;
                pub type Now = ::core::primitive::u64;
            }
            impl Set {
                const PALLET_NAME: &'static str = "Timestamp";
                const CALL_NAME: &'static str = "set";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for Set {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            pub mod api {
                pub struct TransactionApi;
                impl TransactionApi {
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "[`Config::MinimumPeriod`]."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _None_."]
                    #[doc = ""]
                    #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                    #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                    #[doc = "block to execute any other calls."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    pub fn set(
                        &self,
                        now: super::set::Now,
                    ) -> ::subxt::transactions::StaticPayload<super::Set> {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Timestamp",
                            "set",
                            super::Set { now },
                            [
                                37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                                199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                                200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8,
                                227u8,
                            ],
                        )
                    }
                }
            }
        }
        pub mod storage {
            use super::root_mod;
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), now::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Timestamp",
                        "Now",
                        [
                            33u8, 56u8, 170u8, 83u8, 141u8, 145u8, 85u8, 240u8, 128u8, 31u8, 207u8,
                            119u8, 3u8, 202u8, 67u8, 6u8, 117u8, 189u8, 75u8, 35u8, 142u8, 183u8,
                            127u8, 182u8, 208u8, 169u8, 153u8, 229u8, 251u8, 53u8, 181u8, 45u8,
                        ],
                    )
                }
                #[doc = " Whether the timestamp has been updated in this block."]
                #[doc = ""]
                #[doc = " This value is updated to `true` upon successful submission of a timestamp by a node."]
                #[doc = " It is then checked at the end of each block execution in the `on_finalize` hook."]
                pub fn did_update(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), did_update::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Timestamp",
                        "DidUpdate",
                        [
                            159u8, 174u8, 212u8, 192u8, 172u8, 1u8, 246u8, 2u8, 150u8, 55u8, 251u8,
                            62u8, 194u8, 210u8, 15u8, 214u8, 177u8, 160u8, 15u8, 138u8, 142u8,
                            125u8, 113u8, 227u8, 201u8, 250u8, 223u8, 252u8, 123u8, 144u8, 209u8,
                            10u8,
                        ],
                    )
                }
            }
            pub mod now {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::u64;
            }
            pub mod did_update {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::bool;
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks."]
                #[doc = ""]
                #[doc = " Be aware that this is different to the *expected* period that the block production"]
                #[doc = " apparatus provides. Your chosen consensus system will generally work with this to"]
                #[doc = " determine a sensible block time. For example, in the Aura pallet it will be double this"]
                #[doc = " period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u64> {
                    ::subxt::constants::StaticAddress::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::root_mod;
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current authority set."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), authorities::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Aura",
                        "Authorities",
                        [
                            115u8, 164u8, 253u8, 15u8, 110u8, 193u8, 198u8, 238u8, 70u8, 39u8,
                            107u8, 5u8, 197u8, 103u8, 95u8, 110u8, 83u8, 156u8, 209u8, 81u8, 44u8,
                            44u8, 8u8, 12u8, 0u8, 98u8, 33u8, 100u8, 228u8, 128u8, 8u8, 88u8,
                        ],
                    )
                }
                #[doc = " The current slot of this block."]
                #[doc = ""]
                #[doc = " This will be set in `on_initialize`."]
                pub fn current_slot(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), current_slot::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Aura",
                        "CurrentSlot",
                        [
                            43u8, 143u8, 102u8, 240u8, 243u8, 39u8, 191u8, 181u8, 112u8, 100u8,
                            100u8, 92u8, 169u8, 252u8, 192u8, 187u8, 231u8, 43u8, 235u8, 136u8,
                            116u8, 180u8, 82u8, 36u8, 140u8, 92u8, 203u8, 143u8, 4u8, 90u8, 86u8,
                            13u8,
                        ],
                    )
                }
            }
            pub mod authorities {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                >;
            }
            pub mod current_slot {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::sp_consensus_slots::Slot;
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The slot duration Aura should run with, expressed in milliseconds."]
                #[doc = " The effective value of this type should not change while the chain is running."]
                #[doc = ""]
                #[doc = " For backwards compatibility either use [`MinimumPeriodTimesTwo`] or a const."]
                pub fn slot_duration(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u64> {
                    ::subxt::constants::StaticAddress::new_static(
                        "Aura",
                        "SlotDuration",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_grandpa::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_grandpa::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
            #[doc = "equivocation proof and validate the given key ownership proof"]
            #[doc = "against the extracted offender. If both are valid, the offence"]
            #[doc = "will be reported."]
            pub struct ReportEquivocation {
                pub equivocation_proof:
                    ::subxt::alloc::boxed::Box<report_equivocation::EquivocationProof>,
                pub key_owner_proof: report_equivocation::KeyOwnerProof,
            }
            pub mod report_equivocation {
                use super::runtime_types;
                pub type EquivocationProof = runtime_types::sp_consensus_grandpa::EquivocationProof<
                    ::subxt::utils::H256,
                    ::core::primitive::u32,
                >;
                pub type KeyOwnerProof = runtime_types::sp_core::Void;
            }
            impl ReportEquivocation {
                const PALLET_NAME: &'static str = "Grandpa";
                const CALL_NAME: &'static str = "report_equivocation";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for ReportEquivocation {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
            #[doc = "equivocation proof and validate the given key ownership proof"]
            #[doc = "against the extracted offender. If both are valid, the offence"]
            #[doc = "will be reported."]
            #[doc = ""]
            #[doc = "This extrinsic must be called unsigned and it is expected that only"]
            #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
            #[doc = "if the block author is defined it will be defined as the equivocation"]
            #[doc = "reporter."]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof:
                    ::subxt::alloc::boxed::Box<report_equivocation_unsigned::EquivocationProof>,
                pub key_owner_proof: report_equivocation_unsigned::KeyOwnerProof,
            }
            pub mod report_equivocation_unsigned {
                use super::runtime_types;
                pub type EquivocationProof = runtime_types::sp_consensus_grandpa::EquivocationProof<
                    ::subxt::utils::H256,
                    ::core::primitive::u32,
                >;
                pub type KeyOwnerProof = runtime_types::sp_core::Void;
            }
            impl ReportEquivocationUnsigned {
                const PALLET_NAME: &'static str = "Grandpa";
                const CALL_NAME: &'static str = "report_equivocation_unsigned";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for ReportEquivocationUnsigned {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
            #[doc = ""]
            #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
            #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
            #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
            #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
            #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
            #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
            #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
            #[doc = "block of all validators of the new authority set."]
            #[doc = ""]
            #[doc = "Only callable by root."]
            pub struct NoteStalled {
                pub delay: note_stalled::Delay,
                pub best_finalized_block_number: note_stalled::BestFinalizedBlockNumber,
            }
            pub mod note_stalled {
                use super::runtime_types;
                pub type Delay = ::core::primitive::u32;
                pub type BestFinalizedBlockNumber = ::core::primitive::u32;
            }
            impl NoteStalled {
                const PALLET_NAME: &'static str = "Grandpa";
                const CALL_NAME: &'static str = "note_stalled";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for NoteStalled {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            pub mod api {
                pub struct TransactionApi;
                impl TransactionApi {
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    pub fn report_equivocation(
                        &self,
                        equivocation_proof: super::report_equivocation::EquivocationProof,
                        key_owner_proof: super::report_equivocation::KeyOwnerProof,
                    ) -> ::subxt::transactions::StaticPayload<super::ReportEquivocation>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Grandpa",
                            "report_equivocation",
                            super::ReportEquivocation {
                                equivocation_proof: ::subxt::alloc::boxed::Box::new(
                                    equivocation_proof,
                                ),
                                key_owner_proof,
                            },
                            [
                                187u8, 224u8, 115u8, 5u8, 236u8, 32u8, 180u8, 155u8, 218u8, 109u8,
                                238u8, 253u8, 30u8, 225u8, 4u8, 225u8, 132u8, 232u8, 243u8, 54u8,
                                56u8, 158u8, 94u8, 192u8, 94u8, 206u8, 189u8, 61u8, 14u8, 49u8,
                                48u8, 131u8,
                            ],
                        )
                    }
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
                    pub fn report_equivocation_unsigned(
                        &self,
                        equivocation_proof: super::report_equivocation_unsigned::EquivocationProof,
                        key_owner_proof: super::report_equivocation_unsigned::KeyOwnerProof,
                    ) -> ::subxt::transactions::StaticPayload<super::ReportEquivocationUnsigned>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Grandpa",
                            "report_equivocation_unsigned",
                            super::ReportEquivocationUnsigned {
                                equivocation_proof: ::subxt::alloc::boxed::Box::new(
                                    equivocation_proof,
                                ),
                                key_owner_proof,
                            },
                            [
                                98u8, 103u8, 6u8, 54u8, 0u8, 200u8, 166u8, 163u8, 202u8, 45u8,
                                131u8, 226u8, 114u8, 166u8, 237u8, 174u8, 207u8, 214u8, 2u8, 227u8,
                                32u8, 166u8, 47u8, 83u8, 166u8, 239u8, 232u8, 72u8, 224u8, 242u8,
                                156u8, 44u8,
                            ],
                        )
                    }
                    #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                    #[doc = ""]
                    #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                    #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                    #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                    #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                    #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                    #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                    #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                    #[doc = "block of all validators of the new authority set."]
                    #[doc = ""]
                    #[doc = "Only callable by root."]
                    pub fn note_stalled(
                        &self,
                        delay: super::note_stalled::Delay,
                        best_finalized_block_number: super::note_stalled::BestFinalizedBlockNumber,
                    ) -> ::subxt::transactions::StaticPayload<super::NoteStalled>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Grandpa",
                            "note_stalled",
                            super::NoteStalled {
                                delay,
                                best_finalized_block_number,
                            },
                            [
                                158u8, 25u8, 64u8, 114u8, 131u8, 139u8, 227u8, 132u8, 42u8, 107u8,
                                40u8, 249u8, 18u8, 93u8, 254u8, 86u8, 37u8, 67u8, 250u8, 35u8,
                                241u8, 194u8, 209u8, 20u8, 39u8, 75u8, 186u8, 21u8, 48u8, 124u8,
                                151u8, 31u8,
                            ],
                        )
                    }
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "New authority set has been applied."]
            pub struct NewAuthorities {
                pub authority_set: new_authorities::AuthoritySet,
            }
            pub mod new_authorities {
                use super::runtime_types;
                pub type AuthoritySet = ::subxt::alloc::vec::Vec<(
                    runtime_types::sp_consensus_grandpa::app::Public,
                    ::core::primitive::u64,
                )>;
            }
            impl NewAuthorities {
                const PALLET_NAME: &'static str = "Grandpa";
                const EVENT_NAME: &'static str = "NewAuthorities";
            }
            impl ::subxt::events::DecodeAsEvent for NewAuthorities {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl Paused {
                const PALLET_NAME: &'static str = "Grandpa";
                const EVENT_NAME: &'static str = "Paused";
            }
            impl ::subxt::events::DecodeAsEvent for Paused {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Current authority set has been resumed."]
            pub struct Resumed;
            impl Resumed {
                const PALLET_NAME: &'static str = "Grandpa";
                const EVENT_NAME: &'static str = "Resumed";
            }
            impl ::subxt::events::DecodeAsEvent for Resumed {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
        }
        pub mod storage {
            use super::root_mod;
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " State of the current authority set."]
                pub fn state(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), state::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Grandpa",
                        "State",
                        [
                            114u8, 253u8, 11u8, 149u8, 130u8, 90u8, 171u8, 105u8, 7u8, 240u8,
                            141u8, 95u8, 1u8, 133u8, 52u8, 124u8, 23u8, 200u8, 144u8, 133u8, 51u8,
                            253u8, 195u8, 207u8, 131u8, 12u8, 80u8, 13u8, 98u8, 204u8, 167u8,
                            166u8,
                        ],
                    )
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]
                pub fn pending_change(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), pending_change::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Grandpa",
                        "PendingChange",
                        [
                            157u8, 46u8, 184u8, 78u8, 39u8, 39u8, 170u8, 225u8, 221u8, 4u8, 80u8,
                            16u8, 3u8, 106u8, 207u8, 217u8, 102u8, 247u8, 20u8, 67u8, 166u8, 102u8,
                            78u8, 209u8, 54u8, 21u8, 117u8, 46u8, 229u8, 215u8, 1u8, 148u8,
                        ],
                    )
                }
                #[doc = " next block number where we can force a change."]
                pub fn next_forced(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), next_forced::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Grandpa",
                        "NextForced",
                        [
                            26u8, 66u8, 120u8, 170u8, 150u8, 89u8, 5u8, 170u8, 18u8, 250u8, 222u8,
                            75u8, 86u8, 62u8, 62u8, 211u8, 98u8, 5u8, 55u8, 152u8, 8u8, 172u8,
                            221u8, 19u8, 87u8, 166u8, 217u8, 52u8, 118u8, 161u8, 79u8, 4u8,
                        ],
                    )
                }
                #[doc = " `true` if we are currently stalled."]
                pub fn stalled(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), stalled::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Grandpa",
                        "Stalled",
                        [
                            222u8, 146u8, 140u8, 145u8, 154u8, 199u8, 130u8, 232u8, 53u8, 97u8,
                            136u8, 16u8, 6u8, 100u8, 178u8, 2u8, 43u8, 13u8, 149u8, 238u8, 135u8,
                            238u8, 147u8, 11u8, 181u8, 27u8, 68u8, 111u8, 149u8, 206u8, 54u8, 28u8,
                        ],
                    )
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), current_set_id::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Grandpa",
                        "CurrentSetId",
                        [
                            24u8, 21u8, 118u8, 180u8, 203u8, 56u8, 108u8, 220u8, 19u8, 26u8, 56u8,
                            207u8, 254u8, 160u8, 122u8, 106u8, 47u8, 55u8, 128u8, 86u8, 175u8,
                            225u8, 132u8, 100u8, 128u8, 62u8, 148u8, 171u8, 163u8, 234u8, 35u8,
                            19u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
                #[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
                #[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
                #[doc = " was the owner of a given key on a given session, and what the active set ID was"]
                #[doc = " during that session."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (set_id_session::input::Param0,),
                    set_id_session::Output,
                    ::subxt::utils::Maybe,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "Grandpa",
                        "SetIdSession",
                        [
                            103u8, 143u8, 223u8, 143u8, 232u8, 62u8, 107u8, 53u8, 99u8, 129u8,
                            124u8, 35u8, 87u8, 83u8, 188u8, 228u8, 91u8, 129u8, 131u8, 154u8,
                            208u8, 221u8, 54u8, 83u8, 144u8, 66u8, 54u8, 35u8, 72u8, 126u8, 97u8,
                            63u8,
                        ],
                    )
                }
                #[doc = " The current list of authorities."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), authorities::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Grandpa",
                        "Authorities",
                        [
                            210u8, 230u8, 47u8, 62u8, 255u8, 111u8, 43u8, 183u8, 246u8, 15u8, 54u8,
                            24u8, 160u8, 228u8, 106u8, 168u8, 219u8, 241u8, 231u8, 234u8, 54u8,
                            109u8, 72u8, 4u8, 76u8, 205u8, 130u8, 30u8, 150u8, 46u8, 5u8, 147u8,
                        ],
                    )
                }
            }
            pub mod state {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output =
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
            }
            pub mod pending_change {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output =
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
            }
            pub mod next_forced {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::u32;
            }
            pub mod stalled {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = (::core::primitive::u32, ::core::primitive::u32);
            }
            pub mod current_set_id {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::u64;
            }
            pub mod set_id_session {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                    pub type Param0 = ::core::primitive::u64;
                }
                pub type Output = ::core::primitive::u32;
            }
            pub mod authorities {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output =
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>;
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u32> {
                    ::subxt::constants::StaticAddress::new_static(
                        "Grandpa",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of nominators for each validator."]
                pub fn max_nominators(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u32> {
                    ::subxt::constants::StaticAddress::new_static(
                        "Grandpa",
                        "MaxNominators",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of entries to keep in the set id to session index mapping."]
                #[doc = ""]
                #[doc = " Since the `SetIdSession` map is only used for validating equivocations this"]
                #[doc = " value should relate to the bonding duration of whatever staking system is"]
                #[doc = " being used (if any). If equivocation handling is not enabled then this value"]
                #[doc = " can be zero."]
                pub fn max_set_id_session_entries(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u64> {
                    ::subxt::constants::StaticAddress::new_static(
                        "Grandpa",
                        "MaxSetIdSessionEntries",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Transfer some liquid free balance to another account."]
            #[doc = ""]
            #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
            #[doc = "If the sender's account is below the existential deposit as a result"]
            #[doc = "of the transfer, the account will be reaped."]
            #[doc = ""]
            #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
            pub struct TransferAllowDeath {
                pub dest: transfer_allow_death::Dest,
                #[codec(compact)]
                pub value: transfer_allow_death::Value,
            }
            pub mod transfer_allow_death {
                use super::runtime_types;
                pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                pub type Value = ::core::primitive::u128;
            }
            impl TransferAllowDeath {
                const PALLET_NAME: &'static str = "Balances";
                const CALL_NAME: &'static str = "transfer_allow_death";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for TransferAllowDeath {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
            #[doc = "may be specified."]
            pub struct ForceTransfer {
                pub source: force_transfer::Source,
                pub dest: force_transfer::Dest,
                #[codec(compact)]
                pub value: force_transfer::Value,
            }
            pub mod force_transfer {
                use super::runtime_types;
                pub type Source = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                pub type Value = ::core::primitive::u128;
            }
            impl ForceTransfer {
                const PALLET_NAME: &'static str = "Balances";
                const CALL_NAME: &'static str = "force_transfer";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for ForceTransfer {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
            #[doc = "kill the origin account."]
            #[doc = ""]
            #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
            #[doc = ""]
            #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
            pub struct TransferKeepAlive {
                pub dest: transfer_keep_alive::Dest,
                #[codec(compact)]
                pub value: transfer_keep_alive::Value,
            }
            pub mod transfer_keep_alive {
                use super::runtime_types;
                pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                pub type Value = ::core::primitive::u128;
            }
            impl TransferKeepAlive {
                const PALLET_NAME: &'static str = "Balances";
                const CALL_NAME: &'static str = "transfer_keep_alive";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for TransferKeepAlive {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Transfer the entire transferable balance from the caller account."]
            #[doc = ""]
            #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
            #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
            #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
            #[doc = "you might need to prepare the account by removing any reference counters, storage"]
            #[doc = "deposits, etc..."]
            #[doc = ""]
            #[doc = "The dispatch origin of this call must be Signed."]
            #[doc = ""]
            #[doc = "- `dest`: The recipient of the transfer."]
            #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
            #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
            #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
            #[doc = "  keep the sender account alive (true)."]
            pub struct TransferAll {
                pub dest: transfer_all::Dest,
                pub keep_alive: transfer_all::KeepAlive,
            }
            pub mod transfer_all {
                use super::runtime_types;
                pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                pub type KeepAlive = ::core::primitive::bool;
            }
            impl TransferAll {
                const PALLET_NAME: &'static str = "Balances";
                const CALL_NAME: &'static str = "transfer_all";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for TransferAll {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Unreserve some balance from a user by force."]
            #[doc = ""]
            #[doc = "Can only be called by ROOT."]
            pub struct ForceUnreserve {
                pub who: force_unreserve::Who,
                pub amount: force_unreserve::Amount,
            }
            pub mod force_unreserve {
                use super::runtime_types;
                pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                pub type Amount = ::core::primitive::u128;
            }
            impl ForceUnreserve {
                const PALLET_NAME: &'static str = "Balances";
                const CALL_NAME: &'static str = "force_unreserve";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for ForceUnreserve {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Upgrade a specified account."]
            #[doc = ""]
            #[doc = "- `origin`: Must be `Signed`."]
            #[doc = "- `who`: The account to be upgraded."]
            #[doc = ""]
            #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
            #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
            #[doc = "possibility of churn)."]
            pub struct UpgradeAccounts {
                pub who: upgrade_accounts::Who,
            }
            pub mod upgrade_accounts {
                use super::runtime_types;
                pub type Who = ::subxt::alloc::vec::Vec<::subxt::utils::AccountId32>;
            }
            impl UpgradeAccounts {
                const PALLET_NAME: &'static str = "Balances";
                const CALL_NAME: &'static str = "upgrade_accounts";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for UpgradeAccounts {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Set the regular balance of a given account."]
            #[doc = ""]
            #[doc = "The dispatch origin for this call is `root`."]
            pub struct ForceSetBalance {
                pub who: force_set_balance::Who,
                #[codec(compact)]
                pub new_free: force_set_balance::NewFree,
            }
            pub mod force_set_balance {
                use super::runtime_types;
                pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                pub type NewFree = ::core::primitive::u128;
            }
            impl ForceSetBalance {
                const PALLET_NAME: &'static str = "Balances";
                const CALL_NAME: &'static str = "force_set_balance";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for ForceSetBalance {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Adjust the total issuance in a saturating way."]
            #[doc = ""]
            #[doc = "Can only be called by root and always needs a positive `delta`."]
            #[doc = ""]
            #[doc = "# Example"]
            pub struct ForceAdjustTotalIssuance {
                pub direction: force_adjust_total_issuance::Direction,
                #[codec(compact)]
                pub delta: force_adjust_total_issuance::Delta,
            }
            pub mod force_adjust_total_issuance {
                use super::runtime_types;
                pub type Direction = runtime_types::pallet_balances::types::AdjustmentDirection;
                pub type Delta = ::core::primitive::u128;
            }
            impl ForceAdjustTotalIssuance {
                const PALLET_NAME: &'static str = "Balances";
                const CALL_NAME: &'static str = "force_adjust_total_issuance";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for ForceAdjustTotalIssuance {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Burn the specified liquid free balance from the origin account."]
            #[doc = ""]
            #[doc = "If the origin's account ends up below the existential deposit as a result"]
            #[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
            #[doc = ""]
            #[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
            #[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
            pub struct Burn {
                #[codec(compact)]
                pub value: burn::Value,
                pub keep_alive: burn::KeepAlive,
            }
            pub mod burn {
                use super::runtime_types;
                pub type Value = ::core::primitive::u128;
                pub type KeepAlive = ::core::primitive::bool;
            }
            impl Burn {
                const PALLET_NAME: &'static str = "Balances";
                const CALL_NAME: &'static str = "burn";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for Burn {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            pub mod api {
                pub struct TransactionApi;
                impl TransactionApi {
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    pub fn transfer_allow_death(
                        &self,
                        dest: super::transfer_allow_death::Dest,
                        value: super::transfer_allow_death::Value,
                    ) -> ::subxt::transactions::StaticPayload<super::TransferAllowDeath>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Balances",
                            "transfer_allow_death",
                            super::TransferAllowDeath { dest, value },
                            [
                                51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8,
                                35u8, 140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8,
                                86u8, 219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8,
                                132u8, 130u8,
                            ],
                        )
                    }
                    #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                    #[doc = "may be specified."]
                    pub fn force_transfer(
                        &self,
                        source: super::force_transfer::Source,
                        dest: super::force_transfer::Dest,
                        value: super::force_transfer::Value,
                    ) -> ::subxt::transactions::StaticPayload<super::ForceTransfer>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Balances",
                            "force_transfer",
                            super::ForceTransfer {
                                source,
                                dest,
                                value,
                            },
                            [
                                154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8,
                                250u8, 153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8,
                                77u8, 180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8,
                                185u8, 96u8,
                            ],
                        )
                    }
                    #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                    #[doc = "kill the origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                    pub fn transfer_keep_alive(
                        &self,
                        dest: super::transfer_keep_alive::Dest,
                        value: super::transfer_keep_alive::Value,
                    ) -> ::subxt::transactions::StaticPayload<super::TransferKeepAlive>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Balances",
                            "transfer_keep_alive",
                            super::TransferKeepAlive { dest, value },
                            [
                                245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8,
                                76u8, 55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8,
                                54u8, 157u8, 208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8,
                                222u8, 59u8,
                            ],
                        )
                    }
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true)."]
                    pub fn transfer_all(
                        &self,
                        dest: super::transfer_all::Dest,
                        keep_alive: super::transfer_all::KeepAlive,
                    ) -> ::subxt::transactions::StaticPayload<super::TransferAll>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Balances",
                            "transfer_all",
                            super::TransferAll { dest, keep_alive },
                            [
                                105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
                                112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8,
                                238u8, 9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8,
                                154u8, 6u8,
                            ],
                        )
                    }
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    pub fn force_unreserve(
                        &self,
                        who: super::force_unreserve::Who,
                        amount: super::force_unreserve::Amount,
                    ) -> ::subxt::transactions::StaticPayload<super::ForceUnreserve>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Balances",
                            "force_unreserve",
                            super::ForceUnreserve { who, amount },
                            [
                                142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8,
                                223u8, 140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8,
                                123u8, 199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8,
                                84u8, 171u8,
                            ],
                        )
                    }
                    #[doc = "Upgrade a specified account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed`."]
                    #[doc = "- `who`: The account to be upgraded."]
                    #[doc = ""]
                    #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                    #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                    #[doc = "possibility of churn)."]
                    pub fn upgrade_accounts(
                        &self,
                        who: super::upgrade_accounts::Who,
                    ) -> ::subxt::transactions::StaticPayload<super::UpgradeAccounts>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Balances",
                            "upgrade_accounts",
                            super::UpgradeAccounts { who },
                            [
                                66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8,
                                224u8, 233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8,
                                68u8, 47u8, 214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8,
                                98u8, 226u8,
                            ],
                        )
                    }
                    #[doc = "Set the regular balance of a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    pub fn force_set_balance(
                        &self,
                        who: super::force_set_balance::Who,
                        new_free: super::force_set_balance::NewFree,
                    ) -> ::subxt::transactions::StaticPayload<super::ForceSetBalance>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Balances",
                            "force_set_balance",
                            super::ForceSetBalance { who, new_free },
                            [
                                114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
                                39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8,
                                164u8, 116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8,
                                43u8,
                            ],
                        )
                    }
                    #[doc = "Adjust the total issuance in a saturating way."]
                    #[doc = ""]
                    #[doc = "Can only be called by root and always needs a positive `delta`."]
                    #[doc = ""]
                    #[doc = "# Example"]
                    pub fn force_adjust_total_issuance(
                        &self,
                        direction: super::force_adjust_total_issuance::Direction,
                        delta: super::force_adjust_total_issuance::Delta,
                    ) -> ::subxt::transactions::StaticPayload<super::ForceAdjustTotalIssuance>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Balances",
                            "force_adjust_total_issuance",
                            super::ForceAdjustTotalIssuance { direction, delta },
                            [
                                208u8, 134u8, 56u8, 133u8, 232u8, 164u8, 10u8, 213u8, 53u8, 193u8,
                                190u8, 63u8, 236u8, 186u8, 96u8, 122u8, 104u8, 87u8, 173u8, 38u8,
                                58u8, 176u8, 21u8, 78u8, 42u8, 106u8, 46u8, 248u8, 251u8, 190u8,
                                150u8, 202u8,
                            ],
                        )
                    }
                    #[doc = "Burn the specified liquid free balance from the origin account."]
                    #[doc = ""]
                    #[doc = "If the origin's account ends up below the existential deposit as a result"]
                    #[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
                    #[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
                    pub fn burn(
                        &self,
                        value: super::burn::Value,
                        keep_alive: super::burn::KeepAlive,
                    ) -> ::subxt::transactions::StaticPayload<super::Burn> {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Balances",
                            "burn",
                            super::Burn { value, keep_alive },
                            [
                                176u8, 64u8, 7u8, 109u8, 16u8, 44u8, 145u8, 125u8, 147u8, 152u8,
                                130u8, 114u8, 221u8, 201u8, 150u8, 162u8, 118u8, 71u8, 52u8, 92u8,
                                240u8, 116u8, 203u8, 98u8, 5u8, 22u8, 43u8, 102u8, 94u8, 208u8,
                                101u8, 57u8,
                            ],
                        )
                    }
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: endowed::Account,
                pub free_balance: endowed::FreeBalance,
            }
            pub mod endowed {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
                pub type FreeBalance = ::core::primitive::u128;
            }
            impl Endowed {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Endowed";
            }
            impl ::subxt::events::DecodeAsEvent for Endowed {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: dust_lost::Account,
                pub amount: dust_lost::Amount,
            }
            pub mod dust_lost {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl DustLost {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "DustLost";
            }
            impl ::subxt::events::DecodeAsEvent for DustLost {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: transfer::From,
                pub to: transfer::To,
                pub amount: transfer::Amount,
            }
            pub mod transfer {
                use super::runtime_types;
                pub type From = ::subxt::utils::AccountId32;
                pub type To = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Transfer {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Transfer";
            }
            impl ::subxt::events::DecodeAsEvent for Transfer {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: balance_set::Who,
                pub free: balance_set::Free,
            }
            pub mod balance_set {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Free = ::core::primitive::u128;
            }
            impl BalanceSet {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "BalanceSet";
            }
            impl ::subxt::events::DecodeAsEvent for BalanceSet {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: reserved::Who,
                pub amount: reserved::Amount,
            }
            pub mod reserved {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Reserved {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Reserved";
            }
            impl ::subxt::events::DecodeAsEvent for Reserved {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: unreserved::Who,
                pub amount: unreserved::Amount,
            }
            pub mod unreserved {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Unreserved {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Unreserved";
            }
            impl ::subxt::events::DecodeAsEvent for Unreserved {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: reserve_repatriated::From,
                pub to: reserve_repatriated::To,
                pub amount: reserve_repatriated::Amount,
                pub destination_status: reserve_repatriated::DestinationStatus,
            }
            pub mod reserve_repatriated {
                use super::runtime_types;
                pub type From = ::subxt::utils::AccountId32;
                pub type To = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
                pub type DestinationStatus =
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
            }
            impl ReserveRepatriated {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "ReserveRepatriated";
            }
            impl ::subxt::events::DecodeAsEvent for ReserveRepatriated {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: deposit::Who,
                pub amount: deposit::Amount,
            }
            pub mod deposit {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Deposit {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Deposit";
            }
            impl ::subxt::events::DecodeAsEvent for Deposit {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: withdraw::Who,
                pub amount: withdraw::Amount,
            }
            pub mod withdraw {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Withdraw {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Withdraw";
            }
            impl ::subxt::events::DecodeAsEvent for Withdraw {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: slashed::Who,
                pub amount: slashed::Amount,
            }
            pub mod slashed {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Slashed {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Slashed";
            }
            impl ::subxt::events::DecodeAsEvent for Slashed {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was minted into an account."]
            pub struct Minted {
                pub who: minted::Who,
                pub amount: minted::Amount,
            }
            pub mod minted {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Minted {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Minted";
            }
            impl ::subxt::events::DecodeAsEvent for Minted {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some credit was balanced and added to the TotalIssuance."]
            pub struct MintedCredit {
                pub amount: minted_credit::Amount,
            }
            pub mod minted_credit {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl MintedCredit {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "MintedCredit";
            }
            impl ::subxt::events::DecodeAsEvent for MintedCredit {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was burned from an account."]
            pub struct Burned {
                pub who: burned::Who,
                pub amount: burned::Amount,
            }
            pub mod burned {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Burned {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Burned";
            }
            impl ::subxt::events::DecodeAsEvent for Burned {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some debt has been dropped from the Total Issuance."]
            pub struct BurnedDebt {
                pub amount: burned_debt::Amount,
            }
            pub mod burned_debt {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl BurnedDebt {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "BurnedDebt";
            }
            impl ::subxt::events::DecodeAsEvent for BurnedDebt {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was suspended from an account (it can be restored later)."]
            pub struct Suspended {
                pub who: suspended::Who,
                pub amount: suspended::Amount,
            }
            pub mod suspended {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Suspended {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Suspended";
            }
            impl ::subxt::events::DecodeAsEvent for Suspended {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was restored into an account."]
            pub struct Restored {
                pub who: restored::Who,
                pub amount: restored::Amount,
            }
            pub mod restored {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Restored {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Restored";
            }
            impl ::subxt::events::DecodeAsEvent for Restored {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was upgraded."]
            pub struct Upgraded {
                pub who: upgraded::Who,
            }
            pub mod upgraded {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl Upgraded {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Upgraded";
            }
            impl ::subxt::events::DecodeAsEvent for Upgraded {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
            pub struct Issued {
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl Issued {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Issued";
            }
            impl ::subxt::events::DecodeAsEvent for Issued {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
            pub struct Rescinded {
                pub amount: rescinded::Amount,
            }
            pub mod rescinded {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl Rescinded {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Rescinded";
            }
            impl ::subxt::events::DecodeAsEvent for Rescinded {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was locked."]
            pub struct Locked {
                pub who: locked::Who,
                pub amount: locked::Amount,
            }
            pub mod locked {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Locked {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Locked";
            }
            impl ::subxt::events::DecodeAsEvent for Locked {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was unlocked."]
            pub struct Unlocked {
                pub who: unlocked::Who,
                pub amount: unlocked::Amount,
            }
            pub mod unlocked {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Unlocked {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Unlocked";
            }
            impl ::subxt::events::DecodeAsEvent for Unlocked {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was frozen."]
            pub struct Frozen {
                pub who: frozen::Who,
                pub amount: frozen::Amount,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Frozen {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Frozen";
            }
            impl ::subxt::events::DecodeAsEvent for Frozen {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was thawed."]
            pub struct Thawed {
                pub who: thawed::Who,
                pub amount: thawed::Amount,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Thawed {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Thawed";
            }
            impl ::subxt::events::DecodeAsEvent for Thawed {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `TotalIssuance` was forcefully changed."]
            pub struct TotalIssuanceForced {
                pub old: total_issuance_forced::Old,
                pub new: total_issuance_forced::New,
            }
            pub mod total_issuance_forced {
                use super::runtime_types;
                pub type Old = ::core::primitive::u128;
                pub type New = ::core::primitive::u128;
            }
            impl TotalIssuanceForced {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "TotalIssuanceForced";
            }
            impl ::subxt::events::DecodeAsEvent for TotalIssuanceForced {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was placed on hold."]
            pub struct Held {
                pub reason: held::Reason,
                pub who: held::Who,
                pub amount: held::Amount,
            }
            pub mod held {
                use super::runtime_types;
                pub type Reason = runtime_types::solochain_template_runtime::RuntimeHoldReason;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Held {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Held";
            }
            impl ::subxt::events::DecodeAsEvent for Held {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Held balance was burned from an account."]
            pub struct BurnedHeld {
                pub reason: burned_held::Reason,
                pub who: burned_held::Who,
                pub amount: burned_held::Amount,
            }
            pub mod burned_held {
                use super::runtime_types;
                pub type Reason = runtime_types::solochain_template_runtime::RuntimeHoldReason;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl BurnedHeld {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "BurnedHeld";
            }
            impl ::subxt::events::DecodeAsEvent for BurnedHeld {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A transfer of `amount` on hold from `source` to `dest` was initiated."]
            pub struct TransferOnHold {
                pub reason: transfer_on_hold::Reason,
                pub source: transfer_on_hold::Source,
                pub dest: transfer_on_hold::Dest,
                pub amount: transfer_on_hold::Amount,
            }
            pub mod transfer_on_hold {
                use super::runtime_types;
                pub type Reason = runtime_types::solochain_template_runtime::RuntimeHoldReason;
                pub type Source = ::subxt::utils::AccountId32;
                pub type Dest = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl TransferOnHold {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "TransferOnHold";
            }
            impl ::subxt::events::DecodeAsEvent for TransferOnHold {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `transferred` balance is placed on hold at the `dest` account."]
            pub struct TransferAndHold {
                pub reason: transfer_and_hold::Reason,
                pub source: transfer_and_hold::Source,
                pub dest: transfer_and_hold::Dest,
                pub transferred: transfer_and_hold::Transferred,
            }
            pub mod transfer_and_hold {
                use super::runtime_types;
                pub type Reason = runtime_types::solochain_template_runtime::RuntimeHoldReason;
                pub type Source = ::subxt::utils::AccountId32;
                pub type Dest = ::subxt::utils::AccountId32;
                pub type Transferred = ::core::primitive::u128;
            }
            impl TransferAndHold {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "TransferAndHold";
            }
            impl ::subxt::events::DecodeAsEvent for TransferAndHold {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was released from hold."]
            pub struct Released {
                pub reason: released::Reason,
                pub who: released::Who,
                pub amount: released::Amount,
            }
            pub mod released {
                use super::runtime_types;
                pub type Reason = runtime_types::solochain_template_runtime::RuntimeHoldReason;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl Released {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Released";
            }
            impl ::subxt::events::DecodeAsEvent for Released {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An unexpected/defensive event was triggered."]
            pub struct Unexpected(pub unexpected::Field0);
            pub mod unexpected {
                use super::runtime_types;
                pub type Field0 = runtime_types::pallet_balances::pallet::UnexpectedKind;
            }
            impl Unexpected {
                const PALLET_NAME: &'static str = "Balances";
                const EVENT_NAME: &'static str = "Unexpected";
            }
            impl ::subxt::events::DecodeAsEvent for Unexpected {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
        }
        pub mod storage {
            use super::root_mod;
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), total_issuance::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Balances",
                        "TotalIssuance",
                        [
                            138u8, 120u8, 138u8, 119u8, 4u8, 166u8, 22u8, 216u8, 227u8, 249u8,
                            161u8, 193u8, 54u8, 68u8, 55u8, 74u8, 230u8, 68u8, 131u8, 253u8, 146u8,
                            73u8, 54u8, 85u8, 212u8, 83u8, 162u8, 188u8, 171u8, 5u8, 232u8, 21u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (),
                    inactive_issuance::Output,
                    ::subxt::utils::Yes,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "Balances",
                        "InactiveIssuance",
                        [
                            97u8, 194u8, 82u8, 3u8, 40u8, 108u8, 109u8, 245u8, 175u8, 189u8, 212u8,
                            193u8, 229u8, 82u8, 107u8, 169u8, 9u8, 176u8, 124u8, 102u8, 151u8,
                            98u8, 87u8, 194u8, 82u8, 130u8, 41u8, 137u8, 3u8, 230u8, 145u8, 58u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (account::input::Param0,),
                    account::Output,
                    ::subxt::utils::Maybe,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        [
                            14u8, 88u8, 174u8, 192u8, 241u8, 142u8, 159u8, 255u8, 178u8, 117u8,
                            55u8, 78u8, 218u8, 161u8, 146u8, 139u8, 170u8, 180u8, 187u8, 177u8,
                            89u8, 157u8, 91u8, 225u8, 90u8, 174u8, 247u8, 47u8, 47u8, 23u8, 234u8,
                            50u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                #[doc = ""]
                #[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn locks(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (locks::input::Param0,),
                    locks::Output,
                    ::subxt::utils::Maybe,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        [
                            201u8, 50u8, 65u8, 126u8, 43u8, 153u8, 207u8, 145u8, 240u8, 59u8,
                            160u8, 111u8, 144u8, 245u8, 193u8, 13u8, 227u8, 118u8, 72u8, 168u8,
                            37u8, 147u8, 139u8, 221u8, 36u8, 177u8, 202u8, 209u8, 152u8, 122u8,
                            250u8, 89u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                #[doc = ""]
                #[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn reserves(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (reserves::input::Param0,),
                    reserves::Output,
                    ::subxt::utils::Maybe,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        [
                            76u8, 220u8, 133u8, 100u8, 127u8, 174u8, 237u8, 103u8, 211u8, 104u8,
                            140u8, 100u8, 49u8, 169u8, 114u8, 112u8, 193u8, 115u8, 234u8, 160u8,
                            97u8, 104u8, 194u8, 47u8, 119u8, 136u8, 132u8, 196u8, 136u8, 121u8,
                            45u8, 161u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (holds::input::Param0,),
                    holds::Output,
                    ::subxt::utils::Maybe,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        [
                            127u8, 213u8, 56u8, 163u8, 150u8, 56u8, 111u8, 121u8, 191u8, 137u8,
                            59u8, 114u8, 75u8, 24u8, 70u8, 242u8, 138u8, 133u8, 215u8, 74u8, 230u8,
                            129u8, 232u8, 84u8, 145u8, 55u8, 37u8, 63u8, 69u8, 59u8, 121u8, 243u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (freezes::input::Param0,),
                    freezes::Output,
                    ::subxt::utils::Maybe,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        [
                            103u8, 248u8, 73u8, 236u8, 239u8, 124u8, 250u8, 210u8, 106u8, 200u8,
                            122u8, 101u8, 229u8, 47u8, 253u8, 152u8, 20u8, 59u8, 6u8, 172u8, 157u8,
                            83u8, 244u8, 197u8, 179u8, 15u8, 20u8, 168u8, 227u8, 104u8, 210u8,
                            76u8,
                        ],
                    )
                }
            }
            pub mod total_issuance {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::u128;
            }
            pub mod inactive_issuance {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::u128;
            }
            pub mod account {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub type Output =
                    runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>;
            }
            pub mod locks {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub type Output =
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >;
            }
            pub mod reserves {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub type Output = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    runtime_types::pallet_balances::types::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u128,
                    >,
                >;
            }
            pub mod holds {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub type Output = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    runtime_types::frame_support::traits::tokens::misc::IdAmount<
                        runtime_types::solochain_template_runtime::RuntimeHoldReason,
                        ::core::primitive::u128,
                    >,
                >;
            }
            pub mod freezes {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub type Output = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    runtime_types::frame_support::traits::tokens::misc::IdAmount<
                        runtime_types::solochain_template_runtime::RuntimeFreezeReason,
                        ::core::primitive::u128,
                    >,
                >;
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
                #[doc = ""]
                #[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
                #[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
                #[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
                #[doc = " behaviour if you set this to zero."]
                #[doc = ""]
                #[doc = " Bottom line: Do yourself a favour and make it at least one!"]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u128> {
                    ::subxt::constants::StaticAddress::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                #[doc = ""]
                #[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn max_locks(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u32> {
                    ::subxt::constants::StaticAddress::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                #[doc = ""]
                #[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn max_reserves(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u32> {
                    ::subxt::constants::StaticAddress::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
                pub fn max_freezes(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u32> {
                    ::subxt::constants::StaticAddress::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who`."]
            pub struct TransactionFeePaid {
                pub who: transaction_fee_paid::Who,
                pub actual_fee: transaction_fee_paid::ActualFee,
                pub tip: transaction_fee_paid::Tip,
            }
            pub mod transaction_fee_paid {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type ActualFee = ::core::primitive::u128;
                pub type Tip = ::core::primitive::u128;
            }
            impl TransactionFeePaid {
                const PALLET_NAME: &'static str = "TransactionPayment";
                const EVENT_NAME: &'static str = "TransactionFeePaid";
            }
            impl ::subxt::events::DecodeAsEvent for TransactionFeePaid {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
        }
        pub mod storage {
            use super::root_mod;
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (),
                    next_fee_multiplier::Output,
                    ::subxt::utils::Yes,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        [
                            54u8, 78u8, 153u8, 36u8, 231u8, 148u8, 27u8, 187u8, 224u8, 89u8, 193u8,
                            138u8, 18u8, 92u8, 61u8, 225u8, 78u8, 186u8, 175u8, 214u8, 45u8, 237u8,
                            65u8, 225u8, 177u8, 110u8, 113u8, 22u8, 164u8, 172u8, 191u8, 241u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), storage_version::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "TransactionPayment",
                        "StorageVersion",
                        [
                            102u8, 2u8, 115u8, 199u8, 149u8, 230u8, 163u8, 131u8, 198u8, 138u8,
                            203u8, 116u8, 26u8, 120u8, 43u8, 39u8, 234u8, 52u8, 229u8, 102u8,
                            194u8, 18u8, 44u8, 249u8, 84u8, 142u8, 217u8, 129u8, 80u8, 5u8, 194u8,
                            214u8,
                        ],
                    )
                }
                #[doc = " The `OnChargeTransaction` stores the withdrawn tx fee here."]
                #[doc = ""]
                #[doc = " Use `withdraw_txfee` and `remaining_txfee` to access from outside the crate."]
                pub fn tx_payment_credit(
                    &self,
                ) -> ::subxt::storage::StaticAddress<
                    (),
                    tx_payment_credit::Output,
                    ::subxt::utils::Yes,
                > {
                    ::subxt::storage::StaticAddress::new_static(
                        "TransactionPayment",
                        "TxPaymentCredit",
                        [
                            200u8, 46u8, 84u8, 207u8, 2u8, 81u8, 201u8, 150u8, 218u8, 189u8, 138u8,
                            151u8, 91u8, 194u8, 144u8, 2u8, 28u8, 38u8, 88u8, 233u8, 242u8, 207u8,
                            20u8, 172u8, 99u8, 167u8, 57u8, 12u8, 121u8, 0u8, 162u8, 148u8,
                        ],
                    )
                }
            }
            pub mod next_fee_multiplier {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::sp_arithmetic::fixed_point::FixedU128;
            }
            pub mod storage_version {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::pallet_transaction_payment::Releases;
            }
            pub mod tx_payment_credit {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = runtime_types::frame_support::traits::storage::NoDrop<
                    runtime_types::frame_support::traits::tokens::fungible::imbalance::Imbalance<
                        ::core::primitive::u128,
                    >,
                >;
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A fee multiplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multiplied by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::constants::StaticAddress<::core::primitive::u8> {
                    ::subxt::constants::StaticAddress::new_static(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the Sudo pallet."]
        pub type Error = runtime_types::pallet_sudo::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_sudo::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
            pub struct Sudo {
                pub call: ::subxt::alloc::boxed::Box<sudo::Call>,
            }
            pub mod sudo {
                use super::runtime_types;
                pub type Call = runtime_types::solochain_template_runtime::RuntimeCall;
            }
            impl Sudo {
                const PALLET_NAME: &'static str = "Sudo";
                const CALL_NAME: &'static str = "sudo";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for Sudo {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
            #[doc = "This function does not check the weight of the call, and instead allows the"]
            #[doc = "Sudo user to specify the weight of the call."]
            #[doc = ""]
            #[doc = "The dispatch origin for this call must be _Signed_."]
            pub struct SudoUncheckedWeight {
                pub call: ::subxt::alloc::boxed::Box<sudo_unchecked_weight::Call>,
                pub weight: sudo_unchecked_weight::Weight,
            }
            pub mod sudo_unchecked_weight {
                use super::runtime_types;
                pub type Call = runtime_types::solochain_template_runtime::RuntimeCall;
                pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
            }
            impl SudoUncheckedWeight {
                const PALLET_NAME: &'static str = "Sudo";
                const CALL_NAME: &'static str = "sudo_unchecked_weight";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for SudoUncheckedWeight {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
            #[doc = "key."]
            pub struct SetKey {
                pub new: set_key::New,
            }
            pub mod set_key {
                use super::runtime_types;
                pub type New = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
            }
            impl SetKey {
                const PALLET_NAME: &'static str = "Sudo";
                const CALL_NAME: &'static str = "set_key";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for SetKey {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
            #[doc = "a given account."]
            #[doc = ""]
            #[doc = "The dispatch origin for this call must be _Signed_."]
            pub struct SudoAs {
                pub who: sudo_as::Who,
                pub call: ::subxt::alloc::boxed::Box<sudo_as::Call>,
            }
            pub mod sudo_as {
                use super::runtime_types;
                pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                pub type Call = runtime_types::solochain_template_runtime::RuntimeCall;
            }
            impl SudoAs {
                const PALLET_NAME: &'static str = "Sudo";
                const CALL_NAME: &'static str = "sudo_as";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for SudoAs {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Permanently removes the sudo key."]
            #[doc = ""]
            #[doc = "**This cannot be un-done.**"]
            pub struct RemoveKey;
            impl RemoveKey {
                const PALLET_NAME: &'static str = "Sudo";
                const CALL_NAME: &'static str = "remove_key";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for RemoveKey {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            pub mod api {
                pub struct TransactionApi;
                impl TransactionApi {
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    pub fn sudo(
                        &self,
                        call: super::sudo::Call,
                    ) -> ::subxt::transactions::StaticPayload<super::Sudo> {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Sudo",
                            "sudo",
                            super::Sudo {
                                call: ::subxt::alloc::boxed::Box::new(call),
                            },
                            [
                                204u8, 117u8, 156u8, 144u8, 213u8, 29u8, 184u8, 8u8, 60u8, 81u8,
                                64u8, 54u8, 157u8, 209u8, 251u8, 232u8, 148u8, 93u8, 78u8, 226u8,
                                208u8, 81u8, 195u8, 18u8, 235u8, 248u8, 96u8, 134u8, 240u8, 68u8,
                                181u8, 230u8,
                            ],
                        )
                    }
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    pub fn sudo_unchecked_weight(
                        &self,
                        call: super::sudo_unchecked_weight::Call,
                        weight: super::sudo_unchecked_weight::Weight,
                    ) -> ::subxt::transactions::StaticPayload<super::SudoUncheckedWeight>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Sudo",
                            "sudo_unchecked_weight",
                            super::SudoUncheckedWeight {
                                call: ::subxt::alloc::boxed::Box::new(call),
                                weight,
                            },
                            [
                                96u8, 172u8, 230u8, 69u8, 80u8, 232u8, 8u8, 207u8, 149u8, 142u8,
                                92u8, 1u8, 183u8, 57u8, 87u8, 182u8, 214u8, 18u8, 71u8, 54u8,
                                184u8, 185u8, 218u8, 142u8, 104u8, 53u8, 116u8, 125u8, 107u8,
                                166u8, 163u8, 120u8,
                            ],
                        )
                    }
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    pub fn set_key(
                        &self,
                        new: super::set_key::New,
                    ) -> ::subxt::transactions::StaticPayload<super::SetKey> {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Sudo",
                            "set_key",
                            super::SetKey { new },
                            [
                                9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8,
                                8u8, 227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8,
                                226u8, 158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8,
                                92u8, 198u8,
                            ],
                        )
                    }
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    pub fn sudo_as(
                        &self,
                        who: super::sudo_as::Who,
                        call: super::sudo_as::Call,
                    ) -> ::subxt::transactions::StaticPayload<super::SudoAs> {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Sudo",
                            "sudo_as",
                            super::SudoAs {
                                who,
                                call: ::subxt::alloc::boxed::Box::new(call),
                            },
                            [
                                245u8, 123u8, 101u8, 188u8, 40u8, 72u8, 74u8, 217u8, 92u8, 89u8,
                                29u8, 190u8, 161u8, 52u8, 151u8, 161u8, 73u8, 61u8, 143u8, 85u8,
                                127u8, 164u8, 238u8, 21u8, 78u8, 233u8, 18u8, 211u8, 114u8, 209u8,
                                208u8, 66u8,
                            ],
                        )
                    }
                    #[doc = "Permanently removes the sudo key."]
                    #[doc = ""]
                    #[doc = "**This cannot be un-done.**"]
                    pub fn remove_key(
                        &self,
                    ) -> ::subxt::transactions::StaticPayload<super::RemoveKey>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Sudo",
                            "remove_key",
                            super::RemoveKey {},
                            [
                                133u8, 253u8, 54u8, 175u8, 202u8, 239u8, 5u8, 198u8, 180u8, 138u8,
                                25u8, 28u8, 109u8, 40u8, 30u8, 56u8, 126u8, 100u8, 52u8, 205u8,
                                250u8, 191u8, 61u8, 195u8, 172u8, 142u8, 184u8, 239u8, 247u8, 10u8,
                                211u8, 79u8,
                            ],
                        )
                    }
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A sudo call just took place."]
            pub struct Sudid {
                pub sudo_result: sudid::SudoResult,
            }
            pub mod sudid {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl Sudid {
                const PALLET_NAME: &'static str = "Sudo";
                const EVENT_NAME: &'static str = "Sudid";
            }
            impl ::subxt::events::DecodeAsEvent for Sudid {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The sudo key has been updated."]
            pub struct KeyChanged {
                pub old: key_changed::Old,
                pub new: key_changed::New,
            }
            pub mod key_changed {
                use super::runtime_types;
                pub type Old = ::core::option::Option<::subxt::utils::AccountId32>;
                pub type New = ::subxt::utils::AccountId32;
            }
            impl KeyChanged {
                const PALLET_NAME: &'static str = "Sudo";
                const EVENT_NAME: &'static str = "KeyChanged";
            }
            impl ::subxt::events::DecodeAsEvent for KeyChanged {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The key was permanently removed."]
            pub struct KeyRemoved;
            impl KeyRemoved {
                const PALLET_NAME: &'static str = "Sudo";
                const EVENT_NAME: &'static str = "KeyRemoved";
            }
            impl ::subxt::events::DecodeAsEvent for KeyRemoved {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
            pub struct SudoAsDone {
                pub sudo_result: sudo_as_done::SudoResult,
            }
            pub mod sudo_as_done {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl SudoAsDone {
                const PALLET_NAME: &'static str = "Sudo";
                const EVENT_NAME: &'static str = "SudoAsDone";
            }
            impl ::subxt::events::DecodeAsEvent for SudoAsDone {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
        }
        pub mod storage {
            use super::root_mod;
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), key::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Sudo",
                        "Key",
                        [
                            135u8, 9u8, 151u8, 148u8, 179u8, 127u8, 153u8, 3u8, 158u8, 91u8, 244u8,
                            242u8, 201u8, 85u8, 31u8, 10u8, 151u8, 125u8, 201u8, 113u8, 15u8,
                            104u8, 164u8, 246u8, 174u8, 14u8, 251u8, 184u8, 57u8, 223u8, 162u8,
                            139u8,
                        ],
                    )
                }
            }
            pub mod key {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::subxt::utils::AccountId32;
            }
        }
    }
    pub mod template {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Errors that can be returned by this pallet."]
        #[doc = ""]
        #[doc = "Errors tell users that something went wrong so it's important that their naming is"]
        #[doc = "informative. Similar to events, error documentation is added to a node's metadata so it's"]
        #[doc = "equally important that they have helpful documentation associated with them."]
        #[doc = ""]
        #[doc = "This type of runtime error can be up to 4 bytes in size should you want to return additional"]
        #[doc = "information."]
        pub type Error = runtime_types::pallet_template::pallet::Error;
        #[doc = "The pallet's dispatchable functions ([`Call`]s)."]
        #[doc = ""]
        #[doc = "Dispatchable functions allows users to interact with the pallet and invoke state changes."]
        #[doc = "These functions materialize as \"extrinsics\", which are often compared to transactions."]
        #[doc = "They must always return a `DispatchResult` and be annotated with a weight and call index."]
        #[doc = ""]
        #[doc = "The [`call_index`] macro is used to explicitly"]
        #[doc = "define an index for calls in the [`Call`] enum. This is useful for pallets that may"]
        #[doc = "introduce new dispatchables over time. If the order of a dispatchable changes, its index"]
        #[doc = "will also change which will break backwards compatibility."]
        #[doc = ""]
        #[doc = "The [`weight`] macro is used to assign a weight to each call."]
        pub type Call = runtime_types::pallet_template::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An example dispatchable that takes a single u32 value as a parameter, writes the value"]
            #[doc = "to storage and emits an event."]
            #[doc = ""]
            #[doc = "It checks that the _origin_ for this call is _Signed_ and returns a dispatch"]
            #[doc = "error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>"]
            pub struct DoSomething {
                pub something: do_something::Something,
            }
            pub mod do_something {
                use super::runtime_types;
                pub type Something = ::core::primitive::u32;
            }
            impl DoSomething {
                const PALLET_NAME: &'static str = "Template";
                const CALL_NAME: &'static str = "do_something";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for DoSomething {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An example dispatchable that may throw a custom error."]
            #[doc = ""]
            #[doc = "It checks that the caller is a signed origin and reads the current value from the"]
            #[doc = "`Something` storage item. If a current value exists, it is incremented by 1 and then"]
            #[doc = "written back to storage."]
            #[doc = ""]
            #[doc = "## Errors"]
            #[doc = ""]
            #[doc = "The function will return an error under the following conditions:"]
            #[doc = ""]
            #[doc = "- If no value has been set ([`Error::NoneValue`])"]
            #[doc = "- If incrementing the value in storage causes an arithmetic overflow"]
            #[doc = "  ([`Error::StorageOverflow`])"]
            pub struct CauseError;
            impl CauseError {
                const PALLET_NAME: &'static str = "Template";
                const CALL_NAME: &'static str = "cause_error";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for CauseError {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct VerifySr25519 {
                pub n: verify_sr25519::N,
                pub signer: verify_sr25519::Signer,
                pub msg: verify_sr25519::Msg,
                pub sig_scale: verify_sr25519::SigScale,
            }
            pub mod verify_sr25519 {
                use super::runtime_types;
                pub type N = ::core::primitive::u32;
                pub type Signer = ::subxt::utils::AccountId32;
                pub type Msg = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
                pub type SigScale = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl VerifySr25519 {
                const PALLET_NAME: &'static str = "Template";
                const CALL_NAME: &'static str = "verify_sr25519";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for VerifySr25519 {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct VerifyEcdsa {
                pub n: verify_ecdsa::N,
                pub signer: verify_ecdsa::Signer,
                pub msg: verify_ecdsa::Msg,
                pub sig_scale: verify_ecdsa::SigScale,
            }
            pub mod verify_ecdsa {
                use super::runtime_types;
                pub type N = ::core::primitive::u32;
                pub type Signer = ::subxt::utils::AccountId32;
                pub type Msg = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
                pub type SigScale = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl VerifyEcdsa {
                const PALLET_NAME: &'static str = "Template";
                const CALL_NAME: &'static str = "verify_ecdsa";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for VerifyEcdsa {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct VerifyDilithium {
                pub n: verify_dilithium::N,
                pub signer: verify_dilithium::Signer,
                pub msg: verify_dilithium::Msg,
                pub sig_scale: verify_dilithium::SigScale,
            }
            pub mod verify_dilithium {
                use super::runtime_types;
                pub type N = ::core::primitive::u32;
                pub type Signer = ::subxt::utils::AccountId32;
                pub type Msg = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
                pub type SigScale = ::subxt::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl VerifyDilithium {
                const PALLET_NAME: &'static str = "Template";
                const CALL_NAME: &'static str = "verify_dilithium";
            }
            impl ::subxt::extrinsics::DecodeAsExtrinsic for VerifyDilithium {
                fn is_extrinsic(pallet_name: &str, call_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && call_name == Self::CALL_NAME
                }
            }
            pub mod api {
                pub struct TransactionApi;
                impl TransactionApi {
                    #[doc = "An example dispatchable that takes a single u32 value as a parameter, writes the value"]
                    #[doc = "to storage and emits an event."]
                    #[doc = ""]
                    #[doc = "It checks that the _origin_ for this call is _Signed_ and returns a dispatch"]
                    #[doc = "error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>"]
                    pub fn do_something(
                        &self,
                        something: super::do_something::Something,
                    ) -> ::subxt::transactions::StaticPayload<super::DoSomething>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Template",
                            "do_something",
                            super::DoSomething { something },
                            [
                                236u8, 120u8, 161u8, 55u8, 61u8, 88u8, 224u8, 28u8, 174u8, 255u8,
                                69u8, 156u8, 44u8, 11u8, 255u8, 87u8, 183u8, 239u8, 120u8, 48u8,
                                187u8, 157u8, 151u8, 49u8, 105u8, 193u8, 12u8, 1u8, 183u8, 58u8,
                                73u8, 4u8,
                            ],
                        )
                    }
                    #[doc = "An example dispatchable that may throw a custom error."]
                    #[doc = ""]
                    #[doc = "It checks that the caller is a signed origin and reads the current value from the"]
                    #[doc = "`Something` storage item. If a current value exists, it is incremented by 1 and then"]
                    #[doc = "written back to storage."]
                    #[doc = ""]
                    #[doc = "## Errors"]
                    #[doc = ""]
                    #[doc = "The function will return an error under the following conditions:"]
                    #[doc = ""]
                    #[doc = "- If no value has been set ([`Error::NoneValue`])"]
                    #[doc = "- If incrementing the value in storage causes an arithmetic overflow"]
                    #[doc = "  ([`Error::StorageOverflow`])"]
                    pub fn cause_error(
                        &self,
                    ) -> ::subxt::transactions::StaticPayload<super::CauseError>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Template",
                            "cause_error",
                            super::CauseError {},
                            [
                                29u8, 131u8, 81u8, 134u8, 218u8, 173u8, 12u8, 104u8, 19u8, 10u8,
                                117u8, 192u8, 155u8, 3u8, 171u8, 11u8, 177u8, 248u8, 43u8, 252u8,
                                4u8, 46u8, 244u8, 69u8, 220u8, 51u8, 188u8, 75u8, 165u8, 41u8,
                                16u8, 188u8,
                            ],
                        )
                    }
                    pub fn verify_sr25519(
                        &self,
                        n: super::verify_sr25519::N,
                        signer: super::verify_sr25519::Signer,
                        msg: super::verify_sr25519::Msg,
                        sig_scale: super::verify_sr25519::SigScale,
                    ) -> ::subxt::transactions::StaticPayload<super::VerifySr25519>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Template",
                            "verify_sr25519",
                            super::VerifySr25519 {
                                n,
                                signer,
                                msg,
                                sig_scale,
                            },
                            [
                                45u8, 191u8, 17u8, 215u8, 152u8, 130u8, 228u8, 204u8, 109u8, 118u8,
                                41u8, 86u8, 142u8, 195u8, 226u8, 198u8, 182u8, 190u8, 182u8, 199u8,
                                113u8, 16u8, 196u8, 82u8, 41u8, 67u8, 7u8, 126u8, 36u8, 47u8,
                                162u8, 141u8,
                            ],
                        )
                    }
                    pub fn verify_ecdsa(
                        &self,
                        n: super::verify_ecdsa::N,
                        signer: super::verify_ecdsa::Signer,
                        msg: super::verify_ecdsa::Msg,
                        sig_scale: super::verify_ecdsa::SigScale,
                    ) -> ::subxt::transactions::StaticPayload<super::VerifyEcdsa>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Template",
                            "verify_ecdsa",
                            super::VerifyEcdsa {
                                n,
                                signer,
                                msg,
                                sig_scale,
                            },
                            [
                                170u8, 179u8, 4u8, 175u8, 21u8, 172u8, 225u8, 115u8, 99u8, 69u8,
                                255u8, 55u8, 133u8, 163u8, 68u8, 156u8, 254u8, 61u8, 150u8, 35u8,
                                221u8, 46u8, 117u8, 126u8, 147u8, 21u8, 142u8, 19u8, 212u8, 94u8,
                                141u8, 197u8,
                            ],
                        )
                    }
                    pub fn verify_dilithium(
                        &self,
                        n: super::verify_dilithium::N,
                        signer: super::verify_dilithium::Signer,
                        msg: super::verify_dilithium::Msg,
                        sig_scale: super::verify_dilithium::SigScale,
                    ) -> ::subxt::transactions::StaticPayload<super::VerifyDilithium>
                    {
                        ::subxt::transactions::StaticPayload::new_static(
                            "Template",
                            "verify_dilithium",
                            super::VerifyDilithium {
                                n,
                                signer,
                                msg,
                                sig_scale,
                            },
                            [
                                150u8, 98u8, 24u8, 147u8, 212u8, 78u8, 232u8, 65u8, 36u8, 132u8,
                                42u8, 91u8, 125u8, 11u8, 71u8, 88u8, 222u8, 151u8, 61u8, 177u8,
                                46u8, 149u8, 195u8, 247u8, 58u8, 22u8, 40u8, 235u8, 89u8, 230u8,
                                119u8, 255u8,
                            ],
                        )
                    }
                }
            }
        }
        #[doc = "Events that functions in this pallet can emit."]
        #[doc = ""]
        #[doc = "Events are a simple means of indicating to the outside world (such as dApps, chain explorers"]
        #[doc = "or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the"]
        #[doc = "documentation for each event field and its parameters is added to a node's metadata so it"]
        #[doc = "can be used by external interfaces or tools."]
        #[doc = ""]
        #[doc = "\tThe `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which"]
        #[doc = "will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's"]
        #[doc = "[`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`]."]
        pub type Event = runtime_types::pallet_template::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A user has successfully set a new value."]
            pub struct SomethingStored {
                pub something: something_stored::Something,
                pub who: something_stored::Who,
            }
            pub mod something_stored {
                use super::runtime_types;
                pub type Something = ::core::primitive::u32;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl SomethingStored {
                const PALLET_NAME: &'static str = "Template";
                const EVENT_NAME: &'static str = "SomethingStored";
            }
            impl ::subxt::events::DecodeAsEvent for SomethingStored {
                fn is_event(pallet_name: &str, event_name: &str) -> bool {
                    pallet_name == Self::PALLET_NAME && event_name == Self::EVENT_NAME
                }
            }
        }
        pub mod storage {
            use super::root_mod;
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " A storage item for this pallet."]
                #[doc = ""]
                #[doc = " In this template, we are declaring a storage item called `Something` that stores a single"]
                #[doc = " `u32` value. Learn more about runtime storage here: <https://docs.substrate.io/build/runtime-storage/>"]
                pub fn something(
                    &self,
                ) -> ::subxt::storage::StaticAddress<(), something::Output, ::subxt::utils::Yes>
                {
                    ::subxt::storage::StaticAddress::new_static(
                        "Template",
                        "Something",
                        [
                            89u8, 17u8, 227u8, 228u8, 49u8, 222u8, 111u8, 116u8, 186u8, 213u8,
                            164u8, 182u8, 185u8, 168u8, 197u8, 151u8, 130u8, 195u8, 131u8, 58u8,
                            87u8, 121u8, 5u8, 166u8, 249u8, 75u8, 152u8, 49u8, 193u8, 155u8, 140u8,
                            212u8,
                        ],
                    )
                }
            }
            pub mod something {
                use super::root_mod;
                use super::runtime_types;
                pub mod input {
                    use super::runtime_types;
                }
                pub type Output = ::core::primitive::u32;
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BoundedVec<_0>(pub ::subxt::alloc::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeakBoundedVec<_0>(pub ::subxt::alloc::vec::Vec<_0>);
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_metadata_hash_extension {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CheckMetadataHash {
                pub mode: runtime_types::frame_metadata_hash_extension::Mode,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Mode {
                #[codec(index = 0)]
                Disabled,
                #[codec(index = 1)]
                Enabled,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod storage {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct NoDrop<_0>(pub _0);
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod fungible {
                        use super::runtime_types;
                        pub mod imbalance {
                            use super::runtime_types;
                            #[derive(
                                :: subxt :: ext :: scale_decode :: DecodeAsType,
                                :: subxt :: ext :: scale_encode :: EncodeAsType,
                                Debug,
                            )]
                            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                            pub struct Imbalance<_0> {
                                pub amount: _0,
                            }
                        }
                    }
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                        #[derive(
                            :: subxt :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                        pub struct IdAmount<_0, _1> {
                            pub id: _0,
                            pub amount: _1,
                        }
                    }
                }
            }
            pub mod view_functions {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum ViewFunctionDispatchError {
                    #[codec(index = 0)]
                    NotImplemented,
                    #[codec(index = 1)]
                    NotFound(runtime_types::frame_support::view_functions::ViewFunctionId),
                    #[codec(index = 2)]
                    Codec,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ViewFunctionId {
                    pub prefix: [::core::primitive::u8; 16usize],
                    pub suffix: [::core::primitive::u8; 16usize],
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckWeight;
                }
                pub mod weight_reclaim {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct WeightReclaim;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "Can be executed by every `origin`."]
                    remark {
                        remark: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Set the new runtime code."]
                    set_code {
                        code: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                    #[doc = "version!"]
                    set_code_without_checks {
                        code: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::subxt::alloc::vec::Vec<(
                            ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                            ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::subxt::alloc::vec::Vec<
                            ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    authorize_upgrade { code_hash: ::subxt::utils::H256 },
                    #[codec(index = 10)]
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                    #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                    #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    authorize_upgrade_without_checks { code_hash: ::subxt::utils::H256 },
                    #[codec(index = 11)]
                    #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                    #[doc = ""]
                    #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                    #[doc = "remains unchanged and that the spec version has increased."]
                    #[doc = ""]
                    #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                    #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                    #[doc = ""]
                    #[doc = "All origins are allowed."]
                    apply_authorized_upgrade {
                        code: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                    #[codec(index = 6)]
                    #[doc = "A multi-block migration is ongoing and prevents the current code from being replaced."]
                    MultiBlockMigrationsOngoing,
                    #[codec(index = 7)]
                    #[doc = "No upgrade authorized."]
                    NothingAuthorized,
                    #[codec(index = 8)]
                    #[doc = "The submitted code is not authorized."]
                    Unauthorized,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_system::DispatchEventInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_system::DispatchEventInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::utils::AccountId32,
                        hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 6)]
                    #[doc = "An upgrade was authorized."]
                    UpgradeAuthorized {
                        code_hash: ::subxt::utils::H256,
                        check_version: ::core::primitive::bool,
                    },
                    #[codec(index = 7)]
                    #[doc = "An invalid authorized upgrade was rejected while trying to apply it."]
                    RejectedInvalidAuthorizedUpgrade {
                        code_hash: ::subxt::utils::H256,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CodeUpgradeAuthorization {
                pub code_hash: ::subxt::utils::H256,
                pub check_version: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct DispatchEventInfo {
                pub weight: runtime_types::sp_weights::weight_v2::Weight,
                pub class: runtime_types::frame_support::dispatch::DispatchClass,
                pub pays_fee: runtime_types::frame_support::dispatch::Pays,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::subxt::alloc::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::subxt::alloc::string::String,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    transfer_allow_death {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                    #[doc = "may be specified."]
                    force_transfer {
                        source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                    #[doc = "kill the origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true)."]
                    transfer_all {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Upgrade a specified account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed`."]
                    #[doc = "- `who`: The account to be upgraded."]
                    #[doc = ""]
                    #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                    #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                    #[doc = "possibility of churn)."]
                    upgrade_accounts {
                        who: ::subxt::alloc::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 8)]
                    #[doc = "Set the regular balance of a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    force_set_balance {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Adjust the total issuance in a saturating way."]
                    #[doc = ""]
                    #[doc = "Can only be called by root and always needs a positive `delta`."]
                    #[doc = ""]
                    #[doc = "# Example"]
                    force_adjust_total_issuance {
                        direction: runtime_types::pallet_balances::types::AdjustmentDirection,
                        #[codec(compact)]
                        delta: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Burn the specified liquid free balance from the origin account."]
                    #[doc = ""]
                    #[doc = "If the origin's account ends up below the existential deposit as a result"]
                    #[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
                    #[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
                    burn {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        keep_alive: ::core::primitive::bool,
                    },
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value."]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal."]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit."]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account."]
                    Expendability,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account."]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist."]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed `MaxReserves`."]
                    TooManyReserves,
                    #[codec(index = 8)]
                    #[doc = "Number of holds exceed `VariantCountOf<T::RuntimeHoldReason>`."]
                    TooManyHolds,
                    #[codec(index = 9)]
                    #[doc = "Number of freezes exceed `MaxFreezes`."]
                    TooManyFreezes,
                    #[codec(index = 10)]
                    #[doc = "The issuance cannot be modified since it is already deactivated."]
                    IssuanceDeactivated,
                    #[codec(index = 11)]
                    #[doc = "The delta cannot be zero."]
                    DeltaZero,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Some amount was minted into an account."]
                    Minted {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Some credit was balanced and added to the TotalIssuance."]
                    MintedCredit { amount: ::core::primitive::u128 },
                    #[codec(index = 12)]
                    #[doc = "Some amount was burned from an account."]
                    Burned {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    #[doc = "Some debt has been dropped from the Total Issuance."]
                    BurnedDebt { amount: ::core::primitive::u128 },
                    #[codec(index = 14)]
                    #[doc = "Some amount was suspended from an account (it can be restored later)."]
                    Suspended {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 15)]
                    #[doc = "Some amount was restored into an account."]
                    Restored {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 16)]
                    #[doc = "An account was upgraded."]
                    Upgraded { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 17)]
                    #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 18)]
                    #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 19)]
                    #[doc = "Some balance was locked."]
                    Locked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "Some balance was unlocked."]
                    Unlocked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 21)]
                    #[doc = "Some balance was frozen."]
                    Frozen {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 22)]
                    #[doc = "Some balance was thawed."]
                    Thawed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 23)]
                    #[doc = "The `TotalIssuance` was forcefully changed."]
                    TotalIssuanceForced {
                        old: ::core::primitive::u128,
                        new: ::core::primitive::u128,
                    },
                    #[codec(index = 24)]
                    #[doc = "Some balance was placed on hold."]
                    Held {
                        reason: runtime_types::solochain_template_runtime::RuntimeHoldReason,
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 25)]
                    #[doc = "Held balance was burned from an account."]
                    BurnedHeld {
                        reason: runtime_types::solochain_template_runtime::RuntimeHoldReason,
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 26)]
                    #[doc = "A transfer of `amount` on hold from `source` to `dest` was initiated."]
                    TransferOnHold {
                        reason: runtime_types::solochain_template_runtime::RuntimeHoldReason,
                        source: ::subxt::utils::AccountId32,
                        dest: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 27)]
                    #[doc = "The `transferred` balance is placed on hold at the `dest` account."]
                    TransferAndHold {
                        reason: runtime_types::solochain_template_runtime::RuntimeHoldReason,
                        source: ::subxt::utils::AccountId32,
                        dest: ::subxt::utils::AccountId32,
                        transferred: ::core::primitive::u128,
                    },
                    #[codec(index = 28)]
                    #[doc = "Some balance was released from hold."]
                    Released {
                        reason: runtime_types::solochain_template_runtime::RuntimeHoldReason,
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 29)]
                    #[doc = "An unexpected/defensive event was triggered."]
                    Unexpected(runtime_types::pallet_balances::pallet::UnexpectedKind),
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum UnexpectedKind {
                    #[codec(index = 0)]
                    BalanceUpdated,
                    #[codec(index = 1)]
                    FailedToMutateAccount,
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum AdjustmentDirection {
                    #[codec(index = 0)]
                    Increase,
                    #[codec(index = 1)]
                    Decrease,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    report_equivocation {
                        equivocation_proof: ::subxt::alloc::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
                    report_equivocation_unsigned {
                        equivocation_proof: ::subxt::alloc::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                    #[doc = ""]
                    #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                    #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                    #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                    #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                    #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                    #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                    #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                    #[doc = "block of all validators of the new authority set."]
                    #[doc = ""]
                    #[doc = "Only callable by root."]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                    #[doc = "(either paused or already pending pause)."]
                    PauseFailed,
                    #[codec(index = 1)]
                    #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                    #[doc = "(either live or already pending resume)."]
                    ResumeFailed,
                    #[codec(index = 2)]
                    #[doc = "Attempt to signal GRANDPA change with one already pending."]
                    ChangePending,
                    #[codec(index = 3)]
                    #[doc = "Cannot signal forced change so soon after last."]
                    TooSoon,
                    #[codec(index = 4)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New authority set has been applied."]
                    NewAuthorities {
                        authority_set: ::subxt::alloc::vec::Vec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Current authority set has been paused."]
                    Paused,
                    #[codec(index = 2)]
                    #[doc = "Current authority set has been resumed."]
                    Resumed,
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    sudo {
                        call: ::subxt::alloc::boxed::Box<
                            runtime_types::solochain_template_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    sudo_unchecked_weight {
                        call: ::subxt::alloc::boxed::Box<
                            runtime_types::solochain_template_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    set_key {
                        new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    sudo_as {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        call: ::subxt::alloc::boxed::Box<
                            runtime_types::solochain_template_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Permanently removes the sudo key."]
                    #[doc = ""]
                    #[doc = "**This cannot be un-done.**"]
                    remove_key,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the Sudo pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account."]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo call just took place."]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The sudo key has been updated."]
                    KeyChanged {
                        old: ::core::option::Option<::subxt::utils::AccountId32>,
                        new: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "The key was permanently removed."]
                    KeyRemoved,
                    #[codec(index = 3)]
                    #[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_template {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The pallet's dispatchable functions ([`Call`]s)."]
                #[doc = ""]
                #[doc = "Dispatchable functions allows users to interact with the pallet and invoke state changes."]
                #[doc = "These functions materialize as \"extrinsics\", which are often compared to transactions."]
                #[doc = "They must always return a `DispatchResult` and be annotated with a weight and call index."]
                #[doc = ""]
                #[doc = "The [`call_index`] macro is used to explicitly"]
                #[doc = "define an index for calls in the [`Call`] enum. This is useful for pallets that may"]
                #[doc = "introduce new dispatchables over time. If the order of a dispatchable changes, its index"]
                #[doc = "will also change which will break backwards compatibility."]
                #[doc = ""]
                #[doc = "The [`weight`] macro is used to assign a weight to each call."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "An example dispatchable that takes a single u32 value as a parameter, writes the value"]
                    #[doc = "to storage and emits an event."]
                    #[doc = ""]
                    #[doc = "It checks that the _origin_ for this call is _Signed_ and returns a dispatch"]
                    #[doc = "error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>"]
                    do_something { something: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "An example dispatchable that may throw a custom error."]
                    #[doc = ""]
                    #[doc = "It checks that the caller is a signed origin and reads the current value from the"]
                    #[doc = "`Something` storage item. If a current value exists, it is incremented by 1 and then"]
                    #[doc = "written back to storage."]
                    #[doc = ""]
                    #[doc = "## Errors"]
                    #[doc = ""]
                    #[doc = "The function will return an error under the following conditions:"]
                    #[doc = ""]
                    #[doc = "- If no value has been set ([`Error::NoneValue`])"]
                    #[doc = "- If incrementing the value in storage causes an arithmetic overflow"]
                    #[doc = "  ([`Error::StorageOverflow`])"]
                    cause_error,
                    #[codec(index = 2)]
                    verify_sr25519 {
                        n: ::core::primitive::u32,
                        signer: ::subxt::utils::AccountId32,
                        msg: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                        sig_scale: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    verify_ecdsa {
                        n: ::core::primitive::u32,
                        signer: ::subxt::utils::AccountId32,
                        msg: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                        sig_scale: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    verify_dilithium {
                        n: ::core::primitive::u32,
                        signer: ::subxt::utils::AccountId32,
                        msg: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                        sig_scale: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Errors that can be returned by this pallet."]
                #[doc = ""]
                #[doc = "Errors tell users that something went wrong so it's important that their naming is"]
                #[doc = "informative. Similar to events, error documentation is added to a node's metadata so it's"]
                #[doc = "equally important that they have helpful documentation associated with them."]
                #[doc = ""]
                #[doc = "This type of runtime error can be up to 4 bytes in size should you want to return additional"]
                #[doc = "information."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The value retrieved was `None` as no value was previously set."]
                    NoneValue,
                    #[codec(index = 1)]
                    #[doc = "There was an attempt to increment the value in storage over `u32::MAX`."]
                    StorageOverflow,
                    #[codec(index = 2)]
                    BadSignature,
                    #[codec(index = 3)]
                    BadSignatureEncoding,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Events that functions in this pallet can emit."]
                #[doc = ""]
                #[doc = "Events are a simple means of indicating to the outside world (such as dApps, chain explorers"]
                #[doc = "or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the"]
                #[doc = "documentation for each event field and its parameters is added to a node's metadata so it"]
                #[doc = "can be used by external interfaces or tools."]
                #[doc = ""]
                #[doc = "\tThe `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which"]
                #[doc = "will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's"]
                #[doc = "[`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`]."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A user has successfully set a new value."]
                    SomethingStored {
                        something: ::core::primitive::u32,
                        who: ::subxt::utils::AccountId32,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "[`Config::MinimumPeriod`]."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _None_."]
                    #[doc = ""]
                    #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                    #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                    #[doc = "block to execute any other calls."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."]
                    TransactionFeePaid {
                        who: ::subxt::utils::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FeeDetails<_0> {
                    pub inclusion_fee: ::core::option::Option<
                        runtime_types::pallet_transaction_payment::types::InclusionFee<_0>,
                    >,
                    pub tip: _0,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InclusionFee<_0> {
                    pub base_fee: _0,
                    pub len_fee: _0,
                    pub adjusted_weight_fee: _0,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RuntimeDispatchInfo<_0, _1> {
                    pub weight: _1,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub partial_fee: _0,
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod solochain_template_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 7)]
                Template(runtime_types::pallet_template::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Error),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
                #[codec(index = 7)]
                Template(runtime_types::pallet_template::pallet::Error),
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 5)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 7)]
                Template(runtime_types::pallet_template::pallet::Event),
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeFreezeReason {}
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeHoldReason {}
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Public(pub [::core::primitive::u8; 32usize]);
                }
            }
        }
        pub mod sp_consensus_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Slot(pub ::core::primitive::u64);
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct SlotDuration(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueMetadata(pub ::subxt::alloc::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Void {}
        }
        pub mod sp_inherents {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CheckInherentsResult {
                pub okay: ::core::primitive::bool,
                pub fatal_error: ::core::primitive::bool,
                pub errors: runtime_types::sp_inherents::InherentData,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InherentData {
                pub data: ::subxt::utils::KeyedVec<
                    [::core::primitive::u8; 8usize],
                    ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                >,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod block {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Block<_0, _1> {
                        pub header: _0,
                        pub extrinsics: ::subxt::alloc::vec::Vec<_1>,
                    }
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct LazyBlock<_0, _1> {
                        pub header: _0,
                        pub extrinsics:
                            ::subxt::alloc::vec::Vec<runtime_types::sp_runtime::OpaqueExtrinsic>,
                        #[codec(skip)]
                        pub __ignore: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Digest {
                        pub logs: ::subxt::alloc::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem,
                        >,
                    }
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::subxt::alloc::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Header<_0> {
                        pub parent_hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::utils::H256,
                        pub extrinsics_root: ::subxt::utils::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                    }
                }
            }
            pub mod proving_trie {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum TrieError {
                    #[codec(index = 0)]
                    InvalidStateRoot,
                    #[codec(index = 1)]
                    IncompleteDatabase,
                    #[codec(index = 2)]
                    ValueAtIncompleteKey,
                    #[codec(index = 3)]
                    DecoderError,
                    #[codec(index = 4)]
                    InvalidHash,
                    #[codec(index = 5)]
                    DuplicateKey,
                    #[codec(index = 6)]
                    ExtraneousNode,
                    #[codec(index = 7)]
                    ExtraneousValue,
                    #[codec(index = 8)]
                    ExtraneousHashReference,
                    #[codec(index = 9)]
                    InvalidChildReference,
                    #[codec(index = 10)]
                    ValueMismatch,
                    #[codec(index = 11)]
                    IncompleteProof,
                    #[codec(index = 12)]
                    RootMismatch,
                    #[codec(index = 13)]
                    DecodeError,
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlakeTwo256;
            }
            pub mod transaction_validity {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum InvalidTransaction {
                    #[codec(index = 0)]
                    Call,
                    #[codec(index = 1)]
                    Payment,
                    #[codec(index = 2)]
                    Future,
                    #[codec(index = 3)]
                    Stale,
                    #[codec(index = 4)]
                    BadProof,
                    #[codec(index = 5)]
                    AncientBirthBlock,
                    #[codec(index = 6)]
                    ExhaustsResources,
                    #[codec(index = 7)]
                    Custom(::core::primitive::u8),
                    #[codec(index = 8)]
                    BadMandatory,
                    #[codec(index = 9)]
                    MandatoryValidation,
                    #[codec(index = 10)]
                    BadSigner,
                    #[codec(index = 11)]
                    IndeterminateImplicit,
                    #[codec(index = 12)]
                    UnknownOrigin,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum TransactionSource {
                    #[codec(index = 0)]
                    InBlock,
                    #[codec(index = 1)]
                    Local,
                    #[codec(index = 2)]
                    External,
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum TransactionValidityError {
                    #[codec(index = 0)]
                    Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
                    #[codec(index = 1)]
                    Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum UnknownTransaction {
                    #[codec(index = 0)]
                    CannotLookup,
                    #[codec(index = 1)]
                    NoUnsignedValidator,
                    #[codec(index = 2)]
                    Custom(::core::primitive::u8),
                }
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidTransaction {
                    pub priority: ::core::primitive::u64,
                    pub requires:
                        ::subxt::alloc::vec::Vec<::subxt::alloc::vec::Vec<::core::primitive::u8>>,
                    pub provides:
                        ::subxt::alloc::vec::Vec<::subxt::alloc::vec::Vec<::core::primitive::u8>>,
                    pub longevity: ::core::primitive::u64,
                    pub propagate: ::core::primitive::bool,
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct DilithiumMultiSig {
                pub signature: [::core::primitive::u8; 4627usize],
                pub public: [::core::primitive::u8; 2592usize],
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
                #[codec(index = 14)]
                Trie(runtime_types::sp_runtime::proving_trie::TrieError),
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ExtrinsicInclusionMode {
                #[codec(index = 0)]
                AllExtrinsics,
                #[codec(index = 1)]
                OnlyInherents,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Falcon1024MultiSig {
                pub signature: [::core::primitive::u8; 1577usize],
                pub public: [::core::primitive::u8; 1793usize],
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Falcon512MultiSig {
                pub signature: [::core::primitive::u8; 809usize],
                pub public: [::core::primitive::u8; 897usize],
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519([::core::primitive::u8; 64usize]),
                #[codec(index = 1)]
                Sr25519([::core::primitive::u8; 64usize]),
                #[codec(index = 2)]
                Ecdsa([::core::primitive::u8; 65usize]),
                #[codec(index = 3)]
                Eth([::core::primitive::u8; 65usize]),
                #[codec(index = 4)]
                Dilithium(runtime_types::sp_runtime::DilithiumMultiSig),
                #[codec(index = 5)]
                Falcon512(runtime_types::sp_runtime::Falcon512MultiSig),
                #[codec(index = 6)]
                Falcon1024(runtime_types::sp_runtime::Falcon1024MultiSig),
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueExtrinsic(pub ::subxt::alloc::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueValue(pub ::subxt::alloc::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_session {
            use super::runtime_types;
            pub mod runtime_api {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct OpaqueGeneratedSessionKeys {
                    pub keys: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                    pub proof: ::subxt::alloc::vec::Vec<::core::primitive::u8>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::subxt::alloc::string::String,
                pub impl_name: ::subxt::alloc::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis: ::subxt::alloc::vec::Vec<(
                    [::core::primitive::u8; 8usize],
                    ::core::primitive::u32,
                )>,
                pub transaction_version: ::core::primitive::u32,
                pub system_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
    }
}
