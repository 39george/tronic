use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use crate::domain::Message;
use crate::domain::contract::ContractType;
use crate::domain::contract::TransferAssetContract;
use crate::domain::contract::TransferContract;

//  {
//    parameter: {
//      value: {
//        amount: 1000,
//        owner_address: '41608f8da72479edc7dd921e4c30bb7e7cddbe722e',
//        to_address: '41e9d79cc47518930bc322d9bf7cddd260a0260a8d'
//      },
//      type_url: 'type.googleapis.com/protocol.TransferContract'
//    },
//    type: 'TransferContract'
//  }

#[derive(Serialize, Deserialize)]
struct Parameter {
    #[serde(rename = "type_url")]
    type_url: String,
    value: serde_json::Value,
}

impl Serialize for super::Contract {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pb_contract: crate::protocol::transaction::Contract =
            self.to_owned().into();
        let type_url = format!(
            "type.googleapis.com/protocol.{}",
            pb_contract.r#type().as_str_name()
        );

        let value = match &self.contract_type {
            ContractType::TransferContract(t) => {
                // serde_json::to_value(t).map_err(de::Error::custom)?
                serde_json::to_value(t).unwrap()
            }
            ContractType::TransferAssetContract(t) => {
                // serde_json::to_value(t).map_err(de::Error::custom)?
                serde_json::to_value(t).unwrap()
            }
            // ... handle all other variants
            _ => serde_json::Value::Null,
        };

        let mut state = serializer.serialize_struct("Contract", 2)?;
        state.serialize_field("parameter", &Parameter { type_url, value })?;
        state.serialize_field(
            "type",
            &match &self.contract_type {
                ContractType::TransferContract(_) => "TransferContract",
                ContractType::TransferAssetContract(_) => {
                    "TransferAssetContract"
                }
                // ... all other variants
                _ => "UnknownContract",
            },
        )?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for super::Contract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ContractHelper {
            parameter: Parameter,
            #[serde(rename = "type")]
            contract_type_str: String,
        }

        let helper = ContractHelper::deserialize(deserializer)?;

        let contract_type = match helper.contract_type_str.as_str() {
            "TransferContract" => {
                let value: TransferContract =
                    serde_json::from_value(helper.parameter.value).unwrap();
                ContractType::TransferContract(value)
            }
            "TransferAssetContract" => {
                let value: TransferAssetContract =
                    serde_json::from_value(helper.parameter.value).unwrap();
                ContractType::TransferAssetContract(value)
            }
            // ... handle all other variants
            _ => {
                return Err(serde::de::Error::unknown_variant(
                    &helper.contract_type_str,
                    &[
                        "TransferContract",
                        "TransferAssetContract",
                        // ... all other variant names
                    ],
                ));
            }
        };

        Ok(super::Contract {
            contract_type,
            provider: Vec::new(), // These fields aren't in the JSON
            contract_name: Message::default(),
            permission_id: 0,
        })
    }
}
