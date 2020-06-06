pub struct TreeSegment {
    pub coordinates: [usize; 2],
    pub gain: u8,
    pub leaves: Vec<TreeSegment>,
}

impl TreeSegment {
    pub fn get_highest_gain(&self) -> u8 {
        if self.leaves.len() == 0 {
            return self.gain;
        }
        let mut proposed_leaf_gain = 0u8;
        for leaf_id in 0..self.leaves.len() {
            let leaf = &self.leaves[leaf_id];
            let proposition = leaf.get_highest_gain();
            if proposition > proposed_leaf_gain {
                proposed_leaf_gain = proposition;
            }
        }
        return proposed_leaf_gain + self.gain;
    }
}
