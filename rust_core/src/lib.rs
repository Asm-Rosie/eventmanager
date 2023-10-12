use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{self, prelude::*, BufRead, BufReader, BufWriter, Read};
use std::path::PathBuf;
use std::path::Path;
use chrono::prelude::*;
use std::fs::OpenOptions;
use std::fs;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::fs::read_to_string;
use chrono::{Local, Datelike, Utc, offset::TimeZone};
use std::thread::sleep;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use rodio::{Decoder, OutputStream, Sink};













mod play_ui_sound;

mod does_file_exist;

mod create_new_entry;

mod check_if_event_expired;

mod delete_block_call;

mod edit_content;

mod expose_strings;

