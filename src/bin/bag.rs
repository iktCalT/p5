use rand::{SeedableRng, seq::SliceRandom};
use rand_xoshiro::Xoshiro128PlusPlus;

type RngAlgorithm = Xoshiro128PlusPlus;

/// `BagRandomizer<T>` creates a bag randomizer of items of type T. This can
/// be thought of as a bag of items which are drawn without replacement. Once
/// the bag is empty, the bag is refilled with the items initally drawn. This
/// bounds the time between drawing a duplicate item to the total number of
/// items 2 * (#items - 1).
///
/// This struct provides a `cur()` method to get an item from the randomzier.
/// The same item will be returned until it is removed with a `next()` method.
///
/// ```rs
/// let mut randomizer = BagRandomizer::from_parts(1..=3, [4; 16]);
/// let first_item = *randomizer.cur();
/// let same_item = *randomizer.cur();
/// assert_eq!(first_item, same_item);
///
/// randomizer.next();
/// let different_item = *randomizer.cur();
/// assert_ne!(first_item, different_item);
///
/// randomizer.next();
/// let different_item = *randomizer.cur();
/// assert_ne!(first_item, different_item);
/// ```
pub struct BagRandomizer<T> {
    bag: Vec<T>,
    idx: usize,
    rng: RngAlgorithm,
}

impl<T> BagRandomizer<T> {
    /// Creates a `BagRandomizer` from an interator of items to put in the bag
    /// and a seed for the underlying RNG.
    pub fn from_parts(
        bag: impl Iterator<Item = T>,
        seed: <RngAlgorithm as SeedableRng>::Seed,
    ) -> Self {
        let mut rng = RngAlgorithm::from_seed(seed);

        let mut bag: Vec<T> = bag.collect();
        bag.shuffle(&mut rng);

        Self { idx: 0, bag, rng }
    }

    /// Removes the current exposed value in the bag, cycling to the next
    /// random value.
    pub fn next(&mut self) {
        self.idx += 1;
        if self.idx >= self.bag.len() {
            self.bag.shuffle(&mut self.rng);
            self.idx = 0;
        }
    }

    /// Returns the current exposed value in the bag.
    pub fn cur(&self) -> &T {
        &self.bag[self.idx]
    }
}