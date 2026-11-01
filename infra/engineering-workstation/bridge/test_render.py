from datetime import datetime
from zoneinfo import ZoneInfo
import unittest

from slack_render import chunks, render_native, thread_root_payload


class RenderingTests(unittest.TestCase):
    def test_codex_cards_preserve_finding_evidence_without_display_syntax(self):
        source = ('**One finding.**\n\n'
                  '::code-comment{title="[P3] Replay" body="A second request can reuse the link." '
                  'file="/work/src/auth.rs" start=12 end=19 priority=3}\n'
                  '::inbox-item{title="Review complete" summary="One static finding; runtime unverified"}\n')
        rendered = render_native(source)
        for evidence in ('[P3] Replay', 'A second request can reuse the link.',
                         '/work/src/auth.rs:12–19', 'Review complete',
                         'One static finding; runtime unverified'):
            self.assertIn(evidence, rendered)
        self.assertNotIn('::code-comment{', rendered)
        self.assertNotIn('::inbox-item{', rendered)

    def test_examples_and_unrecognized_cards_are_preserved(self):
        source = ('```text\n::inbox-item{title="Example" summary="Literal syntax"}\n```\n'
                  '::code-comment{title="Incomplete card" unknown data}\n')
        self.assertEqual(render_native(source), source)

    def test_chunk_boundary_does_not_cut_a_source_path(self):
        first = 'Evidence.\n\n' * 40
        location = 'Source: `/work/crates/auth/src/store.rs:277–281`\n\n'
        text = first + location + 'Next finding.\n\n' * 40
        parts = chunks(text, size=len(first) + 20)
        self.assertEqual(''.join(parts), text)
        self.assertTrue(any(location in part for part in parts))

    def test_status_header_stays_within_slack_limit(self):
        title = 'Reviewed migration ' * 12
        payload = thread_root_payload(title, datetime(2026, 10, 31, 9, 0, tzinfo=ZoneInfo('Europe/London')),
                                      'Working', 'UOWNER', 'Available during the chosen working hours.')
        header = payload['blocks'][0]['text']['text']
        self.assertLessEqual(len(header), 150)
        self.assertTrue(header.endswith('…'))
        self.assertIn(title, payload['text'])


if __name__ == '__main__':
    unittest.main()
