/// A strategy for normalizing a list of scores from a population
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)] // TODO
pub enum ScoreNormalizationStrategy {
    /// Makes no changes to the input scores, except replacing all 0.0s with all
    /// 1.0s
    NoNormalization,

    /// Linearly interpolates scores from [old_min, old_max] to [0.0, 1.0]
    LerpMinMax,
}

impl ScoreNormalizationStrategy {
    /// Normalizes the scores of a population. If all the scores are the same,
    /// normalizes all scores to 1.0
    ///
    /// # Panics
    ///
    /// - If the length of `scores` is zero
    /// - If any of the values in `scores` is NaN or Infinite
    ///
    /// Normalized scores are all guaranteed to:
    /// 1. Be non-negative
    /// 2. Be finite
    /// 3. Not be NaN
    /// 4. Have a non-zero sum
    pub fn normalize(&self, scores: &[f64]) -> Vec<f64> {
        if scores.is_empty() {
            return Vec::new();
        }

        // Validate input
        assert!(!scores
            .iter()
            .any(|score| score.is_infinite() || score.is_nan()));

        // If all the scores are the same, just return all 1.0s
        if scores.windows(2).all(|pair| pair[0] == pair[1]) {
            return vec![1.0; scores.len()];
        }

        let old_max: f64 = scores.iter().copied().reduce(f64::max).unwrap();
        let old_min: f64 = scores.iter().copied().reduce(f64::min).unwrap();
        let old_sum: f64 = scores.iter().sum();

        let normalized: Vec<f64> = scores
            .iter()
            .copied()
            .map(|score| self.normalize_single_score(score, old_min, old_max, old_sum))
            .collect();

        assert!(!normalized
            .iter()
            .any(|score| score.is_sign_negative() || score.is_infinite() || score.is_nan()));

        normalized
    }

    fn normalize_single_score(
        &self,
        old_score: f64,
        old_min: f64,
        old_max: f64,
        old_sum: f64,
    ) -> f64 {
        assert!(old_score.is_finite() && !old_score.is_nan());
        assert!(old_min.is_finite() && !old_min.is_nan());
        assert!(old_max.is_finite() && !old_max.is_nan());
        assert!(old_min <= old_score && old_score <= old_max);
        assert!(old_min <= old_max);
        assert!(old_max <= old_sum);

        match self {
            ScoreNormalizationStrategy::NoNormalization => old_score,
            ScoreNormalizationStrategy::LerpMinMax => (old_score - old_min) / (old_max - old_min),
        }
    }
}
