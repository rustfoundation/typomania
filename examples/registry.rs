use std::collections::{HashMap, HashSet};

use clap::Parser;
use typomania::{
    checks::{Bitflips, Omitted, SwappedWords, Typos},
    AuthorSet, Corpus, Harness, Package,
};

#[derive(Debug, Parser)]
struct Opt {
    /// Valid characters in package names
    #[arg(
        long,
        default_value = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-_"
    )]
    alphabet: String,

    /// Package names to consider top (or popular) packages, delimited by commas
    #[arg(short, long, value_name = "PACKAGE", value_delimiter = ',')]
    top_packages: Vec<String>,

    /// Packages to check against the top packages
    #[arg(value_name = "PACKAGE")]
    packages: Vec<String>,
}

fn main() -> typomania::Result<()> {
    let opt = Opt::parse();

    // Build a corpus of the top packages that we want to match against.
    let corpus = TopPackages::from(opt.top_packages);

    // Build a harness that uses the checks built into typomania.
    let harness = Harness::builder()
        .with_check(Bitflips::new(
            &opt.alphabet,
            corpus.0.keys().map(|s| s.as_str()),
        ))
        .with_check(Omitted::new(&opt.alphabet))
        .with_check(SwappedWords::new("-_."))
        .with_check(Typos::new(TYPOS.iter().map(|(c, typos)| {
            (*c, typos.iter().map(|ss| ss.to_string()).collect())
        })))
        .build(corpus);

    // Actually check the given packages.
    for (name, squats) in harness
        .check(opt.packages.into_iter().map(|name| {
            let package: Box<dyn Package> = Box::new(FakePackage::new(&name));
            (name, package)
        }))?
        .into_iter()
    {
        println!("{name}: {squats:?}");
    }

    Ok(())
}

struct TopPackages(HashMap<String, FakePackage>);

impl From<Vec<String>> for TopPackages {
    fn from(value: Vec<String>) -> Self {
        Self(
            value
                .into_iter()
                .map(|name| {
                    let package = FakePackage::new(&name);
                    (name, package)
                })
                .collect(),
        )
    }
}

impl Corpus for TopPackages {
    fn contains_name(&self, name: &str) -> typomania::Result<bool> {
        Ok(self.0.contains_key(name))
    }

    fn get(&self, name: &str) -> typomania::Result<Option<&dyn typomania::Package>> {
        Ok(self
            .0
            .get(name)
            .map(|package| package as &dyn typomania::Package))
    }
}

struct FakePackage {
    authors: HashSet<String>,
    description: String,
}

impl FakePackage {
    fn new(name: &str) -> Self {
        Self {
            // We'll set up a fake author based on the name so that there's no possibility of
            // having a match excluded because of a shared author.
            authors: [format!("{name} author <{name}@example.com>")]
                .into_iter()
                .collect(),
            description: format!("{name} is a package that does {name}"),
        }
    }
}

impl Package for FakePackage {
    fn authors(&self) -> &dyn AuthorSet {
        self
    }

    fn description(&self) -> Option<&str> {
        Some(&self.description)
    }

    fn shared_authors(&self, other: &dyn AuthorSet) -> bool {
        self.authors.iter().any(|author| other.contains(author))
    }
}

impl AuthorSet for FakePackage {
    fn contains(&self, author: &str) -> bool {
        self.authors.contains(author)
    }
}

// This is based on a pre-existing list we've used with crates.io for "easily confused characters".
// (I'm not really sure that I consider all of these easily confused, but it's better than nothing.)
static TYPOS: &[(char, &[&str])] = &[
    ('1', &["2", "q", "i", "l"]),
    ('2', &["1", "q", "w", "3"]),
    ('3', &["2", "w", "e", "4"]),
    ('4', &["3", "e", "r", "5"]),
    ('5', &["4", "r", "t", "6", "s"]),
    ('6', &["5", "t", "y", "7"]),
    ('7', &["6", "y", "u", "8"]),
    ('8', &["7", "u", "i", "9"]),
    ('9', &["8", "i", "o", "0"]),
    ('0', &["9", "o", "p", "-"]),
    ('-', &["_", "0", "p", ".", ""]),
    ('_', &["-", "0", "p", ".", ""]),
    ('q', &["1", "2", "w", "a"]),
    ('w', &["2", "3", "e", "s", "a", "q", "vv"]),
    ('e', &["3", "4", "r", "d", "s", "w"]),
    ('r', &["4", "5", "t", "f", "d", "e"]),
    ('t', &["5", "6", "y", "g", "f", "r"]),
    ('y', &["6", "7", "u", "h", "t", "i"]),
    ('u', &["7", "8", "i", "j", "y", "v"]),
    ('i', &["1", "8", "9", "o", "l", "k", "j", "u", "y"]),
    ('o', &["9", "0", "p", "l", "i"]),
    ('p', &["0", "-", "o"]),
    ('a', &["q", "w", "s", "z"]),
    ('s', &["w", "d", "x", "z", "a", "5"]),
    ('d', &["e", "r", "f", "c", "x", "s"]),
    ('f', &["r", "g", "v", "c", "d"]),
    ('g', &["t", "h", "b", "v", "f"]),
    ('h', &["y", "j", "n", "b", "g"]),
    ('j', &["u", "i", "k", "m", "n", "h"]),
    ('k', &["i", "o", "l", "m", "j"]),
    ('l', &["i", "o", "p", "k", "1"]),
    ('z', &["a", "s", "x"]),
    ('x', &["z", "s", "d", "c"]),
    ('c', &["x", "d", "f", "v"]),
    ('v', &["c", "f", "g", "b", "u"]),
    ('b', &["v", "g", "h", "n"]),
    ('n', &["b", "h", "j", "m"]),
    ('m', &["n", "j", "k", "rn"]),
    ('.', &["-", "_", ""]),
];
