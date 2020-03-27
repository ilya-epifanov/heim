#[test]
fn logical_count() -> tests::Result<()> {
    let count = heim::cpu::logical_count()?;

    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            assert_eq!(count, tests::linux::nproc()?);
        }
    }

    Ok(())
}
