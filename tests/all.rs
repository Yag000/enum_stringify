use enum_stringify::EnumStringify;
use std::convert::TryFrom;

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "flat", prefix = "Pre", suffix = "Post")]
enum Action {
    Start,
    Stop,
}

#[test]
fn test_flat_case_with_prefix_suffix() {
    assert_eq!(Action::Start.to_string(), "prestartpost");
    assert_eq!(Action::Stop.to_string(), "prestoppost");

    assert_eq!(Action::try_from("prestartpost").unwrap(), Action::Start);
    assert_eq!(Action::try_from("prestoppost").unwrap(), Action::Stop);
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "upper_flat", prefix = "Start", suffix = "End")]
enum Phase {
    Begin,
    End,
}

#[test]
fn test_upper_flat_case_with_prefix_suffix() {
    assert_eq!(Phase::Begin.to_string(), "STARTBEGINEND");
    assert_eq!(Phase::End.to_string(), "STARTENDEND");

    assert_eq!(Phase::try_from("STARTBEGINEND").unwrap(), Phase::Begin);
    assert_eq!(Phase::try_from("STARTENDEND").unwrap(), Phase::End);
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "camel", prefix = "Begin", suffix = "Finish")]
enum Process {
    StepOne,
    StepTwo,
}

#[test]
fn test_camel_case_with_prefix_suffix() {
    assert_eq!(Process::StepOne.to_string(), "beginStepOneFinish");
    assert_eq!(Process::StepTwo.to_string(), "beginStepTwoFinish");

    assert_eq!(
        Process::try_from("beginStepOneFinish").unwrap(),
        Process::StepOne
    );
    assert_eq!(
        Process::try_from("beginStepTwoFinish").unwrap(),
        Process::StepTwo
    );
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "kebab", prefix = "start-", suffix = "-end")]
enum Task {
    Initialize,
    Complete,
}

#[test]
fn test_kebab_case_with_prefix_suffix() {
    assert_eq!(Task::Initialize.to_string(), "start-initialize-end");
    assert_eq!(Task::Complete.to_string(), "start-complete-end");

    assert_eq!(
        Task::try_from("start-initialize-end").unwrap(),
        Task::Initialize
    );
    assert_eq!(
        Task::try_from("start-complete-end").unwrap(),
        Task::Complete
    );
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "snake", prefix = "task_", suffix = "_done")]
enum Operation {
    Start,
    Stop,
}

#[test]
fn test_snake_case_with_prefix_suffix() {
    assert_eq!(Operation::Start.to_string(), "task_start_done");
    assert_eq!(Operation::Stop.to_string(), "task_stop_done");

    assert_eq!(
        Operation::try_from("task_start_done").unwrap(),
        Operation::Start
    );
    assert_eq!(
        Operation::try_from("task_stop_done").unwrap(),
        Operation::Stop
    );
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "upper_flat", prefix = "Prefix", suffix = "Suffix")]
enum Level {
    Beginner,
    Expert,
}

#[test]
fn test_upper_case_with_prefix_suffix() {
    assert_eq!(Level::Beginner.to_string(), "PREFIXBEGINNERSUFFIX");
    assert_eq!(Level::Expert.to_string(), "PREFIXEXPERTSUFFIX");

    assert_eq!(
        Level::try_from("PREFIXBEGINNERSUFFIX").unwrap(),
        Level::Beginner
    );
    assert_eq!(
        Level::try_from("PREFIXEXPERTSUFFIX").unwrap(),
        Level::Expert
    );
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "flat")]
enum CaseEnum {
    FirstCase,
    SecondCase,
}

#[test]
fn test_special_characters_errors_detected() {
    assert_eq!(CaseEnum::FirstCase.to_string(), "firstcase");
    assert_eq!(CaseEnum::SecondCase.to_string(), "secondcase");

    assert_eq!(
        CaseEnum::try_from("firstcase").unwrap(),
        CaseEnum::FirstCase
    );
    assert_eq!(
        CaseEnum::try_from("secondcase").unwrap(),
        CaseEnum::SecondCase
    );

    // Ensure incorrect casing fails
    assert!(CaseEnum::try_from("FirstCase").is_err());
    assert!(CaseEnum::try_from("SECONDCASE").is_err());
}
