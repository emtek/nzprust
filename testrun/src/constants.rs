pub mod constants {
    pub const TD_A: f64 = 2.0;
    pub const TD_B: f64 = 20.0;
    pub const TD_PERIOD: f64 = 1096.0;
    pub const PN_MAX: f64 = 1.2;
    pub const PQ_MIN: f64 = 0.2;
    // TODO decide between constants and storing in the DB
    pub const Td_a: f64 = 0.0;
    pub const Td_b: f64 = 0.0;
    pub const Td_period: f64 = 0.0;
    pub const Pn_max: f64 = 0.0;
    pub const Pq_min: f64 = 0.0;

    /**
     * Get the value of Td_a
     *
     * @return the value of Td_a
     */
    fn getTd_a() -> f64 {
        return Td_a;
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
    fn getTd_b() -> f64 {
        return Td_b;
    }

    /**
     * Set the value of Td_b
     *
     * @param Td_b new value of Td_b
     */
    // fn setTd_b(td_b: f64) {
    //     Td_b = td_b;
    // }

    pub(crate) fn getTd_period() -> f64 {
        return Td_period;
    }

    // fn setTd_period(td_period: f64) {
    //     Td_period = td_period;
    // }

    pub fn getPn_max() -> f64 {
        return Pn_max;
    }

    // fn setPn_max(pn_max: f64) {
    //     Pn_max = pn_max;
    // }

    /**
     * Get the value of Pq_min
     *
     * @return the value of Pq_min
     */
    fn getPq_min() -> f64 {
        return Pq_min;
    }
}
