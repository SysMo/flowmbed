extern crate embflow_shared;
use embflow_shared::messages as msg;
use embflow_shared::messages::{dynsys as rdf};

fn main() {
    msg::SystemMessage::test_round_trip(
        msg::SystemMessage::command(
            rdf::ParamSet::Real(10, 4.5).into()
        )
    );

    msg::SystemMessage::test_round_trip(
        msg::SystemMessage::query(
            rdf::ParamGet::Real(10).into()
        )
    );
}