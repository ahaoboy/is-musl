pub fn is_musl() -> bool {
  if std::env::consts::OS == "linux" {
      if is_musl_from_filesystem() == Some(true) {
          return true;
      }
      if is_musl_from_child_process() == Some(true) {
          return true;
      }
  }

  false
}


fn is_musl_from_filesystem() -> Option<bool> {
  std::fs::read_to_string("/usr/bin/ldd")
      .map(|i| i.contains("musl"))
      .ok()
}

fn is_musl_from_child_process() -> Option<bool> {
  let s = std::process::Command::new("ldd").arg("--version").output();
  s.ok()
      .and_then(|i| String::from_utf8(i.stdout).ok().map(|s| s.contains("musl")))
}
