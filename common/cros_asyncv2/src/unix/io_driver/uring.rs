// Copyright 2021 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{sync::Arc, time::Duration};

use sys_util::{AsRawDescriptor, SafeDescriptor};

use crate::AsIoBufs;

pub struct Waker;

impl Waker {
    pub fn wake(&self) -> anyhow::Result<()> {
        todo!();
    }
}

pub fn use_uring() -> bool {
    false
}

pub fn new_waker() -> anyhow::Result<Waker> {
    Ok(Waker)
}

// Wait for more events.
pub fn wait(timeout: Option<Duration>) -> anyhow::Result<()> {
    todo!();
}

// Wake up any tasks that are ready.
pub fn dispatch() -> anyhow::Result<()> {
    todo!();
}

pub async fn read(
    desc: &Arc<SafeDescriptor>,
    buf: &mut [u8],
    offset: Option<u64>,
) -> anyhow::Result<usize> {
    todo!();
}

pub async fn read_iobuf<B: AsIoBufs + 'static>(
    desc: &Arc<SafeDescriptor>,
    buf: B,
    offset: Option<u64>,
) -> (anyhow::Result<usize>, B) {
    todo!();
}

pub async fn write(
    desc: &Arc<SafeDescriptor>,
    buf: &[u8],
    offset: Option<u64>,
) -> anyhow::Result<usize> {
    todo!();
}

pub async fn write_iobuf<B: AsIoBufs + 'static>(
    desc: &Arc<SafeDescriptor>,
    buf: B,
    offset: Option<u64>,
) -> (anyhow::Result<usize>, B) {
    todo!();
}

pub async fn fallocate(
    desc: &Arc<SafeDescriptor>,
    file_offset: u64,
    len: u64,
    mode: u32,
) -> anyhow::Result<()> {
    todo!();
}

pub async fn ftruncate(desc: &Arc<SafeDescriptor>, len: u64) -> anyhow::Result<()> {
    todo!();
}

pub async fn stat(desc: &Arc<SafeDescriptor>) -> anyhow::Result<libc::stat64> {
    todo!();
}

pub async fn fsync(desc: &Arc<SafeDescriptor>, datasync: bool) -> anyhow::Result<()> {
    todo!();
}

pub fn prepare(fd: &dyn AsRawDescriptor) -> anyhow::Result<()> {
    todo!();
}
