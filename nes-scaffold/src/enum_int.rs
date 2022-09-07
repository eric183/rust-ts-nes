struct StudyWhat(String, String);

struct DoOtherThings { work: bool, eat: String }

fn enum_int() {
  enum EricTorrowMay {
    Contact,
    Study(String),
    DoOthers { work: bool, eat: String }
  }
}

fn enum_detail() {
  enum EricTomorrow {
    ContactSomeone(bool),
    Study(StudyWhat),
    DoOthers(DoOtherThings)
  }

  let study = StudyWhat("Rust", "Blender");

  let MeDo = DoOtherThings { work: false, eat: String::from("bread") };

  let MeDoOthers = enum_detail::DoOthers(MeDo);

  let MeStudy = enum_detail::Study(study);
}