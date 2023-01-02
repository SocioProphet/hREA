use chrono::{DateTime, NaiveDate, NaiveDateTime, Timelike, Datelike, Utc};
use hdk::prelude::*;

use crate::{
    INDEX_DEPTH, CHUNK_INTERVAL, HAS_CHUNK_LEAVES,
    IndexType, TimeIndexResult, TimeIndexingError,
};

/// An index segment stores a wrapped unsigned int representing the timestamp on the DHT
///
// TODO: this entry type should be defined in the index_integrity zome

#[hdk_entry_defs]
#[unit_enum(UnitEntryType)]
pub enum EntryTypes {
    IndexSegment(IndexSegment),
}

// does this need an entry def id of "time_index"
#[hdk_entry_helper]
#[derive(Clone)]
pub struct IndexSegment(u64);

impl IndexSegment {
    /// Generate an index segment by truncating a timestamp (in ms)
    /// from the input `DateTime<Utc>` to the given `granularity`
    ///
    /// :TODO: update this method to handle out of range errors more gracefully
    /// (will currently panic due to unwrapping a `None` value)
    ///
    pub fn new(from: &DateTime<Utc>, granularity: &IndexType) -> Self {
        let truncated = match granularity {
            IndexType::Year => NaiveDate::from_ymd_opt(from.year(), 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            IndexType::Month => NaiveDate::from_ymd_opt(from.year(), from.month(), 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            IndexType::Day => NaiveDate::from_ymd_opt(from.year(), from.month(), from.day()).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            IndexType::Hour => NaiveDate::from_ymd_opt(from.year(), from.month(), from.day()).unwrap()
                .and_hms_opt(from.hour(), 0, 0).unwrap(),
            IndexType::Minute => NaiveDate::from_ymd_opt(from.year(), from.month(), from.day()).unwrap()
                .and_hms_opt(from.hour(), from.minute(), 0).unwrap(),
            IndexType::Second => NaiveDate::from_ymd_opt(from.year(), from.month(), from.day()).unwrap()
                .and_hms_opt(from.hour(), from.minute(), from.second()).unwrap(),
        };

        Self(truncated.timestamp_millis() as u64)
    }

    /// Generate an index segment corresponding to the closest leaf chunk for the given timestamp
    ///
    pub fn new_chunk(based_off: u64, from: &DateTime<Utc>) -> Self {
        let from_millis = from.timestamp_millis() as u64;
        let chunk_millis = CHUNK_INTERVAL.as_millis() as u64;
        let diff = from_millis - based_off;
        Self(based_off + ((diff / chunk_millis) * chunk_millis))
    }

    /// Generate a virtual index segment for an exact time, to use with final referencing link tag
    ///
    pub fn leafmost_link(from: &DateTime<Utc>) -> Self {
        Self(from.timestamp_millis() as u64)
    }

    /// :SHONK: clone the `IndexSegment`. For some reason to_owned() is returning a ref?
    pub fn cloned(&self) -> Self {
        Self(self.0)
    }

    /// return the raw timestamp of this `IndexSegment`
    pub fn timestamp(&self) -> u64 {
        self.0
    }

    /// Generate a `LinkTag` with encoded time of this index, suitable for linking from
    /// other entries in the index tree rooted at `index_name`.
    ///
    pub fn tag_for_index<I>(&self, index_name: &I) -> LinkTag
        where I: AsRef<str>,
    {
        LinkTag::new([
            index_name.as_ref().as_bytes(), // prefix with index ID
            &[0x0 as u8],                   // null byte separator
            &self.timestamp().to_be_bytes() // raw timestamp bytes encoded for sorting
        ].concat())
    }

    /// What is the hash for the current [ `IndexSegment` ]?
    pub fn hash(&self) -> TimeIndexResult<EntryHash> {
        Ok(hash_entry(self.to_owned())?)
    }

    /// Does an entry exist at the hash we expect?
    pub fn exists(&self) -> TimeIndexResult<bool> {
        Ok(get(self.hash()?, GetOptions::content())?.is_some())
    }
}

/// :TODO: update this method to handle out of range errors more gracefully
/// (will currently panic due to unwrapping a `None` value)
///
impl Into<DateTime<Utc>> for IndexSegment {
    fn into(self) -> DateTime<Utc> {
        let ts_millis = self.0;
        let ts_secs = ts_millis / 1000;
        let ts_ns = (ts_millis % 1000) * 1_000_000;
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(ts_secs as i64, ts_ns as u32).unwrap(), Utc)
    }
}

impl TryFrom<LinkTag> for IndexSegment {
    type Error = TimeIndexingError;

    fn try_from(l: LinkTag) -> Result<Self, Self::Error> {
        Ok(Self::leafmost_link(&decode_link_tag_timestamp(l)?))
    }
}

/// Generate a list of `IndexSegment` representing nodes in a radix trie for the given `time`.
/// The segments are returned in order of granularity, with least granular first.
///
pub (crate) fn get_index_segments(time: &DateTime<Utc>) -> Vec<IndexSegment> {
    let mut segments = vec![];

    // build main segments
    if INDEX_DEPTH.contains(&IndexType::Year) {
        segments.push(IndexSegment::new(&time, &IndexType::Year));
    }
    if INDEX_DEPTH.contains(&IndexType::Month) {
        segments.push(IndexSegment::new(&time, &IndexType::Month));
    }
    if INDEX_DEPTH.contains(&IndexType::Day) {
        segments.push(IndexSegment::new(&time, &IndexType::Day));
    }
    if INDEX_DEPTH.contains(&IndexType::Hour) {
        segments.push(IndexSegment::new(&time, &IndexType::Hour));
    }
    if INDEX_DEPTH.contains(&IndexType::Minute) {
        segments.push(IndexSegment::new(&time, &IndexType::Minute));
    }
    if INDEX_DEPTH.contains(&IndexType::Second) {
        segments.push(IndexSegment::new(&time, &IndexType::Second));
    }

    // add remainder chunk segment if it doesn't round evenly
    if *HAS_CHUNK_LEAVES {
        segments.push(IndexSegment::new_chunk(segments.last().unwrap().timestamp(), &time));
    }

    segments
}

/// Decode a timestamp from a time index link tag.
///
/// Returns a `TimeIndexingError::Malformed` if an invalid link tag is passed.
///
/// :TODO: update this method to handle out of range errors more gracefully
/// (will currently panic due to unwrapping a `None` value)
///
fn decode_link_tag_timestamp(tag: LinkTag) -> TimeIndexResult<DateTime<Utc>> {
    // take the raw bytes of the LinkTag and split on the first null byte separator. All bytes following are the timestamp as u64.
    let bits: Vec<&[u8]> = tag.as_ref().splitn(2, |byte| { *byte == 0x0 as u8 }).collect();

    // return an error on any invalid format
    let time_bytes = match bits.len() {
        2 => bits.last().ok_or(TimeIndexingError::Malformed(tag.as_ref().to_owned())),
        _ => Err(TimeIndexingError::Malformed(tag.as_ref().to_owned())),
    }?;

    // interpret time data and construct a DateTime<Utc> from it
    let ts_millis = u64::from_be_bytes(time_bytes.to_owned().try_into().map_err(|_e| { TimeIndexingError::Malformed(tag.as_ref().to_owned()) })?);
    let ts_secs = ts_millis / 1000;
    let ts_ns = (ts_millis % 1000) * 1_000_000;

    Ok(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(ts_secs as i64, ts_ns as u32).unwrap(), Utc))
}
