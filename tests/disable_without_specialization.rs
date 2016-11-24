#[macro_use]
extern crate type_operators;

use std::fmt::Debug;

type_operators! {
    [A, B, C, D, E]

    #[derive(Default, Debug)]
    data Nat: Default + Debug {
        P,
        I(Nat = P),
        O(Nat = P),
        #[cfg(features = "specialization")]
        Error,
        #[cfg(features = "specialization")]
        DEFAULT,
    }

    (Sum) Adding(Nat, Nat): Nat {
        [P, P] => P
        forall (N: Nat) {
            [(O N), P] => (O N)
            [(I N), P] => (I N)
            [P, (O N)] => (O N)
            [P, (I N)] => (I N)
        }
        forall (N: Nat, M: Nat) {
            [(O M), (O N)] => (O (# M N))
            [(I M), (O N)] => (I (# M N))
            [(O M), (I N)] => (I (# M N))
            [(I M), (I N)] => (O (# (# M N) I))

            #[cfg(features = "specialization")] {
                {M, N} => Error
            }
        }
    }
}
