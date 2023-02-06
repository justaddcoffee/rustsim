
use std::{collections::{HashSet, HashMap}, fs::File};
use csv::{ReaderBuilder, Reader};

#[derive(Debug)]
struct Stats {
    set_id: String,
    original_reference_set: HashSet<String>,
    expanded_reference_set: HashSet<String>,
    original_new_set: HashSet<String>,
    expanded_new_set: HashSet<String>,
    jaccard_similarity: f64,
}
fn main() {
    // let set1: HashSet<&str> = ["apple", "banana", "cherry"].iter().cloned().collect();
    // let set2: HashSet<&str> = ["banana", "cherry", "date"].iter().cloned().collect();

    /*
    read in TSV file
    csv::ReaderBuilder instead of just csv::Reader because we need to specify
    that the file has no headers.
    */

    let data_dict = parse_associations(read_file("test_set.tsv"));
    let closures_dict = parse_associations(read_file("closures.tsv"));
    let ref_set = data_dict.get("set1").unwrap();

    let mut stat_info = Stats{
        set_id: String::new(),
        original_reference_set: HashSet::new(),
        expanded_reference_set: HashSet::new(),
        original_new_set: HashSet::new(),
        expanded_new_set: HashSet::new(),
        jaccard_similarity: 0.0
    };

    stat_info.original_reference_set = ref_set.clone();
    stat_info.expanded_reference_set = expand_terms_using_closure(&stat_info.original_reference_set, &closures_dict);
    // iterate over dict
    for (key, terms) in &data_dict {
        // println!("Original HashMap : key => {key} ; value: {terms:?}");
        // let expanded_terms = expand_terms_using_closure(terms, &closures_dict);
        stat_info.set_id = key.to_string();
        stat_info.original_new_set = terms.clone();
        stat_info.expanded_new_set = expand_terms_using_closure(&stat_info.original_new_set, &closures_dict);
        // println!("Expanded HashMap : key => {key} ; value: {expanded_terms:?}");
        // let score:f64 = jaccard_similarity(&stat_info.expanded_reference_set, &stat_info.expanded_new_set);
        stat_info.jaccard_similarity = jaccard_similarity(&stat_info.expanded_reference_set, &stat_info.expanded_new_set);
        // println!("Jaccard score : {score:?}")
        println!("{stat_info:?}")
    }
}

fn jaccard_similarity(set1: &HashSet<String>, set2: &HashSet<String>) -> f64 {
    /* Returns Jaccard similarity between the two sets. */

    let intersection = set1.intersection(set2).count();
    let union_measure = set1.union(set2).count();
    intersection as f64 / union_measure as f64
}

fn read_file(filename: &str) -> Reader<File> {
    /* Build CSV reader from filepath.*/
    ReaderBuilder::new().has_headers(false)
                        .from_path(filename)
                        .unwrap()
}

fn parse_associations(mut reader: Reader<File>) -> HashMap<String, HashSet<String>> {
    /* Parse CSV files using ReaderBuilder.*/
    let mut dict_from_csv: HashMap<String, HashSet<String>> = HashMap::new();

    for result in reader.records() {
        let record = result.unwrap();
        let key = &record[0];
        let value = &record[1];
        let n = dict_from_csv.entry(key.to_string());
        n.or_default().insert(value.to_string());
    }
    dict_from_csv
}

fn expand_terms_using_closure(terms:&HashSet<String> , term_closure_map: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    /* Expand terms by inclusing ancestors in the set. */
    let mut expanded_set = HashSet::<String>::new();
    for item in terms.iter() {
        expanded_set.extend(term_closure_map.get(item).unwrap().clone());
    }
    expanded_set
}