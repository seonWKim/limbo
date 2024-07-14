use crate::types::{Cursor, CursorResult, OwnedRecord, OwnedValue};
use anyhow::Result;
use log::trace;
use ordered_multimap::ListOrderedMultimap;
use std::cell::{Ref, RefCell};
use crate::sqlite3_ondisk::read_value;

pub struct Sorter {
    records: ListOrderedMultimap<String, OwnedRecord>,
    current: RefCell<Option<OwnedRecord>>,
}

impl Sorter {
    pub fn new() -> Self {
        Self {
            records: ListOrderedMultimap::new(),
            current: RefCell::new(None),
        }
    }

    pub fn insert(&mut self, key: String, record: OwnedRecord) {
        self.records.insert(key, record);
    }
}

impl Cursor for Sorter {
    fn is_empty(&self) -> bool {
        self.current.borrow().is_none()
    }

    fn rewind(&mut self) -> Result<CursorResult<()>> {
        let current = self.records.pop_front();
        match current {
            Some((_, record)) => {
                *self.current.borrow_mut() = Some(record);
            }
            None => {
                *self.current.borrow_mut() = None;
            }
        };
        Ok(CursorResult::Ok(()))
    }

    fn next(&mut self) -> Result<CursorResult<()>> {
        let current = self.records.pop_front();
        match current {
            Some((_, record)) => {
                *self.current.borrow_mut() = Some(record);
            }
            None => {
                *self.current.borrow_mut() = None;
            }
        };
        Ok(CursorResult::Ok(()))
    }

    fn wait_for_completion(&mut self) -> Result<()> {
        Ok(())
    }

    fn rowid(&self) -> Result<Ref<Option<u64>>> {
        todo!();
    }

    fn record(&self) -> Result<Ref<Option<OwnedRecord>>> {
        Ok(self.current.borrow())
    }

    fn insert(&mut self, record: &OwnedRecord) -> Result<()> {
        let key = match read_value(record, 0) {
            Ok(OwnedValue::Integer(i)) => i.to_string(),
            Ok(OwnedValue::Text(ref s)) => s.to_string(),
            _ => todo!(),
        };
        trace!("Inserting record with key: {}", key);
        self.insert(key, record.clone());
        Ok(())
    }
}
