// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

//! Deterministic statistical helpers for evidence-backed plugins.
//!
//! Functions contain no sampling or hidden randomness. Callers must expose sample
//! size and assumptions beside any resulting score.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ConfidenceEstimate;

/// Invalid input supplied to a statistical estimator.
#[derive(Clone, Debug, PartialEq)]
pub enum StatisticsError {
    /// Sample contained NaN or infinity.
    NonFinite,
    /// Resource sample contained a negative value.
    NegativeSample(f64),
    /// A normalized probability was outside 0.0 through 1.0.
    InvalidProbability(f64),
    /// Success count exceeded trial count.
    InvalidCounts,
    /// Beta prior parameters were not positive.
    InvalidPrior,
    /// Paired inputs had different lengths or were empty.
    InvalidPairs,
}

impl fmt::Display for StatisticsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("samples must contain only finite values"),
            Self::NegativeSample(value) => {
                write!(formatter, "resource samples cannot be negative: {value}")
            }
            Self::InvalidProbability(value) => {
                write!(
                    formatter,
                    "probability must be between 0.0 and 1.0: {value}"
                )
            }
            Self::InvalidCounts => formatter.write_str("successes cannot exceed trials"),
            Self::InvalidPrior => formatter.write_str("beta prior parameters must be positive"),
            Self::InvalidPairs => formatter
                .write_str("probabilities and outcomes must be non-empty equal-length inputs"),
        }
    }
}

impl Error for StatisticsError {}

/// Descriptive statistics for an observed finite sample.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SampleStatistics {
    /// Number of observations.
    pub count: usize,
    /// Arithmetic mean.
    pub mean: f64,
    /// Population variance.
    pub variance: f64,
    /// Population standard deviation.
    pub standard_deviation: f64,
    /// Standard deviation divided by absolute mean, when mean is non-zero.
    pub coefficient_of_variation: Option<f64>,
    /// Smallest observation.
    pub minimum: f64,
    /// Largest observation.
    pub maximum: f64,
}

/// Deterministic Beta-Binomial probability estimate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProbabilityEstimate {
    /// Posterior mean probability.
    pub probability: f64,
    /// Approximate 95% interval, clipped to the unit interval.
    pub confidence: ConfidenceEstimate,
    /// Number of observed Bernoulli trials.
    pub sample_size: usize,
    /// Declared Beta prior alpha.
    pub prior_alpha: f64,
    /// Declared Beta prior beta.
    pub prior_beta: f64,
}

/// Computes descriptive population statistics without random sampling.
pub fn summarize(samples: &[f64]) -> Result<Option<SampleStatistics>, StatisticsError> {
    if samples.is_empty() {
        return Ok(None);
    }
    if samples.iter().any(|value| !value.is_finite()) {
        return Err(StatisticsError::NonFinite);
    }

    let count = samples.len();
    let mean = samples.iter().sum::<f64>() / count as f64;
    let variance = samples
        .iter()
        .map(|value| {
            let delta = value - mean;
            delta * delta
        })
        .sum::<f64>()
        / count as f64;
    let standard_deviation = variance.sqrt();

    Ok(Some(SampleStatistics {
        count,
        mean,
        variance,
        standard_deviation,
        coefficient_of_variation: (mean.abs() > f64::EPSILON)
            .then_some(standard_deviation / mean.abs()),
        minimum: samples.iter().copied().fold(f64::INFINITY, f64::min),
        maximum: samples.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    }))
}

/// Scores repeated Boolean outcomes independently from their success rate.
///
/// `1 - 4p(1-p)` is 1.0 when every outcome agrees and 0.0 at maximum
/// Bernoulli variance (`p = 0.5`). Consistent failure remains consistent, not good.
#[must_use]
pub fn outcome_consistency(outcomes: &[bool]) -> Option<f64> {
    if outcomes.is_empty() {
        return None;
    }
    let successes = outcomes.iter().filter(|outcome| **outcome).count();
    let probability = successes as f64 / outcomes.len() as f64;
    Some((1.0 - 4.0 * probability * (1.0 - probability)).clamp(0.0, 1.0))
}

/// Scores repeatability of a non-negative resource measurement.
///
/// Uses `1 - min(coefficient_of_variation, 1)`. This is a heuristic score;
/// consumers should also retain the raw statistics.
pub fn resource_consistency(samples: &[f64]) -> Result<Option<f64>, StatisticsError> {
    if let Some(value) = samples.iter().find(|value| **value < 0.0) {
        return Err(StatisticsError::NegativeSample(*value));
    }
    let Some(summary) = summarize(samples)? else {
        return Ok(None);
    };
    let score = match summary.coefficient_of_variation {
        Some(coefficient) => 1.0 - coefficient.min(1.0),
        None => 1.0,
    };
    Ok(Some(score))
}

/// Computes the Brier score: mean squared error for probability forecasts.
///
/// The result is between 0.0 (perfect) and 1.0 (maximal error). This is a raw
/// metric, not a higher-is-better normalized Harness Lens score.
pub fn brier_score(probabilities: &[f64], outcomes: &[bool]) -> Result<f64, StatisticsError> {
    if probabilities.is_empty() || probabilities.len() != outcomes.len() {
        return Err(StatisticsError::InvalidPairs);
    }
    for probability in probabilities {
        if !probability.is_finite() {
            return Err(StatisticsError::NonFinite);
        }
        if !(0.0..=1.0).contains(probability) {
            return Err(StatisticsError::InvalidProbability(*probability));
        }
    }

    Ok(probabilities
        .iter()
        .zip(outcomes)
        .map(|(probability, outcome)| {
            let observed = if *outcome { 1.0 } else { 0.0 };
            (probability - observed).powi(2)
        })
        .sum::<f64>()
        / probabilities.len() as f64)
}

/// Computes the higher-is-better complement of the Brier score.
///
/// This is named `brier_accuracy`, not Brier skill score: standard Brier skill
/// requires a declared reference forecast.
pub fn brier_accuracy(probabilities: &[f64], outcomes: &[bool]) -> Result<f64, StatisticsError> {
    Ok((1.0 - brier_score(probabilities, outcomes)?).clamp(0.0, 1.0))
}

/// Estimates a Bernoulli probability with an explicit Beta prior.
///
/// Interval uses a normal approximation to the Beta posterior. Small or highly
/// skewed samples should be treated as provisional evidence.
pub fn beta_probability(
    successes: usize,
    trials: usize,
    prior_alpha: f64,
    prior_beta: f64,
) -> Result<ProbabilityEstimate, StatisticsError> {
    if successes > trials {
        return Err(StatisticsError::InvalidCounts);
    }
    if !prior_alpha.is_finite()
        || !prior_beta.is_finite()
        || prior_alpha <= 0.0
        || prior_beta <= 0.0
    {
        return Err(StatisticsError::InvalidPrior);
    }

    let alpha = prior_alpha + successes as f64;
    let beta = prior_beta + (trials - successes) as f64;
    let total = alpha + beta;
    let probability = alpha / total;
    let variance = alpha * beta / (total * total * (total + 1.0));
    let margin = 1.959_963_984_540_054 * variance.sqrt();

    Ok(ProbabilityEstimate {
        probability,
        confidence: ConfidenceEstimate {
            lower: (probability - margin).clamp(0.0, 1.0),
            upper: (probability + margin).clamp(0.0, 1.0),
            level: 0.95,
            method: "normal approximation to Beta posterior".to_owned(),
        },
        sample_size: trials,
        prior_alpha,
        prior_beta,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consistency_separates_repeatability_from_success() {
        assert_eq!(outcome_consistency(&[true, true, true]), Some(1.0));
        assert_eq!(outcome_consistency(&[false, false, false]), Some(1.0));
        assert_eq!(outcome_consistency(&[true, false]), Some(0.0));
    }

    #[test]
    fn brier_metrics_reward_calibrated_certainty() {
        let score = brier_score(&[0.9, 0.1], &[true, false]).unwrap();
        let accuracy = brier_accuracy(&[0.9, 0.1], &[true, false]).unwrap();
        assert!((score - 0.01).abs() < 1e-12);
        assert!((accuracy - 0.99).abs() < 1e-12);
    }

    #[test]
    fn beta_probability_exposes_prior_and_uncertainty() {
        let estimate = beta_probability(8, 10, 1.0, 1.0).unwrap();
        assert!((estimate.probability - 0.75).abs() < 1e-12);
        assert_eq!(estimate.sample_size, 10);
        assert!(estimate.confidence.lower < estimate.probability);
        assert!(estimate.confidence.upper > estimate.probability);
    }

    #[test]
    fn resource_consistency_uses_observed_variation() {
        assert_eq!(resource_consistency(&[10.0, 10.0]).unwrap(), Some(1.0));
        assert!(resource_consistency(&[10.0, 20.0]).unwrap().unwrap() < 1.0);
    }
}
