// Terminology notice copy is maintained in coda-app/ios/DuetApp/Resources/ClinicalTerminologyNotices.json.
import type { LegalSection } from "@/components/LegalPage";
import clinicalTerminologyNotices from "./clinical-terminology-notices.json";
import { contactEmail } from "@/lib/site";

// The Terms version Duet records when you accept, in the same form. It must equal
// CURRENT_TERMS_VERSION in coda-app; publish this page with the production deploy
// that sets that version.
export const duetTermsVersion = "2026-09-23";

// The day each document's current text goes live. Set it at deploy, in the commit
// that publishes the change. When a Privacy Policy or Terms is replaced, archive it
// with its date written out in legal-archive.ts. The consumer health data policy
// shares the Privacy Policy's date because they describe the same collection.
export const duetTermsEffective = "September 24, 2026";
export const duetPrivacyEffective = "October 3, 2026";

// These two passages appear word for word in the app (coda-app
// DuetDisclosureCopy in SettingsView.swift). Change both together; a change to the
// AI purposes also needs a new AI consent version in coda-app.
export const duetAIPurposes =
  "your Duet conversation, remembering what you share, check-ins and reminders you ask for, reviews of new information, insights, and summaries you prepare for your doctor";
export const duetProductBoundary =
  "Duet does not diagnose or monitor emergencies, and it does not judge how serious or urgent something is. It does not recommend treatment, tests, or dose changes for you. Information may be incomplete or delayed. For medical advice, contact a qualified clinician. In an emergency, call 911.";

const contact = "legal@codahealth.ai";

// Washington's My Health My Data Act policy, linked from every page's footer.
// The Attorney General's FAQ asks for a separate page holding only what
// RCW 19.373.020 requires, so keep anything else out of it.
export const duetConsumerHealthDataPath = "/duet/consumer-health-data";

export const duetPrivacySummary = [
  "We never sell your health information or use it for advertising.",
  "You choose what Duet can read. Duet never changes your records at the source.",
  "AI is optional. Duet asks first, and the rest of the app works without it.",
  "We don't use your health information to train AI models, and we don't let our AI provider do so.",
  "Measurements you share with another person are end-to-end encrypted.",
  "You can export your information or delete your account at any time in the app.",
];

export const duetPrivacySections: LegalSection[] = [
  {
    id: "collect",
    title: "What does Duet collect?",
    body: [
      "It depends on the features you use. You choose which health sources to connect. Duet reads what you allow and never changes records in Apple Health, Epic MyChart, or another source.",
      {
        table: {
          columns: ["Information", "What it includes", "Why we use it"],
          rows: [
            [
              "Account",
              "Your account details, display name, and measurement display preferences.",
              "To sign you in, keep your account, and show your name to people you share with.",
            ],
            [
              "Connected health information",
              "What you allow Duet to read from Apple Health, MyChart, or another supported source.",
              "To organize your health history and calculate measurements and comparisons.",
            ],
            [
              "Conversation",
              "What you write, Duet's replies, and what Duet remembers about you.",
              "To reply to you and pick up where you left off.",
            ],
            [
              "Plans and check-ins",
              "Plans you save, and check-ins and reminders you ask Duet to set up.",
              "To follow through on what you asked for.",
            ],
            [
              "Insights and doctor materials",
              "Insights Duet shows you, and the items, notes, and summaries you prepare for your doctor.",
              "To point out useful changes and help you prepare for appointments.",
            ],
            [
              "Feedback",
              "Your ratings of Duet's replies, any reasons you pick, and an optional note.",
              "To keep your feedback in your account and count ratings, as described below.",
            ],
            [
              "Sharing",
              "Who you share with, which measurement types, and delivery timing. Shared values are encrypted, and we can't read them.",
              "To deliver what you choose to share.",
            ],
            [
              "Your agreements",
              "The Terms version you accepted, your AI permission choice and its version, and when you made each choice. These records contain no health information.",
              "To keep a record of what you agreed to.",
            ],
            [
              "App and device",
              "The app's version, your iPhone's time zone with each message, when you last opened Duet, and each iPhone's notification token.",
              "To tell you when an update is needed, apply the daily message limit, refresh MyChart, and send notifications.",
            ],
            [
              "Items from earlier versions",
              "Records, original files, and private Moments you saved in earlier versions of Duet.",
              "To keep what you saved until you delete your account.",
            ],
          ],
        },
      },
      "Our public website, separate from the app, may collect browser, device, page-view, performance, and service-log information through its hosting, analytics, and performance providers. We use it to run and improve the website. It isn't added to your Duet health profile or combined with your health information for advertising.",
    ],
  },
  {
    id: "limits",
    title: "What won't you do with my information?",
    body: [
      "We use your information to provide Duet. We don't:",
      {
        list: [
          "Sell health information.",
          "Use it for advertising or credit decisions.",
          "Use it to train or fine-tune AI models.",
          "Let our service providers use your health information for advertising, data brokerage, or tracking you across apps.",
          "Use it for product improvement or research without your separate permission.",
        ],
      },
    ],
  },
  {
    id: "ai",
    title: "How does Duet use AI?",
    body: [
      "AI is optional. After you accept the Terms and finish setup, Duet asks for your permission before any AI processing. You don't need to connect a health source first.",
      `If you choose Allow AI, Coda uses third-party AI services for ${duetAIPurposes}. If you choose Not now, the rest of Duet still works. You can change your choice in Settings.`,
      "To do this, Duet uses what you write, its earlier replies, what it remembers about you, selected health facts, and relevant details from your connected records and the items you save in Duet. For a doctor summary, it uses only the information and notes you select.",
      "Duet never sends these to AI services:",
      {
        list: [
          "Raw health samples, ECG waveforms, or audiogram measurements.",
          "Workout routes.",
          "Original files from your sources.",
          "Anything another person shares with you.",
        ],
      },
      "Anything you type or paste into the conversation is sent as you wrote it.",
      "Duet keeps one private set of notes about what matters to you, such as your goals, preferences, circumstances, and ongoing concerns. The notes can also reflect your connected records. To correct them, tell Duet in the conversation. In a long conversation, Duet may also keep a shorter summary of what you discussed earlier.",
      "You can pause AI in Settings and resume it later, or withdraw your permission to stop future AI processing. Neither deletes your records, conversation, what Duet remembers, insights, or doctor materials. Withdrawing can't recall information already sent.",
      "OpenAI is our AI provider. Duet sends it information only after you choose Allow AI. Our agreement with OpenAI covers the handling of eligible health information. We turn off ordinary response storage and don't permit model training with your health information.",
      "OpenAI may still keep limited information for abuse monitoring, legal obligations, and technical processing, including encrypted prompt caching for up to 24 hours. Our settings restrict human review. They don't guarantee zero retention or processing only in the United States.",
    ],
  },
  {
    id: "providers",
    title: "Who else handles my information?",
    body: [
      "These service providers process information to provide and protect Duet. We don't let them use your health information for advertising, data brokerage, or tracking you across apps.",
      {
        table: {
          columns: ["Provider", "What they do", "What they receive"],
          rows: [
            [
              "Google Cloud",
              "Account sign-in, hosting, and storage for Duet's services.",
              "Account information, health information, your conversation and what Duet remembers, and temporary files.",
            ],
            [
              "OpenAI",
              "AI processing, only after you choose Allow AI.",
              "The information described in How does Duet use AI?",
            ],
            [
              "Epic",
              "MyChart authorization and access to the records you request.",
              "Your requests to connect and read your MyChart records.",
            ],
            [
              "Apple",
              "Push notifications.",
              "Your iPhone's notification token, silent background notifications, and generic alerts.",
            ],
            [
              "Website providers",
              "Hosting, analytics, and performance for our public website.",
              "Browser, device, page-view, performance, and service-log information from the website.",
            ],
          ],
        },
      },
      "Credentials for connected health services are encrypted and held by Duet's server.",
      "We may also disclose information, including health information, when required by law, legal process, or a governmental request; to protect rights, safety, and security; or in connection with a merger, financing, acquisition, or similar corporate transaction.",
    ],
  },
  {
    id: "sources",
    title: "What happens when I connect or disconnect a source?",
    body: [
      "You choose Apple Health permissions by data type on your iPhone. Each MyChart organization has its own authorization and disconnect control. Connecting one doesn't give Duet access to another.",
      "Duet keeps the source and date with your records. Information from a source may be delayed, incomplete, changed, or unavailable.",
      "While you use Duet, it refreshes MyChart about once a day. To do this, it records when you last opened Duet. If you haven't opened Duet for seven days, automatic refreshes stop until you come back.",
      "Disconnecting a source stops future access and updates and removes its saved credentials. It doesn't change the original source. Duet keeps the history it already imported, along with insights, doctor materials, and what it remembers, and can still use them. They stay until you delete your account. You can remove doctor materials in For my doctor.",
      "Disconnecting MyChart in Duet doesn't confirm that Epic has revoked its authorization. To revoke access at the source, use the connected-apps controls in MyChart.",
    ],
  },
  {
    id: "sharing",
    title: "How does sharing with another person work?",
    body: [
      "You choose what to share and with whom. Receiving someone's information doesn't give them access to yours. Duet has no public profiles, no contact discovery, and no automatic access to new health information.",
      "Your iPhone encrypts a separate copy of the daily measurements you select for each person. The private sharing key stays on that iPhone. In normal operation, Coda stores and delivers this encrypted information without being able to read the shared values. We can see the accounts involved, which measurement types you share, and delivery timing.",
      "Other health information in your Duet account doesn't have this end-to-end encryption.",
      "Information shared with you stays separate from your own. It isn't used for your analytics, AI processing, doctor materials, personal comparisons, Apple Health, widgets, or account export.",
    ],
  },
  {
    id: "notifications",
    title: "What do notifications show?",
    body: [
      "Apple delivers Duet's notifications, and they are generic. When a check-in or reminder you asked for has a reply, the alert says only \"You have a new message.\" Notifications never include a person's name, measurement names or counts, health values, or where the information came from.",
      "Each iPhone signed in to your account registers for notifications on its own. Check-in and reminder alerts can go to every signed-in iPhone that has notifications on. Sharing alerts go only to the iPhone that holds your private sharing key.",
      "Signing out on an iPhone stops notifications to that iPhone. Sign out everywhere signs your account out on all devices and stops notifications to all of them. Another device may keep working for up to an hour.",
    ],
  },
  {
    id: "improvement",
    title: "Do you use my information to improve Duet?",
    body: [
      "You can rate Duet's replies: a thumbs up, or a thumbs down with any of the listed reasons and an optional note. Your feedback stays in your account and is included in your export. Duet doesn't use it in its replies or in what it remembers.",
      "To improve Duet, Coda uses only daily counts of how many people gave each rating and reason. We don't use your note or your conversation for these counts. Counts from fewer than five people are left out.",
      "A day's counts don't change once recorded, and they are kept after you delete your account. They don't identify you. We keep these counts only in deidentified form and do not attempt to reidentify them.",
      "We don't currently use connected health information for optional product-improvement or research programs. Connecting a source, using Duet, or allowing AI doesn't give that permission.",
      "Any future program would ask for a separate, optional choice and use only the information it needs. It would exclude raw health samples, records, documents, doctor PDFs, free text, and anything another person shares with you. You could withdraw without losing core Duet features, and deleting your account would remove the related information Duet holds.",
    ],
  },
  {
    id: "retention",
    title: "How long do you keep my information?",
    body: [
      "We keep your information while you use Duet, and as long as needed to meet legal obligations, protect the service, or carry out a deletion or withdrawal.",
      {
        table: {
          columns: ["Information", "How long we keep it"],
          rows: [
            [
              "Your account, health information, conversation, what Duet remembers, plans, check-ins, insights, and feedback",
              "Until you delete your account. Pausing AI or withdrawing AI permission doesn't delete them.",
            ],
            [
              "History from a source you disconnect",
              "Until you delete your account.",
            ],
            [
              "Doctor materials",
              "Until you remove them in For my doctor or delete your account. Duet keeps no archive of shared PDFs.",
            ],
            [
              "Shared information",
              "Until you stop sharing or delete your account.",
            ],
            [
              "Database backups",
              "We back up Duet's database every day and keep the seven most recent daily backups, with seven days of recovery logs. Backups are kept for a limited time and used only for recovery. This doesn't mean every copy is gone exactly seven days after you delete your account.",
            ],
            [
              "Recovery records",
              "Minimal records of a deletion or withdrawal: the account or item, the restriction, and its time. They contain no health information and are kept while an older backup could otherwise undo your choice.",
            ],
            [
              "Service logs",
              "30 days for routine cloud logs, 365 days for selected security and operational records, and 400 days for required cloud audit records. Access is restricted, and health information in diagnostic logs is kept to a minimum.",
            ],
            [
              "Daily feedback counts",
              "Kept after you delete your account, in deidentified form.",
            ],
          ],
        },
      },
      "Logs aren't used as a separate health profile or research dataset.",
      "Deleting your account can't remove copies in an earlier device or computer backup, or files you exported or sent to someone else. Those copies follow the rules of wherever they are.",
    ],
  },
  {
    id: "security",
    title: "How do you protect my information?",
    body: [
      "We use administrative, technical, and organizational safeguards. These include encryption in transit, encrypted credentials for connected services, restricted account access, and deletion controls. No storage or transmission system is completely secure.",
      "Duet also protects the data it keeps on your iPhone, including your measurement preferences and private sharing key.",
    ],
  },
  {
    id: "choices",
    title: "What choices do I have?",
    body: [
      "In the app, you can:",
      {
        list: [
          "Connect or disconnect health sources.",
          "Pause AI, or withdraw AI permission.",
          "Pause reminders, check-ins, and reviews.",
          "Stop sharing, disconnect, or block someone.",
          "Export your information, including your conversation and what Duet remembers.",
          "Sign out on one iPhone, or sign out everywhere.",
          "Delete your account.",
        ],
      },
      "Start fresh in For my doctor clears the note, selected items, and active summary after you confirm.",
      "Deleting your account removes your account data, Terms and AI permission records, provider credentials, health information, conversation, what Duet remembers, check-ins and reminders, feedback, insights, doctor materials, and sharing content. It also clears account data and sharing keys from your iPhone. Backups and recovery records follow the schedule above. Deletion can't take back information someone else has already received, viewed, or saved.",
    ],
  },
  {
    id: "washington-consumer-health-data",
    title: "Washington consumer health data",
    body: [
      {
        text: "If you live in Washington, or your health data is collected there, our Consumer Health Data Privacy Policy explains your rights under Washington's My Health My Data Act and how to use them.",
        link: { href: duetConsumerHealthDataPath, label: "Consumer Health Data Privacy Policy" },
      },
    ],
  },
  {
    id: "eligibility",
    title: "Who can use Duet?",
    body: [
      "Duet is for adults age 18 or older who live in the United States. You confirm this when you accept the Terms. We don't infer eligibility from your health information, device location, or Apple account.",
    ],
  },
  {
    id: "contact",
    title: "How will I hear about changes, and how do I contact you?",
    body: [
      "We may update this policy as Duet or the law changes. We will post the updated policy here with its effective date and give you additional notice when required.",
      `For privacy, rights, or appeal requests, email ${contact}. For product support, email ${contactEmail}. You can also write to Coda Compute, Inc., 99 Wall Street, New York, NY 10005.`,
    ],
  },
];

export const duetTermsSummary = [
  "Duet is free for adults 18 or older who live in the United States.",
  "Duet helps you understand and organize your health information. It is not medical care.",
  "Don't use Duet in an emergency. Call 911.",
  "AI is optional, and it can be wrong. Check what matters before you rely on it.",
  "You can send Duet up to 50 messages a day.",
  "You control what you connect and share, and you can delete your account at any time.",
];

export const duetTermsSections: LegalSection[] = [
  {
    id: "about",
    title: "What are these Terms?",
    body: [
      "Duet is an iPhone app provided by Coda Compute, Inc., doing business as Coda Health. It helps you organize health information, talk it through with Duet, see changes in your records and measurements, and prepare information for your doctor. These Terms govern your use of Duet and its related services.",
      "You must use Duet lawfully. Provide information or connect a service only when you have the right to do so.",
    ],
  },
  {
    id: "eligibility",
    title: "Who can use Duet, and what does it cost?",
    body: [
      "You may use Duet only if you are at least 18 years old and live in the United States. By choosing I agree and continue, you confirm both and accept the version of these Terms linked in the app.",
      "Duet currently has no subscription price or in-app purchase. You remain responsible for your device, internet, mobile-data, healthcare, and third-party service costs.",
    ],
  },
  {
    id: "message-limit",
    title: "Is there a message limit?",
    body: [
      "You can send Duet up to 50 messages a day. The day resets at midnight in your iPhone's time zone. Duet also accepts no more than 100 messages in any 24 hours.",
      "A message counts once Duet accepts it, even if you send it before Duet finishes replying. Check-ins, reviews, and Today don't count, and neither does a message Duet couldn't answer because of a problem on its side.",
      "At the limit, Duet keeps your draft and is ready again the next day.",
    ],
  },
  {
    id: "not-medical-care",
    title: "Is Duet medical care?",
    body: [
      "No. Duet organizes your health information and can explain it, including your records, measurements, and test results. It may describe a change or pattern, or repeat a finding from a connected record. It can share general information about common causes of a symptom and published warning signs, with their uncertainty. It may also share published guidance, such as general screening recommendations, with its source.",
      duetProductBoundary,
      "Duet also does not determine what is causing your symptoms, assess your personal medical risk or prognosis, or tell you that waiting is safe. It does not replace a qualified healthcare professional.",
      "Duet's replies, observations, and doctor summaries are for understanding, organizing, and communicating information. They are not medical conclusions or recommendations, even when they mention warning signs or suggest talking with a doctor.",
      "Do not use Duet for an emergency or a time-sensitive medical decision. Always seek professional medical advice about diagnosis, treatment, medications, or changes in care.",
    ],
  },
  {
    id: "account",
    title: "What am I responsible for with my account?",
    body: [
      "Duet requires Sign in with Apple. You are responsible for keeping access to your Apple account and device, and for activity under your Duet account. Don't let another person use your account, and don't try to access someone else's account without permission.",
      "The private key used for Duet sharing stays on the registered iPhone. Replacing a lost key pauses sharing, removes old encrypted information, and requires fresh approval from the people you share with.",
    ],
  },
  {
    id: "connected-services",
    title: "How do connected services work?",
    body: [
      "You may let Duet read information from Apple Health, one or more Epic MyChart organizations, or another supported source shown in the app. Access is limited to the permissions you grant and the Duet features you use. You can disconnect a source in Duet or revoke access through the provider.",
      "Disconnecting ends future access and refresh and removes saved provider credentials. Duet keeps the history it already imported and where it came from, along with insights, doctor materials, and what Duet remembers. These stay until you delete your account. You can remove doctor materials in For my doctor.",
      "Third-party services have their own terms, availability, and data practices. Coda Health doesn't control them and isn't responsible for provider outages, delayed device syncing, or information a provider doesn't make available.",
    ],
  },
  {
    id: "ai",
    title: "How does AI work in Duet?",
    body: [
      `AI is optional. After you accept these Terms and finish setup, Duet asks for your permission before any AI processing. If you choose Allow AI, Coda uses third-party AI services for ${duetAIPurposes}, as described in the Privacy Policy. If you choose Not now, the rest of Duet still works.`,
      "You can pause AI or withdraw AI permission in Settings. Pausing keeps your permission so you can resume later. Withdrawing stops future AI processing. Neither deletes information already in your account, including your conversation and what Duet remembers.",
      "AI-generated information may be incomplete or inaccurate. Review Duet's replies, insights, corrections, doctor summaries, exports, and sharing selections before you rely on them or send them. A generated summary is not a medical record or a note written by a clinician.",
    ],
  },
  {
    id: "your-information",
    title: "Who owns my information?",
    body: [
      "You keep your rights in the information you provide or connect. You allow Coda Health to use it to provide, secure, and support Duet, consistent with the Privacy Policy and your choices.",
      "Connecting a source, using Duet, or allowing AI doesn't authorize product-improvement or research use. Any future program requires separate, optional permission.",
    ],
  },
  {
    id: "sharing",
    title: "What should I know about sharing and exports?",
    body: [
      "Duet lets you select information for another person or for your doctor. You control what you share and where it goes. Coda Health can't take information back after a recipient has viewed, downloaded, remembered, forwarded, or captured it.",
      "Don't use sharing to harass, impersonate, deceive, or obtain health information without permission. You can stop or disconnect a Duet sharing relationship in the app.",
    ],
  },
  {
    id: "acceptable-use",
    title: "What am I not allowed to do?",
    body: [
      "You may not interfere with Duet's security or availability, reverse engineer or get around access controls except where law expressly permits, introduce malicious code, use Duet to violate another person's rights, or access systems or information without authorization.",
    ],
  },
  {
    id: "changes-to-duet",
    title: "Can Duet change or end my access?",
    body: [
      "We may change, suspend, or discontinue a feature when needed for safety, security, legal compliance, provider requirements, or product operation. We will give notice when reasonably practicable.",
      "We may suspend or end access when required by law, when needed to protect people or the service, or when these Terms are materially violated. You may stop using Duet and delete your account at any time.",
    ],
  },
  {
    id: "disclaimers",
    title: "Disclaimers",
    body: [
      "To the fullest extent permitted by law, Duet is provided on an 'as is' and 'as available' basis. Coda Compute, Inc. disclaims implied warranties, including merchantability, fitness for a particular purpose, and non-infringement. We do not warrant that connected information is complete, current, uninterrupted, or error-free.",
    ],
  },
  {
    id: "liability",
    title: "Limitation of liability",
    body: [
      "To the fullest extent permitted by law, Coda Compute, Inc. will not be liable for indirect, incidental, special, consequential, exemplary, or punitive damages, or for lost data, profits, or business interruption, arising from or related to Duet. Some jurisdictions do not allow certain exclusions, so parts of this section may not apply to you.",
    ],
  },
  {
    id: "governing-law",
    title: "Which law applies?",
    body: [
      "These Terms are governed by the laws of the State of New York, without regard to conflict-of-law rules, except where mandatory consumer-protection law provides otherwise.",
    ],
  },
  {
    id: "contact",
    title: "How will I hear about changes, and how do I contact you?",
    body: [
      "The Duet Privacy Policy explains how Coda Health handles information for Duet. We may update these Terms as Duet or the law changes. We will post the updated version here with a new effective date. When Duet needs you to accept a new version, the app will ask you to review and accept it before you continue using Duet.",
      `Send legal and privacy questions to ${contact} and product support questions to ${contactEmail}. You can also write to Coda Compute, Inc., 99 Wall Street, New York, NY 10005.`,
    ],
  },
  ...clinicalTerminologyNotices,
];

// Only the disclosures RCW 19.373.020(1)(a) requires, in its order: data collected
// and why, sources, data shared, who receives it, and how to use your rights.
export const duetConsumerHealthDataSections: LegalSection[] = [
  {
    id: "collected",
    title: "What consumer health data do we collect, and how do we use it?",
    body: [
      {
        table: {
          columns: ["Consumer health data", "How we use it"],
          rows: [
            [
              "Health information from sources you connect: what you allow Duet to read from Apple Health, Epic MyChart, or another supported source",
              "To organize your health history, calculate measurements and comparisons, and support the uses below.",
            ],
            [
              "Your conversation with Duet and what Duet remembers about you",
              "To reply to you and pick up where you left off.",
            ],
            [
              "Plans you save, and check-ins and reminders you ask for",
              "To follow through on what you asked for.",
            ],
            [
              "Insights, and the items, notes, and summaries you prepare for your doctor",
              "To point out useful changes and help you prepare for appointments.",
            ],
            [
              "Feedback on Duet's replies, including any note you add",
              "To keep your feedback in your account. To improve Duet, Coda uses only daily counts of how many people gave each rating and reason, kept in deidentified form.",
            ],
            [
              "Sharing information: who you share with and which measurement types. Shared values are end-to-end encrypted.",
              "To deliver what you choose to share.",
            ],
            [
              "Records, original files, and private Moments you saved in earlier versions of Duet",
              "To keep what you saved until you delete your account.",
            ],
          ],
        },
      },
      `If you choose Allow AI, we also use this data for ${duetAIPurposes}. We do not use it for advertising, and we do not sell it.`,
    ],
  },
  {
    id: "sources",
    title: "Where does it come from?",
    body: [
      {
        list: [
          "You: what you write, save, select, and rate in Duet.",
          "Apple Health on your iPhone, for the data types you allow.",
          "Each Epic MyChart organization you authorize.",
          "Duet itself, which creates insights, notes, and summaries from the data above.",
          "People who share measurements with you. We can't read those values.",
        ],
      },
    ],
  },
  {
    id: "shared",
    title: "What do we share, and with whom?",
    body: [
      {
        table: {
          columns: ["Who receives it", "Consumer health data shared"],
          rows: [
            [
              "Cloud hosting and storage provider (Google Cloud)",
              "All of the data above, to run and store Duet.",
            ],
            [
              "AI service provider (OpenAI), only if you choose Allow AI",
              "What you write, Duet's earlier replies, what it remembers, selected health facts, relevant details from your records and saved items, and the items you select for a doctor summary. Never raw health samples, ECG waveforms, audiogram measurements, workout routes, original files from your sources, or anything another person shares with you.",
            ],
            [
              "Health record provider (Epic)",
              "Your requests to connect and read your MyChart records.",
            ],
            [
              "People you choose to share with",
              "The daily measurements you select, end-to-end encrypted.",
            ],
            [
              "Authorities and others, when required",
              "Information needed to comply with law, legal process, or a governmental request, or to protect rights, safety, and security.",
            ],
            [
              "A party to a corporate transaction",
              "Information transferred in a merger, financing, acquisition, or similar corporate transaction.",
            ],
          ],
        },
      },
      "Coda Compute, Inc. has no affiliates, so we do not share consumer health data with any affiliate.",
    ],
  },
  {
    id: "rights",
    title: "How do I use my rights?",
    body: [
      "If you live in Washington, or your consumer health data is collected in Washington, you have the right to:",
      {
        list: [
          "Confirm whether we collect, share, or sell your consumer health data, and get access to it, including a list of the third parties and affiliates we have shared it with and how to contact them.",
          "Withdraw your consent to our collecting and sharing it.",
          "Have it deleted.",
          "Appeal if we decline your request.",
        ],
      },
      `To make a request, email ${contact}. You can also withdraw AI permission, export your information, or delete your account yourself in the app. We may need to confirm your identity before we act on a request.`,
      "We respond within 45 days of receiving your request. If we need more time, we may extend this once by up to 45 more days, and we will tell you why within the first 45 days. We answer requests free of charge, up to twice a year.",
      "When we delete your data, copies in database backups are removed as those backups expire. Until then, backups are used only for recovery.",
      `To appeal a decision, email ${contact} with "Appeal" in the subject line. Within 45 days of receiving your appeal, we will tell you in writing what we did and why. If we deny your appeal, you can file a complaint with the Washington State Attorney General at atg.wa.gov/file-complaint or 1-800-551-4636.`,
    ],
  },
];
