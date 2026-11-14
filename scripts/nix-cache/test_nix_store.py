"""No Nix daemon, sudo, or sticky disk required."""
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import nix_store as store


GIB = store.GIB


class Recorder:
    def __init__(self):
        self.calls = []

    def __call__(self, argv, check):
        self.calls.append(argv)


class CapTests(unittest.TestCase):
    def test_within_cap_frees_nothing(self):
        self.assertEqual(store.bytes_to_free(60, store.Usage(used=59 * GIB, size=200 * GIB)), 0)

    def test_over_cap_frees_down_to_headroom(self):
        amount = store.bytes_to_free(60, store.Usage(used=70 * GIB, size=200 * GIB))
        self.assertEqual(amount, 70 * GIB - int(60 * GIB * store.GC_TARGET))

    def test_a_small_disk_lowers_the_cap(self):
        disk = store.Usage(used=45 * GIB, size=50 * GIB)
        self.assertEqual(store.cap_bytes(60, disk), int(50 * GIB * store.DISK_SHARE))
        self.assertGreater(store.bytes_to_free(60, disk), 0)


class FinishTests(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.summary = Path(temp.name) / 'summary'
        self.store_dir = Path(temp.name) / 'store'
        self.store_dir.mkdir()
        for name in ('a-one', 'b-two', 'c-three', '.links'):
            (self.store_dir / name).mkdir()
        patcher = patch.object(store, 'STORE', self.store_dir)
        patcher.start()
        self.addCleanup(patcher.stop)

    def finish(self, used, start='1'):
        run = Recorder()
        with patch.object(store, 'usage', return_value=store.Usage(used=used, size=200 * GIB)):
            store.finish({store.START_VAR: start, 'GITHUB_STEP_SUMMARY': str(self.summary)}, 60, run)
        return run.calls, self.summary.read_text()

    def test_reports_added_paths_and_releases_the_disk_without_gc(self):
        calls, summary = self.finish(10 * GIB)
        self.assertIn('paths_at_start=1 paths_added=2 ', summary)
        self.assertIn('gc=no', summary)
        self.assertNotIn('gc', [c[2] for c in calls if c[0] == 'nix'])
        self.assertIn('systemctl', calls[0])

    def test_over_cap_collects_garbage_then_releases_the_disk(self):
        calls, summary = self.finish(70 * GIB)
        self.assertIn('gc=yes', summary)
        self.assertEqual(calls[1][:3], ['nix', 'store', 'gc'])
        self.assertIn('systemctl', calls[2])

    def test_the_disk_is_released_when_gc_fails(self):
        calls = []

        def run(argv, check):
            calls.append(argv)
            if argv[:3] == ['nix', 'store', 'gc']:
                raise store.subprocess.CalledProcessError(1, argv)

        with patch.object(store, 'usage', return_value=store.Usage(used=70 * GIB, size=200 * GIB)):
            with self.assertRaises(store.subprocess.CalledProcessError):
                store.finish({store.START_VAR: '3'}, 60, run)
        gc = calls.index(next(c for c in calls if c[:3] == ['nix', 'store', 'gc']))
        self.assertTrue(any('systemctl' in c for c in calls[gc + 1:]))

    def test_an_unknown_start_reads_as_unknown(self):
        _, summary = self.finish(10 * GIB, start='')
        self.assertIn('paths_at_start=unknown paths_added=unknown', summary)


if __name__ == '__main__':
    unittest.main()
