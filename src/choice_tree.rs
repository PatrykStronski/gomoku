pub struct TreeSegment {
    pub coordinates: [usize; 2],
    pub gain: i8,
    pub leaves: Vec<TreeSegment>,
    pub minimize_leaves: bool,
}

impl TreeSegment {
    fn get_lowest_gain(&self) -> i8 {
        let mut proposed_leaf_gain = 0i8;
        for leaf_id in 0..self.leaves.len() {
            let leaf = &self.leaves[leaf_id];
            let proposition = leaf.get_highest_gain();
            if proposition < proposed_leaf_gain {
                proposed_leaf_gain = proposition;
            }
            if proposed_leaf_gain <= -5 {
                return proposed_leaf_gain * 2;
            }
        }
        return proposed_leaf_gain;
    }

    pub fn get_highest_gain(&self) -> i8 {
        if self.leaves.len() == 0 {
            return self.gain;
        }
        if self.gain >= -5 {
            return self.gain;
        }
        if self.minimize_leaves {
            return self.get_lowest_gain() + self.gain;
        }
        let mut proposed_leaf_gain = 0i8;
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
