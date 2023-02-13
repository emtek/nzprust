#[allow(dead_code)]
pub mod constants {
    pub const TD_A: f64 = 2.0;
    pub const TD_B: f64 = 20.0;
    pub const TD_PERIOD: f64 = 1096.0;
    pub const PN_MAX: f64 = 1.2;
    pub const PQ_MIN: f64 = 0.2;

    /**
     * Get the value of Td_a
     *
     * @return the value of Td_a
     */
    fn get_td_a() -> f64 {
        return TD_A;
    }

    /**
     * Set the value of Td_a
     *
     * @param Td_a new value of Td_a
     */
    // fn setTd_a(td_a: f64) {
    //     Td_a = td_a;
    // }

    /**
     * Get the value of Td_b
     *
     * @return the value of Td_b
     */
    fn get_td_b() -> f64 {
        return TD_B;
    }

    /**
     * Set the value of Td_b
     *
     * @param Td_b new value of Td_b
     */
    // fn setTd_b(td_b: f64) {
    //     Td_b = td_b;
    // }

    pub(crate) fn get_td_period() -> f64 {
        return TD_PERIOD;
    }

    // fn setTd_period(td_period: f64) {
    //     Td_period = td_period;
    // }

    pub fn get_pn_max() -> f64 {
        return PN_MAX;
    }

    // fn setPn_max(pn_max: f64) {
    //     Pn_max = pn_max;
    // }

    /**
     * Get the value of Pq_min
     *
     * @return the value of Pq_min
     */
    fn get_pq_min() -> f64 {
        return PQ_MIN;
    }
}
