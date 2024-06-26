use halo2_proofs::{
    circuit::Value,
    pasta::{EqAffine, Fp},
    plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, Error, SingleVerifier},
    poly::commitment::Params,
    transcript::{Blake2bRead, Blake2bWrite, Challenge255},
};
use iced::widget::{Button, Column, TextInput};
use iced::{Alignment, Element, Sandbox, Settings};

use hammster2::circuit::{empty_circuit, HammsterCircuit};
use rand_core::OsRng;
use std::fs;

pub fn main() -> iced::Result {
    Hammster::run(Settings::default())
}
#[derive(Default)]
struct Hammster {
    input_a: String,
    input_b: String,
    hamming_distance: String,
}
#[derive(Debug, Clone)]
pub enum Message {
    InputAChanged(String),
    InputBChanged(String),
    HammingDistanceChanged(String),
    ButtonPressed,
}

impl Sandbox for Hammster {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Hammster")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputAChanged(value) => {
                self.input_a = value;
            }
            Message::InputBChanged(value) => {
                self.input_b = value;
            }
            Message::HammingDistanceChanged(value) => {
                self.hamming_distance = value;
            }
            Message::ButtonPressed => {
                let input_a: Vec<_> = self
                    .input_a
                    .chars()
                    .map(|ch| Value::known(Fp::from(ch.to_digit(10).unwrap() as u64)))
                    .collect();
                let input_b: Vec<_> = self
                    .input_b
                    .chars()
                    .map(|ch| Value::known(Fp::from(ch.to_digit(10).unwrap() as u64)))
                    .collect();
                let hamming_distance =
                    vec![Fp::from(self.hamming_distance.parse::<u64>().unwrap())];

                let proof = generate_proof(&hamming_distance, input_a, input_b);
                println!("proof generated");
                write_vec_to_file(proof, "proof").expect("create proof file should success");
                let proof = fs::read("proof").unwrap();
                match verify(&hamming_distance, &proof) {
                    Ok(_) => println!("verify success"),
                    Err(_) => println!("verify failed"),
                };
            }
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                TextInput::new("Enter 8 bits...", &self.input_a)
                    .on_input(Message::InputAChanged)
                    .padding(10),
            )
            .push(
                TextInput::new("Enter 8 bits...", &self.input_b)
                    .on_input(Message::InputBChanged)
                    .padding(10),
            )
            .push(
                TextInput::new("Enter hamming distance", &self.hamming_distance)
                    .on_input(Message::HammingDistanceChanged)
                    .padding(10),
            )
            .push(Button::new("Prove").on_press(Message::ButtonPressed))
            .into()
    }
}

pub fn generate_proof(
    pub_input: &Vec<Fp>,
    input_a: Vec<Value<Fp>>,
    input_b: Vec<Value<Fp>>,
) -> Vec<u8> {
    println!("Generating proof...");
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    let k = 5;
    let params = Params::<EqAffine>::new(k);
    let circuit = HammsterCircuit { input_a, input_b };
    let vk = keygen_vk(&params, &circuit).unwrap();

    let pk = keygen_pk(&params, vk, &circuit).unwrap();

    create_proof(
        &params,
        &pk,
        &[circuit],
        &[&[pub_input]],
        OsRng,
        &mut transcript,
    )
    .expect("Prover should not fail");
    transcript.finalize()
}
pub fn verify(pub_input: &Vec<Fp>, proof: &Vec<u8>) -> Result<(), Error> {
    println!("Verifying proof...");

    let k = 5;
    let params = Params::<EqAffine>::new(k);
    let strategy = SingleVerifier::new(&params);
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    let circuit = empty_circuit();
    let vk = keygen_vk(&params, &circuit).unwrap();

    verify_proof(&params, &vk, strategy, &[&[pub_input]], &mut transcript)
}
fn write_vec_to_file(data: Vec<u8>, file_name: &str) -> Result<(), Error> {
    fs::write(file_name, data)?;
    Ok(())
}
