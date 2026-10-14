use one_core::{run_process, Capability, CapabilitySet, ProcessSpec};

#[test]
fn process_capability_is_denied_by_default() {
    let spec = ProcessSpec { program: "echo".into(), args: vec!["hello".into()], working_directory: None };
    assert!(run_process(&spec, &CapabilitySet::none()).is_err());
}

#[test]
fn process_result_preserves_exit_and_output_when_allowed() {
    let capability = CapabilitySet::none().allow(Capability::Process);
    let spec = if cfg!(target_os = "windows") {
        ProcessSpec { program: "cmd".into(), args: vec!["/C".into(), "echo hello".into()], working_directory: None }
    } else {
        ProcessSpec { program: "printf".into(), args: vec!["hello".into()], working_directory: None }
    };
    let result = run_process(&spec, &capability).expect("process should run");
    assert!(result.success);
    assert_eq!(result.status_code, Some(0));
    assert!(result.stdout.contains("hello"));
}
