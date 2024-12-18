use std::sync::Arc;

use common_error::DaftResult;

use super::optimizer::OptimizerRuleInBatch;
use crate::{
    optimization::{
        optimizer::{RuleBatch, RuleExecutionStrategy},
        Optimizer,
    },
    LogicalPlan,
};

/// Helper that creates an optimizer with the provided rules registered, optimizes
/// the provided plan with said optimizer, and compares the optimized plan with
/// the provided expected plan.
pub fn assert_optimized_plan_with_rules_eq(
    plan: Arc<LogicalPlan>,
    expected: Arc<LogicalPlan>,
    rules: Vec<Box<dyn OptimizerRuleInBatch>>,
) -> DaftResult<()> {
    let optimizer = Optimizer::with_rule_batches(
        vec![RuleBatch::new(rules, RuleExecutionStrategy::Once)],
        Default::default(),
    );
    let optimized_plan = optimizer
        .optimize_with_rules(optimizer.rule_batches[0].rules.as_slice(), plan.clone())?
        .data;
    assert_eq!(
        optimized_plan,
        expected,
        "\n\nOptimized plan not equal to expected.\n\nBefore Optimization:\n{}\n\nOptimized:\n{}\n\nExpected:\n{}",
        plan.repr_ascii(false),
        optimized_plan.repr_ascii(false),
        expected.repr_ascii(false)
    );

    Ok(())
}
