use anyhow::Result;
use boomphf::Mphf;
use paf::{PafRecord, Reader, RecordsIntoIter};
use std::{collections::HashSet, path::PathBuf};

// Aligned sequence in a PAF file.
#[derive(Debug, PartialEq, Eq)]
pub struct AlignedSeq {
    // name of aligned sequence
    name: String,
    // unique ID of aligned sequence
    id: usize,
    // length of sequence
    length: u64,
    // rank amongst other sequences in query/target set
    rank: u64,
    // start offset in global all-to-all alignment matrix
    offset: u64,
}

impl AlignedSeq {
    // initialise a new AlignedSeq
    fn new() -> Self {
        Self {
            name: String::new(),
            id: usize::MAX,
            length: 0,
            rank: 0,
            offset: 0,
        }
    }

    fn set_nli(&mut self, name: String, length: u64, id: usize) {
        self.name = name;
        self.length = length;
        self.id = id;
    }
}

#[derive(Debug)]
struct AlignedSeqs {
    // list of aligned sequences
    seqs: Vec<AlignedSeq>,
    // map from sequence name to internal ID
    mphf: Mphf<String>,
    // axis length
    axis_length: f64,
}

impl AlignedSeqs {
    // set the axis length to be zero, we'll calculate this later
    fn new(seqs: Vec<AlignedSeq>, mphf: Mphf<String>) -> Self {
        Self {
            seqs,
            mphf,
            axis_length: 0.0,
        }
    }

    // get the id from a sequence name
    fn get_id(&self, name: &String) -> usize {
        self.mphf.hash(name) as usize
    }

    // add a sequence to the set if it doesn't already exist
    fn add_seq_if_it_does_not_exist(&mut self, seq: AlignedSeq) {
        if !self.seqs.contains(&seq) {
            self.seqs.push(seq);
        }
    }

    // add the ranks and the offsets
    fn add_ranks_and_offsets(&mut self) {
        // sort the queries/targets by length
        self.seqs.sort_by(|a, b| b.length.cmp(&a.length));

        let mut index = 0;
        let mut offset = 0;

        for seq in self.seqs.iter_mut() {
            seq.rank = index as u64;
            seq.offset = offset;
            offset += seq.length;
            index += 1;
        }
    }
}

#[derive(Debug)]
pub struct PAFSeqs {
    // The target
    targets: AlignedSeqs,
    // The queries
    queries: AlignedSeqs,
}

impl PAFSeqs {
    // Create the target and query sequences from a PAF file.
    pub fn new(filename: PathBuf) -> Result<Self> {
        // a global reader for the PAF file
        let mut reader = Reader::from_path(filename.clone())?;

        let (query_names, target_names) = {
            let mut qn = HashSet::new();
            let mut tn = HashSet::new();

            // get target and query names from the PAF file
            for record in reader.records() {
                let record = record?;
                // get query and target names
                qn.insert(record.query_name().to_string());
                tn.insert(record.target_name().to_string());
            }
            // convert to vectors
            let qn: Vec<String> = qn.into_iter().collect();
            let tn: Vec<String> = tn.into_iter().collect();

            (qn, tn)
        };

        let mut queries = AlignedSeqs::new(
            Vec::with_capacity(query_names.len()),
            Mphf::new(1.7, &query_names),
        );

        let mut targets = AlignedSeqs::new(
            Vec::with_capacity(target_names.len()),
            Mphf::new(1.7, &target_names),
        );

        // we need to read the PAF file again to get the lengths
        let mut reader = Reader::from_path(filename)?;

        for record in reader.records() {
            let record = record?;

            let query_name = record.query_name().to_string();
            let target_name = record.target_name().to_string();

            let query_id = queries.get_id(&query_name);
            let target_id = targets.get_id(&target_name);

            let query_len = record.query_len();
            let target_len = record.target_len();

            let mut query = AlignedSeq::new();
            query.set_nli(query_name, query_len as u64, query_id);

            let mut target = AlignedSeq::new();
            target.set_nli(target_name, target_len as u64, target_id);

            // add sequences if they don't exist
            queries.add_seq_if_it_does_not_exist(query);
            targets.add_seq_if_it_does_not_exist(target);
        }

        // now add the ranks and the offsets
        queries.add_ranks_and_offsets();
        targets.add_ranks_and_offsets();

        // and finally sort on the IDs so we can index into them nicely later
        queries.seqs.sort_by(|a, b| a.id.cmp(&b.id));
        targets.seqs.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(Self { targets, queries })
    }
}
