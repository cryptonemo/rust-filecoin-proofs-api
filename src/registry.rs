use std::sync::atomic::Ordering;

use anyhow::Result;
use filecoin_proofs_v1::types::{PoRepConfig, PoRepProofPartitions, PoStConfig, SectorSize};

/// Available seal proofs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RegisteredSealProof {
    StackedDrg1KiBV1,
    StackedDrg16MiBV1,
    StackedDrg256MiBV1,
    StackedDrg1GiBV1,
    StackedDrg32GiBV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Version {
    V1,
}

impl RegisteredSealProof {
    /// Return the version for this proof.
    pub fn version(self) -> Version {
        use RegisteredSealProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => Version::V1,
        }
    }

    /// Return the sector size for this proof.
    pub fn sector_size(self) -> SectorSize {
        use filecoin_proofs_v1::constants;
        use RegisteredSealProof::*;
        let size = match self {
            StackedDrg1KiBV1 => constants::SECTOR_SIZE_ONE_KIB,
            StackedDrg16MiBV1 => constants::SECTOR_SIZE_16_MIB,
            StackedDrg256MiBV1 => constants::SECTOR_SIZE_256_MIB,
            StackedDrg1GiBV1 => constants::SECTOR_SIZE_1_GIB,
            StackedDrg32GiBV1 => constants::SECTOR_SIZE_32_GIB,
        };
        SectorSize(size)
    }

    /// Return the number of partitions for this proof.
    pub fn partitions(self) -> u8 {
        use RegisteredSealProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => filecoin_proofs_v1::constants::DEFAULT_POREP_PROOF_PARTITIONS
                .load(Ordering::Relaxed),
        }
    }

    pub fn single_partition_proof_len(self) -> usize {
        use RegisteredSealProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => filecoin_proofs_v1::SINGLE_PARTITION_PROOF_LEN,
        }
    }

    pub fn as_v1_config(self) -> PoRepConfig {
        use RegisteredSealProof::*;

        assert_eq!(self.version(), Version::V1);

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => PoRepConfig {
                sector_size: self.sector_size(),
                partitions: PoRepProofPartitions(self.partitions()),
            },
            // _ => panic!("Can only be called on V1 configs"),
        }
    }

    /// Returns the cache identifier.
    pub fn cache_identifier(self) -> Result<String> {
        match self.version() {
            Version::V1 => self.as_v1_config().get_cache_identifier(),
        }
    }
}

/// Available seal proofs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RegisteredPoStProof {
    StackedDrg1KiBV1,
    StackedDrg16MiBV1,
    StackedDrg256MiBV1,
    StackedDrg1GiBV1,
    StackedDrg32GiBV1,
}

impl RegisteredPoStProof {
    /// Return the version for this proof.
    pub fn version(self) -> Version {
        use RegisteredPoStProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => Version::V1,
        }
    }

    /// Return the sector size for this proof.
    pub fn sector_size(self) -> SectorSize {
        use filecoin_proofs_v1::constants;
        use RegisteredPoStProof::*;

        let size = match self {
            StackedDrg1KiBV1 => constants::SECTOR_SIZE_ONE_KIB,
            StackedDrg16MiBV1 => constants::SECTOR_SIZE_16_MIB,
            StackedDrg256MiBV1 => constants::SECTOR_SIZE_256_MIB,
            StackedDrg1GiBV1 => constants::SECTOR_SIZE_1_GIB,
            StackedDrg32GiBV1 => constants::SECTOR_SIZE_32_GIB,
        };
        SectorSize(size)
    }

    /// Return the number of partitions for this proof.
    pub fn partitions(self) -> u8 {
        use RegisteredPoStProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => 1,
        }
    }

    pub fn single_partition_proof_len(self) -> usize {
        use RegisteredPoStProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => filecoin_proofs_v1::SINGLE_PARTITION_PROOF_LEN,
        }
    }

    pub fn as_v1_config(self) -> PoStConfig {
        assert_eq!(self.version(), Version::V1);

        use RegisteredPoStProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => PoStConfig {
                sector_size: self.sector_size(),
                challenge_count: filecoin_proofs_v1::constants::POST_CHALLENGE_COUNT,
                challenged_nodes: filecoin_proofs_v1::constants::POST_CHALLENGED_NODES,
            },
            // _ => panic!("Can only be called on V1 configs"),
        }
    }

    /// Returns the cache identifier.
    pub fn cache_identifier(self) -> Result<String> {
        match self.version() {
            Version::V1 => self.as_v1_config().get_cache_identifier(),
        }
    }
}
