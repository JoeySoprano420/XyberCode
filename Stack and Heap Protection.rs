fn check_stack_canary(stack: &mut Vec<u8>, canary_value: u8) -> Result<(), String> {
    let stack_canary = stack.last();
    if stack_canary != Some(&canary_value) {
        return Err("Stack canary mismatch - possible stack overflow".to_string());
    }
    Ok(())
}
