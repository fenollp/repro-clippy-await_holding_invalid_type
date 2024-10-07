pub struct CustomLockType;
pub struct OtherCustomLockType;
pub async fn foo() {
    let _x = CustomLockType;
    let _y = OtherCustomLockType;
    baz().await; // Lint violation
}

async fn baz() -> u32 {
    42
}
