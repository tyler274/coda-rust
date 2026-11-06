"""Exercise the actual preseed owner against the synthetic immutable transport."""
import subprocess

from nix_cache_copy import copy_closure
import test_nix_cache_transport as transport


class PreseedTransport(transport.NixCacheTransport):
    # Inherit the opt-in and disposable store setup, not the baseline cases.
    test_concurrent_failure_and_retry = None
    test_interrupted_publish_retry_preserves_nar = None
    test_serialized_paths_succeed_and_repeat_preserves_objects = None

    def test_actual_preseed_preserves_parallel_completion(self):
        with transport.CacheServer() as server:
            def run(args, *, env, data=None, label=None):
                command = [self.nix, '--offline', '--store', self.source, *args[1:]]
                result = subprocess.run(command, env=self.env, input=data,
                                        capture_output=True, text=True, timeout=45)
                self.assertEqual(result.returncode, 0, result.stderr)
                return result.stdout

            copy_closure(run, self.env, server.destination, '\n'.join(self.paths) + '\n', 'copy', ValueError)
            self.assertEqual(sum('/nar/' in key for key in server.objects), 1)
            self.assertEqual(sum(key.endswith('.narinfo') for key in server.objects), 2)
            before = self.snapshot(server)
            puts = sum(method == 'PUT' for method, _, _ in server.events)
            copy_closure(run, self.env, server.destination, '\n'.join(self.paths) + '\n', 'copy', ValueError)
            self.assert_preserved(before, server)
            self.assertEqual(sum(method == 'PUT' for method, _, _ in server.events), puts)
