use crate::network::Network;
use crate::objectdictionary;
use crate::sdo::errors::SDOAbortCode;

// use alloc::vec::Vec;

pub struct Node<'a> {
    pub network: &'a dyn Network,
    pub od: objectdictionary::ObjectDictionary,
}

impl<'a> Node<'a> {
    pub fn get_data(&self, index: u16, _subindex: u8) -> Result<Vec<u8>, SDOAbortCode> {
        let _object = self.find_object(index)?;
        if index == 1 {
            return Ok(vec![1, 2, 3, 4]);
        }
        Ok(vec![1, 2, 3, 4, 5])
    }

    fn find_object(&self, index: u16) -> Result<&objectdictionary::Object, SDOAbortCode> {
        match self.od.get(index) {
            Some(object) => Ok(object),
            None => Err(SDOAbortCode::ObjectDoesNotExist),
        }
    }
}
