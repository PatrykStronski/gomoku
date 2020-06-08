pub struct TreeSegment {
    pub coordinates: [usize; 2],
    pub gain: i8,
    pub leaves: Vec<TreeSegment>,
    pub minimize_leaves: bool,
}

impl TreeSegment {
    fn get_best_opponent_move(&self) -> i8 {
        let mut proposed_leaf_id = 0usize;
        let mut best_leaf_gain = 0i8;
        for leaf_id in 0..self.leaves.len() {
            let leaf = &self.leaves[leaf_id];
            let proposition = leaf.gain;
            if self.leaves[leaf_id].gain < best_leaf_gain {
                best_leaf_gain = self.leaves[leaf_id].gain;
                proposed_leaf_id = leaf_id;
            }
            if self.leaves[leaf_id].gain == -5 {
                break;
            }
        }
        return self.leaves[proposed_leaf_id].get_highest_gain();
    }

    pub fn get_highest_gain(&self) -> i8 {
        if self.leaves.len() == 0 {
            return self.gain;
        }
        if self.minimize_leaves {
            return self.gain + self.get_best_opponent_move();
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
