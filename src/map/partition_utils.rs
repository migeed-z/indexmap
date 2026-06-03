use crate::inner::Core;
use crate::IndexMap;

/// Build the final partition result from the two cores produced by
/// `Core::partition_entries`.  Packages each core together with
/// a clone of the original hash builder.
pub(super) fn build_partition_result<K, V, S: Clone>(
    yes_core: Core<K, V>,
    no_core: Core<K, V>,
    hash_builder: &S,
) -> (IndexMap<K, V, S>, IndexMap<K, V, S>) {
    // Assemble the two maps from the partitioned cores.
    (
        IndexMap { core: no_core, hash_builder: hash_builder.clone() },
        IndexMap { core: yes_core, hash_builder: hash_builder.clone() },
    )
}
