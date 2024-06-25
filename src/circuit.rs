use std::{alloc::Layout, marker::PhantomData};

use halo2_proofs::{
    circuit::{layouter, AssignedCell, Cell, Layouter, SimpleFloorPlanner, Value},
    pasta::group::ff::PrimeField,
    pasta::Fp,
    plonk::*,
    poly::Rotation,
};
#[derive(Clone)]
pub struct HammsterChipConfig {
    hamming_distance: Column<Advice>,
    sum: Column<Advice>,
    input_a: Column<Advice>,
    input_b: Column<Advice>,
    s1: Selector,
    s2: Selector,
    s_sum: Selector,
    s_hamming: Selector,
    instance: Column<Instance>,
}
pub struct HammsterChip<F: PrimeField> {
    config: HammsterChipConfig,
    _marker: PhantomData<F>,
}
const BINARY_LENGTH: usize = 8;

impl<F: PrimeField> HammsterChip<F> {
    fn construct(config: HammsterChipConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }
    fn configure(meta: &mut ConstraintSystem<F>) -> HammsterChipConfig {
        let hamming_distance = meta.advice_column();
        let sum = meta.advice_column();
        let input_a = meta.advice_column();
        let input_b = meta.advice_column();
        let s1 = meta.selector();
        let s2 = meta.selector();
        let s_sum = meta.selector();
        let s_hamming = meta.selector();
        let instance = meta.instance_column();

        meta.enable_equality(hamming_distance);
        meta.enable_equality(instance);

        let ZERO = Expression::Constant(F::ZERO);
        let ONE = Expression::Constant(F::ONE);
        let TWO = Expression::Constant(F::from(2));

        meta.create_gate("binary check gate", |meta| {
            let bit1 = meta.query_advice(input_a, Rotation::cur());
            let selector1 = meta.query_selector(s1);
            let bit2 = meta.query_advice(input_b, Rotation::cur());
            let selector2 = meta.query_selector(s2);

            vec![
                selector1 * bit1.clone() * (ONE.clone() - bit1),
                selector2 * bit2.clone() * (ONE - bit2),
            ]
        });
        meta.create_gate("sum check gate", |meta| {
            let s_sum = meta.query_selector(s_sum);
            let sum = meta.query_advice(sum, Rotation::cur());
            let bit1 = meta.query_advice(input_a, Rotation::cur());
            let bit2 = meta.query_advice(input_b, Rotation::cur());
            vec![s_sum * (sum - (bit1.clone() + bit2.clone() - TWO * bit1 * bit2))]
        });
        meta.create_gate("hamming distance check gate", |meta| {
            let s_hamming = meta.query_selector(s_hamming);
            let hamming_distance_input = meta.query_advice(hamming_distance, Rotation::cur());
            let res = (0..BINARY_LENGTH as i32)
                .map(|x| meta.query_advice(sum, Rotation(x)))
                .fold(ZERO, |acc, val| acc + val);

            vec![s_hamming * (res - hamming_distance_input)]
        });
        HammsterChipConfig {
            hamming_distance,
            sum,
            input_a,
            input_b,
            s1,
            s2,
            s_sum,
            s_hamming,
            instance,
        }
    }
    fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        input_a: &Vec<Value<F>>,
        input_b: &Vec<Value<F>>,
    ) -> Result<AssignedCell<F, F>, Error> {
        let input_a_col = self.config.input_a;
        let input_b_col = self.config.input_b;
        let sum_col = self.config.sum;
        let hamming_distance_col = self.config.hamming_distance;

        let s_1 = self.config.s1;
        let s_2 = self.config.s2;
        let s_sum = self.config.s_sum;
        let s_hamming = self.config.s_hamming;
        layouter.assign_region(
            || "hamming distance region",
            |mut region| {
                for i in 0..BINARY_LENGTH {
                    region.assign_advice(|| "input 1", input_a_col, i, || input_a[i]);
                    region.assign_advice(|| "input 2", input_b_col, i, || input_b[i]);
                    region.assign_advice(|| "sum", sum_col, i, || input_a[i] + input_b[i]);

                    self.config.s1.enable(&mut region, i);
                    self.config.s2.enable(&mut region, i);
                    self.config.s_sum.enable(&mut region, i);
                }
                let hamming_distance = input_a
                    .iter()
                    .zip(input_b.iter())
                    .map(|(a, b)| *a + *b)
                    .fold(Value::known(F::ZERO), |acc, val| acc + val);

                let hamming_distance_cell = region.assign_advice(
                    || "hamming distance",
                    hamming_distance_col,
                    0,
                    || hamming_distance,
                )?;

                self.config.s_sum.enable(&mut region, 0);
                Ok(hamming_distance_cell)
            },
        )
    }
    fn expose_public(&self, mut layouter: impl Layouter<F>, cell: Cell) -> Result<(), Error> {
        layouter.constrain_instance(cell, self.config.instance, 0)
    }
}
#[derive(Clone)]
pub struct HammsterCircuitConfig {
    hammsterchip_config: HammsterChipConfig,
}
#[derive(Default)]
pub struct HammsterCircuit<F: PrimeField> {
    pub input_a: Vec<Value<F>>,
    pub input_b: Vec<Value<F>>,
}
impl<F: PrimeField> Circuit<F> for HammsterCircuit<F> {
    type Config = HammsterCircuitConfig;
    type FloorPlanner = SimpleFloorPlanner;
    fn without_witnesses(&self) -> Self {
        HammsterCircuit::default()
    }
    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        HammsterCircuitConfig {
            hammsterchip_config: HammsterChip::configure(meta),
        }
    }
    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let hammster_chip = HammsterChip::construct(config.hammsterchip_config);
        let hamming_distance_cell = hammster_chip.assign(
            layouter.namespace(|| "hamming distance layouter"),
            &self.input_a,
            &self.input_b,
        )?;
        hammster_chip.expose_public(
            layouter.namespace(|| "expose public layouter"),
            hamming_distance_cell.cell(),
        )?;
        Ok(())
    }
}
pub fn empty_circuit() -> HammsterCircuit<Fp> {
    HammsterCircuit {
        input_a: vec![Value::unknown(); BINARY_LENGTH],
        input_b: vec![Value::unknown(); BINARY_LENGTH],
    }
}
#[cfg(test)]
mod test {
    use halo2_proofs::{
        arithmetic::Field,
        circuit::{layouter, Layouter, SimpleFloorPlanner, Value},
        dev::MockProver,
        pasta::Fp,
        plonk::*,
        poly::Rotation,
    };

    use super::HammsterCircuit;

    #[test]
    fn test_circuit() {
        let k = 5;
        let input_a = vec![
            Value::known(Fp::ONE),
            Value::known(Fp::ONE),
            Value::known(Fp::ONE),
            Value::known(Fp::ONE),
            Value::known(Fp::ONE),
            Value::known(Fp::ONE),
            Value::known(Fp::ONE),
            Value::known(Fp::ONE),
        ];
        let input_b = vec![
            Value::known(Fp::zero()),
            Value::known(Fp::zero()),
            Value::known(Fp::zero()),
            Value::known(Fp::zero()),
            Value::known(Fp::zero()),
            Value::known(Fp::zero()),
            Value::known(Fp::zero()),
            Value::known(Fp::zero()),
        ];
        let circuit = HammsterCircuit { input_a, input_b };
        let prover = MockProver::run(k, &circuit, vec![vec![Fp::from(7)]]).unwrap();
        prover.assert_satisfied();
    }
}
