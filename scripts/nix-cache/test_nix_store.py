"""No Nix daemon, sudo, or sticky disk required."""
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import nix_store as store


GIB = store.GIB


class Recorder:
    def __init__(self, fail=None):
        self.calls = []
        self.fail = fail

    def __call__(self, argv, check):
        self.calls.append(argv)
        if self.fail and argv[:len(self.fail)] == self.fail:
            raise store.subprocess.CalledProcessError(1, argv)

    def ran(self, *prefix):
        return [i for i, c in enumerate(self.calls) if c[:len(prefix)] == list(prefix)]


class CapTests(unittest.TestCase):
    def test_within_cap(self):
        self.assertFalse(store.over_cap(60, store.Usage(used=59 * GIB, size=200 * GIB)))

    def test_over_cap(self):
        self.assertTrue(store.over_cap(60, store.Usage(used=61 * GIB, size=200 * GIB)))

    def test_a_small_disk_lowers_the_cap(self):
        disk = store.Usage(used=45 * GIB, size=50 * GIB)
        self.assertEqual(store.cap_bytes(60, disk), int(50 * GIB * store.DISK_SHARE))
        self.assertTrue(store.over_cap(60, disk))


class FinishTests(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.summary = Path(temp.name) / 'summary'
        self.store_dir = Path(temp.name) / 'store'
        self.store_dir.mkdir()
        for name in ('a-one', 'b-two', 'c-three', '.links'):
            (self.store_dir / name).mkdir()
        for patcher in (patch.object(store, 'STORE', self.store_dir),
                        patch.object(store.shutil, 'which', lambda name: '/usr/bin/' + name)):
            patcher.start()
            self.addCleanup(patcher.stop)

    def finish(self, used, start='1', run=None, mounted=True):
        run = run or Recorder()
        with patch.object(store, 'usage', return_value=store.Usage(used=used, size=200 * GIB)):
            store.finish({store.START_VAR: start, 'GITHUB_STEP_SUMMARY': str(self.summary)},
                         60, run, lambda path: mounted)
        return run, self.summary.read_text()

    def test_reports_added_paths_and_releases_the_disk_without_gc(self):
        run, summary = self.finish(10 * GIB)
        self.assertIn('paths_at_start=1 paths_added=2 ', summary)
        self.assertIn('gc=no', summary)
        self.assertEqual(run.ran('nix', 'store', 'gc'), [])
        self.assertEqual(len(run.ran('sudo', '-n', 'systemctl', 'stop')), 1)

    def test_over_cap_collects_all_garbage_then_releases_the_disk(self):
        run, summary = self.finish(70 * GIB)
        self.assertIn('gc=yes', summary)
        self.assertIn(['nix', 'store', 'gc'], run.calls)
        self.assertLess(run.ran('nix', 'store', 'gc')[0], run.ran('sudo', '-n', 'systemctl')[0])

    def test_the_disk_is_released_when_gc_fails(self):
        run = Recorder(fail=['nix', 'store', 'gc'])
        with self.assertRaises(store.subprocess.CalledProcessError):
            self.finish(70 * GIB, run=run)
        self.assertEqual(len(run.ran('sudo', '-n', 'systemctl', 'stop')), 1)

    def test_only_processes_on_the_mounted_disk_are_killed(self):
        run, _ = self.finish(10 * GIB)
        [kill] = run.ran('sudo', '-n', 'fuser')
        self.assertIn('-M', run.calls[kill])

    def test_a_fallback_directory_is_left_alone(self):
        # Without the mount, `fuser -m /nix` would match the root filesystem.
        run, _ = self.finish(10 * GIB, mounted=False)
        self.assertEqual(run.ran('sudo'), [])

    def test_an_unknown_start_reads_as_unknown(self):
        _, summary = self.finish(10 * GIB, start='')
        self.assertIn('paths_at_start=unknown paths_added=unknown', summary)


if __name__ == '__main__':
    unittest.main()
