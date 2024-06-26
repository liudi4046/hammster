use halo2_proofs::{
    pasta::{EqAffine, Fp},
    plonk::{keygen_vk, verify_proof, SingleVerifier},
    poly::commitment::Params,
    transcript::{Blake2bRead, Challenge255},
};

use wasm_bindgen::prelude::*;

use crate::circuit::empty_circuit;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn verify(pub_input: &[u32], proof: &[u8]) -> Result<(), JsValue> {
    println!("Verifying proof...");
    print_u32_array(pub_input);
    print_u8_array(proof);
    let pub_input: Vec<_> = pub_input.iter().map(|x| Fp::from(*x as u64)).collect();

    let k = 5;
    let params = Params::<EqAffine>::new(k);
    let strategy = SingleVerifier::new(&params);
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(proof);
    let circuit = empty_circuit();
    let vk = keygen_vk(&params, &circuit).unwrap();

    verify_proof(&params, &vk, strategy, &[&[&pub_input]], &mut transcript)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
pub fn print_u32_array(arr: &[u32]) {
    let string_representation: String = arr
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    log(&format!("u32 array: [{}]", string_representation));
}
pub fn print_u8_array(arr: &[u8]) {
    let string_representation: String = arr
        .iter()
        .map(|&byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join(", ");
    log(&format!("u8 array: [{}]", string_representation));
}
