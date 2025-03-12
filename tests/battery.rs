use enum_stringify::EnumStringify;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(EnumStringify, Debug, PartialEq)]
enum Numbers {
    One,
    Two,
}

#[test]
fn test_display() {
    assert_eq!(Numbers::One.to_string(), "One");
    assert_eq!(Numbers::Two.to_string(), "Two");
}

#[test]
fn test_try_from_str() {
    assert_eq!(Numbers::try_from("One").unwrap(), Numbers::One);
    assert_eq!(Numbers::try_from("Two").unwrap(), Numbers::Two);
    assert!(Numbers::try_from("Three").is_err());
}

#[test]
fn test_try_from_string() {
    assert_eq!(Numbers::try_from("One".to_string()).unwrap(), Numbers::One);
    assert_eq!(Numbers::try_from("Two".to_string()).unwrap(), Numbers::Two);
    assert!(Numbers::try_from("Three".to_string()).is_err());
}

#[test]
fn test_from_str() {
    assert_eq!(Numbers::from_str("One").unwrap(), Numbers::One);
    assert_eq!(Numbers::from_str("Two").unwrap(), Numbers::Two);
    assert!(Numbers::from_str("Three").is_err());
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "Pre", suffix = "Post")]
enum Status {
    Okk,
    Error3,
}

#[test]
fn test_prefix_suffix() {
    assert_eq!(Status::Okk.to_string(), "PreOkkPost");
    assert_eq!(Status::Error3.to_string(), "PreError3Post");

    assert_eq!(Status::try_from("PreOkkPost").unwrap(), Status::Okk);
    assert_eq!(Status::try_from("PreError3Post").unwrap(), Status::Error3);
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "upper")]
enum Letters {
    A,
    B,
}

#[test]
fn test_case_conversion_upper() {
    assert_eq!(Letters::A.to_string(), "A");
    assert_eq!(Letters::B.to_string(), "B");

    assert_eq!(Letters::try_from("A").unwrap(), Letters::A);
    assert_eq!(Letters::try_from("B").unwrap(), Letters::B);
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "lower")]
enum Colors {
    Red,
    Blue,
}

#[test]
fn test_case_conversion_lower() {
    assert_eq!(Colors::Red.to_string(), "red");
    assert_eq!(Colors::Blue.to_string(), "blue");

    assert_eq!(Colors::try_from("red").unwrap(), Colors::Red);
    assert_eq!(Colors::try_from("blue").unwrap(), Colors::Blue);
}

#[derive(EnumStringify, Debug, PartialEq)]
enum Istari {
    #[enum_stringify(rename = "Ólorin")]
    Gandalf,
    Saruman,
}

#[test]
fn test_rename_variants() {
    assert_eq!(Istari::Gandalf.to_string(), "Ólorin");
    assert_eq!(Istari::try_from("Ólorin").unwrap(), Istari::Gandalf);
    assert_eq!(Istari::Saruman.to_string(), "Saruman");
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "Pre", suffix = "Post", case = "upper_flat")]
enum Severity {
    #[enum_stringify(rename = "critical")]
    High,
    Low,
}

#[test]
fn test_all_options() {
    assert_eq!(Severity::High.to_string(), "critical");
    assert_eq!(Severity::Low.to_string(), "PRELOWPOST");

    assert_eq!(Severity::try_from("critical").unwrap(), Severity::High);
    assert_eq!(Severity::try_from("PRELOWPOST").unwrap(), Severity::Low);
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "Pre", suffix = "Post", case = "upper_flat")]
enum Response {
    #[enum_stringify(rename = "okay")]
    Success,
    ErroR,
}

#[test]
fn test_combined_prefix_suffix_case_rename() {
    assert_eq!(Response::Success.to_string(), "okay");
    assert_eq!(Response::ErroR.to_string(), "PREERRORPOST");

    assert_eq!(Response::try_from("okay").unwrap(), Response::Success);
    assert_eq!(Response::try_from("PREERRORPOST").unwrap(), Response::ErroR);
}

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
#[enum_stringify(case = "upper_flat")]
enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

#[test]
fn test_upper_flat_case() {
    assert_eq!(Season::Spring.to_string(), "SPRING");
    assert_eq!(Season::Summer.to_string(), "SUMMER");
    assert_eq!(Season::Fall.to_string(), "FALL");
    assert_eq!(Season::Winter.to_string(), "WINTER");

    assert_eq!(Season::try_from("SPRING").unwrap(), Season::Spring);
    assert_eq!(Season::try_from("SUMMER").unwrap(), Season::Summer);
    assert_eq!(Season::try_from("FALL").unwrap(), Season::Fall);
    assert_eq!(Season::try_from("WINTER").unwrap(), Season::Winter);
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
enum SpecialChars {
    #[enum_stringify(rename = "Hello, World!")]
    HelloWorld,

    #[enum_stringify(rename = "100%Success")]
    FullSuccess,

    #[enum_stringify(rename = "Café")]
    Cafe,
}

#[test]
fn test_special_characters() {
    assert_eq!(SpecialChars::HelloWorld.to_string(), "Hello, World!");
    assert_eq!(SpecialChars::FullSuccess.to_string(), "100%Success");
    assert_eq!(SpecialChars::Cafe.to_string(), "Café");

    assert_eq!(
        SpecialChars::try_from("Hello, World!").unwrap(),
        SpecialChars::HelloWorld
    );
    assert_eq!(
        SpecialChars::try_from("100%Success").unwrap(),
        SpecialChars::FullSuccess
    );
    assert_eq!(SpecialChars::try_from("Café").unwrap(), SpecialChars::Cafe);
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

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "", suffix = "")]
enum Punctuated {
    Excited,
    Happy,
}

#[test]
fn test_empty_suffix_prefix() {
    assert_eq!(Punctuated::Excited.to_string(), "Excited");
    assert_eq!(Punctuated::Happy.to_string(), "Happy");

    assert_eq!(
        Punctuated::try_from("Excited").unwrap(),
        Punctuated::Excited
    );
    assert_eq!(Punctuated::try_from("Happy").unwrap(), Punctuated::Happy);
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "😀", suffix = "🥳")]
enum UnicodeEnumRename {
    #[enum_stringify(rename = "日本語")]
    Japanese,

    #[enum_stringify(rename = "🌟 Star")]
    Star,
}

#[test]
fn test_unicode_rename() {
    assert_eq!(UnicodeEnumRename::Japanese.to_string(), "日本語");
    assert_eq!(UnicodeEnumRename::Star.to_string(), "🌟 Star");

    assert_eq!(
        UnicodeEnumRename::try_from("日本語").unwrap(),
        UnicodeEnumRename::Japanese
    );
    assert_eq!(
        UnicodeEnumRename::try_from("🌟 Star").unwrap(),
        UnicodeEnumRename::Star
    );
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "😀", suffix = "🥳")]
enum UnicodeEnum {
    Japanese,
    Star,
}

#[test]
fn test_unicode() {
    assert_eq!(UnicodeEnum::Japanese.to_string(), "😀Japanese🥳");
    assert_eq!(UnicodeEnum::Star.to_string(), "😀Star🥳");

    assert_eq!(
        UnicodeEnum::try_from("😀Japanese🥳").unwrap(),
        UnicodeEnum::Japanese
    );
    assert_eq!(
        UnicodeEnum::try_from("😀Star🥳").unwrap(),
        UnicodeEnum::Star
    );
}

#[derive(EnumStringify, Debug, PartialEq)]
enum LargeEnum {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    B10,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
}

#[test]
fn large_enum() {
    assert_eq!(LargeEnum::A1.to_string(), "A1");
    assert_eq!(LargeEnum::C10.to_string(), "C10");

    assert_eq!(LargeEnum::try_from("B5").unwrap(), LargeEnum::B5);
    assert!(LargeEnum::try_from("Z100").is_err());
}

#[derive(EnumStringify, Debug, PartialEq)]
enum EmptyString {
    #[enum_stringify(rename = "")]
    Silent,
}

#[test]
fn empty_rename() {
    assert_eq!(EmptyString::Silent.to_string(), "");
    assert_eq!(EmptyString::try_from("").unwrap(), EmptyString::Silent);
}

#[derive(EnumStringify, Debug, PartialEq)]
enum FuzzyMatch {
    Alpha,
    Beta,
    Gamma,
}

#[test]
fn wrong_similar_names() {
    assert!(FuzzyMatch::try_from("alpha ").is_err()); // Extra space
    assert!(FuzzyMatch::try_from("ALPHA").is_err()); // Wrong case
    assert!(FuzzyMatch::try_from("alpHa").is_err()); // Mixed case
    assert!(FuzzyMatch::try_from("Alphaa").is_err()); // Extra character
}
