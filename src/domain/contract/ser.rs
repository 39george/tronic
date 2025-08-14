use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

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

impl Serialize for super::Contract {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Contract", 2)?;
        let flat_mapping = self.to_flat_mapping();
        for (key, value) in flat_mapping {
            state.serialize_field(&key, &value)?;
        }
        state.end()
        serializer.serialize
    }
}
