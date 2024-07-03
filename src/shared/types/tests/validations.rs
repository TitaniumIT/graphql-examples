use cucumber::{given, when,then, World};

use shared_types::{self, shared_types::EmailAddress};

#[given(expr="Email {word}")]
fn email(world: &mut EmailWorld, emailaddress:String) {
    world.given_email = emailaddress;
}

#[when("Created")]
fn created(world: &mut EmailWorld) {
    world.has_error = EmailAddress::new(&world.given_email).is_err();
}

#[then("Validation should not fail")]
fn validationoke(world: &mut EmailWorld) {
    if world.has_error {
        panic!("Validation failed for {}",world.given_email);
    }
}

#[derive(Debug, Default,World)]
// Accepts both sync/async and fallible/infallible functions.
pub struct EmailWorld {
    given_email: String,
    has_error: bool,
}

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(EmailWorld::run("tests/features/email/validation.feature"));
}
