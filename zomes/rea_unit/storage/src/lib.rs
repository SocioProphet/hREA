/**
 * Holo-REA measurement unit zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    generate_record_entry,
    record_interface::{ Updateable },
};

use hc_zome_rea_unit_rpc::{ CreateRequest, UpdateRequest };

pub use vf_attributes_hdk::{ UnitInternalAddress };

// :SHONK: needed as re-export in zome logic to allow validation logic to parse entries
pub use hdk_records::record_interface::Identified;

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub unit: UnitZomeConfig,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct UnitZomeConfig {
    pub index_zome: String,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Default, Clone)]
pub struct EntryData {
    pub label: String,
    pub symbol: String,
}

impl<'a> EntryData {
    pub fn get_symbol(&'a self) -> String {
        self.symbol.to_owned()
    }
}

generate_record_entry!(EntryData, UnitInternalAddress, EntryStorage);

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl TryFrom<CreateRequest> for EntryData {
    type Error = DataIntegrityError;

    fn try_from(e: CreateRequest) -> RecordAPIResult<EntryData> {
        Ok(EntryData {
            label: e.label.into(),
            symbol: e.symbol.into(),
        })
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
            label:   if !e.label.is_some()   { self.label.to_owned()   } else { e.label.to_owned().unwrap() },
            symbol: if !e.symbol.is_some() { self.symbol.to_owned() } else { e.symbol.to_owned().unwrap() },
        }
    }
}
