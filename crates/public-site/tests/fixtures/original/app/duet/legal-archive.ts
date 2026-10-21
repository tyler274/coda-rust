// Earlier versions of the Duet Terms and Privacy Policy, frozen exactly as published.
// Add a version here, newest first, when the current text changes. Write its dates
// out as they appeared on the published page; never reference the current
// duetPrivacyEffective or duetTermsEffective, which move with the next version.
import privacy20260909 from "./archive/privacy-2026-09-09.json";
import privacy20260923 from "./archive/privacy-2026-09-23.json";
import privacy20260924 from "./archive/privacy-2026-09-24.json";
import terms20260908 from "./archive/terms-2026-09-08.json";
import type { LegalSection } from "@/components/LegalPage";

export type ArchivedLegalVersion = {
  slug: string;
  version?: string;
  effective: string;
  lastUpdated: string;
  summary?: string[];
  sections: LegalSection[];
};

export const duetTermsArchive: ArchivedLegalVersion[] = [
  {
    slug: "2026-09-08",
    version: "2026-09-08",
    effective: "September 8, 2026",
    lastUpdated: "September 9, 2026",
    sections: terms20260908,
  },
];

export const duetPrivacyArchive: ArchivedLegalVersion[] = [
  {
    slug: "2026-09-24",
    effective: "September 24, 2026",
    lastUpdated: "September 24, 2026",
    summary: [
      "We never sell your health information or use it for advertising.",
      "You choose what Duet can read. Duet never changes your records at the source.",
      "AI is optional. Duet asks first, and the rest of the app works without it.",
      "We don't use your health information to train AI models, and we don't let our AI provider do so.",
      "Measurements you share with another person are end-to-end encrypted.",
      "You can export your information or delete your account at any time in the app.",
    ],
    sections: privacy20260924,
  },
  {
    slug: "2026-09-23",
    effective: "September 8, 2026",
    lastUpdated: "September 23, 2026",
    sections: privacy20260923,
  },
  {
    slug: "2026-09-09",
    effective: "September 8, 2026",
    lastUpdated: "September 9, 2026",
    sections: privacy20260909,
  },
];

export function archivedVersion(archive: ArchivedLegalVersion[], slug: string) {
  return archive.find((entry) => entry.slug === slug);
}
