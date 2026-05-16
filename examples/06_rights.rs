use localauthentication::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let right = LARight::new()?;
    right.set_tag(42)?;

    println!("state: {:?}", right.state()?);
    println!("tag: {}", right.tag()?);
    println!("check_can_authorize: {:?}", right.check_can_authorize());
    right.deauthorize()?;
    println!("state after deauthorize: {:?}", right.state()?);
    println!("✅ right lifecycle OK");
    Ok(())
}
