#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""External Windows process CPU/memory observation around explicit idle markers."""
import ctypes
from ctypes import wintypes
import subprocess
import time
import threading

from run_perf import memory_sample


def cpu_seconds(process):
    api = ctypes.WinDLL('kernel32', use_last_error=True).GetProcessTimes
    api.argtypes = [wintypes.HANDLE] + [ctypes.POINTER(wintypes.FILETIME)] * 4
    api.restype = wintypes.BOOL
    values = [wintypes.FILETIME() for _ in range(4)]
    if not api(wintypes.HANDLE(int(process._handle)), *(ctypes.byref(v) for v in values)):
        raise ctypes.WinError(ctypes.get_last_error())
    return sum((v.dwHighDateTime << 32) | v.dwLowDateTime for v in values[2:]) / 10_000_000


def observe(command, cwd, log, env):
    start = None
    report = None
    process = subprocess.Popen(command, cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, encoding='utf-8')
    deadline = threading.Timer(180, process.kill)
    deadline.daemon = True
    deadline.start()
    try:
        with log.open('w', encoding='utf-8') as stream:
            for line in process.stdout:
                stream.write(line)
                stream.flush()
                if line.strip() == 'IDLE_BEGIN':
                    if start is not None:
                        raise ValueError('Duplicate idle start')
                    start = (time.monotonic(), cpu_seconds(process), memory_sample(process))
                elif line.startswith('FINAL '):
                    if start is None:
                        raise ValueError('Missing idle start')
                    elapsed = time.monotonic() - start[0]
                    cpu = cpu_seconds(process) - start[1]
                    report = dict(seconds=elapsed, cpu_seconds=cpu, one_core_percent=100*cpu/elapsed,
                                  before=start[2], after=memory_sample(process))
        code = process.wait(timeout=15)
        # Pipe delivery has millisecond jitter; the host timer is explicitly 60s.
        if code or report is None or report['seconds'] < 59.99:
            raise ValueError('Incomplete idle observation')
        return report
    finally:
        deadline.cancel()
        if process.poll() is None:
            process.kill()
            process.wait()
