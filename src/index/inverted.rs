use std::{collections::HashMap, vec, collections::HashSet};
use uuid::Uuid;

type Term = String;

#[derive(Debug)]
pub struct SegmentInfo {
    pub id: Uuid,
    pub num_docs: u64,
    pub docs: HashSet<Uuid>
}

#[derive(Debug)]
pub struct TermInfo {
    pub count: u64, 
    pub freq: u64,
    pub doc_ids: Vec<Uuid>,
    pub positions: Vec<u64>
}

#[derive(Debug)]
pub struct TermIndex {
    info: SegmentInfo,
    segments: HashMap<Term, TermInfo>,
}

impl TermIndex {
    pub fn new() -> Self {
        TermIndex {
            info: SegmentInfo {
                id: Uuid::new_v4(),
                num_docs: 0,
                docs: HashSet::new(),
            },
            segments: HashMap::new(),
        }
    }

    fn add_term(&mut self, term: Term, doc_id: Uuid, postion: u64) {
        let _ = self.segments.entry(term)
            .and_modify(|f| {
                f.count += 1;
                f.freq += 1;
                f.doc_ids.push(doc_id);
                f.positions.push(postion);
            })
            .or_insert(TermInfo {
                count: 1,
                freq: 1,
                doc_ids: vec![doc_id],
                positions: vec![postion],
            });

        if !self.info.docs.contains(&doc_id) {
            self.info.num_docs += 1;
            self.info.docs.insert(doc_id);
        }
    }

    fn get_term(&self, term: &Term) -> Option<&TermInfo> {
        self.segments.get(term)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_term_index() {
        let mut index = TermIndex::new();
        let doc_id = Uuid::new_v4();
        index.add_term("hello".to_string(), doc_id, 1);
        index.add_term("world".to_string(), doc_id, 2);
        index.add_term("hello".to_string(), doc_id, 3);
        index.add_term("world".to_string(), doc_id, 4);

        let term_info = index.get_term(&"hello".to_string());
        assert!(term_info.is_some());

        let term_info = term_info.unwrap();
        assert_eq!(term_info.count, 2);
        assert_eq!(term_info.freq, 2);
        assert_eq!(term_info.doc_ids.len(), 2);
        assert_eq!(term_info.positions.len(), 2);

        let non_found = "non_found".to_string();
        let term_info = index.get_term(&non_found);
        assert!(term_info.is_none());

        assert_eq!(index.info.num_docs, 1);
    }
}

