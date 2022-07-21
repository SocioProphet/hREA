/**
 * Holo-REA fulfillment zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    MaybeUndefined,
    record_interface::Updateable,
    generate_record_entry,
};
use vf_measurement::QuantityValue;

pub use vf_attributes_hdk::{
    FulfillmentAddress,
    EconomicEventAddress,
    CommitmentAddress,
};

use hc_zome_rea_fulfillment_rpc::{ CreateRequest, UpdateRequest };

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlicePlanning {
    pub fulfillment: FulfillmentZomeConfigPlanning,
}

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSliceObservation {
    pub fulfillment: FulfillmentZomeConfigObservation,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct FulfillmentZomeConfigPlanning {
    pub commitment_index_zome: String,
    pub index_zome: String,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct FulfillmentZomeConfigObservation {
    pub economic_event_index_zome: String,
    pub index_zome: String,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct EntryData {
    pub fulfilled_by: EconomicEventAddress,
    pub fulfills: CommitmentAddress,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub note: Option<String>,
    pub _nonce: Bytes,
}

generate_record_entry!(EntryData, FulfillmentAddress, EntryStorage);

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl TryFrom<CreateRequest> for EntryData {
    type Error = DataIntegrityError;

    fn try_from(e: CreateRequest) -> RecordAPIResult<EntryData> {
        Ok(EntryData {
            fulfilled_by: e.fulfilled_by.into(),
            fulfills: e.fulfills.into(),
            resource_quantity: e.resource_quantity.into(),
            effort_quantity: e.effort_quantity.into(),
            note: e.note.into(),
            _nonce: if e.nonce.is_none_or_undefined() { random_bytes(32)? } else { e.nonce.unwrap() },
        })
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
            fulfilled_by: match &e.fulfilled_by {
                MaybeUndefined::Some(fulfilled_by) => fulfilled_by.clone(),
                _ => self.fulfilled_by.clone(),
            },
            fulfills: match &e.fulfills {
                MaybeUndefined::Some(fulfills) => fulfills.clone(),
                _ => self.fulfills.clone(),
            },
            resource_quantity: if e.resource_quantity== MaybeUndefined::Undefined { self.resource_quantity.clone() } else { e.resource_quantity.clone().into() },
            effort_quantity: if e.effort_quantity== MaybeUndefined::Undefined { self.effort_quantity.clone() } else { e.effort_quantity.clone().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.clone() } else { e.note.clone().into() },
            _nonce: self._nonce.to_owned(),
        }
    }
}
