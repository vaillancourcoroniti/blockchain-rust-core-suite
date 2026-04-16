use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::{PrimeField, Zero, One};
use ark_relations::r1cs::{
    ConstraintSystemRef, SynthesisError, Constraint, Variable
};
use ark_groth16::{
    create_random_proof, generate_random_parameters,
    verify_proof, Proof, ProvingKey, VerifyingKey
};
use rand::thread_rng;

#[derive(Clone)]
pub struct ZkCircuit<F: PrimeField> {
    pub secret: F,
    pub public: F,
}

impl<F: PrimeField> ark_relations::r1cs::ConstraintSynthesizer<F> for ZkCircuit<F> {
    fn generate_constraints<CS: ark_relations::r1cs::ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let secret_var = cs.new_witness_variable(|| Ok(self.secret))?;
        let public_var = cs.new_input_variable(|| Ok(self.public))?;
        
        cs.enforce_constraint(
            (secret_var, F::one()),
            (secret_var, F::one()),
            (public_var, F::one()),
        )?;
        
        Ok(())
    }
}

pub struct ZkSnarksCore {
    proving_key: ProvingKey<Bls12_381>,
    verifying_key: VerifyingKey<Bls12_381>,
}

impl ZkSnarksCore {
    pub fn new() -> Result<Self, SynthesisError> {
        let rng = &mut thread_rng();
        let circuit = ZkCircuit {
            secret: Fr::zero(),
            public: Fr::zero(),
        };
        let params = generate_random_parameters(circuit, rng)?;
        
        Ok(Self {
            proving_key: params.proving_key,
            verifying_key: params.verifying_key,
        })
    }

    pub fn generate_proof(&self, secret: Fr, public: Fr) -> Result<Proof<Bls12_381>, SynthesisError> {
        let rng = &mut thread_rng();
        let circuit = ZkCircuit { secret, public };
        let proof = create_random_proof(circuit, &self.proving_key, rng)?;
        Ok(proof)
    }

    pub fn verify(&self, public: Fr, proof: &Proof<Bls12_381>) -> Result<bool, SynthesisError> {
        let result = verify_proof(&self.verifying_key, proof, &[public])?;
        Ok(result)
    }
}
