

// use bellman::groth16::{
//     create_random_proof, verify_proof, Parameters, PreparedVerifyingKey, Proof,
//     prepare_verifying_key, generate_random_parameters,
// };
// use byteorder::{LittleEndian, ReadBytesExt};
// use pairing::{
//     bls12_381::{Bls12, Fr, FrRepr},
//     Field, PrimeField, PrimeFieldRepr, Engine,
// };
// use rand::{thread_rng, Rng, ChaChaRng};
// use scrypto::{
//     circuit::{
//         multipack,
//         sapling::{Output, Spend, TREE_DEPTH},
//     },
//     jubjub::{edwards, fs::Fs, FixedGenerators, JubjubBls12, Unknown},
//     primitives::{Diversifier, Note, PaymentAddress, ProofGenerationKey, ValueCommitment},
//     redjubjub::{PrivateKey, PublicKey, Signature},
// };


// pub struct ProvingContext {
//     bsk: Fs,
//     bvk: edwards::Point<Bls12, Unknown>,
// }

// impl ProvingContext {
//     pub fn new() -> Self {
//         ProvingContext {
//             bsk: Fs::zero(),
//             bvk: edwards::Point::zero(),
//         }
//     }

//     pub fn spend_proof(
//         &mut self, 
//         old_value: u64, 
//         sender_value: u64,
//         receiver_value: u64,
//         proving_key: &Parameters<Bls12>, 
//         verifying_key: &PreparedVerifyingKey<Bls12>,
//         proof_generation_key: ProofGenerationKey<Bls21>,
//         params: &JubjubBls12,
//         public_key: [u8; 32],
//     ) -> Result<
//         (
//             Proof<Bls12>,
//             edwards::Point<Bls12, Unknown>,
//             PublicKey<Bls12>,
//         ),
//         (),    
//     >{
//         let mut rng = OsRng::new().expect("should be able to construct RNG");
//         let note = Note {
//             value,
//             public_key,
//         };
//         let nullifier = note.nf();

//         let instance =

//         // Crate proof
//         let proof = create_random_proof(, proving_key, &mut rng).expect("proving should not fail");
//     }

//     pub fn output_proof() {

//     }
// }
