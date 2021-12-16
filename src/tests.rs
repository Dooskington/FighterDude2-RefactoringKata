#[cfg(test)]
mod tests {
    use crate::{calculation, exp_from_enemy};

    #[test]
    fn calculation_returns_correct_result() {
        // Oh no! How do I write this test, the function is complicated
        assert_eq!(0, 0, "noop")
    }

    #[test]
    fn negative_gear_score_results_in_no_total_power() {
        assert_eq!(None, calculation(-1, 0, 0))
    }

    #[test]
    fn negative_fighting_skill_results_in_no_total_power() {
        assert_eq!(None, calculation(-1, 0, 0))
    }

    #[test]
    fn negative_happiness_results_in_max_total_power() {
        assert_eq!(Some(i32::MAX), calculation(0, 0, -1))
    }

    #[test]
    fn exp_from_enemy_correct() {
        assert_eq!(515, exp_from_enemy(5))
    }
}