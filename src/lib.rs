use othercrate::Ctx;

pub struct CustomLockType;

#[expect(clippy::await_holding_invalid_type)]
pub async fn foo() {
    let x = CustomLockType;
    baz().await; // Lint violation
    drop(x);
}

#[expect(clippy::await_holding_invalid_type)]
pub async fn quu() {
    let y = othercrate::OtherCustomLockType;
    baz().await; // Lint violation
    drop(y);
}

#[expect(clippy::await_holding_invalid_type)]
pub async fn qux() {
    let y = othercrate::OtherCustomLockTypeBis {};
    baz().await; // Lint violation
    drop(y);
}

#[expect(clippy::await_holding_invalid_type)]
pub async fn quxx() {
    let y = othercrate::new_bis();
    baz().await; // Lint violation
    drop(y);
}

#[expect(clippy::await_holding_invalid_type)]
pub async fn quxxx() {
    let y = othercrate::try_new_bis().unwrap();
    baz().await; // Lint violation
    drop(y);
}

//
// vvv
//

#[allow(unused_mut)]
#[expect(clippy::await_holding_invalid_type)]
pub async fn ah(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = ctx.db_conn().await?;

    baz().await; // Lint violation

    println!(">>> {:#?}", &mut conn);
    drop(conn);

    Ok(())
}

//
// ^^^
//

async fn baz() -> u32 {
    42
}
