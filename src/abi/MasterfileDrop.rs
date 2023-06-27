    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod functions {
        use super::INTERNAL_ERR;
    }
    /// Contract's events.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct DropCreated {
            pub creator: Vec<u8>,
            pub name: String,
            pub symbol: String,
            pub series: String,
            pub volume: substreams::scalar::BigInt,
        }
        impl DropCreated {
            const TOPIC_ID: [u8; 32] = [
                46u8,
                4u8,
                125u8,
                66u8,
                70u8,
                18u8,
                197u8,
                211u8,
                217u8,
                227u8,
                161u8,
                20u8,
                103u8,
                106u8,
                246u8,
                37u8,
                128u8,
                31u8,
                14u8,
                215u8,
                115u8,
                98u8,
                243u8,
                57u8,
                234u8,
                173u8,
                89u8,
                140u8,
                231u8,
                220u8,
                73u8,
                69u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 224usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::String,
                            ethabi::ParamType::String,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    creator: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'creator' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    symbol: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    series: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    volume: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for DropCreated {
            const NAME: &'static str = "DropCreated";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DropSaleWindowSet {
            pub drop_start_date: substreams::scalar::BigInt,
            pub drop_end_date: substreams::scalar::BigInt,
        }
        impl DropSaleWindowSet {
            const TOPIC_ID: [u8; 32] = [
                67u8,
                219u8,
                98u8,
                164u8,
                107u8,
                229u8,
                248u8,
                129u8,
                144u8,
                46u8,
                202u8,
                209u8,
                53u8,
                71u8,
                228u8,
                138u8,
                129u8,
                67u8,
                13u8,
                250u8,
                176u8,
                20u8,
                86u8,
                218u8,
                75u8,
                113u8,
                117u8,
                53u8,
                236u8,
                44u8,
                42u8,
                126u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    drop_start_date: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    drop_end_date: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for DropSaleWindowSet {
            const NAME: &'static str = "DropSaleWindowSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EditionSet {
            pub edition_number: substreams::scalar::BigInt,
            pub arweave_cid: [u8; 32usize],
            pub recipient: Vec<u8>,
        }
        impl EditionSet {
            const TOPIC_ID: [u8; 32] = [
                220u8,
                85u8,
                91u8,
                93u8,
                124u8,
                122u8,
                17u8,
                80u8,
                171u8,
                176u8,
                38u8,
                186u8,
                120u8,
                89u8,
                245u8,
                67u8,
                203u8,
                188u8,
                154u8,
                177u8,
                152u8,
                182u8,
                112u8,
                44u8,
                18u8,
                87u8,
                143u8,
                88u8,
                138u8,
                102u8,
                164u8,
                67u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    edition_number: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(24usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'edition_number' from topic of type 'uint24': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    arweave_cid: {
                        let mut result = [0u8; 32];
                        let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(32usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'arweave_cid' from topic of type 'bytes32': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    recipient: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'recipient' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for EditionSet {
            const NAME: &'static str = "EditionSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ListingSet {
            pub edition_number: substreams::scalar::BigInt,
            pub quantity: substreams::scalar::BigInt,
            pub price: substreams::scalar::BigInt,
        }
        impl ListingSet {
            const TOPIC_ID: [u8; 32] = [
                172u8,
                167u8,
                164u8,
                67u8,
                185u8,
                41u8,
                4u8,
                215u8,
                209u8,
                122u8,
                175u8,
                105u8,
                213u8,
                122u8,
                255u8,
                59u8,
                110u8,
                122u8,
                9u8,
                4u8,
                181u8,
                176u8,
                84u8,
                134u8,
                183u8,
                143u8,
                96u8,
                79u8,
                58u8,
                212u8,
                237u8,
                192u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    edition_number: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(24usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'edition_number' from topic of type 'uint24': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    quantity: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    price: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ListingSet {
            const NAME: &'static str = "ListingSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PrimarySale {
            pub buyer: Vec<u8>,
            pub recipient: Vec<u8>,
            pub edition_number: substreams::scalar::BigInt,
            pub quantity: substreams::scalar::BigInt,
            pub total_price: substreams::scalar::BigInt,
        }
        impl PrimarySale {
            const TOPIC_ID: [u8; 32] = [
                55u8,
                204u8,
                156u8,
                86u8,
                192u8,
                185u8,
                162u8,
                102u8,
                177u8,
                12u8,
                179u8,
                214u8,
                31u8,
                199u8,
                12u8,
                82u8,
                85u8,
                197u8,
                45u8,
                24u8,
                80u8,
                199u8,
                227u8,
                213u8,
                28u8,
                154u8,
                102u8,
                69u8,
                167u8,
                181u8,
                204u8,
                231u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    buyer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'buyer' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    recipient: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'recipient' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    edition_number: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(24usize)],
                                log.topics[3usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'edition_number' from topic of type 'uint24': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    quantity: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    total_price: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for PrimarySale {
            const NAME: &'static str = "PrimarySale";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TokenEditionSet {
            pub from_token_id: substreams::scalar::BigInt,
            pub to_token_id: substreams::scalar::BigInt,
            pub edition_number: substreams::scalar::BigInt,
        }
        impl TokenEditionSet {
            const TOPIC_ID: [u8; 32] = [
                235u8,
                145u8,
                142u8,
                28u8,
                224u8,
                218u8,
                95u8,
                75u8,
                77u8,
                87u8,
                172u8,
                251u8,
                234u8,
                175u8,
                146u8,
                91u8,
                220u8,
                66u8,
                233u8,
                95u8,
                119u8,
                7u8,
                247u8,
                175u8,
                225u8,
                86u8,
                193u8,
                136u8,
                153u8,
                85u8,
                24u8,
                7u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    from_token_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'from_token_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    to_token_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'to_token_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    edition_number: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(24usize)],
                                log.topics[3usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'edition_number' from topic of type 'uint24': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for TokenEditionSet {
            const NAME: &'static str = "TokenEditionSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }