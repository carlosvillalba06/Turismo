#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Vec};

#[contract]
pub struct TurismoJsContract;

#[contractimpl]
impl TurismoJsContract {
    pub fn hola(env: Env, nombre: String) -> Vec<String> {
        Vec::from_array(
            &env,
            [
                String::from_str(&env, "¡Hola, "),
                nombre,
                String::from_str(&env, "! Bienvenido al proyecto Turismo JS "),
            ],
        )
    }
}
