use Result;
use core::SegmentReader;
use core::index::Index;
use core::index::Segment;
use schema::Document;
use collector::Collector;
use common::TimerTree;
use query::Query;
use DocId;
use DocAddress;
use schema::Term;


#[derive(Debug)]
pub struct Searcher {
    segments: Vec<SegmentReader>,
}

impl Searcher {

    pub fn doc(&self, doc_address: &DocAddress) -> Result<Document> {
        // TODO err
        let DocAddress(segment_local_id, doc_id) = *doc_address;
        let segment_reader = &self.segments[segment_local_id as usize];
        segment_reader.doc(doc_id)
    }
    
    pub fn num_docs(&self,) -> DocId {
        self.segments
            .iter()
            .map(|segment_reader| segment_reader.num_docs())
            .fold(0u32, |acc, val| acc + val)
    }

    pub fn doc_freq(&self, term: &Term) -> u32 {
        self.segments
            .iter()
            .map(|segment_reader| segment_reader.doc_freq(term))
            .fold(0u32, |acc, val| acc + val)
    }

    fn add_segment(&mut self, segment: Segment) -> Result<()> {
        let segment_reader = try!(SegmentReader::open(segment.clone()));
        self.segments.push(segment_reader);
        Ok(())
    }

    fn new() -> Searcher {
        Searcher {
            segments: Vec::new(),
        }
    }
    
    pub fn segments(&self,) -> &Vec<SegmentReader> {
        &self.segments
    }

    pub fn for_index(index: Index) -> Result<Searcher> {
        let mut searcher = Searcher::new();
        for segment in index.segments() {
            try!(searcher.add_segment(segment));
        }
        Ok(searcher)
    }
    
    pub fn search<Q: Query, C: Collector>(&self, query: &Q, collector: &mut C) -> Result<TimerTree> {
        query.search(self, collector)
    }
}
