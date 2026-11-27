#![forbid(unsafe_code)]
use verdant_admit::{Adt10 as Request, Adt3 as Event, Adt0 as Session, Adt5 as Failure, Adt6 as Kind, Adt4 as Start, Bytes, List};

use std::env;

use std::io::{Error, ErrorKind, IsTerminal, Write};

use std::{io, process};

use std::thread::Builder;

pub struct World { files: Files, procs: Procs }

pub fn failure(error: Error) -> Failure { let kind = match error.kind() { ErrorKind::NotFound => Kind::C0, ErrorKind::AlreadyExists => Kind::C1, ErrorKind::PermissionDenied => Kind::C2, _ => Kind::C3, }; Failure::C0(kind, Bytes::lit(error.to_string().as_bytes())) }

pub fn done(result: Result<(), Error>) -> Event { match result { Ok(_) => Event::C2(Ok(())), Err(error) => Event::C2(Err(failure(error))), } }

pub fn refuse() -> Event { Event::C2(Err(Failure::C0(Kind::C3, Bytes::lit("refused".as_bytes())))) }

pub fn bytes_of(items: Vec<Vec<u8>>) -> List<Bytes> { let mut out: List<Bytes> = List::nil(); let mut rest = items; while let Some(last) = rest.pop() { out = List::cons(Bytes::lit(&last), out); }; out }

pub fn waited_of(items: Vec<(u64, Option<i32>)>) -> List<(u64, Option<i32>)> { let mut out: List<(u64, Option<i32>)> = List::nil(); let mut rest = items; while let Some(last) = rest.pop() { out = List::cons(last, out); }; out }

pub fn pairs_of(items: Vec<(Vec<u8>, Vec<u8>)>) -> List<(Bytes, Bytes)> { let mut out: List<(Bytes, Bytes)> = List::nil(); let mut rest = items; while let Some(last) = rest.pop() { out = List::cons((Bytes::lit(&last.0), Bytes::lit(&last.1)), out); }; out }

pub fn start() -> Event { let mut arguments: Vec<Vec<u8>> = Vec::new(); for argument in env::args_os().skip(1) { arguments.push(argument.into_encoded_bytes()); }; let directory = match env::current_dir() { Ok(found) => Ok(Bytes::lit(&found.into_os_string().into_encoded_bytes())), Err(error) => Err(failure(error)), }; let executable = match env::current_exe() { Ok(found) => Ok(Bytes::lit(&found.into_os_string().into_encoded_bytes())), Err(error) => Err(failure(error)), }; let mut environment: Vec<(Vec<u8>, Vec<u8>)> = Vec::new(); for pair in env::vars_os() { environment.push((pair.0.into_encoded_bytes(), pair.1.into_encoded_bytes())); }; Event::C0(Start::C0(bytes_of(arguments), directory, executable, pairs_of(environment), io::stderr().is_terminal())) }

pub fn perform(world: &mut World, request: Request) -> Event { match request { Request::C0(path) => files_read(&path), Request::C1(path, data) => files_write(&path, &data), Request::C2(path) => files_list(&path), Request::C3(path) => files_stat(&path), Request::C4(path) => files_create_dir(&path), Request::C5(path) => files_remove_file(&path), Request::C6(path) => files_remove_tree(&path), Request::C7(from, to) => files_rename(&from, &to), Request::C8(path) => files_canonical(&path), Request::C9(path) => files_lock(&mut world.files, &path), Request::C10(spec) => process_spawn(&mut world.procs, spec), Request::C11(handles, limit) => process_wait(&mut world.procs, handles, limit), Request::C12(handle) => process_stop(&mut world.procs, handle), Request::C13(_, _, _) => refuse(), } }

pub fn leave(status: u8, output: &Bytes, errors: &Bytes) -> i32 { let _ = io::stdout().write_all(output.octets()); let _ = io::stdout().flush(); let _ = io::stderr().write_all(errors.octets()); let _ = io::stderr().flush(); i32::from(status) }

pub fn overflowed() -> i32 { let _ = io::stderr().write_all("lexlean: arithmetic overflow in the program".as_bytes()); 70 }

pub fn drive() -> i32 { let mut world = World { files: Files::start(), procs: Procs::start() }; let mut state: Option<Session> = None; let mut event = start(); loop { match next(state, event) { Ok((kept, request)) => { state = Some(kept); match request { Request::C13(status, output, errors) => return leave(status, &output, &errors), other => { event = perform(&mut world, other); }, } }, Err(_) => return overflowed(), } } }

pub fn main() { match Builder::new().stack_size(1073741824).spawn(drive) { Ok(running) => match running.join() { Ok(code) => process::exit(code), Err(_) => process::exit(70), }, Err(_) => process::exit(70), } }

use verdant_admit::{R, run as entry};

pub fn next(state: Option<Session>, event: Event) -> R<(Session, Request)> { entry(state, event) }

use verdant_admit::{Str, Adt7 as Entry, Adt8 as Shape};

use std::fs;

use std::fs::{File, Metadata};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

pub struct Files { locks: Vec<File> }

impl Files {
    pub fn start() -> Files { Files { locks: Vec::new() } }
}

pub fn files_read(path: &Str) -> Event { match fs::read(path.text()) { Ok(data) => Event::C1(Ok(Bytes::lit(&data))), Err(error) => Event::C1(Err(failure(error))), } }

pub fn files_write(path: &Str, data: &Bytes) -> Event { done(fs::write(path.text(), data.octets())) }

pub fn files_list(path: &Str) -> Event { match fs::read_dir(path.text()) { Ok(entries) => { let mut out: Vec<Vec<u8>> = Vec::new(); for found in entries { match found { Ok(item) => out.push(item.file_name().into_encoded_bytes()), Err(error) => return Event::C3(Err(failure(error))), } }; out.sort(); Event::C3(Ok(bytes_of(out))) }, Err(error) => Event::C3(Err(failure(error))), } }

#[cfg(unix)]
pub fn identity(meta: &Metadata) -> Option<(u64, u64)> { Some((meta.dev(), meta.ino())) }

#[cfg(not(unix))]
pub fn identity(_meta: &Metadata) -> Option<(u64, u64)> { None }

pub fn files_stat(path: &Str) -> Event { match fs::symlink_metadata(path.text()) { Ok(meta) => { let shape = match (meta.is_symlink(), meta.is_file(), meta.is_dir()) { (true, _, _) => Shape::C2, (_, true, _) => Shape::C0, (_, _, true) => Shape::C1, _ => Shape::C3, }; Event::C4(Ok(Entry::C0(shape, meta.len(), identity(&meta)))) }, Err(error) => Event::C4(Err(failure(error))), } }

pub fn files_create_dir(path: &Str) -> Event { done(fs::create_dir_all(path.text())) }

pub fn files_remove_file(path: &Str) -> Event { done(fs::remove_file(path.text())) }

pub fn files_remove_tree(path: &Str) -> Event { done(fs::remove_dir_all(path.text())) }

pub fn files_rename(from: &Str, to: &Str) -> Event { done(fs::rename(from.text(), to.text())) }

pub fn files_canonical(path: &Str) -> Event { match fs::canonicalize(path.text()) { Ok(found) => Event::C5(Ok(Bytes::lit(&found.into_os_string().into_encoded_bytes()))), Err(error) => Event::C5(Err(failure(error))), } }

pub fn files_lock(files: &mut Files, path: &Str) -> Event { match File::create(path.text()) { Ok(file) => match file.lock() { Ok(_) => { files.locks.push(file); Event::C2(Ok(())) }, Err(error) => Event::C2(Err(failure(error))), }, Err(error) => Event::C2(Err(failure(error))), } }

use verdant_admit::{Adt11 as Spec, Adt9 as Waited};

use std::mem;

use std::process::{Child, Command, Stdio};

use std::thread;

use std::time::{Duration, Instant};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

pub struct Procs { children: Vec<(u64, Child)>, next: u64, epoch: Instant }

impl Procs {
    pub fn start() -> Procs { Procs { children: Vec::new(), next: 0, epoch: Instant::now() } }
}

#[cfg(unix)]
pub fn grouped(command: &mut Command) { command.process_group(0); }

#[cfg(not(unix))]
pub fn grouped(_command: &mut Command) { }

#[cfg(unix)]
pub fn end_group(pid: u32) { let mut target = String::new(); target.push('-'); target.push_str(&pid.to_string()); let mut kill = Command::new("kill"); kill.arg("-KILL"); kill.arg("--"); kill.arg(target); kill.stdin(Stdio::null()); kill.stdout(Stdio::null()); kill.stderr(Stdio::null()); let _ = kill.status(); }

#[cfg(not(unix))]
pub fn end_group(pid: u32) { let mut kill = Command::new("taskkill"); kill.arg("/PID"); kill.arg(pid.to_string()); kill.arg("/T"); kill.arg("/F"); kill.stdin(Stdio::null()); kill.stdout(Stdio::null()); kill.stderr(Stdio::null()); let _ = kill.status(); }

pub fn end(child: &mut Child) { end_group(child.id()); let _ = child.kill(); let _ = child.wait(); }

pub fn process_spawn(procs: &mut Procs, spec: Spec) -> Event { match spec { Spec::C0(program, arguments, directory, environment, output, errors) => { let mut command = Command::new(program.text()); let mut rest = arguments; while let Some((head, tail)) = rest.uncons() { command.arg(head.text()); rest = tail; }; command.current_dir(directory.text()); command.env_clear(); let mut pairs = environment; while let Some((pair, tail)) = pairs.uncons() { command.env(pair.0.text(), pair.1.text()); pairs = tail; }; command.stdin(Stdio::null()); grouped(&mut command); match File::create(output.text()) { Ok(out) => match File::create(errors.text()) { Ok(err) => { command.stdout(out); command.stderr(err); match Command::spawn(&mut command) { Ok(child) => { let handle = procs.next; procs.next = 1 + handle; let pid = u64::from(child.id()); procs.children.push((handle, child)); Event::C6(Ok((handle, pid))) }, Err(error) => Event::C6(Err(failure(error))), } }, Err(error) => Event::C6(Err(failure(error))), }, Err(error) => Event::C6(Err(failure(error))), } }, } }

pub fn clock(procs: &Procs) -> u64 { match u64::try_from(procs.epoch.elapsed().as_millis()) { Ok(millis) => millis, Err(_) => u64::MAX, } }

pub fn process_wait(procs: &mut Procs, handles: List<u64>, limit: u64) -> Event { let mut wanted: Vec<u64> = Vec::new(); let mut rest = handles; while let Some((head, tail)) = rest.uncons() { wanted.push(head); rest = tail; }; let deadline = clock(procs).saturating_add(limit); loop { let all = mem::take(&mut procs.children); let mut finished: Vec<(u64, Option<i32>)> = Vec::new(); let mut any = false; for (handle, mut child) in all { if wanted.contains(&handle) { match child.try_wait() { Ok(Some(status)) => { finished.push((handle, status.code())); any = true; }, Ok(None) => procs.children.push((handle, child)), Err(_) => { end(&mut child); finished.push((handle, None)); any = true; }, } } else { procs.children.push((handle, child)); } }; let now = clock(procs); if any || now >= deadline { return Event::C7(Ok(Waited::C0(waited_of(finished), now))); } else { thread::sleep(Duration::from_millis(2)); } } }

pub fn process_stop(procs: &mut Procs, handle: u64) -> Event { let all = mem::take(&mut procs.children); for (held, mut child) in all { if held == handle { end(&mut child); } else { procs.children.push((held, child)); } }; Event::C2(Ok(())) }
