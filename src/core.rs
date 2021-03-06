use sha2::{Digest, Sha256};

use std::{error, fmt, result, time};

pub const MAX_CONNS: usize = 4;

#[derive(Clone, Debug)]
/// Configuration parameters for Client.
pub struct Config {
    /// A previously fetched round serving as a verification checkpoint.
    ///
    /// * if `determinism` is true and check_point is None, Round-1 acts
    ///   as the the check_point round.
    /// * if `determinism` is false, lastest round is assumed as verified
    ///   round and treated as `check_point`.
    /// * if `secure` is false, every beacon round is assumed as verfied
    ///   round.
    /// * if `secure` is true, every new round is verified with
    ///   `check_point` round.
    ///
    /// Default: None
    pub check_point: Option<Random>,
    /// Ensure all rounds from check_point to the latest round is valid
    ///
    /// Default: false,
    pub determinism: bool,
    /// Ensure all future rounds from latest round is verified.
    ///
    /// Default: false
    pub secure: bool,
    /// Maximum number of concurrent connections allowed per remote.
    ///
    /// Default: MAX_CONNS
    pub max_conns: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            check_point: None,
            determinism: false,
            secure: false,
            max_conns: MAX_CONNS,
        }
    }
}

impl Config {
    pub fn set_check_point(&mut self, check_point: Option<Random>) -> &mut Self {
        self.check_point = check_point;
        self
    }

    pub fn set_determinism(&mut self, determinism: bool) -> &mut Self {
        self.determinism = determinism;
        self
    }

    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.secure = secure;
        self
    }

    pub fn set_max_conns(&mut self, max_conns: usize) -> &mut Self {
        self.max_conns = max_conns;
        self
    }
}

/// Type alias for Result return type, used by this package.
pub type Result<T> = result::Result<T, Error>;

/// Error variants that can be returned by this package's API.
///
/// Each variant carries a prefix, typically identifying the
/// error location.
pub enum Error {
    Fatal(String, String),
    PoisonedLock(String, String),
    NotSecure(String, String),
    Invalid(String, String),
    IOError(String, String),
    JsonParse(String, String),
    StringParse(String, String),
    HexParse(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Error::*;

        match self {
            Fatal(p, msg) => write!(f, "{} Fatal: {}", p, msg),
            PoisonedLock(p, msg) => write!(f, "{} PoisonedLock: {}", p, msg),
            NotSecure(p, msg) => write!(f, "{} NotSecure: {}", p, msg),
            Invalid(p, msg) => write!(f, "{} Invalid: {}", p, msg),
            IOError(p, msg) => write!(f, "{} IOError: {}", p, msg),
            JsonParse(p, msg) => write!(f, "{} JsonParse: {}", p, msg),
            StringParse(p, msg) => write!(f, "{} StringParse: {}", p, msg),
            HexParse(p, msg) => write!(f, "{} HexParse: {}", p, msg),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {}

// TODO: Is there any way to use info.hash to validate the first round of
// randomness.

/// Type captures the drand-group's hash-info.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Info {
    /// Distributed public key of the drand group.
    pub public_key: Vec<u8>,
    /// Time in seconds between randomness beacon rounds.
    pub period: time::Duration,
    /// Time in seconds since the Unix Epoch that the group began generating
    /// randomness
    pub genesis_time: time::SystemTime,
    /// Chain-hash, which uniquely identifies the drand chain. It is used as
    /// a root of trust for validation of the first round of randomness.
    pub hash: Vec<u8>,
    /// Use as previous_signature to validate the first round of randomness.
    pub group_hash: Vec<u8>,
}

impl Default for Info {
    fn default() -> Self {
        Info {
            public_key: Vec::default(),
            period: time::Duration::default(),
            genesis_time: time::UNIX_EPOCH,
            hash: Vec::default(),
            group_hash: Vec::default(),
        }
    }
}

/// Type captures randomness from drand-group for a single round.
///
/// This randomness can be verified at the client side using root-of-trust
/// and the group's hash-info.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Random {
    /// Sequentially increasing integer - the randomness round index.
    pub round: u128,
    /// SHA-256 hash of the signature.
    pub randomness: Vec<u8>,
    /// Boneh-Lynn-Shacham (BLS) signature for this round of randomness.
    pub signature: Vec<u8>,
    /// Signature of the previous round of randomness.
    pub previous_signature: Vec<u8>,
}

impl fmt::Display for Random {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Random<{}>", self.round)
    }
}

impl Random {
    pub fn to_digest(&self) -> Result<Vec<u8>> {
        let mut hasher = Sha256::default();
        hasher.update(&self.previous_signature);
        hasher.update(self.round.to_be_bytes());
        Ok(hasher.finalize().to_vec())
    }
}
