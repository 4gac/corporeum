use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use uniform::Corpus;
use walkdir::WalkDir;

// https://universaldependencies.org/format.html
const _ID: usize = 0b0;
const FORM: usize = 0b1;
const LEMMA: usize = 0b10;
const UPOS: usize = 0b11;
const XPOS: usize = 0b100;
const FEATS: usize = 0b101;
const HEAD: usize = 0b110;
const DEPREL: usize = 0b111;
const DEPS: usize = 0b1000;
const _MISC: usize = 0b1001;

const UNSPECIFIED: &str = "_";

fn convert(path: &Path, pretty: bool) -> Result<(), Box<std::io::Error>> {
    let file = OpenOptions::new().read(true).open(path)?;

    let mut treebank = Corpus::new();

    let br = BufReader::new(file);

    let mut doc = treebank.create_doc();

    let mut sent = doc.create_sentence("en");

    for line in br.lines() {
        let line = line.unwrap();

        if line.starts_with("#") {
            continue;
        }

        if line.trim().is_empty() {
            doc.add_sentence(sent)
                .expect("Failed to create sentence for line: {line}");
            sent = doc.create_sentence("en");
            continue;
        }

        let annots = line.split("\t").collect::<Vec<&str>>();

        let mut tok = sent.create_token(annots[FORM]);
        if annots[LEMMA] != UNSPECIFIED {
            tok.set_lemma(annots[LEMMA]);
        }

        if annots[UPOS] != UNSPECIFIED {
            tok.set_upos(annots[UPOS]);
        }

        if annots[XPOS] != UNSPECIFIED {
            tok.set_xpos(annots[XPOS]);
        }

        if annots[FEATS] != UNSPECIFIED {
            tok.set_feats(annots[FEATS]);
        }

        if annots[HEAD] != UNSPECIFIED {
            tok.set_head(annots[HEAD]);
        }

        if annots[DEPREL] != UNSPECIFIED {
            tok.set_deprel(annots[DEPREL]);
        }

        if annots[DEPS] != UNSPECIFIED {
            tok.set_deps(annots[DEPS]);
        }

        sent.add_token(tok);
    }

    treebank.add_doc(doc).unwrap();

    let out_path = String::from("output/") + path.file_name().unwrap().to_str().unwrap() + ".json";

    // create subdirs
    // in this case it is just "uniform" dir

    std::fs::create_dir_all("output").unwrap();

    let out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out_path)
        .expect("Failed to open output file");
    if pretty {
        treebank
            .save_into_pretty(out_file)
            .expect("Uniform failed to save to file (formatted).")
    } else {
        treebank
            .save_into(out_file)
            .expect("Uniform failed to save file.")
    }

    Ok(())
}

fn main() -> Result<(), Box<std::io::Error>> {
    let mut pretty = false;
    for (i, arg) in std::env::args().enumerate() {
        // path of the executable
        if i == 0 {
            continue;
        }

        if i == 1 && arg == "--pretty" {
            pretty = true;
        }

        let ud_files: Vec<PathBuf> = WalkDir::new(arg)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| {
                entry.path().is_file()
                    && entry
                        .path()
                        .extension()
                        .map(|ext| ext == "conllu")
                        .unwrap_or(false)
            })
            .map(|entry| entry.path().to_path_buf())
            .collect();

        ud_files
            .par_iter()
            .try_for_each(|path| convert(path, pretty))?;
    }

    Ok(())
}
