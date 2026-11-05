import json
import unittest
from unittest.mock import Mock

from nix_cache_copy import copy_closure, seed_paths


class SeedTests(unittest.TestCase):
    def info(self):
        return {'a': {'narHash': 'same', 'references': ['dep']},
                'b': {'narHash': 'same', 'references': ['dep']},
                'dep': {'narHash': 'other', 'references': []}}

    def test_shared_content_seed_has_dependencies_first(self):
        info = self.info()
        self.assertEqual(seed_paths(json.dumps(info), info, ValueError), ['dep', 'a'])

    def test_no_duplicates_no_seed(self):
        info = self.info()
        info['b']['narHash'] = 'unique'
        self.assertEqual(seed_paths(json.dumps(info), info, ValueError), [])

    def test_list_format(self):
        info = self.info()
        self.assertEqual(seed_paths(json.dumps([dict(path=p, **v) for p, v in info.items()]), info, ValueError), ['dep', 'a'])

    def test_missing_metadata_and_references_fail_closed(self):
        info = self.info()
        for raw in ('{}', 'null', '{', json.dumps(dict(info, a={'narHash': 'same', 'references': ['missing']}))):
            with self.subTest(raw=raw), self.assertRaises(ValueError):
                seed_paths(raw, info, ValueError)

    def test_seed_then_parallel_copy(self):
        run = Mock(side_effect=[json.dumps(self.info()), '', '', ''])
        copy_closure(run, {}, 'synthetic', 'a\nb\ndep\n', 'copy', ValueError)
        calls = run.call_args_list
        self.assertEqual(len(calls), 4)
        self.assertIn('--no-recursive', calls[1].args[0])
        self.assertEqual(calls[1].kwargs['data'], 'dep\n')
        self.assertEqual(calls[2].kwargs['data'], 'a\n')
        self.assertNotIn('--no-recursive', calls[3].args[0])


if __name__ == '__main__':
    unittest.main()
