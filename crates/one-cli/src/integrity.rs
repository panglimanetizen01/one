use std::{fs, path::{Path, PathBuf}, process::Command};

#[derive(Debug)]
struct Finding { detector: &'static str, rule: &'static str, severity: &'static str, path: String, line: usize, summary: String, evidence: String, fix: String }
const DETECTORS: &[&str] = &["revision-integrity","path-integrity","file-integrity","markdown-links","markdown-structure","workflow-integrity","contract-drift","artifact-integrity"];
fn selected(scope: &str) -> Option<Vec<&'static str>> { Some(match scope {
    "all" => DETECTORS.to_vec(), "sha" => vec!["revision-integrity"], "workflow" => vec!["workflow-integrity"],
    "structure" => vec!["path-integrity","file-integrity","contract-drift"],
    "documentation" => vec!["markdown-links","markdown-structure","file-integrity"],
    "artifacts" => vec!["artifact-integrity","revision-integrity"], "contracts" => vec!["contract-drift","file-integrity"],
    "markdown" => vec!["markdown-links","markdown-structure"], "readme" => vec!["markdown-links","markdown-structure","file-integrity"], _ => return None,
})}
fn finding(d: &'static str,r: &'static str,s: &'static str,p: String,l: usize,m: &str,e: &str,f: &str)->Finding { Finding{detector:d,rule:r,severity:s,path:p,line:l,summary:m.into(),evidence:e.into(),fix:f.into()} }
fn rel(root:&Path,path:&Path)->String { path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\',"/") }
fn files(root:&Path)->Result<Vec<PathBuf>,String>{
 let mut stack=vec![root.to_path_buf()]; let mut out=Vec::new();
 while let Some(dir)=stack.pop(){ for entry in fs::read_dir(&dir).map_err(|e|format!("{}: {e}",dir.display()))? {
  let entry=entry.map_err(|e|e.to_string())?; let path=entry.path();
  if matches!(entry.file_name().to_str(),Some(".git"|"target"|".venv"|"venv"|"node_modules"|"__pycache__")){continue;}
  let meta=fs::symlink_metadata(&path).map_err(|e|format!("{}: {e}",path.display()))?;
  if meta.file_type().is_symlink(){continue;} if meta.is_dir(){stack.push(path);}else if meta.is_file(){out.push(path);}
 }} out.sort(); Ok(out)
}
fn head_sha(root:&Path)->Option<String>{let o=Command::new("git").arg("-C").arg(root).args(["rev-parse","--verify","HEAD"]).output().ok()?;if !o.status.success(){return None;}let s=String::from_utf8(o.stdout).ok()?.trim().to_owned();(s.len()==40&&s.bytes().all(|b|b.is_ascii_hexdigit())).then_some(s)}
fn scan(root:&Path,scope:&str,expected:Option<&str>)->Result<(Vec<Finding>,Vec<&'static str>,Option<String>),String>{
 let run=selected(scope).ok_or_else(||format!("scope tidak dikenal: {scope}"))?;
 let skipped=DETECTORS.iter().copied().filter(|d|!run.contains(d)).collect::<Vec<_>>();
 let root=fs::canonicalize(root).map_err(|e|format!("root tidak valid: {e}"))?; let all=files(&root)?;let head=head_sha(&root);let mut out=Vec::new();
 if run.contains(&"revision-integrity"){match(expected,head.as_deref()){
  (Some(w),Some(a)) if w.eq_ignore_ascii_case(a)=>{},
  (Some(w),Some(a))=>out.push(finding("revision-integrity","expected-sha-mismatch","critical",".".into(),0,"HEAD tidak cocok dengan SHA target.",&format!("expected={w}; actual={a}"),"Periksa checkout dan ulangi pada commit yang benar.")),
  (Some(w),None)=>out.push(finding("revision-integrity","head-unavailable","high",".".into(),0,"HEAD tidak dapat diverifikasi melalui Git.",w,"Jalankan dari checkout Git yang valid.")),
  (None,_)=>out.push(finding("revision-integrity","expected-sha-not-supplied","info",".".into(),0,"SHA target tidak diberikan.",head.as_deref().unwrap_or("HEAD unavailable"),"Gunakan --expected-sha untuk audit exact-SHA.")),
 }}
 if run.contains(&"path-integrity"){for p in ["Cargo.toml","README.md","AGENTS.md","docs/ONE_CONSTITUTION.md","crates/one-core/src/lib.rs"]{if !root.join(p).is_file(){out.push(finding("path-integrity","required-path-missing","high",p.into(),0,"Jalur fondasi wajib tidak ditemukan.",p,"Pulihkan file atau tinjau kontrak fondasi."));}}}
 if run.contains(&"file-integrity"){for p in &all{if matches!(p.extension().and_then(|v|v.to_str()),Some("md"|"toml"|"json"|"yml"|"yaml"|"one"|"rs"))&&fs::read_to_string(p).is_err(){out.push(finding("file-integrity","text-not-utf8","medium",rel(&root,p),0,"File teks tidak dapat dibaca sebagai UTF-8.","read_to_string gagal","Periksa encoding dan pulihkan UTF-8 valid."));}}}
 if run.contains(&"markdown-links")||run.contains(&"markdown-structure"){for p in &all{if p.extension().and_then(|v|v.to_str())!=Some("md"){continue;}let Ok(text)=fs::read_to_string(p)else{continue};let rp=rel(&root,p);
  if run.contains(&"markdown-structure"){let mut open:Option<(u8,usize)>=None;for(i,line)in text.lines().enumerate(){let t=line.trim_start();let b=t.as_bytes();let marker=if b.starts_with(&[96,96,96]){Some(96)}else if t.starts_with("~~~"){Some(126)}else{None};if let Some(m)=marker{let n=b.iter().take_while(|v|**v==m).count();if n>=3{if let Some((old,_))=open{if old==m&&b.iter().all(|v|*v==m||v.is_ascii_whitespace()){open=None;}}else{open=Some((m,i+1));}}}}if let Some((m,l))=open{out.push(finding("markdown-structure","unclosed-code-fence","medium",rp.clone(),l,"Markdown code fence tidak ditutup.",m.to_string().as_str(),"Tambahkan fence penutup yang cocok."));}}
  if run.contains(&"markdown-links"){for(i,line)in text.lines().enumerate(){let mut rest=line;while let Some(start)=rest.find("]("){rest=&rest[start+2..];let Some(end)=rest.find(')')else{break};let target=rest[..end].trim().trim_matches('<').trim_matches('>');rest=&rest[end+1..];if target.is_empty()||target.starts_with('#')||target.starts_with("//")||target.starts_with("http:")||target.starts_with("https:")||target.starts_with("mailto:"){continue;}let local=target.split('#').next().unwrap_or("").split('?').next().unwrap_or("");if !local.is_empty()&&!p.parent().unwrap_or(&root).join(local).exists(){out.push(finding("markdown-links","broken-relative-link","medium",rp.clone(),i+1,"Tautan relatif tidak ditemukan.",target,"Perbaiki target relatif atau hapus tautan usang."));}}}
 }}
 if run.contains(&"workflow-integrity"){for p in &all{let rp=rel(&root,p);if !rp.starts_with(".github/workflows/")||!matches!(p.extension().and_then(|v|v.to_str()),Some("yml"|"yaml")){continue;}let Ok(t)=fs::read_to_string(p)else{continue};if !t.lines().any(|l|l.trim_start().starts_with("name:")){out.push(finding("workflow-integrity","workflow-name-missing","medium",rp.clone(),1,"Workflow tidak memiliki name yang terlihat.","name: tidak ditemukan","Tambahkan nama workflow eksplisit."));}for(i,line)in t.lines().enumerate(){if let Some(target)=line.trim().strip_prefix("uses: ./"){let target=target.split_whitespace().next().unwrap_or(target);if !root.join(target).exists(){out.push(finding("workflow-integrity","local-action-missing","high",rp.clone(),i+1,"Local action tidak ditemukan.",target,"Pulihkan action atau perbaiki referensi."));}}}}}
 if run.contains(&"contract-drift"){let r=fs::read_to_string(root.join("README.md")).unwrap_or_default();for c in ["one buat","one periksa","one jalankan","one test","one bangun","one package","one version","one help"]{if !r.contains(c){out.push(finding("contract-drift","cli-doc-command-missing","medium","README.md".into(),0,"Dokumentasi tidak memuat perintah CLI inti.",c,"Selaraskan README dengan kontrak CLI."));}}}
 if run.contains(&"artifact-integrity")&&!root.join("Cargo.lock").is_file(){out.push(finding("artifact-integrity","cargo-lock-missing","medium","Cargo.lock".into(),0,"Cargo.lock tidak ditemukan.","lockfile hilang","Tambahkan lockfile yang sesuai toolchain."));}
 out.sort_by(|a,b|(&a.path,a.line,a.detector,a.rule).cmp(&(&b.path,b.line,b.detector,b.rule)));Ok((out,skipped,head))
}
pub(crate) fn run(args:impl IntoIterator<Item=String>)->i32{
 let mut args=args.into_iter();let mut scope="all".to_owned();let mut root=PathBuf::from(".");let mut expected=None;
 while let Some(a)=args.next(){match a.as_str(){
 "--scope"=>{let Some(v)=args.next()else{eprintln!("ONE-INTEGRITY: --scope membutuhkan nilai");return 2;};scope=v;},
 "--root"=>{let Some(v)=args.next()else{eprintln!("ONE-INTEGRITY: --root membutuhkan direktori");return 2;};root=PathBuf::from(v);},
 "--expected-sha"=>{let Some(v)=args.next()else{eprintln!("ONE-INTEGRITY: --expected-sha membutuhkan SHA");return 2;};if v.len()!=40||!v.bytes().all(|b|b.is_ascii_hexdigit()){eprintln!("ONE-INTEGRITY: SHA harus 40 digit heksadesimal");return 2;}expected=Some(v);},
 "--help"|"-h"=>{println!("one pindai [--scope all|sha|workflow|structure|documentation|artifacts|contracts|markdown|readme] [--root DIR] [--expected-sha FULL_SHA]");return 0;},
 _=>{eprintln!("ONE-INTEGRITY: opsi tidak dikenal '{a}'");return 2;}
 }}
 match scan(&root,&scope,expected.as_deref()){Ok((findings,skipped,head))=>{println!("ONE Integrity Engine (native preview)\nroot: {}\nscope: {}\nHEAD: {}\ndetectors_run: {}\ndetectors_not_requested: {}\nfindings: {}",root.display(),scope,head.as_deref().unwrap_or("UNAVAILABLE"),selected(&scope).unwrap_or_default().join(", "),skipped.join(", "),findings.len());for f in &findings{println!("[{}] {} {}:{} [{}] {}\n  evidence: {}\n  recommendation: {}",f.severity.to_uppercase(),f.detector,f.path,f.line,f.rule,f.summary,f.evidence,f.fix);}if findings.is_empty(){println!("Tidak ada temuan pada detector yang dijalankan; detector yang tidak diminta bukan berarti lulus.");}if findings.iter().any(|f|f.severity=="critical"||f.severity=="high"){1}else{0}},Err(e)=>{eprintln!("ONE-INTEGRITY: {e}");2}}
}
#[cfg(test)]mod tests{use super::*;#[test]fn scopes_are_explicit_and_fail_closed(){assert_eq!(selected("sha").unwrap(),vec!["revision-integrity"]);assert!(selected("markdown").unwrap().contains(&"markdown-links"));assert!(selected("unknown").is_none());}
#[test]fn finds_broken_markdown_link_and_unclosed_fence(){let root=std::env::temp_dir().join(format!("one-integrity-test-{}",std::process::id()));let _=fs::remove_dir_all(&root);fs::create_dir_all(&root).unwrap();fs::write(root.join("test.md"),"[broken](missing.md)\n~~~rs\ncode\n").unwrap();let(f,s,_)=scan(&root,"markdown",None).unwrap();assert!(f.iter().any(|x|x.rule=="broken-relative-link"));assert!(f.iter().any(|x|x.rule=="unclosed-code-fence"));assert!(s.contains(&"revision-integrity"));fs::remove_dir_all(root).unwrap();}}
